# BUK Utility Rename Effort

## Effort Description

Systematically rename the four Bash Utility Kit (BUK) utilities to follow consistent naming conventions:
- `bcu_BashCommandUtility.sh` → `buc_command.sh`
- `bdu_BashDispatchUtility.sh` → `bud_dispatch.sh`
- `btu_BashTestUtility.sh` → `but_test.sh`
- `bvu_BashValidationUtility.sh` → `buv_validation.sh`

This includes renaming all public functions, internal functions, and variables throughout the codebase.

**Total scope:** 74 shell scripts with ~2,180 references across the repository.

### Files by Category

#### Utility Files (4 files in Tools/buk/)
1. `Tools/buk/bcu_BashCommandUtility.sh` → rename to `buc_command.sh`
2. `Tools/buk/bdu_BashDispatchUtility.sh` → rename to `bud_dispatch.sh`
3. `Tools/buk/btu_BashTestUtility.sh` → rename to `but_test.sh`
4. `Tools/buk/bvu_BashValidationUtility.sh` → rename to `buv_validation.sh`

#### Tools/buk/ Consumer Files (3 files)
- `Tools/buk/burc_regime.sh` - sources bcu and bvu
- `Tools/buk/burs_regime.sh` - sources bcu and bvu
- `Tools/buk/buw_workbench.sh` - sources bcu

#### Launcher Files (3 files in .buk/)
- `.buk/launcher.buw_workbench.sh`
- `.buk/launcher.cccw_workbench.sh`
- `.buk/launcher.rbk_Coordinator.sh`

#### Tools/ccck/ (1 file)
- `Tools/ccck/cccw_workbench.sh`

#### Tools/rgbs/ (2 files)
- `Tools/rgbs_cli.sh`
- `Tools/rgbs_ServiceAccounts.sh`

#### Tools/rbw/ (27 files)
- CLI files: `rbf_cli.sh`, `rbga_cli.sh`, `rbgb_cli.sh`, `rbgg_cli.sh`, `rbgm_cli.sh`, `rbgp_cli.sh`, `rbv_cli.sh`
- Implementation files: `rbf_Foundry.sh`, `rbga_ArtifactRegistry.sh`, `rbgb_Buckets.sh`, `rbgc_Constants.sh`, `rbgd_DepotConstants.sh`, `rbgg_Governor.sh`, `rbgi_IAM.sh`, `rbgo_OAuth.sh`, `rbgp_Payor.sh`, `rbgu_Utility.sh`, `rbi_Image.sh`, `rbl_Locator.sh`, `rbv_PodmanVM.sh`
- Validator files: `rbre.validator.sh`, `rbrg.validator.sh`, `rbrn.validator.sh`, `rbrr.validator.sh`, `rbrs.validator.sh`, `rbrv.validator.sh`

#### Tools/test/ (2 files)
- `Tools/test/tbvu_suite_xname.sh`
- `Tools/test/trbim_suite.sh`

#### Tools/ABANDONED-github/ (6 files)
- `rbha_GithubActions.sh`
- `rbhcr_GithubContainerRegistry.sh`
- `rbhh_GithubHost.sh`
- `rbhim_GithubContainerRegistry.sh`
- `rbhr_cli.sh`
- `rbhr_GithubRemote.sh`

#### tt/ TabTargets (26 files)
TabTarget scripts in `tt/` that invoke coordinator scripts using BUK utilities. All files with prefixes: `gadcf-`, `gadi-`, `rbw-`

### Naming Mappings

#### Function Renames (85+ functions across 4 utilities)
- BCU: `bcu_*` → `buc_*` (21 public), `zbcu_*` → `zbuc_*` (11 internal)
- BDU: `zbdu_*` → `zbud_*` (9 internal)
- BTU: `btu_*` → `but_*` (9 public), `zbtu_*` → `zbut_*` (3 internal)
- BVU: `bvu_*` → `buv_*` (28 public)

#### Variable Renames (30+ variables)
- BCU: `ZBCU_*` → `ZBUC_*` (10 variables), `BCU_VERBOSE` → `BUC_VERBOSE`
- BDU: `BDU_*` → `BUD_*` (9 variables), `zBDU_*` → `zBUD_*`
- BTU: `BTU_*` → `BUT_*` (2 variables), `ZBTU_*` → `ZBUT_*` (5 variables)
- BVU: `ZBVU_INCLUDED` → `ZBUV_INCLUDED`

### Important Notes

Do not add comments identifying phases or groupings of steps. Comments like "# Phase 1:" or "# Step 2a:" create clutter and maintenance burden. The step numbering and structure already communicate the sequence clearly.

## Steps

- [x] Rename the 4 utility files in Tools/buk/ (bcu → buc, bdu → bud, btu → but, bvu → buv)

- [x] Rename all functions and variables within the 4 utility files (dispatch 4 parallel subagents):
  - [x] Update buc_command.sh (was bcu_BashCommandUtility.sh)
  - [x] Update bud_dispatch.sh (was bdu_BashDispatchUtility.sh)
  - [x] Update but_test.sh (was btu_BashTestUtility.sh)
  - [x] Update buv_validation.sh (was bvu_BashValidationUtility.sh)

- [x] Update all references across the repository (dispatch parallel subagents by directory):
  - [x] Update Tools/buk/ consumer files (3 files: burc_regime.sh, burs_regime.sh, buw_workbench.sh)
  - [x] Update .buk/ launcher files (3 files)
  - [x] Update Tools/ccck/cccw_workbench.sh
  - [x] Update Tools/rgbs/ files (2 files)
  - [x] Update Tools/rbw/ files (27 files)
  - [x] Update Tools/test/ files (2 files)
  - [x] Update Tools/ABANDONED-github/ files (6 files)
  - [x] Update tt/ TabTarget files (26 files)
