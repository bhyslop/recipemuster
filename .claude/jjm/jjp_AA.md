# Paddock: vok-fresh-install-release

## MVP Scope

Complete release/install/uninstall cycle. Single platform. Hardcoded CLAUDE.md templates (externalized post-MVP).

## Test Environment

- **Target repo**: /Users/bhyslop/projects/pb_paneboard02
- **Staging dir**: ../release-install-tarball/

**Iteration workflow** (from kit forge):
```bash
# 1. Build release (creates vvk-parcel-NNNN.tar.gz in kit forge)
tt/vow-R.Release.sh

# 2. Nuke and extract to staging
rm -rf ../release-install-tarball/* && tar -xzf vvk-parcel-*.tar.gz -C ../release-install-tarball/

# 3. Install to target
../release-install-tarball/vvi_install.sh /Users/bhyslop/projects/pb_paneboard02

# 4. Uninstall from target
/Users/bhyslop/projects/pb_paneboard02/Tools/vvk/vvu_uninstall.sh
```

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

**Reminder**: Content inside `<!-- MANAGED:{tag}:BEGIN/END -->` markers is overwritten on install.

**Template sources** (update these when modifying managed content):

| Section | MVP Source (₢AAAAF) | Post-MVP Source (₢AAABK) |
|---------|---------------------|--------------------------|
| BUK | `Tools/vok/vof/src/vofm_managed.rs` | `Tools/buk/vov_veiled/vocbumc_core.md` |
| CMK | `Tools/vok/vof/src/vofm_managed.rs` | `Tools/cmk/vov_veiled/voccmmc_core.md` |
| JJK | `Tools/vok/vof/src/vofm_managed.rs` | `Tools/jjk/vov_veiled/vocjjmc_core.md` |
| VVK | `Tools/vok/vof/src/vofm_managed.rs` | `Tools/vvk/vov_veiled/vocvvmc_core.md` |

Content outside markers is user content — survives reinstall.

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
