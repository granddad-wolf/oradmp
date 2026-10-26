-- 16_seg.sql (v1, 2026-10-25)
-- R-017: empty tables with and without a segment on 18c. Run as CORPUS by make_pending.sh (18c only).
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
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name IN ('T_SEG0','T_SEG1','T_SEG2')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/
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
SPOOL /fixtures/truth_seg.txt
SELECT t.table_name || '|segment_created=' || t.segment_created || '|segments='
       || (SELECT COUNT(*) FROM user_segments s WHERE s.segment_name = t.table_name)
       || '|rows=0'
  FROM user_tables t WHERE t.table_name LIKE 'T_SEG_' ORDER BY 1;
SPOOL OFF
EXIT
