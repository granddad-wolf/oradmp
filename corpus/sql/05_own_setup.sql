-- 05_own_setup.sql
-- R-012. Run as SYSTEM (make_own.sh does this for you).
--   Oracle XE 18c: service XEPDB1.   Oracle XE 11g: service XE.
-- Creates the small user CORPUS_OWN that owns the objects of 05_own_objects.sql.
-- Safe to run again: the user is dropped and recreated.

SET ECHO OFF
SET DEFINE OFF

WHENEVER SQLERROR EXIT FAILURE

DECLARE
  l_cnt NUMBER;
BEGIN
  SELECT COUNT(*) INTO l_cnt FROM all_users WHERE username = 'CORPUS_OWN';
  IF l_cnt > 0 THEN
    EXECUTE IMMEDIATE 'DROP USER corpus_own CASCADE';
  END IF;
END;
/

CREATE USER corpus_own IDENTIFIED BY "Corpus_own_1"
  DEFAULT TABLESPACE users
  TEMPORARY TABLESPACE temp
  QUOTA UNLIMITED ON users;

GRANT CREATE SESSION, CREATE TABLE, CREATE VIEW, CREATE SEQUENCE, CREATE SYNONYM,
      CREATE TRIGGER, CREATE PROCEDURE TO corpus_own;

PROMPT Setup done.
EXIT
