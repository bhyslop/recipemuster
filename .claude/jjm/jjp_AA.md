# Paddock: vok-fresh-install-release

## MVP Scope

Complete release/install/uninstall cycle. Single platform. Hardcoded CLAUDE.md templates (externalized post-MVP).

## Test Environment

- **Target repo**: /Users/bhyslop/projects/pb_paneboard02
- **Staging dir**: ../release-install-tarball/

**Iteration workflow** (from kit forge):
```bash
# 0. Notch to establish known state
/jjc-pace-notch

# 1. Build release (creates vvk-parcel-NNNN.tar.gz in kit forge)
tt/vow-R.Release.sh

# 2. Nuke staging (including hidden files), extract, delete ALL local tarballs
rm -rf ../release-install-tarball/.* ../release-install-tarball/* 2>/dev/null
tar -xzf vvk-parcel-*.tar.gz -C ../release-install-tarball/
rm -f vvk-parcel-*.tar.gz

# 3. Install to target (parameter is path to burc.env, NOT just target dir)
../release-install-tarball/vvi_install.sh /Users/bhyslop/projects/pb_paneboard02/.buk/burc.env

# 4. Uninstall from target
/Users/bhyslop/projects/pb_paneboard02/Tools/vvk/vvu_uninstall.sh
```

**Test discipline**: Continue as far as possible through steps 0-4. Diagnose any errors encountered. **STOP before editing kit forge code to fix** — report findings to human first.

## Parcel Structure (established)

```
vvk-parcel-{hallmark}/
├── vvi_install.sh          # Bootstrap: detect platform, invoke vvx
├── vvbf_brand.json         # {vvbh_hallmark, vvbd_date, vvbs_sha, vvbc_commit, vvbk_kits}
└── kits/{kit}/             # Kit assets (excludes vov_veiled/)
    └── bin/vvx-{platform}  # Binary (vvk only)
```

## Operation Contracts

**vvx_emplace** (Rust, with git commit):
- Parse burc.env → BURC_TOOLS_DIR, BURC_PROJECT_ROOT, BURC_MANAGED_KITS
- Validate exact match: parcel's vvbk_kits == BURC_MANAGED_KITS
- Nuclear cleanup: delete existing .vvk/ and kit directories
- Copy kits/* → ${BURC_TOOLS_DIR}/
- Route commands ({cipher}c-*.md) → .claude/commands/
- Freshen CLAUDE.md via voff_freshen()
- Copy brand → .vvk/vvbf_brand.json
- Git commit

**vvx_vacate** (Rust, with git commit):
- Parse burc.env (requires BURC_MANAGED_KITS for defense in depth)
- Read .vvk/vvbf_brand.json for kit list
- Remove routed commands/hooks
- Collapse CLAUDE.md via voff_collapse()
- Remove vvx binaries from Tools/vvk/bin/
- **Preserve** kit directories (unveiled content remains functional)
- Remove brand file and .vvk/
- Git commit

**Bash wrappers** (vvi_install.sh, vvu_uninstall.sh) are thin bootstraps that exec Rust.

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
- **260118**: BURC_MANAGED_KITS added for kit inventory control. Exact match validation on emplace prevents binary capability leakage. Vacate updated to preserve kit directories (unveiled content is open-source). Install to pb tested successfully (hallmark 1005). Orphaned CMK from previous install cleaned up manually. VOS updated to match implementation.

## Next Steps

1. Test uninstall/reinstall cycle on pb
2. Verify pb bash utilities remain functional after vacate
3. Test reinstall after vacate
