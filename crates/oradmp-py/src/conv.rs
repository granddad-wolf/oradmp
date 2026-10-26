//! The core's metadata structures as plain Python dicts; `oradmp/_api.py` turns them into dataclasses.

use oradmp_core::charset::Charset;
use oradmp_core::dump::{
    Catalog, CatalogEnd, ColumnInfo, DumpInfo, ExportMode, PartInfo, TableInfo, TablePath,
};
use oradmp_core::exp::rows::{EndMarker, LobSummary, LongStats};
use oradmp_core::exp::table::PathKind;
use oradmp_core::survey::{
    CandidateCounts, CandidateSummary, ColumnCounts, NumberProfile, TableCounts,
};
use oradmp_core::types::map::TypeSource;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};

use crate::errors::error_instance;
use crate::names::Names;

fn charset_name(id: u16) -> Option<&'static str> {
    Charset::from_id(id).map(Charset::name)
}

pub fn info_dict<'py>(py: Python<'py>, i: &DumpInfo) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("format", i.format)?;
    d.set_item("version", &i.version)?;
    d.set_item("user", &i.user)?;
    d.set_item("user_possibly_damaged", i.user_possibly_damaged)?;
    d.set_item(
        "mode",
        match i.mode {
            ExportMode::Tables => "tables",
            ExportMode::Users => "users",
            ExportMode::Other => "other",
        },
    )?;
    d.set_item("record_length", i.record_length)?;
    d.set_item("direct_flag", i.direct_flag)?;
    d.set_item("client_charset_id", i.client_charset_id)?;
    d.set_item("client_charset", charset_name(i.client_charset_id))?;
    d.set_item("db_charset_id", i.db_charset_id)?;
    d.set_item("db_charset", charset_name(i.db_charset_id))?;
    d.set_item("national_charset_id", i.national_charset_id)?;
    d.set_item("national_charset", charset_name(i.national_charset_id))?;
    d.set_item("text_converted_at_export", i.text_converted_at_export)?;
    d.set_item("db_charset_override", &i.db_charset_override)?;
    d.set_item("client_charset_override", &i.client_charset_override)?;
    d.set_item("file_size", i.file_size)?;
    Ok(d)
}

fn lob_dict<'py>(py: Python<'py>, s: &LobSummary) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("values", s.values)?;
    d.set_item("short", s.short)?;
    d.set_item("empty", s.empty)?;
    d.set_item("long", s.long)?;
    d.set_item("stored_bytes", s.stored_bytes)?;
    d.set_item("largest", s.largest)?;
    Ok(d)
}

fn long_dict<'py>(py: Python<'py>, s: &LongStats) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("values", s.values)?;
    d.set_item("chained", s.chained)?;
    d.set_item("chunked", s.chunked)?;
    d.set_item("largest", s.largest)?;
    d.set_item("above_32000", s.above_32000)?;
    d.set_item("zero_pieces", s.zero_pieces)?;
    d.set_item("sizes_65521_to_65534", s.sizes_65521_to_65534)?;
    Ok(d)
}

fn column_dict<'py>(py: Python<'py>, c: &ColumnInfo) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("position", c.position)?;
    d.set_item("file_position", c.file_position)?;
    d.set_item("name", &c.name)?;
    d.set_item("name_possibly_damaged", c.name_possibly_damaged)?;
    d.set_item("type_code", c.type_code)?;
    d.set_item("oracle_type", c.oracle_type)?;
    d.set_item("max_bytes", c.max_bytes)?;
    d.set_item("charset_id", c.charset_id)?;
    d.set_item("form", c.form)?;
    d.set_item("declared", &c.declared_text)?;
    d.set_item("declared_known", c.declared.is_some())?;
    d.set_item("not_null", c.not_null)?;
    d.set_item("arrow_type", c.arrow_type.to_string())?;
    d.set_item(
        "type_source",
        match c.type_source {
            TypeSource::Declared => "declared",
            TypeSource::Default => "default",
            TypeSource::Override => "override",
        },
    )?;
    d.set_item(
        "lob",
        match &c.lob {
            Some(l) => Some(lob_dict(py, l)?),
            None => None,
        },
    )?;
    d.set_item(
        "long",
        match &c.long {
            Some(l) => Some(long_dict(py, l)?),
            None => None,
        },
    )?;
    Ok(d)
}

fn part_dict<'py>(py: Python<'py>, p: &PartInfo) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("partition", &p.partition)?;
    d.set_item("subpartition", &p.subpartition)?;
    d.set_item(
        "path",
        match p.path {
            PathKind::Conventional => "conventional",
            PathKind::Direct => "direct",
        },
    )?;
    d.set_item("num_rows", p.num_rows)?;
    d.set_item("rows_offset", p.rows_offset)?;
    d.set_item("rows_end_offset", p.rows_end_offset)?;
    d.set_item(
        "end_marker",
        match p.end_marker {
            EndMarker::FfFf => "ff-ff",
            EndMarker::LfOnly => "lf-only",
        },
    )?;
    d.set_item("file_order", &p.file_order)?;
    Ok(d)
}

pub fn table_dict<'py>(py: Python<'py>, t: &TableInfo) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("ordinal", t.ordinal)?;
    d.set_item("name", &t.name)?;
    d.set_item("name_raw", PyBytes::new(py, &t.name_raw))?;
    d.set_item("name_possibly_damaged", t.name_possibly_damaged)?;
    let cols = PyList::empty(py);
    for c in &t.columns {
        cols.append(column_dict(py, c)?)?;
    }
    d.set_item("columns", cols)?;
    d.set_item("ddl_fallback", t.ddl_fallback)?;
    d.set_item(
        "path",
        match t.path {
            TablePath::Conventional => "conventional",
            TablePath::Direct => "direct",
            TablePath::Mixed => "mixed",
        },
    )?;
    let parts = PyList::empty(py);
    for p in &t.parts {
        parts.append(part_dict(py, p)?)?;
    }
    d.set_item("parts", parts)?;
    d.set_item("partitioned", t.partitioned)?;
    d.set_item("subpartitioned", t.subpartitioned)?;
    d.set_item("num_rows", t.num_rows)?;
    d.set_item("possibly_incomplete", t.possibly_incomplete)?;
    d.set_item("ddl", &t.ddl)?;
    d.set_item("ddl_lossy", t.ddl_lossy)?;
    Ok(d)
}

pub fn catalog_dict<'py>(
    py: Python<'py>,
    c: &Catalog,
    names: &Names,
) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    let tables = PyList::empty(py);
    for t in &c.tables {
        tables.append(table_dict(py, t)?)?;
    }
    d.set_item("tables", tables)?;
    match &c.end {
        CatalogEnd::Complete => {
            d.set_item("end", "complete")?;
            d.set_item("stop_error", py.None())?;
        }
        CatalogEnd::Stopped(e) => {
            d.set_item("end", "stopped")?;
            d.set_item("stop_error", error_instance(py, e, Some(names))?)?;
        }
    }
    match &c.stopped_table {
        Some(s) => {
            let t = PyDict::new(py);
            t.set_item("ordinal", s.ordinal)?;
            t.set_item("name", &s.name)?;
            t.set_item("table_line_offset", s.table_line_offset)?;
            d.set_item("stopped_table", t)?;
        }
        None => d.set_item("stopped_table", py.None())?,
    }
    d.set_item("end_offset", c.end_offset)?;
    let ic = PyDict::new(py);
    ic.set_item("lines", c.item_counts.lines)?;
    ic.set_item("g1_fields", c.item_counts.g1_fields)?;
    let entries = PyList::empty(py);
    for e in &c.item_counts.per_entry {
        entries.append((e.text.as_str(), e.plain, e.marked, e.unmarked))?;
    }
    ic.set_item("per_entry", entries)?;
    d.set_item("item_counts", ic)?;
    Ok(d)
}

fn candidate_dict<'py>(py: Python<'py>, k: &CandidateCounts) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("label", &k.label)?;
    d.set_item("exact", k.exact)?;
    d.set_item("rounded", k.rounded)?;
    d.set_item("rounded_to_zero", k.rounded_to_zero)?;
    d.set_item("int_too_large", k.int_too_large)?;
    Ok(d)
}

fn profile_dict<'py>(py: Python<'py>, p: &NumberProfile) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("invalid", p.invalid)?;
    d.set_item("integers", p.integers)?;
    d.set_item("fit_int64", p.fit_int64)?;
    d.set_item("sig_39_or_40", p.sig_39_or_40)?;
    d.set_item("max_sig", p.max_sig)?;
    d.set_item("max_frac", p.max_frac)?;
    d.set_item("max_int", p.max_int)?;
    d.set_item("min_exp10", p.min_exp10)?;
    d.set_item("max_exp10", p.max_exp10)?;
    d.set_item("min_lead", p.min_lead)?;
    d.set_item("max_lead", p.max_lead)?;
    let c = PyList::empty(py);
    for k in &p.candidates {
        c.append(candidate_dict(py, k)?)?;
    }
    d.set_item("candidates", c)?;
    Ok(d)
}

fn counts_dict<'py>(py: Python<'py>, c: &ColumnCounts) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("position", c.position)?;
    d.set_item("type_code", c.type_code)?;
    d.set_item("values", c.values)?;
    d.set_item("nulls", c.nulls)?;
    d.set_item("years_below_1", c.years_below_1)?;
    d.set_item("months_beyond_32_bits", c.months_beyond_32_bits)?;
    d.set_item("no_decimal128_holds", c.no_decimal128_holds)?;
    d.set_item("lob_refused_short", c.lob_refused_short)?;
    d.set_item("lob_refused_long", c.lob_refused_long)?;
    d.set_item("len_min", c.len_min)?;
    d.set_item("len_max", c.len_max)?;
    d.set_item("len_total", c.len_total)?;
    d.set_item(
        "number",
        match &c.number {
            Some(p) => Some(profile_dict(py, p)?),
            None => None,
        },
    )?;
    Ok(d)
}

pub fn table_counts_dict<'py>(py: Python<'py>, t: &TableCounts) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("ordinal", t.ordinal)?;
    d.set_item("rows", t.rows)?;
    let cols = PyList::empty(py);
    for c in &t.columns {
        cols.append(counts_dict(py, c)?)?;
    }
    d.set_item("columns", cols)?;
    Ok(d)
}

pub fn summary_dict<'py>(py: Python<'py>, s: &CandidateSummary) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("label", &s.label)?;
    d.set_item("columns", s.columns)?;
    d.set_item("exact", s.exact)?;
    d.set_item("rounded", s.rounded)?;
    d.set_item("rounded_to_zero", s.rounded_to_zero)?;
    d.set_item("int_too_large", s.int_too_large)?;
    d.set_item("columns_needing_override", s.columns_needing_override)?;
    Ok(d)
}
