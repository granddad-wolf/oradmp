# Scope

## Phase 1 (now)

Goal: read table data from Oracle dump files and deliver it as Arrow record batches, verified
against ground truth, for the dumps our company produces.

| Aspect | In scope | Notes |
|---|---|---|
| Oracle versions | 11g, 18c | fixtures: 18c generated, 11g to do |
| Formats | legacy EXP and Data Pump (expdp) | EXP first, because it is simpler |
| Export mode | tables and schema | conventional and direct-path `exp`; default Data Pump |
| Character sets | US7ASCII (1), CL8MSWIN1251 (171), WE8MSWIN1252 (178), AL32UTF8 (873); national AL16UTF16 (2000) | the sets of the fixtures (architecture 6.2); any other set: W-CHARSET-UNSUPPORTED |
| Types, first wave | NUMBER, VARCHAR2, CHAR, DATE, TIMESTAMP(n), RAW, NVARCHAR2 | `t_num`, `t_date`, `t_ts`, `t_raw`, `t_text` |
| Types, second wave | TIMESTAMP WITH TIME ZONE, WITH LOCAL TIME ZONE, CLOB/BLOB, INTERVAL | after first wave works |
| Data Pump only | BINARY_FLOAT, BINARY_DOUBLE | legacy `exp` refuses these tables (EXP-00104) |
| Time zones | TIMESTAMP WITH TIME ZONE as the UTC instant (the zone is not delivered); WITH LOCAL TIME ZONE "as stored", with a warning | architecture 5.1, ADR-7 |

## Out of scope for now

Newer Oracle versions (19c, 21c, 23ai), compression, encryption, multi-file and parallel dumps,
XMLTYPE, object types, partition-level extraction, writing dump files.

## Open questions (answers from the DBA)

1. Which tool produces the dumps on 11g and on 18c: `exp`, `expdp`, or both?
2. Database character sets and the `NLS_LANG` used by `exp`.
3. Options in use: `DIRECT=Y`, `COMPRESSION`, `ENCRYPTION`, `PARALLEL`, multiple dump files,
   `ACCESS_METHOD`.
4. Column types present in the real tables (especially LOBs, LONG, XMLTYPE, INTERVAL).
5. Typical and maximum dump size.

Update this file when the answers arrive. Scope decisions are made by the human, not by agents.
