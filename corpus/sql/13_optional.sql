-- 13_optional.sql (v1)
-- Architect requests R-027 (LOB), R-029 (INTERVAL, ROWID, UROWID) and R-030 (partitions) from docs/design/open-questions.md.
-- Run as CORPUS by make_optional.sh AFTER 01b_optional_tables.sql (T_LOB, T_INTERVAL, T_PART must exist; T_PART only on 18c).
-- Writes /fixtures/truth_lob.txt, truth_misc.txt and (if T_PART exists) truth_part.txt.
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
  SELECT COUNT(*) INTO n FROM user_tables WHERE table_name IN ('T_LOB', 'T_INTERVAL', 'T_FRAME');
  IF n <> 3 THEN
    RAISE_APPLICATION_ERROR(-20001, 'T_LOB, T_INTERVAL or T_FRAME is missing (run 01b_optional_tables.sql and the original corpus first)');
  END IF;
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name IN ('T_LOB2', 'T_ROWID')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

-- R-027
CREATE TABLE t_lob2 (id NUMBER(3), a CLOB, v VARCHAR2(10), n NCLOB, b BLOB);
INSERT INTO t_lob2 VALUES (1, TO_CLOB('abc'), 'x', TO_NCLOB('abc'), HEXTORAW('00FF'));
INSERT INTO t_lob2 VALUES (2, NULL, 'y', EMPTY_CLOB(), NULL);
COMMIT;

-- R-029
CREATE TABLE t_rowid (id NUMBER(3), rid ROWID, urid UROWID);
INSERT INTO t_rowid SELECT 1, ROWID, ROWID FROM t_frame WHERE ROWNUM = 1;
INSERT INTO t_rowid VALUES (2, NULL, NULL);
COMMIT;

SPOOL /fixtures/truth_lob.txt
SELECT 'T_LOB|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(c)), 'NULL') || '|'
       || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL') || '|' || NVL(RAWTOHEX(DBMS_LOB.SUBSTR(b, 30, 1)), 'NULL')
  FROM t_lob ORDER BY id;
SELECT 'T_LOB2|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(a)), 'NULL') || '|' || NVL(v, 'NULL') || '|'
       || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(n)), 'NULL') || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL')
  FROM t_lob2 ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_misc.txt
SELECT 'INTERVAL|' || id || '|' || NVL(TO_CHAR(iym), 'NULL') || '|' || NVL(DUMP(iym, 16), 'NULL') || '|'
       || NVL(TO_CHAR(ids), 'NULL') || '|' || NVL(DUMP(ids, 16), 'NULL')
  FROM t_interval ORDER BY id;
SELECT 'ROWID|' || id || '|' || NVL(ROWIDTOCHAR(rid), 'NULL') || '|' || NVL(DUMP(rid, 16), 'NULL') || '|'
       || NVL(DUMP(urid, 16), 'NULL')
  FROM t_rowid ORDER BY id;
SPOOL OFF

EXIT
