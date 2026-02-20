# Paddock: vok-liturgical-linter

## Context

Build a linter that evaluates the health and consistency of naming across all liturgical domains. The goal is a simple command (`vvx_lint`) that scans the project and prints warnings for any inscription that violates its vesture rules or uses an unrecognized cipher.

**Why this matters:** Naming consistency enables tooling magic. Before you can do global search/replace, shadow AsciiDoc→Markdown emission, or legacy word hunts, you must be able to *detect and error on malformations*. This linter is the foundation.

**What this is:** A passive scanner that validates inscriptions against vesture rules. Single-threaded (IO-bound, small corpus). Reports warnings, does not transform.

**What this is NOT:**
- Content scanning (function names inside files, AsciiDoc attributes) — future work
- CMK semantic validation (linked term resolution, anchor existence) — separate tranche
- Any kind of transformation or code generation

## Authoritative Specifications

These AsciiDoc documents define the vocabulary and rules the linter must enforce:

| Document | Path | Defines |
|----------|------|---------|
| **VLS** | `Tools/vok/vov_veiled/VLS-VoxLiturgicalSpec.adoc` | Universal naming vocabulary: Cipher, Signet, Epithet, Inscription, Vesture. Defines the six domain vestures (Rust, Bash, AsciiDoc, Publication, Git, Slash). |
| **VOS0** | `Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc` | Kit distribution infrastructure. Defines `vosk_prefix_validation` premise — all assets must have signets derived from their kit's cipher. |
| **BUS** | `Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc` | Tabtarget dispatch vocabulary: Colophon, Frontispiece, Imprint, Formulary, Launcher. Defines the tabtarget vesture pattern. |

## Domains to Lint

From VLS, these vestures define the rules:

| Domain | Vesture | Signet Case | Separator | Epithet Case | Envelope | Location |
|--------|---------|-------------|-----------|--------------|----------|----------|
| Rust files | `vosldr_rust` | snake_case | `_` | PascalCase | — | `Tools/*/` |
| Bash files | `vosldb_bash` | lowercase | `_` | snake_case | `.sh` | `Tools/*/` |
| AsciiDoc attrs | `voslda_asciidoc` | lowercase | `_` | snake_case | — | inside `.adoc` |
| Publications | `vosldp_publication` | UPPERCASE | `-` | PascalCase | `.adoc`/`.md` | `lenses/`, `vov_veiled/` |
| Git refs | `vosldg_git` | lowercase | `/` | kebab-case | — | `.git/refs/` |
| Slash commands | `voslds_slash` | lowercase | `-` | kebab-case | `.md` | `.claude/commands/` |
| Tabtargets | (BUS) | — | `.` | PascalCase | `.sh` | `tt/` |

## Cipher Registry

Single source of truth: `Tools/vok/vof/src/vofc_registry.rs`

The linter must use this registry to:
1. Validate that inscription signets derive from known ciphers
2. Enforce terminal exclusivity (no cipher is prefix of another)

## Deferred Work

- Function name scanning within `.sh` files (public `{signet}_{name}()`, private `z{signet}_{name}()`)
- AsciiDoc attribute scanning within `.adoc` files
- CMK semantic layer (linked term resolution, anchor validation, annotation voices)
- Any transformation or fix-it capability

## Initial Scope

Start with filename linting only:
- Scan `Tools/`, `lenses/`, `tt/`, `.claude/commands/`
- Parse each filename, attempt to match a vesture
- Extract signet, validate against cipher registry
- Print warnings for: unknown ciphers, case violations, separator violations, terminal exclusivity failures

## References

- Prior art that collapsed: `/Users/bhyslop/projects/usi-UtilitiesScaleInvariant/SubtreeRoots/yeti-YourEditorToolsIntegration/yeti/builder.sh` (750 lines of bash associative array hell)
- Key insight: Rust has actual types. This problem is tractable with structs and enums.
- Session context: `vok-secret-plans` chat, 2026-01-21/22
