//! `oradmp._core`: the PyO3 binding of `oradmp-core` (task T-12, ADR-3). It exposes plain data and two
//! classes, `_Dump` and `_Reader`; `python/oradmp/_api.py` wraps them in the public API of architecture 3.4.
//!
//! * Every call that can take time (`open`, the scan, every batch, the survey) runs with the GIL released
//!   (`Python::detach`); the core touches no Python object.
//! * A batch reaches pyarrow through `pyo3-arrow` (`PyRecordBatch::into_pyarrow`) without a copy; the whole reader
//!   can be exported as an Arrow C stream capsule.
//! * The reader's state is shared (`Arc<Mutex<..>>`) between `_Reader` and the Arrow C stream (`SharedBatches`), so
//!   the counts and warnings follow the consumer's reads (ADR-55). No code holds a lock while it calls Python; the
//!   Python-facing methods take every lock inside `Python::detach` and copy the data out (ADR-55 item 7).
//! * Every call into the core that computes on input runs inside `oradmp_core::guard::catch_panic`: a panic is
//!   `E-INTERNAL` (ADR-59). Exempt (ADR-59 item 2, amended in session 16 / ADR-65 item 1): `Dump::info()`,
//!   `Dump::catalog()`, `Dump::warnings()` and `TableBatches::schema()` (accessors that return or clone data built
//!   under the guard), `Error`'s `Display` in `to_pyerr` (formats literals and integers), the drops in both
//!   `close()` (free buffers, close a file), and the `core_open` of `_probe_panic` (feature `test-probes` only).
//!   Also exempt (session 18 amendment, ADR-71 item 4; review 05 N-1; each is panic-free by reading): `Warning`'s
//!   `Display` in `warning_dict` and `Error`'s `Display` in `error_instance` (both in `errors.rs`; `render` of
//!   literals and integers), `Charset::from_id` and `Charset::name` (`conv.rs`; a `match` on a `u16`), the `as_str`
//!   constants of `ErrorCode` and `WarningCode` and `ALL` (module initialisation), and the implicit drops of
//!   `PyReader` and `PyDump` at garbage collection (not `close()`).
//! * Errors are built by `oradmp._errors.make_error` (class table in Python) with the names resolved from the
//!   catalog; no message holds a value of the file.

mod conv;
mod digest;
mod errors;
mod names;
mod options;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use arrow_array::{RecordBatch, RecordBatchReader};
use arrow_schema::{ArrowError, SchemaRef};
use oradmp_core::dump::{open as core_open, Catalog, CatalogEnd, Dump as CoreDump, TableRef};
use oradmp_core::guard::{catch_panic, internal_error_later};
use oradmp_core::reader::{OnValueError, ReadOptions, TableBatches};
use oradmp_core::survey::{count_table, number_summary, TableCounts};
use oradmp_core::{Error, ErrorCode, Warning};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3_arrow::{PyRecordBatch, PyRecordBatchReader, PySchema};

use crate::errors::{error_instance, to_pyerr, warning_dict};
use crate::names::Names;

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

fn closed() -> Error {
    Error::new(ErrorCode::Config, "the Dump is closed")
}

/// An opened dump.
#[pyclass(module = "oradmp._core", name = "_Dump", frozen)]
struct PyDump {
    inner: Mutex<Option<CoreDump>>,
    names: Mutex<Arc<Names>>,
    /// A call into the core panicked (ADR-59): the core dump was dropped and every later call is `E-INTERNAL`.
    broken: AtomicBool,
}

impl PyDump {
    fn names(&self) -> Arc<Names> {
        Arc::clone(&lock(&self.names))
    }

    /// The error of a call on a dump that holds no core dump: closed, or unusable after a caught panic.
    fn gone(&self) -> Error {
        if self.broken.load(Ordering::SeqCst) {
            internal_error_later()
        } else {
            closed()
        }
    }

    /// Runs `f` on the dump with the GIL released and inside the panic guard (ADR-59).
    fn with<R: Send>(
        &self,
        py: Python<'_>,
        f: impl FnOnce(&mut CoreDump) -> Result<R, Error> + Send,
    ) -> PyResult<R> {
        let r = py.detach(|| {
            let r = catch_panic(|| match lock(&self.inner).as_mut() {
                Some(d) => f(d),
                None => Err(self.gone()),
            });
            if matches!(&r, Err(e) if e.code == ErrorCode::Internal) {
                self.broken.store(true, Ordering::SeqCst);
                *lock(&self.inner) = None;
            }
            r
        });
        r.map_err(|e| to_pyerr(py, &e, Some(&self.names())))
    }

    /// Scans once (cached by the core) and renews the names; the lock is taken inside `with`, with the GIL released.
    fn scan_inner(&self, py: Python<'_>) -> PyResult<()> {
        let names = self.with(py, |d| {
            d.scan()?;
            Ok(d.catalog().map(|c| Names::from_tables(&c.tables)))
        })?;
        if let Some(n) = names {
            *lock(&self.names) = Arc::new(n);
        }
        Ok(())
    }
}

/// A copy of an error without its operating-system source (a stop of the scan is never an I/O error).
fn copy_error(e: &Error) -> Error {
    Error {
        code: e.code,
        offset: e.offset,
        table: e.table,
        part: e.part,
        column: e.column,
        row: e.row,
        length: e.length,
        type_code: e.type_code,
        warning: e.warning,
        message: e.message.clone(),
        source: None,
    }
}

/// A copy of the catalog, so that the Python objects are built after the lock is released.
fn copy_catalog(c: &Catalog) -> Catalog {
    Catalog {
        tables: c.tables.clone(),
        end: match &c.end {
            CatalogEnd::Complete => CatalogEnd::Complete,
            CatalogEnd::Stopped(e) => CatalogEnd::Stopped(copy_error(e)),
        },
        stopped_table: c.stopped_table.clone(),
        end_offset: c.end_offset,
        item_counts: c.item_counts.clone(),
    }
}

#[pymethods]
impl PyDump {
    /// The first record as a dict.
    fn info<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let info = py.detach(|| lock(&self.inner).as_ref().map(|d| d.info().clone()));
        match info {
            Some(i) => conv::info_dict(py, &i),
            None => Err(to_pyerr(py, &self.gone(), None)),
        }
    }

    /// Runs the scan once (GIL released) and returns the catalog as a dict.
    fn scan<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.scan_inner(py)?;
        let names = self.names();
        let cat = py.detach(|| {
            lock(&self.inner)
                .as_ref()
                .and_then(|d| d.catalog())
                .map(copy_catalog)
        });
        match cat {
            Some(c) => conv::catalog_dict(py, &c, &names),
            None => Err(to_pyerr(py, &self.gone(), None)),
        }
    }

    /// The warnings of the header and, after the scan, of the walk.
    fn warnings<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let names = self.names();
        let ws: Vec<Warning> = py.detach(|| {
            lock(&self.inner)
                .as_ref()
                .map(|d| d.warnings().to_vec())
                .unwrap_or_default()
        });
        let out = PyList::empty(py);
        for w in &ws {
            out.append(warning_dict(py, w, Some(&names))?)?;
        }
        Ok(out)
    }

    /// A reader for one table: by name or by ordinal.
    #[pyo3(signature = (name, ordinal, options))]
    fn read_table(
        &self,
        py: Python<'_>,
        name: Option<String>,
        ordinal: Option<u32>,
        options: &Bound<'_, PyDict>,
    ) -> PyResult<PyReader> {
        let ro = options::read_options(options).map_err(|e| to_pyerr(py, &e, None))?;
        self.scan_inner(py)?;
        let batches = self.with(py, move |d| {
            let which = match (&name, ordinal) {
                (Some(n), _) => TableRef::ByName(n.as_str()),
                (None, Some(o)) => TableRef::ByOrdinal(o),
                (None, None) => {
                    return Err(Error::new(
                        ErrorCode::Config,
                        "a table name or ordinal is needed",
                    ))
                }
            };
            d.read_table(which, &ro)
        })?;
        Ok(PyReader::new(
            batches.schema(),
            self.names(),
            Box::new(batches),
        ))
    }

    /// The counts of every table (and, if `read`, a lenient read of every table that collects the warnings and
    /// the first error of each): the survey's data. No value.
    fn survey<'py>(&self, py: Python<'py>, read: bool) -> PyResult<Bound<'py, PyDict>> {
        self.scan_inner(py)?;
        let data = self.with(py, move |d| Ok(survey_data(d, read)))?;
        let names = self.names();
        let out = PyDict::new(py);
        let tables = PyList::empty(py);
        let mut all: Vec<TableCounts> = Vec::new();
        for t in &data {
            let e = PyDict::new(py);
            e.set_item("ordinal", t.ordinal)?;
            match &t.counts {
                Ok(c) => {
                    e.set_item("counts", conv::table_counts_dict(py, c)?)?;
                    e.set_item("count_error", py.None())?;
                    all.push(c.clone());
                }
                Err(err) => {
                    e.set_item("counts", py.None())?;
                    e.set_item("count_error", error_instance(py, err, Some(&names))?)?;
                }
            }
            match &t.read {
                Some(r) => {
                    let rd = PyDict::new(py);
                    rd.set_item("rows_read", r.rows_read)?;
                    let ws = PyList::empty(py);
                    for w in &r.warnings {
                        ws.append(warning_dict(py, w, Some(&names))?)?;
                    }
                    rd.set_item("warnings", ws)?;
                    rd.set_item(
                        "error",
                        match &r.error {
                            Some(err) => Some(error_instance(py, err, Some(&names))?),
                            None => None,
                        },
                    )?;
                    e.set_item("read", rd)?;
                }
                None => e.set_item("read", py.None())?,
            }
            tables.append(e)?;
        }
        out.set_item("tables", tables)?;
        let summary = PyList::empty(py);
        // ADR-65 item 1: the only computation on input-derived data outside `with`; guarded like every other call
        let sums = py
            .detach(|| catch_panic(|| Ok(number_summary(&all))))
            .map_err(|e| to_pyerr(py, &e, Some(&names)))?;
        for s in sums {
            summary.append(conv::summary_dict(py, &s)?)?;
        }
        out.set_item("number_summary", summary)?;
        Ok(out)
    }

    /// The canonical digests (see `digest.rs`) of the columns of one table, read by the core without any
    /// Python object: `(name, rows, nulls, digest)` per column. For the pytest files.
    fn digest<'py>(
        &self,
        py: Python<'py>,
        ordinal: u32,
        options: &Bound<'py, PyDict>,
    ) -> PyResult<Vec<(String, u64, u64, u64)>> {
        let ro = options::read_options(options).map_err(|e| to_pyerr(py, &e, None))?;
        self.scan_inner(py)?;
        let r = self.with(py, move |d| {
            let mut r = d.read_table(TableRef::ByOrdinal(ordinal), &ro)?;
            let names: Vec<String> = r
                .schema()
                .fields()
                .iter()
                .map(|f| f.name().clone())
                .collect();
            let mut batches = Vec::new();
            while let Some(b) = r.next_batch()? {
                batches.push(b);
            }
            Ok(digest::digest(&batches, &names))
        })?;
        Ok(r.into_iter()
            .map(|c| (c.name, c.rows, c.nulls, c.digest))
            .collect())
    }

    /// Drops the dump: the catalog and the open state. Readers handed out keep their own handle.
    fn close(&self, py: Python<'_>) {
        py.detach(|| *lock(&self.inner) = None);
    }

    fn is_closed(&self, py: Python<'_>) -> bool {
        py.detach(|| lock(&self.inner).is_none())
    }

    /// Test probe (feature `test-probes`, ADR-59 item 5): a panicking closure through [`PyDump::with`].
    #[cfg(feature = "test-probes")]
    fn _probe_panic(&self, py: Python<'_>) -> PyResult<()> {
        self.with(py, |_d| -> Result<(), Error> { probe::boom() })
    }
}

struct TableSurvey {
    ordinal: u32,
    counts: Result<TableCounts, Error>,
    read: Option<ReadReport>,
}

struct ReadReport {
    rows_read: u64,
    warnings: Vec<Warning>,
    error: Option<Error>,
}

/// Counts every table; with `read`, also reads it leniently (`on_value_error="null"`, defaults otherwise).
fn survey_data(d: &mut CoreDump, read: bool) -> Vec<TableSurvey> {
    let ordinals: Vec<u32> = match d.tables() {
        Ok(t) => t.iter().map(|t| t.ordinal).collect(),
        Err(_) => Vec::new(),
    };
    let mut out = Vec::new();
    for ordinal in ordinals {
        let counts = count_table(d, TableRef::ByOrdinal(ordinal));
        let report = read.then(|| {
            let ro = ReadOptions {
                on_value_error: OnValueError::Null,
                ..ReadOptions::default()
            };
            match d.read_table(TableRef::ByOrdinal(ordinal), &ro) {
                Err(e) => ReadReport {
                    rows_read: 0,
                    warnings: Vec::new(),
                    error: Some(e),
                },
                Ok(mut r) => {
                    let mut error = None;
                    loop {
                        match r.next_batch() {
                            Ok(Some(_)) => {}
                            Ok(None) => break,
                            Err(e) => {
                                error = Some(e);
                                break;
                            }
                        }
                    }
                    ReadReport {
                        rows_read: r.rows_read(),
                        warnings: r.warnings().to_vec(),
                        error,
                    }
                }
            }
        });
        out.push(TableSurvey {
            ordinal,
            counts,
            read: report,
        });
    }
    out
}

/// Where the batches come from: the core reader, or (feature `test-probes`) a source that panics (ADR-59).
trait BatchSource: Send {
    fn next_batch(&mut self) -> Result<Option<RecordBatch>, Error>;
    fn rows_read(&self) -> u64;
    fn warnings(&self) -> Vec<Warning>;
}

impl BatchSource for TableBatches {
    fn next_batch(&mut self) -> Result<Option<RecordBatch>, Error> {
        TableBatches::next_batch(self)
    }
    fn rows_read(&self) -> u64 {
        TableBatches::rows_read(self)
    }
    fn warnings(&self) -> Vec<Warning> {
        TableBatches::warnings(self).to_vec()
    }
}

/// The state of one read, shared by `_Reader` and the Arrow C stream (ADR-55).
struct ReaderState {
    /// `None` after `close()` or a caught panic.
    source: Option<Box<dyn BatchSource>>,
    /// Rows of the batches handed out so far, and the warnings raised so far (kept after `close()`).
    rows: u64,
    warnings: Vec<Warning>,
    /// The source returned its end.
    finished: bool,
    /// The reader was exported to an Arrow stream: only the stream pulls from now on.
    exported: bool,
    /// A pull panicked: the reader is unusable (ADR-59 item 3).
    broken: bool,
}

type Shared = Arc<Mutex<ReaderState>>;

/// One pull under the lock, without the panic guard (the callers wrap it). `from_stream` is true for the Arrow
/// stream, which is the only puller once the reader was exported.
fn pull_raw(shared: &Shared, from_stream: bool) -> Result<Option<RecordBatch>, Error> {
    let mut g = lock(shared);
    let st = &mut *g;
    if st.broken {
        return Err(internal_error_later());
    }
    if st.exported && !from_stream {
        return Err(Error::new(
            ErrorCode::Config,
            "the reader was exported to an Arrow stream",
        ));
    }
    let Some(src) = st.source.as_mut() else {
        return if st.finished {
            Ok(None)
        } else {
            Err(Error::new(ErrorCode::Config, "the reader is closed"))
        };
    };
    let r = src.next_batch();
    st.rows = src.rows_read();
    st.warnings = src.warnings();
    if matches!(r, Ok(None)) {
        st.finished = true;
    }
    r
}

/// After a guarded pull: a caught panic drops the source and marks the reader unusable.
fn after_pull(shared: &Shared, r: &Result<Option<RecordBatch>, Error>) {
    if matches!(r, Err(e) if e.code == ErrorCode::Internal) {
        let mut g = lock(shared);
        g.broken = true;
        g.source = None;
    }
}

/// The Arrow stream's view of the shared state (ADR-55 item 1): the same pull, under the lock, inside the guard,
/// so that a panic never unwinds into the C caller.
struct SharedBatches {
    shared: Shared,
    schema: SchemaRef,
}

impl Iterator for SharedBatches {
    type Item = Result<RecordBatch, ArrowError>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = catch_panic(|| pull_raw(&self.shared, true));
        after_pull(&self.shared, &r);
        match r {
            Ok(Some(b)) => Some(Ok(b)),
            Ok(None) => None,
            Err(e) => Some(Err(ArrowError::ExternalError(Box::new(e)))),
        }
    }
}

impl RecordBatchReader for SharedBatches {
    fn schema(&self) -> SchemaRef {
        Arc::clone(&self.schema)
    }
}

/// The batches of one table.
#[pyclass(module = "oradmp._core", name = "_Reader", frozen)]
struct PyReader {
    shared: Shared,
    schema: SchemaRef,
    names: Arc<Names>,
}

impl PyReader {
    fn new(schema: SchemaRef, names: Arc<Names>, source: Box<dyn BatchSource>) -> Self {
        Self {
            shared: Arc::new(Mutex::new(ReaderState {
                source: Some(source),
                rows: 0,
                warnings: Vec::new(),
                finished: false,
                exported: false,
                broken: false,
            })),
            schema,
            names,
        }
    }
}

#[pymethods]
impl PyReader {
    fn schema(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(PySchema::new(Arc::clone(&self.schema))
            .into_pyarrow(py)?
            .unbind())
    }

    /// The next batch as a `pyarrow.RecordBatch`, or `None` at the end. The read runs with the GIL released.
    fn next_batch(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let r = py.detach(|| {
            let r = catch_panic(|| pull_raw(&self.shared, false));
            after_pull(&self.shared, &r);
            r
        });
        match r {
            Ok(Some(b)) => Ok(Some(PyRecordBatch::new(b).into_pyarrow(py)?.unbind())),
            Ok(None) => Ok(None),
            Err(e) => Err(to_pyerr(py, &e, Some(&self.names))),
        }
    }

    /// The warnings of the read so far; the counted ones are complete once the reader has returned `None` or an
    /// error (on the stream path: once the stream has returned its end or an error). The lock is taken with the
    /// GIL released and dropped before any Python object is built.
    fn warnings<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let ws: Vec<Warning> = py.detach(|| lock(&self.shared).warnings.clone());
        let out = PyList::empty(py);
        for w in &ws {
            out.append(warning_dict(py, w, Some(&self.names))?)?;
        }
        Ok(out)
    }

    fn rows_read(&self, py: Python<'_>) -> u64 {
        py.detach(|| lock(&self.shared).rows)
    }

    /// Hands the whole reader to an Arrow C stream capsule (`__arrow_c_stream__`). The stream shares this reader's
    /// state, so `rows_read` and `warnings` follow the consumer's reads; the reader is spent for iteration.
    #[pyo3(signature = (requested_schema=None))]
    fn arrow_c_stream<'py>(
        &self,
        py: Python<'py>,
        requested_schema: Option<Bound<'py, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let checked = py.detach(|| {
            let mut g = lock(&self.shared);
            if g.broken {
                return Err(internal_error_later());
            }
            if g.exported {
                return Err(Error::new(
                    ErrorCode::Config,
                    "the reader was exported to an Arrow stream",
                ));
            }
            if g.source.is_none() {
                return Err(Error::new(ErrorCode::Config, "the reader is closed"));
            }
            g.exported = true;
            Ok(())
        });
        checked.map_err(|e| to_pyerr(py, &e, Some(&self.names)))?;
        let stream = SharedBatches {
            shared: Arc::clone(&self.shared),
            schema: Arc::clone(&self.schema),
        };
        let reader = Py::new(py, PyRecordBatchReader::new(Box::new(stream)))?;
        reader
            .bind(py)
            .call_method1("__arrow_c_stream__", (requested_schema,))
    }

    /// Drops the core reader (the file handle) and keeps the counts; a pull afterwards is `E-CONFIG` unless the
    /// source had returned its end.
    fn close(&self, py: Python<'_>) {
        py.detach(|| lock(&self.shared).source = None);
    }
}

#[cfg(feature = "test-probes")]
mod probe {
    //! Test probes (feature `test-probes`, off in a release build; `check_dist.py` rejects a wheel that has them).
    #![allow(clippy::panic)]

    use super::*;

    pub fn boom<R>() -> Result<R, Error> {
        panic!("probe")
    }

    pub struct PanicSource;

    impl BatchSource for PanicSource {
        fn next_batch(&mut self) -> Result<Option<RecordBatch>, Error> {
            boom()
        }
        fn rows_read(&self) -> u64 {
            0
        }
        fn warnings(&self) -> Vec<Warning> {
            Vec::new()
        }
    }
}

/// `kind` is `dump` (an opened dump of `path`; call `_probe_panic()` on it), `reader` or `stream` (a reader whose
/// batch source panics; pull with `next_batch()` or export with `arrow_c_stream()`).
#[cfg(feature = "test-probes")]
#[pyfunction]
#[pyo3(signature = (kind, path=None))]
fn _probe_panic(py: Python<'_>, kind: &str, path: Option<String>) -> PyResult<Py<PyAny>> {
    match (kind, path) {
        ("dump", Some(p)) => {
            let d = py
                .detach(|| core_open(&p, oradmp_core::dump::Options::default()))
                .map_err(|e| to_pyerr(py, &e, None))?;
            Ok(Py::new(py, new_dump(d))?.into_any())
        }
        ("reader" | "stream", _) => {
            let r = PyReader::new(
                Arc::new(arrow_schema::Schema::empty()),
                Arc::new(Names::default()),
                Box::new(probe::PanicSource),
            );
            Ok(Py::new(py, r)?.into_any())
        }
        _ => Err(to_pyerr(
            py,
            &Error::new(ErrorCode::Config, "unknown probe kind"),
            None,
        )),
    }
}

fn new_dump(d: CoreDump) -> PyDump {
    PyDump {
        inner: Mutex::new(Some(d)),
        names: Mutex::new(Arc::new(Names::default())),
        broken: AtomicBool::new(false),
    }
}

/// Opens a dump (header only; GIL released).
#[pyfunction]
fn open(py: Python<'_>, path: PathBuf, options: &Bound<'_, PyDict>) -> PyResult<PyDump> {
    let o = options::open_options(options).map_err(|e| to_pyerr(py, &e, None))?;
    let d = py
        .detach(|| catch_panic(|| core_open(&path, o)))
        .map_err(|e| to_pyerr(py, &e, None))?;
    Ok(new_dump(d))
}

/// Checks the per-read options (the same parse as `read_table`); `E-CONFIG` on a bad value.
#[pyfunction]
fn check_read_options(py: Python<'_>, options: &Bound<'_, PyDict>) -> PyResult<()> {
    options::read_options(options)
        .map(|_| ())
        .map_err(|e| to_pyerr(py, &e, None))
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(open, m)?)?;
    m.add_function(wrap_pyfunction!(check_read_options, m)?)?;
    m.add_class::<PyDump>()?;
    m.add_class::<PyReader>()?;
    #[cfg(feature = "test-probes")]
    m.add_function(wrap_pyfunction!(_probe_panic, m)?)?;
    m.add(
        "ERROR_CODES",
        oradmp_core::ErrorCode::ALL
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>(),
    )?;
    m.add(
        "WARNING_CODES",
        oradmp_core::WarningCode::ALL
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>(),
    )?;
    m.add("CORE_VERSION", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
