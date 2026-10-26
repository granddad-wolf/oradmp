
## Decisions of 2026-10-16 (project owner)

- **Q-1 accepted: `round`.** A column declared `NUMBER` without precision is Decimal128(38, 10). A value that does not fit is rounded
  only below the 10th decimal (half away from zero); an integer part that does not fit stays an error; every rounded value is counted
  per column (`W-NUMBER-ROUNDED`, including how many became 0) and shown by the survey. `error` and `null` stay available per column
  or globally; `decimal256(76,38)` and `string` are per-column or per-dump options, not the default (ADR-14). With `strict=True`
  a rounded value is an error.
- **Q-7 accepted (ADR-25).** In a database with a single-byte character set, short CLOB values are decoded with the database character set;
  long CLOB values and any NCLOB are a value error (`E-LOB-UNCONFIRMED`) until the Analyst confirms the rule (request R-038); the opt-in
  `unconfirmed_lob="decode"` decodes them with a warning. This narrows the answer to Q-2 for this one case. The default of `on_value_error`
  for it is to stop with a clear message, not to write NULL.
