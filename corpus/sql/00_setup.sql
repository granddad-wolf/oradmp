-- 00_setup.sql
-- Run as SYSTEM (db_setup.sh does this for you).
--   Oracle XE 18c: service XEPDB1.   Oracle XE 11g: service XE.
-- Creates the CORPUS user and two directory objects that match the Docker mounts:
--   WORK_DIR      -> /work       (large files, not committed to git)
--   FIXTURES_DIR  -> /fixtures   (small files, committed to git)

SET ECHO OFF
SET DEFINE OFF

WHENEVER SQLERROR EXIT FAILURE

-- Drop the user only if it exists, so a first run produces no error.
DECLARE
  l_cnt NUMBER;
BEGIN
  SELECT COUNT(*) INTO l_cnt FROM all_users WHERE username = 'CORPUS';
  IF l_cnt > 0 THEN
    EXECUTE IMMEDIATE 'DROP USER corpus CASCADE';
  END IF;
END;
/

-- &1 is the password of the CORPUS user (run_sql passes $CORPUS_PW).
SET DEFINE ON
CREATE USER corpus IDENTIFIED BY "&1"
  DEFAULT TABLESPACE users
  TEMPORARY TABLESPACE temp
  QUOTA UNLIMITED ON users;
SET DEFINE OFF

GRANT CREATE SESSION, CREATE TABLE, CREATE SEQUENCE, CREATE VIEW, CREATE PROCEDURE TO corpus;

CREATE OR REPLACE DIRECTORY work_dir AS '/work';
CREATE OR REPLACE DIRECTORY fixtures_dir AS '/fixtures';
GRANT READ, WRITE ON DIRECTORY work_dir TO corpus;
GRANT READ, WRITE ON DIRECTORY fixtures_dir TO corpus;

PROMPT Setup done.
EXIT
