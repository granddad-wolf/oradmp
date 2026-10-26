-- 14_part2.sql (v1, 2026-10-15)
-- R-037 (18c only): an empty partition, subpartitions, partition statistics
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
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name IN ('T_PART2')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

CREATE TABLE t_part2 (id NUMBER(6), k NUMBER(3), v VARCHAR2(10))
  PARTITION BY RANGE (id) SUBPARTITION BY HASH (k) SUBPARTITIONS 2
  (PARTITION p1 VALUES LESS THAN (100), PARTITION p_empty VALUES LESS THAN (200),
   PARTITION p3 VALUES LESS THAN (MAXVALUE));
INSERT INTO t_part2 SELECT LEVEL, MOD(LEVEL, 7), 'v' || LEVEL FROM dual CONNECT BY LEVEL <= 50;
INSERT INTO t_part2 SELECT 299 + LEVEL, MOD(LEVEL, 7), 'w' || LEVEL FROM dual CONNECT BY LEVEL <= 50;
COMMIT;
EXEC DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_PART2');
SPOOL /fixtures/truth_part2.txt
SELECT 'SP|' || partition_name || '|' || subpartition_name FROM user_tab_subpartitions
 WHERE table_name = 'T_PART2' ORDER BY partition_name, subpartition_position;
SELECT 'ROW|' || t.id || '|' || o.subobject_name FROM t_part2 t
  JOIN user_objects o ON o.data_object_id = DBMS_ROWID.ROWID_OBJECT(t.ROWID) ORDER BY t.id;
SPOOL OFF
EXIT
