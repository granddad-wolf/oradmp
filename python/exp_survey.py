#!/usr/bin/env python3
"""
exp_survey.py v3.4 (2026-10-08) - STRUCTURE-ONLY survey of a legacy EXP dump (conventional or direct path,
table mode RTABLES or owner mode RUSERS).

What it is for
    A first test of the rules of docs/formats/exp.md (section 11, "Rules for a reader") on a real dump
    that must not leave your machine. It walks the whole file with those rules and prints counts,
    lengths, type codes and validity checks. Run it yourself, read the output, then paste it.

What it prints
    header fields (versions, record length, character set ids), the two binary blocks' ids, the string
    list of the second record, per table: path, column descriptors, row count, per-column NULL count,
    byte-length min / mean / max, validity counters, the kinds of statement items, the trailer, and a
    comparison with the export log when --log is given.

What it NEVER prints
    a column value. It does not print the CREATE TABLE text or any statement text either.
    With --no-names it also hides the user name, the file name, table names and column names
    (T1, C1, ...). The date text of the header is printed.

Changes in v3.4: rule K is now the complete list of eight introducing lines of exp.md section 11 (Analyst session 6:
    MODIFY DEFAULT suffix, ANALSTATS prefix, COMMENT ON prefix, CALTER TABLE, PROCACTION, CREATE FORMAT71 PROCEDURE
    prefix, CREATE FORMAT92TRIG, CREATE VIEW prefix; ANALCOMPUTE is not in the list). The header date text is found
    without leading zeros (day, hour, minute, second of one or two digits) and printed as it stands.
Changes in v3.3: the column of a statistics item is read after the LAST SET_COLUMN_STATS in the whole field
    (the call is at the END of the field, after MINVAL / MAXVAL / NOVALS); the shape and raw sample show start and end.
Changes in v3.2: identifiers in the field text are accepted in double OR single quotes; when a column item
    cannot be matched, a SHAPE of its text is printed (every name, value and number replaced by X / 9, only the
    PL/SQL keywords kept); --raw-sample N prints the raw start of the first N statistics fields - for FIXTURES
    only, never run it on a real dump.
Changes in v3.1: every ANALSTATS / ANALCOMPUTE item is recorded and a section "statistics items" prints, per
    table, how many table items (plain field) and column items (field in pieces) it has and WHICH COLUMN NUMBERS
    (C1, C2 ...) have no column item. The column of an item is found by reading the quoted identifiers in the
    first bytes of its field (the DBMS_STATS.SET_COLUMN_STATS call) and matching them with the INSERT column
    list; only the column NUMBER is printed, never a name or a value (with --no-names the table is T1, T2 ...).
    (v3 counted by the quoted names of the line, which was wrong: table and column items both have the table
    name only. Letters: T = table and its columns, I = index, C = constraint index; ANALCOMPUTE has a
    second quoted word.)
Changes in v2: owner mode (second record has METRICSU) is walked, not skipped; field or line is decided by
    rule K (list of introducing lines) first, then by rule G only when the candidate is VALID (a plain field
    followed by LF, a field in pieces, or pieces without the fd ff marker); every item where G and K differ is
    counted and the line before it is named, so a missing line of the K list shows up; a per-kind table of
    the fields (count, length, bytes >= 0x80, "?" bytes) is printed - it covers comments and statistics.
    The K list below has the five lines known to this tool; the others of exp.md section 11 (eight lines in
    all) can be added with --k-prefix / --k-exact / --k-suffix (see Usage) until the tool is updated.

Rules used (all from exp.md, 18c / 11g fixtures; the version of THIS dump may differ, which is the point)
    header 4.x, record 2 at R with METRICST at R + 0x104, items: lines (text + LF), plain fields
    (u16 LE length, bytes, LF), fields in pieces (fd ff, pieces, 00 00, LF); rule G and rule K for
    "field or line" (their disagreements are counted); table data after the INSERT INTO line; rows of
    u16 LE length + bytes, NULL = fe ff, conventional rows end with 00 00, end of table = ff ff + LF,
    direct path with no rows = LF followed by an upper-case letter (rule E2).

Usage
    python3 exp_survey.py DUMP [--log EXPORT.log] [--codec cp1251] [--no-names] [--max-problems 40]
                           [--k-prefix TEXT]... [--k-exact TEXT]... [--k-suffix TEXT]...
    --k-prefix 'PROCACTION' adds "a line starting with TEXT is followed by a field" (also --k-exact: the line
    is TEXT; --k-suffix: the line ends with TEXT). Texts are ASCII, e.g. --k-exact 'CALTER TABLE'.
Exit code: 0 when the whole file was walked and nothing is reported under "problems", 1 otherwise.
Memory: the file is memory-mapped, not read into memory.
"""
import argparse
import collections
import mmap
import re
import struct
import sys

u16le = struct.Struct('<H').unpack_from
u16be = struct.Struct('>H').unpack_from

TYPES = {1: 'VARCHAR2/NVARCHAR2', 2: 'NUMBER', 12: 'DATE', 23: 'RAW', 96: 'CHAR',
         180: 'TIMESTAMP', 181: 'TIMESTAMP WITH TIME ZONE', 231: 'TIMESTAMP WITH LOCAL TIME ZONE'}
CHARSETS = {1: 'US7ASCII', 171: 'CL8MSWIN1251 (id assumed)', 178: 'WE8MSWIN1252',
            873: 'AL32UTF8', 2000: 'AL16UTF16'}
CYR = bytes(range(0xC0, 0x100))     # letters of CL8MSWIN1251 (also valid in other 8-bit sets)
INSERT_RE = re.compile(r'INSERT INTO "(.+?)" \((.*)\) VALUES \((.*)\)\s*$', re.S)


class Fatal(Exception):
    def __init__(self, off, msg):
        super().__init__(msg)
        self.off = off
        self.msg = msg


def cs(n):
    return '%d (%s)' % (n, CHARSETS[n]) if n in CHARSETS else str(n)


K_PREFIX, K_EXACT, K_SUFFIX = [], [], []        # extra introducing lines given on the command line


def is_intro(line):
    """rule K: the eight lines after which a field follows (exp.md section 11, version 3.1) + the extra ones"""
    return (line.endswith(b' MODIFY DEFAULT') or line.startswith(b'ANALSTATS ')
            or line.startswith(b'COMMENT ON ')
            or line == b'CALTER TABLE' or line == b'PROCACTION'
            or line.startswith(b'CREATE FORMAT71 PROCEDURE ')
            or line == b'CREATE FORMAT92TRIG '
            or line.startswith(b'CREATE VIEW ')
            or any(line.startswith(x) for x in K_PREFIX)
            or line in K_EXACT
            or any(line.endswith(x) for x in K_SUFFIX))


def line_kind(line):
    if line.startswith(b'METRICST'):
        return 'METRICST'
    if line.startswith(b'METRICET'):
        return 'METRICET'
    words = []
    for w in line.split():
        if len(words) == 3 or not re.fullmatch(rb'[A-Z_]{1,24}', w):
            break                      # only upper-case command words are shown: no names, no values
        words.append(w.decode('latin-1'))
    return ' '.join(words) if words else '(line)'


class Survey:
    def __init__(self, mm, mask):
        self.mm = mm
        self.size = len(mm)
        self.mask = mask
        self.problems = []
        self.tables = []
        self.kinds = collections.Counter()
        self.gk = []                       # (offset, kind of the line before, who says field) where G and K differ
        self.anal = []                     # statistics items: dict(kw, letters, names, how, flen, off)
        self.fstat = {}                    # kind of the line before a field -> [count, bytes, min, max, hi values, hi bytes, "?" bytes, pieces]
        self.g_rejected = 0                # G candidates that are not valid fields (treated as lines)
        self.nomarker = 0                  # fields in pieces without the fd ff marker
        self.field_max = 0
        self.fields_plain = 0
        self.fields_pieces = 0
        self.tmap = {}
        self.out = []

    # ---- helpers
    def p(self, s=''):
        print(s)

    def problem(self, off, msg):
        self.problems.append((off, msg))

    def tname(self, s):
        if not self.mask:
            return s
        return self.tmap.setdefault(s, 'T%d' % (len(self.tmap) + 1))

    # ---- header and record 2
    def header(self):
        mm = self.mm
        if self.size < 0x100 or mm[0] != 3:
            raise Fatal(0, 'file does not start with byte 03: not a legacy EXP file')
        client = u16be(mm, 1)[0]
        pos = 3
        direct = mm[3:5] == b'D\n'          # exp.md 4.x: direct path inserts 'D' LF at 0x0003
        if direct:
            pos = 5
        lines = []
        for _ in range(7):
            e = mm.find(b'\n', pos, pos + 200)
            if e < 0:
                raise Fatal(pos, 'header text lines not found')
            lines.append(bytes(mm[pos:e]))
            pos = e + 1
        if not lines[0].startswith(b'EXPORT:V'):
            raise Fatal(3, 'no EXPORT:V line after the first bytes')
        try:
            reclen = int(lines[3])
        except ValueError:
            raise Fatal(pos, 'RECORDLENGTH line is not a number')
        ids = [u16be(mm, pos + 2 * i)[0] for i in range(3)]
        head = bytes(mm[pos:min(reclen, self.size)])
        dm = re.search(rb'(?:Mon|Tue|Wed|Thu|Fri|Sat|Sun) (?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) \d{1,2} \d{1,2}:\d{1,2}:\d{1,2} \d{4}', head)
        last = max((i for i, b in enumerate(head) if b), default=0) + pos

        self.p('== header')
        self.p('header path marker        : %s' % ('D LF present (direct path)' if direct else 'none (conventional path)'))
        self.p('exp version line          : %s' % lines[0].decode('latin-1'))
        self.p('exporting user line       : %s' % ('(hidden)' if self.mask else lines[1].decode('latin-1')))
        self.p('export mode line          : %s' % lines[2].decode('latin-1'))
        self.p('RECORDLENGTH              : %d' % reclen)
        self.p('three text numbers        : %s  (18c / 11g fixtures: 0, 72, 0)' %
               ', '.join(l.decode('latin-1') for l in lines[4:7]))
        self.p('client charset, byte 1-2  : %s' % cs(client))
        self.p('ids after the text lines  : client %s | database? %s | national? %s' %
               (cs(ids[0]), cs(ids[1]), cs(ids[2])))
        self.p('date text in record 1     : %s' % (dm.group(0).decode('latin-1') if dm else '(not found)'))
        self.p('last non-zero byte of record 1 at offset 0x%x' % last)
        if client != ids[0]:
            self.problem(0x30, 'client id at byte 1 (%d) differs from the first id after the text lines (%d)' % (client, ids[0]))
        if self.size % reclen:
            self.problem(self.size, 'file size %d is not a multiple of RECORDLENGTH %d' % (self.size, reclen))
        self.p('file size                 : %d bytes = %d records of %d (mod = %d)' %
               (self.size, self.size // reclen, reclen, self.size % reclen))
        self.mode = lines[2].decode('latin-1')
        return reclen, direct

    def record2(self, R):
        mm = self.mm
        m = mm.find(b'METRICST\n', R, R + 0x400)
        self.rec2_kind = 'METRICST'
        if m < 0:
            m = mm.find(b'METRICSU\n', R, R + 0x400)
            self.rec2_kind = 'METRICSU'
        if m < 0:
            raise Fatal(R, 'no METRICST / METRICSU line within 0x400 bytes of the second record')
        self.p('')
        self.p('== second record (starts at %d = 0x%x)' % (R, R))
        self.p('%s line at R + 0x%x  (fixtures: R + 0x104; METRICST = table mode, METRICSU = owner mode)' % (self.rec2_kind, m - R))
        if m - R != 0x104:
            self.problem(m, '%s is at R + 0x%x, not R + 0x104' % (self.rec2_kind, m - R))
        b1 = (u16be(mm, R + 0x3c)[0], u16be(mm, R + 0x3e)[0])
        b2 = (u16be(mm, R + 102 + 0x3c)[0], u16be(mm, R + 102 + 0x3e)[0])
        self.p('block 1 ids (+0x3c, +0x3e): %s | %s    (fixtures: database set | national set)' % (cs(b1[0]), cs(b1[1])))
        self.p('block 2 ids (+0x3c, +0x3e): %s | %s    (fixtures: both = client set)' % (cs(b2[0]), cs(b2[1])))
        strings = []
        q = R + 0xcc
        ok = True
        while q < m - 1:
            n = u16le(mm, q)[0]
            if q + 2 + n > m - 1:
                ok = False
                break
            strings.append(bytes(mm[q + 2:q + 2 + n]).decode('latin-1'))
            q += 2 + n
        ok = ok and q == m - 1 and mm[m - 1] == 0x0a
        self.p('string list (%d strings, %s): %s' % (len(strings), 'consumed exactly' if ok else 'DOES NOT FIT',
                                                    ' | '.join(repr(s) for s in strings)))
        if not ok:
            self.problem(R + 0xcc, 'string list does not end exactly before METRICST')
        return m

    # ---- items
    def try_field(self, p):
        """Is there a valid field at p? Returns (end, pieces list [(start, n)], marker) or None. Reads no value."""
        mm, size = self.mm, self.size
        if p + 2 > size:
            return None
        if mm[p] == 0xfd and mm[p + 1] == 0xff:
            q = p + 2
            pcs = []
            while True:
                if q + 2 > size:
                    return None
                n = u16le(mm, q)[0]
                q += 2
                if n == 0:
                    break
                if q + n > size:
                    return None
                pcs.append((q, n))
                q += n
            if q >= size or mm[q] != 0x0a:
                return None
            return q + 1, pcs, 'fdff'
        n = u16le(mm, p)[0]
        q = p + 2 + n
        if q < size and mm[q] == 0x0a:
            return q + 1, [(p + 2, n)], 'plain'
        # no LF after a plain field: read on as pieces without the marker (u16 length + bytes ... 00 00 LF)
        q = p
        pcs = []
        while True:
            if q + 2 > size:
                return None
            n = u16le(mm, q)[0]
            q += 2
            if n == 0:
                break
            if q + n > size:
                return None
            pcs.append((q, n))
            q += n
        if pcs and q < size and mm[q] == 0x0a:
            return q + 1, pcs, 'nomarker'
        return None

    @staticmethod
    def anal_item(line, off, how, flen):
        words = line.split(b' ', 2)
        kw = words[0].decode('latin-1')
        letters = words[1].decode('latin-1')[:3] if len(words) > 1 else '?'
        names = [x.decode('latin-1') for x in re.findall(rb'"([^"]*)"', line)]
        return dict(kw=kw, letters=letters, names=names, how=how, flen=flen, off=off)

    KEEP = {'DECLARE', 'BEGIN', 'END', 'NULL', 'DBMS_STATS', 'SET_COLUMN_STATS', 'SET_TABLE_STATS',
            'SET_INDEX_STATS', 'SREC', 'STATREC', 'EPC', 'MINVAL', 'MAXVAL', 'BKVALS', 'NOVALS', 'RNGLEN',
            'HISTOGRAM', 'CHARARRAY', 'NUMARRAY', 'DATEARRAY', 'RAWARRAY', 'DISTCNT', 'DENSITY', 'NULLCNT',
            'AVGCLEN', 'EAVS', 'FILLER', 'TRUE', 'FALSE', 'FORCE', 'NO_INVALIDATE', 'STATTAB', 'STATID',
            'OWNNAME', 'TABNAME', 'COLNAME', 'PARTNAME', 'NUMROWS', 'NUMBLKS', 'AVGRLEN', 'SSIZE', 'FLAGS',
            'INDNAME', 'NUMLBLKS', 'NUMDIST', 'AVGLBLK', 'AVGDBLK', 'CLSTFCT', 'INDLEVEL', 'ARRAY'}

    @classmethod
    def shape(cls, head, limit=420):
        t = head[:limit]
        t = re.sub(rb'[A-Za-z_][A-Za-z0-9_$#]*',
                   lambda m: m.group(0) if m.group(0).decode('latin-1').upper() in cls.KEEP else b'X', t)
        t = re.sub(rb'[0-9]+', b'9', t)
        t = re.sub(rb'X(?:\s*X)+', b'X', t)
        return ''.join(chr(c) if 32 <= c < 127 else '?' for c in t)

    @staticmethod
    def column_of(item, table):
        """number (1-based) of the column an ANALSTATS column item is about, or None (reads identifiers only)"""
        head = item.get('head', b'')
        k = head.rfind(b'SET_COLUMN_STATS')
        text = head[k:] if k >= 0 else head
        idents = [(a or b).decode('latin-1').strip('"') for a, b in re.findall(rb"""\"([^\"]*)\"|'([^']*)'""", text[:500])]
        cols = table['names']
        tn = item['names'][0] if item['names'] else None
        start = idents.index(tn) + 1 if tn in idents else 0
        for x in idents[start:]:
            if x in cols:
                return cols.index(x) + 1
        return None

    def report_stats(self):
        if not self.anal:
            return
        self.p('')
        self.p('== statistics items (ANALSTATS / ANALCOMPUTE): framing, tables and columns')
        by = collections.Counter((a['kw'], a['letters'], a['how'] or 'line', min(len(a['names']), 3)) for a in self.anal)
        self.p('  %-12s %-8s %-8s %-9s %6s' % ('keyword', 'letters', 'field', 'names', 'items'))
        for (kw, le, how, nn), n in sorted(by.items()):
            self.p('  %-12s %-8s %-8s %-9s %6d' % (kw, le, how, ('%d+' % nn) if nn == 3 else str(nn), n))
        self.p('  ("names" = quoted names on the line. T-items: plain field = the table, field in pieces = one column.')
        self.p('   I- and C-items are indexes and constraint indexes; they are not counted per table below)')
        tab = {}
        for t in self.tables:
            tab.setdefault(t['name'], t)
        per = {name: dict(tbl=0, cmp=0, cols=collections.Counter(), unparsed=0) for name in tab}
        other = unknown = 0
        for a in self.anal:
            nm = a['names']
            if a['letters'][:1] not in ('T',) and a['kw'] == 'ANALSTATS':
                other += 1
                continue
            if not nm or nm[0] not in per:
                unknown += 1
                continue
            r = per[nm[0]]
            if a['kw'] == 'ANALCOMPUTE':
                r['cmp'] += 1
            elif a['how'] == 'plain':
                r['tbl'] += 1
            else:
                c = self.column_of(a, tab[nm[0]])
                if c is None:
                    r['unparsed'] += 1
                else:
                    r['cols'][c] += 1
        self.p('')
        self.p('  per table')
        self.p('  %-14s %7s %8s %10s %12s  %s' % ('table', 'columns', 'ANALCMP', 'table item', 'column items', 'columns WITHOUT a column item'))
        for name, t in tab.items():
            r = per[name]
            have = set(r['cols'])
            n_items = sum(r['cols'].values()) + r['unparsed']
            if r['tbl'] == 0 and n_items == 0 and r['cmp'] == 0:
                missing_txt = '(table has no statistics items at all)'
            else:
                missing = [i for i in range(1, t['ncols'] + 1) if i not in have]
                missing_txt = ' '.join('C%d' % i for i in missing) if missing else '(none)'
                if r['unparsed']:
                    missing_txt += '   [%d column item(s) whose column was not recognised: the list above is unreliable]' % r['unparsed']
                dup = sorted(i for i, n in r['cols'].items() if n > 1)
                if dup:
                    missing_txt += '   listed twice: ' + ' '.join('C%d' % i for i in dup)
            self.p('  %-14s %7d %8d %10d %12d  %s' % (self.tname(name)[:14], t['ncols'], r['cmp'], r['tbl'], n_items, missing_txt))
        self.p('  not per table: %d index / constraint-index item(s); %d item(s) that match no table of this dump' % (other, unknown))
        shown = 0
        for a in self.anal:
            nm = a['names']
            if (a['kw'] == 'ANALSTATS' and a['letters'][:1] == 'T' and a['how'] not in (None, 'plain') and nm and nm[0] in tab
                    and self.column_of(a, tab[nm[0]]) is None and shown < 2):
                if shown == 0:
                    self.p('')
                    self.p('  column not recognised - SHAPE of the start of such a field (names, values, numbers masked):')
                h = a.get('head', b'')
                self.p('    start: %s' % self.shape(h))
                self.p('    end  : %s' % self.shape(h[-300:], 300))
                shown += 1
        if getattr(self, 'raw_sample', 0):
            self.p('')
            self.p('  raw start of the first %d statistics fields (FIXTURES ONLY):' % self.raw_sample)
            for a in [x for x in self.anal if x.get('head') is not None][:self.raw_sample]:
                self.p('    %s %s %s' % (a['kw'], a['letters'], a['how']))
                self.p('      start: ' + repr(a['head'][:200]))
                self.p('      end  : ' + repr(a['head'][-300:]))

    def note_field(self, kind, pcs, how):
        mm = self.mm
        total = sum(n for _, n in pcs)
        hi = hib = qm = 0
        for st, n in pcs:
            v = mm[st:st + n]
            if not v.isascii():
                hi = 1
                hib += len(v.translate(None, bytes(range(128))))
            qm += v.count(b'?')
        r = self.fstat.setdefault(kind or '(start)', [0, 0, 1 << 30, 0, 0, 0, 0, 0])
        r[0] += 1
        r[1] += total
        r[2] = min(r[2], total)
        r[3] = max(r[3], total)
        r[4] += hi
        r[5] += hib
        r[6] += qm
        r[7] += 1 if how != 'plain' else 0
        if how == 'plain':
            self.fields_plain += 1
        else:
            self.fields_pieces += 1
            if how == 'nomarker':
                self.nomarker += 1
        self.field_max = max(self.field_max, total)

    def walk(self, p):
        mm = self.mm
        prev = b''
        prev_kind = ''
        exits = 0
        while True:
            if p + 2 > self.size:
                raise Fatal(p, 'file ends before the second EXIT line')
            b0, b1 = mm[p], mm[p + 1]
            g = (b0 == 0xfd and b1 == 0xff) or b1 < 0x20       # rule G: looks like a field
            k = is_intro(prev)                                  # rule K: the line before says "field"
            fld = None
            if k or g:
                fld = self.try_field(p)
                if k and fld is None:
                    raise Fatal(p, 'no valid field after the line of kind %r (rule K says field)' % prev_kind)
                if g and not k:
                    if fld is None:
                        self.g_rejected += 1                     # looks like a field, is not: a text line
                    else:
                        self.gk.append((p, prev_kind or '(start)', 'G only'))
                if k and not g:
                    self.gk.append((p, prev_kind or '(start)', 'K only'))
            if fld is not None:
                end, pcs, how = fld
                if prev.startswith(b'ANALSTATS '):
                    it = self.anal_item(prev, p, how, sum(n for _, n in pcs))
                    it['head'] = b''.join(bytes(mm[st:st + n]) for st, n in pcs)[:20000]
                    self.anal.append(it)
                self.kinds[(prev_kind or 'start') + '  -> field'] += 1
                self.note_field(prev_kind, pcs, how)
                p = end
                prev, prev_kind, exits = b'', '', 0
                continue
            e = mm.find(b'\n', p)
            if e < 0:
                raise Fatal(p, 'no LF found for a line')
            line = bytes(mm[p:e])
            kind = line_kind(line)
            self.kinds[kind] += 1
            if line.startswith(b'ANALCOMPUTE '):
                self.anal.append(self.anal_item(line, p, None, 0))
            if line.startswith(b'TABLE "'):
                self.cur = line[7:-1].decode('latin-1') if line.endswith(b'"') else '?'
                self.cur_off = p
            if line.startswith(b'INSERT INTO '):
                p = self.table_data(e + 1, line, p)
                prev, prev_kind, exits = b'', '', 0
                continue
            p = e + 1
            if line == b'EXIT':
                exits += 1
                if exits == 2:
                    return p
            else:
                exits = 0
            prev, prev_kind = line, kind

    # ---- table data
    def table_data(self, p, line, line_off):
        mm = self.mm
        size = self.size
        m = INSERT_RE.match(line.decode('latin-1'))
        if not m:
            raise Fatal(line_off, 'INSERT INTO line not understood')
        tname = m.group(1)
        names = re.findall(r'"([^"]*)"', m.group(2))
        nvals = len(re.findall(r':\d+', m.group(3)))
        ncols = u16le(mm, p)[0]
        p += 2
        if not (ncols == nvals == len(names)):
            self.problem(p - 2, 'column count %d, INSERT lists %d names and %d binds' % (ncols, len(names), nvals))
        ctype, cmax, ccs, csx = [], [], [], []
        for i in range(ncols):
            t = u16le(mm, p)[0]
            ln = u16le(mm, p + 2)[0]
            if t not in TYPES:
                raise Fatal(p, 'unknown column type code %d (column %d of table %s, length field %d): '
                               'descriptor size unknown, cannot continue' % (t, i + 1, self.tname(tname), ln))
            if t in (1, 96):
                c, x = u16le(mm, p + 4)[0], u16le(mm, p + 6)[0]
                p += 8
            else:
                c = x = None
                p += 4
            ctype.append(t)
            cmax.append(ln)
            ccs.append(c)
            csx.append(x)
        zero = u16le(mm, p)[0]
        flag = u16le(mm, p + 2)[0]
        p += 4
        if zero != 0:
            self.problem(p - 4, 'the u16 before the path flag is %d, not 0' % zero)
        if flag not in (0, 1):
            self.problem(p - 2, 'path flag is %d' % flag)
        direct = flag == 1
        rows_from = p

        nulls = [0] * ncols
        minl = [1 << 30] * ncols
        maxl = [0] * ncols
        suml = [0] * ncols
        over = [0] * ncols
        bad = [0] * ncols
        hi_vals = [0] * ncols
        non_utf8 = [0] * ncols
        cyr = [0] * ncols
        hib = [0] * ncols
        qm = [0] * ncols
        cbad = [0] * ncols
        codec = getattr(self, 'codec', None)
        rows = 0
        first = True
        rng = range(ncols)
        while True:
            if p + 2 > size:
                raise Fatal(p, 'file ends inside the rows of table %s' % self.tname(tname))
            if first and direct and mm[p] == 0x0a and 0x41 <= mm[p + 1] <= 0x5a:
                p += 1                                   # rule E2: direct path, no rows
                break
            if u16le(mm, p)[0] == 0xFFFF:
                if mm[p + 2] != 0x0a:
                    raise Fatal(p + 2, 'LF missing after the end marker ff ff of table %s' % self.tname(tname))
                p += 3
                break
            first = False
            for i in rng:
                n = u16le(mm, p)[0]
                p += 2
                if n == 0xFFFE:
                    nulls[i] += 1
                    continue
                end = p + n
                if end > size:
                    raise Fatal(p - 2, 'field of %d bytes runs past the end of the file (row %d, column %d of %s)'
                                % (n, rows + 1, i + 1, self.tname(tname)))
                if n < minl[i]:
                    minl[i] = n
                if n > maxl[i]:
                    maxl[i] = n
                suml[i] += n
                if n > cmax[i]:
                    over[i] += 1
                t = ctype[i]
                if t == 1 or t == 96:
                    if ccs[i] == 2000:
                        if n & 1:
                            bad[i] += 1
                    else:
                        v = mm[p:end]
                        if t == 96 and n != cmax[i]:
                            bad[i] += 1
                        qm[i] += v.count(b'?')
                        if not v.isascii():
                            hi_vals[i] += 1
                            hib[i] += len(v.translate(None, bytes(range(128))))
                            cyr[i] += len(v) - len(v.translate(None, CYR))
                            if codec:
                                try:
                                    v.decode(codec)
                                except UnicodeDecodeError:
                                    cbad[i] += 1
                            try:
                                v.decode('utf-8')
                            except UnicodeDecodeError:
                                non_utf8[i] += 1
                elif t == 2:
                    if n < 1 or n > 22:
                        bad[i] += 1
                elif t == 12:
                    v = mm[p:end]
                    if n != 7 or not (1 <= v[2] <= 12 and 1 <= v[3] <= 31 and 1 <= v[4] <= 24
                                      and 1 <= v[5] <= 60 and 1 <= v[6] <= 60):
                        bad[i] += 1
                elif t == 180 or t == 231 or t == 181:
                    v = mm[p:end]
                    okn = (n == 13) if t == 181 else (n == 7 or n == 11)
                    if not okn or not (1 <= v[2] <= 12 and 1 <= v[3] <= 31 and 1 <= v[4] <= 24
                                       and 1 <= v[5] <= 60 and 1 <= v[6] <= 60):
                        bad[i] += 1
                p = end
            rows += 1
            if not direct:
                if p + 2 > size or u16le(mm, p)[0] != 0:
                    raise Fatal(p, 'conventional path: 00 00 expected after row %d of %s' % (rows, self.tname(tname)))
                p += 2
        self.tables.append(dict(
            name=tname, names=names, off=self.cur_off if hasattr(self, 'cur_off') else line_off,
            rows_from=rows_from, rows_to=p, direct=direct, rows=rows, ncols=ncols,
            ctype=ctype, cmax=cmax, ccs=ccs, csx=csx, nulls=nulls, minl=minl, maxl=maxl, suml=suml,
            over=over, bad=bad, hi=hi_vals, nonutf8=non_utf8, cyr=cyr, hib=hib, qm=qm, cbad=cbad))
        return p

    # ---- report
    def report_tables(self):
        self.p('')
        self.p('== tables (%d)' % len(self.tables))
        for ti, t in enumerate(self.tables):
            nm = self.tname(t['name'])
            self.p('')
            self.p('%s: %s path, %d columns, %d rows, table section at 0x%x, rows 0x%x-0x%x (%d bytes)' %
                   (nm, 'direct' if t['direct'] else 'conventional', t['ncols'], t['rows'], t['off'],
                    t['rows_from'], t['rows_to'], t['rows_to'] - t['rows_from']))
            self.p('  %-3s %-26s %-24s %5s %6s %6s %8s %6s %5s %4s' %
                   ('#', 'column', 'type / charset', 'decl', 'nulls', 'min', 'mean', 'max', '>decl', 'bad'))
            for i in range(t['ncols']):
                cn = ('C%d' % (i + 1)) if self.mask else (t['names'][i] if i < len(t['names']) else '?')
                ty = TYPES[t['ctype'][i]]
                if t['ccs'][i] is not None:
                    ty += ' %s,%s' % (cs(t['ccs'][i]).split(' ')[0], t['csx'][i])
                filled = t['rows'] - t['nulls'][i]
                mean = '%.1f' % (t['suml'][i] / filled) if filled else '-'
                self.p('  %-3d %-26s %-24s %5d %6d %6s %8s %6d %5d %4d' %
                       (i + 1, cn[:26], ty[:24], t['cmax'][i], t['nulls'][i],
                        t['minl'][i] if filled else '-', mean, t['maxl'][i], t['over'][i], t['bad'][i]))
            for i in range(t['ncols']):
                if t['ctype'][i] in (1, 96) and t['ccs'][i] != 2000 and (t['hi'][i] or t['qm'][i]):
                    cn = ('C%d' % (i + 1)) if self.mask else (t['names'][i] if i < len(t['names']) else '?')
                    self.p('  text column %-20s: values with bytes >= 0x80: %d, of them not valid UTF-8: %d, '
                           'bytes 0xC0-0xFF inside them: %d of %d high bytes; "?" bytes in all values: %d%s' %
                           (cn[:20], t['hi'][i], t['nonutf8'][i], t['cyr'][i], t['hib'][i], t['qm'][i],
                            ('; NOT decodable as %s: %d' % (self.codec, t['cbad'][i])) if self.codec else ''))
        # summary of character data
        hi = sum(sum(t['hi']) for t in self.tables)
        nu = sum(sum(t['nonutf8']) for t in self.tables)
        self.p('')
        self.p('== character data, all single-byte-set text columns together')
        self.p('values with bytes >= 0x80: %d; of them not valid UTF-8: %d' % (hi, nu))
        if hi == 0:
            self.p('reading: no non-ASCII text values (or all lost / replaced by "?"; see the "?" counts above)')
        elif nu == 0:
            self.p('reading: every non-ASCII value is valid UTF-8: the text is probably written in UTF-8')
        else:
            self.p('reading: %.1f%% of the non-ASCII values are NOT valid UTF-8: the text is in a single-byte set' %
                   (100.0 * nu / hi))

    def report_items(self):
        self.p('')
        self.p('== statement items (counts only)')
        for k, n in sorted(self.kinds.items(), key=lambda kv: (-kv[1], kv[0])):
            self.p('%8d  %s' % (n, k))
        self.p('plain fields: %d, fields in pieces (fd ff): %d, longest field: %d bytes' %
               (self.fields_plain, self.fields_pieces, self.field_max))
        self.p('rule G / rule K: %d item(s) where they differ (a G-only item is a field that no line of the K list '
               'introduces: a candidate for a missing line), %d G candidates rejected as text lines, '
               '%d fields in pieces without the fd ff marker' % (len(self.gk), self.g_rejected, self.nomarker))
        by = collections.Counter((who, kind) for _, kind, who in self.gk)
        for (who, kind), n in sorted(by.items(), key=lambda kv: -kv[1])[:20]:
            self.p('    %-7s after line kind %-28s %d item(s), first at 0x%x' % (
                who, repr(kind), n, next(o for o, kd, w in self.gk if kd == kind and w == who)))
        self.p('')
        self.p('== fields by the kind of the line before them (counts and lengths only)')
        self.p('  %-30s %7s %10s %6s %8s %7s %9s %9s %8s' % (
            'line before', 'fields', 'bytes', 'min', 'max', 'pieces', 'with>=80', 'bytes>=80', '"?" bytes'))
        for kd, r in sorted(self.fstat.items(), key=lambda kv: (-kv[1][1], kv[0])):
            self.p('  %-30s %7d %10d %6d %8d %7d %9d %9d %8d' % (
                kd[:30], r[0], r[1], r[2], r[3], r[7], r[4], r[5], r[6]))
        self.p('  ("with>=80" = fields holding a byte >= 0x80; "bytes>=80" = how many such bytes; "?" bytes = count of 0x3f.')
        self.p('   In a comment field of a US7ASCII-client dump, bytes>=80 = 0 and many "?" mean the text was lost.)')

    def compare_log(self, path):
        pat = re.compile(r'^\. \. exporting table\s+(\S+)\s+(\d+) rows exported')
        logged = []
        with open(path, 'r', errors='replace') as f:
            for ln in f:
                m = pat.match(ln)
                if m:
                    logged.append((m.group(1), int(m.group(2))))
        self.p('')
        self.p('== comparison with the export log (%d tables in the log)' % len(logged))
        if len(logged) != len(self.tables):
            self.problem(0, 'log lists %d tables, the walk found %d' % (len(logged), len(self.tables)))
        for (ln, lc), t in zip(logged, self.tables):
            same_name = ln.upper() == t['name'].upper()
            ok = same_name and lc == t['rows']
            self.p('  %-4s %s: log %d rows, file %d rows%s' %
                   ('OK' if ok else 'DIFF', self.tname(t['name']), lc, t['rows'], '' if same_name else ' (NAME DIFFERS)'))
            if not ok:
                self.problem(t['off'], 'table %s: log %d rows, file %d rows' % (self.tname(t['name']), lc, t['rows']))


def main():
    ap = argparse.ArgumentParser(description='Structure-only survey of a legacy EXP dump (prints no values).')
    ap.add_argument('dump')
    ap.add_argument('--log', help='export log, to compare the row counts')
    ap.add_argument('--codec', help='also count text values that do NOT decode in this Python codec, e.g. cp1251 (counts only)')
    ap.add_argument('--no-names', action='store_true', help='hide user, file, table and column names')
    ap.add_argument('--max-problems', type=int, default=40)
    ap.add_argument('--raw-sample', type=int, default=0,
                    help='print the raw start of the first N statistics fields (FIXTURES only; prints text of the dump)')
    ap.add_argument('--k-prefix', action='append', default=[], help='extra rule K line: lines starting with TEXT are followed by a field')
    ap.add_argument('--k-exact', action='append', default=[], help='extra rule K line: the line is exactly TEXT')
    ap.add_argument('--k-suffix', action='append', default=[], help='extra rule K line: lines ending with TEXT')
    a = ap.parse_args()
    K_PREFIX.extend(x.encode('ascii') for x in a.k_prefix)
    K_EXACT.extend(x.encode('ascii') for x in a.k_exact)
    K_SUFFIX.extend(x.encode('ascii') for x in a.k_suffix)

    with open(a.dump, 'rb') as f:
        mm = mmap.mmap(f.fileno(), 0, access=mmap.ACCESS_READ)
        s = Survey(mm, a.no_names)
        s.codec = a.codec
        s.raw_sample = a.raw_sample
        fatal = None
        try:
            R, direct = s.header()
            s.p('export mode line: %r (RTABLES = table mode, RUSERS = owner mode)' % s.mode)
            m = s.record2(R)
            end = s.walk(m)
            s.p('')
            s.p('== end of data')
            tail = bytes(mm[end:end + 4096])
            s.p('second EXIT line ends at 0x%x; bytes after it: %d, non-zero: %d' %
                (end, len(mm) - end, sum(1 for b in mm[end:] if b)))
            if any(mm[end:]):
                s.problem(end, 'non-zero bytes after the second EXIT line')
        except Fatal as e:
            fatal = e
            s.problem(e.off, 'FATAL: ' + e.msg)
        s.report_tables()
        s.report_items()
        s.report_stats()
        if a.log:
            s.compare_log(a.log)
        print()
        print('== problems (%d)' % len(s.problems))
        for off, msg in s.problems[:a.max_problems]:
            print('  0x%x: %s' % (off, msg))
        if fatal:
            print('  walk stopped here; offset 0x%x, %d of %d bytes covered' % (fatal.off, fatal.off, len(mm)))
        print()
        print('RESULT: %s%s' % ('WHOLE FILE WALKED, NO PROBLEMS' if not s.problems else 'PROBLEMS FOUND',
                                '; NOTE: %d G/K difference(s), see above' % len(s.gk) if s.gk else ''))
        mm.close()
    return 1 if s.problems else 0


if __name__ == '__main__':
    sys.exit(main())
