# docs/formats

Format specifications derived from fixtures. Owner: Analyst.

## Files

| File | Content |
|---|---|
| `exp.md` | legacy EXP layout |
| `exp_reader_rules.md` | extract of `exp.md` for the Architect: the rules a reader of table data needs, with status and fixtures (generated from `exp.md` section 11; `exp.md` is the reference) |
| `datapump.md` | Data Pump layout: **planned, not written** (Data Pump analysis is phase 2; the file does not exist yet) |
| `requests.md` | fixtures the Analyst needs, as SQL plus the file names wanted |
| `tools/` | small Python scripts used for the analysis |

## Conventions

Layout tables use these columns: **offset (hex) | length | meaning | evidence | status**.

Status values:

- `observed`: seen at this offset in one fixture
- `confirmed`: seen in at least two independent fixtures and consistent with `truth_*.txt`
- `hypothesis`: suggested by the data, not yet tested
- `unknown`: bytes that are not explained yet

Evidence is written `file@offset` and names the truth row it matches, for example
`t_frame_exp_18c.dmp@0x2a (truth_frame.txt row 1)`.

Every document starts with a table of the fixtures it cites: file, Oracle version, export mode,
character set, SHA-256 prefix.

## requests.md format

    ### R-001 short title
    - Why: which question this answers
    - SQL: the statements that create the table or change a value
    - File names wanted: e.g. t_probe1_exp_18c.dmp, t_probe1_dp_18c.dmp
    - Status: open | proposed | ready | generated | rejected | superseded | reserved

Status values used in `requests.md`:

- `open`: written by the Analyst, not yet turned into fixtures
- `proposed`: written (often copied from `docs/design/open-questions.md`), waiting for the owner to decide or run it
- `ready`: the owner can run it; no more analysis needed (for example R-040 until session 17)
- `generated`: the fixtures exist in `corpus/fixtures/`; a qualifier may follow, e.g. `generated and used`
  (R-045, the result was analysed) or `generated, part rejected` (R-031, Oracle refused part of the SQL)
- `rejected`: Oracle or the owner refused the request (or the named part of it)
- `superseded`: replaced by a later request, named in the status (e.g. R-011 by R-034)
- `reserved`: the number is taken but nothing was generated; do not reuse it (R-014)

The human turns an open request into fixtures (or asks for help doing so) and sets the status.
