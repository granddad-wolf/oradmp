-- 14_misc2.sql (v1, 2026-10-15)
-- R-036: INTERVAL with mixed negative fields and other precisions; more ROWIDs; a logical UROWID (index-organised table)
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
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name IN ('T_INTERVAL2','T_ROWID2','T_IOT')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

DECLARE n NUMBER;
BEGIN
  SELECT COUNT(*) INTO n FROM user_tables WHERE table_name = 'T_PROBE6';
  IF n = 0 THEN RAISE_APPLICATION_ERROR(-20001, 'T_PROBE6 is missing in CORPUS (R-006 table)'); END IF;
END;
/
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
SPOOL /fixtures/truth_misc2.txt
SELECT 'INTERVAL|' || id || '|' || TO_CHAR(iym) || '|' || DUMP(iym, 16) || '|' || TO_CHAR(ids) || '|' || DUMP(ids, 16)
       || '|' || TO_CHAR(ids0) || '|' || DUMP(ids0, 16) FROM t_interval2 ORDER BY id;
SELECT 'ROWID|' || id || '|' || NVL(ROWIDTOCHAR(rid), 'NULL') || '|' || NVL(DUMP(rid, 16), 'NULL') || '|'
       || CAST(urid AS VARCHAR2(4000)) || '|' || DUMP(urid, 16) FROM t_rowid2 ORDER BY id;
SPOOL OFF
EXIT
