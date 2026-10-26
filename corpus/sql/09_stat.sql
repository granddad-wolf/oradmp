-- 09_stat.sql (v1)
-- R-016 part a (docs/formats/requests.md, written by the Analyst): tables with optimizer statistics.
-- Run as CORPUS (make_stat.sh does this). Same SQL on 18c and 11g. Also writes /fixtures/truth_stat.txt.
--   T_STAT1  statistics with histograms on every column (FOR ALL COLUMNS SIZE 10), index T_STAT1_IX + primary key
--   T_STAT2  the same data, statistics without histograms (SIZE 1)
--   T_STAT3  the same data, old-style ANALYZE TABLE ... COMPUTE STATISTICS
--   T_STAT4  an empty table with statistics
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
             WHERE table_name IN ('T_STAT1', 'T_STAT2', 'T_STAT3', 'T_STAT4')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

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

SPOOL /fixtures/truth_stat.txt
SELECT 'ROW|' || id || '|' || v || '|' || n || '|' || TO_CHAR(d, 'YYYY-MM-DD') FROM t_stat1 ORDER BY id;
-- the Analyst's lines
SELECT 'TABSTAT|' || table_name || '|' || num_rows || '|' || blocks || '|' || avg_row_len
  FROM user_tab_statistics WHERE table_name LIKE 'T_STAT_' ORDER BY 1;
SELECT 'COLSTAT|' || table_name || '|' || column_name || '|' || num_distinct || '|' || num_buckets || '|' || histogram
  FROM user_tab_col_statistics WHERE table_name LIKE 'T_STAT_' ORDER BY 1;
-- more detail (same layout as truth_stat.txt of R-016 part b)
SELECT 'TAB|' || table_name || '|' || NVL(TO_CHAR(num_rows), 'NULL') || '|' || NVL(TO_CHAR(blocks), 'NULL') || '|'
       || NVL(TO_CHAR(empty_blocks), 'NULL') || '|' || NVL(TO_CHAR(avg_space), 'NULL') || '|'
       || NVL(TO_CHAR(chain_cnt), 'NULL') || '|' || NVL(TO_CHAR(avg_row_len), 'NULL') || '|'
       || NVL(TO_CHAR(sample_size), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD HH24:MI:SS'), 'NULL')
  FROM user_tables WHERE table_name LIKE 'T_STAT_' ORDER BY table_name;
SELECT 'COL|' || table_name || '|' || column_name || '|' || NVL(TO_CHAR(num_distinct), 'NULL') || '|'
       || NVL(TO_CHAR(num_nulls), 'NULL') || '|' || NVL(TO_CHAR(num_buckets), 'NULL') || '|'
       || NVL(TO_CHAR(density, 'TM9'), 'NULL') || '|' || histogram || '|' || NVL(TO_CHAR(avg_col_len), 'NULL') || '|'
       || NVL(TO_CHAR(sample_size), 'NULL') || '|' || NVL(RAWTOHEX(low_value), 'NULL') || '|'
       || NVL(RAWTOHEX(high_value), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD HH24:MI:SS'), 'NULL')
  FROM user_tab_col_statistics WHERE table_name LIKE 'T_STAT_' ORDER BY table_name, column_name;
SELECT 'HIST|' || table_name || '|' || column_name || '|' || endpoint_number || '|'
       || NVL(TO_CHAR(endpoint_value, 'TM9'), 'NULL') || '|' || NVL(DUMP(endpoint_actual_value, 16), 'NULL')
  FROM user_tab_histograms WHERE table_name LIKE 'T_STAT_' ORDER BY table_name, column_name, endpoint_number;
SELECT 'IND|' || index_name || '|' || table_name || '|' || uniqueness || '|' || NVL(TO_CHAR(blevel), 'NULL') || '|'
       || NVL(TO_CHAR(leaf_blocks), 'NULL') || '|' || NVL(TO_CHAR(distinct_keys), 'NULL') || '|'
       || NVL(TO_CHAR(avg_leaf_blocks_per_key), 'NULL') || '|' || NVL(TO_CHAR(avg_data_blocks_per_key), 'NULL') || '|'
       || NVL(TO_CHAR(clustering_factor), 'NULL') || '|' || NVL(TO_CHAR(num_rows), 'NULL') || '|'
       || NVL(TO_CHAR(sample_size), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD HH24:MI:SS'), 'NULL')
  FROM user_indexes WHERE table_name LIKE 'T_STAT_' ORDER BY index_name;
SPOOL OFF
EXIT
