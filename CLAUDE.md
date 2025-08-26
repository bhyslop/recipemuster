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
- Related utilities: BCU, BDU, BTU, BVU in Tools/

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
- **RBAGS** → `rbw-RBAGS-AdminGoogleSpec.adoc` (Recipe Bottle Admin Google Spec)
- **RBP** → `rbw-RBP-planner.adoc` (Recipe Bottle Planner)
- **RBRN** → `rbw-RBRN-RegimeNameplate.adoc` (Recipe Bottle Regime Nameplate)
- **RBRR** → `rbw-RBRR-RegimeRepo.adoc` (Recipe Bottle Regime Repository)
- **RBS** → `rbw-RBS-Specification.adoc` (Recipe Bottle Specification)
- **CRR** → `crg-CRR-ConfigRegimeRequirements.adoc` (Config Regime Requirements)

### Tools Directory (`Tools/`)
- **RBF** → `rbf_Foundry.sh`, `rbf_cli.sh` (Recipe Bottle Foundry)
- **RBGA** → `rbga_ArtifactRegistry.sh`, `rbga_cli.sh` (Recipe Bottle Google Artifact Registry)
- **RBGB** → `rbgb_Buckets.sh`, `rbgb_cli.sh` (Recipe Bottle Google Buckets)
- **RBGC** → `rbgc_Constants.sh` (Recipe Bottle Google Constants)
- **RBGG** → `rbgg_Governor.sh`, `rbgg_cli.sh` (Recipe Bottle Google Governor)
- **RBGI** → `rbgi_GoogleIAM.sh` (Recipe Bottle Google IAM)
- **RBGM** → `rbgm_ManualProcedures.sh`, `rbgm_cli.sh` (Recipe Bottle Google Manual)
- **RBGO** → `rbgo_GoogleOAuth.sh` (Recipe Bottle Google OAuth)
- **RBGP** → `rbgp_Payor.sh`, `rbgp_cli.sh` (Recipe Bottle Google Payor)
- **RBGU** → `rbgu_GoogleUtility.sh` (Recipe Bottle Google Utility)
- **RBI** → `rbi_Image.sh` (Recipe Bottle Image)
- **RBK** → `rbk_Coordinator.sh` (Recipe Bottle Koordinator)
- **RBL** → `rbl_Locator.sh` (Recipe Bottle Locator)
- **RBV** → `rbv_PodmanVM.sh`, `rbv_cli.sh` (Recipe Bottle VM)
- **RGBS** → `rgbs_ServiceAccounts.sh`, `rgbs_cli.sh` (Recipe Google Business Service)
- **BCU** → `bcu_BashCommandUtility.sh` (Bash Command Utility)
- **BDU** → `bdu_BashDispatchUtility.sh` (Bash Dispatch Utility)
- **BTU** → `btu_BashTestUtility.sh` (Bash Test Utility)
- **BVU** → `bvu_BashValidationUtility.sh` (Bash Validation Utility)

### CNMP Lenses Directory (`../cnmp_CellNodeMessagePrototype/lenses/`)
- **ANCIENT** → `a-roe-ANCIENT.md`
- **ANNEAL** → `a-roe-ANNEAL-spec-fine.adoc`
- **CRAFT** → `a-roe-CRAFT-cmodel-format.adoc`
- **METAL** → `a-roe-METAL-sequences.adoc`
- **MIND** → `a-roe-MIND-cmodel-semantic.adoc`
- **BCG** → `bpu-BCG-BashConsoleGuide.md` (Bash Console Guide)
- **PCG** → `bpu-PCG-ProcedureCurationGuide-005.md` (Procedure Curation Guide)
- **JRR** → `jrr-JobRookRadar-sspec.adoc` (Job Rook Radar)
- **MBC** → `lens-mbc-MakefileBashConsole-cmodel.adoc` (Makefile Bash Console)
- **YAK** → `lens-yak-YetAnotherKludge-cmodel.adoc` (Yet Another Kludge)
- **MCM** → `mcm-MCM-MetaConceptModel.adoc` (Meta Concept Model)
- **M2C** → `mcm-M2C-ModelToClaudex.md` (Model To Claudex)
- **SRFC** → `srf-SRFC-StudyRaftConcepts.adoc` (Study Raft Concepts)
- **ABG** → `wrs-ABG-AccordBuilderGuide.md` (Accord Builder Guide)
- **ALTL** → `wrs-ALTL-AccordLogicalTaskLens.claudex` (Accord Logical Task Lens)
- **PMTL** → `wrs-PMTL-ProtocolMachineryTaskLens.claudex` (Protocol Machinery Task Lens)
- **SDTL** → `wrs-SDTL-ShapeDesignTaskLens.claudex` (Shape Design Task Lens)
- **TITL** → `wrs-TITL-TestInfrastructureTaskLens.claudex` (Test Infrastructure Task Lens)
- **TLG** → `wrs-TLG-TaskLensGuide.md` (Task Lens Guide)
- **WRC** → `wrs-WRC-WardRealmConcepts.adoc` (Ward Realm Concepts)

## Working Preferences
- When user mentions an acronym, immediately navigate to the corresponding file
- Assume full edit permissions for all files in the three main directories
- For bash scripts, prefer functional programming style with clear error handling
- For .adoc files, maintain consistent AsciiDoc formatting
- For .claudex files, preserve the specific format requirements

## Common Workflows
1. **Bash Development**: Start with relevant utility (BCU/BDU/BTU/BVU), check dependencies
2. **Requirements Writing**: Open spec file, review related documents in same directory
3. **Cross-Reference**: Many Tools/ scripts implement specs from ../recipebottle-admin/
4. **Testing**: BTU (Bash Test Utility) is the primary testing framework

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides