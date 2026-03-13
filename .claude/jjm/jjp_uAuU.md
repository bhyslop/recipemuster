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

## References

- ₣Ak (rbk-mvp-1-finalization) — retired trophy, prior art for all directory/spec/test work
- ₣As (rbk-mvp-2-iam-grant-unification) — racing, parallel IAM refactor
- Tools/rbk/rbf_Foundry.sh — summon, vouch, conjure operations
- Tools/rbk/rbq_Qualify.sh — qualification orchestrator (fast/release tiers)
- Tools/rbk/rbob_bottle.sh — bottle lifecycle, vouch gate
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — main specification
- scaleinv/recipebottle — open source destination repo