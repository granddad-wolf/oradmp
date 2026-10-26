-- 03_truth_work.sql
-- Large ground-truth files (not committed to git). Run as CORPUS (make_work.sh does this).
-- Writes /work/truth_rows.txt (inside the container; corpus/work/<version>/ on the host).
-- Format: id|name|amount|created|flag|note, NULL shown as <NULL>.

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
WHENEVER SQLERROR CONTINUE

ALTER SESSION SET NLS_DATE_LANGUAGE = 'ENGLISH';
ALTER SESSION SET NLS_NUMERIC_CHARACTERS = '.,';
ALTER SESSION SET NLS_DATE_FORMAT = 'YYYY-MM-DD HH24:MI:SS';

SPOOL /work/truth_rows.txt
SELECT id || '|' || name || '|' || TO_CHAR(amount, 'TM') || '|' || TO_CHAR(created, 'YYYY-MM-DD HH24:MI:SS') || '|'
       || NVL(flag, '<NULL>') || '|' || NVL(note, '<NULL>')
  FROM t_rows ORDER BY id;
SPOOL OFF

SET TERMOUT ON
PROMPT Wrote /work/truth_rows.txt
EXIT
