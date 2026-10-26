# Project owner's decisions on the Architect's questions

Date: 2026-10-08. Source: the project owner, through the assistant. The Implementer follows this file where it
differs from `decisions.md`; the Architect folds it into `decisions.md` in its next session.

| Q | Decision |
|---|---|
| Q-1 NUMBER without precision | **Accepted**: Decimal128(38, 10), exact or an error, per-column override. Add one convenience option for the whole table or dump: `number_default` may also be `float64` or `string`. |
| Q-2 default strictness | **Accepted**: warn and continue by default, `strict=True` opt-in. Refinement: (1) a type that is not supported is always an error (ADR-12); (2) warnings are returned in the result (`warnings`) and printed by `survey`, never only logged; (3) an unknown introducing line read by rule G1 stays a warning (rule G agrees with list K on all fixtures, and a wrong G1 decision breaks the framing, which is fatal), but the number of such lines is part of `Catalog.item_counts`, so a skipped item cannot go unnoticed. |
| Q-3 minimum Python | **Open**: the owner will name the Python version of the target ETL hosts. Until then T-12 and T-13 are not started; T-1 to T-11 do not depend on it. |
| Q-4 real column types | **Open**: the owner sends the list of type codes of the real tables (counts only). |
| Q-5 documents | **Accepted**: `CLAUDE.md` says the Architect writes to `docs/design/`; `docs/scope.md` lists the five character sets of the fixtures and the TSTZ / TSLTZ limits of arch 5.1. The owner edits those two files. |

Housekeeping the owner asks for: the Analyst marks R-023 `generated`, regenerates the counts for 161 files (A-1) and
resolves A-2 to A-5 of `open-questions.md` in its next session.

## 2026-10-15: the T-3a choices (N-9), through the prompt of Architect session 4

Recorded by the Architect in session 4; folded into `decisions.md` as ADR-23.

| choice (N-9) | decision |
|---|---|
| A part not directly preceded by a `PARTITION` line: `Framing` if no `PARTITION` line was seen in the section, else `W-UNTESTED-LAYOUT` | **Accepted** (provisional; extended to subpartitions in ADR-21) |
| A path flag other than 0 or 1 is `UnsupportedLayout` | **Accepted** |
| `W-LONG-DIRECT` is raised by the walker | **Accepted** |
| `strict_e2a` is a placeholder until T-11 | **Accepted** |
| A long LOB value with total 0 is accepted | **Changed**: accepted silently only if a fixture contains one; otherwise a warning `W-UNTESTED-LAYOUT` (the Architect asks the Implementer which, AQ-1) |
| Q-1 (reopened part, default of `on_number_overflow`) | still **open** |

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

- **Q-3 accepted: Python 3.11 or later**, one abi3-py311 wheel per platform, pyarrow 15 or later.
## 2026-10-18
- Q-8: copyright holder is "Maksim Tkachenko". LICENSE-MIT line: "Copyright (c) 2026 Maksim Tkachenko".
- Q-9: RHEL 8+ / x86_64 only
