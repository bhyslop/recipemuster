# Claude Code Project Memory

## Startup Menu
Please display this menu at the start of each session:

```
=== Quick Start Menu ===
bcbcg: Bash Coding with BCG (Bash Console Guide)
adocrbags: Requirements writing with RBAGS (Admin Google Spec)
gad: Work on GAD (Google AsciiDoc Differ)

Which would you like to start with? (Enter acronym)
```

## Preset Activities

### bcbcg: Bash Coding with BCG
- Primary file: `../cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`
- Context: Bash scripting and console development
- Related utilities: BCU, BDU, BTU, BVU in Tools/buk/

### adocrbags: Requirements writing with RBAGS
- Primary file: `lenses/rbw-RBAGS-AdminGoogleSpec.adoc`
- Context: Requirements documentation and specification writing
- Related specs: RBRN, RBRR, RBS, CRR in lenses/

### gad: Work on GAD (Google AsciiDoc Differ)
- Primary directory: `Tools/gad/`
- Context: AsciiDoc diff visualization tool (spec, implementation, memos)
- See `Tools/gad/CLAUDE.md` for complete GAD acronym mappings and architecture

## Directory Permissions
Full read and edit access is pre-approved for all files in:
- `lenses/`
- `Tools/`
- `../cnmp_CellNodeMessagePrototype/lenses/`

## File Acronym Mappings

### Lenses Directory (`lenses/`)
- **RBAGS** → `rbw-RBAGS-AdminGoogleSpec.adoc`
- **RBRN**  → `rbw-RBRN-RegimeNameplate.adoc`
- **RBRR**  → `rbw-RBRR-RegimeRepo.adoc`
- **RBS**   → `rbw-RBS-Specification.adoc`
- **CRR**   → `crg-CRR-ConfigRegimeRequirements.adoc`

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
- **See `Tools/gad/CLAUDE.md` for complete GAD acronym mappings**
- Quick reference: GADF (factory), GADI* (inspector), GADS (spec), GADP (planner), GADM* (memos)

#### CMK Subdirectory (`Tools/cmk/`)
- **MCM**   → `cmk/mcm-MCM-MetaConceptModel.adoc`
- **AXLA**  → `cmk/axl-AXLA-Lexicon.adoc`
- **AXMCM** → `cmk/axl-AXMCM-ClaudeMarkConceptMemo.md`

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
- **M2C**     → `mcm-M2C-ModelToClaudex.md`
- **SRFC**    → `srf-SRFC-StudyRaftConcepts.adoc`
- **ABG**     → `wrs-ABG-AccordBuilderGuide.md`
- **ALTL**    → `wrs-ALTL-AccordLogicalTaskLens.claudex`
- **PMTL**    → `wrs-PMTL-ProtocolMachineryTaskLens.claudex`
- **SDTL**    → `wrs-SDTL-ShapeDesignTaskLens.claudex`
- **TITL**    → `wrs-TITL-TestInfrastructureTaskLens.claudex`
- **TLG**     → `wrs-TLG-TaskLensGuide.md`
- **WRC**     → `wrs-WRC-WardRealmConcepts.adoc`
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

## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Heat**: Bounded initiative (3-50 sessions), has paces. Location indicates state: `current/` (active), `pending/` (parked), `retired/` (done). Move to `current/` via prose when ready to work. Park in `pending/` via prose when blocked or deferring.
- **Pace**: Discrete action within a heat; mode is `manual` (human drives) or `delegated` (model drives from spec)
- **Itch**: Future idea, lives in Future or Shelved

- Target repo dir: `.`
- JJ Kit path: `Tools/jjk/job-jockey-kit.md`

**Available commands:**
- `/jja-heat-resume` - Resume current heat, show current pace
- `/jja-heat-retire` - Move completed heat to retired with datestamp
- `/jja-pace-find` - Show current pace (with mode)
- `/jja-pace-left` - List all remaining paces (with mode)
- `/jja-pace-add` - Add a new pace (defaults to manual)
- `/jja-pace-refine` - Refine pace spec, set mode (manual or delegated)
- `/jja-pace-delegate` - Execute a delegated pace
- `/jja-pace-wrap` - Mark pace complete
- `/jja-sync` - Commit and push JJ state and target repo
- `/jja-itch-list` - List all itches (future and shelved)
- `/jja-itch-find` - Find an itch by keyword
- `/jja-itch-move` - Move or promote an itch
- `/jja-doctor` - Validate Job Jockey setup

**Important**: New commands are not available in this installation session. You must restart Claude Code before the new commands become available.

## Concept Model Kit Configuration

Concept Model Kit (CMK) is installed for managing concept model documents.

**Configuration:**
- Lenses directory: `lenses/`
- Kit path: `Tools/cmk/concept-model-kit.md`
- Upstream remote: `OPEN_SOURCE_UPSTREAM`

**Concept Model Patterns:**
- **Linked Terms**: `{category_term}` - references defined vocabulary
- **Attribute References**: `:category_term: <<anchor,Display Text>>` - in mapping section
- **Anchors**: `[[anchor_name]]` - definition targets
- **Annotations**: `// ⟦content⟧` - Strachey brackets for type categorization

**Available commands:**
- `/cma-normalize` - Apply full MCM normalization (haiku)
- `/cma-render` - Transform to ClaudeMark (sonnet)
- `/cma-validate` - Check links and annotations
- `/cma-prep-pr` - Prepare upstream contribution
- `/cma-doctor` - Validate installation

For full MCM specification, see `Tools/cmk/mcm-MCM-MetaConceptModel.adoc`.

**Important**: Restart Claude Code session after installation for new commands to become available.

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides