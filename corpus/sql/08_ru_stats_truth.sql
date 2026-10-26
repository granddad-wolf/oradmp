-- 08_ru_stats_truth.sql (v1): run as CORPUS_RU. Truth for R-016 = what the data dictionary holds about
-- the statistics (user_tables, user_tab_col_statistics, user_tab_histograms, user_indexes), as ASCII
-- text. Texts are shown as DUMP(...,16) (the bytes in the database set).
SET DEFINE OFF
SET LINESIZE 32767
SET PAGESIZE 0
SET FEEDBACK OFF
SET HEADING OFF
SET TRIMSPOOL ON
SET LONG 100000
WHENEVER SQLERROR EXIT FAILURE
SPOOL /fixtures/truth_stat.txt
SELECT 'ENV|' || parameter || '|' || value FROM nls_database_parameters
  WHERE parameter IN ('NLS_CHARACTERSET', 'NLS_NCHAR_CHARACTERSET') ORDER BY parameter;
SELECT 'ENV|' || banner FROM v$version WHERE ROWNUM = 1;
-- TAB|table|num_rows|blocks|empty_blocks|avg_space|chain_cnt|avg_row_len|sample_size|last_analyzed
SELECT 'TAB|' || table_name || '|' || NVL(TO_CHAR(num_rows), 'NULL') || '|' || NVL(TO_CHAR(blocks), 'NULL') || '|'
       || NVL(TO_CHAR(empty_blocks), 'NULL') || '|' || NVL(TO_CHAR(avg_space), 'NULL') || '|'
       || NVL(TO_CHAR(chain_cnt), 'NULL') || '|' || NVL(TO_CHAR(avg_row_len), 'NULL') || '|'
       || NVL(TO_CHAR(sample_size), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD HH24:MI:SS'), 'NULL')
  FROM user_tables ORDER BY table_name;
-- COL|table|column|num_distinct|num_nulls|num_buckets|density|histogram|avg_col_len|sample_size|low_value|high_value|last_analyzed
SELECT 'COL|' || table_name || '|' || column_name || '|' || NVL(TO_CHAR(num_distinct), 'NULL') || '|'
       || NVL(TO_CHAR(num_nulls), 'NULL') || '|' || NVL(TO_CHAR(num_buckets), 'NULL') || '|'
       || NVL(TO_CHAR(density, 'TM9'), 'NULL') || '|' || histogram || '|' || NVL(TO_CHAR(avg_col_len), 'NULL') || '|'
       || NVL(TO_CHAR(sample_size), 'NULL') || '|' || NVL(RAWTOHEX(low_value), 'NULL') || '|'
       || NVL(RAWTOHEX(high_value), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD HH24:MI:SS'), 'NULL')
  FROM user_tab_col_statistics ORDER BY table_name, column_name;
-- HIST|table|column|endpoint_number|endpoint_value|endpoint_actual_value (DUMP, hexadecimal)
SELECT 'HIST|' || table_name || '|' || column_name || '|' || endpoint_number || '|'
       || NVL(TO_CHAR(endpoint_value, 'TM9'), 'NULL') || '|' || NVL(DUMP(endpoint_actual_value, 16), 'NULL')
  FROM user_tab_histograms
  ORDER BY table_name, column_name, endpoint_number;
-- IND|index|table|uniqueness|blevel|leaf_blocks|distinct_keys|avg_leaf_blocks_per_key|avg_data_blocks_per_key|clustering_factor|num_rows|sample_size|last_analyzed
SELECT 'IND|' || index_name || '|' || table_name || '|' || uniqueness || '|' || NVL(TO_CHAR(blevel), 'NULL') || '|'
       || NVL(TO_CHAR(leaf_blocks), 'NULL') || '|' || NVL(TO_CHAR(distinct_keys), 'NULL') || '|'
       || NVL(TO_CHAR(avg_leaf_blocks_per_key), 'NULL') || '|' || NVL(TO_CHAR(avg_data_blocks_per_key), 'NULL') || '|'
       || NVL(TO_CHAR(clustering_factor), 'NULL') || '|' || NVL(TO_CHAR(num_rows), 'NULL') || '|'
       || NVL(TO_CHAR(sample_size), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD HH24:MI:SS'), 'NULL')
  FROM user_indexes ORDER BY index_name;
SPOOL OFF
EXIT
