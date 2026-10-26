# Fixture requests (Analyst)

Format: see `docs/formats/README.md`. All requests are for Oracle 18c, user `CORPUS`, legacy `exp`
in table mode, unless stated otherwise. For each table I would like the conventional file, the
`direct=y` file, the log, and a truth file written the same way as `truth_frame.txt`.

Requests come from Analyst session 1 (`docs/formats/exp.md`). Each one names the open question
it answers.

Note for the human: some questions below may already be answerable with fixtures that exist in
`corpus/fixtures/18c/` but were outside the fixture list of session 1 (for example `t_empty`,
`multi_framing`, `t_text`). I did not open them. If you prefer, point a later session at those
files instead of generating new ones; R-005, R-006 and R-007 have no existing fixture as far as
the file names show.

### R-001 one row, one column, value changes
- Why: differential experiment for the smallest possible data region. Shows exactly which bytes
  depend on the value and on its length, and whether anything else in the file (header, trailer,
  `METRICET` number) changes with the data. Three files that differ in one thing each.
- SQL:

      CREATE TABLE t_probe1 (a VARCHAR2(10));
      INSERT INTO t_probe1 VALUES ('A');
      COMMIT;
      -- export -> t_probe1a_*
      UPDATE t_probe1 SET a = 'B';
      COMMIT;
      -- export -> t_probe1b_*   (same length, other value)
      UPDATE t_probe1 SET a = 'AB';
      COMMIT;
      -- export -> t_probe1c_*   (other length)

- File names wanted: `t_probe1a_exp_18c.dmp`, `t_probe1b_exp_18c.dmp`, `t_probe1c_exp_18c.dmp`,
  the same three with `_exp_direct_`, logs, `truth_probe1a.txt`, `truth_probe1b.txt`,
  `truth_probe1c.txt`.
- Status: open
- Still missing (session 13): a value-only differential (does anything outside the row data change with a value?); no fixture answers it, only this request's own `t_probe1a/b/c` files would. Value and length encodings are covered (`exp.md` 5.1, 5.2, 6).
- Analyst note (session 2): not generated. Lower priority now: lengths and values are covered by `t_len`, `t_num`, `t_text` (`exp.md` sections 5, 6). Still the only way to see whether anything outside the row data depends on a value.

### R-002 column name and declared length change
- Why: the column descriptor (`exp.md` section 4) is `01 00 0a 00 69 03 01 00` for all three
  columns of `t_frame`, so the field `0a 00` cannot be told apart from a constant. Changing only
  the declared length tests the hypothesis "+2 = declared length". Changing only the column name
  shows whether the name is stored anywhere besides the `CREATE TABLE` and `INSERT` text.
- SQL:

      CREATE TABLE t_probe2a (a VARCHAR2(10));
      INSERT INTO t_probe2a VALUES ('A');
      CREATE TABLE t_probe2b (bb VARCHAR2(10));      -- other column name
      INSERT INTO t_probe2b VALUES ('A');
      CREATE TABLE t_probe2c (a VARCHAR2(20));       -- other declared length
      INSERT INTO t_probe2c VALUES ('A');
      COMMIT;

  One export per table. (The table names differ by one character and have the same length, so
  the files stay aligned.)
- File names wanted: `t_probe2a_exp_18c.dmp`, `t_probe2b_exp_18c.dmp`, `t_probe2c_exp_18c.dmp`,
  the same with `_exp_direct_`, logs, `truth_probe2a.txt`, `truth_probe2b.txt`, `truth_probe2c.txt`.
- Status: open
- Still missing (session 13): a name-only differential (`t_probe2a` against `t_probe2b` of this request). Declared length is answered (`exp.md` 4.1, descriptor +2 = 10, 20, 100, 400; confirmed); names are known to stand also in LOB column entries and statistics fields (`exp.md` 14.6 `lob_exp_18c.dmp`@0x240b, 4.6), but no fixture shows that a plain table stores its column names nowhere else.
- Analyst note (session 2): not generated. The declared-length half is answered by existing fixtures: descriptor +2 is 10, 20, 100, 400 for VARCHAR2(10), (20), (100), (400) (`exp.md` section 4.1). The column-name half is still open, low priority.

### R-003 one row vs two rows
- Why: `exp.md` section 5.3: the `00 00` in the conventional file can be read as a row terminator
  or as a row prefix. Section 6: is the `7` in `METRICET 7` the row count? One row against two
  identical rows shows what one more row adds and whether the `METRICET` number changes.
- SQL:

      CREATE TABLE t_probe3 (a VARCHAR2(10));
      INSERT INTO t_probe3 VALUES ('A');
      COMMIT;
      -- export -> t_probe3a_*
      INSERT INTO t_probe3 VALUES ('A');
      COMMIT;
      -- export -> t_probe3b_*

- File names wanted: `t_probe3a_exp_18c.dmp`, `t_probe3b_exp_18c.dmp`, the same with
  `_exp_direct_`, logs, `truth_probe3a.txt`, `truth_probe3b.txt`.
- Status: rejected (session 14, owner: not needed: answered by `lob_exp_18c.dmp`, see the reasoning below). Re-checked in session 14: `lob_exp_18c.dmp`@0x2502 `02 00` (k = 2), @0x2504 `12 00` + 18 bytes `hello lob`, @0x2518 `0a 00` + `Hello blob` (`truth_lob.txt` row 1), row 2 @0x2524 `02 00 c1 03` `fe ff` `fe ff` (ID 2, C and B NULL, `truth_lob.txt` row 2) ends with `00 00` @0x252c (k = 0), row 3 @0x252e `02 00 c1 04`; agrees with `exp_reader_rules.md` rule L3
- Reasoning (session 13): no `t_probe3` pair exists. `METRICET` half answered (`exp.md` 7.1). The `00 00` half is answered in substance by the LOB fixtures of R-027: the u16 after a row's last field is k, the count of that row's LOB values that follow it (`lob_exp_18c.dmp`@0x2502 `02 00`, then row 1's values `hello lob` / `Hello blob` (`truth_lob.txt` row 1), then row 2 at @0x2524; `exp.md` 14.6 rule L3), so it belongs to the row before it. Left `open` for the owner to decide whether that closes R-003 (session 14: the owner decided it does; status `rejected` above).
- Corrected in session 15: the reasoning above cites @0x2502 as its only example, but at `lob_exp_18c.dmp`@0x2502 stands `02 00` (k = 2, closes **row 1**, values follow @0x2504 and @0x2518), not `00 00`; the `00 00` example is @0x252c (k = 0, closes **row 2**, which starts @0x2524 `02 00 c1 03 fe ff fe ff`); row 3 starts @0x252e `02 00 c1 04` (re-checked with `xxd -s 0x24f0 -l 0x50`; rule L3). Status and conclusion unchanged.
- Analyst note (session 2): not generated. The `METRICET` half is answered by `t_empty`, `multi_framing`, `multi_types`, `ddl` (`exp.md` section 7.1: not a per-table row count). The `00 00` half (row terminator or row prefix) is still open; both readings decode every fixture.

### R-004 single NUMBER column
- Why: the descriptor field at +0 (`01 00`) is suspected to be a type code and the fields at +4
  and +6 (`69 03`, `01 00`) to be character-set related. A NUMBER column should change +0 and,
  if the hypothesis is right, +4/+6. Also shows the framing of a non-character value in the
  simplest case, and whether the direct path writes it the same way.
- SQL:

      CREATE TABLE t_probe4 (n NUMBER);
      INSERT INTO t_probe4 VALUES (1);
      INSERT INTO t_probe4 VALUES (NULL);
      INSERT INTO t_probe4 VALUES (123);
      COMMIT;
      -- truth: SELECT NVL(TO_CHAR(n), '<NULL>') || '|' || NVL(DUMP(n, 16), '<NULL>') FROM t_probe4;

- File names wanted: `t_probe4_exp_18c.dmp`, `t_probe4_exp_direct_18c.dmp`, logs,
  `truth_probe4.txt`.
- Status: rejected (session 13, owner: not needed; type code +0 `t_num_exp_18c.dmp`@0x223a `02 00`, 4-byte NUMBER descriptor, +4/+6 only for types 1/96/112, NULL `fe ff` and NUMBER framing on both paths: `exp.md` 4.1, 5.3, 6.1, `truth_num.txt`, `truth_nullmix.txt`)
- Analyst note (session 2): not generated and no longer needed: `t_num`, `t_nullmix`, `t_date` answer it (`exp.md` sections 4.1, 5.3, 6.1). Suggest status `rejected`.

### R-005 same table, other export character set
- Why: the header holds `03 69 03 69 07 d0` (`exp.md` section 2), the binary blocks hold
  `03 69 07 d0` / `03 69 03 69` (section 3) and every column descriptor holds `69 03`. The
  hypothesis is that 0x0369 and 0x07d0 are character set ids. `truth_env.txt` and the logs name
  AL32UTF8 and AL16UTF16, but only one character set exists in the fixtures, so this cannot be
  tested. No new table is needed, only a second export of `t_frame` with another client character
  set.
- SQL: none (table `t_frame` as it is). Export `t_frame` again with the environment variable
  `NLS_LANG=AMERICAN_AMERICA.WE8MSWIN1252` set for `exp`, conventional and `direct=y`. Please keep
  the file name the same length as the existing one if possible, so that the header stays aligned.
- File names wanted: `t_frame_exp_w1252_18c.dmp`, `t_frame_exp_direct_w1252_18c.dmp`, logs
  (the log line "Export done in ... character set" is the truth for this experiment).
- Status: generated and used (`t_frame_exp_w1252_18c.dmp`, `t_frame_exp_direct_w1252_18c.dmp`, logs: "Export done in WE8MSWIN1252 character set"; also the US7ASCII pair; `exp.md` 2.1; follow-up R-008)
- Analyst note (session 2): fixtures received (`t_frame_exp_w1252_18c.dmp`, `t_frame_exp_us7ascii_18c.dmp` and the two direct files) and used: `exp.md` section 2.1. Suggest status `generated`. Follow-up: R-008.

### R-006 row data longer than one 8192-byte unit
- Why: both session 1 files are 2 x 8192 bytes and the data end long before 0x4000. What a reader
  meets at an 8192-byte boundary inside the row data (nothing, padding, a repeated header, a
  value split in two) is unknown, and every real dump will cross that boundary.
- SQL:

      CREATE TABLE t_probe6 (id VARCHAR2(10), v VARCHAR2(100));
      INSERT INTO t_probe6
        SELECT 'R' || LPAD(LEVEL, 5, '0'), RPAD('V' || LPAD(LEVEL, 5, '0') || '_', 100, 'x')
        FROM dual CONNECT BY LEVEL <= 300;
      COMMIT;
      -- truth: SELECT id || '|' || v FROM t_probe6 ORDER BY id;

  About 34 KB of row data, so at least four boundaries are crossed. Every value is readable and
  carries its row number.
- File names wanted: `t_probe6_exp_18c.dmp`, `t_probe6_exp_direct_18c.dmp`, logs,
  `truth_probe6.txt`.
- Status: generated and used (`t_probe6_exp_18c.dmp`, `t_probe6_exp_direct_18c.dmp`, `truth_probe6.txt` 300 rows; also `t_probe6_dp_18c.dmp`; `exp.md` 1.2)
- Analyst note (session 2): fixtures received (`t_probe6_exp_18c.dmp`, `t_probe6_exp_direct_18c.dmp`, `truth_probe6.txt`) and used: `exp.md` section 1.2. Suggest status `generated`.

### R-007 same table, other RECORDLENGTH
- Why: the header text line `8192` (`exp.md` section 2, CONV@0x0024) is suspected to be the unit
  size that also explains the zero fill to 0x2000 and the file length. One export with another
  record length tests this.
- SQL: none (table `t_frame` as it is). Export `t_frame` with `exp ... recordlength=4096`,
  conventional path only.
- File names wanted: `t_frame_exp_rl4096_18c.dmp`, log.
- Status: generated and used (`t_frame_exp_rl4096_18c.dmp`@0x0024 `4096`; also `t_frame_exp_rl16384_18c.dmp`; `exp.md` 1.1)
- Analyst note (session 2): fixtures received (`t_frame_exp_rl4096_18c.dmp`, `t_frame_exp_rl16384_18c.dmp`) and used: `exp.md` section 1.1. Suggest status `generated`.

## Requests from Analyst session 2

### R-008 non-ASCII text exported with another client character set
- Why: `exp.md` section 2.1, open question 1. With `NLS_LANG` set to US7ASCII or WE8MSWIN1252
  the header fields at 0x0001 / 0x0030 follow the client set, the logs say "possible charset
  conversion", but the column descriptor still holds 873 (AL32UTF8) and `t_frame` has only ASCII
  bytes, so the fixtures cannot show whether the value bytes (and the DDL text) are written in
  the client set or in the database set. A reader that trusts the wrong field would decode
  every non-ASCII value wrongly. `t_text` has the right content: row 3 (`Café Zürich ñ`) is 16
  bytes in AL32UTF8 and would be 13 bytes in WE8MSWIN1252; rows 2, 4, 5 cannot be represented
  in WE8MSWIN1252 at all, which shows what a lossy conversion looks like; the NVARCHAR2 column
  shows whether national data are touched.
- SQL: none (table `t_text` as it is). Export `t_text` with
  `NLS_LANG=AMERICAN_AMERICA.WE8MSWIN1252` and with `NLS_LANG=AMERICAN_AMERICA.US7ASCII`,
  conventional and `direct=y` (the direct path may behave differently here).
- File names wanted: `t_text_exp_w1252_18c.dmp`, `t_text_exp_direct_w1252_18c.dmp`,
  `t_text_exp_us7ascii_18c.dmp`, `t_text_exp_direct_us7ascii_18c.dmp`, logs. Truth: the existing
  `truth_text.txt` (database bytes) is enough; the log line "Export done in ..." is the rest.
- Status: generated
- Analyst note (session 3): fixtures received (the four files and their logs) and used: `exp.md` sections 2.1 and 6.3. Result: the table section is byte-identical for all three client sets; nothing is converted, so the expectation written above ("13 bytes in WE8MSWIN1252", "lossy conversion") was wrong. Suggest status `generated`. Follow-up for non-ASCII text outside the row data: R-012.
- Analyst note (session 4): status `generated` confirmed. Used again in session 4 on both versions: the 11g files `t_text_exp_w1252_11g.dmp`, `t_text_exp_us7ascii_11g.dmp` and the two direct files give the same result as 18c (`exp.md` section 13.7). The follow-up for text outside the row data is answered by R-012 and R-015: text items are converted to the client set (`exp.md` 14.2).

### R-009 empty table between two tables, direct path
- Why: `exp.md` section 5.4. In `t_empty_exp_direct_18c.dmp` the table without rows has no
  `ff ff` end marker (0x220a is already the LF before the trailer). The only fixture has the
  empty table last, so it is unknown what a reader meets when another table follows, and
  whether the rule "no `ff ff` after zero rows" is general on the direct path.
- SQL: none (existing tables). Export `tables=(t_frame,t_empty,t_len)` in this order,
  conventional and `direct=y`. A second pair with the empty table first,
  `tables=(t_empty,t_frame)`, would settle it completely.
- File names wanted: `multi_empty_exp_18c.dmp`, `multi_empty_exp_direct_18c.dmp`,
  `multi_empty2_exp_18c.dmp`, `multi_empty2_exp_direct_18c.dmp`, logs. No new truth needed.
- Status: generated
- Analyst note (session 3): fixtures received and used: `exp.md` section 5.4. Suggest status `generated`. Follow-up (empty table with its own index / comment): part of R-012.
- Analyst note (session 4): status `generated` confirmed. The 11g files (`multi_empty_exp_direct_11g.dmp`, `multi_empty2_exp_direct_11g.dmp`) show a version difference: on 11g the empty table on the direct path has `ff ff` (`exp.md` 13.2). Follow-up: R-017.

### R-010 truth file for `t_raw` is an error message
- Why: `corpus/fixtures/18c/truth_raw.txt` (and the 11g one) contains
  `ORA-01489: result of string concatenation is too long` instead of data, because
  `RAWTOHEX(r)` of a 2000-byte RAW is 4000 characters and the concatenation exceeds the
  VARCHAR2 limit. Without it RAW stays `observed` (`exp.md` section 6.5) and H-017 cannot be
  decided. This is a truth-file problem, not a new dump.
- SQL (suggestion for `corpus/sql/02_truth_fixtures.sql`; any form that stays below 4000 bytes
  per line and still pins down every byte is fine):

      SELECT id || '|' || NVL(TO_CHAR(UTL_RAW.LENGTH(r)), '<NULL>') || '|'
             || NVL(DUMP(r, 16, 1, 16), '<NULL>') || '|'
             || NVL(RAWTOHEX(STANDARD_HASH(r, 'SHA256')), '<NULL>')
        FROM t_raw ORDER BY id;

  (length, the first 16 bytes, and a SHA-256 of the whole value; `STANDARD_HASH` exists from
  12c on, so for 11g `DBMS_CRYPTO.HASH` or two `DUMP(r,16,pos,len)` slices would be needed.)
- File names wanted: a corrected `truth_raw.txt` in `18c/` and `11g/`; the dumps stay as they are.
- Status: generated
- Analyst note (session 3): corrected `truth_raw.txt` received (two-part format) and used: `exp.md` section 6.5, H-017. Suggest status `generated`.
- Analyst note (session 4): status `generated` confirmed; `truth_raw.txt` of `11g/` has the same format and all 7 rows equal it on both paths (`exp.md` 13.1).

### R-011 `VARCHAR2(n CHAR)` column
- Why: `exp.md` section 4.1. Descriptor +2 equals the declared length for byte-semantics
  columns only. For `VARCHAR2(10 CHAR)` in AL32UTF8 it might be 10, 40 or something else; a
  reader that uses this field as a buffer size or as a sanity limit needs to know. Low priority
  unless the real tables use CHAR semantics (question for the DBA).
- SQL:

      CREATE TABLE t_probe11 (b VARCHAR2(10 BYTE), c VARCHAR2(10 CHAR), k CHAR(3 CHAR));
      INSERT INTO t_probe11 VALUES ('abc', 'abc', 'abc');
      INSERT INTO t_probe11 VALUES (NULL, UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'));
      COMMIT;
      -- truth: SELECT DUMP(b,1016) || '|' || DUMP(c,1016) || '|' || DUMP(k,1016) FROM t_probe11;

- File names wanted: `t_probe11_exp_18c.dmp`, `t_probe11_exp_direct_18c.dmp`, logs,
  `truth_probe11.txt`.
- Status: superseded by R-034 (generated in session 9)
- Analyst note (session 9): answered by the R-034 files: `VARCHAR2(10 CHAR)` has descriptor length 40 in AL32UTF8
  (`t_probe11_exp_<v>.dmp`@0x2311 `01 00 28 00`), `CHAR(3 CHAR)` 12; the values are as this request expected
  (`exp.md` 4.1, 6.3; rule C6). Nothing more is needed here.
- Analyst note (session 8): still no fixture with `VARCHAR2(n CHAR)` in an AL32UTF8 database (`tools/collist.py`: the 25
  `VARCHAR2(n CHAR)` columns of the corpus are all R_TEXT.VC in the CL8MSWIN1251 database). Superseded by **R-034**, which
  keeps this SQL and adds the 4000 / 2000 byte caps; please generate R-034 instead.
- Analyst note (session 7): partly answered without this fixture: `CHAR(4 CHAR)` in the AL32UTF8 database
  (`nchar_exp_<v>.dmp`@0x228a, R-026a) has descriptor length 16 = 4 x 4, and its values are padded to 4
  characters. `VARCHAR2(n CHAR)` in AL32UTF8 is still not in the fixtures; the request stays open at low priority
  (expected: 4n; one export of `t_probe11` with the SQL above would confirm it).
- Analyst note (session 5): status `open` confirmed; still no `t_probe11*` file in `18c/` or `11g/`.
- Analyst note (session 4): the status says `generated`, but no `t_probe11*` file and no `truth_probe11.txt` is in `corpus/fixtures/18c/` or `11g/` (checked at the start of session 4), so nothing of this request could be used. What exists instead: column `VC VARCHAR2(20 CHAR)` of R_TEXT in `corpus/fixtures/11g-cp1251/` (R-015): descriptor +2 is 20 (`ru_tbl_exp_cp1251_11g.dmp`@0x0af2). That database is single-byte, so 20 characters and 20 bytes cannot be told apart; the question of this request (AL32UTF8 database) is still open (`exp.md` 14.4, open question 4). Please either generate the files or set the status back to `open`.

## Requests from Analyst session 3

### R-012 small owner-mode export with every statement kind a reader must skip
- Why: `exp.md` sections 4.2, 4.4, 5.4, open questions 1, 2, 6, 7. The framing of the text region
  (lines, length-prefixed fields, `fd ff` pieces) rests on one table (`ddl`), in table mode. Open:
  (a) is the list of lines that introduce a field complete (rule K) or is "second byte below
  0x20" (rule G) the safer test; (b) `fd ff` has one example; (c) no text field is longer than 255
  bytes; (d) an empty table on the direct path that has its own index / comment (rules E1 / E2);
  (e) whether text items are written in the client character set (all text in the fixtures is
  ASCII); (f) owner mode and the objects it adds (sequence, view, synonym, trigger, procedure,
  grant) were never seen. The work files `corpus/work/18c/schema_exp_18c.dmp` and
  `schema_exp_direct_18c.dmp` named in the session 3 prompt do not exist on the VM, and they
  would not be citable anyway. A separate small user keeps the export small (no `T_ROWS`).
- SQL (as a DBA user for the first two statements, then as `CORPUS_OWN`; no table has more than
  3 rows):

      CREATE USER corpus_own IDENTIFIED BY corpus_own DEFAULT TABLESPACE users QUOTA UNLIMITED ON users;
      GRANT CREATE SESSION, CREATE TABLE, CREATE VIEW, CREATE SEQUENCE, CREATE SYNONYM,
            CREATE TRIGGER, CREATE PROCEDURE TO corpus_own;
      -- connect corpus_own

      CREATE TABLE o_parent (id NUMBER(6) PRIMARY KEY, name VARCHAR2(30) NOT NULL);
      INSERT INTO o_parent VALUES (1, 'p1');
      INSERT INTO o_parent VALUES (2, 'p2');

      CREATE TABLE o_child (
        id        NUMBER(6) CONSTRAINT o_child_pk PRIMARY KEY,
        parent_id NUMBER(6) CONSTRAINT o_child_fk REFERENCES o_parent (id),
        status    VARCHAR2(10) DEFAULT 'NEW' NOT NULL
                  CONSTRAINT o_child_ck CHECK (status IN ('NEW', 'DONE')),
        note      VARCHAR2(100) DEFAULT 'n/a',
        -- a long CHECK condition (about 400 bytes): a second example of a value in pieces (fd ff)
        CONSTRAINT o_child_ck2 CHECK (
          note IS NULL OR note NOT IN (
            'forbidden value number 01 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
            'forbidden value number 02 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
            'forbidden value number 03 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
            'forbidden value number 04 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
            'forbidden value number 05 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'))
      );
      INSERT INTO o_child VALUES (1, 1, 'NEW', 'first');
      INSERT INTO o_child VALUES (2, 2, 'DONE', NULL);
      INSERT INTO o_child (id) VALUES (3);
      CREATE INDEX o_child_ix ON o_child (status, note);
      COMMENT ON TABLE o_child IS 'short ascii comment';
      BEGIN
        -- 300 bytes: the length needs the second byte of the u16 (0x012c)
        EXECUTE IMMEDIATE 'COMMENT ON COLUMN o_child.note IS ''' || RPAD('L300_', 300, 'x') || '''';
        -- an LF and a quote inside a comment
        EXECUTE IMMEDIATE 'COMMENT ON COLUMN o_child.status IS ''line1' || CHR(10) || 'line2 it''''s''';
        -- non-ASCII: Caf + e acute, euro sign, Cyrillic de (2-, 3-, 2-byte characters in AL32UTF8)
        EXECUTE IMMEDIATE 'COMMENT ON COLUMN o_child.parent_id IS ''Caf' || UNISTR('\00E9') || ' '
                          || UNISTR('\20AC') || ' ' || UNISTR('\0434') || '''';
      END;
      /

      -- empty table with its own primary key, index and comment
      CREATE TABLE o_empty (id NUMBER(6) CONSTRAINT o_empty_pk PRIMARY KEY, name VARCHAR2(20));
      CREATE INDEX o_empty_ix ON o_empty (name);
      COMMENT ON TABLE o_empty IS 'no rows';

      CREATE SEQUENCE o_seq START WITH 100 INCREMENT BY 5;
      CREATE VIEW o_view AS SELECT id, status FROM o_child WHERE status = 'NEW';
      CREATE SYNONYM o_syn FOR o_parent;
      CREATE OR REPLACE PROCEDURE o_proc (p_id IN NUMBER) IS
      BEGIN
        -- a comment line
        UPDATE o_child SET status = 'DONE' WHERE id = p_id;
      END;
      /
      CREATE OR REPLACE TRIGGER o_trg BEFORE INSERT ON o_child FOR EACH ROW
      BEGIN
        IF :NEW.status IS NULL THEN
          :NEW.status := 'NEW';
        END IF;
      END;
      /
      GRANT SELECT ON o_parent TO corpus;
      COMMIT;

      -- truth_own.txt (rows)
      SELECT 'O_PARENT|' || id || '|' || name FROM o_parent ORDER BY id;
      SELECT 'O_CHILD|' || id || '|' || NVL(TO_CHAR(parent_id), '<NULL>') || '|' || status || '|'
             || NVL(note, '<NULL>') FROM o_child ORDER BY id;
      SELECT 'O_EMPTY|count=' || COUNT(*) FROM o_empty;
      -- truth_own_ddl.txt (what the text items should contain)
      SELECT 'OBJECT|' || object_type || '|' || object_name FROM user_objects ORDER BY 1;
      SELECT 'TABCOMMENT|' || table_name || '|' || LENGTHB(comments) || '|' || DUMP(comments, 1016)
        FROM user_tab_comments WHERE comments IS NOT NULL ORDER BY table_name;
      SELECT 'COLCOMMENT|' || table_name || '.' || column_name || '|' || LENGTHB(comments) || '|'
             || DUMP(comments, 1016, 1, 40)
        FROM user_col_comments WHERE comments IS NOT NULL ORDER BY table_name, column_name;
      SELECT 'CONSTRAINT|' || constraint_name || '|' || constraint_type || '|'
             || NVL(TO_CHAR(LENGTHB(search_condition_vc)), '<NULL>')
        FROM user_constraints ORDER BY constraint_name;      -- search_condition_vc: 12c and later
      SELECT 'SOURCE|' || name || '|' || type || '|' || line || '|' || LENGTHB(text)
        FROM user_source ORDER BY name, type, line;

- Exports (all as `corpus_own`, default RECORDLENGTH):

      exp corpus_own/...@XEPDB1 owner=corpus_own file=own_exp_18c.dmp log=own_exp_18c.log
      exp corpus_own/...@XEPDB1 owner=corpus_own direct=y file=own_exp_direct_18c.dmp log=own_exp_direct_18c.log
      # the same objects in table mode: shows what owner mode adds, item by item
      exp corpus_own/...@XEPDB1 tables=(o_parent,o_child,o_empty) file=own_tables_exp_18c.dmp log=own_tables_exp_18c.log
      exp corpus_own/...@XEPDB1 tables=(o_parent,o_child,o_empty) direct=y file=own_tables_exp_direct_18c.dmp log=own_tables_exp_direct_18c.log
      # one more, conventional, with another client set: are the comments converted?
      NLS_LANG=AMERICAN_AMERICA.WE8MSWIN1252 exp corpus_own/...@XEPDB1 owner=corpus_own file=own_exp_w1252_18c.dmp log=own_exp_w1252_18c.log

- File names wanted: `own_exp_18c.dmp`, `own_exp_direct_18c.dmp`, `own_tables_exp_18c.dmp`,
  `own_tables_exp_direct_18c.dmp`, `own_exp_w1252_18c.dmp`, their logs, `truth_own.txt`,
  `truth_own_ddl.txt`. The same for 11g would be welcome (`search_condition_vc` does not exist
  there; leave that column out).
- Note: I could not run this SQL (no database access for agents); please tell me if a statement
  fails, and I adjust the request.
- Status: generated
- Analyst note (session 4): fixtures received on both versions (`own_exp_<v>.dmp`, `own_exp_direct_<v>.dmp`, `own_tables_exp_<v>.dmp`, `own_tables_exp_direct_<v>.dmp`, `own_exp_w1252_<v>.dmp`, logs, `truth_own.txt`, `truth_own_ddl.txt`, with the added procedure `O_PROC_LONG`) and used: `exp.md` sections 4.2, 4.4, 4.5, 5.4, 7.1, 14.2. Results: (a) the list of introducing lines was incomplete and rule G as written failed at the view; both corrected; (b) `fd ff` confirmed; (c) lengths above 255 are u16 LE; (d) the empty table with statements is read correctly by both rules, and 11g differs from 18c; (e) text items are converted to the client set; (f) owner mode described. `O_PROC_LONG` is written in pieces of at most 80 bytes, so "a plain field of 8192 bytes or more" is still untested: R-018.

### R-013 a record boundary between the two bytes of a length field and of a NULL marker
- Why: `exp.md` section 1.2, open question 3. `t_probe6` shows that values continue across a
  multiple of RECORDLENGTH and that a boundary can fall between a length field and its value
  (`t_probe6_exp_18c.dmp`@0x7ffe). No fixture puts the boundary **inside** a two-byte length field
  or inside `fe ff`. A reader that handles records separately would break exactly there.
- Design: all rows of a table have the same size, so the arithmetic does not depend on the order
  in which the rows are written (`t_probe6` showed that the order is not the insertion order).
  The tables copy the definitions of existing tables under names of the same length, so the
  table section starts at 0x210d and the first row starts where it starts in the existing
  fixture: T_RAW (ID NUMBER(3), R RAW(2000)): first row at 0x2249 on both paths
  (`t_raw_exp_18c.dmp`, `t_raw_exp_direct_18c.dmp`); T_LEN (ID NUMBER(4), A VARCHAR2(400)): first
  row at 0x2251 on both paths (`t_len_exp_18c.dmp`, `t_len_exp_direct_18c.dmp`).
  RECORDLENGTH 8192, boundary at 0x4000 = 16384.
- Arithmetic, variant 1 (length field). Row of T_RAW: ID field `02 00 c1 xx` = 4 bytes (ids 1 to
  8 have one base-100 digit: `c1 02` to `c1 09`), R field = 2 + L bytes, on the conventional path
  + 2 (`00 00`). The length field of R in row k (k = 0, 1, ...) starts at 0x2249 + k x S + 4. It
  must start at 0x3fff: k x S = 0x3fff - 0x224d = 7602 = 6 x 1267.
  - conventional: S = 4 + 2 + L + 2 = 1267, so L = 1259 (0x04eb). Row 7 (k = 6) starts at
    0x2249 + 7602 = 0x3ffb; its ID field is 0x3ffb-0x3ffe; its length field is `eb` at 0x3fff and
    `04` at 0x4000.
  - direct: S = 4 + 2 + L = 1267, so L = 1261 (0x04ed): `ed` at 0x3fff, `04` at 0x4000.
- Arithmetic, variant 2 (NULL marker). Row of T_LEN with ID NULL: `fe ff` = 2 bytes, A field =
  2 + L bytes, on the conventional path + 2. The marker of row k starts at 0x2251 + k x S. It
  must start at 0x3fff: k x S = 0x3fff - 0x2251 = 7598 = 29 x 262.
  - conventional: S = 2 + 2 + L + 2 = 262, so L = 256. Row 30 (k = 29) starts at 0x3fff: `fe` at
    0x3fff, `ff` at 0x4000.
  - direct: S = 2 + 2 + L = 262, so L = 258.
- SQL:

      -- variant 1, conventional path (export t_bn1 without direct=y; its direct file is a control)
      CREATE TABLE t_bn1 (id NUMBER(3), r RAW(2000));
      INSERT INTO t_bn1 SELECT LEVEL, HEXTORAW(RPAD('A' || LEVEL, 2 * 1259, 'A' || LEVEL))
        FROM dual CONNECT BY LEVEL <= 8;
      -- variant 1, direct path (export t_bn2 with direct=y; its conventional file is a control)
      CREATE TABLE t_bn2 (id NUMBER(3), r RAW(2000));
      INSERT INTO t_bn2 SELECT LEVEL, HEXTORAW(RPAD('B' || LEVEL, 2 * 1261, 'B' || LEVEL))
        FROM dual CONNECT BY LEVEL <= 8;
      -- variant 2, conventional path
      CREATE TABLE t_bn3 (id NUMBER(4), a VARCHAR2(400));
      INSERT INTO t_bn3 SELECT NULL, RPAD('N' || LPAD(LEVEL, 3, '0') || '_', 256, 'x')
        FROM dual CONNECT BY LEVEL <= 32;
      -- variant 2, direct path
      CREATE TABLE t_bn4 (id NUMBER(4), a VARCHAR2(400));
      INSERT INTO t_bn4 SELECT NULL, RPAD('N' || LPAD(LEVEL, 3, '0') || '_', 258, 'x')
        FROM dual CONNECT BY LEVEL <= 32;
      COMMIT;
      -- truth_bn1.txt ... truth_bn4.txt
      SELECT id || '|len=' || UTL_RAW.LENGTH(r) || '|' || RAWTOHEX(UTL_RAW.SUBSTR(r, 1, 4)) FROM t_bn1 ORDER BY id;
      SELECT id || '|len=' || UTL_RAW.LENGTH(r) || '|' || RAWTOHEX(UTL_RAW.SUBSTR(r, 1, 4)) FROM t_bn2 ORDER BY id;
      SELECT NVL(TO_CHAR(id), '<NULL>') || '|len=' || LENGTHB(a) || '|' || SUBSTR(a, 1, 5) FROM t_bn3 ORDER BY a;
      SELECT NVL(TO_CHAR(id), '<NULL>') || '|len=' || LENGTHB(a) || '|' || SUBSTR(a, 1, 5) FROM t_bn4 ORDER BY a;

  One table per dump file (table mode, `tables=t_bn1` and so on), each exported on both paths,
  default RECORDLENGTH 8192. The table names must keep 5 characters and the column definitions
  must stay exactly as written, otherwise the first row moves and the arithmetic is void.
- Assumption to check after generation: the `CREATE TABLE` text of a copy has the same length as
  that of T_RAW / T_LEN (same storage clause). `tools/exp_parse.py` must print `rows:` at 0x2249
  (t_bn1, t_bn2) and 0x2251 (t_bn3, t_bn4). If it prints another offset, tell me the offset and I
  recompute L.
- File names wanted: `t_bn1_exp_18c.dmp`, `t_bn1_exp_direct_18c.dmp`, the same for `t_bn2`,
  `t_bn3`, `t_bn4`, logs, `truth_bn1.txt` to `truth_bn4.txt`.
- Status: generated
- Analyst note (session 4): fixtures received on both versions (16 files, `truth_bn1.txt` to `truth_bn4.txt`) and used: `exp.md` section 1.2. The designed offsets hold (first rows at 0x2249 / 0x2251; `eb 04`, `ed 04`, `fe ff` across 0x3fff / 0x4000). Result: nothing is inserted. Note on the truth: it gives the length and the first 4 bytes / 5 characters only; the rest was compared with what `06_bn.sql` must produce.

## Requests added by the project owner after Analyst session 3 (not written by the Analyst)

### R-014 reserved: boundary tables with RECORDLENGTH 2048 and 1024
- Why: `exp.md` section 1.2 tested record boundaries only at RECORDLENGTH 8192 (`t_probe6`, R-013).
  A survey of a real conventional-path dump with RECORDLENGTH 2048 (14774 records, about 370000 rows,
  every row count equal to the export log) found nothing special at the 2048-byte boundaries: the data
  stream is continuous. R-015 fixtures also use 2048 (see there).
- Design: not designed. If a later session needs it: `t_probe6`, `t_bn1` to `t_bn4`, `t_text`, `t_raw`
  exported with `recordlength=2048` and `recordlength=1024`, both paths. Note that on the direct path
  the header of the R-015 files says 8192 although 2048 was asked for (R-015, F5).
- Status: reserved (not generated; do not reuse the number)

### R-015 a CL8MSWIN1251 database with Cyrillic data, several client character sets (11g)
- Why: the real dumps this project must read come from an Oracle 11.2.0.4 server whose database character
  set is CL8MSWIN1251, exported by an `exp` 10.2 client with `NLS_LANG` US7ASCII (log: "Export done in
  US7ASCII character set and AL16UTF16 NCHAR character set; server uses CL8MSWIN1251 character set (possible
  charset conversion)"). All fixtures R-001 to R-013 come from AL32UTF8 databases, so `exp.md` could only say
  what the client set does when the database set is AL32UTF8. Open questions it answers: which character set
  the row data, the statement text and the identifiers are in; what the header ids mean (0x0032);
  what exp does with names that the client set cannot represent.
- Design: a separate throwaway container `ora11ru` (image `gvenzl/oracle-xe:11`, 11.2.0.2). The image creates
  an AL32UTF8 database; `07_ru_charset.sql` switches the still empty database to CL8MSWIN1251
  (`ALTER DATABASE CHARACTER SET INTERNAL_USE`, unsupported, fine for a throwaway database). Checked after the
  switch: `NLS_CHARACTERSET` CL8MSWIN1251, `NLS_CHARSET_ID` 171, national AL16UTF16 (`truth_ru.txt`, `ENV|`).
  User CORPUS_RU:
  - `R_TEXT` (309 rows): `ID NUMBER(5)`, `V VARCHAR2(200)`, `C CHAR(12)`, `VC VARCHAR2(20 CHAR)`,
    `N NVARCHAR2(40)`, `LV VARCHAR2(4000)`, `D DATE`; Latin, Russian text, the whole alphabet, Ё ё № « »,
    typographic punctuation and euro, characters not in CL8MSWIN1251 (stored by the database as `?`),
    a row of NULLs, values of 3000, 2000 and 200 bytes, 300 deterministic filler rows;
  - `R_CLOB` (3 rows), a table with a Cyrillic name and Cyrillic column names, an index with a Cyrillic name,
    Cyrillic table and column comments, a CHECK constraint with a Cyrillic literal, a view, an index.
  - Truth: `truth_ru.txt`, the bytes the database holds (`DUMP(x,16)`), independent of any client set.
  Exports, all `recordlength=2048`: owner mode (`RUSERS`) with US7ASCII, CL8MSWIN1251, AL32UTF8, WE8MSWIN1252
  clients (conventional), US7ASCII and CL8MSWIN1251 (direct); table mode `tables=r_text,r_clob` with a
  US7ASCII client (as the real dumps); table mode `tables=r_text` with the four clients on both paths
  (the files a table-mode walker can read from end to end, because R_CLOB, type code 112, is not yet decoded).
- Files (folder `corpus/fixtures/11g-cp1251/`, own `SHA256SUMS`): `ru_exp_{us7ascii,cp1251,al32utf8,w1252}_11g`,
  `ru_exp_direct_{us7ascii,cp1251}_11g`, `ru_tables_exp_us7ascii_11g`,
  `ru_tbl_exp_{us7ascii,cp1251,al32utf8,w1252}_11g`, `ru_tbl_exp_direct_{us7ascii,cp1251,al32utf8,w1252}_11g`
  (`.dmp` and `.log` each), `truth_ru.txt`.
- Scripts: `corpus/scripts/make_ru.sh`, `corpus/scripts/make_ru_tables.sh`, `corpus/sql/07_ru_*.sql`.
- Known from the logs: with US7ASCII and WE8MSWIN1252 clients the table with the Cyrillic name is exported
  as `???_???` and fails (`EXP-00056` or `EXP-00008`, `ORA-00942`); the dump contains a table section that
  was cut short. These files are kept on purpose.
- Evaluated in: Analyst session 4, Part F.
- Status: generated
- Analyst note (session 4): used: `exp.md` sections 1.2, 13, 14. All 15 files walked; R_TEXT equals `truth_ru.txt` in every file (1854 fields each). Corrections to the description above: with the WE8MSWIN1252 client the failed table is written as `bf bf bf 5f bf bf bf` (in the dump and in the log), not as `???_???`; type code 112 has an 8-byte descriptor and the table is read by `tools/exp_parse.py` with hypotheses only (`exp.md` 14.6).

## Requests from Analyst session 4

All four are for legacy `exp`. R-016 and R-018 are wanted on 18c and 11g; file names with `_18c` /
`_11g` as usual (`<v>` below). I could not run any of this SQL; please tell me if a statement
fails, and I adjust the request.

### R-016 tables with table and column statistics (`ANALSTATS T?`)
- Why: `exp.md` section 14.5, open question 12. Every `ANALSTATS` / `ANALCOMPUTE` line in the 133
  fixtures has the first letter `I` (index statistics), because no fixture table has optimizer
  statistics. The project owner reports `ANALSTATS TR` on every table of the real dumps. What
  follows such a line is unknown: one plain field, several fields (one per column?), a field in
  pieces, or a field of 8192 bytes or more (a histogram). A reader that meets an unknown form here
  loses its place in every table section of a real dump, so this is the most urgent request.
- SQL (as `CORPUS`):

      CREATE TABLE t_stat1 (id NUMBER(6) PRIMARY KEY, v VARCHAR2(30), n NUMBER, d DATE);
      INSERT INTO t_stat1 SELECT LEVEL, 'v' || MOD(LEVEL, 7), MOD(LEVEL, 5), DATE '2020-01-01' + LEVEL
        FROM dual CONNECT BY LEVEL <= 50;
      CREATE INDEX t_stat1_ix ON t_stat1 (v);
      -- statistics with histograms on every column
      BEGIN
        DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STAT1', method_opt => 'FOR ALL COLUMNS SIZE 10', cascade => TRUE);
      END;
      /
      -- the same data, statistics without histograms
      CREATE TABLE t_stat2 AS SELECT * FROM t_stat1;
      BEGIN
        DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STAT2', method_opt => 'FOR ALL COLUMNS SIZE 1');
      END;
      /
      -- old-style statistics
      CREATE TABLE t_stat3 AS SELECT * FROM t_stat1;
      ANALYZE TABLE t_stat3 COMPUTE STATISTICS;
      -- an empty table with statistics
      CREATE TABLE t_stat4 (id NUMBER(6), v VARCHAR2(30));
      BEGIN
        DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STAT4');
      END;
      /
      COMMIT;
      -- truth_stat.txt
      SELECT 'ROW|' || id || '|' || v || '|' || n || '|' || TO_CHAR(d, 'YYYY-MM-DD') FROM t_stat1 ORDER BY id;
      SELECT 'TABSTAT|' || table_name || '|' || num_rows || '|' || blocks || '|' || avg_row_len
        FROM user_tab_statistics WHERE table_name LIKE 'T_STAT_' ORDER BY 1;
      SELECT 'COLSTAT|' || table_name || '|' || column_name || '|' || num_distinct || '|' || num_buckets || '|' || histogram
        FROM user_tab_col_statistics WHERE table_name LIKE 'T_STAT_' ORDER BY 1;

- Exports (table mode, `tables=(t_stat1,t_stat2,t_stat3,t_stat4)`, default `statistics=estimate`):

      exp ... file=stat_exp_<v>.dmp                      # client = database set: I expect `ANALSTATS TS`
      exp ... direct=y file=stat_exp_direct_<v>.dmp
      NLS_LANG=AMERICAN_AMERICA.US7ASCII exp ... file=stat_exp_us7ascii_<v>.dmp   # I expect `ANALSTATS TR` and EXP-00091
      exp ... statistics=none file=stat_exp_none_<v>.dmp   # control: which lines disappear

  ("I expect" is a hypothesis from `exp.md` 14.5, not a fact.) If the CL8MSWIN1251 container still
  exists: the same four tables there, exported with the US7ASCII client
  (`ru_stat_exp_us7ascii_11g.dmp`), which is the combination of the real dumps.
- File names wanted: `stat_exp_<v>.dmp`, `stat_exp_direct_<v>.dmp`, `stat_exp_us7ascii_<v>.dmp`,
  `stat_exp_none_<v>.dmp`, logs, `truth_stat.txt`; optional `ru_stat_exp_us7ascii_11g.dmp` + log.
- Part b (made by the project owner, CL8MSWIN1251 database, 2026-10-07; scripts `make_ru_stats.sh`,
  `08_ru_stats_objects.sql`, `08_ru_stats_truth.sql`): tables R_STAT (2000 rows, histograms: frequency GRP,
  height-balanced TXT/DT/AMT, none ID/CODE/NOTE; Cyrillic text in TXT; indexes R_STAT_PK, R_STAT_TXT_IX,
  R_STAT_GD_IX) and R_TEXT (statistics without histograms). Table mode `tables=r_stat,r_text`, recordlength
  2048, in folder `corpus/fixtures/11g-cp1251/`: `ru_stat_exp_{us7ascii,cp1251}_{est,cmp}_11g`,
  `ru_stat_exp_direct_{us7ascii,cp1251}_{est,cmp}_11g`, `ru_stat_exp_al32utf8_est_11g`,
  `ru_stat_exp_us7ascii_none_11g` (.dmp and .log), `truth_stat.txt` (TAB, COL, HIST, IND lines).
  Result in the logs: US7ASCII and AL32UTF8 clients give EXP-00091 once per table and per index (6 per run);
  the CL8MSWIN1251 client and statistics=none give none. Part a (t_stat1..4, 18c and 11g, AL32UTF8) is still open.
- Status: generated (part a and part b)
- Analyst note (session 5): both parts received and used: `exp.md` sections 4.6, 13.9, 14.10 and rules S1 to S6
  of section 11. Part a: `stat_exp_<v>`, `stat_exp_direct_<v>`, `stat_exp_us7ascii_<v>`, `stat_exp_none_<v>` and
  the extra `stat_exp_compute_<v>` on both versions, `truth_stat.txt` with `ROW|`, `TABSTAT|`, `COLSTAT|`, `TAB|`,
  `COL|`, `HIST|`, `IND|`. Part b: the ten `ru_stat_exp_*` files and `truth_stat.txt` of `11g-cp1251/` (at the time
  of the session these part b files were in the folder but not yet committed to git). Result: one field after every
  `ANALSTATS` line (table and index: plain; column: pieces after `fd ff`), one table item plus one item per column,
  `ANALCOMPUTE T?` is a line; the fields are PL/SQL text and every number equals `truth_stat.txt` (3108 of 3116
  comparisons; the 8 others are the 11g `EPC` of the table without rows). My expectation "`TS` with the database
  client, `TR` and `EXP-00091` with US7ASCII" held. Remarks on the truth: (1) only T_STAT1 has `ROW|` lines; T_STAT2
  and T_STAT3 were compared with them because they are `CREATE TABLE AS SELECT` copies; (2) part b has no rows of
  R_STAT, which I compared with the SQL loop; (3) no table of either part has a column without statistics, which is
  what the counts of the real dump need (R-020).

### R-017 18c: an empty table that has a segment, on the direct path
- Why: `exp.md` sections 5.4, 13.2. On 18c a table without rows has no `ff ff` on the direct
  path; on 11g it has. In all 11 cases the marker is absent exactly when the `CREATE TABLE` text
  has no `STORAGE(...)` clause, which suggests that the 18c tables have no segment (deferred
  segment creation) and that the direct path writes nothing for such a table. The fixtures cannot
  separate "version" from "segment". One experiment on 18c does.
- SQL (as `CORPUS`, 18c only; the column definitions are those of T_EMPTY):

      -- empty, segment created at once
      CREATE TABLE t_seg0 (id NUMBER, name VARCHAR2(20), d DATE) SEGMENT CREATION IMMEDIATE;
      -- empty now, but a row was inserted and deleted: the segment exists
      CREATE TABLE t_seg1 (id NUMBER, name VARCHAR2(20), d DATE);
      INSERT INTO t_seg1 VALUES (1, 'x', DATE '2020-01-01');
      COMMIT;
      DELETE FROM t_seg1;
      COMMIT;
      -- control: empty, no segment (as T_EMPTY)
      CREATE TABLE t_seg2 (id NUMBER, name VARCHAR2(20), d DATE) SEGMENT CREATION DEFERRED;
      -- truth_seg.txt
      SELECT t.table_name || '|segment_created=' || t.segment_created || '|segments='
             || (SELECT COUNT(*) FROM user_segments s WHERE s.segment_name = t.table_name)
             || '|rows=0'
        FROM user_tables t WHERE t.table_name LIKE 'T_SEG_' ORDER BY 1;

- Exports: `tables=(t_seg0,t_seg1,t_seg2)`, conventional and `direct=y`.
- Prediction to test (hypothesis): in `seg_exp_direct_18c.dmp` the `CREATE TABLE` lines of T_SEG0
  and T_SEG1 carry a `STORAGE(...)` clause and `ff ff 0a` follows their path flag; T_SEG2 has no
  clause and `0a` follows its path flag. If T_SEG0 and T_SEG1 have no `ff ff` either, the
  difference is one of the version and the hypothesis is refuted.
- File names wanted: `seg_exp_18c.dmp`, `seg_exp_direct_18c.dmp`, logs, `truth_seg.txt`.
- Status: open
- Status (session 17): **generated and used** (owner, 2026-10-25, `corpus/scripts/make_pending.sh`): `18c/seg_exp_18c.dmp`,
  `18c/seg_exp_direct_18c.dmp`, `seg_exp(_direct)_18c.log`, `18c/truth_seg.txt`. Result: the prediction holds. T_SEG0 and
  T_SEG1 (`segment_created=YES`, 1 segment) have a `STORAGE(...)` clause and `ff ff 0a` after the direct path flag
  (`seg_exp_direct_18c.dmp`@0x226a, @0x23ca); T_SEG2 (`NO`, 0 segments) has no clause and `0a` (@0x24c7). The conventional
  file has `ff ff 0a` for all three. The hypothesis "the marker depends on the segment, not on the version" is
  **confirmed for 18c** (`exp.md` 5.4, 13.2). 11g with a table without a segment: R-049.

### R-018 framing cases the fixtures do not contain: long view, long plain field, more object kinds
- Why: `exp.md` section 4.2 ("Session 4: which rule survives"), open question 6. The two corrected
  item rules agree on all 133 files and each can fail on input that no fixture has: (a) the text
  of a view is written as pieces without a marker, and every view in the fixtures has one piece
  (41 to 51 bytes): is a long view split, and where; (b) the longest plain field is 302 bytes: a
  plain field of 8192 bytes or more has a second length byte of 0x20 or more and would be taken for
  a text line by rule G; (c) the list of lines that introduce a field is known for tables,
  sequences, synonyms, views, procedures and triggers only: functions, packages, package bodies
  and types probably add more introducing lines, and rule K reads their source as lines without
  noticing.
- SQL (as a small separate user, e.g. `CORPUS_FRM`, created like `CORPUS_OWN` plus the privilege
  `CREATE TYPE`; owner mode keeps the file small):

      CREATE TABLE f_base (id NUMBER(6) PRIMARY KEY, a VARCHAR2(100), b VARCHAR2(100));
      INSERT INTO f_base VALUES (1, 'a1', 'b1');
      COMMIT;
      -- (b) a DEFAULT expression of about 10000 bytes and a comment of 4000 bytes (the maximum)
      DECLARE
        e VARCHAR2(32767) := '''d''';
      BEGIN
        FOR i IN 1 .. 1000 LOOP
          e := e || ' || ''' || LPAD(i, 4, '0') || '''';       -- 10 bytes per step, 10003 in all
        END LOOP;
        EXECUTE IMMEDIATE 'CREATE TABLE f_def (id NUMBER(6), c VARCHAR2(4000) DEFAULT ' || e || ')';
        EXECUTE IMMEDIATE 'COMMENT ON TABLE f_def IS ''' || RPAD('C4000_', 4000, 'c') || '''';
      END;
      /
      INSERT INTO f_def (id) VALUES (1);
      COMMIT;
      -- (a) views: 300 bytes with line breaks; about 9000 bytes; one of 266 bytes (0x010a: the low
      --     length byte is 0x0a, the value that the corrected rule G could confuse with LF)
      DECLARE
        q VARCHAR2(32767);
      BEGIN
        EXECUTE IMMEDIATE 'CREATE VIEW f_view300 AS SELECT id,' || CHR(10) || '  a,' || CHR(10) || '  b' || CHR(10)
                          || 'FROM f_base WHERE a <> ''' || RPAD('x', 240, 'x') || '''';
        q := 'SELECT id FROM f_base WHERE a NOT IN (''v0000''';
        FOR i IN 1 .. 1000 LOOP
          q := q || ', ''v' || LPAD(i, 4, '0') || '''';        -- 9 bytes per step, about 9000 in all
        END LOOP;
        EXECUTE IMMEDIATE 'CREATE VIEW f_view9000 AS ' || q || ')';
        q := 'SELECT id FROM f_base WHERE a <> ''';
        EXECUTE IMMEDIATE 'CREATE VIEW f_view266 AS ' || q || RPAD('y', 266 - LENGTH(q) - 1, 'y') || '''';
      END;
      /
      -- (c) more object kinds
      CREATE OR REPLACE FUNCTION f_func (p IN NUMBER) RETURN NUMBER IS
      BEGIN
        RETURN p + 1;
      END;
      /
      CREATE OR REPLACE PACKAGE f_pkg IS
        FUNCTION one RETURN NUMBER;
      END f_pkg;
      /
      CREATE OR REPLACE PACKAGE BODY f_pkg IS
        FUNCTION one RETURN NUMBER IS
        BEGIN
          RETURN 1;
        END;
      END f_pkg;
      /
      CREATE OR REPLACE TYPE f_type AS OBJECT (x NUMBER, y VARCHAR2(10));
      /
      CREATE OR REPLACE TRIGGER f_trg1 BEFORE INSERT ON f_base FOR EACH ROW WHEN (NEW.a IS NULL)
      BEGIN
        :NEW.a := 'n/a';
      END;
      /
      CREATE OR REPLACE TRIGGER f_trg2 AFTER UPDATE ON f_base
      BEGIN
        NULL;
      END;
      /
      -- truth_frm.txt
      SELECT 'VIEW|' || view_name || '|' || text_length FROM user_views ORDER BY 1;
      SELECT 'DEFAULT|' || table_name || '.' || column_name || '|' || default_length FROM user_tab_columns
        WHERE default_length IS NOT NULL ORDER BY 1;
      SELECT 'TABCOMMENT|' || table_name || '|' || LENGTHB(comments) FROM user_tab_comments
        WHERE comments IS NOT NULL ORDER BY 1;
      SELECT 'SOURCE|' || name || '|' || type || '|' || line || '|' || LENGTHB(text) FROM user_source
        ORDER BY name, type, line;
      SELECT 'OBJECT|' || object_type || '|' || object_name FROM user_objects ORDER BY 1;

- Exports: `owner=corpus_frm`, conventional and `direct=y`, AL32UTF8 client.
- Arithmetic to check after generation: `truth_frm.txt` should show `VIEW|F_VIEW266|266` (the text
  from `SELECT` to the closing quote: 34 bytes of query start, 231 `y`, 1 quote). If Oracle stores
  another length, tell me the number and I adjust the padding. `DEFAULT|F_DEF.C|` should be above
  8192 (3 + 1000 x 10 = 10003 expected, if Oracle stores the expression as written).
- File names wanted: `frm_exp_<v>.dmp`, `frm_exp_direct_<v>.dmp`, logs, `truth_frm.txt`.
- Status: open

### R-019 time zones: a database whose time zone is not +00:00; offsets with minutes
- Why: `exp.md` section 6.5, open questions 9 and 10 (second-wave types; lower priority than
  R-016 to R-018). Both databases have `dbtimezone=+00:00`, so the fixtures cannot say whether a
  TIMESTAMP WITH LOCAL TIME ZONE value is stored in UTC or in the database time zone. Also
  missing: a TSLTZ value with a zero fraction; a TSTZ offset that is negative with minutes.
- SQL (a throwaway database like the one of R-015; as a DBA **before** any TSLTZ column exists):

      ALTER DATABASE SET TIME_ZONE = '+03:00';
      -- restart the database, then check: SELECT dbtimezone FROM dual;   must show +03:00

  then as the data user:

      CREATE TABLE t_tz2 (id NUMBER(3), tstz TIMESTAMP(6) WITH TIME ZONE, tsltz TIMESTAMP(6) WITH LOCAL TIME ZONE);
      ALTER SESSION SET TIME_ZONE = '+00:00';
      INSERT INTO t_tz2 VALUES (1, TIMESTAMP '2016-03-26 19:34:55.123456 -03:30', TIMESTAMP '2016-03-26 19:34:55.123456');
      INSERT INTO t_tz2 VALUES (2, TIMESTAMP '2016-03-26 19:34:55.123456 +05:45', TIMESTAMP '2016-03-26 19:34:55');
      ALTER SESSION SET TIME_ZONE = '+05:30';
      INSERT INTO t_tz2 VALUES (3, TIMESTAMP '2016-03-26 19:34:55 -00:30', TIMESTAMP '2016-03-27 01:04:55.123456');
      COMMIT;
      -- truth_tz2.txt (run with ALTER SESSION SET TIME_ZONE = '+00:00')
      SELECT 'ENV|dbtimezone=' || dbtimezone FROM dual;
      SELECT id || '|' || TO_CHAR(tstz, 'YYYY-MM-DD HH24:MI:SS.FF6 TZH:TZM') || '|' || DUMP(tstz, 16) || '|'
             || TO_CHAR(tsltz, 'YYYY-MM-DD HH24:MI:SS.FF6') || '|' || DUMP(tsltz, 16) FROM t_tz2 ORDER BY id;

- Expectation to test: rows 1 and 3 hold the same instant in `tsltz` (19:34:55.123456 UTC). If the
  date bytes read 19:34:55 the value is stored in UTC; if they read 22:34:55 it is stored in the
  database time zone.
- File names wanted: `t_tz2_exp_<v>.dmp`, `t_tz2_exp_direct_<v>.dmp`, logs, `truth_tz2.txt`.
- Status: open

## Requests from Analyst session 5

All three are for legacy `exp`, table mode. R-020 answers a question about the project's real
dump and is the one I need; R-021 and R-022 are lower priority. I could not run any of this SQL;
please tell me if a statement fails.

### R-020 tables in which not every column has statistics; a table with table statistics only
- Why: `exp.md` section 4.6 ("The real dump"). In the fixtures every table with statistics has one
  `ANALSTATS T?` item for the table and one for **every** column. The real dump (6 tables, 63
  columns) has 5 table items and 53 column items; five tables with all their columns would give 54
  (if the table without statistics has 9 columns), 55, 51 or 50, never 53. So at least one column of
  an analysed table has no item, and no fixture shows when that happens. Also unknown: whether a
  table can have a table item without column items, an all-NULL column, and where `GRANT` lines
  stand relative to `ANALSTATS T?` (the real dump has both).
- SQL (as `CORPUS`, on 18c and 11g; in the CL8MSWIN1251 container too if it still exists):

      CREATE TABLE t_stc1 (id NUMBER(6), a VARCHAR2(20), b VARCHAR2(20), c DATE, allnull VARCHAR2(10));
      INSERT INTO t_stc1 SELECT LEVEL, 'a' || MOD(LEVEL, 7), 'b' || LEVEL, DATE '2020-01-01' + LEVEL, NULL
        FROM dual CONNECT BY LEVEL <= 50;
      COMMIT;
      CREATE TABLE t_stc2 AS SELECT id, a, b, c FROM t_stc1;
      CREATE TABLE t_stc3 AS SELECT * FROM t_stc1;
      CREATE TABLE t_stc4 AS SELECT * FROM t_stc1;
      CREATE TABLE t_stc5 AS SELECT * FROM t_stc1;
      -- 18c gathers statistics during CREATE TABLE AS SELECT: start all five from "no statistics"
      BEGIN
        FOR t IN (SELECT table_name FROM user_tables WHERE table_name LIKE 'T_STC_') LOOP
          DBMS_STATS.DELETE_TABLE_STATS(USER, t.table_name);
        END LOOP;
      END;
      /
      -- (1) statistics for two of five columns only
      BEGIN DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STC1', method_opt => 'FOR COLUMNS SIZE 1 id a'); END;
      /
      -- (2) all four columns analysed, then a fifth column is added
      BEGIN DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STC2', method_opt => 'FOR ALL COLUMNS SIZE 1'); END;
      /
      ALTER TABLE t_stc2 ADD (added_later VARCHAR2(10));
      -- (3) all five columns analysed (ALLNULL is NULL in every row), then the statistics of B are deleted
      BEGIN
        DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STC3', method_opt => 'FOR ALL COLUMNS SIZE 1');
        DBMS_STATS.DELETE_COLUMN_STATS(USER, 'T_STC3', 'B');
      END;
      /
      -- (4) no statistics at all (control): nothing more to do for t_stc4
      -- (5) table statistics without column statistics
      ANALYZE TABLE t_stc5 COMPUTE STATISTICS FOR TABLE;
      GRANT SELECT ON t_stc1 TO PUBLIC;
      GRANT SELECT ON t_stc4 TO PUBLIC;
      COMMENT ON TABLE t_stc1 IS 'stc1';
      COMMENT ON COLUMN t_stc1.b IS 'no statistics on this column';
      -- truth_stc.txt
      SELECT 'TAB|' || table_name || '|' || NVL(TO_CHAR(num_rows), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD'), 'NULL')
        FROM user_tables WHERE table_name LIKE 'T_STC_' ORDER BY 1;
      SELECT 'COLUMN|' || c.table_name || '|' || c.column_id || '|' || c.column_name || '|'
             || NVL((SELECT 'stats:' || s.num_distinct || ':' || s.num_nulls FROM user_tab_col_statistics s
                      WHERE s.table_name = c.table_name AND s.column_name = c.column_name), 'NO STATISTICS')
        FROM user_tab_columns c WHERE c.table_name LIKE 'T_STC_' ORDER BY c.table_name, c.column_id;

- Exports: `tables=(t_stc1,t_stc2,t_stc3,t_stc4,t_stc5)`, default `statistics=estimate`, conventional;
  the same with `NLS_LANG=AMERICAN_AMERICA.US7ASCII` (the combination of the real dump).
- Arithmetic to check (hypothesis "one column item per row of `user_tab_col_statistics`"): column
  items T_STC1 2, T_STC2 4, T_STC3 4, T_STC4 0, T_STC5 0: 10 in all; table items 4 (all but
  T_STC4); `ANALCOMPUTE T?` lines 4. The other reading ("one item per column of every analysed
  table") gives 5 + 5 + 5 + 0 + 5 = 20. If `truth_stc.txt` shows other `NO STATISTICS` lines than I
  expect (for example because Oracle keeps a row for B), tell me the lines and I adjust.
- File names wanted: `stc_exp_<v>.dmp`, `stc_exp_us7ascii_<v>.dmp`, logs, `truth_stc.txt`.
- Status: generated (with two changes made by the project owner, `corpus/sql/10_stc.sql`: T_STC2 gets **two**
  added columns, `ADDED_LATER VARCHAR2(10)` left NULL and `ADDED_NUM NUMBER` filled by an UPDATE after the
  `ALTER TABLE`; the truth query tests `last_analyzed IS NOT NULL`). Files: `stc_exp_<v>.dmp` and
  `stc_exp_direct_<v>.dmp` on 18c and 11g, logs, `truth_stc.txt`. Not generated: `stc_exp_us7ascii_<v>.dmp`
  (see R-023).
- Analyst note (session 6): received and used: `exp.md` section 4.6 ("Columns without statistics", "The real
  dump, reconciled"), 13.9, rule S7 of section 11. Result: the arithmetic of this request holds exactly (column
  items 2 + 4 + 4 + 0 + 0 = 10, table items 4, `ANALCOMPUTE T?` lines 4, in all four files); the other reading
  (20) is refuted. A column has an item exactly when `truth_stc.txt` has a `stats:` line for it; the two added
  columns of T_STC2 have none, filled or not; the all-NULL column of T_STC3 has one; `GRANT` stands before the
  table item. Remark on the truth: it has no rows and no `blocks` / `avg_row_len` / `density`; the rows were
  compared with the SQL, and only `num_rows`, `num_distinct` and `num_nulls` of the items with the truth.

### R-021 histograms that `exp` could write as text or as a very long piece
- Why: `exp.md` sections 4.2 ("Session 5"), 4.6, 14.10. (a) The list `SREC.CHVALS :=
  DBMS_STATS.CHARARRAY(...)` is empty in all 8 fields that have it and `NULL` in the 230 others, so
  the fixtures cannot say how a text endpoint would be written (characters in the client set,
  hexadecimal, `?`). (b) The largest piece of a statistics field is 797 bytes; a histogram with 254
  endpoints of a text column gives a list of about 254 x 38 = 9650 bytes: one piece above 8192, or
  several? Lower priority: the real dump has no field above 531 bytes.
- SQL (as `CORPUS` on 18c and 11g; `t_sth2` also as `CORPUS_RU` in the CL8MSWIN1251 container):

      -- (b) 254 distinct text values: frequency histogram with 254 endpoints
      CREATE TABLE t_sth1 (id NUMBER(6), w VARCHAR2(20));
      INSERT INTO t_sth1 SELECT LEVEL, 'w' || LPAD(MOD(LEVEL, 254), 3, '0') FROM dual CONNECT BY LEVEL <= 2540;
      -- (a) 5 distinct values that share their first 40 bytes: endpoint actual values are needed to tell them apart
      CREATE TABLE t_sth2 (id NUMBER(6), s VARCHAR2(60));
      INSERT INTO t_sth2 SELECT LEVEL, RPAD('same_prefix_longer_than_32_bytes_', 40, 'x') || MOD(LEVEL, 5)
        FROM dual CONNECT BY LEVEL <= 500;
      -- in the CL8MSWIN1251 container use a Cyrillic prefix instead:
      --   RPAD(UNISTR('\0410\0411\0412'), 40, UNISTR('\0416')) || MOD(LEVEL, 5)
      COMMIT;
      BEGIN
        DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STH1', method_opt => 'FOR COLUMNS SIZE 254 w');
        DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STH2', method_opt => 'FOR COLUMNS SIZE 254 s');
      END;
      /
      -- truth_sth.txt
      SELECT 'COL|' || table_name || '|' || column_name || '|' || num_buckets || '|' || histogram
        FROM user_tab_col_statistics WHERE table_name LIKE 'T_STH_' ORDER BY 1;
      SELECT 'HIST|' || table_name || '|' || column_name || '|' || endpoint_number || '|'
             || TO_CHAR(endpoint_value, 'TM9') || '|' || NVL(DUMP(endpoint_actual_value, 16), 'NULL')
        FROM user_tab_histograms WHERE table_name LIKE 'T_STH_' ORDER BY table_name, column_name, endpoint_number;

- Check after generation: `truth_sth.txt` should show 254 `HIST|T_STH1|W|` rows and, for `T_STH2.S`,
  rows whose last field is not `NULL`. If it is `NULL` on 11g, the prefix must be made longer.
- Exports: `tables=(t_sth1,t_sth2)`, conventional, client = database set, and US7ASCII.
- File names wanted: `sth_exp_<v>.dmp`, `sth_exp_us7ascii_<v>.dmp`, logs, `truth_sth.txt`; optional
  `ru_sth_exp_{cp1251,us7ascii}_11g.dmp`.
- Status: open
- State at Analyst session 6: open, not generated (no `sth_*` file in `corpus/fixtures/`), still wanted at low priority; nothing was done for it in session 6.

### R-022 an `exp` client older than the database (no SQL)
- Why: `exp.md` "Limit of confirmed", open question 8. The real dumps are written by an `exp` 10.2
  client against an 11.2.0.4 server. Every fixture is written by the `exp` of the database's own
  version. The header line would read `EXPORT:V10.02.01`; whether the statistics items, the
  introducing lines and the empty-table marker are the same is unknown. One explanation of the
  53 column items of the real dump (R-020) could be the older client.
- Wanted, if such a client can be installed at all: the tables of R-016 part a and of R-020,
  exported from the 11g database with a 10.2 `exp` (`stat_exp_c102_11g.dmp`, `stc_exp_c102_11g.dmp`,
  logs). If it cannot be done, please mark this request `rejected`; the limit stays in `exp.md`.
- Status: open
- State at Analyst session 6: open, not generated (no `*_c102_*` file in `corpus/fixtures/`); it is no longer needed to explain the 53 column items of the real dump (R-020 does), but it stays the only way to confirm any rule for a 10.2 client; nothing was done for it in session 6.

## Requests from Analyst session 6

One request, low priority. No rule of `exp.md` section 11 waits for it; it closes the two limits named in
section 4.6 ("The real dump, reconciled") with exports of tables that already exist.

### R-023 the R-020 tables with a US7ASCII client and with `statistics=none` (no new SQL)
- Why: `exp.md` section 4.6. The real dump was written with a US7ASCII client (letters `TR`, `EXP-00091` in the
  log); the R-020 files were written with the AL32UTF8 client only. In the R-016 files the client set changes
  the second letter of every statistics line and nothing else (24 bytes), but no file shows a table with
  **missing** column items written with that client, and R-020 asked for it (`stc_exp_us7ascii_<v>.dmp`). Also
  not shown: that `statistics=none` removes the items of such tables like those of the R-016 tables.
- SQL: none. The tables T_STC1 to T_STC5 as made by `corpus/sql/10_stc.sql`, statistics unchanged; if the
  tables have to be made again, the truth file must be written again too (`truth_stc.txt`, same query).
- Exports (18c and 11g, table mode, `tables=(t_stc1,t_stc2,t_stc3,t_stc4,t_stc5)`, conventional path):
  (a) `NLS_LANG=AMERICAN_AMERICA.US7ASCII`, default statistics; (b) default client, `statistics=none`.
- Arithmetic to check (hypothesis: only the letter changes). (a) 14 `ANALSTATS TR` lines and 4 `ANALCOMPUTE TR`
  lines, each field byte-identical to the one of `stc_exp_<v>.dmp`; `cmp` from offset R on gives 4 + 18 = 22
  differing bytes (4 client ids in block 2, one letter per line); the log has 4 `EXP-00091` lines (one per
  plain field: the 4 table items; rule of section 4.6). (b) 114 - 14 x 2 - 11 = 75 items: no `ANALSTATS` line
  or field, and the section "Deferred analyze commands " (section line, 4 `TABLE` lines, 4 `ANALCOMPUTE` lines,
  `ENDTABLE`, `METRICET 54`) gone, as in `stat_exp_none_<v>.dmp` (111 - 51 = 60); the 5 binary parts identical to `stc_exp_<v>.dmp`.
- File names wanted: `stc_exp_us7ascii_<v>.dmp`, `stc_exp_none_<v>.dmp`, logs.
- Status: generated (with `truth_stc_extra.txt`, written by `corpus/sql/10b_stc_truth.sql` immediately before the two exports)
- Analyst note (session 7): received and used: `exp.md` section 4.6 ("The R-020 tables with a US7ASCII client and
  with `statistics=none`"). (b) holds exactly: 75 items, no statistics item, the 5 binary parts identical. (a) is
  refuted in its counts (22 `ANALSTATS TR`, 5 `ANALCOMPUTE TR`, 5 `EXP-00091` instead of 14, 4, 4) because the
  database's statistics had changed since `truth_stc.txt` (T_STC4 and the two added columns of T_STC2 were analysed
  in between; `truth_stc_extra.txt` shows it); against `truth_stc_extra.txt` every item is as rule S7 says, and the
  items that both files have differ only in the letter, except one number of the T_STC2 table item.

## Requests from the Architect (`docs/design/open-questions.md`, section (b)), recorded in Analyst session 7

The Architect wrote R-024 to R-030 in `docs/design/open-questions.md` (full SQL and arithmetic there; the
Architect does not edit this folder). They are listed here so that this file has every request number.

### R-024 NUMBER boundary values
- Files: `t_num2_exp_<v>.dmp`, log, `truth_num2.txt` (SQL `corpus/sql/11_types.sql`).
- Status: generated
- Analyst note (session 7): used in `exp.md` 6.1 ("Bytes to a decimal number"). All 14 byte strings of the
  Architect's arithmetic are confirmed, none refuted; the decoding procedure is stated as rule Y1.

### R-025 the column list of the `CREATE TABLE` line
- Files: `t_decl_exp_<v>.dmp`, log, `truth_decl.txt` (SQL `corpus/sql/11_types.sql`).
- Status: generated
- Analyst note (session 7): used in `exp.md` 4.7 and 4.1. The arithmetic holds: column count `0e 00`; 12 x 4 + 2 x 8
  = 64 bytes of descriptors (@0x2461-0x24a0); one ` MODIFY DEFAULT` item before `INSERT INTO` (@0x2323); 14 lines
  of truth. Grammar: rule T9.

### R-026 (a) NCHAR and `CHAR(n CHAR)`; (b) Oracle's byte-to-Unicode map
- Files: (a) `nchar_exp_<v>.dmp` (the request named it `t_nchar_exp_<v>.dmp`), log, `truth_nchar.txt`;
  (b) `truth_csmap.txt` in both folders (SQL `corpus/sql/12_csmap.sql`).
- Status: generated (a and b)
- Analyst note (session 7): (a) every byte of the Architect's hypothesis is confirmed; the descriptor length of
  `CHAR(4 CHAR)` is 16 (`exp.md` 4.1, 6.3; rules C5, C6). (b) `exp.md` 14.11, rule C7: Oracle = code page + C1
  controls for the undefined bytes (1251: 0x98; 1252: 0x81, 0x8D, 0x8F, 0x90, 0x9D); US7ASCII 0x80-0xFF -> U+FFFD.

### R-027 LOB columns
- Files wanted: `lob_exp_<v>.dmp`, `lob_exp_direct_<v>.dmp`, logs, `truth_lob.txt` (SQL in the Architect's text).
- Status: generated
- Analyst note (session 8): used in `exp.md` 14.6 (rewritten), 4.1; rules L1 to L6. The arithmetic holds: 5 + 2 rows; C
  lengths 9, NULL, 0, 3000, 100,000 characters, B 10, NULL, 0, 1000, 100,000 bytes; row 5 is written in 13 chunks per
  column after `fc ff` and an 8-byte total; T_FRAME after the LOB tables is byte-identical to `t_frame_exp_<v>` on both
  paths; the log of the direct file says both LOB tables went the conventional way. 60 checks per version, 0 differ
  (`tools/lob_check.py`). The SQL file was `13_optional.sql` + `01b_optional_tables.sql`.
- Analyst note (session 7): still needed, first of the three open Architect requests: a LOB column stops the
  file (rule X1) and changes the bytes before the rows (`exp.md` 14.6), and only one CLOB table in a single-byte
  database is known. The pattern of R-028 (the unknown table followed by T_FRAME) is the right design.

### R-028 LONG and LONG RAW
- Files: `long_exp_<v>.dmp` (`tables=t_long,t_longraw,t_frame`), log (SQL `corpus/sql/11_types.sql`; no truth file).
- Status: generated
- Analyst note (session 7): used in `exp.md` 6.6, rule Y9: 4-byte descriptor with length 0; values are plain
  fields (5, NULL, 32,000 bytes; 2 and 32,000 bytes); T_FRAME after them is byte-identical to `t_frame_exp_<v>`.
  Values above 65,533 bytes and the direct path: R-031.

### R-029 INTERVAL, ROWID, UROWID
- Files wanted: `misc_exp_<v>.dmp`, log, `truth_misc.txt`.
- Status: generated
- Analyst note (session 8): used in `exp.md` 6.7, 4.1; rules Y10, Y11. Every descriptor is 4 bytes and its type code equals
  the `Typ=` of `DUMP` (182, 183, 69, 208); interval values are the `DUMP` bytes, the ROWID field is 23 bytes (5 unknown
  bytes + the 18-character text), the UROWID field is the `DUMP` bytes; 40 checks, 0 differ (`tools/misc_types.py`).
- Analyst note (session 7): still needed (descriptor size unknown: one such column stops the file). Second
  priority after R-027, unless the owner's type survey (question Q-4) shows none of these codes.

### R-030 a partitioned table (18c only)
- Files wanted: `part_exp_18c.dmp`, `part_exp_direct_18c.dmp`, logs, `truth_part.txt`.
- Status: generated
- Analyst note (session 8): used in `exp.md` 4.8; rule T10. One table section, one `PARTITION "name"` line + `INSERT INTO`
  line + binary part per partition, one `ENDPARTITION` line. Row counts 1095, 1098, 807 = `truth_part.txt` = the log =
  the Architect's arithmetic; every row equals the SQL; 38 checks, 0 differ (`tools/part_check.py`).
- Analyst note (session 7): still needed; third. Nothing in session 7 bears on it.

### R-032 NUMBER: computed values in a column declared `NUMBER` (Architect, `docs/design/open-questions.md`)
- Status: generated
- Analyst note (session 9): used in `exp.md` 6.1, rule Y1. All six rows have 20 digit bytes (Len=21) on both versions,
  byte-identical between 18c and 11g: **H40 holds for every row**; H38 is refuted in both readings (row 6, 5/3 =
  `c1 02` + 18 x `43` + `44` at `t_num3_exp_<v>.dmp`@0x22de: neither `... + 17 x 43 + 44` (19 digit bytes) nor `... 47`
  (38 significant digits, note A-11)). Row 6 has 39 significant decimal digits, the others 40.
- Analyst note (session 8, check of the text): SQL and truth (`TO_CHAR(v, 'TM9')`, `DUMP(v, 16)`) are enough. The H40 bytes of
  rows 1 to 6 are right (checked by hand with the rule of `exp.md` 6.1: e.g. row 3, 1/7: 6 x `0f 1d 3a` + `0f 1e`, 20 digit
  bytes, Len=21). **One ambiguity in H38**: the request defines H38 as "19 digit bytes" and also calls it 38 digits. The two are the
  same for rows 1 to 5, not for row 6 (5/3 = 1.666...): its first base-100 digit is `01`, which carries one significant
  decimal digit, so "38 significant decimal digits" is `1` + 37 decimals = 20 digit bytes, the last being 70: `c1 02` +
  18 x `43` + `47`, Len=21, while "19 digit bytes" gives `c1 02` + 17 x `43` + `44`, Len=20. Please state which H38 is
  meant; with the truth file both readings and H40 (`c1 02` + 18 x `43` + `44`, Len=21) can be told apart, so the SQL
  and the truth are enough.

### R-033 characters outside the BMP (Architect, `docs/design/open-questions.md`)
- Status: generated (with a CLOB and an NCLOB column added to `t_nchar2`)
- Analyst note (session 9): used in `exp.md` 6.3, 14.6; rules C5, C6, L5. The arithmetic of the request holds byte for
  byte on both versions; NC `NCHAR(3)` is `00 61 d8 3d de 00` without padding (NCHAR pads to code units); CL and NCL
  are both `00 61 d8 3d de 00` (surrogate pair) and `GETLENGTH` is 3 (code units).
- Analyst note (session 8, check of the text): the arithmetic is right: NV = `00 61 d8 3d de 00` (6 bytes), CC padded to 3
  characters = `61 f0 9f 98 80 20` (6 bytes), VC = 5 bytes; the descriptor lengths follow C5 / C6. SQL and truth (`DUMP(...,
  16)` of each column) are enough. Suggestion: add a CLOB and an NCLOB column with the same value (the LOB data are
  UTF-16 BE, rule L5, so they show whether a surrogate pair is written as such and whether `DBMS_LOB.GETLENGTH` counts it as
  2), and `DBMS_LOB.GETLENGTH` of both in the truth; R-035 does the same and can share the export.

## Requests from Analyst session 7

### R-031 LONG above 65,533 bytes, LONG on the direct path, a non-ASCII LONG value
- Why: `exp.md` 6.6, rule Y9. A LONG value can hold up to 2 GB, but a row field has a u16 length (largest value
  seen: 32,000; `fe ff` and `ff ff` are markers). How `exp` writes a longer value is unknown, and a reader that
  reads type 8 by rule Y9 would misread it. Also unknown: LONG on the direct path (the log may say that such a
  table goes the conventional way, as for LOB tables), and the character set of a LONG value with non-ASCII
  characters (its descriptor has no set id).
- SQL (as `CORPUS`, AL32UTF8 database; 18c and 11g). The value of 100,000 bytes needs a LONG longer than a PL/SQL
  `VARCHAR2` (32,767); binding a CLOB to a LONG column is the attempt below. If Oracle refuses it, please say so
  (that part of the request is then `rejected`; the rest stays useful):

      CREATE TABLE t_long2 (id NUMBER(3), l LONG, v VARCHAR2(10));
      INSERT INTO t_long2 VALUES (1, UNISTR('\0434\0436 Caf\00E9'), 'a');
      DECLARE
        c CLOB;
      BEGIN
        DBMS_LOB.CREATETEMPORARY(c, TRUE);
        FOR i IN 1 .. 10 LOOP
          DBMS_LOB.WRITEAPPEND(c, 10000, RPAD(TO_CHAR(i), 10000, 'y'));
        END LOOP;
        EXECUTE IMMEDIATE 'INSERT INTO t_long2 VALUES (2, :1, ''b'')' USING c;
        COMMIT;
      END;
      /
      -- truth_long2.txt (LENGTH of a LONG cannot be selected in SQL; the PL/SQL block reads it)
      SET SERVEROUTPUT ON
      BEGIN
        FOR r IN (SELECT id, l FROM t_long2 ORDER BY id) LOOP
          DBMS_OUTPUT.PUT_LINE(r.id || '|' || LENGTHB(r.l) || '|' || RAWTOHEX(UTL_RAW.CAST_TO_RAW(SUBSTR(r.l, 1, 20))));
        END LOOP;
      END;
      /

  (Reading a LONG of 100,000 bytes into PL/SQL fails above 32,760 bytes; if it does, write the line for id 2 by
  hand as `2|100000|` + the hex of `1yyyyyyyyyyyyyyyyyyy` and say so in the file.)
- Exports (18c and 11g): `tables=t_long2,t_frame` conventional (`long2_exp_<v>.dmp`) and `direct=y`
  (`long2_exp_direct_<v>.dmp`); also `tables=t_long,t_longraw,t_frame` with `direct=y` (`long_exp_direct_<v>.dmp`);
  and `t_long2` with `NLS_LANG=AMERICAN_AMERICA.US7ASCII` (`long2_exp_us7ascii_<v>.dmp`). `statistics=none`.
- Arithmetic to check: row 1 L is `d0 b4 d0 b6 20 43 61 66 c3 a9` (10 bytes) if a LONG is in the database set like
  VARCHAR2, in every client set; row 2 L has 100,000 bytes, `31` + 9,999 x `79`, `32` + 9,999 x `79`, ..., `31 30` +
  9,998 x `79` (10 blocks of 10,000; the last block starts with `10`); it cannot have a u16 length, so either it is
  cut into pieces (how many, and is there a count?) or something else happens. V must follow L in both rows; T_FRAME
  must be byte-identical to `t_frame_exp_<v>.dmp` in every file.
- File names wanted: `long2_exp_<v>.dmp`, `long2_exp_direct_<v>.dmp`, `long2_exp_us7ascii_<v>.dmp`,
  `long_exp_direct_<v>.dmp`, logs, `truth_long2.txt`.
- Status: generated, part rejected (Oracle refused the 100,000-byte value with ORA-22835: a CLOB bound to a LONG column
  goes through a 4000-byte buffer; `truth_long2.txt` line 2 records the refusal). The value above 65,533 bytes is
  requested again by another route in **R-040**.
- Analyst note (session 9): used in `exp.md` 6.6, 5.4, 4.1; rules Y9, R2, T3, T9, E2a. Row 1: the LONG bytes are the
  database bytes `d0 b4 d0 b6 20 43 61 66 c3 a9` with the AL32UTF8 and with the US7ASCII client (note A-8 answered).
  The direct path writes LONG tables with flag 1 (no switch to the conventional path), moves the LONG column to the end
  of `INSERT INTO` and the descriptors, and writes a long LONG value as a chain of pieces with the flag bit 0x8000
  (`long_exp_direct_<v>.dmp`@0x2288: 156 + 4 x 7961 bytes). T_LONG_EMPTY: no `ff ff` on 18c, `ff ff` on 11g; note A-7
  does not arise because V, not L, is column 1 on the direct path.
- Analyst note (session 8): two additions, no change to the SQL above. (1) Hypothesis for the 100,000-byte value, from the
  LOB values of R-027 (`exp.md` 14.6, rule L4): `fc ff`, the total as 8 bytes LE (`a0 86 01 00 00 00 00 00`), then
  chunks `u16 n + n` adding up to 100,000; then V. The other possibility seen nowhere yet: a different marker. (2) Note
  A-7 of the Architect (rule E2 with a LONG first column on the direct path) needs a LONG table **without rows** exported
  with `direct=y` on 18c: please add `CREATE TABLE t_long_empty (l LONG, v VARCHAR2(10));` (no rows) to the
  `tables=` list of `long_exp_direct_<v>.dmp`, before T_FRAME. Arithmetic: if `exp` writes LONG tables on the direct
  path at all (the log says "will be exported in conventional path" if not), the flag is 1 and after it either `ff ff`
  LF (as on 11g) or LF directly followed by `TABLE "T_FRAME"` (as for T_EMPTY on 18c).
- Analyst note (session 8, check of the text): the hex `d0 b4 d0 b6 20 43 61 66 c3 a9` (10 bytes) is right for
  `UNISTR('\0434\0436 Caf\00E9')` in AL32UTF8; the 10 blocks of 10,000 bytes add up to 100,000; the truth block reads a
  LONG into PL/SQL, which fails above 32,760 bytes, as the request says (the hand-written line for id 2 is then fine).

## Requests from Analyst session 8

### R-034 `VARCHAR2(n CHAR)` and `CHAR(n CHAR)` in an AL32UTF8 database, with the byte caps (replaces R-011)
- Why: `exp.md` 4.1, rules C6 and X4. `CHAR(4 CHAR)` has descriptor length 16 = 4n; `VARCHAR2(n CHAR)` in AL32UTF8 is not in
  any fixture (the only one is in the single-byte database). A reader that checks value lengths against the descriptor
  needs the rule, and the dictionary caps `data_length` at 4000 (VARCHAR2) and 2000 (CHAR) bytes, so 4n cannot hold for
  large n.
- SQL (as `CORPUS`, AL32UTF8 database; 18c and 11g):

      CREATE TABLE t_probe11 (id NUMBER(3), b VARCHAR2(10 BYTE), c VARCHAR2(10 CHAR), k CHAR(3 CHAR),
                              w VARCHAR2(1000 CHAR), x VARCHAR2(1001 CHAR), y CHAR(500 CHAR), z CHAR(501 CHAR));
      INSERT INTO t_probe11 VALUES (1, 'abc', 'abc', 'abc', 'abc', 'abc', 'abc', 'abc');
      INSERT INTO t_probe11 VALUES (2, NULL, UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'),
                                    UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'));
      COMMIT;
      -- truth_probe11.txt
      SELECT 'COL|' || column_name || '|' || data_type || '|' || data_length || '|' || char_length || '|' || char_used
        FROM user_tab_columns WHERE table_name = 'T_PROBE11' ORDER BY column_id;
      SELECT 'ROW|' || id || '|' || NVL(DUMP(b, 16), 'NULL') || '|' || DUMP(c, 16) || '|' || DUMP(k, 16) || '|'
             || DUMP(w, 16) || '|' || DUMP(x, 16) || '|' || LENGTHB(y) || '|' || LENGTHB(z)
        FROM t_probe11 ORDER BY id;

- Arithmetic to check (hypothesis H: descriptor length = `data_length` of the dictionary = min(4n, 4000) for VARCHAR2 and
  min(4n, 2000) for CHAR): B `01 00 0a 00 69 03 01 00`; C `01 00 28 00 69 03 01 00` (40); K `60 00 0c 00 69 03 01 00` (12,
  as C6); W `01 00 a0 0f ...` (4000); X 4000 if capped (`a0 0f`), 4004 (`a4 0f`) if not; Y `60 00 d0 07 ...` (2000); Z 2000
  if capped, 2004 if not. Values: row 1 B, C, K, W, X = `61 62 63` (3 bytes; K is exactly 3 characters, no padding); Y =
  3 + 497 spaces = 500 bytes, Z = 501 bytes. Row 2: C, K, W, X = `e6 97 a5 e6 9c ac e8 aa 9e` (9 bytes, 3 characters; K
  not padded); Y = 9 + 497 = 506 bytes, Z = 9 + 498 = 507 bytes (padding counts characters, C6). B row 2 is `fe ff`.
- Exports: `tables=t_probe11,t_frame`, conventional and `direct=y`, `statistics=none`. Can share the export of R-033.
- File names wanted: `t_probe11_exp_<v>.dmp`, `t_probe11_exp_direct_<v>.dmp`, logs, `truth_probe11.txt`.
- Status: generated
- Analyst note (session 9): used in `exp.md` 4.1, 6.3; rule C6 (X4 closed). Hypothesis H holds in all 7 columns, 4 files:
  descriptor = `data_length` = min(4n, 4000) / min(4n, 2000) (X 4000, Z 2000: capped); values and padding exactly as the
  arithmetic says (Y 500 / 506, Z 501 / 507 bytes); conventional = direct.

### R-035 LOB values around the switch from the short to the long form; non-ASCII and non-BMP CLOB text
- Why: `exp.md` 14.6, rules L4, L5. A LOB value was written in the short form (`u16 n`) at 6000 bytes and in the long form
  (`fc ff` + 8-byte length + chunks) at 100,000 and 200,000 bytes; where `exp` switches is unknown (a reader that accepts
  both forms does not need it, but an implementer's test does, and a short value must stay below 0xfff0). The CLOB data
  of an AL32UTF8 database are UTF-16 BE, shown with ASCII only; non-ASCII and non-BMP characters (surrogate pairs) and the
  meaning of `DBMS_LOB.GETLENGTH` for them are not in the fixtures.
- SQL (as `CORPUS`, AL32UTF8 database; 18c and 11g):

      CREATE TABLE t_lob3 (id NUMBER(3), c CLOB, b BLOB, n NCLOB);
      DECLARE
        TYPE t_sizes IS TABLE OF PLS_INTEGER;
        s t_sizes := t_sizes(8000, 8060, 8061, 8132, 8133, 16120, 16121, 32766, 32767, 32768, 65536);
        l_c CLOB; l_b BLOB; r PLS_INTEGER;
      BEGIN
        FOR i IN 1 .. s.COUNT LOOP
          INSERT INTO t_lob3 VALUES (i, EMPTY_CLOB(), EMPTY_BLOB(), NULL) RETURNING c, b INTO l_c, l_b;
          FOR j IN 1 .. TRUNC(s(i) / 1000) LOOP
            DBMS_LOB.WRITEAPPEND(l_c, 1000, RPAD('x', 1000, 'x'));
            DBMS_LOB.WRITEAPPEND(l_b, 1000, UTL_RAW.COPIES(HEXTORAW('5A'), 1000));
          END LOOP;
          r := MOD(s(i), 1000);
          IF r > 0 THEN
            DBMS_LOB.WRITEAPPEND(l_c, r, RPAD('x', r, 'x'));
            DBMS_LOB.WRITEAPPEND(l_b, r, UTL_RAW.COPIES(HEXTORAW('5A'), r));
          END IF;
        END LOOP;
        INSERT INTO t_lob3 VALUES (20, TO_CLOB(UNISTR('\0434\0436 Caf\00E9 a\D83D\DE00')), NULL,
                                   TO_NCLOB(UNISTR('\0434\0436 Caf\00E9 a\D83D\DE00')));
        COMMIT;
      END;
      /
      -- truth_lob3.txt
      SELECT 'T_LOB3|' || id || '|' || DBMS_LOB.GETLENGTH(c) || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL') || '|'
             || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(n)), 'NULL') || '|' || DUMP(DBMS_LOB.SUBSTR(c, 20, 1), 1016)
        FROM t_lob3 ORDER BY id;

- Arithmetic to check: rows 1 to 11: B = S bytes of `5a`, C = S characters `x` = 2S bytes `00 78` (rule L5); S = 8000,
  8060, 8061, 8132, 8133, 16120, 16121, 32766, 32767, 32768, 65536. A short value needs 2S <= 0xffef, so C of rows 10 and 11
  (65,536 and 131,072 bytes) and B of row 11 must be long. Hypothesis "long iff the value is larger than one chunk" (18c
  chunk 8060 bytes for BLOB, 16,120 bytes for CLOB; 11g 8132 / 16,264): on 18c B short for 8000, 8060, long from 8061; C
  short up to S = 8060 (16,120 bytes), long from 8061; on 11g the switch at 8132 / 8133. Row 20: C and N are
  `04 34 04 36 00 20 00 43 00 61 00 66 00 e9 00 20 00 61 d8 3d de 00` (22 bytes, short form) if a non-BMP character is a
  surrogate pair; `GETLENGTH` 11 if it counts UTF-16 code units, 10 if characters. T_FRAME after T_LOB3 must equal
  `t_frame_exp_<v>`. If the CL8MSWIN1251 database is still available, the same table there without row 20 shows NCLOB and
  BLOB in a single-byte database (`lob3_exp_cp1251_11g.dmp`; C then 1 byte per character).
- Exports: `tables=t_lob3,t_frame`, conventional only (LOB tables are never written on the direct path), `statistics=none`.
- File names wanted: `lob3_exp_<v>.dmp`, log, `truth_lob3.txt` (optional: `lob3_exp_cp1251_11g.dmp`).
- Status: generated (without the optional `lob3_exp_cp1251_11g.dmp`)
- Analyst note (session 9): used in `exp.md` 14.6; rules L4, L5. The hypothesis "long iff larger than one chunk" holds for
  CLOB and BLOB on both versions (18c switch after 8060 characters / bytes, 11g after 8132); row 20 is the 22 bytes of the
  request (surrogate pair), `GETLENGTH` 11 (code units). The single-byte part (CLOB, NCLOB, BLOB in CL8MSWIN1251) remains
  open as the Architect's R-038.

### R-036 INTERVAL with mixed negative fields and other precisions; more ROWIDs; a logical UROWID
- Why: `exp.md` 6.7, rules Y10, Y11: only +, - and 0 with zero lower fields were seen; the 5 bytes before the ROWID text
  are unknown (one value per version); only the physical UROWID form (`01`) is in the fixtures.
- SQL (as `CORPUS`; 18c and 11g):

      CREATE TABLE t_interval2 (id NUMBER(3), iym INTERVAL YEAR(9) TO MONTH, ids INTERVAL DAY(9) TO SECOND(9),
                                ids0 INTERVAL DAY(2) TO SECOND(0));
      INSERT INTO t_interval2 VALUES (1, INTERVAL '-1-11' YEAR(9) TO MONTH, INTERVAL '-1 02:03:04.5' DAY(9) TO SECOND(9),
                                      INTERVAL '-0 00:00:01' DAY(2) TO SECOND(0));
      INSERT INTO t_interval2 VALUES (2, INTERVAL '999999999-11' YEAR(9) TO MONTH,
                                      INTERVAL '999999999 23:59:59.999999999' DAY(9) TO SECOND(9), INTERVAL '99 23:59:59' DAY(2) TO SECOND(0));
      INSERT INTO t_interval2 VALUES (3, INTERVAL '-999999999-11' YEAR(9) TO MONTH,
                                      INTERVAL '-999999999 23:59:59.999999999' DAY(9) TO SECOND(9), INTERVAL '-99 23:59:59' DAY(2) TO SECOND(0));
      CREATE TABLE t_iot (k NUMBER(3) PRIMARY KEY, v VARCHAR2(10)) ORGANIZATION INDEX;
      INSERT INTO t_iot VALUES (1, 'a');
      CREATE TABLE t_rowid2 (id NUMBER(3), rid ROWID, urid UROWID);
      INSERT INTO t_rowid2 SELECT ROWNUM, ROWID, ROWID FROM t_probe6 WHERE ROWNUM <= 3;
      INSERT INTO t_rowid2 SELECT 10, NULL, ROWID FROM t_iot;
      COMMIT;
      -- truth_misc2.txt
      SELECT 'INTERVAL|' || id || '|' || TO_CHAR(iym) || '|' || DUMP(iym, 16) || '|' || TO_CHAR(ids) || '|' || DUMP(ids, 16)
             || '|' || TO_CHAR(ids0) || '|' || DUMP(ids0, 16) FROM t_interval2 ORDER BY id;
      SELECT 'ROWID|' || id || '|' || NVL(ROWIDTOCHAR(rid), 'NULL') || '|' || NVL(DUMP(rid, 16), 'NULL') || '|'
             || CAST(urid AS VARCHAR2(4000)) || '|' || DUMP(urid, 16) FROM t_rowid2 ORDER BY id;

- Arithmetic to check (rule Y10 and the hypothesis "every field of a negative interval is negative"): row 1 IYM
  `7f ff ff ff 31` (-1, 60 - 11), text `-000000001-11`; IDS `7f ff ff ff 3a 39 38 62 32 9b 00` (-1 d, -2 h, -3 min, -4 s,
  -500,000,000 ns), text `-000000001 02:03:04.500000000`; IDS0 `80 00 00 00 3c 3c 3b 80 00 00 00` (0 d, -1 s), text
  `-00 00:00:01`. Row 2: IYM `bb 9a c9 ff 47`; IDS `bb 9a c9 ff 53 77 77 bb 9a c9 ff`; IDS0 `80 00 00 63 53 77 77 80 00 00 00`.
  Row 3: IYM `44 65 36 01 31`; IDS `44 65 36 01 25 01 01 44 65 36 01`; IDS0 `7f ff ff 9d 25 01 01 80 00 00 00`. Descriptor
  lengths 5 and 11 whatever the precision (hypothesis). ROWID rows 1 to 3: 23-byte fields; do the 5 bytes before the text
  stay `01 00 00 00 17`? URID row 10: a logical UROWID, first byte not `01` (hypothesis `02`), its text starts with `*`;
  RID row 10 `fe ff`.
- Exports: `tables=t_interval2,t_rowid2,t_frame`, conventional, `statistics=none`.
- File names wanted: `misc2_exp_<v>.dmp`, log, `truth_misc2.txt`.
- Status: generated
- Analyst note (session 9): used in `exp.md` 6.7; rules Y10, Y11. All interval bytes and texts as the arithmetic says
  (every field of a negative interval is negative; descriptor lengths 5 / 11 for every precision); the ROWID prefix stays
  `01 00 00 00 17` (6 more values); the logical UROWID has first byte `02`, its text is `*` + base 64 of the other bytes.

### R-037 partitions: an empty partition, subpartitions, partition statistics (18c)
- Why: `exp.md` 4.8, rule T10: one range-partitioned table without statistics. Not known: what an empty partition looks
  like on the 18c direct path (rule E2: no `ff ff`?), whether subpartitions get their own lines and binary parts, and
  where the statistics items of a partitioned table stand.
- SQL (as `CORPUS`, 18c):

      CREATE TABLE t_part2 (id NUMBER(6), k NUMBER(3), v VARCHAR2(10))
        PARTITION BY RANGE (id) SUBPARTITION BY HASH (k) SUBPARTITIONS 2
        (PARTITION p1 VALUES LESS THAN (100), PARTITION p_empty VALUES LESS THAN (200),
         PARTITION p3 VALUES LESS THAN (MAXVALUE));
      INSERT INTO t_part2 SELECT LEVEL, MOD(LEVEL, 7), 'v' || LEVEL FROM dual CONNECT BY LEVEL <= 50;
      INSERT INTO t_part2 SELECT 299 + LEVEL, MOD(LEVEL, 7), 'w' || LEVEL FROM dual CONNECT BY LEVEL <= 50;
      COMMIT;
      EXEC DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_PART2');
      -- truth_part2.txt
      SELECT 'SP|' || partition_name || '|' || subpartition_name FROM user_tab_subpartitions
       WHERE table_name = 'T_PART2' ORDER BY partition_name, subpartition_position;
      SELECT 'ROW|' || t.id || '|' || o.subobject_name FROM t_part2 t
        JOIN user_objects o ON o.data_object_id = DBMS_ROWID.ROWID_OBJECT(t.ROWID) ORDER BY t.id;

- Arithmetic to check: 100 rows; P1 ids 1 to 50, P_EMPTY none, P3 ids 300 to 349; each row's subpartition is in the ROW
  lines (hash of K, not computable here). Questions answered by the file: one binary part per partition or per
  subpartition, and which lines name them (`PARTITION "P1"`, `SUBPARTITION ...`?); the binary part of P_EMPTY on the
  conventional path (expected `ff ff` LF at once) and on the direct path (`ff ff`, or LF and the next `PARTITION` line);
  the statistics items (one table item, items per partition?) after `ENDPARTITION` or elsewhere.
- Exports: `tables=t_part2,t_frame` conventional and `direct=y`, default statistics.
- File names wanted: `part2_exp_18c.dmp`, `part2_exp_direct_18c.dmp`, logs, `truth_part2.txt`.
- Status: generated
- Analyst note (session 9): used in `exp.md` 4.8, 5.4; rules T10, E2. One binary part per **subpartition**, named by
  `SUBPARTITION "s"` inside `COMPARTITION "p"` ... `ENDCOMPARTITION`; P_EMPTY's two subpartitions are empty parts (`ff ff`
  conventional, no `ff ff` on the 18c direct path); the 16 statistics items (table + 3 partitions, each with 3 columns)
  stand after the last `ENDCOMPARTITION` and before `ENDPARTITION`.

## Requests from Analyst session 9

### R-040 LONG values above 32,767 and 65,535 bytes, by a route Oracle accepts
- Why: `exp.md` 6.6, rules Y9, R2, R8. R-031 could not create a LONG above 4000 bytes from a CLOB (ORA-22835); the largest
  LONG in the fixtures is 32,000 bytes (session 7, PL/SQL `VARCHAR2` bind). Unknown: (a) on the **conventional** path,
  how `exp` writes a value of 32,768 bytes or more: one field with a u16 length that has bit 0x8000 set (0x8000 to 0xffef
  would then be lengths), the piece chain of the direct path (bit 0x8000 = "more follows"), or something else; (b) what
  happens at 65,520 (0xfff0, where the markers `fc ff` .. `ff ff` start) and above 65,535 (no u16 can hold it); (c) on the
  **direct** path, whether a longer value is simply a longer chain. A reader built on the fixtures today would misread
  (a) or (b) if the answer is not "a chain".
- Route 1 (recommended): **SQL*Loader inside the container**, a LONG column loaded from a data file with `CHAR(n)`.
  SQL*Loader binds the field as a long character buffer, not through PL/SQL or a CLOB, so neither the 32,760-byte PL/SQL
  limit nor the 4000-byte CLOB conversion applies. `sqlldr` is part of the Oracle home of the XE images (assumed: please
  check `ls $ORACLE_HOME/bin/sqlldr` in both containers; if it is missing, use route 2). As `CORPUS`, AL32UTF8 database,
  18c and 11g:

      -- SQL (sqlplus)
      CREATE TABLE t_long3 (id NUMBER(3), l LONG, v VARCHAR2(10));

      # data file (bash, in the container): one record per line, fields separated by '|';
      # value = 'L' + 6-digit size + 'y' x (size - 7), so the first bytes identify the row
      for n in 32767 32768 65519 65520 65535 65536 100000; do
        printf '%d|L%06d' "$n" "$n"; head -c $((n - 7)) /dev/zero | tr '\0' 'y'; printf '|%s\n' "v$n"
      done > /tmp/long3.dat
      # ids: the size itself does not fit NUMBER(3); use the line number instead:
      awk -F'|' '{ printf "%d|%s|%s\n", NR, $2, $3 }' /tmp/long3.dat > /tmp/long3b.dat

      # control file /tmp/long3.ctl
      LOAD DATA
      INFILE '/tmp/long3b.dat'
      APPEND INTO TABLE t_long3
      FIELDS TERMINATED BY '|'
      (id INTEGER EXTERNAL, l CHAR(100000), v CHAR(10))

      sqlldr userid=corpus/<password> control=/tmp/long3.ctl log=/tmp/long3.log \
             readsize=1048576 bindsize=1048576 rows=1

  Rows (id: size): 1: 32,767 (0x7fff, the largest length without bit 0x8000); 2: 32,768 (0x8000); 3: 65,519 (0xffef, the
  largest u16 that is not a marker by rule R8); 4: 65,520 (0xfff0); 5: 65,535; 6: 65,536 (needs more than a u16);
  7: 100,000. V = `v` + the size, so a misplaced value shows at once.
- Truth (`truth_long3.txt`; a LONG cannot be measured in SQL, so copy it to a CLOB first, which Oracle allows with
  `TO_LOB` in `CREATE TABLE ... AS SELECT`):

      CREATE TABLE t_long3_chk AS SELECT id, TO_LOB(l) c, v FROM t_long3;
      SELECT id || '|' || DBMS_LOB.GETLENGTH(c) || '|' || RAWTOHEX(UTL_RAW.CAST_TO_RAW(DBMS_LOB.SUBSTR(c, 20, 1)))
             || '|' || RAWTOHEX(UTL_RAW.CAST_TO_RAW(DBMS_LOB.SUBSTR(c, 20, DBMS_LOB.GETLENGTH(c) - 19))) || '|' || v
        FROM t_long3_chk ORDER BY id;
      DROP TABLE t_long3_chk PURGE;

- Exports (18c and 11g): `tables=t_long3,t_frame` conventional (`long3_exp_<v>.dmp`) and `direct=y`
  (`long3_exp_direct_<v>.dmp`), `statistics=none`.
- Arithmetic to check: every value is `4c` + the 6 ASCII digits of its size + `79` x (size - 7). Hypothesis A
  (conventional = one field): row 1 length `ff 7f`, row 2 `00 80`, row 3 `ef ff`; rows 4 to 7 cannot be one field.
  Hypothesis B (conventional = the direct-path chain): row 1 `ff 7f` (one piece), rows 2 to 7 several pieces, the
  first with bit 0x8000. Direct path: chains whose piece lengths add up to the size (session 9 saw 156 + 4 x 7961 for
  32,000: the remainder first). In every file V must follow L, and T_FRAME must equal `t_frame_exp(_direct)_<v>`.
- Route 2 (if `sqlldr` is missing): **python-oracledb from the host**, binding the value as a LONG:

      import oracledb
      con = oracledb.connect(user="corpus", password="...", dsn="localhost:1521/XEPDB1")   # 18c; 11g: XE
      cur = con.cursor()
      cur.setinputsizes(None, oracledb.DB_TYPE_LONG, None)
      for i, n in enumerate([32767, 32768, 65519, 65520, 65535, 65536, 100000], 1):
          cur.execute("INSERT INTO t_long3 VALUES (:1, :2, :3)", [i, "L%06d" % n + "y" * (n - 7), "v%d" % n])
      con.commit()

  The thin mode of python-oracledb supports Oracle 12.1 and later only, so this works for 18c as is; for 11g it needs the
  thick mode (`oracledb.init_oracle_client()` with an Oracle Instant Client on the host). The same script with
  `oracledb.DB_TYPE_LONG_RAW` and `bytes` would fill an optional `t_longraw3 (id NUMBER(3), lr LONG RAW)` (LONG RAW is
  not needed for the question: session 9 shows LONG and LONG RAW framed the same way).
- Route 3 (not recommended): PL/SQL. A PL/SQL `LONG` variable holds at most 32,760 bytes, `INSERT ... SELECT` of a LONG
  column is refused (ORA-00997), and a CLOB bind fails as in R-031; I expect all three to fail above 32,760 bytes.
- Which route is most likely to work in the `gvenzl/oracle-xe` containers: route 1, because it runs inside the container
  with the tools of the Oracle home, needs no host software and no network listener, and works the same on 11g and 18c
  (assumed: `sqlldr` present; to be checked first). Route 2 is certain to bind more than 32 KB (LONG binds are what the
  driver's `DB_TYPE_LONG` is for) but needs host software and, for 11g, the Instant Client.
- File names wanted: `long3_exp_<v>.dmp`, `long3_exp_direct_<v>.dmp`, logs, `truth_long3.txt` (18c and 11g).
- Status: **proposed** (session 10: until the owner reports whether `sqlldr` exists in the 18c container; route 1
  needs it, route 2 does not). Priority: **low** unless the owner's tables have LONG columns (question Q-4); a reader can meanwhile stop
  with an error on a conventional-path LONG length from 0x8000 up, which no fixture has.
- Status (session 11): **ready**, not yet run. The owner reports `sqlldr` in the 18c container (SQL*Loader 18.4.0.0.0),
  so route 1 applies on 18c. In the 11g container `sqlldr` is not checked yet (please `ls $ORACLE_HOME/bin/sqlldr` there
  too; if it is missing, 11g can use route 2 or be left out: 18c alone answers (a) to (c) for 18c).
- Analyst check of the text (session 11, nothing run). Complete as to why, SQL, data file, control file, truth, exports,
  arithmetic and file names. Two additions: (1) the `sqlldr userid=` needs the connect string of the PDB, as every other
  corpus script uses (`corpus/scripts/config.sh` `CONNECT_CORPUS`, e.g. `corpus/<password>@//localhost:1521/XEPDB1`
  on 18c); without `@...` it connects to the root container, where `CORPUS` does not exist; (2) run the truth `SELECT`
  with the settings of the other truth files (`SET PAGESIZE 0 LINESIZE 32767 HEADING OFF FEEDBACK OFF TRIMSPOOL ON`, as
  in `corpus/sql/14_long2.sql`), so that a line is not wrapped.
- **The output lines that prove the point** (that a LONG above 65,535 bytes exists in the database and was exported):
  `sqlldr`'s log line reporting 7 rows loaded (`7 Rows successfully loaded.` is the usual wording, assumed); in `truth_long3.txt` the lines
  `6|65536|4C30363535333679797979797979797979797979|7979797979797979797979797979797979797979|v65536` and `7|100000|4C31303030303079797979797979797979797979|7979797979797979797979797979797979797979|v100000` (GETLENGTH 65,536 and 100,000; the head is
  `L065536` / `L100000` + 13 x `y`, the tail 20 x `y`); and in both `exp` logs the line `. . exporting table` ... `T_LONG3` ... `7 rows
  exported` (padded with spaces as in the other logs). With these lines present the dumps `long3_exp(_direct)_<v>.dmp` hold values the fixtures lack, and their bytes
  answer (a) to (c). If row 6 or 7 is missing from the truth, the load was refused and the request is answered negatively
  for that route.
- Status (session 17): **generated and used** (owner, 2026-10-25, route 1, SQL*Loader, both versions):
  `long3_exp_<v>.dmp`, `long3_exp_direct_<v>.dmp`, logs, `long3_sqlldr_<v>.log` (`7 Rows successfully loaded.`),
  `truth_long3.txt` (18c and 11g identical; lengths 32,767 to 100,000). Answers (`exp.md` 6.6, `tools/long3_check.py`, 0
  differences, the same on 18c and 11g): (a) conventional, 32,768 or more: **one field**, the u16 is the length with bit
  0x8000 (hypothesis A; `long3_exp_<v>.dmp`@0xa280 `00 80`, @0x12290 `ef ff`); hypothesis B refuted for the conventional
  path. (b) 65,520 is still one field `f0 ff` (@0x2228f); 65,535, 65,536 and 100,000 use `fd ff` + chunks of 4000 (+
  remainder) + `00 00` (@0x3228f, 0x422c2, 0x522f6). (c) direct path: a longer chain (pieces of 7952 / 7961 with bit
  0x8000; 100,000 = 12 x 7952 + 4576 @0x522fb); 18c and 11g cut the 32,767-byte value differently. V and T_FRAME intact
  in all four files. Open: the switch between 65,521 and 65,535 bytes, LONG RAW above 32,000: R-048.

## Requests written by the Architect (copied in Analyst session 10, note A-21)

The Architect cannot edit `docs/formats/`, so R-038, R-039 and R-041 to R-045 were written in
`docs/design/open-questions.md` only. They are copied here unchanged (text, SQL, arithmetic) so that this file lists
every request; the line `Status (requests.md)` and the Analyst notes are this file's own. If the two copies ever
differ, the text in `open-questions.md` is the Architect's and the status here is the Analyst's record.

### R-038 LOB values in the CL8MSWIN1251 database: long Cyrillic CLOB, NCLOB, BLOB (session 3) (Architect, `docs/design/open-questions.md`)
- Why: rule L5 confirms CLOB data of the single-byte database as database bytes, 1 per character, for
  short values only (R_CLOB: 6 and 6,000 bytes, `exp.md` 14.6); the long form [L4] (`fc ff`, 8-byte
  total, chunks) is seen only in AL32UTF8 (UTF-16 BE data, chunk sizes 16,264 / 8,132 on 11g). Unknown
  for the single-byte database: whether a long CLOB is still database bytes (or UTF-16), its chunk
  sizes, where the short form ends, and NCLOB and BLOB there at all (`exp.md` 14.6, "Not in the
  fixtures"). The project owner's data are Cyrillic, written from an 11.2.0.4 database; if that database
  is CL8MSWIN1251, this is the case the reader meets on every CLOB above a few KB. R-035's optional
  CL8MSWIN1251 part uses only `x` and `5a` and no NCLOB text, so it cannot show a wrong character set.
- SQL (as the R-015 user, the 11g CL8MSWIN1251 database; `CHR(n)` gives the database byte n):

      CREATE TABLE r_lob2 (id NUMBER(3), c CLOB, n NCLOB, b BLOB);
      DECLARE
        blk VARCHAR2(66);
        l_c CLOB; l_n NCLOB; l_b BLOB;
      BEGIN
        FOR i IN 192 .. 255 LOOP blk := blk || CHR(i); END LOOP;   -- 0xC0..0xFF: А..я
        blk := blk || CHR(168) || CHR(184);                        -- 0xA8 Ё, 0xB8 ё
        INSERT INTO r_lob2 VALUES (1, TO_CLOB(blk), TO_NCLOB(blk), UTL_RAW.CAST_TO_RAW(blk));
        INSERT INTO r_lob2 VALUES (2, EMPTY_CLOB(), EMPTY_CLOB(), EMPTY_BLOB())
          RETURNING c, n, b INTO l_c, l_n, l_b;
        FOR j IN 1 .. 303 LOOP
          DBMS_LOB.WRITEAPPEND(l_c, 66, blk);
          DBMS_LOB.WRITEAPPEND(l_n, 66, TO_NCHAR(blk));
          DBMS_LOB.WRITEAPPEND(l_b, 66, UTL_RAW.CAST_TO_RAW(blk));
        END LOOP;
        DBMS_LOB.WRITEAPPEND(l_c, 2, SUBSTR(blk, 1, 2));
        DBMS_LOB.WRITEAPPEND(l_n, 2, TO_NCHAR(SUBSTR(blk, 1, 2)));
        DBMS_LOB.WRITEAPPEND(l_b, 2, UTL_RAW.CAST_TO_RAW(SUBSTR(blk, 1, 2)));
        INSERT INTO r_lob2 VALUES (3, NULL, NULL, NULL);
        COMMIT;
      END;
      /
      -- truth_lob_ru.txt
      SELECT 'R_LOB2|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(c)), 'NULL') || '|'
             || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(n)), 'NULL') || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL') || '|'
             || NVL(DUMP(DBMS_LOB.SUBSTR(c, 4, 1), 16), 'NULL') || '|' || NVL(DUMP(DBMS_LOB.SUBSTR(n, 4, 1), 16), 'NULL') || '|'
             || NVL(DUMP(DBMS_LOB.SUBSTR(c, 4, DBMS_LOB.GETLENGTH(c) - 3), 16), 'NULL') || '|'
             || NVL(DUMP(DBMS_LOB.SUBSTR(n, 4, DBMS_LOB.GETLENGTH(n) - 3), 16), 'NULL')
        FROM r_lob2 ORDER BY id;

- Arithmetic to check. Row 1: C 66 characters; if database bytes (L5), the short value `42 00` + `c0 c1
  ... ff a8 b8` (66 bytes); N in UTF-16 BE (L5) `84 00` + `04 10 04 11 ... 04 4f 04 01 04 51` (132 bytes:
  U+0410 to U+044F, U+0401, U+0451); B `42 00` + the same 66 bytes as C; k = 3. Row 2: 303 x 66 = 19,998
  + 2 = 20,000 characters; the last 4 are `a8 b8 c0 c1` (end of block 303, then the first two of block
  1), so the truth's tail DUMP of C is `Typ=1 Len=4: a8,b8,c0,c1`, of N `Len=8: 4,1,4,51,4,10,4,11`.
  C: 20,000 bytes if database bytes, 40,000 if UTF-16; either can be short (`20 4e` or `40 9c`, both
  below 0xfff0) or long (`fc ff` + `20 4e 00 00 00 00 00 00` or `40 9c 00 00 00 00 00 00` + chunks); on
  11g the AL32UTF8 chunks were 8,132 bytes (BLOB) and 16,264 (CLOB): hypothesis for B 8,132 + 8,132 +
  3,736 and for N 16,264 + 16,264 + 7,472. Row 3: three `fe ff`, k = 0. GETLENGTH: 66 / 20,000 for C, N,
  B. R_TEXT after R_LOB2 must equal `truth_ru.txt` (rule T8).
- Exports (11g, CL8MSWIN1251 database): `tables=(r_lob2,r_text)` with `NLS_LANG` client CL8MSWIN1251
  (`ru_lob2_exp_cp1251_11g.dmp`) and US7ASCII (`ru_lob2_exp_us7ascii_11g.dmp`; L5 says the data do not
  depend on the client), conventional, `statistics=none`.
- File names wanted: the two dumps, logs, `truth_lob_ru.txt`. Status: proposed. Priority: high if Q-4
  shows CLOB / NCLOB columns in a single-byte database, else medium.
- Status (requests.md): **generated** (session 10; `corpus/scripts/make_ru_lob2.sh`, `corpus/sql/15_ru_lob2.sql`; files
  `ru_lob2_exp_cp1251_11g.dmp`, `ru_lob2_exp_us7ascii_11g.dmp`, logs, `truth_lob_ru.txt`; both logs "Export terminated
  successfully without warnings").
- Analyst note (session 10): used in `exp.md` 14.6 ("Session 10"); rules L4, L5, L6. Every point of the arithmetic holds:
  row 1 C `42 00` + 66 database bytes (`ru_lob2_exp_cp1251_11g.dmp`@0x274c), N `84 00` + 132 bytes UTF-16 BE (@0x2790),
  B `42 00` + 66 bytes (@0x2816), k = 3 (@0x274a). Row 2: **all three long**: C `fc ff` + `20 4e 00 00 00 00 00 00`
  (20,000 = database bytes, not 40,000) + 8,132 + 8,132 + 3,736 (@0x29c4); N `fc ff` + `40 9c 00 ..` (40,000) +
  16,264 + 16,264 + 7,472 (@0x77f4); B 8,132 + 8,132 + 3,736 (@0x11444): both hypotheses of the request hold, and C has
  the BLOB chunk size (8,132 bytes), not the AL32UTF8 CLOB chunk (16,264). Row 3: three `fe ff`, k = 0 (@0x1627e).
  The US7ASCII file has the same LOB values at the same offsets; R_TEXT equals `truth_ru.txt` and `ru_tbl_exp_*` (T8).
  One correction to the arithmetic text: the last two characters of row 2 are `c0 c1` = 'АБ' (capital), as the truth's
  tail DUMP says, not 'Аб'. Where the short form ends for C in this database is not shown (66 and 20,000 only): R-046.

### R-039 the column list of NOT NULL interval columns and default precisions (session 3) (Architect, `docs/design/open-questions.md`)
- Why: rule T9: `INTERVAL YEAR (p) TO MONTH ` ends with a space before `, ` or `)`; how ` NOT NULL
  ENABLE` follows it (one space or two) and how the default precisions are written are not in the
  fixtures. The design parses only the forms seen and falls back per column otherwise (arch 5.5), which
  costs the column its fractional precision (Duration unit).
- SQL (as `CORPUS`; 18c and 11g):

      CREATE TABLE t_ivnn (id NUMBER(3), iym INTERVAL YEAR TO MONTH NOT NULL,
                           ids INTERVAL DAY TO SECOND NOT NULL, ids9 INTERVAL DAY(9) TO SECOND(9) NOT NULL,
                           iym9 INTERVAL YEAR(9) TO MONTH);
      INSERT INTO t_ivnn VALUES (1, INTERVAL '1-2' YEAR TO MONTH, INTERVAL '3 04:05:06.7' DAY TO SECOND,
                                 INTERVAL '0 00:00:00.000000001' DAY(9) TO SECOND(9), NULL);
      COMMIT;
      -- truth_ivnn.txt
      SELECT 'COL|' || column_id || '|' || column_name || '|' || data_type || '|' || NVL(TO_CHAR(data_precision), 'NULL')
             || '|' || NVL(TO_CHAR(data_scale), 'NULL') || '|' || data_length || '|' || nullable
        FROM user_tab_columns WHERE table_name = 'T_IVNN' ORDER BY column_id;
      SELECT 'ROW|' || id || '|' || DUMP(iym, 16) || '|' || DUMP(ids, 16) || '|' || DUMP(ids9, 16) FROM t_ivnn;

- Arithmetic to check (rule Y10): IYM = 1 year 2 months = `80 00 00 01 3e` (60 + 2 = 0x3e); IDS = 3 d 04:05:06.7
  = `80 00 00 03 40 41 42 a9 b9 27 00` (hours 64, minutes 65, seconds 66; 700,000,000 = 0x29b92700, plus
  0x80000000 = 0xa9b92700); IDS9 = 1 ns = `80 00 00 00 3c 3c 3c 80 00 00 01`; IYM9 `fe ff`. Descriptors 5, 11,
  11, 5 (T5). Expected texts (hypothesis): `"IYM" INTERVAL YEAR (2) TO MONTH  NOT NULL ENABLE` (two spaces)
  or one space; `"IDS" INTERVAL DAY (2) TO SECOND (6) NOT NULL ENABLE`; `"IYM9" INTERVAL YEAR (9) TO MONTH )`.
- Exports: `tables=(t_ivnn,t_frame)`, conventional, `statistics=none`; can share R-036's export.
- File names wanted: `ivnn_exp_<v>.dmp`, log, `truth_ivnn.txt`. Status: proposed. Priority: low.
- Status (requests.md): proposed (not generated). Priority: low.

### R-041 single-column LONG and LONG RAW tables on the direct path (session 4) (Architect, `docs/design/open-questions.md`)
- Why: rules E2a, T3 v3.4: on the direct path the LONG column moves last, so a first column of
  descriptor length 0 occurs only when LONG / LONG RAW is the **only** column; then test (b) of E2 says
  nothing and the reader decides "no rows" by E2a alone (suggested, not in the fixtures). Also: a value
  whose length field has low byte 0x0a (test (a) looks at it).
- SQL (as `CORPUS`, AL32UTF8 database; 18c and 11g):

      CREATE TABLE t_long1 (l LONG);
      INSERT INTO t_long1 VALUES ('a');
      INSERT INTO t_long1 VALUES (NULL);
      DECLARE s VARCHAR2(32767) := RPAD('T', 20000, 'z');
      BEGIN INSERT INTO t_long1 VALUES (s); COMMIT; END;
      /
      CREATE TABLE t_long1e (l LONG);
      CREATE TABLE t_lraw1 (lr LONG RAW);
      INSERT INTO t_lraw1 VALUES (HEXTORAW('00112233445566778899'));
      COMMIT;

- Exports: `tables=(t_long1,t_long1e,t_lraw1,t_frame)`, conventional (`long1_exp_<v>.dmp`) and `direct=y`
  (`long1_exp_direct_<v>.dmp`), `statistics=none`; logs. No truth file needed (the SQL is the truth).
- Arithmetic to check (rules T4 to T6, R2, E1, E2, E2a; Y9): every table has `01 00`, one descriptor
  `08 00 00 00` (T_LRAW1 `18 00 00 00`), `00 00` (m) and the flag. T_LONG1, direct: row 1 `01 00 61` (one
  piece), row 2 `fe ff`, row 3 a chain of 20,000 bytes; if the remainder comes first as in session 9
  (hypothesis): 20,000 = 4,078 + 2 x 7,961, i.e. `ee 8f` + 4,078 bytes (`54` + `7a`...), `19 9f` + 7,961,
  `19 1f` + 7,961; then `ff ff 0a`. Conventional: `01 00 61` `00 00`, `fe ff` `00 00`, `20 4e` + 20,000
  bytes `00 00`, `ff ff 0a`. T_LONG1E, direct: 18c `0a` + `TABLE "T_LRAW1"` (E2a "no rows"), 11g `ff ff 0a`
  (E1); conventional `ff ff 0a`. T_LRAW1, direct: `0a 00` + the 10 bytes: the byte at p is LF but the
  next byte is `00`, so E2a reads a row; then `ff ff 0a`. T_FRAME equals `t_frame_exp(_direct)_<v>`.
- File names wanted: `long1_exp_<v>.dmp`, `long1_exp_direct_<v>.dmp`, logs. Status: proposed.
  Priority: low (E2a only).
- Status (requests.md): proposed (not generated). Priority: low.

### R-042 a partitioned table with LOB columns; NCLOB values in the long form (session 4; 18c) (Architect, `docs/design/open-questions.md`)
- Why: rule T10 lists "a partitioned table with LOB" as unknown; L4 says "NCLOB long values: not in the
  fixtures". Partitioned tables with CLOB columns are common in corporate schemas. The design reads each
  part with its own m, flag and column entries (T6, L1) and its own LOB value area (L3, L4) (ADR-21); a
  different layout would stop the file with `Framing` or `UnsupportedLayout`.
- SQL (as `CORPUS`, 18c, AL32UTF8):

      CREATE TABLE t_plob (id NUMBER(3), c CLOB, n NCLOB, b BLOB)
        PARTITION BY RANGE (id) (PARTITION p1 VALUES LESS THAN (10),
                                 PARTITION p_empty VALUES LESS THAN (20),
                                 PARTITION p3 VALUES LESS THAN (MAXVALUE));
      DECLARE l_c CLOB; l_n NCLOB; l_b BLOB;
      BEGIN
        INSERT INTO t_plob VALUES (1, TO_CLOB('abc'), TO_NCLOB('abc'), HEXTORAW('00FF'));
        INSERT INTO t_plob VALUES (2, NULL, EMPTY_CLOB(), NULL);
        INSERT INTO t_plob VALUES (30, EMPTY_CLOB(), EMPTY_CLOB(), EMPTY_BLOB()) RETURNING c, n, b INTO l_c, l_n, l_b;
        FOR j IN 1 .. 10 LOOP
          DBMS_LOB.WRITEAPPEND(l_c, 1000, RPAD('c', 1000, 'c'));
          DBMS_LOB.WRITEAPPEND(l_n, 1000, TO_NCHAR(RPAD('n', 1000, 'n')));
          DBMS_LOB.WRITEAPPEND(l_b, 1000, UTL_RAW.COPIES(HEXTORAW('B0'), 1000));
        END LOOP;
        COMMIT;
      END;
      /
      -- truth_plob.txt
      SELECT 'ROW|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(c)), 'NULL') || '|'
             || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(n)), 'NULL') || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL')
        FROM t_plob ORDER BY id;
      SELECT 'P|P1|' || COUNT(*) FROM t_plob PARTITION (p1);
      SELECT 'P|P_EMPTY|' || COUNT(*) FROM t_plob PARTITION (p_empty);
      SELECT 'P|P3|' || COUNT(*) FROM t_plob PARTITION (p3);

- Exports: `tables=(t_plob,t_frame)` conventional (`plob_exp_18c.dmp`) and `direct=y`
  (`plob_exp_direct_18c.dmp`), `statistics=none`.
- Arithmetic to check: three `PARTITION` lines, three parts, then `ENDPARTITION` (T10). Each part: `04 00`,
  descriptors (NUMBER; CLOB `70 00 48 00 69 03 01 00`; NCLOB `70 00 48 00 d0 07 02 00`; BLOB
  `71 00 48 00`), m `04 00`, flag `00 00` **in both files** (L6), entries `00 00 00`, `01 43 00 00`,
  `01 4e 00 00`, `01 42 00 00` (L1). P1: row 1 k = 3, values `06 00` + `00 61 00 62 00 63` (C), the same
  for N, `02 00 00 ff` (B); row 2: C NULL, N a locator, B NULL, so k = 1 and the value `00 00` (empty
  NCLOB); `ff ff 0a`. P_EMPTY: `ff ff 0a` right after the entries. P3: row 30, k = 3, C and N 10,000
  characters = 20,000 bytes each, B 10,000 bytes; all three long (L4: above one chunk): C `fc ff` +
  `20 4e 00 00 00 00 00 00` + chunks (hypothesis from 18c: 16,120 + 3,880), N the same (the first NCLOB
  long value), B `fc ff` + `10 27 00 00 00 00 00 00` + 8,060 + 1,940. Truth: `ROW|1|3|3|2`, `ROW|2|NULL|0|NULL`,
  `ROW|30|10000|10000|10000`; P1 2, P_EMPTY 0, P3 1. T_FRAME after it equals `t_frame_exp(_direct)_18c`.
- File names wanted: `plob_exp_18c.dmp`, `plob_exp_direct_18c.dmp`, logs, `truth_plob.txt`. Status:
  proposed. Priority: medium; high if Q-4 shows partitioned tables with LOB columns.
- Status (requests.md): proposed (not generated). Session 10: NCLOB in the long form is now seen in the CL8MSWIN1251
  database (R-038: 16,264 + 16,264 + 7,472 on 11g); R-042 is still the only request for a long NCLOB in AL32UTF8 and on 18c.

### R-043 LONG in the CL8MSWIN1251 database (session 4; 11g) (Architect, `docs/design/open-questions.md`)
- Why: rule Y9 confirms the LONG character set (database bytes, whatever the client) in AL32UTF8 only;
  for a single-byte database it says "assumed the same". The design warns (`W-LONG-CHARSET`) on a
  non-ASCII LONG value there. Can share the session of R-038 (same database, same user).
- SQL (as the R-015 user, 11g CL8MSWIN1251 database; `CHR(n)` is the database byte n):

      CREATE TABLE r_long (id NUMBER(3), l LONG, v VARCHAR2(10));
      INSERT INTO r_long VALUES (1, CHR(192) || CHR(225) || CHR(226) || ' x', 'a');
      DECLARE blk VARCHAR2(66);
      BEGIN
        FOR i IN 192 .. 255 LOOP blk := blk || CHR(i); END LOOP;
        blk := blk || CHR(168) || CHR(184);
        INSERT INTO r_long VALUES (2, blk, 'b');
        COMMIT;
      END;
      /

- Exports: `tables=(r_long,r_text)`, conventional and `direct=y`, with `NLS_LANG` client CL8MSWIN1251
  (`ru_long_exp_cp1251_11g.dmp`, `ru_long_exp_direct_cp1251_11g.dmp`) and US7ASCII
  (`ru_long_exp_us7ascii_11g.dmp`), `statistics=none`. No truth file needed beyond `truth_ru.txt` for R_TEXT.
- Arithmetic to check (Y9, R2, T3): conventional: row 1 L = `05 00 c0 e1 e2 20 78` (`Абв x`), row 2 L =
  `42 00` + `c0 c1 ... ff a8 b8` (66 bytes), with **both** clients; direct: `INSERT INTO` order ID, V, L
  and each L one piece (the same bytes); R_TEXT after R_LONG equals `truth_ru.txt` (T8).
- File names wanted: the three dumps and logs. Status: proposed. Priority: low; medium if Q-4 shows LONG
  columns in a single-byte database.
- Status (requests.md): proposed (not generated; it was not run in the R-038 session). Priority: low.

### R-044 years below 1, dates around 1582, TIME ZONE offsets at the extremes (session 5; 18c and 11g) (Architect, `docs/design/open-questions.md`)
- Why: ADR-27 items 3 and 6, ADR-28 (b). [Y2] states the BC byte pairs from two values (-0100, -4712);
  year -1 and a year of century 0 BC (-50, by the formula `64 32`) are not in the fixtures, and the
  decoder's pair rule (ADR-27 item 6) rests on the formula there. The reader keeps years below 1 an error
  because the **year numbering** is unknown (does Oracle's -0001 directly precede 0001?); and the README
  needs to know whether Oracle counts dates before 1582-10-15 in another calendar. [Y7] has three offsets;
  -12:00 and +14:00 put the formula `hour + 20, minute + 60` at its ends.
- SQL (as `CORPUS`, AL32UTF8 database):

      CREATE TABLE t_bc (id NUMBER(2), d DATE, ts TIMESTAMP(0), tz TIMESTAMP(0) WITH TIME ZONE);
      INSERT INTO t_bc VALUES (1, TO_DATE('-0001-12-31 23:59:59', 'SYYYY-MM-DD HH24:MI:SS'),
        TO_TIMESTAMP('-0001-12-31 23:59:59', 'SYYYY-MM-DD HH24:MI:SS'), NULL);
      INSERT INTO t_bc VALUES (2, TO_DATE('0001-01-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'),
        TO_TIMESTAMP('0001-01-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL);
      INSERT INTO t_bc VALUES (3, TO_DATE('-0050-07-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'),
        TO_TIMESTAMP('-0050-07-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL);
      INSERT INTO t_bc VALUES (4, TO_DATE('-0001-01-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL, NULL);
      INSERT INTO t_bc VALUES (5, TO_DATE('1582-10-04 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL, NULL);
      INSERT INTO t_bc VALUES (6, TO_DATE('1582-10-15 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL, NULL);
      INSERT INTO t_bc VALUES (7, NULL, NULL, TIMESTAMP '2020-06-01 12:00:00 -12:00');
      INSERT INTO t_bc VALUES (8, NULL, NULL, TIMESTAMP '2020-06-01 12:00:00 +14:00');
      COMMIT;

- Truth (`truth_bc.txt`): per row `id|TO_CHAR(d, 'SYYYY-MM-DD HH24:MI:SS')|DUMP(d)|TO_CHAR(d, 'J')|DUMP(ts)|
  TO_CHAR(tz, 'SYYYY-MM-DD HH24:MI:SS TZH:TZM')|DUMP(tz)`; and three lines from `dual`:
  `TO_CHAR(TO_DATE('-0001-12-31', 'SYYYY-MM-DD') + 1, 'SYYYY-MM-DD')`,
  `TO_DATE('0001-01-01', 'SYYYY-MM-DD') - TO_DATE('-0001-01-01', 'SYYYY-MM-DD')`,
  `TO_DATE('1582-10-15', 'SYYYY-MM-DD') - TO_DATE('1582-10-04', 'SYYYY-MM-DD')`.
- Expected by [Y2] (to be confirmed by `DUMP`): row 1 `64,63,c,1f,18,3c,3c` (century 0: 100 - 0 = 100;
  year 1: 100 - 1 = 99); row 2 `64,65,1,1,1,1,1` (as `truth_date` row 4); row 3 `64,32,7,1,1,1,1` (100,
  100 - 50 = 50); row 4 `64,63,1,1,1,1,1`; row 5 `73,b6,a,4,1,1,1` (15 + 100 = 115, 82 + 100 = 182); row 6
  `73,b6,a,f,1,1,1`. The TIMESTAMP(0) bytes equal the DATE bytes [Y5]. TSTZ by [Y7], UTC fields then zone
  bytes: row 7 is 2020-06-02 00:00:00 UTC, `78,78,6,2,1,1,1` and zone `8,3c` (-12 + 20 = 8, 0 + 60);
  row 8 is 2020-05-31 22:00:00 UTC, `78,78,5,1f,17,1,1` and zone `22,3c` (14 + 20 = 34). The `dual` lines
  decide: `0001-01-01` and a difference of one year (365 or 366 days) mean "no year 0"; `0000-01-01` or two
  years (730 or 731) mean a year 0. The third line is 1 if Oracle passes from the Julian to the Gregorian
  calendar at 1582-10-15, 11 if it counts the proleptic Gregorian calendar; `TO_CHAR(d, 'J')` of rows 5 and
  6 differs by the same number.
- Exports: `tables=(t_bc,t_frame)`, conventional and `direct=y`, `statistics=none`. File names wanted:
  `bc_exp_<v>.dmp`, `bc_exp_direct_<v>.dmp`, logs, `truth_bc.txt`. Status: proposed. Priority: low.
- Status (requests.md): proposed (not generated). Priority: low.

### R-045 Oracle refuses a partitioned table with a LONG or LONG RAW column (session 6; 18c) (Architect, `docs/design/open-questions.md`)
- Why: ADR-31 / ADR-32 keep a per-part exemption for the direct-path LONG-last order [T3] and T-3b tests it
  only on synthetic parts, because no fixture has a partitioned table with a LONG column. Oracle's error
  documentation states ORA-14085 "partitioned table cannot have column with LONG datatype"; for LONG RAW
  the design has only an Oracle9i Concepts sentence seen in a search result. If both statements hold on
  18c, the gap cannot be closed by a fixture and is closed by this record. No dump is produced.
- SQL (as `CORPUS`, 18c, in the same session as R-042; each statement is expected to **fail**):

      CREATE TABLE t_plong (id NUMBER(3), l LONG)
        PARTITION BY RANGE (id) (PARTITION p1 VALUES LESS THAN (10),
                                 PARTITION p2 VALUES LESS THAN (MAXVALUE));
      CREATE TABLE t_plongraw (id NUMBER(3), lr LONG RAW)
        PARTITION BY RANGE (id) (PARTITION p1 VALUES LESS THAN (10),
                                 PARTITION p2 VALUES LESS THAN (MAXVALUE));
      CREATE TABLE t_plong2 (id NUMBER(3)) PARTITION BY RANGE (id)
        (PARTITION p1 VALUES LESS THAN (10), PARTITION p2 VALUES LESS THAN (MAXVALUE));
      ALTER TABLE t_plong2 ADD (l LONG);
      DROP TABLE t_plong2 PURGE;

- Expected: statements 1 (`t_plong`), 2 (`t_plongraw`) and 4 (`ALTER ... ADD (l LONG)`) fail; 3 and 5
  succeed. By the documentation the error of 1 and 4 is ORA-14085; the code for 2 (LONG RAW) is what the
  record will state. Arithmetic: none (no table with a LONG column and more
  than one part can exist, so the number of such parts in any dump of 18c is 0).
- File wanted: the SQL*Plus spool of the five statements with their messages
  (`corpus/fixtures/18c/plong_refused.txt`, or a line in `requests.md`). Status: proposed. Priority:
  **low**; it blocks nothing (the exemption stays and is unit-tested either way).
- Status (requests.md): **proposed, not generated**; the owner will run it (with R-042's session).
- Status (session 11): **generated and used: result negative (ORA-14085), no partitioned LONG fixture can exist on 18c.**
  The owner ran it on 18c on 2026-10-17 (`corpus/scripts/run_r045.sh`, read, not run by the Analyst) and delivered
  `corpus/fixtures/18c/truth_r045.txt` (40 lines, SHA-256 prefix `b5fb62d1b3a0`, listed in `18c/SHA256SUMS`). Checked
  against the check of session 10: statement 1 (`CREATE TABLE t_plong ...`, line 10) -> `ORA-14085: partitioned table
  cannot have column with LONG datatype` (line 13); statement 2 (`t_plongraw`, LONG RAW, line 16) -> the same ORA-14085
  (line 19; the message names LONG for LONG RAW too); statement 3 -> `Table created.` (line 23; the partitioned table
  without the column is legal); statement 4 (`ALTER TABLE t_plong2 ADD (l LONG)`, line 25) -> ORA-14085 (line 28);
  statement 5 -> `Table dropped.` (line 32); then `user_tables` / `user_tab_columns` for `T_PLONG%`: `no rows selected`
  twice (lines 36, 39). The premise holds; no partitioned LONG export is needed. Gaps of the spool (do not change the
  result): statements 3 and 5 are not echoed (the script pipes into `sqlplus -s`, where `SET ECHO` has no effect), and
  there is no `SHOW USER` (the user is assumed `CORPUS` from the script's connect string). 11g: not run (assumed the same;
  the T10 partition fixtures are 18c only anyway).
- What it answers: the open point "Oracle refuses a partitioned table with a LONG / LONG RAW column" of
  `exp_reader_rules.md` section 11 (rules **T3** and **T10**: the direct-path LONG-last order can never meet a
  partitioned section in an 18c dump), and the remark on E2a in `exp.md` 5.4 (a partitioned single-column LONG table
  cannot exist). For the design: the ADR-31 / ADR-32 per-part exemption guards a case Oracle 18c cannot produce; it stays
  unit-tested only (the design's choice). No hypothesis of `docs/research/hypotheses.md` names it.
- Analyst note (session 10, request reviewed, nothing run). The text is complete as to why, SQL, expectation and file. Two
  additions so that the spool proves what it should: (1) run it in SQL*Plus with `SET ECHO ON` and `SET FEEDBACK ON`,
  `WHENEVER SQLERROR CONTINUE` (the default; a script with `WHENEVER SQLERROR EXIT` would stop after statement 1), and
  spool from before the first statement: `SPOOL /tmp/plong_refused.txt` ... `SPOOL OFF`; (2) put
  `SELECT banner FROM v$version WHERE ROWNUM = 1;` and `SHOW USER` first, so the record names the version and user.
  **Check applied to the spool**: the echoed statement 1 (`CREATE TABLE t_plong ...`) is followed by an `ERROR at line`
  block whose `ORA-` line is `ORA-14085: partitioned table cannot have column with LONG datatype`; statement 2
  (`t_plongraw`) by an `ORA-` line (its code is the finding: ORA-14085 or another); statement 3 by `Table created.`;
  statement 4 (`ALTER TABLE t_plong2 ADD (l LONG)`) by `ORA-14085`; statement 5 by `Table dropped.`. The point is proved
  by the two `ORA-` lines after statements 1 and 2 together with `Table created.` after statement 3 (the partitioned
  table itself is legal, so the refusal is about the LONG / LONG RAW column). If statement 1 or 2 prints `Table
  created.`, the premise is refuted and a partitioned LONG table must be exported (R-042's export with that table added).

### R-046 where the short form ends for CLOB and NCLOB in the CL8MSWIN1251 database (Analyst, session 10; 11g)
- Why: `exp.md` 14.6 ("Session 10"), rule L4. R-038 shows a 66-character CLOB short and a 20,000-character CLOB long,
  with chunks of 8,132 bytes (= 8,132 characters, the BLOB chunk), and an NCLOB long with chunks of 16,264 bytes. Whether
  the switch is "byte length larger than one chunk" (8,132 bytes for C, as for BLOB; hypothesis H-a, the rule that holds
  in AL32UTF8) or a fixed byte threshold of 16,264 (the AL32UTF8 CLOB chunk; hypothesis H-b) is not shown by 66 and
  20,000. A reader accepts both forms and does not need the answer; an implementer's synthetic tests and the statement
  "a short value never reaches 0xfff0" for a single-byte CLOB do. Low priority.
- SQL (as the R-015 user `CORPUS_RU`, the 11g CL8MSWIN1251 database `ora11ru`; `CHR(n)` is the database byte n):

      CREATE TABLE r_lob3 (id NUMBER(3), c CLOB, n NCLOB, b BLOB);
      DECLARE
        blk VARCHAR2(66);
        l_c CLOB; l_n NCLOB; l_b BLOB;
        PROCEDURE fill(p_id NUMBER, s PLS_INTEGER) IS
          k PLS_INTEGER := s;
          w PLS_INTEGER;
        BEGIN
          INSERT INTO r_lob3 VALUES (p_id, EMPTY_CLOB(), EMPTY_CLOB(), EMPTY_BLOB()) RETURNING c, n, b INTO l_c, l_n, l_b;
          WHILE k > 0 LOOP
            w := LEAST(k, 66);
            DBMS_LOB.WRITEAPPEND(l_c, w, SUBSTR(blk, 1, w));
            DBMS_LOB.WRITEAPPEND(l_n, w, TO_NCHAR(SUBSTR(blk, 1, w)));
            DBMS_LOB.WRITEAPPEND(l_b, w, UTL_RAW.CAST_TO_RAW(SUBSTR(blk, 1, w)));
            k := k - w;
          END LOOP;
        END;
      BEGIN
        FOR i IN 192 .. 255 LOOP blk := blk || CHR(i); END LOOP;
        blk := blk || CHR(168) || CHR(184);
        fill(1, 8132); fill(2, 8133); fill(3, 16264); fill(4, 16265);
        COMMIT;
      END;
      /
      -- truth_lob3_ru.txt
      SELECT 'R_LOB3|' || id || '|' || DBMS_LOB.GETLENGTH(c) || '|' || DBMS_LOB.GETLENGTH(n) || '|' || DBMS_LOB.GETLENGTH(b)
             || '|' || DUMP(DBMS_LOB.SUBSTR(c, 4, DBMS_LOB.GETLENGTH(c) - 3), 16)
             || '|' || DUMP(DBMS_LOB.SUBSTR(n, 4, DBMS_LOB.GETLENGTH(n) - 3), 16)
        FROM r_lob3 ORDER BY id;

- Arithmetic to check: S = 8132, 8133, 16264, 16265 characters; C = S database bytes (L5), N = 2S bytes UTF-16 BE, B = S
  bytes; every value is `blk` repeated and cut at S (8132 = 123 x 66 + 14; 8133 = 123 x 66 + 15; 16264 = 246 x 66 + 28;
  16265 = 246 x 66 + 29). H-a (long iff larger than one chunk; chunks 8,132 for C and B, 16,264 for N as in R-038): row 1
  all short (C `c4 1f` + 8,132, N `88 3f` + 16,264, B `c4 1f` + 8,132); row 2 all long (C and B 8,132 + 1, N 16,264 + 2);
  row 3 C and B 2 x 8,132, N 2 x 16,264; row 4 C and B 2 x 8,132 + 1, N 2 x 16,264 + 2. H-b (C short up to 16,264 bytes):
  C of rows 2 and 3 short (`c5 1f`, `88 3f`), C of row 4 long. B and N must follow H-a under both (they did in AL32UTF8
  and R-038). R_TEXT after R_LOB3 must equal `truth_ru.txt` (T8).
- Exports (11g, CL8MSWIN1251 database): `tables=(r_lob3,r_text)`, client CL8MSWIN1251, conventional, `statistics=none`.
  Can share the session of R-043.
- File names wanted: `ru_lob3_exp_cp1251_11g.dmp`, log, `truth_lob3_ru.txt` (folder `corpus/fixtures/11g-cp1251/`).
- Status: proposed. Priority: low (a reader accepts both forms; only tests and the 0xfff0 statement depend on it).
- Analyst check of the text (session 11, nothing run). Complete as to why, SQL, truth, arithmetic, export and file names
  (checked: `blk` is 64 + 2 = 66 single-byte characters and fits `VARCHAR2(66)`; 8,132 = `c4 1f`, 8,133 = `c5 1f`,
  16,264 = `88 3f` LE). One addition: spool the truth `SELECT` with `SET PAGESIZE 0 LINESIZE 32767 HEADING OFF FEEDBACK OFF
  TRIMSPOOL ON` (as `corpus/sql/14_long2.sql`), so the two `DUMP` texts are not wrapped. Optional: a `DUMP` of the last 4
  bytes of B (`DBMS_LOB.SUBSTR(b, 4, DBMS_LOB.GETLENGTH(b) - 3)`) would let B be checked by truth as well as by the SQL.
  Status unchanged: **proposed, low**.

## Requests written by the Architect (copied in Analyst session 16)

R-047 was written by the Architect in session 22 in `docs/design/open-questions.md` ("State after Architect session
22", section (b)). It is copied here unchanged (text, SQL, expectations); the `Status (requests.md)` line and the
Analyst note are this file's own. If the two copies ever differ, the text in `open-questions.md` is the Architect's.

### R-047 TIMESTAMP(f) for f = 1 to 5, 7 and 8: descriptor length and value bytes (session 22; 18c and 11g) (Architect, `docs/design/open-questions.md`)
- Why: [T5] gives TIMESTAMP the code 180 and [Y5] the value bytes (7 bytes in the DATE layout when the fraction is zero,
  else 11 with a u32 BE nanosecond count). The descriptor's maximum length is in the fixtures only for TIMESTAMP(0) (7),
  TIMESTAMP(6) and TIMESTAMP(9) (11) (review 06b SF-1: every `CREATE TABLE` line and its descriptor counted). The reader
  expects 7 for f = 0 and 11 otherwise; a real column with another length gets the column fallback of arch 5.5 step 4
  (`W-DDL-FALLBACK`, values still decoded, unit Microsecond), not a wrong value. The synthetic generator drew f from 0 to
  9 with the reader's length and is now limited to {0, 6, 9} until this request is answered. TIMESTAMP(3) is common in
  application schemas (Q-4 asks whether the owner's tables have it).
- SQL (as `CORPUS`, in the 18c and the 11g database):

      CREATE TABLE t_tsp (id NUMBER(2), t1 TIMESTAMP(1), t2 TIMESTAMP(2), t3 TIMESTAMP(3), t4 TIMESTAMP(4),
                          t5 TIMESTAMP(5), t7 TIMESTAMP(7), t8 TIMESTAMP(8));
      -- row 1: fraction zero everywhere
      INSERT INTO t_tsp VALUES (1, TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05',
        TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05',
        TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05');
      -- row 2: the largest fraction each precision holds
      INSERT INTO t_tsp VALUES (2, TIMESTAMP '2020-01-02 03:04:05.9', TIMESTAMP '2020-01-02 03:04:05.99',
        TIMESTAMP '2020-01-02 03:04:05.999', TIMESTAMP '2020-01-02 03:04:05.9999',
        TIMESTAMP '2020-01-02 03:04:05.99999', TIMESTAMP '2020-01-02 03:04:05.9999999',
        TIMESTAMP '2020-01-02 03:04:05.99999999');
      -- row 3: one literal with 9 digits in every column (Oracle rounds to the column's precision)
      INSERT INTO t_tsp VALUES (3, TIMESTAMP '2020-01-02 03:04:05.123456789', TIMESTAMP '2020-01-02 03:04:05.123456789',
        TIMESTAMP '2020-01-02 03:04:05.123456789', TIMESTAMP '2020-01-02 03:04:05.123456789',
        TIMESTAMP '2020-01-02 03:04:05.123456789', TIMESTAMP '2020-01-02 03:04:05.123456789',
        TIMESTAMP '2020-01-02 03:04:05.123456789');
      -- row 4: NULL everywhere
      INSERT INTO t_tsp VALUES (4, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
      COMMIT;
      -- truth_tsp.txt
      SELECT column_name || '|' || data_type || '|' || data_length || '|' || data_scale
        FROM user_tab_columns WHERE table_name = 'T_TSP' ORDER BY column_id;
      SELECT id || '|' || DUMP(t1) || '|' || DUMP(t2) || '|' || DUMP(t3) || '|' || DUMP(t4) || '|' || DUMP(t5)
             || '|' || DUMP(t7) || '|' || DUMP(t8) FROM t_tsp ORDER BY id;
      SELECT id || '|' || TO_CHAR(t1, 'YYYY-MM-DD HH24:MI:SS.FF9') || '|' || TO_CHAR(t3, 'YYYY-MM-DD HH24:MI:SS.FF9')
             || '|' || TO_CHAR(t8, 'YYYY-MM-DD HH24:MI:SS.FF9') FROM t_tsp ORDER BY id;

- Exports: `tables=(t_tsp)`, conventional and `direct=y`, both versions.
- Expected (hypothesis; the arithmetic of [Y2] and [Y5]):
  - Descriptors: `02 00 16 00` for ID; `b4 00 0b 00` (type 180, length 11) for each of the seven TIMESTAMP columns
    (hypothesis "11 for every f > 0"; a 7 or any other length refutes it for that f).
  - Row 1, every TIMESTAMP column: field `07 00` + `78 78 01 02 04 05 06` (century 20 + 100 = 0x78, year 20 + 100 = 0x78,
    month 1, day 2, hour 3 + 1, minute 4 + 1, second 5 + 1); `DUMP` = `Typ=180 Len=7: 120,120,1,2,4,5,6`.
  - Row 2: field `0b 00` + `78 78 01 02 04 05 06` + the nanoseconds as u32 BE: t1 900,000,000 = `35 a4 e9 00`; t2
    990,000,000 = `3b 02 33 80`; t3 999,000,000 = `3b 8b 87 c0`; t4 999,900,000 = `3b 99 43 60`; t5 999,990,000 =
    `3b 9a a2 f0`; t7 999,999,900 = `3b 9a c9 9c`; t8 999,999,990 = `3b 9a c9 f6` (each = 1,000,000,000 = `3b 9a ca 00`
    minus 10^(9-f)). `DUMP` of t1: `Typ=180 Len=11: 120,120,1,2,4,5,6,53,164,233,0`.
  - Row 3 (rounded, hypothesis): t1 100,000,000 = `05 f5 e1 00`; t2 120,000,000 = `07 27 0e 00`; t3 123,000,000 =
    `07 54 d4 c0`; t4 123,500,000 = `07 5c 75 e0`; t5 123,460,000 = `07 5b d9 a0`; t7 123,456,800 = `07 5b cd 20`; t8
    123,456,790 = `07 5b cd 16` (123,456,789 = `07 5b cd 15`). If Oracle truncates instead of rounding: t4 123,400,000,
    t5 123,450,000, t7 123,456,700, t8 123,456,780; the truth file decides, the reader decodes either.
  - Row 4: `fe ff` in each of the seven fields.
  - Both paths give the same bytes [R7]; Arrow (arch 5.1): Timestamp(Microsecond) for t1 to t5, Timestamp(Nanosecond)
    for t7 and t8, values as `TO_CHAR ... FF9` truncated to the unit (t1 to t5 hold no digits below microseconds).
- File names wanted: `tsp_exp_<v>.dmp`, `tsp_exp_direct_<v>.dmp`, logs, `truth_tsp.txt`.
- Priority: low unless Q-4 shows TIMESTAMP(f) with f outside {0, 6, 9} in the real tables; small (one table, two
  containers), so it can run with R-017.
- Status (requests.md): **proposed** (not generated; waits for the owner to run it, with R-017 if convenient). Priority: low.
- note (Analyst, session 16; text checked, nothing run, nothing changed above): the arithmetic was recomputed and agrees
  (every u32 BE value of rows 2 and 3, both the rounding and the truncation variants; the decimal `DUMP` of t1 row 2).
  The descriptor pattern `02 00 16 00`, `b4 00 07 00`, `b4 00 0b 00` exists for TIMESTAMP(0) / TIMESTAMP(6) in
  `18c/t_ts_exp_18c.dmp@0x2282`, `@0x2286`, `@0x228a` (18c, table mode, conventional). Points that are unclear, left
  as written: (1) the truth `SELECT`s use `DUMP(x)` (decimal), while the existing truth files use `DUMP(x, 16)` with
  `NVL(..., '<NULL>')` (`corpus/sql/02_truth_fixtures.sql`, e.g. `truth_ts.txt`); both work, but the owner's runner
  should keep the decimal form the expectations quote (`Typ=180 Len=11: 120,120,...`) or the expectations must be
  read in hex; (2) the `TO_CHAR ... FF9` line covers only t1, t3 and t8, so the rounding of t2, t4, t5 and t7 in
  row 3 is decided by `DUMP` alone (enough, but there is no text cross-check for them); (3) no SQL*Plus settings are
  named: one truth row holds eight `DUMP` texts (about 400 characters), so spool with `SET PAGESIZE 0 LINESIZE 32767
  HEADING OFF FEEDBACK OFF TRIMSPOOL ON` (as `corpus/sql/14_long2.sql`) to avoid wrapped lines; (4) no
  `statistics=` option is named for the exports (R-044 and R-046 use `statistics=none`); hypothesis: the row bytes are not
  affected either way, but naming it keeps the two versions' files comparable.
- Status (session 17): **generated and used** (owner, 2026-10-25, `make_pending.sh`, `statistics=none`): `tsp_exp_<v>.dmp`,
  `tsp_exp_direct_<v>.dmp`, logs, `truth_tsp.txt` (18c and 11g identical; decimal `DUMP`). Result (`exp.md` 6.4,
  `tools/tsp_check.py`, 0 differences): every expectation above holds. Descriptors `b4 00 0b 00` for f = 1 to 5, 7, 8
  (`tsp_exp_<v>.dmp`@0x22fb to 0x2313): "11 for every f > 0" confirmed; row 1 `07 00` + 7 bytes, rows 2 and 3 `0b 00` +
  11 bytes with the u32 BE values listed above, row 4 `fe ff` x 7; the fractions are **rounded** (t4 123,500,000 @0x23ec,
  t8 123,456,790 @0x2413); the field bytes are identical on both paths and both versions (R7).

## Requests from Analyst session 17

### R-048 conventional LONG between 65,520 and 65,535 bytes; LONG RAW above 32,000 bytes
- Why: `exp.md` 6.6, rules R2, R8, Y9. R-040 showed a conventional LONG of 65,520 bytes as one field `f0 ff` and one of
  65,535 as `fd ff` + chunks + `00 00`. Unknown: where the switch lies (is 65,521 `f1 ff`? 65,533 `fd ff` would collide
  with the chunk marker, 65,534 `fe ff` with NULL), so whether a reader may ever meet 0xfff1 to 0xfffc as a length. LONG
  RAW above 32,000 bytes is not in the fixtures on either path (assumed framed like LONG).
- SQL (as `CORPUS`, 18c and 11g; route 1 of R-040, SQL*Loader, same data generator and control file with the new sizes):

      CREATE TABLE t_long4 (id NUMBER(3), l LONG, v VARCHAR2(10));
      -- sizes, one per row, ids 1..7: 65521 65525 65530 65532 65533 65534 65535
      -- value = 'L' + 6-digit size + 'y' x (size - 7); v = 'v' + size (as R-040)
      -- truth_long4.txt: the R-040 truth SELECT with t_long4 / t_long4_chk

  LONG RAW (route 2 of R-040, python-oracledb from the host, `oracledb.DB_TYPE_LONG_RAW`; 11g needs the thick mode; may be
  left out on 11g):

      CREATE TABLE t_longraw4 (id NUMBER(3), lr LONG RAW);
      -- rows: (1, 32768 x 0xab), (2, 65520 x 0xab), (3, 65535 x 0xab), (4, 100000 x 0xab)
      -- truth: SELECT id || '|' || DBMS_LOB.GETLENGTH(TO_LOB(lr)) ... via CREATE TABLE t_longraw4_chk AS SELECT id, TO_LOB(lr) b FROM t_longraw4

- Exports: `tables=t_long4,t_frame` (and `t_longraw4,t_frame` if made), conventional and `direct=y`, `statistics=none`.
- File names wanted: `long4_exp_<v>.dmp`, `long4_exp_direct_<v>.dmp`, logs, `long4_sqlldr_<v>.log`, `truth_long4.txt`;
  optional `longraw4_exp(_direct)_<v>.dmp`, `truth_longraw4.txt`.
- Prediction (hypothesis): sizes up to some n are one field with u16 = n; above n the `fd ff` form; n < 65,533.
- Priority: low unless the owner's tables have LONG columns with values above 65,520 bytes (Q-4).
- Status: open
- note (Architect session 24, recorded in Analyst session 19, 2026-10-25): the design (ADR-95, T-19) reads the
  conventional LONG forms of R-040 and stops with `E-UNSUPPORTED-LAYOUT` on a u16 0xfff1 to 0xfffc in a conventional
  LONG field and on a conventional LONG RAW u16 from 0x8000; the LONG RAW half is the cheaper half and goes first if Q-4
  shows LONG RAW columns; a value of 65,533 or 65,534 bytes written as one field would collide with `fd ff` / `fe ff`
  (risk 29), so the sizes 65,533 and 65,534 matter most. **Result (session 19):** neither collides: 65,533 is written
  `fa ff fd ff 00 00` + 65,533 bytes (`long4_exp_<v>.dmp`@0x42295), 65,534 is `fd ff` + chunks (@0x522a6); the stops of
  ADR-95 are reached by 0xfff1 and 0xfff5 (one field, @0x2271, 0x12272) and by `fa ff` (@0x22277, 0x32285, 0x42295) in
  `long4_exp_<v>.dmp`, and by every LONG RAW value of `longraw4_exp_18c.dmp` (@0x225d `00 80`, 0xa265 `f0 ff`, 0x1a25d,
  0x2a288 `fd ff`). All of them decode; see the status line below.
- Status (session 19): **generated and used** (owner, 2026-10-25, `corpus/scripts/make_pending2.sh`): LONG on both
  versions by SQL*Loader (`long4_sqlldr_<v>.log`: `7 Rows successfully loaded.`): `long4_exp_<v>.dmp`,
  `long4_exp_direct_<v>.dmp`, logs, `truth_long4.txt` (18c = 11g); LONG RAW on 18c only by python-oracledb (thin mode needs
  12.1+): `longraw4_exp_18c.dmp`, `longraw4_exp_direct_18c.dmp`, logs, `truth_longraw4.txt`. Answers (`exp.md` 6.6, 5.6;
  `tools/long4_check.py`, 0 differences; 18c = 11g byte for byte from 0x2000): the prediction is **refuted** as stated:
  between the one-field and the `fd ff` form there is a third conventional form, `fa ff` + 4 bytes (u16 n, `00 00`) + n
  bytes. 65,521 `f1 ff` and 65,525 `f5 ff`: one field (@0x2271, 0x12272); 65,530, 65,532, 65,533: `fa ff` (@0x22277,
  0x32285, 0x42295); 65,534 and 65,535: `fd ff` + 16 x 4000 + remainder + `00 00` (@0x522a6, 0x622d8). A reader meets
  0xfff1 and 0xfff5 as lengths and 0xfffa as a marker; 0xfff6 to 0xfff9, 0xfffb, 0xfffc never stand first in these files.
  Direct path: chains (8 x 7952 + remainder). LONG RAW (18c): the LONG forms (`00 80`, `f0 ff` one field; `fd ff` chunks
  at 65,535 and 100,000; chains on the direct path with the LONG piece sizes). V and T_FRAME intact in all six files.
  Open: the exact switch (65,526 to 65,529, 65,531), LONG RAW in the `fa ff` sizes, LONG RAW on 11g: **R-051**.

### R-049 11g: an empty table without a segment, on the direct path
- Why: `exp.md` 5.4, 13.2. R-017 showed on 18c that the direct path writes `ff ff` for an empty table with a segment and
  nothing for one without. Every 11g empty table of the fixtures has a segment. Whether the 11g `exp` also omits `ff ff`
  for a table without a segment (so that the rule "segment, not version" holds on both versions) is not shown. The reader
  rule (E1 / E2 / E3) does not depend on the answer; this request only closes the record.
- SQL (as `CORPUS`, 11g only): the three `CREATE TABLE` statements and the truth `SELECT` of R-017 unchanged. If 11g XE
  refuses `SEGMENT CREATION DEFERRED` (an error in the spool), record the error; that answers the request negatively.
- Exports: `tables=(t_seg0,t_seg1,t_seg2)`, conventional and `direct=y`.
- File names wanted: `seg_exp_11g.dmp`, `seg_exp_direct_11g.dmp`, logs, `truth_seg.txt` (11g).
- Priority: low.
- Status: open
- Status (session 19): **generated and used, result negative** (owner, 2026-10-25, `corpus/scripts/make_pending2.sh`; record
  `11g/truth_r049.txt`, no dump). Oracle Database 11g Express Edition Release 11.2.0.2.0 (line 5) creates T_SEG0 with
  `SEGMENT CREATION IMMEDIATE` (lines 24-26) and refuses T_SEG2 with `ORA-00439: feature not enabled: Deferred Segment
  Creation` (lines 28-32); `user_tables` lists T_SEG0 only, `segment_created` YES (lines 35-43). So an 11g XE table
  without a segment cannot be made, and "segment, not version" cannot be tested on 11g XE (`exp.md` 5.4 session 19,
  13.2). Every 11g empty table of the fixtures has a segment, which fits the 18c finding without testing it. For a real
  11g **Enterprise Edition** dump (deferred segment creation exists there, assumed) the 11g direct-path form of an empty
  table without a segment stays unknown; the reader rule E1 / E2 / E3 accepts both forms on every version and does not
  depend on it. No follow-up request: no container of the project can create such a table.

## Requests written by the Architect (copied in Analyst session 18)

R-050 was written by the Architect in session 24 in `docs/design/open-questions.md` ("State after Architect session
24", section (b), ADR-96). It is copied here unchanged (text, SQL, expectations); the `Status (requests.md)` line and the
Analyst note are this file's own. If the two copies ever differ, the text in `open-questions.md` is the Architect's.

### R-050 TIMESTAMP WITH LOCAL TIME ZONE: descriptor length per precision, zero fraction, rounding (session 24; 18c and 11g) (Architect, `docs/design/open-questions.md`)
- Why: `exp.md` 6.5 ("TIMESTAMP WITH LOCAL TIME ZONE"), [T5], [Y8]. The fixtures show one TSLTZ column, `TIMESTAMP(6) WITH
  LOCAL TIME ZONE`, descriptor `e7 00 0b 00` (`t_tstz_exp_<v>`@0x229a), values of 11 bytes; "a TSLTZ value with a zero
  fraction (7 bytes?)" is unknown. The reader expects length 7 for f = 0 and 11 otherwise (the TIMESTAMP rule of [T5]
  v3.7, assumed for 231); another length gives `W-DDL-FALLBACK` and the descriptor's type, not a wrong value (ADR-96).
- SQL (as `CORPUS`, in the 18c and the 11g database; the session time zone fixed so that the stored instant equals the
  literal; `dbtimezone` is +00:00 in both, `truth_env.txt`):

      ALTER SESSION SET TIME_ZONE = '+00:00';
      CREATE TABLE t_tsltz2 (id NUMBER(2), l0 TIMESTAMP(0) WITH LOCAL TIME ZONE, l3 TIMESTAMP(3) WITH LOCAL TIME ZONE,
                             l9 TIMESTAMP(9) WITH LOCAL TIME ZONE);
      INSERT INTO t_tsltz2 VALUES (1, TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05',
                                      TIMESTAMP '2020-01-02 03:04:05');
      INSERT INTO t_tsltz2 VALUES (2, TIMESTAMP '2020-01-02 03:04:05.123456789', TIMESTAMP '2020-01-02 03:04:05.123456789',
                                      TIMESTAMP '2020-01-02 03:04:05.123456789');
      INSERT INTO t_tsltz2 VALUES (3, TIMESTAMP '2020-01-02 03:04:05.6', TIMESTAMP '2020-01-02 03:04:05.6',
                                      TIMESTAMP '2020-01-02 03:04:05.6');
      INSERT INTO t_tsltz2 VALUES (4, NULL, NULL, NULL);
      COMMIT;
      -- truth_tsltz2.txt
      SELECT dbtimezone || '|' || sessiontimezone FROM dual;
      SELECT column_name || '|' || data_type || '|' || data_length || '|' || data_scale
        FROM user_tab_columns WHERE table_name = 'T_TSLTZ2' ORDER BY column_id;
      SELECT id || '|' || DUMP(l0) || '|' || DUMP(l3) || '|' || DUMP(l9) FROM t_tsltz2 ORDER BY id;
      SELECT id || '|' || TO_CHAR(l0, 'YYYY-MM-DD HH24:MI:SS.FF9') || '|' || TO_CHAR(l3, 'YYYY-MM-DD HH24:MI:SS.FF9')
             || '|' || TO_CHAR(l9, 'YYYY-MM-DD HH24:MI:SS.FF9') FROM t_tsltz2 ORDER BY id;

- Exports: `tables=(t_tsltz2)`, conventional and `direct=y`, `statistics=none`, both versions.
- Expected (hypothesis; the arithmetic of [Y2], [Y5], [Y8]):
  - Descriptors: `e7 00 07 00` for L0 (type 231, length 7, as TIMESTAMP(0)); `e7 00 0b 00` for L3 and L9; `data_length` 7,
    11, 11.
  - Row 1, each column: `07 00` + `78 78 01 02 04 05 06` (century 20 + 100, year 20 + 100, month 1, day 2, hour 3 + 1,
    minute 4 + 1, second 5 + 1).
  - Row 2: L0 rounds .123456789 to 0 seconds: `07 00` + `78 78 01 02 04 05 06`; L3 `0b 00` + the same 7 bytes + 123,000,000
    = `07 54 d4 c0`; L9 `0b 00` + the 7 bytes + 123,456,789 = `07 5b cd 15`.
  - Row 3: L0 rounds .6 up: `07 00` + `78 78 01 02 04 05 07` (second 6 + 1; truncation would give `... 06`); L3 600,000,000
    = `23 c3 46 00`; L9 the same `23 c3 46 00`.
  - Row 4: `fe ff` x 3. Both paths give the same bytes [R7].
  - Arrow (arch 5.1, ADR-7): Timestamp(Microsecond) for L0, L3, Timestamp(Nanosecond) for L9, no zone, `W-TSLTZ` per column.
- File names wanted: `tsltz2_exp_<v>.dmp`, `tsltz2_exp_direct_<v>.dmp`, logs, `truth_tsltz2.txt`.
- Priority: **low**, unless Q-4 shows type 231 in the real tables; small; run with R-048 in the same containers.
- Status (requests.md): **proposed** (not generated; waits for the owner to run it, with R-048 if convenient). Priority: low.
- Status (session 19): **generated and used** (owner, 2026-10-25, `corpus/scripts/make_pending2.sh`, `corpus/sql/17_tsltz2.sql`):
  `tsltz2_exp_<v>.dmp`, `tsltz2_exp_direct_<v>.dmp`, logs, `truth_tsltz2.txt` (18c = 11g; first line `+00:00|+00:00`).
  **Every expectation above holds** (`exp.md` 6.5 session 19, `tools/tsltz2_check.py`, 0 differences): descriptors
  `e7 00 07 00`, `e7 00 0b 00`, `e7 00 0b 00` at `tsltz2_exp_<v>.dmp`@0x22cb, 0x22cf, 0x22d3 (= `data_length` 7, 11, 11);
  row 1 `07 00 78 78 01 02 04 05 06` in all three columns (@0x22df, 0x22e8, 0x22f1: 7 bytes also under a descriptor of
  11); row 2 L0 7 bytes (rounded to 0), L3 `... 07 54 d4 c0`, L9 `... 07 5b cd 15` (@0x2300, 0x2309, 0x2316); row 3 L0
  `... 05 07` (rounded **up**, @0x2329), L3 = L9 `... 23 c3 46 00` (@0x2332, 0x233f); row 4 `fe ff` x 3 (@0x2352);
  conventional = direct = 18c = 11g field bytes. The reader's rule "7 for f = 0, 11 otherwise" holds for type 231 as
  for 180. Session time zone: every value was inserted, read and exported at +00:00 (and the containers' `TZ` / `ORA_SDTZ`
  were empty, UTC clock; owner's run log in ORADMP_WORK), so R-050 says nothing about a dependence on the session zone;
  that rests on `t_tstz` rows 101 / 102 (R-019 unchanged). Nothing open; no follow-up request.
- note (Analyst, session 18; text checked, nothing run, nothing changed above): the cited descriptor `e7 00 0b 00` is at
  `t_tstz_exp_18c.dmp@0x229a` and `t_tstz_exp_11g.dmp@0x229a` (table mode, conventional), and the TSLTZ field bytes
  `0b 00 78 74 03 1a 14 23 38 07 5b ca 00` (`t_tstz_exp_<v>.dmp`@0x233d, @0x2353; `_direct`@0x232f, @0x2343) equal the
  stored bytes of `truth_tstz.txt` rows 101 / 102 (`Typ=231 Len=11: 78,74,3,1a,...`, session_tz +00:00): `exp` wrote the
  stored (database time zone) bytes there. The u32 BE values were recomputed and agree (123,000,000 = `07 54 d4 c0`,
  123,456,789 = `07 5b cd 15`, 600,000,000 = `23 c3 46 00`), and so do the DATE-layout bytes `78 78 01 02 04 05 06`
  (= `truth_tsp.txt` row 1). Points that are unclear, left as written: (1) no SQL*Plus settings are named; the truth rows
  are short (three `DUMP` texts) but spool with `SET PAGESIZE 0 LINESIZE 32767 HEADING OFF FEEDBACK OFF TRIMSPOOL ON` as
  for R-047 (`corpus/sql/16_tsp.sql`); (2) the `ALTER SESSION SET TIME_ZONE` must be in the same SQL*Plus session as the
  `INSERT`s **and** as the truth `SELECT`s (the `TO_CHAR` lines show TSLTZ in the session time zone; the
  `sessiontimezone` line records it), so a runner that splits create and truth into two `sqlplus` calls must repeat it;
  (3) the `exp` client's own session time zone is not named; hypothesis (from `t_tstz` above, one session time zone only):
  the field bytes are the stored bytes whatever the client zone, so it does not matter, but the runner could record
  `ORA_SDTZ` / `TZ` of the container in the log; (4) `data_length` 7 for L0 is not shown by any truth file yet (no fixture
  has a `TIMESTAMP(0)` row in `user_tab_columns`; only the descriptor `b4 00 07 00`, `18c/t_ts_exp_18c.dmp@0x2286`), so
  that expectation is the request's own hypothesis; (5) the truth `DUMP` is decimal (as R-047, `Typ=231 Len=7: 120,120,1,
  2,4,5,6`), not the `DUMP(x, 16)` of `truth_tstz.txt`; either works, the runner should keep the text as written; (6) unlike
  R-048 the export list has no `t_frame`; not needed here (no frame question), stated only so the runner does not add it.

## Requests from Analyst session 19

### R-051 conventional LONG of 65,526 to 65,531 bytes; LONG RAW in the `fa ff` sizes; LONG RAW above 32,000 bytes on 11g
- Why: `exp.md` 6.6 and 5.6 (session 19), rules R2, R8, Y9. R-048 showed three conventional forms: one field at 65,521
  (`f1 ff`) and 65,525 (`f5 ff`), `fa ff` + 4-byte length at 65,530, 65,532, 65,533, `fd ff` chunks from 65,534. Unknown:
  (a) where between 65,526 and 65,530 the one-field form ends (is 65,529 `f9 ff`?) and whether 65,531 uses `fa ff`
  (hypothesis: one field up to 0xfff9, `fa ff` for 0xfffa to 0xfffd, chunks from 0xfffe); a reader stops today on a
  first u16 0xfff6 to 0xfff9, 0xfffb, 0xfffc; (b) whether LONG RAW uses `fa ff` too (only the one-field and `fd ff` forms
  are seen for LONG RAW); (c) LONG RAW above 32,000 bytes on 11g (R-048 ran LONG RAW on 18c only).
- SQL, part (a) (as `CORPUS`, 18c and 11g; route 1 of R-040, SQL*Loader, the R-040 / R-048 generator and control file
  with the new sizes; `corpus/sql/17_long4_pre.sql` / `_post.sql` with `t_long5` for `t_long4`):

      CREATE TABLE t_long5 (id NUMBER(3), l LONG, v VARCHAR2(10));
      -- sizes, one per row, ids 1..6: 65526 65527 65528 65529 65530 65531   (65530 is a control: fa ff seen in R-048)
      -- value = 'L' + 6-digit size + 'y' x (size - 7); v = 'v' + size (as R-040)
      -- truth_long5.txt: the R-040 truth SELECT with t_long5 / t_long5_chk

- SQL, part (b) (18c, python-oracledb thin mode as in R-048; 11g: see (c)):

      CREATE TABLE t_longraw5 (id NUMBER(3), lr LONG RAW);
      -- rows (id, n bytes of 0xab): (1, 65521) (2, 65525) (3, 65529) (4, 65530) (5, 65531) (6, 65533) (7, 65534)
      -- truth_longraw5.txt: the truth SELECT of corpus/sql/17_longraw4_post.sql with t_longraw5 / t_longraw5_chk

- Part (c), 11g LONG RAW: the same `t_longraw5` rows, plus `(8, 32768)`, `(9, 65520)`, `(10, 65535)`, `(11, 100000)` (the
  R-048 sizes), by one of two routes, both untested: route 2 of R-040 in **thick** mode (`oracledb.init_oracle_client()`
  with an Oracle Instant Client on the host), or SQL*Loader in the 11g container with the value as **hex text** in a
  `CHAR(200000)` field (`lr CHAR(200000)`; assumed: Oracle converts the hex text to LONG RAW at INSERT, as it does for a
  character bind into RAW; the data file holds `AB` x n per record). If both fail, record the error; 18c alone then stays
  the evidence.
- Exports: `tables=t_long5,t_frame` and `tables=t_longraw5,t_frame`, conventional and `direct=y`, `statistics=none`.
- File names wanted: `long5_exp_<v>.dmp`, `long5_exp_direct_<v>.dmp`, logs, `long5_sqlldr_<v>.log`, `truth_long5.txt`
  (18c and 11g); `longraw5_exp_<v>.dmp`, `longraw5_exp_direct_<v>.dmp`, logs, `truth_longraw5.txt` (18c; 11g if (c) works).
- Prediction (hypothesis): (a) 65,526 to 65,529 one field `f6 ff` to `f9 ff`; 65,530, 65,531 `fa ff` + `fa ff 00 00` /
  `fb ff 00 00`; (b) LONG RAW = LONG for every size; (c) 11g = 18c byte for byte from 0x2000.
- What the reader does meanwhile (ADR-95 as amended by the T-19b proposal of Analyst session 19): it stops with
  `E-UNSUPPORTED-LAYOUT` on a first u16 0xfff6 to 0xfff9, 0xfffb or 0xfffc in a conventional LONG / LONG RAW field.
- Priority: low unless Q-4 shows conventional LONG / LONG RAW values between 65,521 and 65,535 bytes.
- Status: open (session 19)
- Status (session 20): **generated and used** (owner, 2026-10-25, `corpus/scripts/make_pending3.sh`): (a) on both versions by
  SQL*Loader (`long5_sqlldr_<v>.log`: `6 Rows successfully loaded.`): `long5_exp_<v>.dmp`, `long5_exp_direct_<v>.dmp`, logs,
  `truth_long5.txt` (18c = 11g); (b) on 18c by python-oracledb thin mode: `longraw5_exp_18c.dmp`, `longraw5_exp_direct_18c.dmp`,
  logs, `truth_longraw5.txt`; (c) on 11g: the hex-text route (SQL*Loader, `CHAR(200000)`) was **refused** on all 11 records
  with `ORA-01461: can bind a LONG value only for insert into a LONG column` (`11g/longraw5_sqlldr_11g.log` lines 29-60, `0 Rows
  successfully loaded.`); a second route, SQL*Loader with the value as **`LONG VARRAW(200000)`** (binary, 4-byte length in
  front, records in `"var 7"` format), loaded all 11 (`11g/longraw5_sqlldr_varraw_11g.log` line 32): `longraw5_exp_11g.dmp`,
  `longraw5_exp_direct_11g.dmp`, logs, `truth_longraw5.txt` (11 rows). **SQL*Loader with `LONG VARRAW` is a working route
  for LONG RAW values above 32,000 bytes on 11g XE** (where python-oracledb thin mode cannot connect). Answers (`exp.md` 6.6
  and 5.6 session 20; `tools/long5_check.py`, 0 differences): every prediction holds. (a) 65,526 to 65,529 one field `f6 ff`
  to `f9 ff` (`long5_exp_<v>.dmp`@0x2271, 0x12277, 0x2227e, 0x32286); 65,530 `fa ff fa ff 00 00` (@0x4228f), 65,531
  `fa ff fb ff 00 00` (@0x5229d): the switch lies between 65,529 and 65,530. (b) LONG RAW = LONG for every size: `f1 ff`,
  `f5 ff`, `f9 ff` one field, `fa ff` at 65,530, 65,531, 65,533, `fd ff` chunks at 65,534 (`longraw5_exp_<v>.dmp`@0x225d,
  0x12256, 0x22253, 0x32254, 0x4225a, 0x52261, 0x6226a). (c) 11g = 18c byte for byte from 0x2000 to the end of row 7 (both
  paths), and ids 8 to 11 equal the `longraw4_exp(_direct)_18c.dmp` fields byte for byte. V and T_FRAME intact in all eight
  files. Remainder (sizes not in any fixture: LONG / LONG RAW 65,522 to 65,524, LONG RAW 65,526 to 65,528 and 65,532): **R-052**
  (proposed, very low).

## Requests from Analyst session 20

### R-052 the last conventional LONG / LONG RAW sizes between 65,520 and 65,535 bytes (optional)
- Why: `exp.md` 6.6 and 5.6 (session 20), rules R2, R8. After R-040, R-048 and R-051 every LONG size from 65,525 to 65,535
  is in a fixture on both versions, and the forms follow one rule (one field up to 0xfff9, `fa ff` for 0xfffa to 0xfffd,
  chunks from 0xfffe). Not in any fixture: LONG and LONG RAW of **65,522, 65,523, 65,524** bytes (first u16 0xfff2 to 0xfff4
  if one field, which is the hypothesis: one field is seen at 65,521 and from 65,525), and LONG RAW of **65,526, 65,527,
  65,528 and 65,532** bytes (seen for LONG only). A reader that accepts 0xfff2 to 0xfff4 as lengths does so by
  interpolation; this request turns it into evidence. It changes no reader behaviour if the prediction holds.
- SQL, LONG (as `CORPUS`, 18c and 11g; route 1 of R-040, SQL*Loader, the R-040 / R-048 / R-051 generator and control file):

      CREATE TABLE t_long6 (id NUMBER(3), l LONG, v VARCHAR2(10));
      -- sizes, one per row, ids 1..3: 65522 65523 65524
      -- value = 'L' + 6-digit size + 'y' x (size - 7); v = 'v' + size (as R-040)
      -- truth_long6.txt: the R-040 truth SELECT with t_long6 / t_long6_chk

- SQL, LONG RAW (18c: python-oracledb thin mode as in R-048 / R-051; 11g: SQL*Loader with `LONG VARRAW`, the route of R-051
  (c), `longraw5_sqlldr_varraw_11g.log`):

      CREATE TABLE t_longraw6 (id NUMBER(3), lr LONG RAW);
      -- rows (id, n bytes of 0xab): (1, 65522) (2, 65523) (3, 65524) (4, 65526) (5, 65527) (6, 65528) (7, 65532)
      -- truth_longraw6.txt: the truth SELECT of corpus/sql/17_longraw4_post.sql with t_longraw6 / t_longraw6_chk

- Exports: `tables=t_long6,t_frame` and `tables=t_longraw6,t_frame`, conventional and `direct=y`, `statistics=none`, both
  versions.
- File names wanted: `long6_exp_<v>.dmp`, `long6_exp_direct_<v>.dmp`, logs, `long6_sqlldr_<v>.log`, `truth_long6.txt`;
  `longraw6_exp_<v>.dmp`, `longraw6_exp_direct_<v>.dmp`, logs, `truth_longraw6.txt` (11g: also the SQL*Loader log).
- Prediction (hypothesis): 65,522 to 65,524 one field `f2 ff` to `f4 ff`; LONG RAW 65,526 to 65,528 `f6 ff` to `f8 ff`, 65,532
  `fa ff fc ff 00 00` + the bytes; direct path chains of 8 x 7952 + remainder; 11g = 18c from 0x2000.
- Priority: **very low**; only if the Architect wants the reader to accept 0xfff2 to 0xfff4 on evidence rather than by
  interpolation, or Q-4 shows conventional LONG / LONG RAW values of exactly these sizes. Nothing else about the
  conventional LONG forms in 11g / 18c is open from the fixtures' side.
- Status: proposed (session 20)

## Index of all requests (session 11; updated in sessions 17, 18, 19 and 20)

| request | status | where the text is |
|---|---|---|
| R-001, R-002 | open, partly answered (session 13: what is missing is written under each request) | this file |
| R-003 | rejected (session 14, owner: not needed; answered by `lob_exp_18c.dmp`, `exp_reader_rules.md` L3) | this file |
| R-004 | rejected (session 13, owner: not needed; answered by `t_num`, `t_nullmix`, `t_date`) | this file |
| R-005, R-006, R-007 | generated and used (status set in session 13) | this file |
| R-008, R-009, R-010, R-012, R-013, R-015, R-016, R-020, R-023 to R-030, R-032 to R-037 | generated | this file |
| R-011 | superseded by R-034 | this file |
| R-014 | reserved (do not reuse) | this file |
| R-017 | **generated and used** (session 17): `18c/seg_exp(_direct)_18c.dmp`, `truth_seg.txt`; confirmed: the marker follows the segment | this file |
| R-018, R-019, R-021, R-022 | open | this file |
| R-031 | generated, part rejected (ORA-22835) | this file |
| R-038 | **generated** (session 10) | this file, copied from `docs/design/open-questions.md` |
| R-039, R-041, R-042, R-043, R-044 | proposed | this file, copied from `docs/design/open-questions.md` |
| R-040 | **generated and used** (session 17): `long3_exp(_direct)_<v>.dmp`, `long3_sqlldr_<v>.log`, `truth_long3.txt`, 18c and 11g; open remainder in R-048 | this file |
| R-045 | **generated and used** (session 11): result negative, ORA-14085 for LONG, LONG RAW and `ALTER ... ADD (l LONG)`; no partitioned LONG fixture can exist on 18c (`18c/truth_r045.txt`) | this file, copied from `docs/design/open-questions.md` |
| R-046 | proposed, low (text checked in session 11) | this file |
| R-047 | **generated and used** (session 17): `tsp_exp(_direct)_<v>.dmp`, `truth_tsp.txt`, 18c and 11g; every expectation confirmed | this file, copied from `docs/design/open-questions.md` |
| R-048 | **generated and used** (session 19): `long4_exp(_direct)_<v>.dmp`, `long4_sqlldr_<v>.log`, `truth_long4.txt` (18c and 11g), `longraw4_exp(_direct)_18c.dmp`, `truth_longraw4.txt` (18c); a third form `fa ff`; open remainder in R-051 | this file |
| R-049 | **generated and used, result negative** (session 19): `11g/truth_r049.txt`, ORA-00439 (11g XE has no deferred segment creation); no dump | this file |
| R-050 | **generated and used** (session 19): `tsltz2_exp(_direct)_<v>.dmp`, `truth_tsltz2.txt`, 18c and 11g; every expectation confirmed | this file, copied from `docs/design/open-questions.md` |
| R-051 | **generated and used** (session 20): `long5_exp(_direct)_<v>.dmp`, `long5_sqlldr_<v>.log`, `truth_long5.txt` (18c and 11g), `longraw5_exp(_direct)_<v>.dmp`, `truth_longraw5.txt` (18c and 11g), `11g/longraw5_sqlldr_11g.log` (hex text refused, ORA-01461), `11g/longraw5_sqlldr_varraw_11g.log` (`LONG VARRAW` loaded); every prediction confirmed; remainder in R-052 | this file |
| R-052 | proposed, very low (session 20) | this file |

The next free number is **R-053**.
