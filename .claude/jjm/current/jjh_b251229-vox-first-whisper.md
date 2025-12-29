# Heat: Vox First Whisper

Nucleate VOK (Vox Obscura Kit) as the foundational kit for Claude-powered infrastructure. Migrate JJK to depend on it.

## Paddock

### Vox Obscura - The Hidden Voice

VOK provides the arcane infrastructure that other kits depend on:
- Version tracking (Codex)
- Version identity (Sigil)
- Upstream filtering (Veil)
- Kit operations pattern (install, uninstall, check)

### VO Prefix Map

| Prefix | Name | Purpose |
|--------|------|---------|
| `voa_` | Arcanum | Main kit entry |
| `vox_` | Codex | Version ledger |
| `vos_` | Sigil | Version ID |
| `vov_` | Veil | Upstream filter |

### Reserved Suffixes (Claude Code Native Types)

All kits use these consistently:

| Suffix | Type |
|--------|------|
| `*a_` | Arcanum (kit main) |
| `*b_` | suBagent |
| `*c_` | slash Command |
| `*h_` | Hook |
| `*k_` | sKill |

### Kit Operations Pattern

| Pattern | Operation |
|---------|-----------|
| `*k-i` | Install |
| `*k-u` | Uninstall |
| `*k-c` | Check |

### Key Files

- `Tools/vok/voa_arcanum.sh` - main kit entry
- `Tools/vok/vox_codex.json` - version ledger
- `Tools/vok/README.md` - documentation

## Done

## Remaining

- **Create VOK skeleton**
  Create `Tools/vok/` directory with `voa_arcanum.sh` stub and `README.md` documenting the Vox Obscura concept and prefix conventions.

- **Implement Codex**
  Extract ledger machinery from JJK. Functions: `zvoa_compute_source_hash()`, `zvoa_lookup_sigil_by_hash()`. File: `vox_codex.json`.

- **Implement Sigil**
  Version ID computation and registration. Equivalent to JJK's "brand" concept.

- **Add kit operations**
  Implement `vok-i` (install), `vok-u` (uninstall), `vok-c` (check). Create tabtargets (`tt/vok-i.Install.sh`, `tt/vok-u.Uninstall.sh`, `tt/vok-c.Check.sh`).

- **Implement Veil**
  prep-pr upstream filtering. Config defines what's internal vs upstream-safe. Move from CMK.

- **Migrate JJK to VOK**
  Refactor JJK to call VOK's Codex instead of its own ledger machinery. JJK depends on VOK.

- **Document arcane vocabulary**
  Finalize README with full prefix conventions, reserved suffixes, and arcane term glossary.
