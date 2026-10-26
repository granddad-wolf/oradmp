-- 04_probes.sql
-- Extra tables for Analyst fixture requests. Run as CORPUS (make_probes.sh does this).
--   R-006: T_PROBE6, 300 readable rows (~34 KB of row data) so that the row data cross several
--          8192-byte boundaries. Every value carries its row number.
-- Also writes /fixtures/truth_probe6.txt (format: id|value).
-- R-005 and R-007 need no new table: they re-export T_FRAME with other client settings.

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
WHENEVER SQLERROR EXIT FAILURE ROLLBACK

BEGIN
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name = 'T_PROBE6') LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

CREATE TABLE t_probe6 (id VARCHAR2(10), v VARCHAR2(100));
INSERT INTO t_probe6
SELECT 'R' || LPAD(LEVEL, 5, '0'),
       RPAD('V' || LPAD(LEVEL, 5, '0') || '_', 100, 'x')
  FROM dual CONNECT BY LEVEL <= 300;
COMMIT;

SPOOL /fixtures/truth_probe6.txt
SELECT id || '|' || v FROM t_probe6 ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_charsets.txt
SELECT 'US7ASCII='     || NLS_CHARSET_ID('US7ASCII')     FROM dual;
SELECT 'WE8MSWIN1252=' || NLS_CHARSET_ID('WE8MSWIN1252') FROM dual;
SELECT 'AL32UTF8='     || NLS_CHARSET_ID('AL32UTF8')     FROM dual;
SELECT 'AL16UTF16='    || NLS_CHARSET_ID('AL16UTF16')    FROM dual;
SPOOL OFF

SET TERMOUT ON
PROMPT Probe tables created.
EXIT
