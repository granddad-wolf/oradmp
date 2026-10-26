-- 14_lob3.sql (v1, 2026-10-15)
-- R-035: LOB values around the short/long switch; non-ASCII and non-BMP CLOB / NCLOB
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
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name IN ('T_LOB3')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

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
SPOOL /fixtures/truth_lob3.txt
SELECT 'T_LOB3|' || id || '|' || DBMS_LOB.GETLENGTH(c) || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL') || '|'
       || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(n)), 'NULL') || '|' || DUMP(DBMS_LOB.SUBSTR(c, 20, 1), 1016)
  FROM t_lob3 ORDER BY id;
SPOOL OFF
EXIT
