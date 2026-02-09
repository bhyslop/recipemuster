# Paddock: rbw-mvp-release-finalize

## Context

Prepare Recipe Bottle for MVP release by resolving structural debt and open questions.

## Key Constraints

- `rbk_Coordinator.sh` must be fully deleted. Anything still useful moves into `rbw_workbench.sh`. This clears the `rbk` prefix for use as a kit directory (`Tools/rbk/`).
- `rbw.workbench.mk` is gone (already removed or vestigial — confirm and clean up any references).
- RBS concept model specs move from legacy `lenses/` to `Tools/rbk/vov_veiled/`, following the veiled pattern established by JJK, BUK, VOK, and CMK.

## References

- VOS-VoxObscuraSpec.adoc `vosp_veiled` (veiled concept)
- Existing kit veiled dirs: `Tools/jjk/vov_veiled/`, `Tools/buk/vov_veiled/`, `Tools/vok/vov_veiled/`, `Tools/cmk/vov_veiled/`
- 37 RBS* files currently in `lenses/`