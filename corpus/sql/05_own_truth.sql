-- 05_own_truth.sql
-- R-012 truth files. Run as CORPUS_OWN, after 05_own_objects.sql (make_own.sh does this).
-- Writes /fixtures/truth_own.txt (rows) and /fixtures/truth_own_ddl.txt (what the text items hold).
-- Same script on 18c and 11g: USER_CONSTRAINTS.SEARCH_CONDITION_VC exists from 12c on only,
-- so that column is read by dynamic SQL; on 11g its value is <NULL>.
-- Names of system-generated constraints (SYS_C...) differ from run to run; that is why make_own.sh
-- always recreates the dumps and the truth files together.

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
SET SERVEROUTPUT ON SIZE UNLIMITED FORMAT WRAPPED
WHENEVER SQLERROR CONTINUE

SPOOL /fixtures/truth_own.txt
SELECT 'O_PARENT|' || id || '|' || name FROM o_parent ORDER BY id;
SELECT 'O_CHILD|' || id || '|' || NVL(TO_CHAR(parent_id), '<NULL>') || '|' || status || '|'
       || NVL(note, '<NULL>') FROM o_child ORDER BY id;
SELECT 'O_EMPTY|count=' || COUNT(*) FROM o_empty;
SPOOL OFF

SPOOL /fixtures/truth_own_ddl.txt
SELECT 'OBJECT|' || object_type || '|' || object_name FROM user_objects ORDER BY 1;

SELECT 'TABCOMMENT|' || table_name || '|' || LENGTHB(comments) || '|' || DUMP(comments, 1016)
  FROM user_tab_comments WHERE comments IS NOT NULL ORDER BY table_name;

SELECT 'COLCOMMENT|' || table_name || '.' || column_name || '|' || LENGTHB(comments) || '|'
       || DUMP(comments, 1016, 1, 40)
  FROM user_col_comments WHERE comments IS NOT NULL ORDER BY table_name, column_name;

DECLARE
  l_has  NUMBER;
  c      SYS_REFCURSOR;
  l_name VARCHAR2(128);
  l_type VARCHAR2(1);
  l_len  NUMBER;
BEGIN
  SELECT COUNT(*) INTO l_has FROM all_tab_columns
   WHERE owner = 'SYS' AND table_name = 'USER_CONSTRAINTS' AND column_name = 'SEARCH_CONDITION_VC';
  IF l_has > 0 THEN
    OPEN c FOR 'SELECT constraint_name, constraint_type, LENGTHB(search_condition_vc)
                  FROM user_constraints ORDER BY constraint_name';
  ELSE
    OPEN c FOR 'SELECT constraint_name, constraint_type, CAST(NULL AS NUMBER)
                  FROM user_constraints ORDER BY constraint_name';
  END IF;
  LOOP
    FETCH c INTO l_name, l_type, l_len;
    EXIT WHEN c%NOTFOUND;
    DBMS_OUTPUT.PUT_LINE('CONSTRAINT|' || l_name || '|' || l_type || '|' || NVL(TO_CHAR(l_len), '<NULL>'));
  END LOOP;
  CLOSE c;
END;
/

SELECT 'SOURCE|' || name || '|' || type || '|' || line || '|' || LENGTHB(text)
  FROM user_source ORDER BY name, type, line;

-- total source size per object (the LF at the end of every line is part of TEXT)
SELECT 'SOURCETOTAL|' || name || '|' || type || '|bytes=' || SUM(LENGTHB(text)) || '|lines=' || COUNT(*)
  FROM user_source GROUP BY name, type ORDER BY name, type;
SPOOL OFF

SET TERMOUT ON
PROMPT Truth files written to /fixtures/truth_own.txt and truth_own_ddl.txt
EXIT
