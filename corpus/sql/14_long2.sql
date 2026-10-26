-- 14_long2.sql (v1, 2026-10-15)
-- R-031: LONG above 65,533 bytes, LONG on the direct path, a non-ASCII LONG; also an empty LONG table for note A-7
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
  FOR t IN (SELECT table_name FROM user_tables WHERE table_name IN ('T_LONG2','T_LONG_EMPTY')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

-- tolerant: binding a CLOB to a LONG column may be refused by Oracle; that is recorded in the truth file, not fatal
WHENEVER SQLERROR CONTINUE
DECLARE n NUMBER;
BEGIN
  SELECT COUNT(*) INTO n FROM user_tables WHERE table_name IN ('T_LONG', 'T_LONGRAW', 'T_FRAME');
  IF n <> 3 THEN RAISE_APPLICATION_ERROR(-20001, 'T_LONG, T_LONGRAW or T_FRAME is missing (run make_types.sh first)'); END IF;
END;
/
CREATE TABLE t_long2 (id NUMBER(3), l LONG, v VARCHAR2(10));
CREATE TABLE t_long_empty (l LONG, v VARCHAR2(10));
INSERT INTO t_long2 VALUES (1, UNISTR('\0434\0436 Caf\00E9'), 'a');
COMMIT;
SPOOL /fixtures/truth_long2.txt
DECLARE
  c CLOB;
  v VARCHAR2(32767);
BEGIN
  DBMS_LOB.CREATETEMPORARY(c, TRUE);
  FOR i IN 1 .. 10 LOOP
    DBMS_LOB.WRITEAPPEND(c, 10000, RPAD(TO_CHAR(i), 10000, 'y'));
  END LOOP;
  BEGIN
    EXECUTE IMMEDIATE 'INSERT INTO t_long2 VALUES (2, :1, ''b'')' USING c;
    COMMIT;
    DBMS_OUTPUT.PUT_LINE('2|100000|' || RAWTOHEX(UTL_RAW.CAST_TO_RAW('1' || RPAD('y', 19, 'y'))) || '|constructed, not read back');
  EXCEPTION WHEN OTHERS THEN
    DBMS_OUTPUT.PUT_LINE('2|INSERT REFUSED|' || REPLACE(SQLERRM, 'ORA-', 'ORA_'));
  END;
  SELECT l INTO v FROM t_long2 WHERE id = 1;
  DBMS_OUTPUT.PUT_LINE('1|' || LENGTHB(v) || '|' || RAWTOHEX(UTL_RAW.CAST_TO_RAW(SUBSTR(v, 1, 20))));
END;
/
SPOOL OFF
EXIT
