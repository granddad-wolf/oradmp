-- 14_nchar2.sql (v1, 2026-10-15)
-- R-033: characters outside the BMP in NCHAR, NVARCHAR2, CHAR(n CHAR), VARCHAR2(n CHAR), CLOB, NCLOB
-- Run as CORPUS by make_extra.sh. Same SQL on 18c and 11g unless said otherwise. Writes the truth file(s) to /fixtures.
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
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name IN ('T_NCHAR2')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

CREATE TABLE t_nchar2 (id NUMBER(3), nc NCHAR(3), nv NVARCHAR2(3), cc CHAR(3 CHAR), vc VARCHAR2(3 CHAR), cl CLOB, ncl NCLOB);
INSERT INTO t_nchar2 VALUES (1, UNISTR('a\D83D\DE00'), UNISTR('a\D83D\DE00'), UNISTR('a\D83D\DE00'), UNISTR('a\D83D\DE00'),
                             TO_CLOB(UNISTR('a\D83D\DE00')), TO_NCLOB(UNISTR('a\D83D\DE00')));
COMMIT;
SPOOL /fixtures/truth_nchar2.txt
SELECT id || '|' || DUMP(nc, 16) || '|' || DUMP(nv, 16) || '|' || DUMP(cc, 16) || '|' || DUMP(vc, 16)
       || '|' || DBMS_LOB.GETLENGTH(cl) || '|' || DBMS_LOB.GETLENGTH(ncl)
       || '|' || DUMP(DBMS_LOB.SUBSTR(cl, 10, 1), 16) || '|' || DUMP(DBMS_LOB.SUBSTR(ncl, 10, 1), 16)
  FROM t_nchar2;
SPOOL OFF
EXIT
