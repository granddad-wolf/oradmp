-- 14_probe11.sql (v1, 2026-10-15)
-- R-034 (replaces R-011): VARCHAR2(n CHAR) and CHAR(n CHAR) in an AL32UTF8 database, with the byte caps
-- Run as CORPUS by make_extra.sh. Same SQL on 18c and 11g unless said otherwise. Writes the truth file(s) to /fixtures.
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
SET SERVEROUTPUT ON SIZE UNLIMITED
WHENEVER SQLERROR EXIT FAILURE ROLLBACK

BEGIN
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name IN ('T_PROBE11')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

CREATE TABLE t_probe11 (id NUMBER(3), b VARCHAR2(10 BYTE), c VARCHAR2(10 CHAR), k CHAR(3 CHAR),
                        w VARCHAR2(1000 CHAR), x VARCHAR2(1001 CHAR), y CHAR(500 CHAR), z CHAR(501 CHAR));
INSERT INTO t_probe11 VALUES (1, 'abc', 'abc', 'abc', 'abc', 'abc', 'abc', 'abc');
INSERT INTO t_probe11 VALUES (2, NULL, UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'),
                              UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'), UNISTR('\65E5\672C\8A9E'));
COMMIT;
SPOOL /fixtures/truth_probe11.txt
SELECT 'COL|' || column_name || '|' || data_type || '|' || data_length || '|' || char_length || '|' || char_used
  FROM user_tab_columns WHERE table_name = 'T_PROBE11' ORDER BY column_id;
SELECT 'ROW|' || id || '|' || NVL(DUMP(b, 16), 'NULL') || '|' || DUMP(c, 16) || '|' || DUMP(k, 16) || '|'
       || DUMP(w, 16) || '|' || DUMP(x, 16) || '|' || LENGTHB(y) || '|' || LENGTHB(z)
  FROM t_probe11 ORDER BY id;
SPOOL OFF
EXIT
