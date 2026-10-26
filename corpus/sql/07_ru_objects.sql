-- 07_ru_objects.sql: run as CORPUS_RU in a CL8MSWIN1251 database. The file is pure ASCII: every
-- Cyrillic character is written as a UNISTR escape and converted to the database set on insert.
SET DEFINE OFF
SET SERVEROUTPUT ON
WHENEVER SQLERROR EXIT FAILURE

CREATE TABLE r_text (
  id NUMBER(5) NOT NULL,
  v  VARCHAR2(200),
  c  CHAR(12),
  vc VARCHAR2(20 CHAR),
  n  NVARCHAR2(40),
  lv VARCHAR2(4000),
  d  DATE
);
CREATE INDEX r_text_ix ON r_text (id);
CREATE TABLE r_clob (id NUMBER(3), cl CLOB);

DECLARE
  lv VARCHAR2(4000);
  FUNCTION u(p VARCHAR2) RETURN VARCHAR2 IS BEGIN RETURN TO_CHAR(UNISTR(p)); END;
  FUNCTION run(a PLS_INTEGER, b PLS_INTEGER) RETURN VARCHAR2 IS
    s VARCHAR2(4000);
  BEGIN
    FOR i IN a..b LOOP
      s := s || TO_CHAR(UNISTR('\' || LPAD(TO_CHAR(i, 'FMXXXX'), 4, '0')));
    END LOOP;
    RETURN s;
  END;
  -- local functions cannot be used inside INSERT ... VALUES (PLS-00231): pass their results as arguments
  PROCEDURE ins(p_id NUMBER, p_v VARCHAR2, p_c VARCHAR2, p_vc VARCHAR2, p_n NVARCHAR2, p_lv VARCHAR2, p_d DATE) IS
  BEGIN
    INSERT INTO r_text VALUES (p_id, p_v, p_c, p_vc, p_n, p_lv, p_d);
  END;
  PROCEDURE insc(p_id NUMBER, p_cl VARCHAR2) IS
  BEGIN
    INSERT INTO r_clob VALUES (p_id, TO_CLOB(p_cl));
  END;
BEGIN
  -- row 1: Latin only
  INSERT INTO r_text VALUES (1, 'Latin only', 'abc', 'plain', 'plain', 'plain', DATE '2026-01-01');
  -- row 2: a short Russian phrase
  ins(2, u('\041F\0440\0438\0432\0435\0442, \043C\0438\0440'), u('\041F\0440\0438\0432\0435\0442'),
       u('\041F\0440\0438\0432\0435\0442'), UNISTR('\041F\0440\0438\0432\0435\0442'), u('\041F\0440\0438\0432\0435\0442'), DATE '2026-01-02');
  -- row 3: the whole alphabet, upper and lower case
  ins(3, run(1040, 1071) || run(1072, 1103), run(1040, 1051), run(1072, 1091),
       UNISTR('\0410\0411\0412\0413'), run(1040, 1071), DATE '2026-01-03');
  -- row 4: YO, yo, numero sign, guillemets
  ins(4, u('\0401\0451\2116\00AB\00BB'), u('\0401\0451\2116\00AB\00BB'), u('\0401\0451\2116\00AB\00BB'),
       UNISTR('\0401\0451\2116\00AB\00BB'), u('\0401\0451\2116\00AB\00BB'), DATE '2026-01-04');
  -- row 5: typographic punctuation, euro, trade mark, no-break space
  ins(5, u('\201C\201D\201E\2014\2013\2026\2022\20AC\2122\00A0'), u('\2014\2013\20AC'), u('\201C\201D'),
       UNISTR('\201C\201D\201E\2014\2013\2026\2022\20AC\2122\00A0'), u('\2014\2013'), DATE '2026-01-05');
  -- row 6: characters that are NOT in CL8MSWIN1251 (e-acute, a Chinese character): what does exp write?
  ins(6, u('\00E9\4E2D x'), u('\00E9\4E2D'), u('\00E9\4E2D'),
       UNISTR('\00E9\4E2D x'), u('\00E9\4E2D x'), DATE '2026-01-06');
  -- row 7: all NULL
  INSERT INTO r_text VALUES (7, NULL, NULL, NULL, NULL, NULL, NULL);
  -- a 3000-byte value, rows 8 (long) and 9 (200 bytes exactly in V, 12 in C, 20 in VC)
  FOR i IN 1..3000 LOOP
    lv := lv || TO_CHAR(UNISTR('\' || LPAD(TO_CHAR(1040 + MOD(i * 7, 64), 'FMXXXX'), 4, '0')));
  END LOOP;
  INSERT INTO r_text VALUES (8, SUBSTR(lv, 1, 100), SUBSTR(lv, 1, 12), SUBSTR(lv, 1, 20), TO_NCHAR(SUBSTR(lv, 1, 40)), lv, DATE '2026-01-08');
  INSERT INTO r_text VALUES (9, SUBSTR(lv, 11, 200), SUBSTR(lv, 20, 12), SUBSTR(lv, 30, 20), TO_NCHAR(SUBSTR(lv, 30, 40)), SUBSTR(lv, 1, 2000), DATE '2026-01-09');
  -- rows 10..309: deterministic filler, about 30 KB of rows, so record boundaries (2048) fall inside rows
  FOR i IN 10..309 LOOP
    INSERT INTO r_text VALUES (i,
      CASE WHEN MOD(i, 7) = 0 THEN NULL ELSE SUBSTR(lv, i, MOD(i, 150) + 1) END,
      SUBSTR(lv, i, 12),
      SUBSTR(lv, i, MOD(i, 20) + 1),
      CASE WHEN MOD(i, 11) = 0 THEN NULL ELSE TO_NCHAR(SUBSTR(lv, i, MOD(i, 40) + 1)) END,
      CASE WHEN MOD(i, 13) = 0 THEN SUBSTR(lv, 1, 600 + i) ELSE NULL END,
      DATE '2026-01-01' + i);
  END LOOP;

  -- CLOB with Cyrillic (a single-byte database stores it in the database set)
  insc(1, u('\041F\0440\0438\0432\0435\0442'));
  INSERT INTO r_clob VALUES (2, TO_CLOB(lv) || TO_CLOB(lv));
  INSERT INTO r_clob VALUES (3, NULL);
  COMMIT;

  -- Cyrillic identifiers, comments, constraint text and view text
  EXECUTE IMMEDIATE 'CREATE TABLE "' || u('\0422\0410\0411\005F\0420\0423\0421') || '" ("' || u('\041A\041E\041B\005F\0031') ||
                    '" VARCHAR2(30), "' || u('\041A\041E\041B\005F\0032') || '" NUMBER(5))';
  EXECUTE IMMEDIATE 'CREATE INDEX "' || u('\0418\041D\0414\005F\0420\0423\0421') || '" ON "' || u('\0422\0410\0411\005F\0420\0423\0421') ||
                    '" ("' || u('\041A\041E\041B\005F\0031') || '")';
  EXECUTE IMMEDIATE 'INSERT INTO "' || u('\0422\0410\0411\005F\0420\0423\0421') || '" VALUES (''' || u('\0430\0431\0432') || ''', 1)';
  EXECUTE IMMEDIATE 'INSERT INTO "' || u('\0422\0410\0411\005F\0420\0423\0421') || '" VALUES (NULL, 2)';
  EXECUTE IMMEDIATE 'COMMENT ON TABLE "' || u('\0422\0410\0411\005F\0420\0423\0421') || '" IS ''' ||
                    u('\0422\0430\0431\043B\0438\0446\0430 \2116 1 \00AB\0442\0435\0441\0442\00BB') || '''';
  EXECUTE IMMEDIATE 'COMMENT ON COLUMN "' || u('\0422\0410\0411\005F\0420\0423\0421') || '"."' || u('\041A\041E\041B\005F\0031') ||
                    '" IS ''' || u('\041A\043E\043B\043E\043D\043A\0430 \0451') || '''';
  EXECUTE IMMEDIATE 'COMMENT ON TABLE r_text IS ''' || u('\0422\0435\043A\0441\0442\044B') || '''';
  EXECUTE IMMEDIATE 'COMMENT ON COLUMN r_text.v IS ''' || u('\0417\043D\0430\0447\0435\043D\0438\0435 (\00AB\0442\0435\0441\0442\00BB)') || '''';
  EXECUTE IMMEDIATE 'ALTER TABLE r_text ADD CONSTRAINT r_text_ck CHECK (v IS NULL OR v <> ''' || u('\044F\044F\044F') || ''')';
  EXECUTE IMMEDIATE 'CREATE VIEW r_view AS SELECT id, v FROM r_text WHERE v <> ''' || u('\044F\044F\044F') || '''';
END;
/
EXIT
