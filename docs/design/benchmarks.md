# Legacy EXP reader: benchmark baseline

Architect session 16, 2026-10-21 (new file; arch 4.3, 4.4; ADR-62 item 8, ADR-66); session 20, 2026-10-24 (baseline 2,
ADR-78); session 28, 2026-10-30 (Reviewer 6c's step 3b recorded); session 29, 2026-10-31 (the a4 `stream_memory.py` run). Sources: the T-15 note and Reviewer session 4 (baseline 1, now section 6); the T-15b and T-18b notes
(`implementer-notes.md`) and Reviewer session 6a, section 3 (`docs/review/review-06a-t13j-t18b.md`), which repeated
them (baseline 2, section 4). Nothing here is a promise to users (arch 4.3: "reported, not promised"); the README
carries no performance figure.

## Changes in session 29

- `tools/stream_memory.py` on the a4 release wheel (a4 checklist step 8, on the VM; which venv is not
  in the facts, the checklist names the 3.11 / pyarrow 15.0.2 one; the owner's statement, ADR-111): **PASS**, iterator RSS growth 0 kB, stream 20 kB, limit 10,240 kB. Within
  rule 4 and within baseline 1's range (growth 0 to 12 kB; the stream figure 8 kB above it, far below the limit).
  Added to 4.3. Baseline 2 unchanged.

## Changes in session 28

- Reviewer 6c's step 3b on the a4 candidate C = `4b9883c` (core of `1bb7785`; review 06c, "Step 3b") is the a4 "no
  regression" check: micro-benchmarks -0.3 to -6.3 % (all at the baseline or faster); `T_ROWS` and `T_BIG` throughput
  no regression under rule 3, two single "watch" figures (below); memory within rule 4 (highest end peaks 7,608 to
  7,848 kB, ratios 0.99 to 1.06). The `T_BIG` scan watch
  (-7.4 %, run 1 of 6) is the machine: the a3 core `460aef5`, run alone in the same sitting, gave -4.1 to -6.8 % against
  C's -0.8 to -2.7 % (ADR-106 item 2 done, no bisect). One `T_ROWS` direct "watch" (run 6 of 6: scan -15.0 %, read
  -12.2 %) was not confirmed by the other five runs (6c N-7; rule 3 needs a confirming run): recorded, no action; the
  next step 3b reads `T_ROWS` direct first.
- **Baseline 2 stays.** A baseline 3 from T-19 (m) and 6c would be allowed by rule 1, but 6c's same-sitting runs show the
  machine slower than at baseline 2 for unchanged code; a new baseline now would absorb that drift. Revisited at the
  next core change (ADR-109).
- `tools/stream_memory.py` runs once on the a4 release wheel (a4 checklist step 8); its figures are added to 4.3 from
  the a4 record.

## Changes in session 26

- T-19 (m) (archive `0520cd5`, code of `1bb7785`) measured under section 5: no regression; one rule-3 "watch", the
  `T_BIG` scan 4.8 to 6.8 % slower than baseline 2 in three runs (the `T_BIG` read within 1.2 %; memory ratios 1.01 to
  1.05). Baseline 2 stays; Reviewer 6c repeats step 3b on the a4 candidate (ADR-106 item 2). Figures in the T-19 (m)
  note of `implementer-notes.md`.
- Section 7: the public repository's runner has 4 CPUs (the private one had 2); the `extended` report of S was not
  pasted and is not listed. Even less comparable than before.

## Changes in session 23

- New section 7: the first CI benchmark report (`extended` job, `d081109`), for orientation only; never a baseline and
  never compared under section 5 (ADR-92 item 2).

## Changes in session 20

- **Baseline 2** (section 4): code of `460aef5` (= the core sources of the a3 candidate `d4d20ef`: `git diff --stat
  460aef5 d4d20ef -- crates/oradmp-core/src rust-toolchain.toml` is empty; root `Cargo.toml` differs only in the version
  line). Implementer figures from T-15b (throughput, memory, `0956df0`) and T-18b (micro-benchmarks, memory); Reviewer 6a
  repeated every one under rule 1: all within 9 % (rule 1 asks 15 %), so they replace baseline 1, which moves to section
  6 as history. First baseline with `T_BIG`.
- Section 2: Reviewer 6a's conditions, including the **idle Oracle XE container** that ran during the repeat (review
  06a N-8). Accepted, not repeated (ADR-78 item 2).
- Section 5: the baseline figure is the **slower** of the two columns; rule 3 gains a tighter limit for `T_BIG` (10 %;
  5 to 10 % "watch"), `T_ROWS` and the micro-benchmarks keep 20 %; rule 4 gets per-file peak limits from baseline 2.
- `tools/stream_memory.py` was not repeated by Reviewer 6a: its baseline 1 figures stay the reference (section 4.3).

## Changes in session 19

- `T_BIG` exists (O-17; `corpus/sql/15_big.sql`, 453,304,320 bytes, 5,000,000 rows). T-15b's figures (code of
  `460aef5`) are added in 4.2 and 4.3 as **candidates, not a baseline** (rule 1: no Reviewer repeat yet, and the
  micro-benchmarks were not run). Baseline section 4 unchanged. For 0.1.0a3 they count only as the "no regression"
  check of step 3b for throughput and memory (ADR-75 item 2); the micro-benchmarks come from T-18b (b); Reviewer 6a
  repeats all of them, then the Architect decides the new baseline.

## Changes in session 17

- 4.3: the memory test's resolution corrected with the T-13i runs (ADR-65 amendment, ADR-69 item 3): 8 bytes per row
  is at the limit on `T_ROWS`, not reliably caught. Baseline numbers unchanged. `T_BIG`: owner action O-17, task T-15b.

## Changes in session 16

- New file: the first baseline (code of `39cbb93`; T-14 and T-15 changed no file of `crates/oradmp-core/src/`), the
  conditions, the commands, the rule for a new baseline and what counts as a regression (ADR-66).

## 1. What the numbers mean

They are warm-cache, single-thread speeds of the release build on one VM: decode speed of the committed fixtures
(criterion), of the owner's 10 MB `T_ROWS` exports and of the 453 MB `T_BIG` export. Scan MB/s on a file in the page
cache is memory speed, not disk speed. They do **not** describe: a cold cache or a disk; other column mixes; Python
overhead; another machine.

## 2. Machine and conditions

| item | value |
|---|---|
| machine | the project VM: Intel Core i9-9900T @ 2.10 GHz, `nproc` 8, 11.5 GiB RAM ("12 GB"; Reviewer 6a read 11.7 GB), swap 6.5 GiB |
| OS | Linux x86_64 only (the memory figures read `/proc/self/status`; no other platform measured) |
| toolchain | `rustc 1.99.0 (b940084d7 2026-09-28)` (root `rust-toolchain.toml`), workspace release profile unchanged (no `lto`, default `codegen-units`) |
| criterion | `=0.8.2`, `default-features = false`, `features = ["cargo_bench_support"]` (dev-dependency of `oradmp-core`) |
| files | committed fixtures; `ORADMP_WORK` points read-only at the owner's work folder: `18c/t_rows_exp_18c.dmp` (10,035,200 bytes), `18c/t_rows_exp_direct_18c.dmp` (9,838,592 bytes), `18c/t_big_exp_18c.dmp` (453,304,320 bytes, SHA-256 = the owner's `SHA256SUMS_big`, `exp` log "T_BIG 5000000 rows exported"), all in the **page cache** (read just before) |
| machine rule | ADR-62 item 2: nothing else builds, tests or runs during a measurement; `CARGO_BUILD_JOBS=2`; each run starts when the 1-minute load average is below 0.5 |
| baseline 2, Implementer | T-15b (throughput, memory) and T-18b (micro-benchmarks, memory), 2026-10-24, alone on the VM (T-18b: load 0.14 before) |
| baseline 2, Reviewer 6a | 2026-10-24, release build of `d4d20ef` in a clean clone, own target folders; load 0.10 to 0.64 seen, each run started below 0.5; **an idle Oracle XE container ran the whole session** (54 processes, about 0 % CPU) and the VS Code server; review 06a N-8 |
| Python side | release wheel in a scratch venv (baseline 1: CPython 3.14 / 3.13.5, pyarrow 26.0.0) |

A run with an idle database container is acceptable when the note names it (ADR-78 item 2); a busy one (load above
0.5 at the start) is not.

## 3. Commands (from the repository root)

    # micro-benchmarks (committed fixtures; no ORADMP_WORK needed)
    CARGO_BUILD_JOBS=2 cargo bench -p oradmp-core --bench decode

    # throughput (median of 3 per file; prints, asserts nothing; skips without ORADMP_WORK)
    ORADMP_WORK=<work folder> CARGO_BUILD_JOBS=2 cargo nextest run --release --run-ignored only \
        -E 'binary(t15_throughput)' -j 1 --no-capture

    # memory test (child processes; measures every file present, then asserts the ratio of ADR-65 item 6)
    ORADMP_WORK=<work folder> CARGO_BUILD_JOBS=2 cargo nextest run --release --run-ignored only \
        -E 'binary(t15_memory)' -j 1 --no-capture

    # stream memory (release wheel installed in a scratch venv, run outside the checkout)
    python -I tools/stream_memory.py

`<work folder>` is the owner's path; it is never written into a committed file.

## 4. Baseline 2 (code of `460aef5` = core sources of `d4d20ef`; session 20)

"Implementer" = T-15b (throughput, memory) or T-18b (micro-benchmarks, memory); "Reviewer" = Reviewer 6a, same VM,
same day. "Diff" = Reviewer against Implementer. **"Baseline"** = the slower of the two (higher time, lower speed):
the figure rules 3 and 4 compare against.

### 4.1 Micro-benchmarks (`benches/decode.rs`, criterion mean; inputs as in 6.1)

| benchmark | Implementer (T-18b) | Reviewer 6a | diff | **baseline** | baseline 1 (6.1, "note") |
|---|---|---|---|---|---|
| `number/OraNumber::decode` | 1.4626 µs | 1.4645 µs | +0.1 % | **1.4645 µs** | 1.528 µs |
| `datetime/decode_date` | 145.18 ns | 141.31 ns | -2.7 % | **145.18 ns** | 151.1 ns |
| `datetime/decode_timestamp` | 262.61 ns | 254.71 ns | -3.0 % | **262.61 ns** | 260.2 ns |
| `text_cp1251/Charset::decode (Win1251)` | 173.67 µs | 170.28 µs | -2.0 % | **173.67 µs** | 188.5 µs |
| `read_table/R_STAT` | 1.0034 ms | 970.26 µs | -3.3 % | **1.0034 ms** | 1.0306 ms |
| `read_table/T_NUM` | 36.669 µs | 37.911 µs | +3.4 % | **37.911 µs** | 39.24 µs |
| `read_table/T_TEXT` | 39.685 µs | 37.606 µs | -5.2 % | **39.685 µs** | 41.86 µs |

Noise seen: -5.2 % to +3.4 % between the two sessions (criterion's own interval about +-2 %, 6.1). Every figure is
at or faster than baseline 1.

### 4.2 Throughput (`tests/t15_throughput.rs`, median of 3)

| file | quantity | Implementer (T-15b; runs) | Reviewer 6a (runs) | diff | **baseline** |
|---|---|---|---|---|---|
| `T_ROWS` conventional, 10.0 MB, 100,000 rows | scan | 809.1 MB/s | 846.1 MB/s (12.9, 11.9, 11.8 ms) | +4.6 % | **809.1 MB/s** |
| | full read, default options | 2.17 M rows/s | 2,359,224 rows/s, 236.4 MB/s | +8.7 % | **2.17 M rows/s** |
| `T_ROWS` direct path, 9.8 MB, 100,000 rows | scan | 886.7 MB/s | 888.6 MB/s | +0.2 % | **886.7 MB/s** |
| | full read, default options | 2.45 M rows/s | 2,456,653 rows/s, 241.3 MB/s | +0.3 % | **2.45 M rows/s** |
| `T_BIG` conventional, 453,304,320 bytes, 5,000,000 rows (row area 453,288,049 bytes) | scan | 785.3 MB/s (577.2, 566.8, 600.8 ms) | 779.0 MB/s (567.0, 593.7, 581.9 ms) | -0.8 % | **779.0 MB/s** |
| | full read, default options | 2,195,749 rows/s, 199.1 MB/s (2.305, 2.250, 2.277 s) | 2,232,222 rows/s, 202.4 MB/s (2.224, 2.240, 2.255 s) | +1.7 % | **2,195,749 rows/s** |

(T-15b's two unnamed `T_ROWS` figures per quantity read as conventional / direct, the baseline's order; Reviewer 6a's
run agrees with that reading.)

Noise seen: `T_ROWS` between sessions 0 to +8.7 % (baseline 1: up to 13 %), within one run's three samples up to
9 % (11.8 to 12.9 ms); `T_BIG` between sessions -0.8 % / +1.7 %, within one run's three samples at most 6.0 % (scan,
566.8 to 600.8 ms) and 2.4 % (read). A 0.6 s or 2.2 s run is much steadier than an 11 ms one.

### 4.3 Memory

Peak `VmHWM` in kB, stop / end, ratio = end / stop; limit 1.10 (ADR-65 item 6). Stop points: `T_ROWS` 12,288 rows,
`T_BIG` 102,400 rows; end 100,000 / 5,000,000 rows.

| file | Implementer T-15b (3 runs) | Implementer T-18b (1 run) | Reviewer 6a (3 runs) | **highest end peak** |
|---|---|---|---|---|
| `T_ROWS` conventional | ratios 1.027 to 1.052 | 7,376 / 7,752, 1.0510 | 7,592 / 7,748, 1.0205; 7,428 / 7,920, 1.0662; 7,556 / 7,784, 1.0302 | **7,920** |
| `T_ROWS` direct | ratios 1.026 to 1.037 | 7,516 / 7,744, 1.0303 | 7,512 / 7,844, 1.0442; 7,596 / 7,708, 1.0147; 7,528 / 7,748, 1.0292 | **7,844** |
| `T_BIG` | ratios 1.0016, 1.0252, 1.0137; end peaks 7,660 to 7,684 | 7,480 / 7,664, 1.0246 | 7,556 / 7,644, 1.0116; 7,588 / 7,812, 1.0295; 7,580 / 7,724, 1.0190 | **7,812** |

Noise seen: correct-reader ratios 1.0016 to 1.0662; end peaks 7,644 to 7,920 kB (3.6 % spread over all files and
runs).

What the test resolves (measured, mutation "retain b bytes per row" in `TableSink::row_end`): on `T_BIG` **1 byte per
row** fails every run (Implementer 1.78, 1.90, 1.96; Reviewer 1.743, 1.930, 1.749, and 1.778 with all files present);
4 bytes 3.69 to 3.74; 8 bytes 5.73 to 5.98. On `T_ROWS` 8 bytes per row is at the limit (1.10 to 1.11), 4 bytes passes
(6.3). Since T-18b every file present is measured before the assertion and the message lists each.

`tools/stream_memory.py` (Python iterator and `RecordBatchReader.from_stream`): not repeated for baseline 2; the
baseline 1 figures (6.3: growth 0 to 12 kB, limit 10 MiB) stay the reference until a run. **Session 29**: run once on
the a4 release wheel (owner, VM; venv not named in the facts): PASS, iterator growth 0 kB, stream 20 kB, limit
10,240 kB (ADR-111; the per-read VmRSS lines are not in the record).

## 5. Rules

1. **A new baseline** replaces section 4 only after a Reviewer-checked run: the Implementer runs the commands of
   section 3 on the VM under section 2's conditions (or names every difference) and records the numbers in its note;
   a Reviewer session repeats at least `read_table/R_STAT`, `number/OraNumber::decode`, both `T_ROWS` throughput
   rows, the `T_BIG` throughput rows and the memory test; the numbers agree when each repeated figure lies within
   15 % of the Implementer's (largest honest difference so far: 13 % in baseline 1, 8.7 % in baseline 2); then the
   Architect copies them here with both columns and moves the old baseline to section 6. A baseline from another
   machine (a CI runner, T-16) is a separate table, never compared with this one.
2. **When a measurement is due**: before a release whose commit changes any file of `crates/oradmp-core/src/` or
   the workspace's release profile or toolchain since the baseline commit (`460aef5`; release checklist step 3b);
   after a task that changes the row loop, the decoders or the I/O layer (its "Done when" says so).
3. **What a regression is** (throughput and micro-benchmarks), against the **baseline** column (the slower of the
   two), on the VM, under section 2's conditions, **confirmed** by a second run alone:
   - `T_ROWS` throughput and the micro-benchmarks: more than **20 %** slower; 10 to 20 % is "watch" (recorded, not a
     failure). Why: honest differences reach 13 % between sessions and 9 % within a run of an 11 ms scan; 20 % is
     1.5 times the largest, and arch 4.3's number.
   - `T_BIG` throughput (session 20): more than **10 %** slower; 5 to 10 % is "watch". Why: its honest differences are
     at most 1.7 % between sessions and 6.0 % between single samples of one run (4.2); 10 % is 1.7 times the largest.
     Revisited at the next baseline (two sessions are a small sample).
4. **What a memory regression is**: `t15_memory` failing its ratio (1.10) on any file; `stream_memory.py` printing
   FAIL; or a peak `VmHWM` to the end more than **15 %** above the file's highest end peak of 4.3: `T_ROWS`
   conventional above **9,108 kB** (7,920 x 1.15), direct above **9,021 kB** (7,844 x 1.15), `T_BIG` above **8,984
   kB** (7,812 x 1.15). Why 15 %: the end peaks of honest runs spread 3.6 %; 15 % is about four times that, about 1.2
   MB, roughly three extra batches of 4,096 `T_ROWS` rows.
5. A confirmed regression stops the release (checklist step 3b) until the Architect decides: fix, or accept with a
   new baseline under rule 1.

## 6. Baseline 1 (history; code of `39cbb93`, measured at `b3a5a7f`; replaced in session 20)

Kept for comparison; rules 3 and 4 no longer refer to it. Sections 6.1 to 6.3 are the former 4.1 to 4.3 as written in
sessions 16 to 19 (the T-15b "candidates" in them are now part of baseline 2).

"Note" = the T-15 run; "Reviewer" = Reviewer session 4's runs on the same VM another day. Spread = criterion's
confidence interval for the micro-benchmarks, the min-max of the Reviewer's runs for `T_ROWS`.

### 6.1 Micro-benchmarks (`benches/decode.rs`)

| benchmark | input | note | Reviewer | spread |
|---|---|---|---|---|
| `number/OraNumber::decode` | `18c/t_num_exp_18c.dmp` T_NUM, ID and V (65 fields / iteration) | 1.528 µs = 42.5 M fields/s (205 MiB/s) | 45.4 M fields/s | +-1.5 % |
| `datetime/decode_date` | `18c/t_date_exp_18c.dmp` T_DATE.D (10 fields) | 151.1 ns = 66.2 M fields/s | 71.4 M | +-1.5 % |
| `datetime/decode_timestamp` | `18c/t_ts_exp_18c.dmp` TS0, TS6, TS9 (15 fields) | 260.2 ns = 57.6 M fields/s | 61.6 M | +-1.5 % |
| `text_cp1251/Charset::decode (Win1251)` | `11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp` R_STAT TXT, CODE, NOTE (5,598 fields) | 188.5 µs = 29.7 M fields/s (186 MiB/s) | 33.5 M | +-2 % |
| `read_table/R_STAT` | 2,000 rows, 98,452 row bytes, whole read incl. open | 1.0306 ms = 1.94 M rows/s | 2.02 M | +-2 % |
| `read_table/T_NUM` | 33 rows (`on_value_error = Null`) | 39.24 µs = 841 K rows/s | 846 K | +-2 % |
| `read_table/T_TEXT` | 10 rows | 41.86 µs = 239 K rows/s | 259 K | +-2 % |

The small tables measure the fixed cost of `read_table` (open, builders), not decoding.

### 6.2 Throughput (`tests/t15_throughput.rs`, median of 3)

| file | quantity | note (runs) | Reviewer (4 runs, min to max) | spread seen |
|---|---|---|---|---|
| `T_ROWS` conventional, 10.0 MB, 100,000 rows | scan | 803 MB/s (14.4, 12.5, 12.1 ms) | 804 to 893 MB/s | about +-6 % |
| | full read, default options | 2.15 M rows/s, 216 MB/s of row area (47.4, 46.4, 43.9 ms) | 2.15 to 2.40 M rows/s | about +-6 % |
| `T_ROWS` direct path, 9.8 MB, 100,000 rows | scan | 830 MB/s (13.3, 11.1, 11.9 ms) | 899 to 929 MB/s | about +-5 % |
| | full read, default options | 2.32 M rows/s, 228 MB/s of row area (43.1, 41.8, 43.9 ms) | 2.35 to 2.52 M rows/s | about +-5 % |
| `T_BIG` | - | absent | absent | - |

**Candidates (T-15b, code of `460aef5`, one run, page cache; not a baseline until Reviewer 6a repeats them, rule 1):**

| file | quantity | T-15b (runs) | Reviewer | compared with the baseline |
|---|---|---|---|---|
| `T_BIG` conventional, 453,304,320 bytes, 5,000,000 rows (row area 453,288,049 bytes) | scan | 785.3 MB/s (577.2, 566.8, 600.8 ms) | not yet | no baseline for this file |
| | full read, default options | 2,195,749 rows/s, 199.1 MB/s of row area (2.305, 2.250, 2.277 s) | not yet | no baseline for this file |
| `T_ROWS` conventional / direct (same run) | scan | 809.1 / 886.7 MB/s | not yet | +0.8 % / +6.8 % (no regression, rule 3) |
| | full read | 2.17 / 2.45 M rows/s | not yet | +0.9 % / +5.6 % (no regression) |

The note gives two figures per `T_ROWS` quantity without naming the file; read here as conventional / direct (the
baseline's order). Either way all four are within +-7 % of the baseline.

The Reviewer's figures are 0 to 13 % above the note's: within one session a 3-run median of an 11 ms scan spreads
about 10 %, and the VM's load differs between sessions. They agree within the noise.

### 6.3 Memory

| measurement | note | Reviewer | limit |
|---|---|---|---|
| `t15_memory`, `T_ROWS` conventional, `batch_rows = 4096`: peak `VmHWM` stopping at 12,288 rows / to the end (100,000) | 7,504 / 7,848 kB, ratio 1.046 | ratios 1.028, 1.049 | ratio <= 1.10 (asserted) |
| the same, direct path | 7,648 / 7,820 kB, ratio 1.023 | ratios 1.037, 1.035 | ratio <= 1.10 (asserted) |
| `tools/stream_memory.py`, T_PART x 500, iterator: VmRSS after read 50 -> 500 | 60,704 -> 60,708 kB | 57,348 -> 57,348 kB | growth <= 10 MiB (PASS / FAIL) |
| the same, `pyarrow.RecordBatchReader.from_stream(r).read_all()` | 61,072 -> 61,072 kB | 57,720 -> 57,732 kB | growth <= 10 MiB |

What the memory test resolves (Reviewer 4 mutations): a reader that keeps every batch (ratio 2.37), 16 bytes per row
(1.40) and 8 bytes per row (1.16) fail; 4 bytes per row (1.06 to 1.08) passes. Session 17 (T-13i, three runs each):
correct reader 1.016 to 1.059; 4 bytes per row 1.039 to 1.083 (passes); 8 bytes per row 1.073 to 1.111, **failed in
1 of 3 runs**. So on `T_ROWS` 8 bytes per row is **at the limit**, not reliably caught: by arithmetic it adds 87,712 x
8 = 702 kB, 9.1 % of the about 7,700 kB peak, before allocator rounding. The peak is mostly the 7.5 MB process, so on
a 10 MB file growth below about 8 to 10 bytes per row is not reliably resolved; `T_BIG` would add 4.9 MB per byte
per row between its stop and its end (arithmetic, not measured: O-17, T-15b).

**Session 19, T-15b (candidates, three runs each, code of `460aef5`; peak `VmHWM` stop at 102,400 rows / end at
5,000,000):**

| case | `T_BIG` ratio | `T_ROWS` ratios | result |
|---|---|---|---|
| correct reader | 1.0016, 1.0252, 1.0137 (peaks 7,472 to 7,684 kB) | conventional 1.027 to 1.052; direct 1.026 to 1.037 | passes 3 of 3 |
| retain 1 byte per row | 1.96, 1.78, 1.90 | pass | fails 3 of 3 on `T_BIG` |
| retain 4 bytes per row | 3.69, 3.73, 3.74 | pass (1.02 to 1.06) | fails 3 of 3 on `T_BIG` |
| retain 8 bytes per row | 5.88, 5.98, 5.96 (with `T_ROWS` removed from `ORADMP_WORK`) | 1.1014, 1.1033, 1.1014 | fails 3 of 3; with all files present the test stops at `T_ROWS` (changed in T-18b (a): every file is measured before the assertion) |

So `T_BIG` resolves 1 byte per row (measured). The `T_BIG` end peak (7,660 to 7,684 kB) is below the `T_ROWS` baseline
peaks of rule 4; a `T_BIG` peak limit of its own is set only with the new baseline.

## 7. CI reports (not baselines; ADR-92 item 2)

GitHub's shared `ubuntu-24.04` runner (2 vCPU in the private repository, 4 CPUs in the public one since session 26, owner's reports), `cargo bench -p
oradmp-core --bench decode`, one run per dispatch, criterion's estimate as the summary prints it. Not measured under
section 2's conditions; section 5's rules do not apply; a difference from section 4 means nothing by itself.

| commit | `OraNumber::decode` | `decode_date` | `decode_timestamp` | `Charset::decode` Win1251 | `read_table` `R_STAT` | `T_NUM` | `T_TEXT` |
|---|---|---|---|---|---|---|---|
| `d081109` (session 23, owner's paste) | 2.10 us | 156 ns | 239 ns | 183 us | 943 us | 41.2 us | 41.4 us |
