-- 05_own_objects.sql
-- R-012 (docs/formats/requests.md). Run as CORPUS_OWN (make_own.sh does this).
-- Same SQL on 18c and 11g. No table has more than 3 rows.
-- One addition to the request as the Analyst wrote it: the procedure O_PROC_LONG, about 9600 bytes
-- of source, so that a text item longer than 8192 bytes exists (tests rule G of exp.md 4.2).

SET ECHO OFF
SET DEFINE OFF
SET SQLBLANKLINES ON
-- keep going after an error so that the log shows every failing statement
WHENEVER SQLERROR CONTINUE

CREATE TABLE o_parent (id NUMBER(6) PRIMARY KEY, name VARCHAR2(30) NOT NULL);
INSERT INTO o_parent VALUES (1, 'p1');
INSERT INTO o_parent VALUES (2, 'p2');

CREATE TABLE o_child (
  id        NUMBER(6) CONSTRAINT o_child_pk PRIMARY KEY,
  parent_id NUMBER(6) CONSTRAINT o_child_fk REFERENCES o_parent (id),
  status    VARCHAR2(10) DEFAULT 'NEW' NOT NULL
            CONSTRAINT o_child_ck CHECK (status IN ('NEW', 'DONE')),
  note      VARCHAR2(100) DEFAULT 'n/a',
  CONSTRAINT o_child_ck2 CHECK (
    note IS NULL OR note NOT IN (
      'forbidden value number 01 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
      'forbidden value number 02 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
      'forbidden value number 03 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
      'forbidden value number 04 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
      'forbidden value number 05 xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'))
);
INSERT INTO o_child VALUES (1, 1, 'NEW', 'first');
INSERT INTO o_child VALUES (2, 2, 'DONE', NULL);
INSERT INTO o_child (id) VALUES (3);
CREATE INDEX o_child_ix ON o_child (status, note);
COMMENT ON TABLE o_child IS 'short ascii comment';

BEGIN
  -- 300 bytes: the length needs the second byte of the u16 (0x012c)
  EXECUTE IMMEDIATE 'COMMENT ON COLUMN o_child.note IS ''' || RPAD('L300_', 300, 'x') || '''';
  -- an LF and a quote inside a comment
  EXECUTE IMMEDIATE 'COMMENT ON COLUMN o_child.status IS ''line1' || CHR(10) || 'line2 it''''s''';
  -- non-ASCII: Caf + e acute, euro sign, Cyrillic de (2-, 3-, 2-byte characters in AL32UTF8)
  EXECUTE IMMEDIATE 'COMMENT ON COLUMN o_child.parent_id IS ''Caf' || TO_CHAR(UNISTR('\00E9')) || ' '
                    || TO_CHAR(UNISTR('\20AC')) || ' ' || TO_CHAR(UNISTR('\0434')) || '''';
END;
/

-- empty table with its own primary key, index and comment
CREATE TABLE o_empty (id NUMBER(6) CONSTRAINT o_empty_pk PRIMARY KEY, name VARCHAR2(20));
CREATE INDEX o_empty_ix ON o_empty (name);
COMMENT ON TABLE o_empty IS 'no rows';

CREATE SEQUENCE o_seq START WITH 100 INCREMENT BY 5;
CREATE VIEW o_view AS SELECT id, status FROM o_child WHERE status = 'NEW';
CREATE SYNONYM o_syn FOR o_parent;

CREATE OR REPLACE PROCEDURE o_proc (p_id IN NUMBER) IS
BEGIN
  -- a comment line
  UPDATE o_child SET status = 'DONE' WHERE id = p_id;
END;
/

CREATE OR REPLACE TRIGGER o_trg BEFORE INSERT ON o_child FOR EACH ROW
BEGIN
  IF :NEW.status IS NULL THEN
    :NEW.status := 'NEW';
  END IF;
END;
/

-- Addition: a procedure whose source is about 9600 bytes (120 comment lines of 80 bytes each,
-- plus about 60 bytes), more than 8192, so a length of 0x2000 or more is needed for it.
DECLARE
  v VARCHAR2(32767);
BEGIN
  v := 'CREATE OR REPLACE PROCEDURE o_proc_long IS' || CHR(10) || 'BEGIN' || CHR(10);
  FOR i IN 1 .. 120 LOOP
    v := v || '  -- ' || LPAD(i, 3, '0') || ' ' || RPAD('padding line ', 70, 'p') || CHR(10);
  END LOOP;
  v := v || '  NULL;' || CHR(10) || 'END;';
  EXECUTE IMMEDIATE v;
END;
/

GRANT SELECT ON o_parent TO corpus;
COMMIT;

PROMPT Objects created.
EXIT
