The analyst finished every part of [analyst-06-exp-partial-stats.md](docs/prompts/analyst-06-exp-partial-stats.md) with nothing blocked. This summary is the analyst's own report; I only checked the working tree, which shows changes in `docs/formats/` alone and nothing committed.

## Main result

A column has a statistics item exactly when it has statistics at export time (`last_analyzed` not NULL). The analyst reports this as confirmed on 11g and 18c, conventional and direct path, with 104 column checks and no disagreement with `truth_stc.txt`.

- **A missing item leaves no bytes.** The next item follows directly, so the k-th item is not the k-th column; the column is named only inside the field.
- **The three causes look identical.** Column added after the gather, column not in `method_opt`, and deleted statistics all leave nothing in the file.
- **An all-NULL column still gets an item.** T_STC3.ALLNULL has one, so "almost always NULL" is no reason for a missing item.
- **A table item can stand alone.** T_STC5 has only the table item, and nothing in it says that no column item follows.
- **One `ANALCOMPUTE` line per table with a table item.** This now holds for 66 of 66 tables in 21 files.
- **Order within a table.** Rows, then `GRANT` if present, then the table item, column items, comments. This closes the open point about the place of `GRANT`.
- **One version difference.** The only 11g/18c difference in the new files is the ALLNULL field (`EPC := 2` on 11g, `EPC := 0` with empty arrays on 18c).

The reader consequence is new rule S7: expect 0 to n column items, never count them and never pair them by position.

## Real dump

The rule predicts exactly the counts you supplied: 5 table items plus 53 column items gives 58 `ANALSTATS TR`, and 5 `ANALCOMPUTE TR`. The cause for T3's two missing items (columns added after the last gather) stays an unverified hypothesis, because the file cannot tell the three causes apart.

## Header date text

The date is right-aligned and ends at 0x7f, with no leading zeros, so its length and start offset vary (21 to 23 bytes in the fixtures). The new `header_date.py` matches 157 of 157 files. The analyst thinks the survey tool's "not found" comes from expecting a fixed offset or zero-padded fields. A two-digit day is not in the fixtures, so the 24-byte case is a hypothesis.

## Still unknown

- The meaning of the last argument of the table item (0 for `ANALYZE`, 6 for `DBMS_STATS`).
- `blocks` and `avg_row_len` of the new table items, because `truth_stc.txt` does not hold them.
- "All column statistics deleted" against `ANALYZE ... FOR TABLE`: not in the fixtures.
- R-021 and R-022 are still open and not generated.

## For you to act on

- **New request R-023** (low priority, no new SQL): the T_STC tables exported with a US7ASCII client and with `statistics=none`, on both versions. The analyst notes that the prompt says other-client exports were "not in your request", while R-020 did ask for `stc_exp_us7ascii_<v>.dmp`. No reader rule waits for it.
- **Nothing is committed.** `docs/research/hypotheses.md` was not changed, because it holds no hypothesis about statistics items.

## Files

- [exp.md](docs/formats/exp.md): changed, about 300 lines added; sections 4.6, 9.4 and 11 carry most of it
- [exp_reader_rules.md](docs/formats/exp_reader_rules.md): new, 194 lines
- [requests.md](docs/formats/requests.md): R-020 marked generated, state lines for R-021 and R-022, new R-023
- [README.md](docs/formats/README.md): one row for the new file
- [stc_items.py](docs/formats/tools/stc_items.py): new, checks the R-020 files against `truth_stc.txt`
- [header_date.py](docs/formats/tools/header_date.py): new
- [check_truth.py](docs/formats/tools/check_truth.py): extended for T_STC1..5