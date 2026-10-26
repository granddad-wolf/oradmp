-- 11_types.sql (v1)
-- Architect requests R-024 (NUMBER boundary values), R-025 (declared types in the column list),
-- R-026(a) (NCHAR, CHAR(n CHAR)) and R-028 (LONG, LONG RAW) from docs/design/open-questions.md.
-- Run as CORPUS by make_types.sh. Same SQL on 18c and 11g. Writes /fixtures/truth_num2.txt, truth_decl.txt,
-- truth_nchar.txt. R-028 needs the table T_FRAME of the original corpus (corpus/sql/01_corpus_tables.sql).
-- DO NOT change table names or column definitions.

SET ECHO OFF
SET TERMOUT OFF
SET FEEDBACK OFF
SET HEADING OFF
SET PAGESIZE 0
SET LINESIZE 32767
SET TRIMSPOOL ON
SET VERIFY OFF
SET DEFINE OFF
SET SQLBLANKLINES ON
SET LONG 100000
WHENEVER SQLERROR EXIT FAILURE ROLLBACK

DECLARE
  n NUMBER;
BEGIN
  SELECT COUNT(*) INTO n FROM user_tables WHERE table_name = 'T_FRAME';
  IF n = 0 THEN
    RAISE_APPLICATION_ERROR(-20001, 'T_FRAME is missing: create the original corpus tables first (R-028 needs it)');
  END IF;
  FOR t IN (SELECT table_name FROM user_tables
             WHERE table_name IN ('T_NUM2', 'T_DECL', 'T_NCHAR', 'T_LONG', 'T_LONGRAW')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

-- R-024
CREATE TABLE t_num2 (id NUMBER(3), v NUMBER);
INSERT INTO t_num2 VALUES (1, 1000000);
INSERT INTO t_num2 VALUES (2, -1000000);
INSERT INTO t_num2 VALUES (3, 0.001);
INSERT INTO t_num2 VALUES (4, -0.001);
INSERT INTO t_num2 VALUES (5, 99);
INSERT INTO t_num2 VALUES (6, -99);
INSERT INTO t_num2 VALUES (7, -100);
INSERT INTO t_num2 VALUES (8, 999999999999999999);
INSERT INTO t_num2 VALUES (9, 1000000000000000000);
INSERT INTO t_num2 VALUES (10, 9223372036854775807);
INSERT INTO t_num2 VALUES (11, 9223372036854775808);
INSERT INTO t_num2 VALUES (12, -9223372036854775808);
INSERT INTO t_num2 VALUES (13, 1E-130);
INSERT INTO t_num2 VALUES (14, 0.00000000001);
COMMIT;

-- R-025
CREATE TABLE t_decl (
  n_plain NUMBER, n_star NUMBER(*,2), n_38 NUMBER(38), n_neg NUMBER(5,-2), n_sgp NUMBER(3,5),
  i_int INTEGER, d_dec DECIMAL(10,2), f_plain FLOAT, f_126 FLOAT(126), r_real REAL,
  ts_def TIMESTAMP, d_nn DATE NOT NULL, v_def VARCHAR2(10) DEFAULT 'a,b)', "low,er(x" VARCHAR2(5));
INSERT INTO t_decl VALUES (1, 1.25, 1, 100, 0.00123, 1, 1.25, 1.5, 1.5, 1.5,
  TIMESTAMP '2020-01-02 03:04:05.123456', DATE '2020-01-02', DEFAULT, 'x');
COMMIT;

-- R-026 (a)
CREATE TABLE t_nchar (id NUMBER(3), nc NCHAR(5), cc CHAR(4 CHAR), nv NVARCHAR2(5));
INSERT INTO t_nchar VALUES (1, N'ab', 'ab', N'ab');
INSERT INTO t_nchar VALUES (2, UNISTR('\0434\0436'), UNISTR('\0434\0436'), UNISTR('\0434\0436'));
INSERT INTO t_nchar VALUES (3, NULL, NULL, NULL);
COMMIT;

-- R-028
CREATE TABLE t_long (id NUMBER(3), l LONG, v VARCHAR2(10));
INSERT INTO t_long VALUES (1, 'short', 'a');
INSERT INTO t_long VALUES (2, NULL, 'b');
DECLARE s VARCHAR2(32767) := RPAD('L', 32000, 'x');
BEGIN INSERT INTO t_long VALUES (3, s, 'c'); COMMIT; END;
/
CREATE TABLE t_longraw (id NUMBER(3), lr LONG RAW);
INSERT INTO t_longraw VALUES (1, HEXTORAW('00FF'));
DECLARE r RAW(32767) := UTL_RAW.COPIES(HEXTORAW('AB'), 32000);
BEGIN INSERT INTO t_longraw VALUES (2, r); COMMIT; END;
/

SPOOL /fixtures/truth_num2.txt
SELECT id || '|' || TO_CHAR(v, 'TM9') || '|' || DUMP(v, 16) FROM t_num2 ORDER BY id;
SPOOL OFF
SPOOL /fixtures/truth_decl.txt
SELECT column_id || '|' || column_name || '|' || data_type || '|' || NVL(TO_CHAR(data_precision), 'NULL')
       || '|' || NVL(TO_CHAR(data_scale), 'NULL') || '|' || data_length || '|' || nullable
  FROM user_tab_columns WHERE table_name = 'T_DECL' ORDER BY column_id;
SPOOL OFF
SPOOL /fixtures/truth_nchar.txt
SELECT id || '|' || DUMP(nc, 16) || '|' || DUMP(cc, 16) || '|' || DUMP(nv, 16) FROM t_nchar ORDER BY id;
SPOOL OFF
EXIT
