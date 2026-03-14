# Paddock: rbk-mvp-3-release-finalize

## Context

Harden the Recipe Bottle supply chain trust model and complete release-readiness tooling. MVP-1 (₣Ak, retired) established the core build/test/deploy pipeline, directory restructure (Tools/rbk/, vov_veiled), and specification foundations. MVP-2 (₣As, racing) is unifying IAM grant implementation. MVP-3 picks up the remaining release concerns: supply chain trust gaps, developer qualification tooling, and consumer-facing documentation.

## What MVP-1 Already Accomplished

- Renamed Tools/rbw/ to Tools/rbk/
- Moved 57 RBS* specs from lenses/ to Tools/rbk/vov_veiled/
- Deleted rbk_Coordinator.sh, cleaned rbw.workbench.mk references
- Established test sweep infrastructure and full qualification gate
- SLSA provenance verification via vouch pipeline

## MVP-3 Themes

**Supply chain trust hardening**: Summon now pulls -vouch artifact (landed). Remaining: make -vouch multi-platform (AUAAJ), then require local -vouch presence on every bottle start as unconditional preflight gate (AUAAK).

**Developer tooling**: Qualification split into fast and release tiers (AUAAI, done). Fast qualify (rbw-Qf) runs on bottle-start: tabtargets, colophons, nameplate preflight — no shellcheck. Release qualify (rbw-QR) adds shellcheck + complete test suite. Regime validation testbench added (AUAAE, done). Remaining: manifest digest simplification (AUAAF), consecration inspect command (AUAAL), regime variable completeness check (AUAAH).

**Release prep**: Consumer-facing documentation pipeline: consolidate lenses into veiled (AUAAO), consumer CLAUDE.md template (AUAAN), freshen cosmology and render index.html (AUAAP), getting-started README (AUAAQ). Final: prep-release slash command ceremony (AUAAR) that orchestrates qualification, branch mechanics, content stripping, CLAUDE.md swap, and marshal reset.

## BCG Compliance Audit Results (₢AUAAZ)

### Wave 1 (bd6eba19) — correctness/portability tier
- 5 pseudo-ternary `A && B || C` patterns → if/then/else or inverted tests
- 11 non-POSIX `[[ == ]]` → `test =`
- ~40 `[ ]` bracket tests → `test` command
- 2 `eval` → `${!name}` indirect expansion
- 2 `eval` hardened with variable name validation
- 6 `echo -e` → `printf '%b\n'`

### Wave 2 (627e1c0f) — structural/error-handling tier
- Renamed `zbuc_do_execute` → `zbuc_doc_mode_predicate` (non-predicate in if, 8 sites)
- `[[ ]]` → `test` in bash version checks (2 sites)
- Split combined `local z_var=$(cmd)` → two-line capture pattern
- Added `|| buc_die` to 10 unguarded mkdir/cp/rm/date commands
- `head -1` → `read -r` builtin (2 sites)
- Heredoc → echo sequence (1 site)
- `dirname` → `${var%/*}` parameter expansion (1 site)
- Added `readonly` to kindle array ZRBRR_DOCKER_ENV

### Remaining: command substitution elimination (~20 violations, 8 files)
BCG rule: NO `$(command)` except `$(<file)` and `_capture` functions.
Each needs temp file path (kindle constant), _capture wrapper, or builtin replacement.
Docket reslated with per-file inventory.

### Deferred Technical Debt (itch for post-release)
- **z_ prefix on locals**: 1,519 occurrences across 63 files — naming convention, no runtime impact
- **Unbraced expansions `"$var"`**: 198 occurrences across 49 files — mechanical
- **`2>/dev/null` stderr suppression**: 73 occurrences across 20 files — case-by-case judgment
- **`local -r` missing**: Pervasive — single-assignment locals without readonly
- **Raw echo for user messages**: 200+ in display-heavy modules (rbf inspect, rbj sentry, rbo observe)
- **Bare file truncation** (`> file` without `:`): 38 occurrences, 7 files

## References

- ₣Ak (rbk-mvp-1-finalization) — retired trophy, prior art for all directory/spec/test work
- ₣As (rbk-mvp-2-iam-grant-unification) — racing, parallel IAM refactor
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md — the authoritative guide
- Tools/rbk/rbf_Foundry.sh — summon, vouch, conjure operations
- Tools/rbk/rbq_Qualify.sh — qualification orchestrator (fast/release tiers)
- Tools/rbk/rbob_bottle.sh — bottle lifecycle, vouch gate
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — main specification
- scaleinv/recipebottle — open source destination repo