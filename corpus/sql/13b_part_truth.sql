-- 13b_part_truth.sql (v1) R-030, 18c only. Run as CORPUS after 01b_optional_tables.sql. Writes /fixtures/truth_part.txt.
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
SPOOL /fixtures/truth_part.txt
SELECT 'P|' || partition_name || '|' || num_rows_dummy FROM (
  SELECT 'P2019' AS partition_name, COUNT(*) AS num_rows_dummy FROM t_part PARTITION (p2019)
  UNION ALL SELECT 'P2020', COUNT(*) FROM t_part PARTITION (p2020)
  UNION ALL SELECT 'PMAX', COUNT(*) FROM t_part PARTITION (pmax));
SELECT 'TOTAL|' || COUNT(*) || '|' || SUM(amount) || '|' || MIN(sale_date) || '|' || MAX(sale_date) FROM t_part;
SPOOL OFF
EXIT
