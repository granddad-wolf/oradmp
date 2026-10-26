-- 08_ru_stats_objects.sql (v1): run as CORPUS_RU in the CL8MSWIN1251 database of make_ru.sh.
-- Creates table R_STAT (2000 rows, Cyrillic text column, three indexes) and gathers optimizer statistics
-- WITH histograms on it, and statistics without histograms on R_TEXT. The file is pure ASCII:
-- Cyrillic characters are written as UNISTR escapes and converted to the database set on insert.
SET DEFINE OFF
SET SERVEROUTPUT ON
WHENEVER SQLERROR EXIT FAILURE

BEGIN
  EXECUTE IMMEDIATE 'DROP TABLE r_stat PURGE';
EXCEPTION WHEN OTHERS THEN
  IF SQLCODE != -942 THEN RAISE; END IF;
END;
/
CREATE TABLE r_stat (
  id   NUMBER(6)     NOT NULL,
  grp  NUMBER(3),
  txt  VARCHAR2(40),
  code CHAR(6),
  dt   DATE,
  amt  NUMBER(10,2),
  note VARCHAR2(100)
);

DECLARE
  FUNCTION word(i PLS_INTEGER) RETURN VARCHAR2 IS
    s     VARCHAR2(100);
    start_cp PLS_INTEGER := 1040 + MOD(i * 13, 50);   -- U+0410 ... U+0441 (all in CL8MSWIN1251)
    len   PLS_INTEGER := 3 + MOD(i, 6);
  BEGIN
    FOR k IN 0..len - 1 LOOP
      s := s || TO_CHAR(UNISTR('\' || LPAD(TO_CHAR(start_cp + k, 'FMXXXX'), 4, '0')));
    END LOOP;
    RETURN s;
  END;
  PROCEDURE ins(p_id NUMBER, p_grp NUMBER, p_txt VARCHAR2, p_code VARCHAR2, p_dt DATE, p_amt NUMBER, p_note VARCHAR2) IS
  BEGIN
    INSERT INTO r_stat VALUES (p_id, p_grp, p_txt, p_code, p_dt, p_amt, p_note);
  END;
BEGIN
  FOR i IN 1..2000 LOOP
    ins(i,
        CASE WHEN MOD(i, 2) = 0 THEN 1 WHEN MOD(i, 5) = 0 THEN 2 ELSE MOD(i, 7) + 3 END,   -- skewed: 9 values
        CASE WHEN MOD(i, 17) = 0 THEN NULL ELSE word(i) END,                                -- 150 distinct words
        'K' || LPAD(TO_CHAR(MOD(i, 50)), 3, '0'),
        DATE '2026-01-01' + MOD(i * 3, 200),
        CASE WHEN MOD(i, 11) = 0 THEN NULL ELSE ROUND(MOD(i * 37, 1000) / 7, 2) END,
        CASE WHEN MOD(i, 7) = 0 THEN NULL ELSE 'note ' || TO_CHAR(i) END);
  END LOOP;
  COMMIT;
END;
/
CREATE UNIQUE INDEX r_stat_pk ON r_stat (id);
ALTER TABLE r_stat ADD CONSTRAINT r_stat_pk PRIMARY KEY (id) USING INDEX r_stat_pk;
CREATE INDEX r_stat_txt_ix ON r_stat (txt);
CREATE INDEX r_stat_gd_ix ON r_stat (grp, dt);

-- r_stat: histograms of several kinds (grp: frequency, 9 values; txt: height-balanced, 150 values in 20
-- buckets; dt and amt: height-balanced; id, code, note: none)
BEGIN
  DBMS_STATS.GATHER_TABLE_STATS(
    ownname => USER, tabname => 'R_STAT', estimate_percent => 100,
    method_opt => 'FOR COLUMNS SIZE 254 grp FOR COLUMNS SIZE 20 txt FOR COLUMNS SIZE 10 dt '
               || 'FOR COLUMNS SIZE 5 amt FOR COLUMNS SIZE 1 id code note',
    cascade => TRUE);
  -- r_text: statistics without histograms (text in VARCHAR2, CHAR, NVARCHAR2 and a 4000-byte column)
  DBMS_STATS.GATHER_TABLE_STATS(
    ownname => USER, tabname => 'R_TEXT', estimate_percent => 100,
    method_opt => 'FOR ALL COLUMNS SIZE 1', cascade => TRUE);
END;
/
PROMPT r_stat rows:
SELECT COUNT(*) FROM r_stat;
PROMPT histograms gathered:
SELECT table_name || '.' || column_name || ' ' || histogram || ' ' || num_buckets
  FROM user_tab_col_statistics WHERE histogram != 'NONE' ORDER BY 1;
EXIT
