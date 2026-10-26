# oradump research sources (Scout)

Owner: Scout. Every claim has a source URL, a source type, and a status.
Source types: code | experiment | documentation | hearsay.
Status is "unverified" everywhere. Only the Analyst may change it.

Method caveat: pages were fetched through a tool that summarizes page text with a
small model. Search-engine result summaries were also used where a page would not
load. Each entry says which of the two applies where it matters. The Analyst
should re-read the primary page before relying on any detail (offsets, example
bytes, prices).

Fetched 2026-10-02.

---

# Round 1: Projects reviewed

Scope: README / product page / docs statements only. No source code was read.
"not stated" means the page I read did not say. Licenses flagged GPL or unclear
must not be copied from.

| ID | Name | URL | License | Formats claimed | Oracle versions claimed | Maturity (observed) |
|----|------|-----|---------|-----------------|-------------------------|---------------------|
| P-001 | Open DUMP Viewer (ODV) | https://github.com/Open-DUMP-Viewer/Open-DUMP-Viewer (site https://www.odv.dev/) | Proprietary. Free for personal and educational use, paid for commercial. FLAG: proprietary, do not copy. Whether the repo exposes parser source is not stated. | Both: EXP and EXPDP (Data Pump) | EXP: 7 to 11g; EXPDP: 10g to 23ai (README) | GitHub README: 325 commits on main, 5 stars, 1 open issue, 2 pull requests. Windows x64/ARM64 desktop app, VB.NET UI with a native C DLL for parsing, .NET 10 (so recent activity). Japan-based, paid tiers. Price differs between sources: GitHub org page says 9,800 JPY per license per year; odv.dev says 5,390 JPY (professional) and 10,780 JPY (business). The product site says it cannot guarantee correct results for all dump files and data types. The site has no format specification. |
| P-002 | odv-testdump (test dump generator by the ODV author) | https://github.com/Open-DUMP-Viewer/odv-testdump | not stated. FLAG: unclear. | Both: EXPDP (standard, compressed, option variants) and EXP | Generated on Oracle 23ai Free (Docker image gvenzl/oracle-free:23-slim) | 5 commits on main, shell scripts. README says it covers numeric, string, LOB, date/timestamp, XML, JSON and spatial types, empty tables, wide tables (100+ columns), large tables (10,000+ rows). It supplies SQL that fills the data but no separately maintained expected-output files. Possible source of fixture generators; nothing here is truth data. |
| P-003 | OraDump Export Kit (Convert-in / "OraDump to ..." family) | https://convert-in.com/oracle-dump.htm ; also listed at https://www.componentsource.com/product/oradump-export-kit | Commercial, proprietary. License text and price not retrieved. FLAG: proprietary. | Both: EXP and EXPDP (stated in search-result summary of the product page) | "All versions" (search-result summary, no specific list) | Direct fetch failed with a TLS certificate error, and componentsource returned HTTP 403, so everything here comes from search-result summaries. Reported claims: reads dump files directly, no Oracle components needed, outputs MS Access, Excel, MS SQL, MySQL, PostgreSQL, CSV, no size limit, over 5 GB/hour. Release dates and last update: not found. |
| P-004 | OraDump-to-CSV (same vendor) | https://www.convert-in.com/ord2csv.htm | Commercial, proprietary. Not retrieved. FLAG: proprietary. | Search summary says it reads Oracle dump files directly into CSV; EXP/EXPDP split not stated in what I saw | not stated | Page fetch failed (TLS error). Search-result summary only. |
| P-005 | Dump Read API (same vendor) | https://www.convert-in.com/dump-read-api.htm | not stated (fetch failed). FLAG: unclear, likely commercial. | not stated | not stated | Fetch failed (TLS error). Only the URL and title appeared in search results. Nothing else known. |
| P-006 | oracle-dmp-converter (rjschof) | https://github.com/rjschof/oracle-dmp-converter | MIT | Input: Data Pump and legacy exp dump files. Output: Parquet, Avro, CSV. It does NOT parse dumps itself. It loads them into a temporary Oracle Free database in Docker, then exports. | Uses gvenzl/oracle-free:23-faststart by default | Python 3.12, 71 commits, 0 stars, 0 releases. Last-commit date not shown. Useful only as a way to produce truth data, not as a parser. |
| P-007 | "Scripts to help with converting an Oracle .dmp to CSV" gist (jabley) | https://gist.github.com/jabley/3713603 (also https://gist.github.com/3713603) | No license stated. FLAG: unclear. | Whatever imp accepts (classic EXP). Calls Oracle imp, so not a parser. | not stated | Created 2012-09-13. Single gist. Needs a remote Oracle server. |
| P-008 | DUL (Data UnLoader, Oracle-internal, originally by Bernard van Duijnen) | Description via https://www.cnblogs.com/tianlesoftware/p/3609564.html and https://blog.51cto.com/cndba/5776773 | Proprietary, Oracle-internal. Source not public (no repository found). FLAG: proprietary. | Reads datafiles, not dump files. It WRITES output in export-file format (CREATE TABLE and INSERT content), per search summary of the 51cto post. | The 2014 cnblogs post says Oracle 6, 7, 8; a search summary of another post says later versions are supported | Used under Oracle Support supervision per the posts. Not a dump-file reader, so listed only because it emits EXP-format files. |
| P-009 | ODU (Oracle Data Unloader, oracleodu.com) | https://www.cnblogs.com/tianlesoftware/p/3609564.html | Commercial. The post says it used to be free. | Reads datafiles, not dump files | not stated | Vendor named in the post as "Old Bear" and dbsnake. Another post (https://www.modb.pro/db/21306, search title only) suggests ODU did not handle damaged dmp files while DUL did. I did not read that page. |
| P-010 | AUL (anysql.net) | https://www.cnblogs.com/tianlesoftware/p/3609564.html | Freemium. Free tier limited to 2 datafiles of up to 256 MB each, per the post. | Reads datafiles | not stated | Written in C per the post. Not a dump-file reader. |
| P-011 | PRM-DUL (Oracle) | https://www.onworks.net/software/app-prm-dul-oracle | not stated. FLAG: unclear. | not stated | not stated | Only a search-result title was seen. Page not fetched. |
| P-012 | TestDisk file_dmp.c (file signature recognizer, CGSecurity) | https://git.cgsecurity.org/cgit/testdisk/tree/src/file_dmp.c | Not retrieved: the page returned an Anubis "Access Denied" screen. FLAG: license unclear for this session. Do not copy. | Legacy EXP header signature only (see C-021) | not stated | Only a search-result summary was available. It is a carving signature, not a parser. |
| P-013 | "How to read Datapump Dumpfile Headers" (blog post with dumpinfo.pl) | https://databaseinternalmechanism.com/2016/09/13/how-to-read-datapump-dumpfile-header/ | License of the script not stated. FLAG: unclear. | Data Pump header block (and tells apart Data Pump vs classic export) | Example output shows 11.2.0.4 | Blog post dated 2016-09-13. The author reads the header block with a Perl script and lists the fields it prints (see C-022). The post also describes DBMS_DATAPUMP.GET_DUMPFILE_INFO and impdp SQLFILE, which need Oracle. Author inspected real files, so this counts as an experiment source. |
| P-014 | SQLines "show Oracle dump content" | https://www.sqlines.com/oracle/how-to/show_oracle_dump_content | not stated | Classic EXP only, via imp SHOW=y | not stated | Not a parser. It uses Oracle's imp. Listed for completeness. |
| P-015 | Oracle Community / TechRepublic forum threads | https://forums.oracle.com/ords/apexds/post/free-tool-for-analyzing-dmp-file-7702 ; https://forums.oracle.com/ords/apexds/post/datapump-files-how-to-analyse-0900 ; https://forums.oracle.com/ords/apexds/post/read-dmp-file-4447 ; https://forums.oracle.com/ords/apexds/post/how-to-read-export-dump-file-5736 ; https://www.techrepublic.com/forums/discussions/how-to-extract-data-from-oracle-dmp-file-without-oracle/ | n/a (forum posts) | n/a | n/a | NOT READ. Two fetches timed out and the others were only seen as search results. Search summaries said the common answer is that the dump is a black box and only imp reads it, which is hearsay. |

## Round 1 not found

Open-source parser that reads EXP or Data Pump files directly: not found.
Queries tried:
- "parse Oracle exp dump file without Oracle database open source EXPDAT.DMP parser"
- "github oracle export dump reader imp exp dmp file parser python OR java OR go OR rust"
- "github oracle export file dmp binary format parser library EXPORT:V"
- "github repository parse Oracle original export .dmp file extract table rows INSERT statements export dump parser README"
- "Data Pump dump file reader library open source parse .dmp expdp without database Java Python impdp alternative"
- "convert Oracle dmp file to CSV without Oracle tool github"
The only direct reader found was proprietary (P-001, P-003/4). Everything else on GitHub shells out to Oracle imp/impdp.

Papers or posts with hex dumps of EXP or Data Pump body data (not just header fields): not found.
Queries tried:
- "Oracle Data Pump expdp dump file format reverse engineering header hex"
- "Data Pump dump file internal structure KUPD OR KUPC OR master table strings in dmp file binary analysis blog"
- "Oracle export dump file format exp binary column data internal NUMBER format length prefix two bytes records insert rows EXPORT:V analysis"
- "Oracle exp dump file DATE column stored 7 bytes in dump NUMBER stored internal format in export file hex view"

Public format specification from Oracle for either dump format: not found. Oracle docs only call the formats proprietary and binary, readable only by their own imp/impdp.

Not searched this round: Kaitai Struct or other format-spec repositories, DataNumen / Stellar products, non-English forums beyond the DUL posts.

---

# Round 2: Type encodings

Scope marker used in each entry:
- DATAFILE/DUMP() = describes the in-database (block) or OCI representation. Says nothing about EXP or Data Pump files.
- DUMPFILE = speaks specifically about EXP or Data Pump files.
No entry below says that dump files reuse the datafile bytes. That question is open (see "Dump-file-specific encoding: not found").

All claims: status unverified.

## NUMBER

C-001
- Claim: Storage for NUMBER is 1 to 22 bytes per value, precision up to 38 digits, scale from -84 to 127.
- Source: https://docs.oracle.com/cd/B13789_01/server.101/b10759/sql_elements001.htm
- Type: documentation. License: Oracle documentation, copyright Oracle, not open.
- Scope: DATAFILE/DUMP(). Whether the 22 includes a length byte is not stated (see conflict K-3).
- Status: unverified

C-002
- Claim: The first byte of a NUMBER holds a sign bit (set means positive) in its top bit, and a base-100 exponent in the low 7 bits with an offset of 65. Then come 1 to 20 mantissa bytes, each one base-100 digit. For positive numbers each digit has 1 added, so digits run from 1 to 100. For negative numbers each digit is subtracted from 101. A negative number with fewer than 20 mantissa bytes gets one extra trailing byte of value 102. Zero is the single byte 128. Examples given: 5 is 193,6; -5 is 62,96,102; 100000 is 195,11.
- Source: https://docs.oracle.com/cd/A91202_01/901_doc/appdev.901/a89857/oci03typ.htm (Oracle9i OCI guide, VARNUM section)
- Type: documentation. License: Oracle documentation, not open.
- Scope: DATAFILE/DUMP() and OCI VARNUM. The OCI form includes a leading length byte, so check what the row or dump stores.
- Status: unverified

C-003
- Claim: The DUMP() output for 123 is Typ=2 Len=3: 194,2,24. Storage size depends on how many significant digits the value needs, not on magnitude. A power of ten such as 10^33 takes 2 bytes, while adding or subtracting 1 from a large round number grows it to 19 or 20 bytes.
- Source: https://tanelpoder.com/2010/09/02/which-number-takes-more-space-in-an-oracle-row
- Type: experiment (author ran DUMP() on live data). License: blog, no license stated.
- Scope: DATAFILE/DUMP(). The post does not discuss NULLs.
- Status: unverified

C-004
- Claim: A Chinese-language article describes the same scheme, with these details: the first byte is built from the sign and a halved decimal exponent; positive mantissa bytes add 1 and a value of 100 stands for digit pair 99; negative mantissa bytes add 101 to the negated pair; the 102 terminator is appended to negatives shorter than 20 mantissa bytes to keep sorting correct; zero is a single byte 0x80; 0.5 has exponent -1 with mantissa 50 before offset.
- Source: https://developer.aliyun.com/article/294548
- Type: hearsay (the author's origin of knowledge is unclear). License: not stated.
- Scope: DATAFILE/DUMP().
- Note: the wording of the first-byte rule differs from C-002 (see K-2).
- Status: unverified

C-005
- Claim: The godror Go library documents a NUMBER as at most 22 bytes: one exponent byte with the sign in the top bit, then base-100 mantissa digits (1 to 100). Positive digits get +1, negative digits are 101 minus the digit, and a 102 byte follows negatives unless there are 20 mantissa bytes. Maximum precision is 38 digits. Callers should size output buffers at 21 bytes. It also says only 19 mantissa bytes are guaranteed accurate.
- Source: https://pkg.go.dev/github.com/godror/godror@v0.29.0/num (package documentation page, not source code)
- Type: documentation (of a code package). License: the page lists Apache-2.0, BSD-3-Clause and MIT plus one more that I could not identify. FLAG: unclear.
- Scope: DATAFILE/DUMP() and OCI.
- Status: unverified

C-006
- Claim: A blog says NUMBER has a maximum internal length of 21 bytes (1 exponent byte plus up to 20 digit bytes) with datatype code 2, and that the length byte is included in the OCI representation for backward compatibility. Max 22 when the length byte counts.
- Source: https://blog.51cto.com/luoping/14232424 (seen only as a search-result summary; page not fetched)
- Type: hearsay. License: not stated.
- Scope: DATAFILE/DUMP() and OCI.
- Status: unverified

## DATE

C-007
- Claim: DATE is 7 bytes: century, year, month, day, hour, minute, second. Century and year (year within the century) are excess-100. Hour, minute and second are excess-1. Midnight with no time is 1,1,1 in the last three bytes. Dates before the common era have a century byte under 100. Example 30-NOV-1992 15:17 stored as 119,192,11,30,16,18,1 (the page's example, as I read it).
- Source: https://docs.oracle.com/cd/A91202_01/901_doc/appdev.901/a89857/oci03typ.htm
- Type: documentation. License: Oracle documentation, not open.
- Scope: DATAFILE/DUMP() and OCI.
- Status: unverified

C-008
- Claim: '2012-09-17 13:31:01' is stored as 120,112,9,17,14,32,2. Layout matches C-007.
- Source: https://sqlines.com/oracle/datatypes/date
- Type: hearsay (vendor tutorial page, no stated experiment). License: not stated.
- Scope: DATAFILE/DUMP().
- Status: unverified

## TIMESTAMP (plain)

C-009
- Claim: TIMESTAMP (datatype code 180) occupies 7 to 11 bytes depending on fractional-second precision. TIMESTAMP WITH TIME ZONE (code 181) is fixed at 13 bytes and carries year, month, day, hour, minute, second, time zone hour and time zone minute. TIMESTAMP WITH LOCAL TIME ZONE is code 231.
- Source: https://docs.oracle.com/cd/B13789_01/server.101/b10759/sql_elements001.htm and https://www.dbs.ifi.lmu.de/oradoc10g/DOC/server.101/b10759/sql_elements001.htm
- Type: documentation. License: Oracle documentation, not open.
- Scope: DATAFILE/DUMP(). Seen in the search-result summary. My own fetch of the first URL returned a summary that did not repeat the 7-to-11 and 13 byte figures, so re-read the page. The 13-byte figure is also repeated in several tutorial snippets.
- Status: unverified

C-010
- Claim: A DUMP() of a TIMESTAMP shows Typ=180 Len=11: 120,116,3,26,16,10,28,38,182,114,56. The first seven bytes look like a DATE (excess-100 century and year; the example reads as 2016-03-26 with time fields excess-1). The last 4 bytes are the fractional second, each a power of 256. The article does not state byte order explicitly in what I read. Fractional-part byte order: not stated.
- Source: https://www.pythian.com/blog/more-fun-with-oracle-timestamp-math
- Type: experiment (author ran DUMP()). License: blog, not stated.
- Scope: DATAFILE/DUMP().
- Status: unverified

## TIMESTAMP WITH TIME ZONE

C-011
- Claim: The only byte-level layout found is for the OCI EXTERNAL type 188 (20 bytes), not the in-database type 181. Byte 1 and 2 give the year (byte 1 plus byte 2 times 256), then month, day, hour (stored in UTC), minute, second, a padding byte, four bytes of fraction, then two bytes of time zone offset. Example: Typ=188 Len=20: 224,7,3,26,19,34,55,0,152,108,205,20,252,0,5,0,0,0,0,0.
- Source: https://www.pythian.com/blog/more-fun-with-oracle-timestamp-math
- Type: experiment. License: blog, not stated.
- Scope: client-side OCI external representation only. NOT the on-disk 13-byte type 181 and not any dump file. See K-1.
- Status: unverified

C-012
- Claim: The on-disk 13-byte layout of TIMESTAMP WITH TIME ZONE (byte positions, whether the stored time is UTC, how offset or region id is packed): not found.
- Queries tried: "Oracle DATE internal 7 bytes century+100 year+100 DUMP TIMESTAMP WITH TIME ZONE 13 bytes format"; "Oracle DUMP function datatype 180 181 231 TIMESTAMP WITH TIME ZONE internal bytes explained offset 20 hour minute"; "Typ=181 Len=13 dump timestamp with time zone stored in UTC bytes 12 13 time zone hour offset 20 minute offset 60 region id".
- Status: unverified

## TIMESTAMP WITH LOCAL TIME ZONE

C-013
- Claim: The column holds the value normalized to the database time zone and carries no time zone offset in the stored data. It is shown in the session time zone on read.
- Source: https://docs.oracle.com/cd/A91202_01/901_doc/appdev.901/a89857/oci03typ.htm
- Type: documentation. License: Oracle documentation, not open.
- Scope: DATAFILE/DUMP(). Whether "normalized to database time zone" means UTC or the database zone is worded ambiguously in my summary; the Analyst should read the page.
- Status: unverified

C-014
- Claim: The byte layout of type 231: not found. Oracle's .NET driver docs only say a byte array in "Oracle internal format" can be passed, without a layout. Whether the length is the 7 to 11 bytes of plain TIMESTAMP is not stated anywhere I read.
- Queries tried: "Oracle Typ=231 timestamp with local time zone dump bytes stored in database time zone 11 bytes".
- Related page seen (not read in detail): https://docs.oracle.com/en/database/oracle/oracle-database/18/odpnt/TimeStampLTZBinData.html
- Status: unverified

## RAW

C-015
- Claim: RAW is a byte string Oracle does not interpret and does not character-set convert. Maximum size 2000 bytes for the column type, and LONG RAW up to 2 GB. Text output is hexadecimal (two characters per byte).
- Source: https://docs.oracle.com/cd/A91202_01/901_doc/appdev.901/a89857/oci03typ.htm and https://docs.oracle.com/cd/B13789_01/server.101/b10759/sql_elements001.htm
- Type: documentation. License: Oracle documentation, not open.
- Scope: DATAFILE/DUMP().
- Status: unverified

C-016
- Claim: How RAW is laid out inside an EXP or Data Pump file (length prefix, chunking for LONG RAW): not found.
- Queries tried: the four dump-body queries listed under Round 1 not found.
- Status: unverified

## NULL and row layout

C-017
- Claim: In a data block row, each column is preceded by a length: 1 byte if the column holds 250 bytes or fewer, 3 bytes if more. A NULL stores only a length of zero and no data. Trailing NULL columns store nothing at all, not even a length, because the next row header ends the row. Columns added later via ALTER TABLE go at the end.
- Source: https://docs.oracle.com/cd/A97385_01/server.920/a96524/c11schem.htm (Oracle9i Concepts, "Row Format and Size" and "Nulls")
- Type: documentation. License: Oracle documentation, not open.
- Scope: DATAFILE block row only. It says nothing about EXP or Data Pump.
- Status: unverified

C-018
- Claim: Special length-byte values used in blocks for NULL or long columns (for example 0xFF as the marker for NULL): not found. The doc says "length of zero" for a NULL, which may be a simplification, so I do not state either way. Please do not read this as confirmation of 0xFF.
- Queries tried: "Oracle row piece format column length byte 0xFF null 0xFE long column trailing nulls not stored block dump".
- Status: unverified

C-019
- Claim: How NULL columns, and trailing NULL columns, are represented inside EXP or Data Pump data records: not found.
- Queries tried: see Round 1 not found (dump-body queries).
- Status: unverified

## Dump-file-specific claims (DUMPFILE)

C-020
- Claim: The only dump-file-specific type claim found: none. No source describes the NUMBER, DATE, TIMESTAMP, RAW or NULL byte form inside EXP or Data Pump files. Every type claim above comes from datafile/DUMP()/OCI material.
- Queries tried: the four dump-body queries under Round 1 not found, plus "Oracle export dump file DATE column stored 7 bytes ...".
- Status: unverified

C-021
- Claim: Legacy EXP files begin with a header that starts with the bytes 0x03 0x00 0x01 followed by the ASCII text "EXPORT:V" and a version string (per a search summary of a file-recovery signature). The Oracle-version string that follows is not recorded here.
- Source: https://git.cgsecurity.org/cgit/testdisk/tree/src/file_dmp.c
- Type: code (behavior of a signature matcher). License: not verified. Page returned "Access Denied" so only the search-result summary was used. FLAG: unclear, treat as GPL-like and do not copy.
- Scope: DUMPFILE (EXP header).
- Status: unverified

C-022
- Claim: A Data Pump dump file starts with a header block, and a Perl script can read it directly. The fields it reports: file type (Data Pump or classic export), database version (example 11.02.00.04.0), file version as string and number, job GUID, master table position and length, character set id and name, creation date and time, job name, platform, language, and block size (example 4096). Exact byte offsets were not recorded in this round because the Round 1 scope limit excludes reading code.
- Source: https://databaseinternalmechanism.com/2016/09/13/how-to-read-datapump-dumpfile-header/
- Type: experiment (author read real files). License: not stated. FLAG: unclear for the script.
- Scope: DUMPFILE (Data Pump header).
- Status: unverified

C-023
- Claim: Test dump files covering all common column types in EXPDP, compressed EXPDP and EXP form can be regenerated from SQL scripts in the odv-testdump repository on Oracle 23ai Free. No expected-value files were found there.
- Source: https://github.com/Open-DUMP-Viewer/odv-testdump
- Type: documentation (README). License: not stated. FLAG: unclear.
- Scope: DUMPFILE.
- Status: unverified

C-024
- Claim: Oracle's own docs say Data Pump and Export files are written in an Oracle-proprietary binary format readable only by the matching Import utility, and no layout specification is given.
- Source: https://docs.oracle.com/en/database/oracle/oracle-database/19/sutil/oracle-data-pump-export-utility.html ; https://docs.oracle.com/en/database/oracle/oracle-database/18/sutil/oracle-original-export-utility.html
- Type: documentation (seen in search summaries, pages not opened). License: Oracle documentation, not open.
- Scope: DUMPFILE.
- Status: unverified

## Conflicts and apparent conflicts between sources

K-1: TIMESTAMP WITH TIME ZONE size. C-009 says 13 bytes (in-database, type 181). C-011 says 20 bytes (OCI external, type 188). They describe different representations. Neither says what a dump file contains. Both recorded.

K-2: NUMBER first byte. C-002 describes a sign bit plus a 7-bit exponent with offset 65. C-004 words it as 64 plus half a decimal exponent for positives and 63 minus that for negatives. These may be the same rule in different words, but the summaries I read are lossy and I did not check the arithmetic against real bytes. Not resolved.

K-3: NUMBER maximum length. C-001 and C-005 say up to 22 bytes. C-006 says 21 (exponent plus 20 digits), with 22 only when the length byte is counted. Whether any dump format stores a separate length byte is not known.

K-4 (not a conflict, a gap): C-017 says NULL is a zero length. Whether a dump file uses zero length, 0xFF, a bitmap or some other marker: not found.

## Counts per type

NUMBER: 6 (C-001 to C-006). DATE: 2 (C-007, C-008). TIMESTAMP plain: 2 (C-009, C-010). TIMESTAMP WITH TIME ZONE: 2 (C-011 found, C-012 not found). TIMESTAMP WITH LOCAL TIME ZONE: 2 (C-013 found, C-014 not found). RAW: 2 (C-015 found, C-016 not found). NULL: 3 (C-017 found, C-018 and C-019 not found). Dump-file specific: 5 (C-020 not found, C-021 to C-024 found).
