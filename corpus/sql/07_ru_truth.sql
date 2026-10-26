-- 07_ru_truth.sql: run as CORPUS_RU. Truth = the bytes the database holds (DUMP, hexadecimal), as
-- ASCII text, so the file does not depend on any client character set.
SET DEFINE OFF
SET LINESIZE 32767
SET PAGESIZE 0
SET FEEDBACK OFF
SET HEADING OFF
SET TRIMSPOOL ON
SET LONG 100000
WHENEVER SQLERROR EXIT FAILURE
SPOOL /fixtures/truth_ru.txt
SELECT 'ENV|' || parameter || '|' || value FROM nls_database_parameters
  WHERE parameter IN ('NLS_CHARACTERSET', 'NLS_NCHAR_CHARACTERSET') ORDER BY parameter;
SELECT 'ENV|NLS_CHARSET_ID(CL8MSWIN1251)|' || NLS_CHARSET_ID('CL8MSWIN1251') FROM dual;
SELECT 'ENV|' || banner FROM v$version WHERE ROWNUM = 1;
SELECT 'R_TEXT|' || id || '|' || DUMP(v, 16) || '|' || DUMP(c, 16) || '|' || DUMP(vc, 16) || '|' || DUMP(n, 16) || '|'
       || NVL(TO_CHAR(LENGTHB(lv)), 'NULL') || '|' || NVL(TO_CHAR(ORA_HASH(lv)), 'NULL') || '|' || DUMP(lv, 16, 1, 60) || '|'
       || NVL(TO_CHAR(d, 'YYYY-MM-DD'), 'NULL')
  FROM r_text ORDER BY id;
SELECT 'R_CLOB|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(cl)), 'NULL') || '|' || DUMP(DBMS_LOB.SUBSTR(cl, 60, 1), 16)
  FROM r_clob ORDER BY id;
SELECT 'TABLE|' || DUMP(table_name, 16) FROM user_tables ORDER BY table_name;
SELECT 'TABCOMMENT|' || DUMP(table_name, 16) || '|' || DUMP(comments, 16) FROM user_tab_comments WHERE comments IS NOT NULL ORDER BY table_name;
SELECT 'COLCOMMENT|' || DUMP(table_name, 16) || '|' || DUMP(column_name, 16) || '|' || DUMP(comments, 16)
  FROM user_col_comments WHERE comments IS NOT NULL ORDER BY table_name, column_name;
SELECT 'INDEX|' || DUMP(index_name, 16) FROM user_indexes ORDER BY index_name;
SPOOL OFF
EXIT
