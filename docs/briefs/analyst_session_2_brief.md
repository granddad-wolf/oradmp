The analyst finished session 2 for legacy EXP on 18c. It reports that all 38 EXP fixtures parse end to end with no gaps and every field compared with the truth files matches. I have not re-checked the offsets myself; I only confirmed that the changes stay inside the analyst's directories and nothing is committed.

## What changed
- [docs/formats/exp.md](docs/formats/exp.md) is rewritten as the session 2 state, with session 1 findings kept.
- [docs/formats/requests.md](docs/formats/requests.md) has a note under R-001 to R-007 and new requests R-008 to R-011.
- [docs/research/hypotheses.md](docs/research/hypotheses.md) has `Result` lines only.
- Three new tools are in [docs/formats/tools/](docs/formats/tools/): `exp_parse.py`, `check_truth.py`, `header_fields.py`.

## Character set ids
The mapping from `truth_charsets.txt` is US7ASCII=1, WE8MSWIN1252=178, AL32UTF8=873, AL16UTF16=2000.

| Field | Result |
|---|---|
| Header 0x0001 and 0x0030 (u16 BE) | Confirmed: client character set (873 / 1 / 178 across the three `t_frame` variants) |
| Block 2 at 0x20a2 and 0x20a4 | Confirmed: both follow the client id; purpose unknown |
| Header 0x0032 (always 873) | Hypothesis: database character set; cannot be tested with one database |
| Header 0x0034 (always 2000) | Observed only |
| Column descriptor +4 | Confirmed: 873 for VARCHAR2/CHAR, 2000 for NVARCHAR2 |

Whether value bytes and DDL text are converted to the client set is still unknown, because `t_frame` is pure ASCII. R-008 asks for the fixture that would answer it.

## Other findings
- **RECORDLENGTH:** confirmed. The header text at 0x0024 holds it, the header is zero-filled to that length, and nothing is inserted at record boundaries. All 300 rows of `t_probe6` equal the truth file on both paths.
- **METRICET:** confirmed that it is not a per-table row count. In a one-table file it equals the row count, but in `multi_framing` (7, 22, 8 rows) every line says 8, the last table's count. No per-table count is stored anywhere.
- **Table order:** confirmed that tables follow the log order, and a table's section is byte-identical alone and inside a multi-table file.
- **Row order:** rows are not in ORDER BY order (`t_probe6` starts with R00065).
- **Length field:** confirmed as always 2 bytes little-endian, counting bytes, not characters (`t_text`, `t_len`). NULL is `fe ff`; length 254 is `fe 00`.
- **NUMBER, DATE, TIMESTAMP:** confirmed equal to `DUMP(col,16)` on both paths.
- **Hypotheses:** H-001 to H-006 and H-008 to H-013 are confirmed, H-007 is decided, and H-020 is refuted.

## Open problems
- **Empty table on the direct path:** the conventional path writes `ff ff` after the descriptors, but the direct path writes no end marker at all. The parser tool uses a heuristic there. This rests on one fixture each; R-009 asks for more.
- **Boundary inside a length field:** no fixture has a record boundary between the two bytes of a length field.
- **RAW:** `truth_raw.txt` contains an ORA-01489 error instead of data, in both 18c and 11g. RAW stays observed and H-017 is inconclusive; R-010 asks for a corrected truth file.
- **`ddl` text region:** it is not purely LF-terminated lines. Some statements have a u16 length prefix, and there is an unexplained `fd ff` at 0x2885.
- **Still unknown:** header byte 0, several header text lines, most of the two 102-byte blocks, and the meaning of `METRICST`, `METRICETG0` and `EXIT`.

## Fixture requests for you
- **R-008:** `t_text` exported with WE8MSWIN1252 and US7ASCII clients, both paths.
- **R-009:** an empty table between or before other tables, both paths.
- **R-010:** corrected `truth_raw.txt` for 18c and 11g.
- **R-011:** `VARCHAR2(n CHAR)` and `CHAR(n CHAR)` columns (low priority).

## Things to know
- The analyst filled Result lines for H-004 to H-006, H-009 to H-014 and H-017, which go beyond the ones the prompt names. Say if you want those reverted.
- `truth_len.txt` shows only the first 12 bytes of each value, so the long values were checked by length, those 12 bytes, and the rest being all `x`.
- [corpus/fixtures/README.md](corpus/fixtures/README.md) is out of date: it says 11g is "not generated yet" and that all fixtures use one character set.
- The analyst's first draft of the fixtures table in `exp.md` had placeholder checksums and four wrong offsets. It says it replaced them with real `sha256sum` output and re-checked the offsets; a spot check of that table is worthwhile.
- 11g was not analysed. The tools ran over the 30 11g EXP files without a violation, but nothing is marked confirmed for 11g.