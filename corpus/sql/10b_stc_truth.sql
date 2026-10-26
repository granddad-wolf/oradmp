-- 10b_stc_truth.sql (v1)
-- Truth for the extra exports of make_stc_extra.sh (R-023): the statistics that exist in T_STC1..5 right now,
-- written immediately before those exports. Same line format as truth_stc.txt (10_stc.sql).
-- Why: the database may gather statistics on its own between make_stc.sh and make_stc_extra.sh; the first
-- run of R-023 (2026-10-08) found T_STC2 and T_STC4 analysed in the US7ASCII dumps, not in the default dumps.
SET ECHO OFF
SET TERMOUT OFF
SET FEEDBACK OFF
SET HEADING OFF
SET PAGESIZE 0
SET LINESIZE 32767
SET TRIMSPOOL ON
SET VERIFY OFF
SET DEFINE OFF
WHENEVER SQLERROR EXIT FAILURE ROLLBACK

SPOOL /fixtures/truth_stc_extra.txt
SELECT 'TAB|' || table_name || '|' || NVL(TO_CHAR(num_rows), 'NULL') || '|' || NVL(TO_CHAR(last_analyzed, 'YYYY-MM-DD'), 'NULL')
  FROM user_tables WHERE table_name LIKE 'T_STC_' ORDER BY 1;
SELECT 'COLUMN|' || c.table_name || '|' || c.column_id || '|' || c.column_name || '|'
       || NVL((SELECT 'stats:' || s.num_distinct || ':' || s.num_nulls FROM user_tab_col_statistics s
                WHERE s.table_name = c.table_name AND s.column_name = c.column_name
                  AND s.last_analyzed IS NOT NULL), 'NO STATISTICS')
  FROM user_tab_columns c WHERE c.table_name LIKE 'T_STC_' ORDER BY c.table_name, c.column_id;
SPOOL OFF
EXIT
