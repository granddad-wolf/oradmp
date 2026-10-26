-- 01b_optional_tables.sql
-- Optional tables for later research stages. Run as CORPUS after 01_corpus_tables.sql.
-- Each block is independent: an error in one (for example partitioning on 11g XE,
-- which does not have the Partitioning option) does not stop the others.
--
--   T_LOB       CLOB/BLOB: NULL, empty, small, 3000 chars, 100,000 chars/bytes
--   T_INTERVAL  INTERVAL YEAR TO MONTH, INTERVAL DAY TO SECOND
--   T_PART      range-partitioned table (18c XE only)
--   T_BIG       5,000,000 rows for throughput tests (commented out, enable by hand)

SET SQLBLANKLINES ON
SET DEFINE OFF
SET ECHO OFF
WHENEVER SQLERROR CONTINUE

ALTER SESSION SET NLS_DATE_FORMAT = 'YYYY-MM-DD HH24:MI:SS';

-- Drop this script's own tables if they exist, so a first run produces no error.
BEGIN
  FOR t IN (SELECT table_name FROM user_tables
             WHERE table_name IN ('T_LOB', 'T_INTERVAL', 'T_PART', 'T_BIG')) LOOP
    EXECUTE IMMEDIATE 'DROP TABLE "' || t.table_name || '" CASCADE CONSTRAINTS PURGE';
  END LOOP;
END;
/

---------------------------------------------------------------------------
-- T_LOB
---------------------------------------------------------------------------
CREATE TABLE t_lob (id NUMBER(3), c CLOB, b BLOB);
INSERT INTO t_lob VALUES (1, TO_CLOB('hello lob'), HEXTORAW('48656C6C6F20626C6F62'));
INSERT INTO t_lob VALUES (2, NULL, NULL);
INSERT INTO t_lob VALUES (3, EMPTY_CLOB(), EMPTY_BLOB());
INSERT INTO t_lob VALUES (4, TO_CLOB(RPAD('m', 3000, 'm')), HEXTORAW(RPAD('AB', 2000, 'AB')));
DECLARE
  l_c CLOB;
  l_b BLOB;
BEGIN
  INSERT INTO t_lob VALUES (5, EMPTY_CLOB(), EMPTY_BLOB()) RETURNING c, b INTO l_c, l_b;
  FOR i IN 1 .. 100 LOOP
    DBMS_LOB.WRITEAPPEND(l_c, 1000, RPAD('chunk' || LPAD(i, 3, '0'), 1000, '.'));
    DBMS_LOB.WRITEAPPEND(l_b, 1000, HEXTORAW(RPAD('CD', 2000, 'CD')));
  END LOOP;
  COMMIT;
END;
/

---------------------------------------------------------------------------
-- T_INTERVAL
---------------------------------------------------------------------------
CREATE TABLE t_interval (
  id  NUMBER(3),
  iym INTERVAL YEAR(4) TO MONTH,
  ids INTERVAL DAY(4) TO SECOND(6)
);
INSERT INTO t_interval VALUES (1, INTERVAL '2-6' YEAR TO MONTH, INTERVAL '10 12:30:45.123456' DAY TO SECOND);
INSERT INTO t_interval VALUES (2, INTERVAL '-3-2' YEAR TO MONTH, INTERVAL '-1 00:00:00' DAY TO SECOND);
INSERT INTO t_interval VALUES (3, INTERVAL '0-0' YEAR TO MONTH, INTERVAL '0 00:00:00' DAY TO SECOND);
INSERT INTO t_interval VALUES (4, NULL, NULL);
COMMIT;

---------------------------------------------------------------------------
-- T_PART (needs the Partitioning option; works on 18c XE, fails on 11g XE)
---------------------------------------------------------------------------
CREATE TABLE t_part (id NUMBER(6), sale_date DATE, amount NUMBER(10, 2))
PARTITION BY RANGE (sale_date) (
  PARTITION p2019 VALUES LESS THAN (DATE '2020-01-01'),
  PARTITION p2020 VALUES LESS THAN (DATE '2021-01-01'),
  PARTITION pmax  VALUES LESS THAN (MAXVALUE)
);
INSERT INTO t_part
SELECT lvl, DATE '2019-01-01' + MOD(lvl * 13, 1000), MOD(lvl * 31, 100000) / 100
  FROM (SELECT LEVEL AS lvl FROM dual CONNECT BY LEVEL <= 3000);
COMMIT;

---------------------------------------------------------------------------
-- T_BIG (superseded by 15_big.sql, O-17, session 19; kept as history, do not enable).
-- XE 11g limits user data to 11 GB, so keep it modest.
---------------------------------------------------------------------------
-- DROP TABLE t_big PURGE;
-- CREATE TABLE t_big AS
-- SELECT lvl AS id, 'name_' || lvl AS name, MOD(lvl * 7919, 1000000) / 100 AS amount,
--        DATE '2020-01-01' + MOD(lvl, 366) AS created
--   FROM (SELECT LEVEL AS lvl FROM dual CONNECT BY LEVEL <= 1000)
--        CROSS JOIN (SELECT LEVEL AS k FROM dual CONNECT BY LEVEL <= 5000);
-- (Note: this form repeats ids; use it only for volume, not for value checks.)

PROMPT Optional corpus created (check above for errors).
EXIT
