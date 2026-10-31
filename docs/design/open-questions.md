# Legacy EXP reader: open questions, requests, risks

Architect session 1, 2026-10-07; session 2, 2026-10-14; sessions 3 and 4, 2026-10-15; sessions 5 and
6, 2026-10-16; sessions 7 to 13, 2026-10-17; sessions 14 and 15, 2026-10-18; sessions 16 and 17, 2026-10-21; session 18, 2026-10-22; sessions 19 to 23, 2026-10-24; sessions 24 and 25, 2026-10-25; sessions 26 and 27, 2026-10-29; session 28, 2026-10-30; session 29, 2026-10-31. "arch n" is a
section of `exp-reader-architecture.md`; rule numbers are those of `docs/formats/exp_reader_rules.md`
(v3.4; v3.5 from session 8; v3.6 from session 17; v3.7 from session 24; v3.9 from session 25).

## Changes in session 29

- (a) **0.1.0a4 is released** (2026-10-31, tag on C `4b9883c`; ADR-111; record in the plan). Pending owner actions, in
  order: the docs push; the clean-up commands of the plan ("Clean-up after a4 (session 29)": the versioned name
  `/var/tmp/oradmp-release-0.1.0a4`, the evidence folder, the deletions, the token counts, `gh auth status`);
  **archive `oradmp-private` read-only now** (ADR-113 item 2, decided; deleting it later is the owner's choice, not
  before a5); the GitHub "About" settings, Issues and private vulnerability reporting checked (ADR-113 item 6); the
  pushes and two dispatches of T-21. Then, unchanged and none blocking: Q-4 with `oradmp survey` from the a4 wheel,
  R-052 on Q-4's condition, R-046 (low), the RHEL 8 host test with the a4 wheel, O-24, Q-13, O-20.
- (b) No new request for the Analyst (a4 raised no format question). Next free number stays **R-053**.
- (c) **Risk 31 closed** (did not happen: the owner's build gave C's CI files, and those are on PyPI). **Risk 32**
  (new): the published a4 sdist does not build from source; closed by T-21 (k).

## Changes in session 28

- (a) Reviewer 6c done at C = `4b9883c` (no blocking finding), T-20b done (ADR-109). The a4 release is the owner's, by
  the plan's "Release 0.1.0a4 checklist (session 28)"; the tag goes on C (ADR-110). Pending owner actions, in order: the
  docs push; the checklist (old root aside, the five action pins with `git ls-remote`, build, hashes, `cargo deny`, VM
  and `rockylinux:8` installs, `stream_memory.py`, tag, TestPyPI, PyPI, tag to `origin`, release page, tokens deleted,
  folders); the record paste. After the tag: Q-4 (with the a4 wheel), R-052 on Q-4's condition, R-046 (low), the RHEL 8
  host test, `oradmp-private` (recommendation: archive read-only), O-24, Q-13, O-20. None of the after-tag items blocks
  a4.
- (b) No new request for the Analyst (6c raised no format question). Next free number stays **R-053**.
- (c) **Risk 30 closed** (ADR-109 item 2). **Risk 31** (new): the owner's release build differs from C's CI files.

## Changes in session 27

- (a) T-20 done (`e1ed10b`); the cut date is 2026-10-26 (ADR-108 items 1, 2). The throwaway users `corpus_own`,
  `corpus_ru` keep their literal passwords in the public tree: decided (ADR-108 item 3). New task T-20b (`pipefail` in
  CI, ADR-108 items 4, 5) before Reviewer 6c. Pending owner actions: T-20b's commit C, push and paste; the `t20b-proof`
  branch, its dispatch, its deletion; `extended` and `manylinux2014` on C; Reviewer 6c with C's full hash; then as in
  session 26 (the `mv` of the old canonical root, the release, O-24, the standing items). The Gogs mirror of the new
  commits stays "moved" (Q-14 step 10).
- (b) No new request for the Analyst (no format fact needed). Next free number stays **R-053**.
- (c) **Risk 30** (new, closed by T-20b when its proof run is pasted): a CI step can show green on a failure of a piped
  command (ADR-108 item 4).

## Changes in session 26

- (a) **Q-14 closed** (the cut is done; ADR-105): its section and "Q-14 steps (session 23)" carry a session 26 status
  (every step done or moved). Closed with it: **O-19** ((a) by the cut; the Project-URL goes into T-20), **O-23** ((a)
  done in T-16a2; (b) lapses on a public repository). Q-11's row names the renamed private repository. New **O-24** (the
  private `CLAUDE.md` text for the public layout; not a gate). Pending owner actions: T-20's commit, push and paste; the
  `extended` and `manylinux2014` dispatches on C; the `mv` of the old canonical root before the release; the release;
  O-24; Q-4 (with the a4 wheel), R-052 (on Q-4's condition), R-046 (low), the RHEL 8 host test, scratch folders,
  `oradmp-private` and the snapshot folder; Q-13; O-20 (optional). None blocks a4 (ADR-106 item 4).
- (b) No new request for the Analyst (T-20 and the cut need no format fact). Next free number stays **R-053**.
- (c) **Risk 27 closed**; risk 28: session 26 line.

## Changes in session 25

- (a) **R-048, R-049, R-050, R-051 closed** (generated and used, Analyst 19 and 20; ADR-99, ADR-100): R-049 negative
  (ORA-00439 in 11g XE). **R-052** stays proposed, very low (ADR-101 item 5). Q-4: session 25 line (the LONG counts now
  come from `oradmp survey`; ADR-101 item 4). Pending owner actions: T-19b's two commits (corpus first), one push, paste;
  T-19 (m) (containers stopped, `ORADMP_WORK`); the `mv` of the canonical root if not done; Q-14 with its [OWNER] items,
  its first step now the rename of the private repository and the remotes (ADR-104 item 3); start Analyst session 21
  (documents only); R-046 (low); the RHEL 8 host test; Q-4; Q-13; optional: push `v0.1.0a1` (archive only after the cut),
  O-20; O-23 (b).
- (b) "Inputs for Analyst session 18": done (R-050 is in `requests.md`). New "Inputs for Analyst session 21" (no fixture
  needed). No new request: R-052 covers what is open. Next free number stays **R-053** (`requests.md`).
- (c) Risk 4: session 25 line (lowered); **risk 29 closed** (65,533 is `fa ff` + n, 65,534 chunked; ADR-99 item 7).

## Changes in session 24

- (a) **R-017, R-040, R-047 closed** (generated and used, Analyst 17; ADR-94). Q-4: the R-040 condition replaced (now: R-048
  matters if a conventional LONG between 65,521 and 65,534 bytes or a LONG RAW above 32,767 bytes exists; also type 231
  for R-050). Pending owner actions: T-19's commit, push and paste; the `mv` of the canonical root (T-16a2 note); Q-14 with
  its [OWNER] items after T-19; R-048, R-049, R-050 runs when convenient (the owner's assistant writes the runners from
  `requests.md`); start Analyst session 18 (documents only); R-046 (low); the RHEL 8 host test; Q-4; Q-13; optional: push
  `v0.1.0a1` (archive only after the cut), O-20. O-23 (a) done (T-16a2 note); O-23 (b) (billing figure) still open.
- (b) New request **R-050** (TIMESTAMP WITH LOCAL TIME ZONE precisions; ADR-96). New "Inputs for Analyst session 18". R-048
  and R-049 are the Analyst's (`requests.md`); not repeated here. Next free number after R-050: **R-051**.
- (c) Risk 4 rewritten (R-040 answered); risk 26: session 24 line; new **risk 29** (the 65,533 / 65,534 collision with
  `fd ff` / `fe ff`).

## Changes in session 23

- (a) **O-22 closed** (T-16a ran green and was pasted; ADR-89). New **O-23**: (a) the confirming rebuild of `d081109` at
  the canonical root (ADR-90 item 4; the owner or T-16a2), (b) the billing figure. **Q-14**: recommendation restated
  (the cut after T-16a2's green run, before the a4 version commit; ADR-93) and the new section "Q-14 steps (session
  23)": the steps in order, the scrub list counted at `172e389`, the [OWNER] items. Pending owner actions: the full
  `d081109` hashes for T-16a2; O-23; T-16a2's push and paste; Q-14 with its [OWNER] items; R-047 and R-017 runs; R-040's
  run; R-046 (low); the RHEL 8 host test; Q-4; Q-13; optional: push `v0.1.0a1` (archive only after the cut), O-20.
- (b) No new request for the Analyst (T-16a2, T-16c and the cut need no format fact). Next free number stays **R-048**.
- (c) Risk 27: session 23 line. New **risk 28** (the build-root dependence of the wheel hash; a wrong reproducibility
  claim).

## Changes in session 22

- (a) **O-21** confirmed done. **O-22**: (c) answered, the owner pastes the CI logs; the token route withdrawn
  (`CLAUDE.md` rule 8); closes with T-16a's first pasted run. New **Q-14** (the one-time cut to a public repository:
  steps, scrub list, checks, timing; nothing decided); **O-19 (a)** superseded by it. Q-4 and the RHEL 8 host test
  restated, no deadline (the host test now with a4; Q-4 also asks for TIMESTAMP precisions). The literal test password
  removed from the O-19 (a) text. Pending owner actions: T-17b's commit; O-22 (a), (b); the pushes and pasted summaries
  of T-16a; Q-14; R-047 and R-017 runs; R-040's run (the owner's assistant writes the runner); R-046 (low); the RHEL 8
  host test; Q-4; Q-13; optional: push `v0.1.0a1`, O-20.
- (b) New request **R-047** (TIMESTAMP(1) to (8): descriptor length and value bytes, both versions and paths; from
  review 06b SF-1). R-017 gains a second reason (the 18c E1 case of the generator). New "Inputs for Analyst session
  16". Next free number **R-048**.
- (c) Risk 17 and risk 25: session 22 lines. New **risk 26** (the generator's common mode and unconfirmed
  combinations) and **risk 27** (the cut).

## Changes in session 21

- (a) 0.1.0a3 is released (ADR-81). **O-21 closed** (`9f1dc4d`). **O-22** still open (restated). **O-19 (a)**, public
  or private, restated with what a stranger would see, the Project-URL consequence and a list to check before opening
  (section "O-19 (a) restated (session 21)"). New small item: the owner confirms that `v0.1.0a3` is on `origin` (the
  Architect could check only `github`). Q-4 and the RHEL 8 host test restated, no deadline. Pending owner actions:
  T-18d's commit, O-22, O-19 (a) visibility, the `origin` tag confirmation, the RHEL 8 host test, Q-4, R-040's run
  (the owner's assistant writes the runner), R-046 (low), Q-13; optional: push `v0.1.0a1`, O-20.
- (b) No new request for the Analyst (T-18d, T-17 and T-16 need no new format fact). Next free number stays
  **R-047**. New "Inputs for Analyst session 15" (the R-003 wording; R-040 stays with the owner's assistant).
- (c) Risk 22 closed for a3 (standing check). Risk 24: session 21 line. New **risk 25** (file names that are not
  UTF-8; T-18d).

## Changes in session 20

- (a) **Q-11 closed** (GitHub Actions; private repository `granddad-wolf/oradmp` exists; ADR-79). **O-19 (a)**
  answered: a3 ships without a Project-URL; it comes with a public repository (expected a4). Visibility (public /
  private) stays open under O-19 (a). **O-16** and **O-19 (b)** now visible in the tree and **closed**: `CLAUDE.md`'s
  Human row names the root `.gitignore`; `uv.lock` is untracked (`git ls-files uv.lock` empty) and listed in the root
  `.gitignore`. New owner actions **O-21** (`.github/` in the Implementer's row of `CLAUDE.md`) and **O-22** (what
  T-16 needs on GitHub). Pending owner actions: T-18c's commit, the a3 checklist (steps 1, 4 to 11), O-19 (a)
  visibility, O-20 (optional), O-21, O-22, the RHEL 8 host test (Q-4), R-040's text, R-046 (low), Q-4, Q-13; optional:
  push `v0.1.0a1`.
- (b) No new request for the Analyst (T-18c, the baseline and T-17 need no new format fact). Next free number stays
  **R-047**. New: "Inputs for Analyst session 14" (R-003 rejected by the owner, as the Analyst suggested; the R-040
  runner).
- (c) Risk 22 is the standing check for the a3 upload (closure review of the exact candidate, TestPyPI first, tag
  pushed last). Risk 24 updated (the I/O error line fixed before the tool's first upload).

## Changes in session 19

- (a) **O-18 closed** (ADR-74: built and tagged commit `ff7d525`; PyPI comparison equal; tokens project-scoped; step 3
  not run on the candidate, accepted). **O-17 done** (`T_BIG` by `corpus/scripts/make_big.sh` and
  `corpus/sql/15_big.sql`, not from 01b; corrected below). **O-19 (b)** and **O-16**: the owner says done; **not yet
  visible in the tree** (their sections). **O-19 (a)** asked again with the a3 timing (ADR-76 item 4). **Q-11**: GitHub
  Actions through a push mirror from Gogs, account later, public or private open. New owner action **O-20** (mark the
  01b `T_BIG` block superseded; text given). Pending owner actions: O-16, O-19 (a) and (b), O-20, the `ORADMP_WORK`
  path for T-18b and Reviewer 6a, the RHEL 8 host test, R-040's text, R-046 (low), Q-4, Q-11 (account, visibility),
  Q-13; optional: push `v0.1.0a1` (ADR-74 item 5).
- (b) No new request for the Analyst (T-18b and ADR-75 / ADR-76 need no format fact). Next free number stays
  **R-047**. New: "Inputs for Analyst session 13" (R-001 to R-007 statuses, the stale R-011 notes, `.ruff_cache`).
- (c) Risk 23 **closed** (the tag is on the built commit); risk 24 updated (T-18 done; exit-code walk and mutation
  table exist; one rule changed by T-18b).

## Changes in session 18

- (a) **Q-10 closed** (owner: upload; 0.1.0a2 on TestPyPI and PyPI 2026-10-22; ADR-70). **Q-11** and **Q-4**
  restated (section "Q-11 and Q-4 restated (session 18)"); Q-11 also asks whether T-18 may run before T-16 while Q-11
  is open (ADR-73). New **Q-13** (the DuckDB side project; the number Q-12 is taken by the closed session 14
  question). New owner actions **O-18** (run B record: the tag on `ff7d525`, the PyPI comparison lines and token,
  step 3 on `4e8f212`; session 19: the built commit was `ff7d525`, O-18 closed) and **O-19** (a public URL for Project-URL; an owner for `uv.lock` in `CLAUDE.md`). **O-16**:
  not reconciled with the repository (its section). **O-17** still open. Pending owner actions: O-16, O-17, O-18,
  O-19, the RHEL 8 host test (run B step 8), R-040's text, R-046 (low), Q-4, Q-11, Q-13.
- (b) No new request for the Analyst (T-13j, T-18 and Q-13 need no format fact). Next free number stays **R-047**.
  New: "Inputs for Analyst session 12" (`docs/formats/README.md`: the `datapump.md` row and the missing `ready`
  status).
- (c) Risk 22 closed for 0.1.0a2 (kept as a standing check); new risk 23 (the release is not rebuildable from its
  tag until O-18 (a)); new risk 24 (the command-line tool is a new public surface).

## Changes in session 17

- (a) New owner action **O-17** (make `T_BIG`; replaces O-15's optional `T_BIG`): needed for T-15b only, gates nothing
  (ADR-69 item 3). Pending owner actions: Q-10 (T-13i (l) wrote the "upload" README text; no owner answer is recorded under `docs/design/`: the owner confirms
  it at run B step 1, or the README's install line goes back to the a1 form before the build), run B's
  owner steps, O-16 (the root `.gitignore`; Reviewer 5 item (8) asks about its change in `469416d`), O-17, run A's
  record and the RHEL 8 install test, R-040, R-046 (low), Q-4, Q-11.
- (b) No new request for the Analyst (`T_BIG` is a volume file with no truth file; ADR-69 needs no format fact beyond
  L4 and reader step 6). Next free number stays **R-047**.
- (c) No risk changed.

## Changes in session 16

- (a) Basis: Reviewer session 4 (no blocking finding). New owner action **O-16** (`.gitignore` in `CLAUDE.md`).
  **Q-10** now asks for the answer before T-13i (l) (the README's install text depends on it); recommendation unchanged:
  upload 0.1.0a2, TestPyPI first (ADR-67). **Q-11** restated with what T-16 needs. Pending owner actions: Q-10,
  Analyst session 11 (before run B), O-16, run A's record and the RHEL 8 install test, run B's owner steps, R-040,
  R-046 (low), Q-4, Q-11, O-15's `T_BIG` (optional).
- (b) No new request (T-13i needs no fixture: its patched copy is made by the test, ADR-64 item 2). Next free number
  stays **R-047**. "Inputs for Analyst session 11" unchanged in content; the owner runs that session before run B.
- (c) Risk 19 closed (Reviewer 4 read T-13h); risk 21 lowered (zero findings); new risk 22 (the irreversible upload).

## Changes in session 15

- (a) **Q-12 closed** (owner): 0.1.0a1 is handed out internally now for the RHEL 8 install test; the PyPI upload only
  after Reviewer session 4 (ADR-63: the upload is 0.1.0a2, the reviewed build). **Q-10** upload part now reads "upload
  0.1.0a2 after Reviewer 4?". **O-14** applied (`CLAUDE.md` names `tools/` and `fuzz/`). New owner action **O-15**
  (before T-15: the path of the `T_ROWS` export, and `T_BIG` if made). Pending owner actions: release run A's owner
  steps (gate, network checks with `cargo deny`, the result of the earlier `tools/release_build.sh` run or a new one,
  the **install test on RHEL 8**, the tag, the hand-out), **O-15**, **Q-10** upload part (run B), **R-040**, **R-046**
  (low), **Q-4** (the real dump's database character set id, type codes, partitioning), **Q-11** (CI service, only for
  T-16).
- (b) No new request: T-13h's answers, T-14 and T-15 need no fixture. The next free number stays **R-047**.
- (c) Risk 19 lowered (no upload before Reviewer 4). New risks 20 (stream errors are text only) and 21 (fuzz findings
  in the framing code).

## Changes in session 14

- (a) New **Q-12** (release 0.1.0a1 before Reviewer session 4 has read T-13h?). New owner action **O-14** (ADR-56 item
  5): add `tools/` (release script) and `fuzz/` (T-14) to the Implementer's row of `CLAUDE.md`, before T-13h.
  Pending owner actions: O-14, Q-12, Q-10 upload part, R-040, R-046 (low), Q-4, Q-11 (only for T-16), the release
  checklist's owner steps (network checks with `cargo deny`, the RHEL 8 install, the tag).
- The Q-5 owner-action text (session 11) names the paths with `oradmp` now and is marked as applied (review 03 N-8).
- (b) No new request: review 03's findings need no fixture. The next free number stays **R-047**. New block "Inputs
  for Analyst session 11": rules T9 and X2, the R-045 record, the R-040 write-up.
- (c) New risk 19: the alpha is released with T-13h unreviewed (Q-12).

## Changes in session 13

- (a) **Q-10 name part closed** (owner): the project is **`oradmp`** (PyPI `oradump` belongs to another project;
  `oradmp` free, checked 2026-10-18); ADR-54, task T-13g. The **upload part stays open**. **Q-5 closed**: the owner
  applied the ownership table, rule 7 and the scope edits in `0a0fda5`. New owner action **O-13** (ADR-54 item 6:
  `CLAUDE.md`, `.claude/agents/*.md`, `docs/prompts/implementer-task.md` renamed; rule 1 names the other project).
  Pending owner actions: O-13 (before T-13g), R-040, R-046 (low), Q-4, Q-10 upload part, Q-11 (only for T-16).
- (b) No new request: the rename needs no fixture (no fixture, truth file or golden file contains the name). The next
  free number stays **R-047**.
- (c) New risk 18: the name is not secured before the first upload; a clean-room slip towards the other project.
- The questions and requests above keep the old name where they were written before ADR-54 (history).

## Changes in session 12

- (a) **Q-9 closed** (owner): the first release is RHEL 8+ / x86_64 only, `manylinux_2_28` (ADR-48); other platforms
  are T-16 (ADR-53). **Q-8 closed** (owner): `Copyright (c) 2026 Maksim Tkachenko`, in `LICENSE-MIT` since T-13.
  New **Q-10** (PyPI: is the name free, and is 0.1.0a1 uploaded?) and **Q-11** (CI service for T-16). The Q-5 text
  gains `.gitattributes` and the CI configuration in the Implementer's row (ADR-52, ADR-53); Q-5 is still not
  applied (`CLAUDE.md` unchanged since `7e7a193`). Pending owner actions: R-040, R-046 (low), Q-4, Q-5, Q-10,
  Q-11 (only for T-16).
- (b) No new request: the T-13 fix-up, the release and T-16 need no fixture. The next free number stays **R-047**.
- (c) Risk 16 closed (Q-9 answered; the owner's RHEL 8 install is step 7 of the release checklist). New risk 17:
  users off Linux x86_64 have no tested wheel until T-16.

## Changes in session 11

- (a) New **Q-9** (glibc of the ETL hosts; wheel baseline, ADR-48). **Q-5 is not applied**: at `a19d873` `CLAUDE.md`
  still has the old ownership table (Architect `docs/decisions/`, "Test engineer", "Python API"), no rule 7, and
  `docs/scope.md` still says AL32UTF8 only and LONG out of scope; the complete replacement text is below ("Q-5 owner
  action (session 11)"), superseding the session 9 / 10 texts. Q-8 unanswered (`owner-decisions.md` has no line):
  T-13 leaves a marked placeholder. Pending owner actions: R-040, R-046 (low), Q-4, Q-5, Q-8, Q-9.
- (b) No new request: `interval_as` and the packaging need no fixture. The next free number stays **R-047**.
- (c) Risk 6 updated: Parquet refuses Interval(MonthDayNano) (verified); `interval_as="string"` is in the alpha
  (ADR-47). New risk 16: the wheel's glibc baseline against the owner's hosts (ADR-48, Q-9).

## Changes in session 10

- (a) **Q-6 closed** (owner, 2026-10-17): MIT OR Apache-2.0, ADR-45. New **Q-8** (small): the copyright line of
  `LICENSE-MIT`. The Q-5 owner action gains one line: the root files (`README.md`, `CONTRIBUTING.md`, `LICENSE-*`,
  `deny.toml`, `pyproject.toml`) go to the Implementer. Pending owner actions: R-040, R-046 (low), Q-4, Q-5, Q-8.
- (b) No new request: S-2 is served by patched copies until R-044 is generated (ADR-43 item 2). The next free
  number stays **R-047**. For the next Analyst session (no fixture): rules T9 (386 lines, 1,907 definitions) and X2
  (the third failed export), the R-045 record, the R-040 write-up (below).
- (c) Risks unchanged in rank. Risk 11 (common-mode error of the Analyst's tools) gets a step: Reviewer session 3
  checks the tools' correctness, not only their reruns.

## Changes in session 9

- (a) No new decision question. Q-5 gets an owner action: the exact `CLAUDE.md` text for the roles of ADR-42
  (below the table). Pending owner actions: R-040 (ready to run), R-046 (low), Q-4, Q-5, Q-6.
- (b) No new request: the review's findings need no fixture (ADR-38). The next free number stays **R-047**.
  For the next Analyst session (no fixture): rules T9 and X2 are stale (below, "Notes on rules v3.5").
- (c) Risks unchanged in rank; risk 11 (common-mode error of the golden files) now has a mitigation that ran:
  Reviewer session 1 regenerated all six files byte-identically, and the review gate repeats it (ADR-42).

## Changes in session 8

- (a) Q-4 gets one line: with R-038 answered, the database set of the owner's dump decides whether the
  ADR-25 stop exists for them (171: gone; any other set: the owner names it). No new question; Q-7 stands.
- (b) R-038 **generated and used** (Analyst session 10; ADR-25 session 8). **R-046** is the Analyst's request
  (proposed, low), already in `requests.md`; the next free number is **R-047** (checked against the index of
  `requests.md`). R-045: the owner delivered `truth_r045.txt` (18c); the Analyst has not yet recorded it. No
  new request from the Architect.
- (c) Risk 2 lowered: for a CL8MSWIN1251 database the Cyrillic LOB risk is gone (confirmed on 11g); it stays
  for other single-byte sets and leans on an assumption for 18c.

## Changes in session 7

- (a) No new question for the owner: N-19 / N-20 are decided records (ADR-34, ADR-35), T-7 / T-8 recorded
  (ADR-36). Q-4 and Q-6 stay open. Q-5 is recorded as **closed** (owner, 2026-10-08, `owner-decisions.md`);
  the session 7 prompt lists it among the open ones; the record is left as it stands until the owner
  says what part of Q-5 is still open.
- (b) Pending: **R-038** (CLOB / NCLOB / BLOB in CL8MSWIN1251; the fixture script exists and waits for the
  owner) and **R-045** (partitioned LONG / LONG RAW refused on 18c; with R-042's run). No new request this
  session; the next free number is **R-046**. After R-038 only `lob_text_kind` (`reader.rs`), the
  unconfirmed check and the tests in `t6l_ru.rs` change (ADR-34 item 9); no API or option changes.
- (c) Risks unchanged in rank. Risk 2 (Cyrillic LOB) now has its exact cost of change on R-038's arrival
  (the line above); risk 12 notes `NUMBER(*,s)` with s < 0 (ADR-35).

## Changes in session 6

- (a) No new question for the owner; Q-4 and Q-6 stay open. Q-6 (licence) is now the first owner decision
  a task needs (T-13, wheel metadata). The N-17 / N-18 choices are decided records (ADR-32, ADR-33).
- (b) Nothing generated since R-040 / R-044 except what the owner reports; R-038 is being prepared. New
  request **R-045** (low): record that Oracle refuses a partitioned table with a LONG / LONG RAW column,
  in R-042's run. R-042 (partitioned table with LOB columns) stays medium, high if Q-4 shows such tables;
  it is now the only fixture that can close the "partitioned table with a non-scalar column" gap
  (ADR-32 item 5).
- (c) Risk 5 narrowed: partitioned tables cannot hold LONG columns (ORA-14085), so the per-part LONG order
  case is out; partitioned LOB tables (R-042) remain.

## Changes in session 5

- (a) The owner decided Q-1 (`round`, with `strict` a rounded value is an error), Q-7 (accepted) and Q-3
  (Python 3.11 or later) on 2026-10-16: all three **closed** (ADR-13, ADR-14, ADR-25). Q-4 and Q-6 stay
  open. No new question for the owner: the session 5 choices are decided records (ADR-27 to ADR-31).
- (b) R-038 is being generated by the owner; nothing changes in the design until it arrives. New request
  **R-044** (years below 1, dates around 1582, TSTZ offset extremes; low priority). New note **A-24**
  (`tools/num_decode.py` step 3 fails, N-14).
- (c) Risk 3 (NUMBER) lowered: the policy is decided; what remains is the rounding on real data, counted.
  New risk 15: values the Arrow types cannot hold (ADR-28) are refused per value.

## Changes in session 4

- (a) Q-1 rewritten with [Y1] v3.4 (computed values have 39 / 40 digits) and the Decimal256 analysis
  (arch 5.2a, ADR-14); still **needs the owner**; the owner has not answered it. Q-3 open. Q-4 now also
  asks for the database character set id, partitioning (methods, subpartitions), LONG sizes and the
  export path. New **Q-7**: confirm the default of `unconfirmed_lob` (ADR-25). Q-6 open.
- (b) State of the requests after Analyst session 9: R-031 to R-037 generated and used; **R-040** open
  (does not block the alpha unless Q-4 shows LONG columns written on the conventional path); R-038
  (owner preparing the fixture) and R-039 not generated, and **not yet recorded in `requests.md`**
  (note A-21). New requests **R-041** (single-column LONG tables on the direct path), **R-042** (a
  partitioned table with LOB columns, NCLOB in the long form), **R-043** (LONG in the CL8MSWIN1251
  database). Notes A-11 to A-17 checked against v3.4; new notes A-18 to A-23.
- (c) Risks re-ranked: the single-byte LOB risk turns from "silently wrong" into "refused per value"
  (ADR-25); conventional LONG of 32 KB or more becomes a stop (R-040); Decimal256 consumer support enters.

## Changes in session 3

- (a) Q-1: `owner-decisions.md` (2026-10-08) records the default type only; the reopened part (default
  of `on_number_overflow`) has no owner decision, so it stays "needs the owner" with the session 2
  recommendation; its wording now says the 40-digit fact rests on one value (A-6). Q-3, Q-4 open; Q-4
  now also asks for the database character set and LOB / partition facts. Q-6 open.
- (b) State of R-027 to R-037 after Analyst session 8; which requests block the alpha; new requests
  **R-038** (LOB in the CL8MSWIN1251 database: long Cyrillic CLOB, NCLOB, BLOB) and **R-039** (column
  list of NOT NULL intervals). Notes A-6 to A-10 answered; new notes A-11 to A-17 on rules v3.3.
- Note: 22 new EXP files (R-031 to R-037) appeared in `18c` / `11g` during this session, not yet written
  up; this file uses none of them. No `11g-cp1251` file was added: R-038 stays open.
- (c) Risks re-ranked: the unsupported-type stop drops (LOB, interval, ROWID are read); LOB text beyond
  the fixtures and partitions on 11g enter.

## Changes in session 2

- (a) Q-1 reopened (overflow policy, ADR-14) with a new recommendation; Q-2 and Q-5 closed by the owner
  (2026-10-08); Q-3 and Q-4 still open; new Q-6 (licence, from N-5).
- (b) Status of R-024 to R-030 after Analyst session 7; what R-011, R-027, R-029, R-030, R-031 block, in
  order of value for the alpha; new requests R-032, R-033; notes A-1 to A-5 answered, new notes A-6 to
  A-10 on rules v3.2.
- (c) Risks re-ranked.

## (a) Questions for the project owner

| # | question | recommendation | blocks |
|---|---|---|---|
| Q-1 (**closed**, owner 2026-10-16: Decimal128(38, 10) with `round`; with `strict=True` a rounded value is an error; ADR-14) | Accepted on 2026-10-08: `NUMBER` without precision is Decimal128(38, 10), "exact or an error". New fact [Y1] v3.4 (R-032): a **quotient or average computed by SQL is stored with 39 or 40 significant digits**, on 18c and 11g alike, so a `NUMBER` column holds such values as ordinary data. Of the 52 truth values, Decimal128(38, 10) holds 30; `round` changes 12 (3 to 0), including **all 6 computed values** (digits 11 to 40 lost); 10 integers are too large under any policy. Decimal256(76, 38) holds 42, `round` changes 7 (digits 39-40 of the 40-digit values; 1E-125, 1E-130 to 0), 3 do not fit (1E+125 etc.); it costs 32 bytes per value, Parquet via pyarrow takes it, but Polars and DuckDB have no native decimal above 38 digits and their import of Decimal256 is not verified. Text holds all, as strings. **What should be the default for a column declared `NUMBER`: type and policy?** (ADR-14, arch 5.2a) | **Decimal128(38, 10) with `round`**, plus `decimal256(76,38)` and `string` as per-column (and dump-wide) overrides for the columns the survey flags. Before the first run the survey prints per `NUMBER` column, counts only: non-NULL; integers and Int64-fit; for Decimal128(38, 10) and for Decimal256(76, 38): exact, changed by `round`, of those to 0, integer part too large; count with 39 / 40 significant digits; largest significant / fractional / integer digits; exponent range (arch 5.2b). The owner then sets overrides for the flagged columns. Cost of `round`: computed values arrive rounded to 10 decimals unless overridden (counted, `W-NUMBER-ROUNDED`). Cost of `error`: every quotient stops the job. Cost of Decimal256 by default: every `NUMBER` column (ids included) gets a type some consumers may refuse. | T-11 (T-4, T-5, T-6 build every option) |
| Q-2 (closed) | Warn and continue by default? | Owner, 2026-10-08: yes, with refinements (ADR-11). | - |
| Q-3 (**closed**, owner 2026-10-16) | Minimum Python version of the wheel? (ADR-13) | Owner: Python 3.11 or later, one `abi3-py311` wheel per platform, pyarrow 15 or later. | - |
| Q-4 | Which column **type codes** do the real tables have; are tables **partitioned** (which methods; subpartitioned?); what are the **database, client and national character set ids** of the real dumps (header [H1, H4]: bytes 1-2, and the second and third u16 after the seventh line); were they written with `direct=y`; how large are LONG / LONG RAW values (largest, count above 32,000 bytes); does anybody need the original zone offset of TIMESTAMP WITH TIME ZONE values? `python/exp_survey.py` prints type codes per table without names or values. | Send counts only: type codes per table, the three set ids, the path flag per table, partition and subpartition line counts, and for LONG columns the largest length field seen. The answer orders the requests: a single-byte database (171) with 112 makes **R-038** a blocker of the alpha (else `E-LOB-UNCONFIRMED` on every long CLOB, ADR-25); 8 / 24 on the conventional path with values of 32 KB or more makes **R-040** one (else the file stops there); partitioning other than range or range-hash, or partitioned tables with LOB columns, makes **R-042** useful; database set 171 with LONG makes R-043 useful. **Session 8:** R-038 is answered, so the database set id now decides the ADR-25 stop itself: if it is **171** the stop is gone (both LOB forms decode; please also say 11g or 18c, since the 171 evidence is 11g only); if it is **another single-byte set** (1, 178 or any other id) please name it, because then every long CLOB and every NCLOB value stops until a fixture of that set exists (the R-038 script on such a database would be R-047). **Session 24**: R-040 is answered (ADR-95): conventional LONG of any size reads except a length field 0xfff1 to 0xfffc; please add to the counts: the largest conventional LONG length field, any LONG between 65,521 and 65,534 bytes, any LONG RAW above 32,767 bytes (R-048), and whether type 231 occurs (R-050). **Session 25**: R-048 to R-051 are answered (ADR-99): every conventional LONG / LONG RAW size reads; `python/exp_survey.py` stops on types 8 and 24, so please use **`oradmp survey`** (counts only) from the a4 wheel or a T-19b build for the LONG part: per LONG / LONG RAW column the line `LONG: ... largest N, ..., 65,521 to 65,534 bytes N`, and any `W-UNTESTED-LAYOUT` or stop printed for such a column; the column line shows type 231 if present. R-052 runs only if a value of 65,522 to 65,524 bytes (the warning) or a field starting `fb ff` / `fc ff` (the stop) shows up. | order of R-048, R-050, R-041 to R-043, R-046; whether a request for another set is needed |
| Q-5 (**closed**, session 13: applied by the owner in `0a0fda5`) | Documents that disagreed with the prompt. | Owner, 2026-10-08: accepted; the owner edits `CLAUDE.md` and `docs/scope.md`. Session 11: neither file is changed at `a19d873`; the complete text is under "Q-5 owner action (session 11)". | agents' ownership (the Implementer writes `python/` and `tests/` without a `CLAUDE.md` row) |
| Q-7 (**closed**, owner 2026-10-16: accepted; ADR-25) | In a dump from a **single-byte database**, a CLOB value in the long form and any NCLOB value are not in the fixtures (R-038), and both plausible readings give text without an error. The design refuses such values by default (`E-LOB-UNCONFIRMED`, a value error: `on_value_error` decides stop or NULL; every other value and table reads) and offers `unconfirmed_lob="decode"` (decode by [L5] with `W-LOB-UNTESTED`). This narrows the owner's Q-2 rule "warn by default" for this one case (ADR-25). **Keep the error as the default until R-038 is generated?** | **Yes**: a refused value is visible and costs a NULL or a stop with row and offset; a wrong guess is wrong Cyrillic text in the warehouse with only a warning. The survey counts the affected values first, so the owner knows the exposure; R-038 removes the question. | T-11 (T-6L builds both) |
| Q-6 (**closed**, owner 2026-10-17: `MIT OR Apache-2.0`; ADR-45) | Licence of the crates and the package (N-5: no licence is set in the manifests). | The owner decides; the Rust ecosystem's usual choice is `MIT OR Apache-2.0`. Whatever is chosen, the clean-room record (`docs/research/sources.md`) stays the basis for the claim that no third-party parser code is in the project. | - |
| Q-9 (**closed**, owner: RHEL 8+ / x86_64 only; ADR-48, ADR-53) | What is the **oldest Linux of the ETL hosts** that will install the wheel (distribution and version, or `ldd --version`)? The VM's build was tagged `manylinux_2_34` (glibc 2.34: RHEL 9, Ubuntu 22.04, Debian 12 and later), which RHEL / Oracle Linux 8 refuses (ADR-48). | Build the first release for **`manylinux_2_28`** (glibc 2.28: RHEL / Oracle Linux 8, Debian 10, Ubuntu 20.04 and later) with `maturin --zig`, no Docker; if a host is older (RHEL 7, glibc 2.17), say so and T-13 builds `manylinux2014` instead. Please also confirm x86_64 Linux is the only platform needed for the alpha. | the wheel tag of T-13 only (default 2.28) |
| Q-8 (**closed**, owner: `Copyright (c) 2026 Maksim Tkachenko`; in `LICENSE-MIT` since T-13) | Which **copyright line** goes into `LICENSE-MIT` (the MIT text needs one; the Apache text does not)? The Implementer writes both files in T-13 (ADR-45 item 2). | `Copyright (c) 2026 Maksim Tkachenko` (the name of the repository's git author); if the project should name its contributors instead, `Copyright (c) 2026 The oradump contributors`. | `LICENSE-MIT` in T-13 only |
| Q-10 (**closed**, session 18: answered "upload"; 0.1.0a2 uploaded to TestPyPI and PyPI on 2026-10-22, ADR-70. History: session 12; **name part closed**, owner, session 13: the name is **`oradmp`**, free on PyPI, checked 2026-10-18; ADR-54. **Upload part open**) | ~~Is the name `oradump` free on PyPI and crates.io?~~ (taken on PyPI by another project; the project is now `oradmp`; crates.io names not needed, ADR-50 item 3). Still open: is 0.1.0a1 **uploaded to PyPI**, or handed out as the wheel and sdist files only? **Session 15** (Q-12 answered, ADR-63): 0.1.0a1 is handed out only; the open part is now: **is 0.1.0a2, the build of the commit Reviewer 4 approved, uploaded to PyPI?** **Session 16**: the answer is needed before T-13i's version commit, because the a2 README's install text depends on it (ADR-67 item 2). | Session 16: **yes**, after Reviewer session 5 (closure) and the a2 RHEL 8 install; **TestPyPI first**, then PyPI, by the owner with `uv publish`, token typed in his own shell and never stored in a file or shown to an agent; first upload with an account-wide token, then a project-scoped one (ADR-67, run B step 10). Session 15: **yes, upload 0.1.0a2** after Reviewer 4 and its RHEL 8 install (checklist run B); the upload secures the name. (Session 12 text:) Upload 0.1.0a1 to PyPI only after Reviewer session 3 and the owner's RHEL 8 install (checklist steps 1, 7): a pre-release version is installed only when asked for (`oradmp==0.1.0a1`), the upload is what **secures the name** (until then anyone can take it, risk 18), and the README then shows that line. If the package should stay internal for now, hand out the files with their SHA-256; the README already says so. | T-13i (l) (README text), run B step 10 |
| Q-11 (session 12; **closed**, session 20: GitHub Actions on the owner's private repository `granddad-wolf/oradmp`, ADR-79; what T-16 still needs is O-21 / O-22, the visibility is O-19 (a); session 26: that private repository is now `granddad-wolf/oradmp-private`, and `granddad-wolf/oradmp` is the public repository of the cut, ADR-105) | Which **CI service** may build and test the project on Windows, macOS and Linux for T-16 (ADR-53)? The repository has no CI configuration and its remote is a self-hosted Git server; the VM cannot run Windows or macOS tests. | A hosted service with Windows, macOS (arm64 and x86_64) and Linux runners, through a mirror of the repository; the corpus is synthetic (ADR-45 item 6) and real dumps never enter the repository, so a mirror carries no real data. Self-hosted runners work too if the owner has the machines. Whether the repository may be mirrored publicly is the owner's call. **Session 16 (ADR-68)**: recommendation unchanged. T-16 needs from the owner: the service; public or private mirror (private costs runner minutes for macOS / Windows); the mirror and the runner registration under the owner's accounts; nothing else: no secret (CI never uploads in phase 1), no real data, no `corpus/work/` (CI benchmarks use the committed fixtures only). | T-16 only (after run B) |
| Q-12 (session 14; **closed**, owner, session 15: internal hand-out of 0.1.0a1 now for the RHEL 8 install test; PyPI only after Reviewer session 4; ADR-63) | Review rule 7 makes T-13h, T-14 and T-15 one group, so Reviewer session 4 reads T-13h **after** 0.1.0a1 is out (ADR-60). T-13h changes the binding (shared stream state, panic guard) and the release build. **Is 0.1.0a1 released before a Reviewer has read T-13h?** | **Internal hand-out: yes**; T-13h's "Done when" carries its own mutations and the checklist re-runs every test on the installed wheel. **PyPI upload: no**, first a short Reviewer session on T-13h only (items 1 and 5 of Reviewer 4's list, "Order of the next tasks (session 14)"), because an uploaded version cannot be replaced, only yanked. | checklist steps 1 and 10 |
| Q-13 (session 18) | The owner wants, after the main project, a **DuckDB extension** that reads Oracle dump files (`read_dmp(...)`) with `oradmp-core` directly (Rust, no Python). Not planned now; what must be true before a plan? (Full text: section "Q-13 the DuckDB side project".) | Plan nothing until the main project (Data Pump included) is done; keep the core's conditions of the Q-13 section from now on; first try the no-extension route (DuckDB's Python package over a pyarrow table) once an Implementer task has tested it. | nothing now |
| Q-14 (session 22; **session 23**: steps in order in "Q-14 steps (session 23)"; ADR-93; **closed**, session 26: the cut is done, P `b55eb10`, S `9950ef2`, public `https://github.com/granddad-wolf/oradmp`; ADR-105) | The **one-time cut**: a new public GitHub repository from a scrubbed snapshot (one commit, no-reply e-mail); the Gogs repository keeps the old history as an archive and mirrors the new commits. Does the owner do it, and when? (Steps, scrub list, checks: section "Q-14".) It replaces the visibility question of O-19 (a). | **Yes**, after T-16a2's green push run (T-16a's first green run is met) and before the a4 version commit (a4 then carries the Project-URL); otherwise after the a4 tag; never between the a4 version commit and the a4 tag. Keep `docs/prompts/`, `docs/review/`, `.claude/` (the clean-room record). | the Project-URL of a4; T-16b's minutes (free when public) |

### Q-11 and Q-4 restated (session 18)

**Session 22**: unchanged, no deadline. **Q-4**: the `survey --no-names` counts of one real dump (set ids, type codes
per table, path flags, partition counts, the largest LONG length); new in this session: whether any column is declared
`TIMESTAMP(f)` with f other than 0, 6, 9 (the survey prints the [T9] texts; R-047 depends on it for its priority).
**The real RHEL / Oracle Linux 8 host test**: still not done for a1 to a3; do it with a4 when it is out (install in a venv,
`oradmp tables` and `python -m oradmp survey --no-names` on a copied fixture, and, new with a4, a copy of a fixture under a
CP1251 file name). Neither gates a release.

**Session 21**: Q-11 is closed (ADR-79). **Q-4** is unchanged and has no deadline: the `survey --no-names` counts of
one real dump (the set ids, type codes per table, path flags, partition counts, the largest LONG length); it orders
R-040 to R-043 and decides whether a request for another single-byte set (R-047) is needed. **The real RHEL / Oracle
Linux 8 host test** is still not done for a1, a2 or a3 (a container is not the same: other kernel, no corporate Python
build): install the current wheel (a3, later a4) in a venv there and run `oradmp tables` and `python -m oradmp survey
--no-names` on a copied fixture; no real data leaves the host. Neither gates a release.

**Session 19 status of Q-11** (owner): **GitHub Actions through a push mirror from the Gogs server: agreed.** The owner
creates the GitHub account later; **public or private is undecided**. T-16 waits for the mirror (ADR-76). Still asked:
(1) public or private (recommendation unchanged: public, if the owner accepts that the synthetic corpus and the design
documents are visible; it gives free runner minutes for macOS / Windows and a Project-URL, O-19 (a)); (2) tell the
Architect when the mirror exists. The swap of ADR-73 is done (T-18 ran first).

**Q-11** (CI service for T-16). Recommendation unchanged (ADR-53, ADR-68): a hosted CI service with Windows x86_64,
macOS arm64 and x86_64 and Linux runners, through a mirror of the repository (the corpus is synthetic; real dumps
never enter the repository); self-hosted runners if the owner has the machines. The owner provides: the service;
whether the mirror is public or private (private costs runner minutes for macOS and Windows; a public mirror would
also give the Project-URL of O-19 (a)); the mirror and the runner registration under the owner's accounts. Nothing
else: no secret (CI never uploads in phase 1), no real data, no `corpus/work/`. **New question (ADR-73)**: if Q-11
is still open when T-13j is done, may T-18 run before T-16? Recommendation: **yes** (T-18 needs nothing from the
owner; T-16 then also runs T-18's tests on every platform).

**Q-4** (the real dumps; counts only, never values). Still open. Wanted: the output of `python/exp_survey.py` (or
`python -m oradmp survey --no-names`, which since 0.1.0a2 prints the same counts) for a real dump: type codes per
table, the database, client and national character set ids, the path flag per table, partition and subpartition
counts, the largest LONG length field. The database set id decides whether long CLOB / NCLOB values stop (ADR-25:
171 decodes; any other single-byte set stops until a fixture of that set exists, then R-047); LONG values of 32 KB
or more on the conventional path make R-040 urgent. Run B's step 8 is the same host question: **install the a2
wheel on a real RHEL / Oracle Linux 8 host** and run `python -m oradmp survey` on a copied fixture there (the
container test of run B does not replace it: other kernel, no corporate Python build).

### Q-13 the DuckDB side project (session 18; not planned)

The owner wants, after the main project, a DuckDB extension `read_dmp(...)` built on `oradmp-core` directly, in Rust,
without Python.

**Why the core is already suitable** (checked in session 18 by reading `crates/oradmp-core/Cargo.toml` and
`src/lib.rs`): it is a separate crate; its dependencies are `thiserror`, `sha2`, `arrow-buffer`, `arrow-array`,
`arrow-schema` (59) and `encoding_rs`, with no PyO3 and no Python (those live only in `oradmp-py`); it hands out
arrow-rs `RecordBatch`es with the schema known before the first batch (`Dump::read_table`, `TableBatches::schema` /
`next_batch`); errors and warnings carry stable codes and no value; it has `#![forbid(unsafe_code)]`.

**What the Architect checks before any plan** (none of it is checked now; every item from DuckDB's and the crates'
own documentation at that time):
1. DuckDB's current options for an extension written in Rust: the C extension API and any Rust crate or template
   DuckDB documents for it; whether that API is stable across DuckDB versions; how a table function receives
   arguments, reports progress and errors.
2. Binaries: one per platform and per DuckDB version (or per stable C API version); how they are distributed
   (DuckDB's community extension repository or a local unsigned load) and signed; what CI that needs (T-16's matrix
   is the start).
3. Licences: DuckDB's licence and that of every crate the extension pulls in, against `MIT OR Apache-2.0`
   (`cargo deny`, ADR-45).
4. Arrow hand-over: whether the extension can pass arrow-rs batches through the Arrow C Data Interface / C stream or
   must fill DuckDB's own vectors; which arrow-rs version a DuckDB binding crate pins against the core's 59; how
   Decimal128(38, 10), Decimal256, Interval(MonthDayNano), Duration, Timestamp(s / ns), LargeUtf8 and LargeBinary
   arrive as DuckDB types.
5. Errors and warnings: how a code, an offset and the counted warnings reach a DuckDB user.
6. Threads and files: DuckDB's parallel scans against one sequential reader per table; DuckDB's file system
   (remote paths) against the core's `open(path)`.

**Conditions the core keeps meanwhile** (a change to one of them needs a decision record that names both bindings):
- No Python, PyO3 or pyarrow dependency in `oradmp-core` (T-16's CI can check it with `cargo tree -p oradmp-core`).
- The entry chain `dump::open` -> `Dump::scan` / `catalog` -> `Dump::read_table` -> `TableBatches::schema` /
  `next_batch` / `warnings` / `rows_read` keeps its shape.
- Output stays plain arrow-rs `RecordBatch` with the schema fixed before the first batch and the `oradmp:` field
  metadata of arch 5.
- Error and warning codes stay stable strings (`ErrorCode::as_str`, `WarningCode::as_str`); messages hold no value.
- `#![forbid(unsafe_code)]` stays; any FFI lives in the binding crate, never in the core.
- The arrow crates move only by the bump rule of ADR-3 (arch 9), so a binding can pin the same version.
- Options whose value changes the output (number types, `interval_as`, timestamp unit, charsets) keep their
  defaults in the core's `Options` / `ReadOptions`, not only in a binding (whether every one does today is
  unverified: to check at plan time).
- No global state in the core, and its readers stay movable between threads (`Send`; unverified today: a
  compile-time assertion is a small test task when the side project starts).
- Licence `MIT OR Apache-2.0` and the `deny.toml` allow-list unchanged.

**The cheap alternative** (no extension): DuckDB's Python package can query a pyarrow table directly, so `oradmp` ->
`read_all()` (or the reader's Arrow stream) -> a DuckDB SQL query needs no extension at all; this has **not** been
tested on our output (Decimal128(38, 10), Interval(MonthDayNano), Duration, LargeUtf8, Timestamp(s)). A README
example of it is worth adding, but only after an Implementer task has tested it (a test-only `duckdb` dependency,
one fixture per Arrow type of arch 5.1, on Python 3.11 and the newest; the note records what each type becomes);
recommendation: a small task after T-18, not a gate of a3.

### Q-14 the one-time cut to a public repository (session 22; needs the project owner; nothing decided)

**Session 26: closed (ADR-105).** The cut is done: P = `b55eb10376a9402aee36a1ee61779cf087c7bfb3` (archive), S =
`9950ef204fb20b4d7f5d9da31467ffbe9aeeff60` (public, S's tree = P's tree + `HISTORY.md`), the public repository
`https://github.com/granddad-wolf/oradmp`. The nine checks: 1 to 5 by the owner on S (ADR-105 item 3); 6 green twice,
counts equal T-19b's (488 + 4, 251); 7 `tests/test_project_name.py` passed in the owner's fresh clone (pytest 251); 8
`tools/release_build.sh S` at a scratch root: sdist equal to CI's, wheel different as ADR-90 says (its `check_dist` line
was not in the facts; CI's is `ok`); 9 no archive hash rewritten. Reviewer 6c repeats the residue checks (plan, "Release
0.1.0a4", session 26 block, item 10). The text below is history.

**The owner's preference** (session 22): no change of visibility of the existing repository. Instead, once: a **new
public GitHub repository** is created from a **scrubbed snapshot** (one initial commit, author e-mail GitHub's
no-reply address); development continues there; the private Gogs repository keeps the complete old history as an
**archive** and mirrors the new commits. Hashes cited in the documents before the cut (`ff3d6b2`, `edc62a5`, ...)
then name commits of the archive. This replaces the visibility question of O-19 (a): the private `granddad-wolf/oradmp`
stays private or is deleted; the history is never made public.

**Question**: does the owner do the cut, and when? **Recommendation**: yes, after T-16a's first green Linux run (the
owner's lean) and **before the a4 version commit**, so that a4 is the first release built from public history and can
carry the Project-URL; otherwise after the a4 tag. **Never between the a4 version commit and the pushed a4 tag**:
the reviewed candidate, the build and the tag must be one commit of one repository. No agent session runs during the
cut.

**Steps** (the owner; agents take no part in the push or the GitHub settings):
1. Freeze: all agent work committed; `git status --porcelain` empty in the owner's clone; the last private commit P
   noted (full hash).
2. Snapshot: `git archive P` into an empty folder (tracked files only; nothing of `corpus/work/`, `target/`, `.venv/`,
   untracked files).
3. Scrub in the snapshot (the list below), then `sha256sum -c` of the three `SHA256SUMS` (the fixtures must be
   byte-identical: scrubbing never touches `corpus/fixtures/`).
4. `git init`; `git config user.name` / `user.email` (the no-reply address) **in that repository**; one commit; record
   its hash S and that S's tree is P's tree minus the scrub (a `diff -r` of P's archive and the snapshot lists only
   scrubbed files).
5. Create the public repository; set before the first push: Actions with a read-only `GITHUB_TOKEN`, approval for
   workflows from outside contributors, branch protection on `master` (no force push), Issues / Discussions / a
   security contact as the owner decides (O-19 (a) check 5). Push `master` only; no old tags.
6. Gogs: keep the old history under its own name (for example the branch `archive/master` and the old tags,
   untouched, never force-pushed); add the public repository as the source of new commits (a pull mirror, or pushes
   by hand). The owner's clone and the VM's checkout switch their main remote to the public repository.
7. Tell the Architect S and the date; the next Architect session adds the HISTORY note's hashes to the plan and
   closes Q-14.

**What to scrub, and what to decide** (counted at HEAD `57f28f4` plus this session's working tree, session 22):
- **`<home>` paths**: 9 tracked files (3 in `docs/design/`, 4 in `docs/prompts/`, 2 in `docs/review/`); none in the
  binary fixtures (`git grep -l '<home>' -- corpus/fixtures`: 0). Replace by `<home>` or remove.
- **Container passwords**: `corpus/sql/` and `corpus/scripts/config.sh` (the owner's list). The word "password" stands
  in 7 files of `corpus/scripts/` (`config.sh` among them), in `docs/formats/requests.md`, in three files of
  `docs/design/` and in prompts; in SQL a password follows `IDENTIFIED BY`: the owner greps for each password string
  itself (`git grep -n '<string>'`) and replaces it by a variable or a placeholder. (This session removed the one
  literal password from `open-questions.md`.) The scripts must
  still read the password from somewhere the owner keeps outside the repository.
- **The commit e-mail**: only the new commit carries an identity (the no-reply address). The address
  `<owner e-mail>` also stands in the text of `docs/design/open-questions.md` (O-19 (a)) and
  `docs/prompts/architect-21-update.md`: replace or accept.
- **Keep or drop** (decide): `docs/prompts/` (42+ prompts), `docs/review/` (8 reports), `.claude/` (agent
  definitions). Recommendation: keep all three after reading them once: they are the clean-room record (who read
  what, which source was refused); dropping them leaves `CLAUDE.md` and the plan pointing at missing files. If dropped,
  `CLAUDE.md`'s layout and the plan's references must be edited in the same snapshot.
- **A HISTORY note** (new file; its owner to be named in `CLAUDE.md`, recommendation: the Human): "The history before
  <date> is in a private archive. Commit hashes cited in `docs/` before <date> name commits of that archive. The first
  public commit is S, made from archive commit P with these files changed: <list>."
- **Releases and provenance**: the tags `v0.1.0a1` to `v0.1.0a3` are not pushed to the public repository (their
  commits do not exist there). The README (Implementer, through a task) or the HISTORY note gets one sentence:
  "0.1.0a1 to 0.1.0a3 were built from commits of the private archive (a3: `ff3d6b25...`); `tools/release_build.sh
  <commit>` reproduces them only from there. From 0.1.0a4 on, every release is built from a public commit." The
  reproducible-build claim in the documents is then true per repository.

**Checks before the first push** (the owner, on the snapshot):
1. `git grep -nI '<home>'` and `git grep -n` for each password string, the Gogs host name and the owner e-mail domain: no hit
   (or only the ones the owner accepted).
2. `grep -rnE 'pypi-|ghp_|github_pat_|BEGIN .*PRIVATE KEY' .`: no hit.
3. `sha256sum -c` of the three `SHA256SUMS`: all `OK`.
4. `diff -r` of `git archive P` and the snapshot lists only the scrubbed files.
5. Nothing in `corpus/` ever came from real data (O-19 (a) check 3).
6. On the new repository's first push: T-16a's workflow runs unchanged and is green (ADR-87 item 5); its counts equal
   the last private run's.
7. `tests/test_project_name.py` passes in a clone of the new repository (it uses `git ls-files`).
8. `tools/release_build.sh S` builds, `check_dist: ok` (a test build, not uploaded).
9. The documents that cite archive hashes are not rewritten; the HISTORY note explains them.

### Q-14 steps (session 23; ADR-93; the owner does every step; no agent session runs from step 2 to step 9)

Counted at `172e389` with `git grep -l` (file names only; no value printed). Items marked **[OWNER]** are the owner's
decisions; the recommendation follows each. The a4 version commit comes **after** step 11 (ADR-93 item 1).

0. **Before**: T-16a2 committed and its push run green on the private repository; O-23 (a) recorded; every agent
   session ended; the T-16a2 note holds the full hashes (the public tree should carry a complete record).
1. **[OWNER] Where to scrub.** (i) In place, in the private tree, as normal commits before the freeze (each folder by
   its owner: the owner `docs/prompts/`, `corpus/`, `CLAUDE.md`; an Architect session `docs/design/`; the owner or a
   Reviewer session `docs/review/`), so that the snapshot equals P exactly; or (ii) in the snapshot only, so that the
   private tree keeps the originals. **Recommendation: (i)**: then check 4 is "no difference" except the HISTORY note,
   and later sessions edit the same text that is public.
2. **The scrub list**:
   - the owner's home path: 10 files (`docs/design/decisions.md`, `implementer-notes.md`, `open-questions.md`;
     `docs/prompts/architect-14-update.md`, `architect-22-update.md`, `architect-23-update.md`,
     `review-03-python-packaging.md`, `review-04-t13h-t15.md`; `docs/review/review-02-t9b-t11.md`,
     `review-03-python-packaging.md`); find them with `git grep -nI "$HOME"`; replace by `<home>` (the meaning of the
     ADR-56 evidence survives).
   - the e-mail address: 2 files (`docs/design/open-questions.md`, O-19 (a); `docs/prompts/architect-21-update.md`);
     find with `git grep -n "$(git config user.email)"`. **[OWNER]** replace or accept; recommendation: replace.
   - container passwords: `IDENTIFIED BY` once each in `corpus/sql/00_setup.sql`, `05_own_setup.sql`,
     `07_ru_setup.sql`; `corpus/scripts/config.sh`; the word "password" also in `CLAUDE.md`, six more scripts
     (`db_setup.sh`, `docker_run.sh`, `make_own.sh`, `make_ru.sh`, `order_of_use.sh`, `run_r045.sh`),
     `docs/formats/requests.md`, three files of `docs/design/` and two prompts: the owner greps each password string
     (`git grep -n '<string>'`) and replaces it by a variable the scripts read from outside the repository.
     **[OWNER]** where the scripts read it from. Scrubbing never touches `corpus/fixtures/`.
   - the Gogs host name and any token form: as checks 1 and 2 of section "Q-14".
3. **[OWNER] Keep `docs/prompts/`, `docs/review/`, `.claude/`?** Recommendation: keep all three (the clean-room record;
   `CLAUDE.md` and the plan point at them).
4. **The HISTORY note** (a new root file, e.g. `HISTORY.md`; **[OWNER]** its name, and its row in `CLAUDE.md`'s layout
   and ownership: the Human): the date; "The history before <date> is in a private archive; commit hashes cited in
   `docs/` before <date> name commits of that archive; the first public commit S was made from archive commit P; files
   changed: <list>." and the provenance sentence: "0.1.0a1 to 0.1.0a3 were built from commits of the private archive
   (a2 `ff7d5255...`, a3 `ff3d6b25...`); their tags exist only there, and `tools/release_build.sh <commit>` reproduces
   them only from there. From 0.1.0a4 on, every release is built from a commit of this repository." Recommendation:
   the sentence lives in the HISTORY note only; the README gets nothing about it.
5. **Freeze**: `git status --porcelain` empty; P = the full hash of `HEAD`.
6. **Snapshot**: `git archive P | tar -x -C <empty folder>` (tracked files only); add the HISTORY note (and the
   `CLAUDE.md` row) there if not committed in P; `sha256sum -c` of the three `SHA256SUMS`: all OK.
7. **One commit**: `git init -b master`; in that repository only, `git config user.name` and `user.email` = GitHub's
   no-reply address (**[OWNER]** the exact address from the GitHub e-mail settings); `git add -A`; one commit S. The
   checks 1 to 5 of section "Q-14" on S (with (i), check 4 lists only the HISTORY note and `CLAUDE.md`).
8. **The public repository**: **[OWNER]** its name. The Project-URL wants `granddad-wolf/oradmp`, which the private
   repository holds: rename the private one first (e.g. `oradmp-private`) or delete it, or choose another public
   name. Before the first push (**[OWNER]** each): Actions allowed, recommended "selected actions" = the four pinned
   ones plus `actions/upload-artifact`; workflow permissions "read repository contents"; approval required for
   workflows from outside contributors; a ruleset on `master` (no force push, no deletion); Issues, Discussions, a
   security contact.
9. **Push `master` only** (`git push <public> master`; never `--tags`). The workflow runs unchanged (ADR-87 item 5).
   **Nothing else happens there until this run is green**: no commit (except a fix of this run, by the Implementer),
   no tag, no release, no settings change. Expected: the counts of T-16a2's last private run; `check_dist: ok`. The
   sdist hash is **not** expected to equal P's (S has another commit time, so another `SOURCE_DATE_EPOCH`). Then one
   dispatch (`extended`, `manylinux2014`; free on a public repository) and the summaries pasted (ADR-92 item 3).
10. **Gogs and the clones**: the old history stays as an archive branch (e.g. `archive/master`) and the old tags,
    never force-pushed; the public repository becomes the source of new commits (pull mirror or by hand). The owner's
    clone and the VM checkout switch `master` to the public history in place (`git fetch <public>`, `git checkout -B
    master <public>/master`; the untracked `.venv`, the dev build and `corpus/work/` stay; **[OWNER]** or a fresh
    clone, then the dev build is redone with `maturin develop --features test-probes`).
11. **Checks 6 to 9** of section "Q-14" (run green, `test_project_name.py` in a clone, `tools/release_build.sh S`
    test build at a scratch root, archive hashes not rewritten); then tell the Architect P, S, the date and the public
    address. The Architect session (plan row 2') closes Q-14; the a4 version commit follows with `[project.urls]`.

**Session 25 (the owner's decisions as reported; ADR-104 item 3).** Step 0 now also needs T-19b and T-19 (m) done (plan
order table, session 25). Step 3: `docs/prompts/`, `.claude/` and `CLAUDE.md` stay **private** (own small Gogs
repositories, ignored in the public tree); `docs/design/`, `docs/formats/`, `docs/review/` are public. Step 8: the public
repository is `oradmp`; the private GitHub repository is **renamed** as the cut's first step, and every remote of the
private history is repointed at the renamed address and checked with `git remote -v` **before** the public repository is
created (after that, the old address names the public repository). The detailed checklist is written by the owner's
assistant, not by an Architect session; this section stays the design's summary.

**Session 26: status of every step** (owner's report, ADR-105; "checked" = read in session 26):
| step | status |
|---|---|
| 0 before | **done**: T-16a2 (`af6a54a`), O-23 (a) (ADR-98 item 1), and per session 25 T-19b (`1bb7785`) and T-19 (m) (`0520cd5`) |
| 1 where to scrub | **done**, option (i) in place: `0cbd165`, `6a5869c`, `ac58e08` before P |
| 2 scrub list | **done**: home path and e-mail replaced (`0cbd165`); the `corpus` password read from `CORPUS_PW` (`ac58e08`); the two throwaway users' passwords stay (owner); Gogs host and token checks clean |
| 3 keep or drop | **decided otherwise than recommended** (owner, session 25): `docs/prompts/`, `.claude/`, `CLAUDE.md` private (two Gogs repositories), ignored; `docs/review/` public |
| 4 HISTORY note | **done**: `HISTORY.md` in S only (not in P; checked); its row in `CLAUDE.md` **moved** to O-24 |
| 5 freeze | **done**: P = `b55eb10376a9402aee36a1ee61779cf087c7bfb3` |
| 6 snapshot | **done**; three `SHA256SUMS` OK |
| 7 one commit | **done**: S = `9950ef204fb20b4d7f5d9da31467ffbe9aeeff60`, no-reply author and committer (checked); checks 1 to 5 clean; check 4: only `HISTORY.md` (checked) |
| 8 public repository | **done**: the private one renamed `oradmp-private` first (archive remote checked); `granddad-wolf/oradmp` public; settings as ADR-105 item 8 (allow-list GitHub's actions + the `taiki-e` pin; `upload-artifact` is GitHub's, so covered) |
| 9 push `master` only | **done**: no tags; run green twice; `extended` and `manylinux2014` dispatched and reported |
| 10 Gogs and clones | **done** for the archive (`master` and `archive/master` at P, old tags) and the owner's clone (fresh clone, one remote `origin`, dev build redone); **moved**: a Gogs mirror of the new commits is not reported and no step needs it (the owner's choice) |
| 11 checks 6 to 9 | **done** (see "Session 26: closed" at the top of section "Q-14"); this session closes Q-14 |

### O-24 owner action (session 26, ADR-105 item 10): the private `CLAUDE.md` after the cut

Not a gate; the file is private, so the public tree does not change. In `.claude/CLAUDE.md`, so that agents read a true
layout:
1. "## Repository layout": after `CLAUDE.md` add `HISTORY.md    what the old hashes mean; a1 to a3 provenance (Human)`;
   mark `docs/prompts/`, `.claude/` and `CLAUDE.md` "(private; ignored in the public repository)"; one line "Old history:
   `~/p/oradmp-archive` (read-only; every hash cited in `docs/` before session 26 names a commit there)".
2. "## Ownership", the Human row: add "`HISTORY.md`" after "`docs/scope.md`".
3. Rule 8: "A release is built only from a committed tree with `tools/release_build.sh <commit>`" becomes "... only from
   a commit of the public repository with `tools/release_build.sh <commit> --twice --release` at the canonical root".
4. Optional: one line that the remote is `origin` (the public repository) and that no agent pushes.

### O-14 owner action (session 14, ADR-56 item 5): `tools/` and `fuzz/` for the Implementer (applied; `CLAUDE.md` names both, session 15)

Before T-13h. In `CLAUDE.md`:
1. "## Repository layout": after the `tests/` line add

        tools/                     build and release scripts, e.g. tools/release_build.sh (Implementer; not shipped)
        fuzz/                      cargo-fuzz targets (Implementer, T-14)

2. "## Ownership", the Implementer's row: after "`tests/` (...)" insert "; `tools/` (build and release scripts); `fuzz/`
   (cargo-fuzz targets)".
If the owner prefers no new folder, the script goes to `tests/release_build.sh` (the Implementer's already) and ADR-56
is amended; the recommendation is `tools/`: `tests/` holds test code, the script is a build step, and ADR-57 keeps
`tools/` out of both archives by name.

### O-16 owner action (session 16, ADR-65 item 5): the root `.gitignore` (**closed**, session 20)

**Session 20: closed.** `CLAUDE.md`'s Human row names `.gitignore` (since `13e283d`); nothing more to do (the three
commented `fuzz/` lines in the root `.gitignore` are harmless).

**Session 19 status: owner says done, not yet visible in the tree.** The owner answers "done" again. Checked at the
start of session 19 (HEAD `2ba62eb`): `grep -n gitignore CLAUDE.md` is empty (the Human row unchanged; `CLAUDE.md` last
changed in `c1bfeba`); the root `.gitignore` (last changed in `a84e7c4`) ends with the line `CLAUDE.md` and still has
the three `fuzz/` lines commented out. Not closed: please commit the Human-row text below (and the `.gitignore` you
intend), or say where it is.

**Session 18 status: not reconciled with the repository.** The owner reports O-16 done. At `ff7d525` the Human row of
`CLAUDE.md` does **not** name the root `.gitignore` (`CLAUDE.md` last changed in `c1bfeba`); the root `.gitignore`
has an **uncommitted** change that adds the line `CLAUDE.md` (a tracked file, so the line has no effect on it), and the
three `fuzz/` lines are still commented out (`469416d`). Please commit the intended `CLAUDE.md` row and `.gitignore`
text, or say what O-16 "done" means.

T-14 (`abd42ad`) added three lines to the root `.gitignore` (`fuzz/corpus/`, `fuzz/artifacts/`, `fuzz/target/`), a
file that no row of `CLAUDE.md` names; `fuzz/.gitignore` already ignores the same paths, so the lines are redundant
and harmless. The root file also carries the real-data rules (`corpus/work/`, `real_dumps/`, `*.real.dmp`), so the
recommendation is that it stays the owner's. In `CLAUDE.md`, "## Ownership", the Human row becomes:

    | Human | `corpus/` (fixtures, `corpus/sql/`, scripts), `python/exp_survey.py`, `CLAUDE.md`, `docs/scope.md`, `docs/prompts/`, `.claude/`, the root `.gitignore` (an agent puts ignore rules into the `.gitignore` of a folder it owns, e.g. `fuzz/.gitignore`) |

Whether the three T-14 lines stay is the owner's call (recommendation: remove them in the same commit; nothing
depends on them). If the owner prefers the Implementer to own the root file instead, the alternative is to add
"`.gitignore`" after "`.gitattributes`" in the Implementer's row; then the real-data lines are in an agent's file,
which this design does not recommend.

### O-18 owner action (session 18, ADR-70 items 4, 6): finish the run B record (**closed**, session 19, ADR-74)

**Session 19**: (a) the build clone's `HEAD` was `ff7d525` (owner): the tag is on the built commit, nothing to move;
session 18's `4e8f212` was wrong. (b) PyPI simple-page sha256 = `dist/SHA256SUMS` for both files; install from
pypi.org works; both indexes have project-scoped tokens, the account-wide ones deleted; a stray `dist/UV_PUBLISH_TOKEN`
removed (it could hold only an account-wide token, now deleted; ADR-74 item 3). (c) Step 3 was not run on the
candidate; accepted for a2 (ADR-74 item 4); nothing remains. The text below is history.

(a) **The tag.** In this repository `v0.1.0a2` points to `ff7d525` (`git cat-file -p v0.1.0a2`: `object ff7d5255...`,
tagger date 2026-10-22 14:23:28 -0500), not to the built commit `4e8f212` [session 19: wrong; `ff7d525` was built, ADR-74]. Read the `commit:` line of the run B log of
`tools/release_build.sh`. If it is `4e8f212`: `git tag -f -a v0.1.0a2 -m 0.1.0a2 4e8f212` and `git push --force
origin v0.1.0a2`; then `git rev-parse v0.1.0a2^{commit}` prints `4e8f2122445cf089e183071c40c4e547c613c59c`. If it is
`ff7d525`: say so; the record's candidate is corrected and the tag stays. (Why it matters: the script takes
`SOURCE_DATE_EPOCH` from the commit time, so the tag must name the built commit for the release to be rebuilt.)
(b) **PyPI comparison**: the two `#sha256=` values of `https://pypi.org/simple/oradmp/` against the two of the record,
the install from `https://pypi.org/simple/` and `python -m oradmp survey` (step 10 (c)); and whether the PyPI token
too is now project-scoped (the record says so for one token).
(c) Whether checklist step 3 (the full checks) ran on `4e8f212`; if not, say so (T-13j's green run on `master`
covers the same code).

### O-19 owner action (session 18, ADR-71 items 5, 11): Project-URL and `uv.lock` (**closed**, session 26: (a) by the cut, the Project-URL goes into T-20; (b) closed in session 20)

#### O-19 (a) restated (session 21): public or private `granddad-wolf/oradmp`

**Session 22: superseded by Q-14** (the owner prefers a one-time cut to a new public repository; the private repository's
history is never made public). The "what a stranger would see" list and the six checks below feed Q-14's scrub list
and checks.

**Needs the project owner.** The owner leans to private through a3 (done: a3 shipped without a Project-URL) and to
decide after the first CI runs on Linux.

**What a stranger would see if it is public** (counted at `ef3305d`, session 21; `git ls-files` lists 865 files):
- the **whole history**: 101 commits, every one with the identity `Maksim Tkachenko <<owner e-mail>>`; the
  pushed tags (`v0.1.0a2`, `v0.1.0a3`); a change made later at `HEAD` does not remove anything from the history;
- the **working method**: `docs/prompts/` (42 session prompts), `.claude/agents/` (5 agent definitions),
  `docs/briefs/` (6), `docs/review/` (7 review reports), `docs/design/` (this file and its companions), `CLAUDE.md`
  (it names the other project `oradump` by `elessarelfstone` and the clean-room rule);
- the **format research**: `docs/formats/` (37 files, the Analyst's specifications and tools), `docs/research/`
  (`sources.md` with each source and its licence, `hypotheses.md`);
- the **corpus**: `corpus/fixtures/` (556 files, 16 MB: synthetic dumps, logs, truth files; ADR-45 item 6),
  `corpus/sql/` (with the passwords of the local test users; session 22: the literal example removed from this
  line), `corpus/scripts/` (Docker setup);
- **local paths**: the user name `max` and folder layout in `docs/design/decisions.md`, `implementer-notes.md`,
  reviews 02 and 03 and some prompts (`<home>/...`, from the S-2 work on home paths in the wheel);
- **not** in the tracked files or the history, by a quick scan in session 21 (`git grep` and `git log --all -S` for the
  Gogs host name; `git log --all -p` for `pypi-...`, `ghp_`, `github_pat_` token forms): the Gogs server's address,
  any token. The owner repeats the scan before opening.

**Consequences.** Public: the macOS and Windows jobs run free on every push (ADR-79); a4 can carry `Project-URL:
Source, https://github.com/granddad-wolf/oradmp` on PyPI (T-16 (j)); the PyPI page of every later version links to it
(a3's page stays without it: an upload cannot be changed). Making it public is reversible on GitHub, but anything
cloned, forked or indexed meanwhile stays out there: treat it as irreversible. Private: dispatched macOS / Windows runs
cost included minutes; no Project-URL; nothing else changes.

**Check before opening** (the owner, in order):
1. Scan the history, not only `HEAD`: `git log --all -p | grep -nE 'pypi-|ghp_|github_pat_|BEGIN .*PRIVATE KEY'` and
   for any host name, account or internal name the owner would not publish; a hit means a fresh repository from a
   snapshot (rejected otherwise: rewriting the history moves the commits that `v0.1.0a2` and `v0.1.0a3` and the PyPI
   records name).
2. The commit e-mail: accept `<owner e-mail>` in public history, or decide before opening (it stays in the
   existing commits either way; GitHub's no-reply address affects only new commits).
3. Confirm that nothing in `corpus/` ever came from real data (only the SQL of `corpus/sql/` produced it) and that the
   test-container passwords are used nowhere else.
4. Read once: `CLAUDE.md`, `docs/prompts/`, `.claude/agents/`, `docs/research/sources.md` (each source's licence
   allows what the documents quote), and the `<home>` mentions; keep them (recommended: they document the clean
   room) or remove them from `HEAD` knowing the history keeps them.
5. GitHub settings: Actions with read-only `GITHUB_TOKEN` by default; approval required for workflows from outside
   contributors' pull requests; branch protection on `master` (no force push); decide on Issues, Discussions and a
   security contact (a `SECURITY.md` would be an Implementer file through a task).
6. Push only release tags (never `git push --tags`: the local tags `analyst-s3`, `fixtures-*` and `v0.1.0a1` stay
   local unless decided).

**Recommendation**: decide **after T-16's first green Linux run** (the owner's lean), and before T-16 ends, so that
T-16 (j) puts the Project-URL into a4; if the decision comes later, the URL waits for a5. Public is recommended if the
six checks pass, for the free runner minutes and the source link.

**Session 20 status.** (a) **Answered for a3**: no Project-URL in a3; it needs a public address, so it comes in the
first release after the owner makes `granddad-wolf/oradmp` public (expected a4, T-16 (j)). **Still open**: public or
private (recommendation unchanged: public, if the owner accepts that the synthetic corpus and the design documents are
visible; it makes the macOS / Windows CI jobs free and gives the Project-URL; ADR-79). (b) **Closed**: `uv.lock` is
untracked and in the root `.gitignore`.

**Session 19 status.** (a) Owner: deferred until the GitHub repository exists, "it goes into 0.1.0a3". That cannot
both hold if a3 goes out before the account exists (ADR-76 item 2), and a private repository gives no URL a PyPI
visitor can open. **Question**: may a3 ship without a Project-URL? **Recommendation: yes**; the URL goes into the first
release after a **public** repository exists (with T-16). If the owner prefers a3 to wait for it, a3 waits for Q-11.
(b) Owner: "`uv.lock` removed and added to `.gitignore`". **Owner says done, not yet visible in the tree**: at HEAD
`2ba62eb` `git ls-files uv.lock` lists `uv.lock` (tracked since `469416d`), the file exists, and the root `.gitignore`
has no `uv.lock` line (`git check-ignore uv.lock` finds no rule). When it lands: `git rm --cached uv.lock` (or delete
it) and the line `uv.lock` in the root `.gitignore` in one owner commit; then nothing in `CLAUDE.md` needs to name it
and ADR-71 item 5's recommendation (the Implementer's row) lapses. Neither archive contains it (review 05), so a3 is
not affected either way.

(a) A URL a PyPI visitor can open for the source (a public mirror, see Q-11), or "none". With a URL, T-13j adds it
as `Project-URL: Source`; with none, a3 ships without it. (b) The root `uv.lock` stays (owner's decision); it needs an
owner in `CLAUDE.md`: recommendation, add "`uv.lock`" after "`Cargo.lock`" in the Implementer's row (it is produced
by `uv` in the repository root, like `Cargo.lock` by cargo; neither archive contains it, review 05).

### O-21 owner action (session 20, ADR-79 item 2): `.github/` in `CLAUDE.md` (**closed**, session 21: applied in `9f1dc4d`)

Before T-16. GitHub Actions reads the CI configuration from `.github/workflows/*.yml`, a folder no row of `CLAUDE.md`
names. It is "the CI configuration" of the Implementer's row. In `CLAUDE.md`, "## Ownership", the Implementer's row
ends

    ..., `.gitattributes`, and the CI configuration in `.github/` (T-16) |

instead of "..., `.gitattributes`, and the CI configuration (T-16) |"; and in "## Repository layout" one line after
`fuzz/`:

    .github/workflows/         CI configuration, GitHub Actions (Implementer, T-16)

Repository settings on GitHub (Actions permissions, branch protection, visibility) stay the owner's; they are not
files.

### O-23 owner action (session 23, ADR-89, ADR-90): the confirming rebuild and the billing figure (**closed**, session 26: (a) done in T-16a2, ADR-98 item 1; (b) lapses, the repository is public, ADR-105 item 10)

(a) The full SHA-256 lines and the SBOM member line of the `wheel` summary of the `d081109` run; then, on the VM (or
T-16a2 (a) does it): with `ORADMP_BUILD_ROOT` unset, `tools/release_build.sh d081109b708b08f08cc6c965994b6c094b649b65`
(root `/var/tmp/oradmp-release`, free today); the wheel's full SHA-256 compared with CI's; then `mv
/var/tmp/oradmp-release /var/tmp/oradmp-release-d081109`. Equal: the build-root hypothesis is confirmed (ADR-90 items
4 to 6). (b) The included Actions minutes from Settings > Billing (the plan's "CI minutes budget"). Neither holds T-16a2
except (a)'s hashes.

### O-22 owner action (session 20, ADR-79 item 3): what T-16 needs on GitHub (**closed**, session 23, ADR-89 item 4)

**Session 22 status: (c) answered: the owner pastes the logs** (the run's address, every job summary, the failed step's
log); the token route is withdrawn (`CLAUDE.md` rule 8; ADR-79 amendment). (a) and (b): the owner confirms Actions are
enabled before T-16a's first push and pushes when its note asks. (d) applies to T-16b (ADR-87). O-22 closes when
T-16a's first run has been pasted.

**Session 21 status: not yet.** T-16 starts after T-17 when (a) to (c) are done (ADR-83). Unchanged below; also: the
token for (c) is typed with `read -rs` into `gh auth login --with-token` and never written into the repository or a
prompt; the CI needs no fixture download (`corpus/fixtures` is in git) and no file of `corpus/work/` (the memory and
throughput tests stay on the VM).

Before T-16 starts (after the a3 tag is pushed):
(a) Actions enabled for `granddad-wolf/oradmp` (repository settings).
(b) `master` pushed to the remote `github` after each commit that should be tested (a push mirror from Gogs, or `git
push github master` by hand); CI also runs on a manual `workflow_dispatch`.
(c) How the Implementer reads the results. **Recommendation**: log `gh` in on the VM with a **fine-grained token
limited to this repository, read-only for Actions and contents**; the Implementer then runs `gh run list` and `gh run
view --log-failed` and cannot push. Alternative: the owner pastes the failed job logs into the session.
(d) While the repository is private, the owner starts the macOS and Windows jobs by hand (`workflow_dispatch`) when
T-16 asks; private minutes are billed (GitHub's Actions billing page: 2,000 included minutes a month on GitHub Free; a
macOS minute about ten times a Linux one). No secret is needed: CI never uploads in phase 1.

### O-17 owner action (session 17, ADR-69 item 3): the `T_BIG` export, for T-15b (**done**, session 19)

**Session 19**: done, but **not from 01b**: `corpus/scripts/make_big.sh` with `corpus/sql/15_big.sql` (`0dac62b`;
5,000,000 rows, 6 columns `ID`, `GRP`, `AMOUNT`, `NAME`, `CREATED`, `NOTE`; distinct ids). Output in the owner's work
folder: `18c/t_big_exp_18c.dmp` (453,304,320 bytes, conventional path), `truth_big.txt`, `SHA256SUMS_big`. The owner's
`truth_big.txt` values agree with the SQL by arithmetic where they can be computed: rows 5,000,000; `sum_id` = 5,000,000
x 5,000,001 / 2 = 12,500,002,500,000; `null_notes` = every tenth id = 500,000; `max_created` = 2020-01-01 + 1,499 days
= 2024-02-08 (1,461 days to 2024-01-01, then 38); `min_created` 2020-01-01; `sum_amount` 24,999,993,314.68 not
recomputed here. The owner reverted any change to the commented 01b block. The text below (01b) is history; O-20 marks
the block.

Not urgent; after run B; gates nothing. In `corpus/sql/01b_optional_tables.sql`, enable the commented `T_BIG` block
(`CREATE TABLE t_big AS SELECT ...` over 1,000 x 5,000 rows = **5,000,000 rows**; ids repeat, which is fine: it is a
volume file, no truth file is needed), in the **18c** database. Export it with `exp`, **conventional path**, table
`T_BIG` only, to `corpus/work/18c/t_big_exp_18c.dmp` (the exact path `t15_memory.rs` and `t15_throughput.rs` look for
under `ORADMP_WORK`); keep the `exp` log beside it. Expected: the log says 5,000,000 rows exported. Size estimate,
unverified: `T_ROWS` is about 100 bytes per row (10 MB / 100,000); `T_BIG`'s four short columns should be well under
that, so a few hundred MB at most. Not committed (`corpus/work/`). Then tell the Implementer the path in the T-15b
prompt. A direct-path copy is not needed.

### O-20 owner action (session 19, ADR-75): mark the 01b `T_BIG` block superseded

Optional, a Human file. In `corpus/sql/01b_optional_tables.sql`, so that nobody enables the old block again: line 9
becomes

    --   T_BIG       superseded: made by corpus/sql/15_big.sql (corpus/scripts/make_big.sh); do not enable below

and the block's first comment line ("-- T_BIG (disabled). Remove the comment markers to create 5,000,000 rows.")
becomes

    -- T_BIG (superseded by 15_big.sql, O-17, session 19; kept as history, do not enable).

Nothing reads the block; the change does not alter any fixture. Recommendation: do it with the O-16 / O-19 (b)
commit. Also seen (reading only): the drop loop at the top of 01b (line 21) lists `'T_BIG'`, so a later run of 01b
in the schema where `15_big.sql` made `T_BIG` drops that table (the dump already made is unaffected; `make_big.sh`
recreates it). If that matters, remove `'T_BIG'` from that list in the same commit; the owner's call.

### O-15 owner action (session 15, ADR-62 item 8): the large exports for T-15

Session 17: the `T_BIG` part is now **O-17** (wanted for T-15b; ADR-65's "1 byte per row" is arithmetic until then).

Session 16: done for `T_ROWS` (T-15 used it). `T_BIG` stays **optional**: with it the memory test resolves a
retention of 1 byte per row (ADR-65 item 6); without it nothing is blocked.


Before T-15. Tell the Implementer (in the T-15 prompt) the path of the `T_ROWS` export (`corpus/sql/01_corpus_tables.sql`,
100,000 rows, 6 columns; produced into `corpus/work/<ver>/` by the owner's scripts) and, if it was made, of `T_BIG`
(`01b_optional_tables.sql`, commented out there). The Implementer's T-15 reads them through `ORADMP_WORK` and commits
nothing from them. Without `T_ROWS`, T-15's throughput and memory items cannot be done. No agent runs the scripts.

### Q-5 owner action (session 11): the text the owner applied (history; applied in `0a0fda5`, with the names of ADR-54)

Session 14 (review 03 N-8): the paths below now carry the names of ADR-54, as `CLAUDE.md` does since O-13.

Session 12: still not applied (`CLAUDE.md` unchanged since `7e7a193`). Item 3's Implementer row now also names
`.gitattributes` (ADR-52) and the CI configuration (ADR-53); nothing else changed.

Checked at `a19d873`: **none** of the Q-5 text is in `CLAUDE.md` or `docs/scope.md` (`git log` shows neither file
changed since `7e7a193`). This text replaces the session 9 and 10 texts below; the roles now match what runs: the
Implementer also writes `python/oradmp/`, `crates/oradmp-py/` and the pytest files; the Reviewer only
`docs/review/`; "Test engineer" and "Python API" go.

1. `CLAUDE.md`, "## Repository layout": replace the lines for `docs/decisions/`, `crates/` and `python/` by

        docs/design/               Architect: architecture, decision records, plan, open questions
        docs/review/               Reviewer: review reports
        crates/                    Rust workspace: oradmp-core, oradmp-py (Implementer)
        python/                    Python package python/oradmp/ (Implementer); python/exp_survey.py is the owner's tool
        tests/                     pytest files and golden files (Implementer; golden files by generators only)

2. `CLAUDE.md`, rule 5: replace "Cross-area requests go in `docs/decisions/` as a short note." by
   "Cross-area requests go in `docs/design/` (`open-questions.md` or `implementer-notes.md`) as a short note."
3. `CLAUDE.md`, "## Ownership": the table replaced as a whole by

        | Agent | Writes to |
        |---|---|
        | Scout | `docs/research/` |
        | Analyst | `docs/formats/`; the `Result` lines in `docs/research/hypotheses.md` |
        | Architect | `docs/design/` (architecture, decisions, plan, open questions, answers in the implementer notes), crate layout |
        | Implementer | `crates/` (`oradmp-core`, `oradmp-py`); `python/oradmp/`; `tests/` (pytest files, `tests/check_dist.py`, and `tests/golden/`, regenerated by the project's generators only, never by hand); its own sections of `docs/design/implementer-notes.md`; the root files `Cargo.toml`, `Cargo.lock`, `pyproject.toml`, `deny.toml`, `README.md`, `CONTRIBUTING.md`, `LICENSE-MIT`, `LICENSE-APACHE`, `.gitattributes`, and the CI configuration (T-16) |
        | Reviewer | `docs/review/` only; reviews code, tests, golden and expected files, edits none of them |
        | Human | `corpus/` (fixtures, `corpus/sql/`, scripts), `python/exp_survey.py`, `CLAUDE.md`, `docs/scope.md`, `docs/prompts/`, `.claude/` |

4. `CLAUDE.md`, new rule 7 under "## Rules for every agent" (unchanged from session 9):

        7. **Golden files and reviews.** Files in `tests/golden/` and `crates/oradmp-core/tests/data/` are produced
           only by their generators, which import the Analyst's tools in `docs/formats/tools/`; never from the
           reader's output and never edited by hand. The Implementer regenerates them; the Reviewer checks them.
           After every group of tasks that adds or changes tests over the corpus, a Reviewer session runs before
           the next group starts.

5. `CLAUDE.md`, "## Definition of done", Python line: "Python: `ruff check`, `ruff format --check`, `mypy`, `pytest`
   pass (venv `.venv` at the repository root)."
6. `docs/scope.md`, table: the "Character set" row becomes
   `| Character sets | US7ASCII (1), CL8MSWIN1251 (171), WE8MSWIN1252 (178), AL32UTF8 (873); national AL16UTF16 (2000) | the sets of the fixtures (architecture 6.2); any other set: W-CHARSET-UNSUPPORTED |`;
   a new row `| Time zones | TIMESTAMP WITH TIME ZONE as the UTC instant (the zone is not delivered); WITH LOCAL TIME ZONE "as stored", with a warning | architecture 5.1, ADR-7 |`;
   and "LONG/LONG RAW" leaves "Out of scope for now" (both are read, ADR-15).
7. `.claude/agents/reviewer.md`: the sentence "This is the role `CLAUDE.md` calls 'Test engineer'." can go.

### Q-5 owner action (session 9, ADR-42): replacement text for `CLAUDE.md` (history; superseded by session 11)

The ownership table under "## Ownership", replaced as a whole (the Architect row also names the directory the
Architect has written since session 1; the "Human" row is new; the owner may drop either change):

    | Agent | Writes to |
    |---|---|
    | Scout | `docs/research/` |
    | Analyst | `docs/formats/`; the `Result` lines in `docs/research/hypotheses.md` |
    | Architect | `docs/design/` (architecture, decisions, plan, open questions; `docs/decisions/` is unused), crate layout |
    | Implementer | `crates/`; `tests/golden/` (regenerated by the project's generators only, never by hand) |
    | Reviewer | `docs/review/` only; reviews tests, golden and expected files, edits none of them |
    | Python API | `python/`; the pytest files under `tests/` |
    | Human | `corpus/` (fixtures, `corpus/sql/`, scripts), `CLAUDE.md`, `docs/scope.md`, `docs/prompts/` |

New rule 7 under "## Rules for every agent":

    7. **Golden files and reviews.** Files in `tests/golden/` and `crates/oradump-core/tests/data/` are produced
       only by their generators, which import the Analyst's tools in `docs/formats/tools/`; never from the
       reader's output and never edited by hand. The Implementer regenerates them; the Reviewer checks them.
       After every group of tasks that adds or changes tests over the corpus, a Reviewer session runs before
       the next group starts.

Session 10 (ADR-45): the Implementer row then reads (root files added; nobody owns them today):

    | Implementer | `crates/`; `tests/golden/` (regenerated by the project's generators only, never by hand); the root files `Cargo.toml`, `pyproject.toml`, `deny.toml`, `README.md`, `CONTRIBUTING.md`, `LICENSE-MIT`, `LICENSE-APACHE` |

Optional, same edit: `.claude/agents/reviewer.md` says "This is the role `CLAUDE.md` calls 'Test engineer'"; after
the edit that sentence can go. Recommendation: accept all three. New SQL for fixtures keeps its route: the
Analyst writes it into `docs/formats/requests.md`, the owner puts it into `corpus/sql/` and runs it.

## (b) Requests for the Analyst

Style of `docs/formats/requests.md`; numbers continue after R-023 (session 4: after R-040, so the new
ones are R-041 to R-043). They are **proposed** here: the Architect does not edit `docs/formats/`. None duplicates R-011, R-017 to R-023. Order = priority. The SQL
was not run. `<v>` is `18c` and `11g`.

**State after Analyst session 9** (`requests.md`, Architect session 4): R-031 (except the 100,000-byte
value, refused by Oracle), R-032 to R-037 generated and used (rules R2, R8, T3, T9, Y9, Y1, C5, C6, C8,
L4, L5, Y10, Y11, T10, E2, E2a). New from the Analyst: **R-040** (LONG above 32,767 / 65,535 bytes by
SQL*Loader or python-oracledb), open. R-038 (session 3, LOB in the CL8MSWIN1251 database): not
generated; the owner is preparing the fixture; `requests.md` mentions it only in the R-035 note. R-039
(session 3, NOT NULL intervals): not generated; not in `requests.md` (note A-21). What the open ones
block, **in order of value for the alpha** (Q-4 decides the conditional ones):

| order | request | blocks the alpha? | why |
|---|---|---|---|
| 1 | ~~**R-038**~~ (session 8: generated and used, ADR-25 session 8) LOB in the CL8MSWIN1251 database (long Cyrillic CLOB, NCLOB, BLOB) | **yes, if** Q-4 shows a single-byte database with CLOB / NCLOB columns (probable for the owner's Cyrillic data) | until then every long-form CLOB value and every NCLOB value of such a database is refused (`E-LOB-UNCONFIRMED`, ADR-25): no wrong text, but no text either for the values most likely to matter |
| 2 | ~~**R-040**~~ (session 24: generated and used; ADR-95; remainder in R-048) LONG of 32,767 / 32,768 / 65,519 / 65,520 / 65,535 / 65,536 / 100,000 bytes | **yes, if** Q-4 shows LONG / LONG RAW columns written on the **conventional** path with values of 32,768 bytes or more; otherwise **no** | such a value stops the file at that table (`UnsupportedLayout`, ADR-12); on the direct path the chain reads any length (with `W-LONG-LENGTH` above 32,000) and R-040 only confirms it. It does **not** block the alpha in general: a dump with no such value is unaffected, and the stop names the table and offset |
| 3 | R-042 (new) partitioned table with LOB columns; NCLOB in the long form | yes, if Q-4 shows partitioned tables with LOB columns | the combination of [T10] and [L1 to L4] is designed (each part has its own m and entries) but not in the fixtures; a wrong guess is a `Framing` stop, not a misreading |
| 4 | R-041 (new) single-column LONG / LONG RAW tables on the direct path | no | only the [E2a] case (ADR-22); `W-UNTESTED-LAYOUT` marks it |
| 5 | R-043 (new) LONG in the CL8MSWIN1251 database | no | `W-LONG-CHARSET` on non-ASCII LONG of a single-byte database (ADR-15); the value is delivered by the same rule as VARCHAR2 |
| 6 | R-039 NOT NULL interval text | no | metadata only: column fallback with `W-DDL-FALLBACK` |
| 7 | R-045 (session 6) Oracle refuses partitioned tables with LONG / LONG RAW | no | records why no fixture can show a partitioned LONG table; run with R-042 (R-044 stays after R-043, low) |

Unchanged: R-022 (an `exp` 10.2 client) remains the largest gap; only the owner's survey stands in for
it (arch 8.5).

**State after Architect session 5**: R-038 is being generated by the owner (ADR-25 unchanged until it
arrives); R-040 to R-043 as below; new **R-044** (low priority, after R-043 in the order table: it blocks
nothing, ADR-28 keeps the error either way).

**State after Architect session 6**: no fixture generated since R-040 / R-044 beyond what the owner
reports; R-038 being prepared; R-039 to R-044 unchanged; new **R-045** below (low; with R-042's run).

**State after Architect session 10**: no new request (next free number R-047). R-044 would replace the patched
TIMESTAMP copies of T-11b (ADR-43 item 2); its priority stays low. Owner actions: R-040 (ready; the Analyst writes up
the run), R-046 (low). R-045 still to be recorded by the Analyst. R-039, R-041 to R-044 unchanged.

**State after Architect session 9**: no new request (next free number R-047). Owner actions: R-040 (the owner
reports `sqlldr` in the 18c container: ready to run; the Analyst writes up the run), R-046 (low; `r_lob3` in
`ora11ru`). R-045 still to be recorded by the Analyst. R-039, R-041 to R-044 unchanged.

**State after Architect session 8**: R-038 generated (Analyst session 10, two files and `truth_lob_ru.txt`) and
used: ADR-25 item 2 lifted for database set 171, task T-6M. R-046 (Analyst, proposed, low): where the short
form ends for CLOB / NCLOB in that database; a reader does not need it (ADR-37), only synthetic tests do.
R-045: `truth_r045.txt` delivered by the owner (18c): ORA-14085 for LONG, for LONG RAW and for `ALTER TABLE ...
ADD (l LONG)` on a partitioned table, `Table created.` for the partitioned table itself; waiting for the
Analyst's record; nothing in the design changes (ADR-32 item 4 keeps the unit-tested exemption). R-040 waits
for the owner (is `sqlldr` in the 18c container). R-039, R-041 to R-044 unchanged. Next free number: R-047.

**State after Architect session 7**: R-038 (the fixture script exists; waits for the owner to run it) and
R-045 pending; R-039 to R-044 unchanged; no new request (next number **R-046**). When R-038 arrives, the
design changes in three places only: `lob_text_kind`, the unconfirmed check of the LOB sink, and the tests
in `t6l_ru.rs` (ADR-34 item 9); its values join T-10 step (2).

**State after Architect session 22**: one new request, **R-047** (below; TIMESTAMP precisions other than 0, 6, 9),
from review 06b SF-1 (ADR-86 item 1). **R-017** (open in `requests.md`) gains a user: it is the only route to an 18c
file with E1 after a direct-path flag, which T-17b's generator writes as `Unconfirmed::E1AfterFlag1In18c`; both are
small 18c runs and can go together. R-040 (the owner's assistant writes the runner) and R-046 (low) pending, unchanged.
Next free number after R-047: **R-048**.

**State after Architect session 24**: R-017, R-040, R-047 generated and used (Analyst 17; ADR-94). The Analyst wrote
R-048 (conventional LONG of 65,521 to 65,535 bytes; LONG RAW above 32,000) and R-049 (11g empty table without a
segment) in `requests.md`, both open, low. New here: **R-050** (low). T-19 waits for none of them (ADR-97). Next free
number after R-050: **R-051**.

### Inputs for Analyst session 21 (session 25; no fixture needed)

Documents only:
1. `docs/formats/README.md`: one line that `python/exp_survey.py` (the owner's tool) stops on type codes 8 and 24 (LONG,
   LONG RAW), and that LONG lengths on real dumps come from `oradmp survey` (counts only; ADR-101 item 4).
2. R-052: one line, dated session 25: the design reads 0xfff2 to 0xfff4 as lengths with `W-UNTESTED-LAYOUT` and stops on
   `fb ff` / `fc ff` (ADR-99, ADR-101); the request runs only if Q-4's survey shows either.
3. Unchanged: R-046 (low).

**State after Architect session 25**: R-048 to R-051 generated and used (Analyst 19 and 20); R-049 negative; R-050 closed
(ADR-100). Open from the fixtures' side for phase 1 LONG: R-052 only (proposed, very low). Next free number **R-053**.

### Inputs for Analyst session 18 (session 24; no fixture needed; **done**, Analyst 18: R-050 in `requests.md`)

Documents only, for `docs/formats/requests.md`:
1. Copy **R-050** below into `requests.md` (status "proposed"; the index row; next free number R-051).
2. R-048: one line, dated session 24: the design reads the conventional LONG forms of R-040 and stops on a u16 0xfff1 to
   0xfffc and on a conventional LONG RAW u16 from 0x8000 (ADR-95); the LONG RAW half is the cheaper half and goes first if
   Q-4 shows LONG RAW columns; a value of 65,533 or 65,534 bytes written as one field would collide with `fd ff` / `fe ff`
   (risk 29), so the request's sizes 65,533 and 65,534 are the ones that matter most.
3. Unchanged: R-046 (low), R-049 (low).

### R-050 TIMESTAMP WITH LOCAL TIME ZONE: descriptor length per precision, zero fraction, rounding (session 24; 18c and 11g; **closed**, session 25: generated and used, every expectation held, ADR-100)
- Why: `exp.md` 6.5 ("TIMESTAMP WITH LOCAL TIME ZONE"), [T5], [Y8]. The fixtures show one TSLTZ column, `TIMESTAMP(6) WITH
  LOCAL TIME ZONE`, descriptor `e7 00 0b 00` (`t_tstz_exp_<v>`@0x229a), values of 11 bytes; "a TSLTZ value with a zero
  fraction (7 bytes?)" is unknown. The reader expects length 7 for f = 0 and 11 otherwise (the TIMESTAMP rule of [T5]
  v3.7, assumed for 231); another length gives `W-DDL-FALLBACK` and the descriptor's type, not a wrong value (ADR-96).
- SQL (as `CORPUS`, in the 18c and the 11g database; the session time zone fixed so that the stored instant equals the
  literal; `dbtimezone` is +00:00 in both, `truth_env.txt`):

      ALTER SESSION SET TIME_ZONE = '+00:00';
      CREATE TABLE t_tsltz2 (id NUMBER(2), l0 TIMESTAMP(0) WITH LOCAL TIME ZONE, l3 TIMESTAMP(3) WITH LOCAL TIME ZONE,
                             l9 TIMESTAMP(9) WITH LOCAL TIME ZONE);
      INSERT INTO t_tsltz2 VALUES (1, TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05',
                                      TIMESTAMP '2020-01-02 03:04:05');
      INSERT INTO t_tsltz2 VALUES (2, TIMESTAMP '2020-01-02 03:04:05.123456789', TIMESTAMP '2020-01-02 03:04:05.123456789',
                                      TIMESTAMP '2020-01-02 03:04:05.123456789');
      INSERT INTO t_tsltz2 VALUES (3, TIMESTAMP '2020-01-02 03:04:05.6', TIMESTAMP '2020-01-02 03:04:05.6',
                                      TIMESTAMP '2020-01-02 03:04:05.6');
      INSERT INTO t_tsltz2 VALUES (4, NULL, NULL, NULL);
      COMMIT;
      -- truth_tsltz2.txt
      SELECT dbtimezone || '|' || sessiontimezone FROM dual;
      SELECT column_name || '|' || data_type || '|' || data_length || '|' || data_scale
        FROM user_tab_columns WHERE table_name = 'T_TSLTZ2' ORDER BY column_id;
      SELECT id || '|' || DUMP(l0) || '|' || DUMP(l3) || '|' || DUMP(l9) FROM t_tsltz2 ORDER BY id;
      SELECT id || '|' || TO_CHAR(l0, 'YYYY-MM-DD HH24:MI:SS.FF9') || '|' || TO_CHAR(l3, 'YYYY-MM-DD HH24:MI:SS.FF9')
             || '|' || TO_CHAR(l9, 'YYYY-MM-DD HH24:MI:SS.FF9') FROM t_tsltz2 ORDER BY id;

- Exports: `tables=(t_tsltz2)`, conventional and `direct=y`, `statistics=none`, both versions.
- Expected (hypothesis; the arithmetic of [Y2], [Y5], [Y8]):
  - Descriptors: `e7 00 07 00` for L0 (type 231, length 7, as TIMESTAMP(0)); `e7 00 0b 00` for L3 and L9; `data_length` 7,
    11, 11.
  - Row 1, each column: `07 00` + `78 78 01 02 04 05 06` (century 20 + 100, year 20 + 100, month 1, day 2, hour 3 + 1,
    minute 4 + 1, second 5 + 1).
  - Row 2: L0 rounds .123456789 to 0 seconds: `07 00` + `78 78 01 02 04 05 06`; L3 `0b 00` + the same 7 bytes + 123,000,000
    = `07 54 d4 c0`; L9 `0b 00` + the 7 bytes + 123,456,789 = `07 5b cd 15`.
  - Row 3: L0 rounds .6 up: `07 00` + `78 78 01 02 04 05 07` (second 6 + 1; truncation would give `... 06`); L3 600,000,000
    = `23 c3 46 00`; L9 the same `23 c3 46 00`.
  - Row 4: `fe ff` x 3. Both paths give the same bytes [R7].
  - Arrow (arch 5.1, ADR-7): Timestamp(Microsecond) for L0, L3, Timestamp(Nanosecond) for L9, no zone, `W-TSLTZ` per column.
- File names wanted: `tsltz2_exp_<v>.dmp`, `tsltz2_exp_direct_<v>.dmp`, logs, `truth_tsltz2.txt`.
- Priority: **low**, unless Q-4 shows type 231 in the real tables; small; run with R-048 in the same containers.

### Inputs for Analyst session 16 (session 22)

Documents only, for `docs/formats/requests.md`:
1. Copy **R-047** below into `requests.md` (status "proposed"; the index row; next free number R-048).
2. **R-017**: one line under it, dated session 22: a second reason to run it: the synthetic generator of the tests
   writes E1 after a direct-path flag in an 18c file only as an unconfirmed combination (ADR-86); a T_SEG0 / T_SEG1
   with `ff ff` would confirm the bytes on 18c, a T_SEG0 / T_SEG1 without it would show the generator's case cannot occur
   on 18c (it stays a test of the "never by version" rule).
3. Unchanged: R-040 (no Analyst action), R-046 (low).

### R-047 TIMESTAMP(f) for f = 1 to 5, 7 and 8: descriptor length and value bytes (session 22; 18c and 11g)
- **Status (session 24): generated and used** (`tsp_exp(_direct)_<v>`, `truth_tsp.txt`; Analyst 17): every expectation held, the fractions are rounded; closed (ADR-94 item 1).
- Why: [T5] gives TIMESTAMP the code 180 and [Y5] the value bytes (7 bytes in the DATE layout when the fraction is zero,
  else 11 with a u32 BE nanosecond count). The descriptor's maximum length is in the fixtures only for TIMESTAMP(0) (7),
  TIMESTAMP(6) and TIMESTAMP(9) (11) (review 06b SF-1: every `CREATE TABLE` line and its descriptor counted). The reader
  expects 7 for f = 0 and 11 otherwise; a real column with another length gets the column fallback of arch 5.5 step 4
  (`W-DDL-FALLBACK`, values still decoded, unit Microsecond), not a wrong value. The synthetic generator drew f from 0 to
  9 with the reader's length and is now limited to {0, 6, 9} until this request is answered. TIMESTAMP(3) is common in
  application schemas (Q-4 asks whether the owner's tables have it).
- SQL (as `CORPUS`, in the 18c and the 11g database):

      CREATE TABLE t_tsp (id NUMBER(2), t1 TIMESTAMP(1), t2 TIMESTAMP(2), t3 TIMESTAMP(3), t4 TIMESTAMP(4),
                          t5 TIMESTAMP(5), t7 TIMESTAMP(7), t8 TIMESTAMP(8));
      -- row 1: fraction zero everywhere
      INSERT INTO t_tsp VALUES (1, TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05',
        TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05',
        TIMESTAMP '2020-01-02 03:04:05', TIMESTAMP '2020-01-02 03:04:05');
      -- row 2: the largest fraction each precision holds
      INSERT INTO t_tsp VALUES (2, TIMESTAMP '2020-01-02 03:04:05.9', TIMESTAMP '2020-01-02 03:04:05.99',
        TIMESTAMP '2020-01-02 03:04:05.999', TIMESTAMP '2020-01-02 03:04:05.9999',
        TIMESTAMP '2020-01-02 03:04:05.99999', TIMESTAMP '2020-01-02 03:04:05.9999999',
        TIMESTAMP '2020-01-02 03:04:05.99999999');
      -- row 3: one literal with 9 digits in every column (Oracle rounds to the column's precision)
      INSERT INTO t_tsp VALUES (3, TIMESTAMP '2020-01-02 03:04:05.123456789', TIMESTAMP '2020-01-02 03:04:05.123456789',
        TIMESTAMP '2020-01-02 03:04:05.123456789', TIMESTAMP '2020-01-02 03:04:05.123456789',
        TIMESTAMP '2020-01-02 03:04:05.123456789', TIMESTAMP '2020-01-02 03:04:05.123456789',
        TIMESTAMP '2020-01-02 03:04:05.123456789');
      -- row 4: NULL everywhere
      INSERT INTO t_tsp VALUES (4, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
      COMMIT;
      -- truth_tsp.txt
      SELECT column_name || '|' || data_type || '|' || data_length || '|' || data_scale
        FROM user_tab_columns WHERE table_name = 'T_TSP' ORDER BY column_id;
      SELECT id || '|' || DUMP(t1) || '|' || DUMP(t2) || '|' || DUMP(t3) || '|' || DUMP(t4) || '|' || DUMP(t5)
             || '|' || DUMP(t7) || '|' || DUMP(t8) FROM t_tsp ORDER BY id;
      SELECT id || '|' || TO_CHAR(t1, 'YYYY-MM-DD HH24:MI:SS.FF9') || '|' || TO_CHAR(t3, 'YYYY-MM-DD HH24:MI:SS.FF9')
             || '|' || TO_CHAR(t8, 'YYYY-MM-DD HH24:MI:SS.FF9') FROM t_tsp ORDER BY id;

- Exports: `tables=(t_tsp)`, conventional and `direct=y`, both versions.
- Expected (hypothesis; the arithmetic of [Y2] and [Y5]):
  - Descriptors: `02 00 16 00` for ID; `b4 00 0b 00` (type 180, length 11) for each of the seven TIMESTAMP columns
    (hypothesis "11 for every f > 0"; a 7 or any other length refutes it for that f).
  - Row 1, every TIMESTAMP column: field `07 00` + `78 78 01 02 04 05 06` (century 20 + 100 = 0x78, year 20 + 100 = 0x78,
    month 1, day 2, hour 3 + 1, minute 4 + 1, second 5 + 1); `DUMP` = `Typ=180 Len=7: 120,120,1,2,4,5,6`.
  - Row 2: field `0b 00` + `78 78 01 02 04 05 06` + the nanoseconds as u32 BE: t1 900,000,000 = `35 a4 e9 00`; t2
    990,000,000 = `3b 02 33 80`; t3 999,000,000 = `3b 8b 87 c0`; t4 999,900,000 = `3b 99 43 60`; t5 999,990,000 =
    `3b 9a a2 f0`; t7 999,999,900 = `3b 9a c9 9c`; t8 999,999,990 = `3b 9a c9 f6` (each = 1,000,000,000 = `3b 9a ca 00`
    minus 10^(9-f)). `DUMP` of t1: `Typ=180 Len=11: 120,120,1,2,4,5,6,53,164,233,0`.
  - Row 3 (rounded, hypothesis): t1 100,000,000 = `05 f5 e1 00`; t2 120,000,000 = `07 27 0e 00`; t3 123,000,000 =
    `07 54 d4 c0`; t4 123,500,000 = `07 5c 75 e0`; t5 123,460,000 = `07 5b d9 a0`; t7 123,456,800 = `07 5b cd 20`; t8
    123,456,790 = `07 5b cd 16` (123,456,789 = `07 5b cd 15`). If Oracle truncates instead of rounding: t4 123,400,000,
    t5 123,450,000, t7 123,456,700, t8 123,456,780; the truth file decides, the reader decodes either.
  - Row 4: `fe ff` in each of the seven fields.
  - Both paths give the same bytes [R7]; Arrow (arch 5.1): Timestamp(Microsecond) for t1 to t5, Timestamp(Nanosecond)
    for t7 and t8, values as `TO_CHAR ... FF9` truncated to the unit (t1 to t5 hold no digits below microseconds).
- File names wanted: `tsp_exp_<v>.dmp`, `tsp_exp_direct_<v>.dmp`, logs, `truth_tsp.txt`.
- Priority: low unless Q-4 shows TIMESTAMP(f) with f outside {0, 6, 9} in the real tables; small (one table, two
  containers), so it can run with R-017.

### Inputs for Analyst session 15 (session 21; no fixture needed)

Documents only, for `docs/formats/requests.md`:
1. **R-003 wording** (the owner's request). The bytes, checked by the Architect with `xxd -s 0x24f0 -l 0x50
   corpus/fixtures/18c/lob_exp_18c.dmp`: @0x2502 `02 00` is k = 2 and closes **row 1** (its two LOB values follow at
   0x2504 and 0x2518); row 2 starts @0x2524 (`02 00 c1 03 fe ff fe ff`) and its k = 0 is the `00 00` @0x252c, which
   closes **row 2**; row 3 starts @0x252e. The status line (session 14) says this. The kept "Reasoning (session 13)"
   cites only @0x2502 `02 00` as its example of "the `00 00` half", so a reader can take 0x2502 for the `00 00`: please
   name `00 00` @0x252c (row 2, k = 0) as the `00 00` example next to `02 00` @0x2502 (row 1, k = 2), dated session 15.
   (The owner's summary "the `00 00` at 0x2502 closes row 2" does not match the bytes either: at 0x2502 is `02 00`.)
2. **R-040**: no Analyst action; the owner's assistant writes the runner. Status stays as it is.
3. Still pending, unchanged: **R-046** (low). Next free request number: **R-047**.

### Inputs for Analyst session 14 (session 20; no fixture needed; **done**, `27c0012`: R-003 rejected)

Documents only, for `docs/formats/requests.md` (the Analyst's file):
1. **R-003**: the owner agrees with the Analyst: **rejected (not needed)**. The `METRICET` half is answered (`exp.md`
   7.1) and the `00 00` half in substance by the LOB fixtures (rule L3: the u16 after a row's last field belongs to
   that row), as the session 13 note says. Set the status and date the decision.
2. **R-040**: write the runner for the `sqlldr` route (status `ready`; SQL*Loader 18.4.0.0.0 exists in the 18c
   container), so that the owner can run it: the control file, the data file recipe for LONG values of 32,767 /
   32,768 / 65,519 / 65,520 / 65,535 / 65,536 / 100,000 bytes, the `exp` commands (both paths), the expected file names
   and the truth file with the lengths; the arithmetic of each expected length. The 11g container is still unchecked.
3. Still pending, unchanged: **R-046** (low). Next free request number: **R-047**.

### Inputs for Analyst session 13 (session 19; no fixture needed)

Analyst session 12 is done (`1c17633`: the `datapump.md` row and the `ready` status in `docs/formats/README.md`).
For session 13, documents only:
1. **R-001 to R-007 still say `Status: open`** although the Analyst's session 2 notes answer most of them. The
   owner decides which to close; the fixtures, all in `corpus/fixtures/18c/` (listed by `ls`, session 19):
   - R-005 (other export character set): `t_frame_exp_w1252_18c.dmp`, `t_frame_exp_us7ascii_18c.dmp`,
     `t_frame_exp_direct_w1252_18c.dmp`, `t_frame_exp_direct_us7ascii_18c.dmp` and logs (used in `exp.md` 2.1):
     suggest `generated`.
   - R-006 (rows beyond one 8192-byte unit): `t_probe6_exp_18c.dmp`, `t_probe6_exp_direct_18c.dmp`,
     `truth_probe6.txt` (used in `exp.md` 1.2): suggest `generated`.
   - R-007 (other RECORDLENGTH): `t_frame_exp_rl4096_18c.dmp`, `t_frame_exp_rl16384_18c.dmp` and logs (`exp.md`
     1.1): suggest `generated`.
   - R-004 (single NUMBER column): answered by `t_num_*`, `t_nullmix_*`, `t_date_*` (`exp.md` 4.1, 5.3, 6.1); no
     `t_probe4` file: suggest `rejected` (no longer needed), as the Analyst's note says.
   - R-001, R-002, R-003: no `t_probe1*` / `t_probe2*` / `t_probe3*` file. Partly answered, by the Analyst's session 2
     notes: R-002's declared-length half by the existing VARCHAR2(10 / 20 / 100 / 400) descriptors (`exp.md` 4.1);
     R-003's `METRICET` half by `t_empty`, `multi_framing`, `multi_types`, `ddl` (`exp.md` 7.1); R-001 in part by
     `t_len`, `t_num`, `t_text`. The rest (R-001: whether anything outside the row data depends on a value;
     R-002: the column-name half; R-003: the `00 00` reading) is still open, low priority: the owner keeps them
     `open` or rejects them; the design needs none.
2. **R-011's notes**: the session 4 note (`requests.md` around line 228) says the status was `generated` but no
   `t_probe11*` file existed. Both are now history: the status line is `superseded by R-034`, and `t_probe11_*`
   files and `truth_probe11.txt` **do exist** today in `18c/` and `11g/` (made under R-034, same table name). A dated
   note that the earlier notes are history would stop a reader from concluding that the files are missing.
3. `docs/formats/tools/.ruff_cache`: checked, **no action needed**: not tracked (`git ls-files` lists none of it), and
   ignored through the `.gitignore` that ruff writes inside it (`*`; `git status --ignored` shows it `!!`). The same
   holds for `__pycache__/` (root `.gitignore`).
4. Still pending, unchanged: **R-040** (the `sqlldr` runner; the owner has not given its text) and **R-046** (low).
   Next free request number: **R-047**.

### Inputs for Analyst session 12 (session 18; no fixture needed; **done**, `1c17633`)

1. `docs/formats/README.md`, "Files": the row `datapump.md` names a file that does not exist (`ls docs/formats/`:
   `exp.md`, `exp_reader_rules.md`, `README.md`, `requests.md`, `tools/`). Remove the row, or mark it "phase 2, not
   written yet".
2. `docs/formats/README.md`, "requests.md format": the status list (`open | proposed | generated | rejected |
   superseded | reserved`) has no `ready`, which `requests.md` uses for R-040 ("ready, not yet run"). Add `ready`
   with its meaning, or change R-040's status to a listed value.
3. Still pending, unchanged: **R-040** (the `sqlldr` runner; the owner has not yet given its text) and **R-046** (low).
   Next free request number: **R-047**.

### Inputs for Analyst session 11 (session 14; no fixture needed)

Session 16: still pending (review 04 N-10 and section 8: rule T9 at `exp_reader_rules.md:226` still reads 382 /
1885; X2 unchanged). The list below stands as it is. **The owner runs Analyst session 11 before release run B**
(checklist step 1, run B gate); it builds nothing and can run beside T-13i.

Review 03 (N-10, section 10) re-derived the numbers again with the Analyst's tools; nothing about the formats is new.
For `docs/formats/exp_reader_rules.md` and `docs/formats/requests.md` (the Analyst's files; the Architect does not
edit them):
1. **T9** (rules line 226): "382 lines, 1885 column definitions, `tools/collist.py`" should read **386 lines, 1,907
   column definitions**, PROBLEMS 0 (`collist.py` over the 201 files; review 01 section 3, review 02 section 6, review
   03 sections 3 and 10; the 4 `... WITH LOCAL TIME ZONE` definitions equal the 4 T_TSTZ sections of
   `exp_catalog.tsv`). The rules file's own header (line 16) already says 386 sections.
2. **X2** (rules line 251): the evidence names `ru_exp_us7ascii_11g`, `ru_exp_w1252_11g` and `EXP-00056` only; the
   third failed export, `ru_exp_direct_us7ascii_11g` (its `.log`: `EXP-00008` / `ORA-00604` / `ORA-00942`; `exp.md`
   14.3), belongs in the evidence column. The reader's `W-POSSIBLY-INCOMPLETE` fires on all three (ADR-39).
3. **R-045**: record the owner's `truth_r045.txt` (18c: ORA-14085 for LONG and LONG RAW in a partitioned table) as
   generated and used in `requests.md` (session 8 state above).
4. **R-040**: write up the run if the owner has run it (`sqlldr` in the 18c container).
No request is added; the next free number stays **R-047**.

### R-045 Oracle refuses a partitioned table with a LONG or LONG RAW column (session 6; 18c)
- Why: ADR-31 / ADR-32 keep a per-part exemption for the direct-path LONG-last order [T3] and T-3b tests it
  only on synthetic parts, because no fixture has a partitioned table with a LONG column. Oracle's error
  documentation states ORA-14085 "partitioned table cannot have column with LONG datatype"; for LONG RAW
  the design has only an Oracle9i Concepts sentence seen in a search result. If both statements hold on
  18c, the gap cannot be closed by a fixture and is closed by this record. No dump is produced.
- SQL (as `CORPUS`, 18c, in the same session as R-042; each statement is expected to **fail**):

      CREATE TABLE t_plong (id NUMBER(3), l LONG)
        PARTITION BY RANGE (id) (PARTITION p1 VALUES LESS THAN (10),
                                 PARTITION p2 VALUES LESS THAN (MAXVALUE));
      CREATE TABLE t_plongraw (id NUMBER(3), lr LONG RAW)
        PARTITION BY RANGE (id) (PARTITION p1 VALUES LESS THAN (10),
                                 PARTITION p2 VALUES LESS THAN (MAXVALUE));
      CREATE TABLE t_plong2 (id NUMBER(3)) PARTITION BY RANGE (id)
        (PARTITION p1 VALUES LESS THAN (10), PARTITION p2 VALUES LESS THAN (MAXVALUE));
      ALTER TABLE t_plong2 ADD (l LONG);
      DROP TABLE t_plong2 PURGE;

- Expected: statements 1 (`t_plong`), 2 (`t_plongraw`) and 4 (`ALTER ... ADD (l LONG)`) fail; 3 and 5
  succeed. By the documentation the error of 1 and 4 is ORA-14085; the code for 2 (LONG RAW) is what the
  record will state. Arithmetic: none (no table with a LONG column and more
  than one part can exist, so the number of such parts in any dump of 18c is 0).
- File wanted: the SQL*Plus spool of the five statements with their messages
  (`corpus/fixtures/18c/plong_refused.txt`, or a line in `requests.md`). Status: proposed. Priority:
  **low**; it blocks nothing (the exemption stays and is unit-tested either way).

### R-044 years below 1, dates around 1582, TIME ZONE offsets at the extremes (session 5; 18c and 11g)
- Why: ADR-27 items 3 and 6, ADR-28 (b). [Y2] states the BC byte pairs from two values (-0100, -4712);
  year -1 and a year of century 0 BC (-50, by the formula `64 32`) are not in the fixtures, and the
  decoder's pair rule (ADR-27 item 6) rests on the formula there. The reader keeps years below 1 an error
  because the **year numbering** is unknown (does Oracle's -0001 directly precede 0001?); and the README
  needs to know whether Oracle counts dates before 1582-10-15 in another calendar. [Y7] has three offsets;
  -12:00 and +14:00 put the formula `hour + 20, minute + 60` at its ends.
- SQL (as `CORPUS`, AL32UTF8 database):

      CREATE TABLE t_bc (id NUMBER(2), d DATE, ts TIMESTAMP(0), tz TIMESTAMP(0) WITH TIME ZONE);
      INSERT INTO t_bc VALUES (1, TO_DATE('-0001-12-31 23:59:59', 'SYYYY-MM-DD HH24:MI:SS'),
        TO_TIMESTAMP('-0001-12-31 23:59:59', 'SYYYY-MM-DD HH24:MI:SS'), NULL);
      INSERT INTO t_bc VALUES (2, TO_DATE('0001-01-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'),
        TO_TIMESTAMP('0001-01-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL);
      INSERT INTO t_bc VALUES (3, TO_DATE('-0050-07-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'),
        TO_TIMESTAMP('-0050-07-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL);
      INSERT INTO t_bc VALUES (4, TO_DATE('-0001-01-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL, NULL);
      INSERT INTO t_bc VALUES (5, TO_DATE('1582-10-04 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL, NULL);
      INSERT INTO t_bc VALUES (6, TO_DATE('1582-10-15 00:00:00', 'SYYYY-MM-DD HH24:MI:SS'), NULL, NULL);
      INSERT INTO t_bc VALUES (7, NULL, NULL, TIMESTAMP '2020-06-01 12:00:00 -12:00');
      INSERT INTO t_bc VALUES (8, NULL, NULL, TIMESTAMP '2020-06-01 12:00:00 +14:00');
      COMMIT;

- Truth (`truth_bc.txt`): per row `id|TO_CHAR(d, 'SYYYY-MM-DD HH24:MI:SS')|DUMP(d)|TO_CHAR(d, 'J')|DUMP(ts)|
  TO_CHAR(tz, 'SYYYY-MM-DD HH24:MI:SS TZH:TZM')|DUMP(tz)`; and three lines from `dual`:
  `TO_CHAR(TO_DATE('-0001-12-31', 'SYYYY-MM-DD') + 1, 'SYYYY-MM-DD')`,
  `TO_DATE('0001-01-01', 'SYYYY-MM-DD') - TO_DATE('-0001-01-01', 'SYYYY-MM-DD')`,
  `TO_DATE('1582-10-15', 'SYYYY-MM-DD') - TO_DATE('1582-10-04', 'SYYYY-MM-DD')`.
- Expected by [Y2] (to be confirmed by `DUMP`): row 1 `64,63,c,1f,18,3c,3c` (century 0: 100 - 0 = 100;
  year 1: 100 - 1 = 99); row 2 `64,65,1,1,1,1,1` (as `truth_date` row 4); row 3 `64,32,7,1,1,1,1` (100,
  100 - 50 = 50); row 4 `64,63,1,1,1,1,1`; row 5 `73,b6,a,4,1,1,1` (15 + 100 = 115, 82 + 100 = 182); row 6
  `73,b6,a,f,1,1,1`. The TIMESTAMP(0) bytes equal the DATE bytes [Y5]. TSTZ by [Y7], UTC fields then zone
  bytes: row 7 is 2020-06-02 00:00:00 UTC, `78,78,6,2,1,1,1` and zone `8,3c` (-12 + 20 = 8, 0 + 60);
  row 8 is 2020-05-31 22:00:00 UTC, `78,78,5,1f,17,1,1` and zone `22,3c` (14 + 20 = 34). The `dual` lines
  decide: `0001-01-01` and a difference of one year (365 or 366 days) mean "no year 0"; `0000-01-01` or two
  years (730 or 731) mean a year 0. The third line is 1 if Oracle passes from the Julian to the Gregorian
  calendar at 1582-10-15, 11 if it counts the proleptic Gregorian calendar; `TO_CHAR(d, 'J')` of rows 5 and
  6 differs by the same number.
- Exports: `tables=(t_bc,t_frame)`, conventional and `direct=y`, `statistics=none`. File names wanted:
  `bc_exp_<v>.dmp`, `bc_exp_direct_<v>.dmp`, logs, `truth_bc.txt`. Status: proposed. Priority: low.

### R-041 single-column LONG and LONG RAW tables on the direct path (session 4)
- Why: rules E2a, T3 v3.4: on the direct path the LONG column moves last, so a first column of
  descriptor length 0 occurs only when LONG / LONG RAW is the **only** column; then test (b) of E2 says
  nothing and the reader decides "no rows" by E2a alone (suggested, not in the fixtures). Also: a value
  whose length field has low byte 0x0a (test (a) looks at it).
- SQL (as `CORPUS`, AL32UTF8 database; 18c and 11g):

      CREATE TABLE t_long1 (l LONG);
      INSERT INTO t_long1 VALUES ('a');
      INSERT INTO t_long1 VALUES (NULL);
      DECLARE s VARCHAR2(32767) := RPAD('T', 20000, 'z');
      BEGIN INSERT INTO t_long1 VALUES (s); COMMIT; END;
      /
      CREATE TABLE t_long1e (l LONG);
      CREATE TABLE t_lraw1 (lr LONG RAW);
      INSERT INTO t_lraw1 VALUES (HEXTORAW('00112233445566778899'));
      COMMIT;

- Exports: `tables=(t_long1,t_long1e,t_lraw1,t_frame)`, conventional (`long1_exp_<v>.dmp`) and `direct=y`
  (`long1_exp_direct_<v>.dmp`), `statistics=none`; logs. No truth file needed (the SQL is the truth).
- Arithmetic to check (rules T4 to T6, R2, E1, E2, E2a; Y9): every table has `01 00`, one descriptor
  `08 00 00 00` (T_LRAW1 `18 00 00 00`), `00 00` (m) and the flag. T_LONG1, direct: row 1 `01 00 61` (one
  piece), row 2 `fe ff`, row 3 a chain of 20,000 bytes; if the remainder comes first as in session 9
  (hypothesis): 20,000 = 4,078 + 2 x 7,961, i.e. `ee 8f` + 4,078 bytes (`54` + `7a`...), `19 9f` + 7,961,
  `19 1f` + 7,961; then `ff ff 0a`. Conventional: `01 00 61` `00 00`, `fe ff` `00 00`, `20 4e` + 20,000
  bytes `00 00`, `ff ff 0a`. T_LONG1E, direct: 18c `0a` + `TABLE "T_LRAW1"` (E2a "no rows"), 11g `ff ff 0a`
  (E1); conventional `ff ff 0a`. T_LRAW1, direct: `0a 00` + the 10 bytes: the byte at p is LF but the
  next byte is `00`, so E2a reads a row; then `ff ff 0a`. T_FRAME equals `t_frame_exp(_direct)_<v>`.
- File names wanted: `long1_exp_<v>.dmp`, `long1_exp_direct_<v>.dmp`, logs. Status: proposed.
  Priority: low (E2a only).

### R-042 a partitioned table with LOB columns; NCLOB values in the long form (session 4; 18c)
- Why: rule T10 lists "a partitioned table with LOB" as unknown; L4 says "NCLOB long values: not in the
  fixtures". Partitioned tables with CLOB columns are common in corporate schemas. The design reads each
  part with its own m, flag and column entries (T6, L1) and its own LOB value area (L3, L4) (ADR-21); a
  different layout would stop the file with `Framing` or `UnsupportedLayout`.
- SQL (as `CORPUS`, 18c, AL32UTF8):

      CREATE TABLE t_plob (id NUMBER(3), c CLOB, n NCLOB, b BLOB)
        PARTITION BY RANGE (id) (PARTITION p1 VALUES LESS THAN (10),
                                 PARTITION p_empty VALUES LESS THAN (20),
                                 PARTITION p3 VALUES LESS THAN (MAXVALUE));
      DECLARE l_c CLOB; l_n NCLOB; l_b BLOB;
      BEGIN
        INSERT INTO t_plob VALUES (1, TO_CLOB('abc'), TO_NCLOB('abc'), HEXTORAW('00FF'));
        INSERT INTO t_plob VALUES (2, NULL, EMPTY_CLOB(), NULL);
        INSERT INTO t_plob VALUES (30, EMPTY_CLOB(), EMPTY_CLOB(), EMPTY_BLOB()) RETURNING c, n, b INTO l_c, l_n, l_b;
        FOR j IN 1 .. 10 LOOP
          DBMS_LOB.WRITEAPPEND(l_c, 1000, RPAD('c', 1000, 'c'));
          DBMS_LOB.WRITEAPPEND(l_n, 1000, TO_NCHAR(RPAD('n', 1000, 'n')));
          DBMS_LOB.WRITEAPPEND(l_b, 1000, UTL_RAW.COPIES(HEXTORAW('B0'), 1000));
        END LOOP;
        COMMIT;
      END;
      /
      -- truth_plob.txt
      SELECT 'ROW|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(c)), 'NULL') || '|'
             || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(n)), 'NULL') || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL')
        FROM t_plob ORDER BY id;
      SELECT 'P|P1|' || COUNT(*) FROM t_plob PARTITION (p1);
      SELECT 'P|P_EMPTY|' || COUNT(*) FROM t_plob PARTITION (p_empty);
      SELECT 'P|P3|' || COUNT(*) FROM t_plob PARTITION (p3);

- Exports: `tables=(t_plob,t_frame)` conventional (`plob_exp_18c.dmp`) and `direct=y`
  (`plob_exp_direct_18c.dmp`), `statistics=none`.
- Arithmetic to check: three `PARTITION` lines, three parts, then `ENDPARTITION` (T10). Each part: `04 00`,
  descriptors (NUMBER; CLOB `70 00 48 00 69 03 01 00`; NCLOB `70 00 48 00 d0 07 02 00`; BLOB
  `71 00 48 00`), m `04 00`, flag `00 00` **in both files** (L6), entries `00 00 00`, `01 43 00 00`,
  `01 4e 00 00`, `01 42 00 00` (L1). P1: row 1 k = 3, values `06 00` + `00 61 00 62 00 63` (C), the same
  for N, `02 00 00 ff` (B); row 2: C NULL, N a locator, B NULL, so k = 1 and the value `00 00` (empty
  NCLOB); `ff ff 0a`. P_EMPTY: `ff ff 0a` right after the entries. P3: row 30, k = 3, C and N 10,000
  characters = 20,000 bytes each, B 10,000 bytes; all three long (L4: above one chunk): C `fc ff` +
  `20 4e 00 00 00 00 00 00` + chunks (hypothesis from 18c: 16,120 + 3,880), N the same (the first NCLOB
  long value), B `fc ff` + `10 27 00 00 00 00 00 00` + 8,060 + 1,940. Truth: `ROW|1|3|3|2`, `ROW|2|NULL|0|NULL`,
  `ROW|30|10000|10000|10000`; P1 2, P_EMPTY 0, P3 1. T_FRAME after it equals `t_frame_exp(_direct)_18c`.
- File names wanted: `plob_exp_18c.dmp`, `plob_exp_direct_18c.dmp`, logs, `truth_plob.txt`. Status:
  proposed. Priority: medium; high if Q-4 shows partitioned tables with LOB columns.

### R-043 LONG in the CL8MSWIN1251 database (session 4; 11g)
- Why: rule Y9 confirms the LONG character set (database bytes, whatever the client) in AL32UTF8 only;
  for a single-byte database it says "assumed the same". The design warns (`W-LONG-CHARSET`) on a
  non-ASCII LONG value there. Can share the session of R-038 (same database, same user).
- SQL (as the R-015 user, 11g CL8MSWIN1251 database; `CHR(n)` is the database byte n):

      CREATE TABLE r_long (id NUMBER(3), l LONG, v VARCHAR2(10));
      INSERT INTO r_long VALUES (1, CHR(192) || CHR(225) || CHR(226) || ' x', 'a');
      DECLARE blk VARCHAR2(66);
      BEGIN
        FOR i IN 192 .. 255 LOOP blk := blk || CHR(i); END LOOP;
        blk := blk || CHR(168) || CHR(184);
        INSERT INTO r_long VALUES (2, blk, 'b');
        COMMIT;
      END;
      /

- Exports: `tables=(r_long,r_text)`, conventional and `direct=y`, with `NLS_LANG` client CL8MSWIN1251
  (`ru_long_exp_cp1251_11g.dmp`, `ru_long_exp_direct_cp1251_11g.dmp`) and US7ASCII
  (`ru_long_exp_us7ascii_11g.dmp`), `statistics=none`. No truth file needed beyond `truth_ru.txt` for R_TEXT.
- Arithmetic to check (Y9, R2, T3): conventional: row 1 L = `05 00 c0 e1 e2 20 78` (`Абв x`), row 2 L =
  `42 00` + `c0 c1 ... ff a8 b8` (66 bytes), with **both** clients; direct: `INSERT INTO` order ID, V, L
  and each L one piece (the same bytes); R_TEXT after R_LONG equals `truth_ru.txt` (T8).
- File names wanted: the three dumps and logs. Status: proposed. Priority: low; medium if Q-4 shows LONG
  columns in a single-byte database.

**State after Analyst session 8** (history; `requests.md`, session 3 of the Architect): R-027 (LOB), R-029
(INTERVAL, ROWID, UROWID), R-030 (partitions) generated and used (rules L1 to L6, Y10, Y11, T10); R-031
proposed, with an Analyst addition for [E2a]; R-032, R-033 proposed (checked by the Analyst: R-032 asks
which "H38" is meant, answered in note A-11 below); new from the Analyst: R-034 (replaces R-011), R-035,
R-036, R-037. Files for R-031 to R-037 arrived in `18c` / `11g` during this session (not yet written
up); for those, "blocks" below means "blocks until the Analyst has written them up". What they block,
**in order of value for the alpha** (Q-4 decides the conditional ones):

| order | request | blocks the alpha? | why |
|---|---|---|---|
| 1 | **R-038** (new) LOB in the CL8MSWIN1251 database: long Cyrillic CLOB, NCLOB, BLOB | **yes, if** the owner's database is single-byte and has CLOB / NCLOB columns (probable: Cyrillic data, an 11.2.0.4 database written by a 10.2 client) | without it, every CLOB value above about 8 KB and every NCLOB value of such a database is read by a rule seen only in AL32UTF8 (`W-LOB-UNTESTED`); a wrong reading is silently wrong text, not a stop. R-035 does **not** cover it: its optional CL8MSWIN1251 part writes only `x` and `5a`, ASCII, and drops the non-ASCII row |
| 2 | **R-035** LOB short / long switch; non-ASCII and non-BMP CLOB text in AL32UTF8 | **yes, if** the owner's database is AL32UTF8 with CLOB columns | `W-LOB-CHARSET` fires on every non-ASCII CLOB of such a database; Cyrillic text would hit it on every row |
| 3 | R-037 partitions: empty partition, subpartitions, statistics (18c) | **yes, if** the owner's tables are partitioned; partly answerable only on 18c (11g XE has no partitioning, so the owner's 11.2.0.4 form cannot be generated) | an empty partition (common: a future range partition) on the direct path and subpartitions are read with `W-UNTESTED-LAYOUT` by the design; a wrong guess there is usually a `Framing` stop |
| 4 | R-031 LONG > 65,519 bytes, direct path, non-ASCII, empty direct ([E2a]) | yes, if Q-4 shows 8 or 24 | unchanged from session 2; a long LONG is `Framing` now (ADR-15) |
| 5 | R-032 computed NUMBER values | no; evidence for Q-1 | cheap; the Analyst's H38 question is answered in A-11 |
| 6 | R-036 INTERVAL mixed signs, precisions; ROWID prefix; logical UROWID | no | the reader warns (`W-UNVERIFIED-TYPE`) or errs per value (`E-BAD-ROWID`); never a stop |
| 7 | R-039 (new) NOT NULL interval text | no | metadata only: such a column falls back with `W-DDL-FALLBACK` (Duration(Microsecond), exact or error) |
| 8 | R-034 `VARCHAR2(n CHAR)` caps (replaces R-011) | no | metadata check only |
| 9 | R-033 non-BMP NCHAR / NVARCHAR2 / `CHAR(n CHAR)` | no | can share R-035's export (the Analyst suggests adding CLOB and NCLOB columns, agreed) |

Requests that cannot block anything the corpus could answer: R-022 (an `exp` 10.2 client) stays the
largest gap; only the owner's survey can stand in for it (arch 8.5).

**State after Analyst session 7** (history): R-024, R-025, R-026 (a and b), R-028 generated and
used (rules Y1, T9, C5 to C7, Y9); R-027, R-029, R-030 proposed, not generated; R-031 new (Analyst);
R-011 partly answered (`CHAR(n CHAR)` only). What the open ones block, **in order of value for the
alpha** (Q-4 can reorder it):

| order | request | what it blocks | why here |
|---|---|---|---|
| 1 | R-027 LOB | every table after a CLOB / BLOB / NCLOB column in the same file (ADR-12); 6 fixtures show it (R_TEXT unreachable in `ru_exp_*`) | the most likely unsupported type in a corporate schema, and a stop, not a value |
| 2 | R-031 LONG > 65,519 bytes, direct path, non-ASCII | the three LONG warnings of arch 5.4 / 6.1 becoming rules; without it a LONG above 65,519 bytes may be misread until a later `Framing` error (ADR-15) | LONG is now read, so its unknowns are the only place where the alpha could deliver wrong values instead of stopping; first if Q-4 shows code 8 or 24 |
| 3 | R-029 INTERVAL, ROWID, UROWID | every table after such a column (ADR-12) | rarer in ETL schemas than LOB; a stop, not a misreading |
| 4 | R-030 partitioned table | nothing is known: one section per table or per partition; duplicate names would give `AmbiguousTableName` and need `ByOrdinal` | the reader's framing probably holds (T8), the naming does not |
| 5 | R-011 `VARCHAR2(n CHAR)` in AL32UTF8 | only the `W-LONGER-THAN-DECLARED` check and the length check of arch 5.5 for `n CHAR` forms; values are read by their length field whatever the answer | metadata only; R-033 can share its export |
| 6 | R-032 (new) | evidence for Q-1, not a block | cheap, decides how ordinary 40-digit values are |
| 7 | R-033 (new) | `trim_char` and the survey for non-BMP characters | low |

### R-024 NUMBER: the conversion from bytes to a decimal number as a rule, with boundary values
- Why: rule Y1 confirms the bytes; exp.md 6.1 says the conversion "is not part of this document's
  evidence". A reader needs it as a rule with a status (sign, exponent byte, digit bytes, the closing
  `66`, the longest forms), and vectors at the limits of Int64 and of small scales.
- SQL (as `CORPUS`):

      CREATE TABLE t_num2 (id NUMBER(3), v NUMBER);
      INSERT INTO t_num2 VALUES (1, 1000000);
      INSERT INTO t_num2 VALUES (2, -1000000);
      INSERT INTO t_num2 VALUES (3, 0.001);
      INSERT INTO t_num2 VALUES (4, -0.001);
      INSERT INTO t_num2 VALUES (5, 99);
      INSERT INTO t_num2 VALUES (6, -99);
      INSERT INTO t_num2 VALUES (7, -100);
      INSERT INTO t_num2 VALUES (8, 999999999999999999);
      INSERT INTO t_num2 VALUES (9, 1000000000000000000);
      INSERT INTO t_num2 VALUES (10, 9223372036854775807);
      INSERT INTO t_num2 VALUES (11, 9223372036854775808);
      INSERT INTO t_num2 VALUES (12, -9223372036854775808);
      INSERT INTO t_num2 VALUES (13, 1E-130);
      INSERT INTO t_num2 VALUES (14, 0.00000000001);
      COMMIT;
      -- truth_num2.txt
      SELECT id || '|' || TO_CHAR(v, 'TM9') || '|' || DUMP(v, 16) FROM t_num2 ORDER BY id;

- Arithmetic to check (hypothesis: the base-100 reading that fits the 32 pairs of exp.md 6.1, where
  100 = `c2 02`, .01 = `c0 02`, -123 = `3d 64 4e 66`, 1E-125 = `82 0b`): 1 `c4,2`; 2 `3b,64,66`; 3 `bf,b`;
  4 `40,5b,66`; 5 `c1,64`; 6 `3e,2,66`; 7 `3d,64,66`; 8 `c9` + 9 x `64` (Len=10); 9 `ca,2`; 10
  `ca,a,17,22,49,4,45,37,4e,3b,8` (Len=11); 11 the same with last byte `9`; 12
  `35,5c,4f,44,1d,62,21,2f,18,2b,5d,66` (Len=12); 13 `80,2`; 14 `bb,b`. A line that differs refutes the
  reading for that case.
- File names wanted: `t_num2_exp_<v>.dmp`, log, `truth_num2.txt`. Status: proposed

### R-025 the column list of the `CREATE TABLE` line: declared types as `exp` writes them
- Why: the descriptor has no precision or scale (exp.md 4.1); the reader takes them from the column list
  (ADR-6), whose form is only observed (`"N" NUMBER(10, 2)`, `"V" NUMBER`, `"F" FLOAT(24)`, `"TS6"
  TIMESTAMP (6)`, `NOT NULL ENABLE`). Wanted: the grammar as a rule, and how the forms below are written.
- SQL (as `CORPUS`):

      CREATE TABLE t_decl (
        n_plain NUMBER, n_star NUMBER(*,2), n_38 NUMBER(38), n_neg NUMBER(5,-2), n_sgp NUMBER(3,5),
        i_int INTEGER, d_dec DECIMAL(10,2), f_plain FLOAT, f_126 FLOAT(126), r_real REAL,
        ts_def TIMESTAMP, d_nn DATE NOT NULL, v_def VARCHAR2(10) DEFAULT 'a,b)', "low,er(x" VARCHAR2(5));
      INSERT INTO t_decl VALUES (1, 1.25, 1, 100, 0.00123, 1, 1.25, 1.5, 1.5, 1.5,
        TIMESTAMP '2020-01-02 03:04:05.123456', DATE '2020-01-02', DEFAULT, 'x');
      COMMIT;
      -- truth_decl.txt
      SELECT column_id || '|' || column_name || '|' || data_type || '|' || NVL(TO_CHAR(data_precision), 'NULL')
             || '|' || NVL(TO_CHAR(data_scale), 'NULL') || '|' || data_length || '|' || nullable
        FROM user_tab_columns WHERE table_name = 'T_DECL' ORDER BY column_id;

- Arithmetic to check: 14 columns, so the column count is `0e 00`; by rule T5 the descriptors take
  12 x 4 + 2 x 8 = 64 bytes (ten of type 2 with length 22, one 180 with 11, one 12 with 7, two of type
  1); one ` MODIFY DEFAULT` item stands before the `INSERT INTO` line (rule T7); `truth_decl.txt` has 14
  lines. What the text says for `NUMBER(*,2)`, `INTEGER`, `FLOAT`, `REAL` and the quoted name with a
  comma and a parenthesis is the question.
- File names wanted: `t_decl_exp_<v>.dmp`, log, `truth_decl.txt`. Status: proposed

### R-026 (a) NCHAR and `CHAR(n CHAR)`; (b) Oracle's mapping of the single-byte sets to Unicode
- Why: (a) rule T5 has code 96 for CHAR only; NCHAR and a CHAR with character semantics are not in the
  fixtures (descriptor id, form, length; padding bytes). (b) The reader decodes CL8MSWIN1251 and
  WE8MSWIN1252 with the WHATWG tables (ADR-5); the fixtures do not cover every byte.
- SQL (a) (as `CORPUS`, AL32UTF8 database):

      CREATE TABLE t_nchar (id NUMBER(3), nc NCHAR(5), cc CHAR(4 CHAR), nv NVARCHAR2(5));
      INSERT INTO t_nchar VALUES (1, N'ab', 'ab', N'ab');
      INSERT INTO t_nchar VALUES (2, UNISTR('\0434\0436'), UNISTR('\0434\0436'), UNISTR('\0434\0436'));
      INSERT INTO t_nchar VALUES (3, NULL, NULL, NULL);
      COMMIT;
      -- truth_nchar.txt
      SELECT id || '|' || DUMP(nc, 16) || '|' || DUMP(cc, 16) || '|' || DUMP(nv, 16) FROM t_nchar ORDER BY id;

- SQL (b) (truth only, no dump; any of the databases):

      -- truth_csmap.txt
      SELECT cs || '|' || TO_CHAR(n, 'FM0X') || '|' || DUMP(UTL_I18N.RAW_TO_NCHAR(HEXTORAW(TO_CHAR(n, 'FM0X')), cs), 16)
        FROM (SELECT LEVEL - 1 AS n FROM dual CONNECT BY LEVEL <= 256),
             (SELECT 'US7ASCII' AS cs FROM dual UNION ALL SELECT 'WE8MSWIN1252' FROM dual
              UNION ALL SELECT 'CL8MSWIN1251' FROM dual)
       ORDER BY cs, n;

- Arithmetic to check. (a) Hypothesis: NC descriptor `60 00 0a 00 d0 07 02 00`; row 1 NC is 10 bytes
  `00 61 00 62` + 3 x `00 20`, row 2 `04 34 04 36` + 3 x `00 20`; CC row 1 is 4 bytes `61 62 20 20`, row 2
  is 6 bytes `d0 b4 d0 b6 20 20` (padded to 4 characters); NV rows 1 and 2 are 4 bytes; the descriptor
  length of CC is 4 or 16 (the question of R-011). (b) 3 x 256 = 768 lines; expected equal to the WHATWG
  tables except, possibly, 0x98 (1251) and 0x81, 0x8d, 0x8f, 0x90, 0x9d (1252); what Oracle returns for
  bytes above 0x7f in US7ASCII is the third question. If `UTL_I18N` is missing on 11g XE, 18c is enough.
- File names wanted: `t_nchar_exp_<v>.dmp`, log, `truth_nchar.txt`; `truth_csmap.txt`. Status: proposed

### R-027 LOB columns: the framing, so that a LOB table can at least be skipped
- Why: rule X1 and exp.md 14.6: one CLOB table in a single-byte 11g database, observed only. A reader
  that meets code 112 must stop the file (ADR-12); in 6 fixtures that hides R_TEXT. Wanted as rules: the
  descriptor of CLOB, BLOB, NCLOB; the bytes between descriptors and rows; the row layout with a LOB in
  the middle; NULL against empty; a LOB above 65,535 bytes; a multi-byte database.
- SQL: `T_LOB` of `corpus/sql/01b_optional_tables.sql` (exists), and:

      CREATE TABLE t_lob2 (id NUMBER(3), a CLOB, v VARCHAR2(10), n NCLOB, b BLOB);
      INSERT INTO t_lob2 VALUES (1, TO_CLOB('abc'), 'x', TO_NCLOB('abc'), HEXTORAW('00FF'));
      INSERT INTO t_lob2 VALUES (2, NULL, 'y', EMPTY_CLOB(), NULL);
      COMMIT;
      -- truth_lob.txt
      SELECT 'T_LOB|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(c)), 'NULL') || '|'
             || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL') || '|' || RAWTOHEX(DBMS_LOB.SUBSTR(b, 30, 1))
        FROM t_lob ORDER BY id;

- Exports: `tables=(t_lob,t_lob2,t_frame)`, conventional and `direct=y` (the log is expected to say that
  the LOB tables go the conventional way).
- Arithmetic to check: T_LOB has 5 rows; C lengths 9, NULL, 0, 3000, 100 x 1000 = 100,000 characters;
  B lengths 10, NULL, 0, 1000 (2000 hexadecimal digits), 100 x 1000 = 100,000 bytes. Row 5 does not fit
  one u16 length: how many chunks. T_FRAME after the LOB tables must be byte-identical to
  `t_frame_exp_<v>.dmp` (rule T8) and gives the end of the LOB section.
- File names wanted: `lob_exp_<v>.dmp`, `lob_exp_direct_<v>.dmp`, logs, `truth_lob.txt`. Status: proposed

### R-028 LONG and LONG RAW: the framing
- Why: outside the scope (`docs/scope.md`), but one such column stops the file like a LOB (rule X1).
- SQL (as `CORPUS`):

      CREATE TABLE t_long (id NUMBER(3), l LONG, v VARCHAR2(10));
      INSERT INTO t_long VALUES (1, 'short', 'a');
      INSERT INTO t_long VALUES (2, NULL, 'b');
      DECLARE s VARCHAR2(32767) := RPAD('L', 32000, 'x');
      BEGIN INSERT INTO t_long VALUES (3, s, 'c'); COMMIT; END;
      /
      CREATE TABLE t_longraw (id NUMBER(3), lr LONG RAW);
      INSERT INTO t_longraw VALUES (1, HEXTORAW('00FF'));
      DECLARE r RAW(32767) := UTL_RAW.COPIES(HEXTORAW('AB'), 32000);
      BEGIN INSERT INTO t_longraw VALUES (2, r); COMMIT; END;
      /

- Arithmetic to check (truth = this SQL): L is 5 bytes, NULL, 32,000 bytes (`L` + 31,999 x `x`); LR is 2
  bytes and 32,000 x `ab`; V follows L in every row, which shows where a LONG value ends.
- File names wanted: `long_exp_<v>.dmp` (`tables=(t_long,t_longraw,t_frame)`), log. Status: proposed

### R-029 INTERVAL, ROWID, UROWID: descriptor size and value bytes
- Why: second-wave types of `docs/scope.md` (INTERVAL) and types a corporate schema can hold.
- SQL: `T_INTERVAL` of `corpus/sql/01b_optional_tables.sql` (exists; 4 rows, the fourth all NULL), and:

      CREATE TABLE t_rowid (id NUMBER(3), rid ROWID, urid UROWID);
      INSERT INTO t_rowid SELECT 1, ROWID, ROWID FROM t_frame WHERE ROWNUM = 1;
      INSERT INTO t_rowid VALUES (2, NULL, NULL);
      COMMIT;
      -- truth_misc.txt
      SELECT id || '|' || TO_CHAR(iym) || '|' || DUMP(iym, 16) || '|' || TO_CHAR(ids) || '|' || DUMP(ids, 16)
        FROM t_interval ORDER BY id;
      SELECT id || '|' || ROWIDTOCHAR(rid) || '|' || DUMP(rid, 16) || '|' || DUMP(urid, 16) FROM t_rowid ORDER BY id;

- Arithmetic to check: 3 non-NULL values per interval column and 1 per ROWID column; the type code of
  each descriptor should equal the `Typ=` of its `DUMP` (as for the 8 known codes, exp.md 4.1); whether
  the descriptor is 4 or 8 bytes is the question.
- File names wanted: `misc_exp_<v>.dmp` (`tables=(t_interval,t_rowid,t_frame)`), log, `truth_misc.txt`.
  Status: proposed

### R-030 a partitioned table (18c only)
- Why: "Not covered by any fixture" (rules file, scope). Partition-level extraction is out of scope, but
  the rows of a partitioned table are table data: one section or one per partition, and how they are named.
- SQL: `T_PART` of `corpus/sql/01b_optional_tables.sql` (exists). Truth: `SELECT 'P|' || partition_name ||
  '|' || (row count)` per partition and `COUNT(*)`.
- Arithmetic to check: 3000 rows, `sale_date` = 2019-01-01 + MOD(13 x lvl, 1000); 13 is prime to 1000, so
  every day offset 0 to 999 occurs 3 times: P2019 (offsets 0 to 364) 365 x 3 = 1095 rows, P2020 (365 to
  730, a leap year) 366 x 3 = 1098, PMAX (731 to 999) 269 x 3 = 807; sum 3000.
- File names wanted: `part_exp_18c.dmp`, `part_exp_direct_18c.dmp`, logs, `truth_part.txt`. Status: proposed

### R-032 NUMBER: computed values in a column declared `NUMBER` (session 2)
- Why: Q-1 / ADR-14 rest on the statement that ordinary data (a quotient, an average) are stored with up
  to 40 significant digits. `t_num` rows 30 and 31 were inserted as literals; whether a value **computed**
  by SQL is stored with 40 digits, or rounded to 38, is not in the fixtures. It decides how often
  `on_number_overflow` will fire on real data.
- SQL (as `CORPUS`, both databases):

      CREATE TABLE t_num3 (id NUMBER(3), v NUMBER);
      INSERT INTO t_num3 VALUES (1, 1/3);
      INSERT INTO t_num3 VALUES (2, 2/3);
      INSERT INTO t_num3 VALUES (3, 1/7);
      INSERT INTO t_num3 VALUES (4, 100/3);
      INSERT INTO t_num3 VALUES (5, -1/3);
      INSERT INTO t_num3 SELECT 6, AVG(x) FROM (SELECT 1 x FROM dual UNION ALL SELECT 2 FROM dual UNION ALL SELECT 2 FROM dual);
      COMMIT;
      -- truth_num3.txt
      SELECT id || '|' || TO_CHAR(v, 'TM9') || '|' || DUMP(v, 16) FROM t_num3 ORDER BY id;

- Arithmetic to check (hypothesis H40: 20 digit bytes, the last rounded half up; hypothesis H38: 19
  digit bytes). Row 1, H40: `c0` + 20 x `22` (Len=21, as `t_num` row 31). Row 2, H40: 0.666...67 = `c0` +
  19 x `43` + `44` (digit 66 is byte 0x43, 67 is 0x44). Row 3, H40: 1/7 = 0.(142857); 40 digits are 6 x
  `142857` + `1428`, the 41st digit is 5, so the last pair rounds up to 29: pairs 14, 28, 57 six times
  then 14, 29: `c0` + 6 x `0f 1d 3a` + `0f 1e` (Len=21). Row 4, H40: 33.33...: first pair 33 at E = 0,
  then 19 pairs 33: `c1` + 20 x `22`. Row 5, H40: `3f` + 20 x `44` (101 - 33 = 68 = 0x44), no closing
  `66` because L = 21. Row 6: 5/3 = 1.66...67: H40 `c1 02` + 18 x `43` + `44`. Under H38 each row has
  one digit byte less and Len=20 (row 5: 19 digit bytes and a closing `66`, Len=21).
- Exports: `tables=(t_num3)`, conventional; `t_num3_exp_<v>.dmp`, log, `truth_num3.txt`. Status: proposed

### R-033 characters outside the Basic Multilingual Plane in NCHAR, NVARCHAR2 and `CHAR(n CHAR)` (session 2; can share the export of R-011)
- Why: rule C5 is confirmed for BMP characters only. A character such as U+1F600 is two UTF-16 code
  units (and 4 bytes in AL32UTF8). Whether `NCHAR(n)` pads to n characters or to n code units decides what
  `trim_char` and the survey's lengths mean; a reader that decodes UTF-16BE must also see a correct
  surrogate pair in the bytes.
- SQL (as `CORPUS`, AL32UTF8 database; may be added to `t_probe11` of R-011):

      CREATE TABLE t_nchar2 (id NUMBER(3), nc NCHAR(3), nv NVARCHAR2(3), cc CHAR(3 CHAR), vc VARCHAR2(3 CHAR));
      INSERT INTO t_nchar2 VALUES (1, UNISTR('a\D83D\DE00'), UNISTR('a\D83D\DE00'), UNISTR('a\D83D\DE00'), UNISTR('a\D83D\DE00'));
      COMMIT;
      -- truth_nchar2.txt
      SELECT id || '|' || DUMP(nc, 16) || '|' || DUMP(nv, 16) || '|' || DUMP(cc, 16) || '|' || DUMP(vc, 16) FROM t_nchar2;

- Arithmetic to check: NV = `00 61 d8 3d de 00` (6 bytes). NC: if the padding counts code units, 3
  units are already there: the same 6 bytes; if it counts characters (2 here), one `00 20` more: 8 bytes.
  CC: `61 f0 9f 98 80` (5 bytes) + one `20` if padded to 3 characters (6 bytes). VC: 5 bytes. If Oracle
  refuses the insert (length semantics), that is the answer for that column. Descriptor lengths: NC 6,
  NV 6, CC 12 by C6, VC 3 or 12 (the question of R-011).
- File names wanted: `nchar2_exp_<v>.dmp`, log, `truth_nchar2.txt`. Status: proposed, low priority.

### R-038 LOB values in the CL8MSWIN1251 database: long Cyrillic CLOB, NCLOB, BLOB (session 3)
- Session 8: **generated** (Analyst session 10) and answered: CLOB = database bytes in both forms, NCLOB =
  UTF-16 BE in both forms, BLOB = bytes; chunks 8,132 (CLOB, BLOB) and 16,264 (NCLOB) on 11g [L4, L5 v3.5]. The
  hypotheses on chunk sizes in the arithmetic below were partly wrong for C (8,132, not 16,264): ADR-37.
- Why: rule L5 confirms CLOB data of the single-byte database as database bytes, 1 per character, for
  short values only (R_CLOB: 6 and 6,000 bytes, `exp.md` 14.6); the long form [L4] (`fc ff`, 8-byte
  total, chunks) is seen only in AL32UTF8 (UTF-16 BE data, chunk sizes 16,264 / 8,132 on 11g). Unknown
  for the single-byte database: whether a long CLOB is still database bytes (or UTF-16), its chunk
  sizes, where the short form ends, and NCLOB and BLOB there at all (`exp.md` 14.6, "Not in the
  fixtures"). The project owner's data are Cyrillic, written from an 11.2.0.4 database; if that database
  is CL8MSWIN1251, this is the case the reader meets on every CLOB above a few KB. R-035's optional
  CL8MSWIN1251 part uses only `x` and `5a` and no NCLOB text, so it cannot show a wrong character set.
- SQL (as the R-015 user, the 11g CL8MSWIN1251 database; `CHR(n)` gives the database byte n):

      CREATE TABLE r_lob2 (id NUMBER(3), c CLOB, n NCLOB, b BLOB);
      DECLARE
        blk VARCHAR2(66);
        l_c CLOB; l_n NCLOB; l_b BLOB;
      BEGIN
        FOR i IN 192 .. 255 LOOP blk := blk || CHR(i); END LOOP;   -- 0xC0..0xFF: А..я
        blk := blk || CHR(168) || CHR(184);                        -- 0xA8 Ё, 0xB8 ё
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
      -- truth_lob_ru.txt
      SELECT 'R_LOB2|' || id || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(c)), 'NULL') || '|'
             || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(n)), 'NULL') || '|' || NVL(TO_CHAR(DBMS_LOB.GETLENGTH(b)), 'NULL') || '|'
             || NVL(DUMP(DBMS_LOB.SUBSTR(c, 4, 1), 16), 'NULL') || '|' || NVL(DUMP(DBMS_LOB.SUBSTR(n, 4, 1), 16), 'NULL') || '|'
             || NVL(DUMP(DBMS_LOB.SUBSTR(c, 4, DBMS_LOB.GETLENGTH(c) - 3), 16), 'NULL') || '|'
             || NVL(DUMP(DBMS_LOB.SUBSTR(n, 4, DBMS_LOB.GETLENGTH(n) - 3), 16), 'NULL')
        FROM r_lob2 ORDER BY id;

- Arithmetic to check. Row 1: C 66 characters; if database bytes (L5), the short value `42 00` + `c0 c1
  ... ff a8 b8` (66 bytes); N in UTF-16 BE (L5) `84 00` + `04 10 04 11 ... 04 4f 04 01 04 51` (132 bytes:
  U+0410 to U+044F, U+0401, U+0451); B `42 00` + the same 66 bytes as C; k = 3. Row 2: 303 x 66 = 19,998
  + 2 = 20,000 characters; the last 4 are `a8 b8 c0 c1` (end of block 303, then the first two of block
  1), so the truth's tail DUMP of C is `Typ=1 Len=4: a8,b8,c0,c1`, of N `Len=8: 4,1,4,51,4,10,4,11`.
  C: 20,000 bytes if database bytes, 40,000 if UTF-16; either can be short (`20 4e` or `40 9c`, both
  below 0xfff0) or long (`fc ff` + `20 4e 00 00 00 00 00 00` or `40 9c 00 00 00 00 00 00` + chunks); on
  11g the AL32UTF8 chunks were 8,132 bytes (BLOB) and 16,264 (CLOB): hypothesis for B 8,132 + 8,132 +
  3,736 and for N 16,264 + 16,264 + 7,472. Row 3: three `fe ff`, k = 0. GETLENGTH: 66 / 20,000 for C, N,
  B. R_TEXT after R_LOB2 must equal `truth_ru.txt` (rule T8).
- Exports (11g, CL8MSWIN1251 database): `tables=(r_lob2,r_text)` with `NLS_LANG` client CL8MSWIN1251
  (`ru_lob2_exp_cp1251_11g.dmp`) and US7ASCII (`ru_lob2_exp_us7ascii_11g.dmp`; L5 says the data do not
  depend on the client), conventional, `statistics=none`.
- File names wanted: the two dumps, logs, `truth_lob_ru.txt`. Status: proposed. Priority: high if Q-4
  shows CLOB / NCLOB columns in a single-byte database, else medium.

### R-039 the column list of NOT NULL interval columns and default precisions (session 3)
- Why: rule T9: `INTERVAL YEAR (p) TO MONTH ` ends with a space before `, ` or `)`; how ` NOT NULL
  ENABLE` follows it (one space or two) and how the default precisions are written are not in the
  fixtures. The design parses only the forms seen and falls back per column otherwise (arch 5.5), which
  costs the column its fractional precision (Duration unit).
- SQL (as `CORPUS`; 18c and 11g):

      CREATE TABLE t_ivnn (id NUMBER(3), iym INTERVAL YEAR TO MONTH NOT NULL,
                           ids INTERVAL DAY TO SECOND NOT NULL, ids9 INTERVAL DAY(9) TO SECOND(9) NOT NULL,
                           iym9 INTERVAL YEAR(9) TO MONTH);
      INSERT INTO t_ivnn VALUES (1, INTERVAL '1-2' YEAR TO MONTH, INTERVAL '3 04:05:06.7' DAY TO SECOND,
                                 INTERVAL '0 00:00:00.000000001' DAY(9) TO SECOND(9), NULL);
      COMMIT;
      -- truth_ivnn.txt
      SELECT 'COL|' || column_id || '|' || column_name || '|' || data_type || '|' || NVL(TO_CHAR(data_precision), 'NULL')
             || '|' || NVL(TO_CHAR(data_scale), 'NULL') || '|' || data_length || '|' || nullable
        FROM user_tab_columns WHERE table_name = 'T_IVNN' ORDER BY column_id;
      SELECT 'ROW|' || id || '|' || DUMP(iym, 16) || '|' || DUMP(ids, 16) || '|' || DUMP(ids9, 16) FROM t_ivnn;

- Arithmetic to check (rule Y10): IYM = 1 year 2 months = `80 00 00 01 3e` (60 + 2 = 0x3e); IDS = 3 d 04:05:06.7
  = `80 00 00 03 40 41 42 a9 b9 27 00` (hours 64, minutes 65, seconds 66; 700,000,000 = 0x29b92700, plus
  0x80000000 = 0xa9b92700); IDS9 = 1 ns = `80 00 00 00 3c 3c 3c 80 00 00 01`; IYM9 `fe ff`. Descriptors 5, 11,
  11, 5 (T5). Expected texts (hypothesis): `"IYM" INTERVAL YEAR (2) TO MONTH  NOT NULL ENABLE` (two spaces)
  or one space; `"IDS" INTERVAL DAY (2) TO SECOND (6) NOT NULL ENABLE`; `"IYM9" INTERVAL YEAR (9) TO MONTH )`.
- Exports: `tables=(t_ivnn,t_frame)`, conventional, `statistics=none`; can share R-036's export.
- File names wanted: `ivnn_exp_<v>.dmp`, log, `truth_ivnn.txt`. Status: proposed. Priority: low.

### Notes on rules v3.5 (session 9, no fixture needed; from Reviewer session 1, ADR-38 item 7)

Session 10: both notes are still open (review 02 section 6 re-derived them: `collist.py` 386 `CREATE TABLE` lines,
1,907 column definitions, PROBLEMS 0). They are inputs for the next Analyst session, with the R-045 record and the
R-040 write-up.

- **T9** (rules line 226): "382 lines, 1885 column definitions, `tools/collist.py`". Re-run read-only by the
  Reviewer: 386 `CREATE TABLE` lines, 1,907 definitions, PROBLEMS 0 over the 201 files (review-01 section 3).
  Please update the row and the "Changes in session 10" line.
- **X2** (rules line 251): the evidence names `ru_exp_us7ascii_11g`, `ru_exp_w1252_11g` and `EXP-00056` only;
  `exp.md` 14.3 and `ru_exp_direct_us7ascii_11g.log` add the third failed export with `EXP-00008` / `ORA-00604` /
  `ORA-00942`. Please add it (the reader's `W-POSSIBLY-INCOMPLETE` fires on all three, ADR-39).

### Notes on rules v3.4 (session 4, no fixture needed)

State of A-11 to A-17: A-11 answered (`requests.md` R-032 note: H40 holds, both H38 readings refuted);
A-12 overtaken (R-040 uses 65,519 / 65,520 consistently); A-13 **open** (L4 still names only 0 and
`fc ff` for the first u16 of a value); A-14 answered (E2: "a reader's list also holds `PARTITION "`,
`COMPARTITION "`, `ENDPARTITION`"); A-15 answered (T10: statistics items after the last part, before
`ENDPARTITION`); A-16 answered (the limit paragraph names the tables and "18c only"); A-17 open until
R-036 / R-039 (the ROWID prefix stays unexplained; R-039 not generated).

| # | note |
|---|---|
| A-18 | R2 / R8 (direct-path LONG chain): R8 says 0xfff0 to 0xfffd are errors "in a column field" and that "a u16 with bit 0x8000 is a piece length". A piece of 0x7ff0 to 0x7fff bytes with the flag would be the u16 0xfff0 to 0xffff, i.e. a marker (exp.md 6.6 lists it as unknown). Please state in R2 which wins, or that it is unknown; the design treats a later piece u16 from 0xfff0 as `Framing` and a piece of length 0 (0x8000, or a final 0x0000) as readable with a warning (ADR-15 session 4). Also: can a first piece have the flag and length 0? |
| A-19 | E2a lists the known item kinds without the partition lines, while E2 test (a) adds `PARTITION "`, `COMPARTITION "`, `ENDPARTITION` for "a reader's list" (and saw `SUBPARTITION "`, `ENDCOMPARTITION`). Harmless today (a single-column LONG partitioned table is not in the fixtures), but the two lists should be one, or the difference explained. The design uses one list with all five partition lines (ADR-22). |
| A-20 | L5: "A reader decodes a CLOB by the descriptor's set id: a single-byte set = that set" reads as a rule for every form, but its status confirms the single-byte database for **short** values only and the "Limit of confirmed" paragraph names "a LOB in a single-byte database with values around the switch" as not covered. Please qualify the sentence ("short form; long form: R-038"); a reader taking it literally decodes a long CLOB of CL8MSWIN1251 by a guess (ADR-25 refuses it). |
| A-21 | R-038 (LOB in the CL8MSWIN1251 database) and R-039 (NOT NULL intervals), written in this file in Architect session 3, are not recorded in `requests.md` (R-038 appears only in the R-035 note, R-039 nowhere), unlike R-024 to R-030 and R-032 / R-033. Please add both with their status (R-038: the owner is preparing it), and R-041 to R-043 of this session. |
| A-22 | R8 leaves the reaction to a conventional LONG length from 0x8000 open ("stop or warn"). The design stops the file (`UnsupportedLayout`, ADR-12) because the two readings R-040 names consume different byte counts. If the Analyst sees a reason to prefer a warning (some evidence that only one reading is possible), please say so. |
| A-24 | (session 5, from Implementer note N-14) `uv run docs/formats/tools/num_decode.py` over the three fixture folders prints steps 1 and 2 (120 values) and stops in step 3 with `KeyError: 0` at `w = want[ci]` (line 195), before "3. NUMBER fields in ... files". The design cites "85,456 NUMBER fields of 199 files decode and re-encode" [Y1]; that figure now comes from the Implementer's generator (`exp_parse.parse`), not from the tool. Please fix step 3 so the rules file's count can be reproduced by its own tool. |
| A-23 | C6: "w for other sets: assumed" while the same row cites `VARCHAR2(20 CHAR)` = 20 in CL8MSWIN1251 (w = 1 for 171). Is w = 1 for set 171 confirmed (by `truth_ru.txt` or `ru_*`), and only the other single-byte sets assumed? The design checks the `n CHAR` length for 873 and 171 only (arch 5.5 step 3). |

### Notes on the rules file, session 3 (history; no fixture needed)

Session 3: A-6 to A-10 are answered in rules v3.3 (Y1 38 / 40 digits; E2a; Y9 set; R8 wording; H6
pairing and "base-100 digit"). New notes on v3.3:

| # | note |
|---|---|
| A-11 | R-032, the Analyst's question: "H38" means **38 significant decimal digits** (what Oracle's 38-digit precision would keep), so row 6 under H38 is `c1 02` + 18 x `43` + `47` (Len=21). "19 digit bytes" was a shortcut that holds for rows 1 to 5 only. Please record it so in `requests.md`. |
| A-12 | Y9 and requests R-031 speak of LONG values "above 65,533 bytes"; R8 makes 0xfff0 to 0xfffd errors in a column field, so the largest ordinary length is 0xffef = 65,519 (arch 4.1). Is 65,533 meant (0xfffd), or should it read 65,519? |
| A-13 | L4: the rule names the errors for a **chunk** length (0, from 0xfff0); for the first u16 of a LOB **value** it names 0 (empty) and `fc ff` (long) only. The design treats every other value from 0xfff0 (`fe ff` and `ff ff` included, since a NULL LOB is not counted in k) as `Framing`. Please state it in L4, or say what else could stand there. |
| A-14 | E2 test (a) and E2a list the item kinds that may follow an empty direct-path table; `PARTITION "` and `ENDPARTITION` are not among them, but an empty partition on the 18c direct path [T10, R-037] would put one there. Please add them as "expected, unverified" or say why not. |
| A-15 | T10: can items (statistics, `ALTER`, comments) stand between two parts, or between the last part and `ENDPARTITION`? The fixture has none (`statistics=none`). The design skips items anywhere in a section [T7]; R-037's default statistics will show it. |
| A-16 | T10 status "confirmed" rests on one table in two files of one version. By the status table, "confirmed" needs two independent fixtures; two exports of the same table on two paths are independent in path only. Not wrong, but a reader of the rules file may read more into it; and the owner's 11.2.0.4 database cannot be reproduced (11g XE has no partitioning). Worth one sentence in the "Limit of confirmed" paragraph. |
| A-17 | Y11: the fifth byte of the ROWID prefix `01 00 00 00 17` is 0x17 = 23, the field length; could the prefix be a header with a length (a hypothesis for R-036, not a request). Also T9: is ` NOT NULL ENABLE` after `INTERVAL YEAR (p) TO MONTH ` written with two spaces? (R-039.) |

### Notes on the rules file, session 2 (history)

Session 2: A-1 to A-5 are answered in rules v3.2 (counts for 169 files; E2 states its test; list K has
forms; R8; X1 names the bytes before the rows). New notes on v3.2:

| # | note |
|---|---|
| A-1 | Answered (v3.2). |
| A-2 | Answered (E2). |
| A-3 | Answered (list K `kind:length:forms:text`). |
| A-4 | Answered (R8). |
| A-5 | Answered (X1). |
| A-6 | Y1 and exp.md 6.1 say a column declared `NUMBER` holds values of "39 and 40 significant digits" (`t_num` rows 30, 31). Row 30, `3.1415926535897932384626433832795028841`, has **38** significant digits (20 digit bytes, the first pair `03` and the last pair `10` each carry a zero that is not significant). Only row 31 (40 threes) exceeds 38. Please correct, or name the 39-digit value. |
| A-7 | E2 test (b) compares the u16 at p with the maximum length of column 1. For a first column of type 8 or 24 (descriptor length 0) every length passes, so (b) says nothing and E3 read literally would stop every non-empty such table on the direct path. Not in the fixtures (LONG on the direct path is R-031); arch 5.4 has a safe interim rule. Please cover it when R-031 is generated. |
| A-8 | Y9 says the LONG set is "suggested: the database set, like C1"; C4 forbids header bytes 1-2 for row data. Please say explicitly which id a reader should use for LONG (the design takes the database id of H4, the second u16 after the seventh line). |
| A-9 | R8 says "above 32,000 only `fe ff` and `ff ff` occur". As a statement of the fixtures that is right; as a rule it could be read as "a length of 32,001 to 0xffef is an error". The design reads such lengths (arch 5.4); please state that only 0 and 0xfff0 to 0xfffd are errors. |
| A-10 | H6 says the first item is `METRICST` "(table mode)" or `METRICSU` "(owner mode)". Is the pairing with the mode line of H3 confirmed (every fixture), or only observed? The design warns on a mismatch (ADR-17). Also Y1 "the first and last digit are never 0": "digit" means a base-100 digit (byte), not a decimal digit; worth one word. |

## (c) Risks, ranked

Session 4 ranking. Changed: the single-byte LOB risk is now a refusal per value, not a silent misreading
(ADR-25); a conventional LONG of 32 KB or more is a stop (R-040); NUMBER moves up (computed values are
ordinary data, [Y1] v3.4); consumer support of the Arrow types moves up (Decimal256, Large types for LONG,
MonthDayNano for DAY(9) intervals). Down: the direct-path LONG (confirmed), non-ASCII CLOB text in
AL32UTF8 (confirmed).

| # | risk | why it ranks here | mitigation in the design |
|---|---|---|---|
| 1 | The real dumps come from an `exp` 10.2 client against 11.2.0.4: **no rule is confirmed for them** (R-022). | It is the target input; every rule rests on same-version `exp` of 18c / 11.2.0.2. | Warnings instead of refusal where framing would fail loudly (Q-2); header checks as warnings (ADR-17); `UnsupportedLayout` with offsets; the structure-only `survey` (arch 8.5). |
| 2 | **Cyrillic LOB text of a single-byte database** (R-038): a long-form CLOB or any NCLOB. Session 8: **lowered** for database set 171 (confirmed on 11g; 18c assumed, ADR-25 session 8 item 6); unchanged for sets 1, 178 and others. | The owner's data are Cyrillic; long CLOB values are common. Before session 4 a wrong reading was silent; now such values are **refused** (`E-LOB-UNCONFIRMED`), so the risk is missing data, not wrong data, unless the owner opts into `unconfirmed_lob="decode"`. | ADR-25; the survey counts the affected values; R-038 (owner preparing) turns the refusal into a confirmed decoding; Q-7. Session 7: on R-038 only `lob_text_kind`, the unconfirmed check and `t6l_ru.rs` change (ADR-34 item 9). |
| 3 | NUMBER values that do not fit the column type (ADR-14, decided `round`). | [Y1] v3.4: every computed quotient or average in a `NUMBER` column has 39 / 40 digits; they arrive rounded to 10 decimals (counted); under `strict` they stop the job. Session 5: risk of a wrong policy gone; the rounding itself remains. | The survey's per-candidate counts per column; `decimal256(p,s)` and `string` overrides; integer overflow never rounded; declared `NUMBER` types no Decimal128 holds now warn (ADR-30). |
| 4 | (**session 25: lowered again**, R-048 and R-051 answered, ADR-99: every size reads; a field starting `fb ff` / `fc ff` stops, a value of 65,522 to 65,524 bytes reads with `W-UNTESTED-LAYOUT` by interpolation; the survey counts 65,521 to 65,534) (**session 24: lowered**, R-040 answered, ADR-95) A conventional LONG of 65,521 to 65,534 bytes whose length field is 0xfff1 to 0xfffc, or a conventional LONG RAW of 32,768 bytes or more (R-048). Before session 24: every conventional LONG / LONG RAW value of 32,768 bytes or more. | The file stops at that table (`UnsupportedLayout`): every later table of the dump is unreadable. Only if Q-4 shows such values. | The stop names the table, row, column and offset; earlier tables read; the survey prints the largest LONG length; R-048 (the LONG RAW half first). |
| 5 | Partitioned tables beyond the two 18c tables: other methods (list, interval, reference), partitioned tables with LOB columns (R-042), owner mode, 11g (11g XE has no partitioning). Session 6: LONG columns are excluded by Oracle (ORA-14085, R-045). | Large corporate tables are usually partitioned; the owner's database is 11.2.0.4. | One table with parts; each part checked against the first (`Framing` on a difference); `W-UNTESTED-LAYOUT` on any other line order; [E2] per part; the survey prints parts and counts. |
| 6 | Consumer support of the Arrow types: Decimal256 (if chosen by the owner) in Polars / DuckDB (native decimals stop at 38 digits; import not verified), Interval(MonthDayNano) (YEAR TO MONTH, DAY(9) TO SECOND) in pandas / Parquet (session 11: **pyarrow's Parquet writer refuses it**, verified on pyarrow 15.0.2 and 26.0.0), LargeUtf8 / LargeBinary for LONG in older consumers. | An ETL target may refuse a column type, failing a whole column. | T-6 / T-12 record pyarrow, Parquet, pandas, Polars, DuckDB behaviour on the pinned versions; Decimal256 is an override, not the default; session 11: `interval_as="string"` **in the alpha** (ADR-47, T-13). |
| 7 | LOB and LONG memory: a value is held whole. | A 1 GB BLOB or LONG is legal. | `max_lob_bytes` (also for LONG chains); `columns` skips values without holding them. |
| 8 | An unsupported type (BFILE, object, XML) still stops the file (ADR-12). | Rare in ETL schemas; unknown until Q-4. | `Catalog.end` names the table; earlier tables readable. |
| 9 | An introducing line the list lacks (R-018, [G2]). | Unchanged. | Unchanged. |
| 10 | A failed export looks like an empty table [X2]; empty parts on the direct path rest on two tests [E2, E3]; single-column LONG tables on [E2a] alone (R-041). | Wrong row count without an error. | `possibly_incomplete`; ADR-22 with a warning. |
| 11 | Raw manifest from the Analyst's tool (common-mode error), now with chains and composite parts. | Common-mode error. | Value-level checks against the truth files and the SQL. |
| 12 | Declared types outside [T9]; a column order other than the LONG-last one (ADR-24). Session 7: `NUMBER(*,s)` with s < 0 takes `number_default` (ADR-35). | Schema differs, never wrong values. | Fallback with `W-DDL-FALLBACK`; matching by name with `W-UNTESTED-LAYOUT`; integer overflow is an error under every policy. |
| 13 | `encoding_rs` against Oracle's map; crate method names read from documentation (UTF-16 without replacement, `Decimal256Builder`, `i256`). | Build surprises, rare bytes. | T-9 map test; T-4 / T-6 confirm names on the pinned versions. |
| 14 | Speed unmeasured; LOB and LONG values pass through streaming decoders. | Gigabyte targets. | T-15. |
| 15 | (session 5) Values the Arrow types cannot hold are refused per value: DATE / TIMESTAMP years below 1, INTERVAL YEAR(9) values beyond 178,956,970 years (ADR-28). | With the default `on_value_error="raise"` one such value stops the read of its table. Rare in ETL data; the fixtures hold them only as probes. | The survey counts them per column; `on_value_error="null"`; `interval_as="string"` moves into the alpha if the survey shows any; R-044. |
| 16 (**closed**, session 12: Q-9 answered, RHEL 8+ x86_64; the owner's install on RHEL 8 is checklist step 7) | (session 11) The wheel's glibc baseline is newer than the owner's ETL hosts. | `pip` refuses the wheel ("no matching distribution") without saying why. | ADR-48: `manylinux_2_28` built with `maturin --zig`; Q-9; the owner installs the wheel on a real host as the acceptance step. |
| 17 | (session 12; **session 22**: T-16b after a4, wheels in a5, ADR-87) Users off Linux x86_64 (Windows, macOS, aarch64, RHEL 7) have **no tested wheel** until T-16; a source build there is untested. | Corporate ETL hosts outside the owner's are possible; a build that compiles may still fail on paths, line endings or file locks. | README "Other platforms" (ADR-50): read on a supported host and write Parquet; T-16 with its acceptance rule (ADR-53 item 4) and `.gitattributes` (ADR-52) before any Windows run. |
| 18 | (session 13) The name `oradmp` is free on PyPI but **not secured** before the first upload; and an agent could read or install the other project `oradump` by mistake (a clean-room breach, ADR-54 item 3). | A taken name after the tag means renaming again (ADR-54's work twice); a clean-room breach taints the claim that no third-party parser code is in the project. | Q-10 upload part (the upload secures the name); ADR-54 item 3 and the owner's `CLAUDE.md` rule 1 sentence (O-13); T-13g checks the old import without installing anything; no agent fetches that project's pages. |
| 19 | (**closed**, session 16: Reviewer 4 read T-13h, no blocking finding; the a1 files stay internal) (session 14; **lowered** in session 15: a1 is internal only, the upload is a2 after Reviewer 4, ADR-63) 0.1.0a1 goes out with T-13h (stream state, panic guard, release script) read by no Reviewer (rule 7 groups T-13h with T-14 / T-15). | A lock mistake in the shared stream state deadlocks or loses warnings on the advertised path; an upload cannot be replaced. | T-13h's own mutations and the checklist's full test runs on the installed wheel; Q-12: a short Reviewer session on T-13h before any PyPI upload. |
| 20 | (session 15) On the Arrow C stream path an error is a `pyarrow.ArrowInvalid` with text only; the code is found by a pattern in the text (ADR-61). | An ETL job that catches `oradmp.OradmpError` around a pyarrow consumer misses a stream error, or parses the text wrongly; a later arrow-rs could change the wording around the code. | The iterator path stays typed; the README sentence (T-14 (i)) names the exception and the pattern; `r.warnings` is complete after the error; the stream tests assert the code after the prefix, so an arrow-rs bump that changes the text fails them; a typed `BatchReader.error` is "later" (ADR-55 item 6). |
| 21 | (session 15; **lowered** in session 16: 30 minutes on the final code, 0 crashes / timeouts / OOM; Reviewer 4 repeated 2 minutes per target with the same result) Fuzzing (T-14) finds panics, hangs or unbounded allocations in the framing or decoding code. | A fix there can change rows silently; a finding late in T-14 delays T-15, Reviewer 4 and the upload. | ADR-62 item 7: every finding gets a regression test; a fix that changes a code, a warning, a rule or any fixture's rows stops for the Architect; the full truth tests stay green; a1 users are internal and few. |
| 22 | (session 16; **closed for 0.1.0a2**, session 18: uploaded, hashes compared on TestPyPI, ADR-70; **closed for 0.1.0a3**, session 21: closure review of the exact candidate, TestPyPI first, index hashes equal, tag pushed last, ADR-81; a standing check for every later upload) The PyPI upload is irreversible: a wrong file (wrong version, a home path, a probe build, a README with a false install line) stays visible under its name; a leaked token lets anyone publish as the project. | First upload of the name; nothing published before. | ADR-67: closure review of the exact tree, `--twice` build, `check_dist.py`, TestPyPI upload with download-and-compare before PyPI, the owner alone handles the token (typed, never stored, project-scoped after the first upload). |
| 23 | (**closed**, session 19: the build was of `ff7d525`, the tag's commit; ADR-74) (session 18) The release 0.1.0a2 cannot be rebuilt from its tag: `v0.1.0a2` points to `ff7d525`, the build was of `4e8f212`, and the commit time is the build's `SOURCE_DATE_EPOCH`. | A later check of the uploaded files against a rebuild of the tag fails although nothing is wrong with the files; the record and the repository disagree. | O-18 (a): move the tag to the built commit; checklist step 9 now compares `git rev-parse v<v>^{commit}` with the build log. |
| 24 | (session 18) The command-line tool (T-18) is a new public surface: command names, options, exit codes, CSV text of each type. | ETL scripts will depend on exit codes and the CSV form; a later change breaks them silently. | ADR-72 fixes them in one record; exit codes by error class with a test over every code; types the CSV writer refuses stop with `E-CONFIG` instead of an unchosen text. Session 19: T-18 done with that walk and a mutation table; the one change before a3 (a missing table in a stopped scan, ADR-75 item 6) lands before the first release of the tool, so no user sees the old rule; `head` shows no counted warning (documented). Session 20: review 06a found no blocking finding; SF-2's survivors get tests and the I/O error line gains reason and path in T-18c, before the tool's first upload. Session 21: shipped in a3 as specified; the next changes of the surface (non-UTF-8 names accepted and written `\xHH`, ADR-82) only add cases that crashed before; no line that worked in a3 changes. |
| 25 | (session 21; **session 22**: fixed in T-18d, reviewed in 6b; output paths in T-17b; ships in a4) A dump whose file name is not valid UTF-8 (plausible for CP1251 names copied from Windows) ends with a Python traceback in `oradmp.open`, the tool and `survey`, in a2 and a3 (review 06a-closure N-5). | The owner's hosts may hold such names; the ETL job stops with an unexplained error. | T-18d (ADR-82): the binding takes a path; messages write the bytes as `\xHH`; tests with a CP1251 name on Linux; in a4. Workaround until then: rename the file. |
| 26 | (session 22) The synthetic generator (T-17) and the reader share one reading of the rules; a construct no fixture shows can pass in both and look confirmed (review 06b SF-1: E3, client pairs, TIMESTAMP(1-5, 7, 8), 18c with database 171). | A wrong common assumption would be tested "green" on thousands of files and be believed. | ADR-80 as amended: fixture-shown combinations only, plus two labelled `Unconfirmed` cases under a named rule; a test compares the generator's header choices with the 201 fixture headers; TIMESTAMP and 18c / 171 removed; R-047, R-017 as routes to evidence (ADR-86). Session 24: R-047 and R-017 answered; TIMESTAMP f 0 to 9 back, the 18c E1 label narrowed to owner mode, new label `LongChunkedBelow65535` (ADR-94, T-19). |
| 27 | (**closed**, session 26: the cut is done, checks 1 to 9 clean, ADR-105; Reviewer 6c repeats the residue checks) (session 22; **session 23**: steps in order with the [OWNER] items, "Q-14 steps (session 23)"; the public name collides with the private repository's) The one-time cut (Q-14): a scrub mistake publishes a password, a home path or an e-mail; or a release is built from a commit that the review did not read; or the archive's hashes cited in the documents become unresolvable. | Irreversible once pushed; the a4 candidate could fall on either side of the cut. | Q-14's steps and nine checks (snapshot by `git archive`, fixture checksums, `diff -r` against the archive commit, greps); the rule "never between the a4 version commit and the tag"; the HISTORY note; Reviewer 6c checks the cut if it came first (ADR-88 item 3). |
| 28 | (session 23; **session 26**: at S two CI runs gave one wheel and one sdist at the canonical root, the owner's scratch-root build the same sdist and another wheel, as predicted; claim unchanged, ADR-105 item 6) The wheel hash depends on the build root through the SBOM (ADR-90); a release built at another root, or a claim of reproducibility wider than the evidence, makes a later rebuild disagree with PyPI although nothing is wrong. | The first release after CI exists (a4) is the first whose hash can be cross-checked; a false claim in the README is public. | The canonical root and `--release` (T-16a2); `check_dist.py --build-root`; O-23 (a) decides the claim; the sdist cross-check stays a gate; the README says nothing until the confirmation. |
| 29 | (**closed**, session 25: 65,533 is `fa ff` + n, 65,534 chunked, `long4_exp_<v>`@0x42295, 0x522a6; ADR-99 item 7) (session 24) A conventional LONG of exactly 65,533 or 65,534 bytes, if `exp` writes it as one field, has the u16 `fd ff` (the chunk marker) or `fe ff` (NULL): the reader then reads it as a chunked value or a NULL and misframes the rest (R-048's prediction is that the switch lies below 65,533). | A silent misread is possible until a later `Framing`; most likely a `Framing` or a missing `00 00` row end within the row. | ADR-95: a chunked total below 65,535 warns (`W-UNTESTED-LAYOUT`); the row-end check of [R4]; R-048 asks for exactly these sizes. |
| 30 | (**closed**, session 28: T-20b's proof run turned every affected step red and Reviewer 6c found no failure left that turns a red check green; the five `echo "...$(...)"` lines (6c SF-1) go to T-21, ADR-109) (session 27) A CI step that pipes a command into `tee` without `pipefail` shows green when the command fails (`ci.yml` has no `defaults.run.shell`; GitHub runs `bash -e {0}`): `cargo deny`, test failures of nextest and pytest, the installed-wheel pytest, fuzz findings, fuzz clippy, `auditwheel show` (ADR-108 item 4). | A release built from a commit whose CI is green although a check failed; a fuzz finding never uploaded. | T-20b: `defaults: run: shell: bash` for the whole workflow, two constructs adjusted, a proof run with a deliberate failure in each affected step (ADR-108 item 5). Closed when T-20b's proof run is pasted and Reviewer 6c finds no hidden failure left. |
| 31 | (**closed**, session 29: did not happen; the owner's canonical-root build of C gave wheel `2e4c69a5...` and sdist `510f2e39...` = C's CI files, both builds identical, and PyPI lists exactly these two files; ADR-111 item 1) (session 28) The owner's build of C at the canonical root gives a wheel or sdist other than C's CI files (`2e4c69a5...`, `510f2e39...`), e.g. after a toolchain, `uv` or registry change on the VM since 2026-10-29; or the owner uploads files from another folder. | The uploaded files would be ones no review or CI run produced; PyPI file names cannot be reused. | Both hashes are gates before any upload (checklist step 6, ADR-110 item 2); the build pins rustc 1.99.0, maturin 1.15.0 and zig 0.17.0 and remaps the paths; the upload names the two files of `R/dist` and is compared against the index; TestPyPI first. |
| 32 | (session 29) The published a4 sdist does not build from source (a member missing from the archive, a build step that needs the git clone): no build of C's sdist from source is recorded (the a4 checklist's step 8 source build is not in the owner's facts; 6c and CI compared its bytes only; ADR-111 item 4). | Users without a matching wheel (other platforms, older glibc) install from the sdist; it is the README's fallback ("Other platforms"). | T-21 (k): one source install of the released sdist on the VM, `survey` on a copied fixture (ADR-112 item 5); T-16b adds the check to `extended`. Closed when T-21 (k) passes; a failure is a defect of a4 and makes the a5 fix decision (ADR-113 item 4). |

### Session 3 ranking (history)

Session 3 ranking. Moved down: the unsupported-type stop (LOB, INTERVAL, ROWID are read; only BFILE,
object, XML stop). New: LOB text beyond the fixtures (a silent misreading is possible, not a stop),
partitions beyond one 18c table, LOB memory.

| # | risk | why it ranks here | mitigation in the design |
|---|---|---|---|
| 1 | The real dumps come from an `exp` 10.2 client against 11.2.0.4: **no rule is confirmed for them** (R-022). | It is the target input; the LOB and partition layouts are confirmed on 18c / 11.2.0.2 `exp` only. | Warnings instead of refusal (Q-2); header checks as warnings (ADR-17); `UnsupportedLayout` with offsets for any LOB layout outside [L1 to L4]; the structure-only `survey` (arch 8.5). |
| 2 | **LOB character data beyond the fixtures**: a non-ASCII CLOB of an AL32UTF8 database (R-035), a long CLOB or any NCLOB of a CL8MSWIN1251 database (R-038). If the bytes are not what [L5] says, the text is wrong without an error. | The owner's data are Cyrillic: every CLOB value is non-ASCII; long values are common. | `W-LOB-CHARSET`, `W-LOB-UNTESTED` (strict: errors) once per column, counted by the survey; UTF-16 decoding is strict (odd lengths and lone surrogates are errors), so a single-byte text read as UTF-16 almost always fails loudly; R-038 / R-035 before the alpha by Q-4. |
| 3 | NUMBER values that do not fit the column type (ADR-14, Q-1). | Every `NUMBER` column without precision; 38-digit values are in the fixtures, 40 in one. | Unchanged: the owner's choice; counts per column; the survey's profile; R-032. |
| 4 | Partitioned tables beyond one range-partitioned 18c table: empty partitions, subpartitions, statistics (R-037); partitions on 11g cannot be generated (11g XE). | Large corporate tables are usually partitioned; the owner's database is 11.2.0.4. | One table with parts; every part checked against the first (`Framing` on a difference); `W-UNTESTED-LAYOUT` on unconfirmed orders; [E2] per part; the survey prints parts and counts. |
| 5 | LONG / LONG RAW beyond [Y9] (R-031). | A misreading is possible until a later `Framing`; a long-form LONG now stops with `Framing` (ADR-15). | Unchanged warnings; [E2a] with a warning (ADR-22). |
| 6 | LOB memory: a value is held whole; a table of large BLOBs makes large batches. | A 1 GB BLOB is legal in Oracle. | `max_lob_bytes` (default 256 MiB, value error above, refused before reading); `columns` skips LOB values without holding them; batches close after the row that reaches `batch_bytes`. |
| 7 | An unsupported type (BFILE, object, XML) still stops the file (ADR-12). | Rare in ETL schemas; unknown until Q-4. | `Catalog.end` names the table; earlier tables readable. |
| 8 | An introducing line the list lacks (R-018, [G2]). | Unchanged. | Unchanged. |
| 9 | A failed export looks like an empty table [X2]; empty tables on the direct path rest on two tests [E2, E3, E2a]. | Wrong row count without an error. | `possibly_incomplete`; ADR-22. |
| 10 | Raw manifest from the Analyst's tool (common-mode error), now with LOB values and parts. | Common-mode error. | Value-level checks against `truth_lob`, `truth_ru`, `truth_misc`, `truth_part` and the SQL. |
| 11 | Declared types outside [T9] (NOT NULL intervals: R-039; virtual columns). | Schema differs, never wrong values. | Column fallback with `W-DDL-FALLBACK`. |
| 12 | Consumer support of the chosen Arrow types: Interval(MonthDayNano) in pandas / Parquet (not verified), LargeUtf8 in older consumers. | ETL targets may refuse a column type. | T-6 / T-12 record what pyarrow does; `interval_as="string"` after the alpha. |
| 13 | `encoding_rs` against Oracle's map; crate facts read from documentation. | Unchanged. | T-9 map test; T-6 verifies the incremental builder writes. |
| 14 | Speed unmeasured; LOB values add a copy through the UTF-16 decoder. | Gigabyte targets. | T-15. |

### Session 2 ranking (history)

Session 2 ranking. Moved up: NUMBER values (now a known fact, ordinary data), LONG (read now, so its
unknowns can misread). Moved down: the single-byte maps ([C7] answered; the T-9 test decides) and the
column-list grammar ([T9] confirmed).

| # | risk | why it ranks here | mitigation in the design |
|---|---|---|---|
| 1 | The real dumps come from an `exp` 10.2 client against 11.2.0.4: **no rule is confirmed for them** (R-022). | It is the target input. | Warnings instead of refusal by default (Q-2, accepted); header checks as warnings (ADR-17); `extra_intro_lines` at run time; the structure-only `survey` next to `python/exp_survey.py` (arch 8.5); every error has an offset. |
| 2 | One unsupported column type (LOB, INTERVAL, ROWID, object) stops the file and hides every later table (ADR-12). | Likely in a corporate schema; unknown until Q-4 is answered. LONG no longer stops. | R-027, R-029 before the alpha if Q-4 says so; tables before the stop stay readable; `Catalog.end` names the stopped table. |
| 3 | NUMBER values that do not fit the column type: with `error` a job stops on ordinary data (stored quotients, 40 digits); with `round` values change in the 11th decimal and tiny values become 0 (ADR-14, Q-1). | Affects every `NUMBER` column without precision; the fixtures already hold such values. | The owner's choice; every change counted per column; the survey's NUMBER profile before the first run; declared columns stay exact; integer overflow is never rounded. R-032 shows how common 40-digit values are. |
| 4 | LONG / LONG RAW beyond [Y9]: a value above 65,519 bytes, the direct path, non-ASCII LONG (R-031). | A misreading, not a stop, is possible until a later `Framing` error. | `W-LONG-LENGTH` above 32,000 bytes, `W-LONG-DIRECT`, `W-LONG-CHARSET` (strict: errors); the survey counts long values; R-031 is order 2. |
| 5 | An introducing line the list lacks, followed by a field that G1 does not recognise (plain field of 8192 bytes or more; functions, packages: R-018, [G2]). | A misread is usually a later `Framing` error, but could in principle skip a table silently. | `Catalog.item_counts` counts G1 lines (owner Q-2) and the survey prints them; R-018 is open with the Analyst. |
| 6 | A failed export looks like an empty table [X2]; the two empty-table forms on the direct path rest on two tests [E2, E3]. | Wrong row count without an error. | `possibly_incomplete`; both tests of [E2] required, else stop; LONG first column handled without guessing (arch 5.4). |
| 7 | The raw manifest comes from the Analyst's exploration tool: reader and tool could share a misreading. T-3a regenerates it from the same tool for the LONG files. | Common-mode error. | The value-level check against `truth_*.txt` and, for LONG, against the SQL of `11_types.sql` is independent of the tool. |
| 8 | Declared types outside the 14 forms of [T9] in a real schema (`VARCHAR2(n BYTE)`, INTERVAL, virtual columns, exp.md 4.7). | Schema differs from what a user expects, never wrong values. | Fallback per column with `W-DDL-FALLBACK`; the survey prints the type texts. |
| 9 | `encoding_rs` might differ from Oracle's map in a byte the WHATWG index and [C7] agree on by assumption only. | Silent wrong characters, rare bytes. | T-9 compares all 768 lines; a differing set gets its own table (ADR-5). |
| 10 | Crate facts (versions of `pyo3`, `pyo3-arrow`, arrow-rs; GIL behaviour of pyarrow on the capsule path) were read from documentation pages, not tried. | Build surprises in T-12. | T-1 pinned and built the core; T-12 measures; ADR-3 names what to verify. |
| 11 | Speed is unmeasured; reading everything costs two passes (ADR-4). | Gigabyte targets. | T-15; a single-pass stream can be added on the same walker. |
