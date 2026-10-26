-- 02_truth_fixtures.sql
-- Small ground-truth files for the committed fixtures. Run as CORPUS (make_fixtures.sh does this).
-- Writes /fixtures/truth_*.txt (inside the container; this is corpus/fixtures/<version>/ on the host).
-- Text is shown as DUMP(...) hex wherever a value could be ambiguous, so the
-- result does not depend on the terminal encoding.
-- Format: fields separated by |, NULL shown as <NULL>.

SET ECHO OFF
SET TERMOUT OFF
SET FEEDBACK OFF
SET HEADING OFF
SET PAGESIZE 0
SET LINESIZE 32767
SET LONG 100000
SET TRIMSPOOL ON
SET VERIFY OFF
SET DEFINE OFF
SET SQLBLANKLINES ON
SET NULL '<NULL>'
WHENEVER SQLERROR CONTINUE

ALTER SESSION SET NLS_DATE_LANGUAGE = 'ENGLISH';
ALTER SESSION SET NLS_NUMERIC_CHARACTERS = '.,';
ALTER SESSION SET NLS_DATE_FORMAT = 'YYYY-MM-DD HH24:MI:SS';
ALTER SESSION SET TIME_ZONE = '+00:00';

-- Environment: Oracle version and character sets of this database
SPOOL /fixtures/truth_env.txt
SELECT 'version=' || version FROM product_component_version WHERE ROWNUM = 1;
SELECT parameter || '=' || value FROM nls_database_parameters
 WHERE parameter IN ('NLS_CHARACTERSET', 'NLS_NCHAR_CHARACTERSET', 'NLS_LANGUAGE', 'NLS_TERRITORY');
SELECT 'dbtimezone=' || DBTIMEZONE FROM dual;
SPOOL OFF

-- Row order for tables without id follows insertion order (heap table, no deletes).
SPOOL /fixtures/truth_frame.txt
SELECT NVL(a, '<NULL>') || '|' || NVL(b, '<NULL>') || '|' || NVL(c, '<NULL>') FROM t_frame;
SPOOL OFF

SPOOL /fixtures/truth_len.txt
SELECT id || '|' || NVL(TO_CHAR(LENGTH(a)), '<NULL>') || '|' || DUMP(a, 16, 1, 12) FROM t_len ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_nullmix.txt
SELECT id || '|' || NVL(TO_CHAR(n, 'TM'), '<NULL>') || '|' || NVL(s, '<NULL>') || '|' || NVL(TO_CHAR(d, 'YYYY-MM-DD HH24:MI:SS'), '<NULL>')
  FROM t_nullmix ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_num.txt
SELECT id || '|' || NVL(TO_CHAR(v, 'TM'), '<NULL>') || '|' || NVL(DUMP(v, 16), '<NULL>') FROM t_num ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_numtypes.txt
SELECT id || '|' || NVL(TO_CHAR(n_p10, 'TM'), '<NULL>') || '|' || NVL(TO_CHAR(n_p10s2, 'TM'), '<NULL>') || '|'
       || NVL(TO_CHAR(n_neg, 'TM'), '<NULL>') || '|' || NVL(TO_CHAR(n_fl, 'TM'), '<NULL>') || '|'
       || NVL(DUMP(n_p10s2, 16), '<NULL>') || '|' || NVL(DUMP(n_neg, 16), '<NULL>')
  FROM t_numtypes ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_float.txt
SELECT id || '|' || NVL(TO_CHAR(bf), '<NULL>') || '|' || NVL(DUMP(bf, 16), '<NULL>') || '|'
       || NVL(TO_CHAR(bd), '<NULL>') || '|' || NVL(DUMP(bd, 16), '<NULL>')
  FROM t_float ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_date.txt
SELECT id || '|' || NVL(TO_CHAR(d, 'SYYYY-MM-DD HH24:MI:SS'), '<NULL>') || '|' || NVL(DUMP(d, 16), '<NULL>')
  FROM t_date ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_ts.txt
SELECT id || '|' || NVL(TO_CHAR(ts0, 'YYYY-MM-DD HH24:MI:SS'), '<NULL>') || '|' || NVL(DUMP(ts0, 16), '<NULL>') || '|'
       || NVL(TO_CHAR(ts6, 'YYYY-MM-DD HH24:MI:SS.FF6'), '<NULL>') || '|' || NVL(DUMP(ts6, 16), '<NULL>') || '|'
       || NVL(TO_CHAR(ts9, 'YYYY-MM-DD HH24:MI:SS.FF9'), '<NULL>') || '|' || NVL(DUMP(ts9, 16), '<NULL>')
  FROM t_ts ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_tstz.txt
SELECT id || '|tstz=' || NVL(TO_CHAR(tstz, 'YYYY-MM-DD HH24:MI:SS.FF6 TZH:TZM TZR'), '<NULL>') || '|' || NVL(DUMP(tstz, 16), '<NULL>')
       || '|tsltz=' || NVL(TO_CHAR(tsltz, 'YYYY-MM-DD HH24:MI:SS.FF6'), '<NULL>') || '|' || NVL(DUMP(tsltz, 16), '<NULL>')
       || '|session_tz=' || SESSIONTIMEZONE
  FROM t_tstz ORDER BY id;
SPOOL OFF

-- RAW values of up to 2000 bytes are 4000 hex characters: too long for one concatenated line
-- (ORA-01489). Each value is therefore written in two parts of at most 2000 hex characters
-- (1000 bytes): part=1 for every row, part=2 only for values longer than 1000 bytes.
-- Format: id|len=<bytes>|part=<1 or 2>|<hex>
SPOOL /fixtures/truth_raw.txt
SELECT id || '|len=' || NVL(TO_CHAR(UTL_RAW.LENGTH(r)), '<NULL>') || '|part=1|' || NVL(SUBSTR(RAWTOHEX(r), 1, 2000), '<NULL>')
  FROM t_raw ORDER BY id;
SELECT id || '|len=' || TO_CHAR(UTL_RAW.LENGTH(r)) || '|part=2|' || SUBSTR(RAWTOHEX(r), 2001, 2000)
  FROM t_raw WHERE UTL_RAW.LENGTH(r) > 1000 ORDER BY id;
SPOOL OFF

-- Text: lengths in characters and bytes, and the raw bytes with character set names (format 1016)
SPOOL /fixtures/truth_text.txt
SELECT id || '|v_chars=' || NVL(TO_CHAR(LENGTH(v)), '<NULL>') || '|v_bytes=' || NVL(TO_CHAR(LENGTHB(v)), '<NULL>')
       || '|' || NVL(DUMP(v, 1016), '<NULL>')
       || '|' || NVL(DUMP(c, 1016), '<NULL>')
       || '|' || NVL(DUMP(nv, 1016), '<NULL>')
  FROM t_text ORDER BY id;
SPOOL OFF

-- Wide table: one line per row, columns separated by |
SET COLSEP '|'
SPOOL /fixtures/truth_wide.txt
SELECT * FROM t_wide;
SPOOL OFF
SET COLSEP ' '

SPOOL /fixtures/truth_ddl.txt
SELECT id || '|' || NVL(TO_CHAR(parent_id), '<NULL>') || '|' || status || '|' || NVL(TO_CHAR(amount, 'TM'), '<NULL>') || '|'
       || TO_CHAR(created, 'YYYY-MM-DD HH24:MI:SS')
  FROM t_ddl ORDER BY id;
SPOOL OFF

-- Row counts of the core tables
SPOOL /fixtures/truth_counts.txt
SELECT table_name || '|' || num_rows FROM (
  SELECT 'T_FRAME' AS table_name, COUNT(*) AS num_rows FROM t_frame UNION ALL
  SELECT 'T_LEN', COUNT(*) FROM t_len UNION ALL
  SELECT 'T_NULLMIX', COUNT(*) FROM t_nullmix UNION ALL
  SELECT 'T_NUM', COUNT(*) FROM t_num UNION ALL
  SELECT 'T_NUMTYPES', COUNT(*) FROM t_numtypes UNION ALL
  SELECT 'T_FLOAT', COUNT(*) FROM t_float UNION ALL
  SELECT 'T_DATE', COUNT(*) FROM t_date UNION ALL
  SELECT 'T_TS', COUNT(*) FROM t_ts UNION ALL
  SELECT 'T_TSTZ', COUNT(*) FROM t_tstz UNION ALL
  SELECT 'T_RAW', COUNT(*) FROM t_raw UNION ALL
  SELECT 'T_TEXT', COUNT(*) FROM t_text UNION ALL
  SELECT 'T_ROWS', COUNT(*) FROM t_rows UNION ALL
  SELECT 'T_EMPTY', COUNT(*) FROM t_empty UNION ALL
  SELECT 'T_WIDE', COUNT(*) FROM t_wide UNION ALL
  SELECT 'T_PARENT', COUNT(*) FROM t_parent UNION ALL
  SELECT 'T_DDL', COUNT(*) FROM t_ddl
);
SPOOL OFF

SET TERMOUT ON
PROMPT Truth files written to /fixtures/truth_*.txt
EXIT
