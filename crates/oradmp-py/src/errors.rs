//! Typed exceptions: the class table lives in Python (`oradmp/_errors.py`); this module hands it the fields of a
//! core error or warning, with the names resolved through [`Names`].

use oradmp_core::{Error, Warning};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::names::Names;

/// The Python exception for a core error. Falls back to the error raised by the factory if it cannot be built.
pub fn to_pyerr(py: Python<'_>, e: &Error, names: Option<&Names>) -> PyErr {
    match build(py, e, names) {
        Ok(v) => v,
        Err(err) => err,
    }
}

fn build(py: Python<'_>, e: &Error, names: Option<&Names>) -> PyResult<PyErr> {
    let m = py.import("oradmp._errors")?;
    let table = match (e.table, names) {
        (Some(t), Some(n)) => n.table_name(t),
        _ => None,
    };
    let part = match (e.table, e.part, names) {
        (Some(t), Some(p), Some(n)) => n.part(t, p),
        _ => None,
    };
    let column = match (e.table, e.column, names) {
        (Some(t), Some(c), Some(n)) => n.column(t, e.part, c),
        _ => None,
    };
    let errno = e.source.as_ref().and_then(std::io::Error::raw_os_error);
    let inst = m.getattr("make_error")?.call1((
        e.code.as_str(),
        e.to_string(),
        e.offset,
        table,
        e.table,
        part,
        column,
        e.row,
        e.length,
        e.type_code,
        e.warning.map(oradmp_core::WarningCode::as_str),
        errno,
    ))?;
    Ok(PyErr::from_value(inst))
}

/// The error as an exception *instance* (not raised), for the places that report an error as data.
pub fn error_instance<'py>(
    py: Python<'py>,
    e: &Error,
    names: Option<&Names>,
) -> PyResult<Bound<'py, PyAny>> {
    Ok(to_pyerr(py, e, names)
        .into_value(py)
        .into_bound(py)
        .into_any())
}

/// A warning as a dict for `oradmp._api.DumpWarning`.
pub fn warning_dict<'py>(
    py: Python<'py>,
    w: &Warning,
    names: Option<&Names>,
) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("code", w.code.as_str())?;
    d.set_item("offset", w.offset)?;
    d.set_item(
        "table",
        match (w.table, names) {
            (Some(t), Some(n)) => n.table_name(t),
            _ => None,
        },
    )?;
    d.set_item("table_ordinal", w.table)?;
    d.set_item(
        "part",
        match (w.table, w.part, names) {
            (Some(t), Some(p), Some(n)) => n.part(t, p),
            _ => None,
        },
    )?;
    d.set_item(
        "column",
        match (w.table, w.column, names) {
            (Some(t), Some(c), Some(n)) => n.column(t, w.part, c),
            _ => None,
        },
    )?;
    d.set_item("row", w.row)?;
    d.set_item("length", w.length)?;
    d.set_item("message", w.message.as_ref())?;
    d.set_item("count", w.count)?;
    d.set_item("count_to_zero", w.count_to_zero)?;
    d.set_item("max_digits_dropped", w.max_digits_dropped)?;
    d.set_item("text", w.to_string())?;
    Ok(d)
}
