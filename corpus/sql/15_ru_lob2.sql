set feedback off
set define off
set linesize 1000
set pagesize 0
set trimspool on
BEGIN EXECUTE IMMEDIATE 'DROP TABLE r_lob2 PURGE'; EXCEPTION WHEN OTHERS THEN NULL; END;
/
whenever sqlerror exit failure
CREATE TABLE r_lob2 (id NUMBER(3), c CLOB, n NCLOB, b BLOB);
DECLARE
  blk VARCHAR2(66);
  l_c CLOB; l_n NCLOB; l_b BLOB;
BEGIN
  FOR i IN 192 .. 255 LOOP blk := blk || CHR(i); END LOOP;
  blk := blk || CHR(168) || CHR(184);
  INSERT INTO r_lob2 VALUES (1, TO_CLOB(blk), TO_NCLOB(blk), UTL_RAW.CAST_TO_RAW(blk));
  INSERT INTO r_lob2 VALUES (2, EMPTY_CLOB(), EMPTY_CLOB(), EMPTY_BLOB())
    RETURNING c, n, b INTO l_c, l_n, l_b;
  FOR j IN 1 .. 303 LOOP
    DBMS_LOB.WRITEAPPEND(l_c, 66, blk);
    DBMS_LOB.WRITEAPPEND(l_n, 66, TO_NCHAR(blk));
    DBMS_LOB.WRITEAPPEND(l_b, 66, UTL_RAW.CAST_TO_RAW(blk));
  END LOOP;
  DBMS_LOB.WRITEAPPEND(l_c, 2, SUBSTR(blk, 1, 2));
  DBMS_LOB.WRITEAPPEND(l_n, 2, TO_NCHAR(SUBSTR(blk, 1, 2)));
  DBMS_LOB.WRITEAPPEND(l_b, 2, UTL_RAW.CAST_TO_RAW(SUBSTR(blk, 1, 2)));
  INSERT INTO r_lob2 VALUES (3, NULL, NULL, NULL);
  COMMIT;
END;
/
SELECT 'R_LOB2|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(c)), 'NULL') || '|'
       || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(n)), 'NULL') || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL') || '|'
       || NVL(DUMP(DBMS_LOB.SUBSTR(c, 4, 1), 16), 'NULL') || '|' || NVL(DUMP(DBMS_LOB.SUBSTR(n, 4, 1), 16), 'NULL') || '|'
       || NVL(DUMP(DBMS_LOB.SUBSTR(c, 4, DBMS_LOB.GETLENGTH(c) - 3), 16), 'NULL') || '|'
       || NVL(DUMP(DBMS_LOB.SUBSTR(n, 4, DBMS_LOB.GETLENGTH(n) - 3), 16), 'NULL')
  FROM r_lob2 ORDER BY id;
exit
