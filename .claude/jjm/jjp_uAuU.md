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

**Supply chain trust hardening**: Summon now pulls -vouch artifact (landed). Remaining: make -vouch multi-platform to eliminate arm64 pull warnings, then require local -vouch presence on every bottle start as an unconditional preflight gate.

**Developer tooling**: Qualification currently runs shellcheck (133 files) on every bottle-start and conjure. Split into fast (cheap checks for routine operations) and full (shellcheck + complete test suite for release). Add regime validation testbench.

**Release prep**: Simplify manifest digest extraction, design consumer CLAUDE.md guidance for open-source release, verify regime variable spec completeness.

## References

- ₣Ak (rbk-mvp-1-finalization) — retired trophy, prior art for all directory/spec/test work
- ₣As (rbk-mvp-2-iam-grant-unification) — racing, parallel IAM refactor
- Tools/rbk/rbf_Foundry.sh — summon, vouch, conjure operations
- Tools/rbk/rbq_Qualify.sh — qualification orchestrator
- Tools/rbk/rbob_bottle.sh — bottle lifecycle, vouch gate
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — main specification