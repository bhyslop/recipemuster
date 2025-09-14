# Claude Code Project Memory

## Startup Menu
Please display this menu at the start of each session:

```
=== Quick Start Menu ===
bcbcg: Bash Coding with BCG (Bash Console Guide)
adocrbags: Requirements writing with RBAGS (Admin Google Spec)

Which would you like to start with? (Enter acronym)
```

## Preset Activities

### bcbcg: Bash Coding with BCG
- Primary file: `../cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`
- Context: Bash scripting and console development
- Related utilities: BCU, BDU, BTU, BVU in Tools/buk/

### adocrbags: Requirements writing with RBAGS
- Primary file: `../recipebottle-admin/rbw-RBAGS-AdminGoogleSpec.adoc`
- Context: Requirements documentation and specification writing
- Related specs: RBP, RBRN, RBRR, RBS, CRR in ../recipebottle-admin/

## Directory Permissions
Full read and edit access is pre-approved for all files in:
- `../recipebottle-admin/`
- `Tools/`
- `../cnmp_CellNodeMessagePrototype/lenses/`

## File Acronym Mappings

### Recipe Bottle Admin Directory (`../recipebottle-admin/`)
- **RBAGS** → `rbw-RBAGS-AdminGoogleSpec.adoc`
- **RBP**   → `rbw-RBP-planner.adoc`
- **RBRN**  → `rbw-RBRN-RegimeNameplate.adoc`
- **RBRR**  → `rbw-RBRR-RegimeRepo.adoc`
- **RBS**   → `rbw-RBS-Specification.adoc`
- **CRR**   → `crg-CRR-ConfigRegimeRequirements.adoc`
- **MPCR**  → `rbw-MPCR-MemoPayorCrisisRecovery.md`

### Tools Directory (`Tools/`)

#### RBW Subdirectory (`Tools/rbw/`)
- **RBF**  → `rbw/rbf_Foundry.sh`
- **RBGA** → `rbw/rbga_ArtifactRegistry.sh`
- **RBGB** → `rbw/rbgb_Buckets.sh`
- **RBGC** → `rbw/rbgc_Constants.sh`
- **RBGG** → `rbw/rbgg_Governor.sh`
- **RBGI** → `rbw/rbgi_IAM.sh`
- **RBGM** → `rbw/rbgm_ManualProcedures.sh`
- **RBGO** → `rbw/rbgo_OAuth.sh`
- **RBGP** → `rbw/rbgp_Payor.sh`
- **RBGU** → `rbw/rbgu_Utility.sh`
- **RBI**  → `rbw/rbi_Image.sh`
- **RBK**  → `rbw/rbk_Coordinator.sh`
- **RBL**  → `rbw/rbl_Locator.sh`
- **RBV**  → `rbw/rbv_PodmanVM.sh`

#### BUK Subdirectory (`Tools/buk/`)
- **BCU**  → `buk/bcu_BashCommandUtility.sh`
- **BDU**  → `buk/bdu_BashDispatchUtility.sh`
- **BTU**  → `buk/btu_BashTestUtility.sh`
- **BVU**  → `buk/bvu_BashValidationUtility.sh`

#### CCCK Subdirectory (`Tools/ccck/`)
- **CCCK** → `ccck/cccw_workbench.sh`

#### GAD Subdirectory (`Tools/gad/`)
- **GADF** → `gad/gadf_factory.py`
- **GADI** → `gad/gadi_inspector.html`

#### Other Tools
- **RGBS** → `rgbs_ServiceAccounts.sh`

### CNMP Lenses Directory (`../cnmp_CellNodeMessagePrototype/lenses/`)
- **ANCIENT** → `a-roe-ANCIENT.md`
- **ANNEAL**  → `a-roe-ANNEAL-spec-fine.adoc`
- **CRAFT**   → `a-roe-CRAFT-cmodel-format.adoc`
- **METAL**   → `a-roe-METAL-sequences.adoc`
- **MIND**    → `a-roe-MIND-cmodel-semantic.adoc`
- **BCG**     → `bpu-BCG-BashConsoleGuide.md`
- **PCG**     → `bpu-PCG-ProcedureCurationGuide-005.md`
- **JRR**     → `jrr-JobRookRadar-sspec.adoc`
- **MBC**     → `lens-mbc-MakefileBashConsole-cmodel.adoc`
- **YAK**     → `lens-yak-YetAnotherKludge-cmodel.adoc`
- **MCM**     → `mcm-MCM-MetaConceptModel.adoc`
- **M2C**     → `mcm-M2C-ModelToClaudex.md`
- **SRFC**    → `srf-SRFC-StudyRaftConcepts.adoc`
- **ABG**     → `wrs-ABG-AccordBuilderGuide.md`
- **ALTL**    → `wrs-ALTL-AccordLogicalTaskLens.claudex`
- **PMTL**    → `wrs-PMTL-ProtocolMachineryTaskLens.claudex`
- **SDTL**    → `wrs-SDTL-ShapeDesignTaskLens.claudex`
- **TITL**    → `wrs-TITL-TestInfrastructureTaskLens.claudex`
- **TLG**     → `wrs-TLG-TaskLensGuide.md`
- **WRC**     → `wrs-WRC-WardRealmConcepts.adoc`
- **GADS**    → `gad-GADS-GoogleAsciidocDifferSpecification.adoc`
- **GADP**    → `gad-GADP-GoogleAsciidocDifferPlanner.md`
- **GADMCR**  → `gad-GADMCR-MemoCorsResolution.md`
- **GADMRC**  → `gad-GADMRC-MemoRailCommit.html`
- **GADMDD**  → `gad-GADMDD-MemoDualDiffs.md`
- **GADMRHAI** → `gad-GADMRHAI-HtmlAlgoIssue.md`
- **GADMRHAO** → `gad-GADMRHAO-opus4p1Research.md`
- **GADMRHAP** → `gad-GADMRHAP-HtmlAlgoPrompt.md`
- **GADMRW**  → `gad-GADMRW-MemoRenderWickedFix.md`
- **GADMWP**  → `gad-GADMWP-MemoWebsocketPaths.md`
- **AXLA**    → `axl-AXLA-Lexicon.adoc`
- **AXMCM**   → `axl-AXMCM-ClaudeMarkConceptMemo.md`
- **WCC**     → `WCC-WebClaudetoClaudeCode.md`

## Working Preferences
- When user mentions an acronym, immediately navigate to the corresponding file
- Assume full edit permissions for all files in the three main directories
- For bash scripts, prefer functional programming style with clear error handling
- For .adoc files, maintain consistent AsciiDoc formatting
- For .claudex files, preserve the specific format requirements

### AsciiDoc Linked Terms
When working with .adoc files using MCM patterns:
- **Linked Term**: Concept with three parts:
  - Attribute reference: `:prefix_snake_case:` (mapping section)
  - Replacement text: `<<anchor,Display Text>>` (what readers see)
  - Definition: `[[anchor]] {attribute}:: Definition text` (meaning)
- Definitions may be grouped in lists or dispersed through document
- Maintain consistent prefix categories (e.g., `mcm_`, `rbw_`, `gad_`)
- Use snake_case for anchors, match attribute to anchor

## Common Workflows
1. **Bash Development**: Start with relevant utility (BCU/BDU/BTU/BVU), check dependencies
2. **Requirements Writing**: Open spec file, review related documents in same directory

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides