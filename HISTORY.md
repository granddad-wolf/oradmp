# History of this repository

The history of oradmp before 2026-10-26 is kept in a private archive. This repository starts with one commit, made
on 2026-10-26 from the last commit of that archive, `b55eb10376a9402aee36a1ee61779cf087c7bfb3` (P). The tree of
that first commit is the tree of P plus this file.

- Commit hashes cited in documents under `docs/` before 2026-10-26 name commits of the private archive, not of this
  repository. The documents are not rewritten.
- `0.1.0a1`, `0.1.0a2` and `0.1.0a3` were built from commits of the private archive
  (a1 `b7304d564de32fd6a152c550845c32798adc36dc`, a2 `ff7d52559551a973621c68154e03dba1a157731a`,
  a3 `ff3d6b25c360f93eeb8c1305927dd6cf3550610b`). Their tags exist only there, and
  `tools/release_build.sh <commit>` reproduces them only from there. From `0.1.0a4` on, every release is built
  from a commit of this repository.
- Changes made to the archive's text before P, so that it could become public: the owner's home path in some
  documents is `<home>`, the owner's e-mail address is `<owner e-mail>`, the password of the local test database
  user is read from outside the repository (`corpus/scripts/config.sh`, `corpus/sql/00_setup.sql`), and
  `CONTRIBUTING.md` no longer points to `CLAUDE.md`.
- Three paths of the archive are not part of this repository: `docs/prompts/` (the prompts of the agent sessions),
  `.claude/` (the agent definitions) and `CLAUDE.md` (the project rules). Documents under `docs/` that cite them
  refer to files that are kept privately.
