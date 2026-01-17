# Paddock: jjk-furlough-feature

## Context

Add heat lifecycle management: the ability to pause heats ("stabled") and resume them ("racing"), plus heat renaming.

**Problem:** With multiple concurrent heats, there's no way to temporarily set aside a heat while focusing on another. The current binary (current/retired) doesn't support "I'll come back to this later."

**Solution:** Introduce `jjx_furlough` command with:
- `--stabled` / `--racing` flags to toggle heat status
- `--silks` flag to rename heats
- Lazy migration from "current" → "racing" terminology

**Behavior changes:**
- `mount` filters to racing heats only (execution focus)
- `groom`, `slate`, `reslate`, `restring` show all heats (planning can happen on stabled heats)
- `saddle` fails on stabled heats (can't execute work on paused heat)

## Architecture

Three-pace arc following standard JJK feature pattern:
1. **Spec** (₢ACAAA) — Update JJD-GallopsData.adoc with new concepts
2. **Impl** (₢ACAAB) — Rust implementation in jjrg/jjrx/jjrq modules
3. **Commands** (₢ACAAC) — Slash command + updates to existing commands

## References

- Tools/jjk/vov_veiled/JJD-GallopsData.adoc — JJD spec (target for pace 1)
- Tools/vok/vov_veiled/RCG-RustCodingGuide.md — Rust naming conventions for new code
- Tools/jjk/vov_veiled/src/jjrg_gallops.rs — Heat/Pace structs, HeatStatus enum
- Tools/jjk/vov_veiled/src/jjrx_cli.rs — CLI command dispatch
- Tools/jjk/vov_veiled/src/jjrq_query.rs — Query operations (saddle, muster)

## Key Constraints

1. **Lazy migration** — Accept "current" on read, write "racing". No batch migration needed.
2. **Serde compatibility** — Use `#[serde(alias = "current")]` for backwards compat.
3. **RCG compliance** — New functions use `jjrg_`, `jjrx_`, `jjrq_` prefixes per module.

## Steeplechase

### 2026-01-17 - Heat Created

Restrung 3 furlough paces from ₣AA (vok-fresh-install-release) to focus that heat on MVP delivery.
