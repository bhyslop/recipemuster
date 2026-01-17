# Paddock: vok-fresh-install-release

## MVP Scope

Complete release/install/uninstall cycle. Single platform. Hardcoded CLAUDE.md templates (externalized post-MVP).

## Parcel Structure (established)

```
vvk-parcel-{hallmark}/
├── vvi_install.sh          # Bootstrap: detect platform, invoke vvx
├── vvbf_brand.json         # {vvbh_hallmark, vvbd_date, vvbs_sha, vvbc_commit, vvbk_kits}
└── kits/{kit}/             # Kit assets (excludes vov_veiled/)
    └── bin/vvx-{platform}  # Binary (vvk only)
```

## Operation Contracts

**vvx_emplace** (Rust, no git):
- Parse burc.env → BURC_TOOLS_DIR, BURC_PROJECT_ROOT
- Copy kits/* → ${BURC_TOOLS_DIR}/
- Route commands ({cipher}c-*.md) → .claude/commands/
- Freshen CLAUDE.md via voff_freshen()
- Copy brand → .vvk/vvbf_brand.json

**vvx_vacate** (Rust, no git):
- Read .vvk/vvbf_brand.json for kit list
- Remove kit directories
- Remove routed commands
- Collapse CLAUDE.md via voff_collapse()
- Remove brand file

**Bash wrappers** (vvi_install.sh, vvu_uninstall.sh) handle git commits before/after.

## CLAUDE.md Managed Sections

**Reminder**: CLAUDE.md content inside `<!-- MANAGED:{tag}:BEGIN/END -->` markers is overwritten on install. When modifying managed section content:

1. **MVP (hardcoded)**: Update constants in `vofm_managed.rs` (to be created in ₢AAAAF)
2. **Post-MVP (₢AAABK)**: Update template files `vo{cipher}mc_*.md` in kit's `vov_veiled/`

Content outside markers is user content — survives reinstall, no template update needed.

## References

- RCG: Tools/vok/vov_veiled/RCG-RustCodingGuide.md
- BCG: Tools/buk/vov_veiled/BCG-BashConsoleGuide.md
- VOS: Tools/vok/vov_veiled/VOS-VoxObscuraSpec.adoc
- voff_freshen: Tools/vok/vof/src/voff_freshen.rs
- vofc_registry: Tools/vok/vof/src/vofc_registry.rs

## Steeplechase Summary

- **260113**: Heat created. Decisions: all-Rust, all-or-none install, git-aware with Claude recovery.
- **260115**: Archive-based asset model chosen over embedded binaries.
- **260117**: Release implemented (₢AAAAE). Hallmark 1000 allocated. Paddock condensed for MVP focus.
