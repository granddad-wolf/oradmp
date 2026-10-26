-- 06_bn.sql
-- R-013 (docs/formats/requests.md): record boundary inside a length field / inside an `fe ff` marker.
-- Run as CORPUS (make_bn.sh does this). Same SQL on 18c and 11g.
-- DO NOT change table names or column definitions: the arithmetic of R-013 depends on the first row
-- starting at 0x2249 (T_BN1, T_BN2) and 0x2251 (T_BN3, T_BN4), as in T_RAW and T_LEN.
--   T_BN1  conventional path: R field of 1259 bytes -> length field `eb 04` across 0x3fff/0x4000
--   T_BN2  direct path:       R field of 1261 bytes -> length field `ed 04` across 0x3fff/0x4000
--   T_BN3  conventional path: NULL ID, A of 256 bytes -> `fe ff` across 0x3fff/0x4000
--   T_BN4  direct path:       NULL ID, A of 258 bytes -> `fe ff` across 0x3fff/0x4000
-- The other path of each table is a control. Also writes /fixtures/truth_bn1.txt ... truth_bn4.txt.

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
  FOR t IN (SELECT table_name FROM user_tables
             WHERE table_name IN ('T_BN1', 'T_BN2', 'T_BN3', 'T_BN4')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" PURGE';
  END LOOP;
END;
/

CREATE TABLE t_bn1 (id NUMBER(3), r RAW(2000));
INSERT INTO t_bn1 SELECT LEVEL, HEXTORAW(RPAD('A' || LEVEL, 2 * 1259, 'A' || LEVEL))
  FROM dual CONNECT BY LEVEL <= 8;

CREATE TABLE t_bn2 (id NUMBER(3), r RAW(2000));
INSERT INTO t_bn2 SELECT LEVEL, HEXTORAW(RPAD('B' || LEVEL, 2 * 1261, 'B' || LEVEL))
  FROM dual CONNECT BY LEVEL <= 8;

CREATE TABLE t_bn3 (id NUMBER(4), a VARCHAR2(400));
INSERT INTO t_bn3 SELECT NULL, RPAD('N' || LPAD(LEVEL, 3, '0') || '_', 256, 'x')
  FROM dual CONNECT BY LEVEL <= 32;

CREATE TABLE t_bn4 (id NUMBER(4), a VARCHAR2(400));
INSERT INTO t_bn4 SELECT NULL, RPAD('N' || LPAD(LEVEL, 3, '0') || '_', 258, 'x')
  FROM dual CONNECT BY LEVEL <= 32;
COMMIT;

SPOOL /fixtures/truth_bn1.txt
SELECT id || '|len=' || UTL_RAW.LENGTH(r) || '|' || RAWTOHEX(UTL_RAW.SUBSTR(r, 1, 4)) FROM t_bn1 ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_bn2.txt
SELECT id || '|len=' || UTL_RAW.LENGTH(r) || '|' || RAWTOHEX(UTL_RAW.SUBSTR(r, 1, 4)) FROM t_bn2 ORDER BY id;
SPOOL OFF

SPOOL /fixtures/truth_bn3.txt
SELECT NVL(TO_CHAR(id), '<NULL>') || '|len=' || LENGTHB(a) || '|' || SUBSTR(a, 1, 5) FROM t_bn3 ORDER BY a;
SPOOL OFF

SPOOL /fixtures/truth_bn4.txt
SELECT NVL(TO_CHAR(id), '<NULL>') || '|len=' || LENGTHB(a) || '|' || SUBSTR(a, 1, 5) FROM t_bn4 ORDER BY a;
SPOOL OFF

SET TERMOUT ON
PROMPT Boundary tables created.
EXIT
