-- 10_stc.sql (v1)
-- R-020 (docs/formats/requests.md, written by the Analyst): tables where not every column has statistics.
-- Run as CORPUS (make_stc.sh does this). Same SQL on 18c and 11g. Also writes /fixtures/truth_stc.txt.
-- Changes against the SQL of the request (project owner, 2026-10-08):
--   * the statistics of T_STC2 are followed by TWO added columns (added_later, added_num) and an UPDATE of added_num,
--     because the real dump has a table whose last two columns (a NUMBER and a VARCHAR2(1)) have no statistics;
--   * the truth query tests LAST_ANALYZED IS NOT NULL: USER_TAB_COL_STATISTICS lists columns without statistics too.
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

BEGIN
  FOR t IN (SELECT table_name FROM user_tables
             WHERE table_name IN ('T_STC1', 'T_STC2', 'T_STC3', 'T_STC4', 'T_STC5')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

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
-- (2) all four columns analysed, then two more columns are added (one of them is filled afterwards)
BEGIN DBMS_STATS.GATHER_TABLE_STATS(USER, 'T_STC2', method_opt => 'FOR ALL COLUMNS SIZE 1'); END;
/
ALTER TABLE t_stc2 ADD (added_later VARCHAR2(10), added_num NUMBER);
UPDATE t_stc2 SET added_num = MOD(id, 3);
COMMIT;
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

SPOOL /fixtures/truth_stc.txt
SELECT 'TAB|' || table_name || '|' || NVL(TO_CHAR(num_rows), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD'), 'NULL')
  FROM user_tables WHERE table_name LIKE 'T_STC_' ORDER BY 1;
SELECT 'COLUMN|' || c.table_name || '|' || c.column_id || '|' || c.column_name || '|'
       || NVL((SELECT 'stats:' || s.num_distinct || ':' || s.num_nulls FROM user_tab_col_statistics s
                WHERE s.table_name = c.table_name AND s.column_name = c.column_name
                  AND s.last_analyzed IS NOT NULL), 'NO STATISTICS')
  FROM user_tab_columns c WHERE c.table_name LIKE 'T_STC_' ORDER BY c.table_name, c.column_id;
SPOOL OFF
EXIT
