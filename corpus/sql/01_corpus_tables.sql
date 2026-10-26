-- 01_corpus_tables.sql
-- Core test corpus for the oradump project. Run as user CORPUS.
-- Works on Oracle XE 11g and 18c. Fully deterministic: no random data, no SYSDATE.
-- All non-ASCII text is written with UNISTR escapes, so this file is pure ASCII.
--
-- Table purposes:
--   T_FRAME     row framing, NULL markers, trailing NULLs (readable strings, no id column)
--   T_LEN       column length prefix boundaries (250/251 bytes and others)
--   T_NULLMIX   all 8 NULL/non-NULL combinations of NUMBER, VARCHAR2, DATE
--   T_NUM       NUMBER encoding: signs, fractions, extremes, 20-byte mantissa case
--   T_NUMTYPES  NUMBER(p), NUMBER(p,s), negative scale, FLOAT
--   T_FLOAT     BINARY_FLOAT / BINARY_DOUBLE including NaN and infinity
--   T_DATE      DATE encoding incl. midnight, century boundaries, BC dates
--   T_TS        TIMESTAMP(0), (6), (9)
--   T_TSTZ      TIMESTAMP WITH TIME ZONE and WITH LOCAL TIME ZONE
--   T_RAW       RAW values, including 2000 bytes
--   T_TEXT      VARCHAR2, CHAR, NVARCHAR2 with multi-byte characters
--   T_ROWS      100,000 rows, many blocks
--   T_EMPTY     table without rows
--   T_WIDE      120 columns, many trailing NULLs
--   T_PARENT, T_DDL, T_SEQ, V_DDL   constraints, defaults, comments, index, sequence, view

SET SQLBLANKLINES ON
SET DEFINE OFF
SET ECHO OFF
SET FEEDBACK ON
WHENEVER SQLERROR EXIT FAILURE ROLLBACK

ALTER SESSION SET NLS_DATE_LANGUAGE = 'ENGLISH';
ALTER SESSION SET NLS_NUMERIC_CHARACTERS = '.,';
ALTER SESSION SET NLS_DATE_FORMAT = 'YYYY-MM-DD HH24:MI:SS';
ALTER SESSION SET TIME_ZONE = '+00:00';

-- Start clean: drop every table, the sequence and the view of this schema.
BEGIN
  FOR t IN (SELECT table_name FROM user_tables) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" CASCADE CONSTRAINTS PURGE';
  END LOOP;
  FOR s IN (SELECT sequence_name FROM user_sequences) LOOP
    EXECUTE IMMEDIATE 'DROP SEQUENCE "' || s.sequence_name || '"';
  END LOOP;
  FOR v IN (SELECT view_name FROM user_views) LOOP
    EXECUTE IMMEDIATE 'DROP VIEW "' || v.view_name || '"';
  END LOOP;
END;
/

---------------------------------------------------------------------------
-- T_FRAME: readable strings. Look for AAAA, BBBB, ... in a hex dump.
---------------------------------------------------------------------------
CREATE TABLE t_frame (a VARCHAR2(10), b VARCHAR2(10), c VARCHAR2(10));
INSERT INTO t_frame VALUES ('AAAA', 'BBBB', 'CCCC');
INSERT INTO t_frame VALUES ('DDDD', NULL, 'FFFF');
INSERT INTO t_frame VALUES ('GGGG', NULL, NULL);
INSERT INTO t_frame VALUES (NULL, NULL, NULL);
INSERT INTO t_frame VALUES (NULL, 'HHHH', NULL);
INSERT INTO t_frame VALUES ('K', 'LL', 'MMM');
INSERT INTO t_frame VALUES ('NNNNNNNNNN', 'OOOOOOOOOO', 'PPPPPPPPPP');
COMMIT;

---------------------------------------------------------------------------
-- T_LEN: string lengths around the 250/251 byte boundary and others.
-- Each value starts with 'L<length>_' so it can be found in a hex dump.
---------------------------------------------------------------------------
CREATE TABLE t_len (id NUMBER(4), a VARCHAR2(400));
BEGIN
  FOR r IN (SELECT column_value AS n
              FROM TABLE(sys.odcinumberlist(1, 2, 3, 10, 126, 127, 128, 129, 200, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 300, 400))) LOOP
    INSERT INTO t_len VALUES (r.n, RPAD(SUBSTR('L' || r.n || '_', 1, r.n), r.n, 'x'));
  END LOOP;
  INSERT INTO t_len VALUES (0, NULL);
  COMMIT;
END;
/

---------------------------------------------------------------------------
-- T_NULLMIX: every NULL pattern of three typed columns (8 rows).
-- Bit 0 set -> n is NULL, bit 1 set -> s is NULL, bit 2 set -> d is NULL.
---------------------------------------------------------------------------
CREATE TABLE t_nullmix (id NUMBER(2), n NUMBER, s VARCHAR2(10), d DATE);
INSERT INTO t_nullmix
SELECT lvl + 1,
       CASE WHEN BITAND(lvl, 1) = 0 THEN 11 END,
       CASE WHEN BITAND(lvl, 2) = 0 THEN 'SSSS' END,
       CASE WHEN BITAND(lvl, 4) = 0 THEN DATE '2020-02-29' END
  FROM (SELECT LEVEL - 1 AS lvl FROM dual CONNECT BY LEVEL <= 8);
COMMIT;

---------------------------------------------------------------------------
-- T_NUM: NUMBER encoding.
-- Row 23 and row 32 have 20 mantissa bytes (no trailing 102 byte for the negative one).
---------------------------------------------------------------------------
CREATE TABLE t_num (id NUMBER(3), v NUMBER);
INSERT INTO t_num VALUES (1, 0);
INSERT INTO t_num VALUES (2, 1);
INSERT INTO t_num VALUES (3, -1);
INSERT INTO t_num VALUES (4, 5);
INSERT INTO t_num VALUES (5, -5);
INSERT INTO t_num VALUES (6, 10);
INSERT INTO t_num VALUES (7, 100);
INSERT INTO t_num VALUES (8, 123);
INSERT INTO t_num VALUES (9, -123);
INSERT INTO t_num VALUES (10, 100000);
INSERT INTO t_num VALUES (11, 1234567);
INSERT INTO t_num VALUES (12, 0.5);
INSERT INTO t_num VALUES (13, 0.05);
INSERT INTO t_num VALUES (14, -0.5);
INSERT INTO t_num VALUES (15, 0.1);
INSERT INTO t_num VALUES (16, 0.01);
INSERT INTO t_num VALUES (17, 123.456);
INSERT INTO t_num VALUES (18, -123.456);
INSERT INTO t_num VALUES (19, 1E33);
INSERT INTO t_num VALUES (20, 1000000000000000000000000000000001);
INSERT INTO t_num VALUES (21, 12345678901234567890123456789012345678);
INSERT INTO t_num VALUES (22, -12345678901234567890123456789012345678);
INSERT INTO t_num VALUES (23, 1234567890123456789012345678901234567.8);
INSERT INTO t_num VALUES (24, 99999999999999999999999999999999999999);
INSERT INTO t_num VALUES (25, 0.12345678901234567890123456789012345678);
INSERT INTO t_num VALUES (26, 1E125);
INSERT INTO t_num VALUES (27, 1E-125);
INSERT INTO t_num VALUES (28, 9.9999999999999999999999999999999999999E125);
INSERT INTO t_num VALUES (29, -1E125);
INSERT INTO t_num VALUES (30, 3.1415926535897932384626433832795028841);
INSERT INTO t_num VALUES (31, 1 / 3);
INSERT INTO t_num VALUES (32, -1234567890123456789012345678901234567.8);
INSERT INTO t_num VALUES (33, NULL);
COMMIT;

CREATE TABLE t_numtypes (
  id       NUMBER(3),
  n_p10    NUMBER(10),
  n_p10s2  NUMBER(10, 2),
  n_neg    NUMBER(5, -2),
  n_fl     FLOAT(24)
);
INSERT INTO t_numtypes VALUES (1, 0, 0, 0, 0);
INSERT INTO t_numtypes VALUES (2, 42, 42.5, 12300, 1.5);
INSERT INTO t_numtypes VALUES (3, -42, -42.55, -12300, -1.5);
INSERT INTO t_numtypes VALUES (4, 9999999999, 99999999.99, 9999900, 123456.75);
INSERT INTO t_numtypes VALUES (5, NULL, NULL, NULL, NULL);
COMMIT;

---------------------------------------------------------------------------
-- T_FLOAT: IEEE types
---------------------------------------------------------------------------
CREATE TABLE t_float (id NUMBER(3), bf BINARY_FLOAT, bd BINARY_DOUBLE);
INSERT INTO t_float VALUES (1, 0f, 0d);
INSERT INTO t_float VALUES (2, 1.5f, 1.5d);
INSERT INTO t_float VALUES (3, -1.5f, -1.5d);
INSERT INTO t_float VALUES (4, 3.14159f, 3.141592653589793d);
INSERT INTO t_float VALUES (5, BINARY_FLOAT_NAN, BINARY_DOUBLE_NAN);
INSERT INTO t_float VALUES (6, BINARY_FLOAT_INFINITY, BINARY_DOUBLE_INFINITY);
INSERT INTO t_float VALUES (7, -BINARY_FLOAT_INFINITY, -BINARY_DOUBLE_INFINITY);
INSERT INTO t_float VALUES (8, 3.4028235E38f, 1.7976931348623157E308d);
INSERT INTO t_float VALUES (9, 1.17549435E-38f, 2.2250738585072014E-308d);
INSERT INTO t_float VALUES (10, NULL, NULL);
COMMIT;

---------------------------------------------------------------------------
-- T_DATE
---------------------------------------------------------------------------
CREATE TABLE t_date (id NUMBER(3), d DATE);
INSERT INTO t_date VALUES (1, TO_DATE('2012-09-17 13:31:01', 'YYYY-MM-DD HH24:MI:SS'));
INSERT INTO t_date VALUES (2, TO_DATE('2016-03-26 00:00:00', 'YYYY-MM-DD HH24:MI:SS'));
INSERT INTO t_date VALUES (3, TO_DATE('1992-11-30 15:17:00', 'YYYY-MM-DD HH24:MI:SS'));
INSERT INTO t_date VALUES (4, TO_DATE('0001-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'));
INSERT INTO t_date VALUES (5, TO_DATE('9999-12-31 23:59:59', 'YYYY-MM-DD HH24:MI:SS'));
INSERT INTO t_date VALUES (6, TO_DATE('2000-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'));
INSERT INTO t_date VALUES (7, TO_DATE('1999-12-31 23:59:59', 'YYYY-MM-DD HH24:MI:SS'));
INSERT INTO t_date VALUES (8, TO_DATE('0100-06-15 12:00:00 BC', 'YYYY-MM-DD HH24:MI:SS BC'));
INSERT INTO t_date VALUES (9, TO_DATE('4712-01-01 00:00:00 BC', 'YYYY-MM-DD HH24:MI:SS BC'));
INSERT INTO t_date VALUES (10, TO_DATE('2020-02-29 12:00:00', 'YYYY-MM-DD HH24:MI:SS'));
INSERT INTO t_date VALUES (11, NULL);
COMMIT;

---------------------------------------------------------------------------
-- T_TS: plain TIMESTAMP at three precisions
---------------------------------------------------------------------------
CREATE TABLE t_ts (id NUMBER(3), ts0 TIMESTAMP(0), ts6 TIMESTAMP(6), ts9 TIMESTAMP(9));
INSERT INTO t_ts VALUES (1, TIMESTAMP '2016-03-26 15:09:27', TIMESTAMP '2016-03-26 15:09:27.649491', TIMESTAMP '2016-03-26 15:09:27.649491000');
INSERT INTO t_ts VALUES (2, TIMESTAMP '2016-03-26 16:09:27', TIMESTAMP '2016-03-26 16:09:27.000000', TIMESTAMP '2016-03-26 16:09:27.000000000');
INSERT INTO t_ts VALUES (3, TIMESTAMP '2016-03-26 16:09:27', TIMESTAMP '2016-03-26 16:09:27.000001', TIMESTAMP '2016-03-26 16:09:27.000000001');
INSERT INTO t_ts VALUES (4, TIMESTAMP '1970-01-01 00:00:00', TIMESTAMP '1970-01-01 00:00:00.5', TIMESTAMP '2000-01-01 00:00:00.123456789');
INSERT INTO t_ts VALUES (5, TIMESTAMP '9999-12-31 23:59:59', TIMESTAMP '9999-12-31 23:59:59.999999', TIMESTAMP '9999-12-31 23:59:59.999999999');
INSERT INTO t_ts VALUES (6, NULL, NULL, NULL);
COMMIT;

---------------------------------------------------------------------------
-- T_TSTZ: TIMESTAMP WITH TIME ZONE (offset and region) and WITH LOCAL TIME ZONE.
-- Rows 101/102 store the same instant (19:34:55.123456 UTC) from two session time zones.
---------------------------------------------------------------------------
CREATE TABLE t_tstz (id NUMBER(3), tstz TIMESTAMP(6) WITH TIME ZONE, tsltz TIMESTAMP(6) WITH LOCAL TIME ZONE);
INSERT INTO t_tstz (id, tstz) VALUES (1, TIMESTAMP '2016-03-26 19:34:55.123456 +05:30');
INSERT INTO t_tstz (id, tstz) VALUES (2, TIMESTAMP '2016-03-26 19:34:55.123456 +00:00');
INSERT INTO t_tstz (id, tstz) VALUES (3, TIMESTAMP '2016-03-26 19:34:55.123456 -08:00');
INSERT INTO t_tstz (id, tstz) VALUES (4, TIMESTAMP '2016-03-26 19:34:55.123456 Europe/Moscow');
INSERT INTO t_tstz (id, tstz) VALUES (5, TIMESTAMP '2016-03-26 19:34:55.123456 UTC');
INSERT INTO t_tstz (id, tstz) VALUES (6, TIMESTAMP '2016-07-01 12:00:00 America/New_York');
INSERT INTO t_tstz (id, tstz) VALUES (7, NULL);

ALTER SESSION SET TIME_ZONE = '+00:00';
INSERT INTO t_tstz (id, tsltz) VALUES (101, TIMESTAMP '2016-03-26 19:34:55.123456');
ALTER SESSION SET TIME_ZONE = '+05:30';
INSERT INTO t_tstz (id, tsltz) VALUES (102, TIMESTAMP '2016-03-27 01:04:55.123456');
ALTER SESSION SET TIME_ZONE = '+00:00';
INSERT INTO t_tstz (id, tsltz) VALUES (103, NULL);
COMMIT;

---------------------------------------------------------------------------
-- T_RAW
---------------------------------------------------------------------------
CREATE TABLE t_raw (id NUMBER(3), r RAW(2000));
INSERT INTO t_raw VALUES (1, HEXTORAW('00FF807F'));
INSERT INTO t_raw VALUES (2, HEXTORAW('00'));
INSERT INTO t_raw VALUES (3, NULL);
INSERT INTO t_raw VALUES (4, HEXTORAW(RPAD('AB', 4000, 'AB')));
INSERT INTO t_raw VALUES (5, HEXTORAW(RPAD('CD', 502, 'CD')));
INSERT INTO t_raw VALUES (6, HEXTORAW(RPAD('EF', 510, 'EF')));
INSERT INTO t_raw VALUES (7, HEXTORAW(RPAD('12', 512, '12')));
COMMIT;

---------------------------------------------------------------------------
-- T_TEXT: character data. Non-ASCII only via UNISTR escapes.
-- Characters the database character set cannot hold are replaced by Oracle at insert time;
-- that is itself useful to know (check the truth file).
---------------------------------------------------------------------------
CREATE TABLE t_text (id NUMBER(3), v VARCHAR2(200), c CHAR(10), nv NVARCHAR2(50));
INSERT INTO t_text VALUES (1, 'plain ascii', 'abc', 'plain ascii');
INSERT INTO t_text VALUES (2, UNISTR('\041F\0440\0438\0432\0435\0442, \043C\0438\0440'), UNISTR('\0434\0430'), UNISTR('\041F\0440\0438\0432\0435\0442'));
INSERT INTO t_text VALUES (3, UNISTR('Caf\00E9 Z\00FCrich \00F1'), UNISTR('\00E9\00E8'), UNISTR('Caf\00E9 Z\00FCrich'));
INSERT INTO t_text VALUES (4, UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5'), UNISTR('\65E5\672C\8A9E'));
INSERT INTO t_text VALUES (5, UNISTR('\D83D\DE00'), 'x', UNISTR('\D83D\DE00'));
INSERT INTO t_text VALUES (6, NULL, NULL, NULL);
INSERT INTO t_text VALUES (7, 'trailing spaces   ', 'ab', 'trailing spaces   ');
INSERT INTO t_text VALUES (8, RPAD('long', 200, '.'), 'abcdefghij', RPAD('long', 50, '.'));
INSERT INTO t_text VALUES (9, 'line1' || CHR(10) || 'line2' || CHR(9) || 'tab', 'q', 'line1' || CHR(10) || 'line2');
INSERT INTO t_text VALUES (10, 'a,"b";c|d' || CHR(39) || 'e', 'z', 'a,"b";c|d');
COMMIT;

---------------------------------------------------------------------------
-- T_ROWS: 100,000 deterministic rows (several thousand blocks)
---------------------------------------------------------------------------
CREATE TABLE t_rows (
  id      NUMBER(10),
  name    VARCHAR2(40),
  amount  NUMBER(12, 2),
  created DATE,
  flag    CHAR(1),
  note    VARCHAR2(200)
);
INSERT INTO t_rows
SELECT lvl,
       'name_' || lvl,
       MOD(lvl * 7919, 1000000) / 100,
       DATE '2020-01-01' + MOD(lvl, 366) + MOD(lvl, 86400) / 86400,
       CASE MOD(lvl, 3) WHEN 0 THEN 'Y' WHEN 1 THEN 'N' END,
       CASE WHEN MOD(lvl, 5) = 0 THEN NULL ELSE RPAD('n', MOD(lvl, 150) + 1, 'z') END
  FROM (SELECT LEVEL AS lvl FROM dual CONNECT BY LEVEL <= 100000);
COMMIT;

---------------------------------------------------------------------------
-- T_EMPTY: no rows
---------------------------------------------------------------------------
CREATE TABLE t_empty (id NUMBER, name VARCHAR2(20), d DATE);

---------------------------------------------------------------------------
-- T_WIDE: 120 columns. Row 1 all values, row 2 all NULL,
-- row 3 only the last column set, row 4 only the first column set.
---------------------------------------------------------------------------
DECLARE
  l_cols  VARCHAR2(4000);
  l_full  VARCHAR2(4000);
  l_nulls VARCHAR2(4000);
  l_last  VARCHAR2(4000);
  l_first VARCHAR2(4000);
  l_sep   VARCHAR2(1);
BEGIN
  FOR i IN 1 .. 120 LOOP
    l_sep   := CASE WHEN i > 1 THEN ',' END;
    l_cols  := l_cols  || l_sep || 'c' || LPAD(i, 3, '0') || ' VARCHAR2(8)';
    l_full  := l_full  || l_sep || '''v' || LPAD(i, 3, '0') || '''';
    l_nulls := l_nulls || l_sep || 'NULL';
    l_last  := l_last  || l_sep || CASE WHEN i = 120 THEN '''last''' ELSE 'NULL' END;
    l_first := l_first || l_sep || CASE WHEN i = 1 THEN '''first''' ELSE 'NULL' END;
  END LOOP;
  EXECUTE IMMEDIATE 'CREATE TABLE t_wide (' || l_cols || ')';
  EXECUTE IMMEDIATE 'INSERT INTO t_wide VALUES (' || l_full  || ')';
  EXECUTE IMMEDIATE 'INSERT INTO t_wide VALUES (' || l_nulls || ')';
  EXECUTE IMMEDIATE 'INSERT INTO t_wide VALUES (' || l_last  || ')';
  EXECUTE IMMEDIATE 'INSERT INTO t_wide VALUES (' || l_first || ')';
  COMMIT;
END;
/

---------------------------------------------------------------------------
-- DDL variety: constraints, defaults, check, comments, index, sequence, view
---------------------------------------------------------------------------
CREATE TABLE t_parent (id NUMBER(6) PRIMARY KEY, name VARCHAR2(30) NOT NULL);
INSERT INTO t_parent VALUES (1, 'p1');
INSERT INTO t_parent VALUES (2, 'p2');

CREATE TABLE t_ddl (
  id        NUMBER(6) CONSTRAINT t_ddl_pk PRIMARY KEY,
  parent_id NUMBER(6) CONSTRAINT t_ddl_fk REFERENCES t_parent (id),
  status    VARCHAR2(10) DEFAULT 'NEW' NOT NULL CONSTRAINT t_ddl_ck CHECK (status IN ('NEW', 'DONE')),
  amount    NUMBER(10, 2),
  created   DATE DEFAULT DATE '2020-01-01'
);
INSERT INTO t_ddl VALUES (1, 1, 'NEW', 10.5, DATE '2021-05-05');
INSERT INTO t_ddl VALUES (2, 2, 'DONE', NULL, DATE '2021-06-06');
INSERT INTO t_ddl (id) VALUES (3);
COMMIT;

CREATE INDEX t_ddl_ix ON t_ddl (amount, created);
COMMENT ON TABLE t_ddl IS 'table with constraints and defaults';
COMMENT ON COLUMN t_ddl.amount IS 'amount in euro';
CREATE SEQUENCE t_seq START WITH 100 INCREMENT BY 5;
CREATE VIEW v_ddl AS SELECT id, status FROM t_ddl;

PROMPT Core corpus created.
EXIT
