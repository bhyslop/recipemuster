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
- Related utilities: BUC, BUD, BUT, BUV, BUW in Tools/buk/

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
- **RBWMBX** → `rbw-RBWMBX-BuildxMultiPlatformAuth.adoc` (Memo: Buildx multi-platform authentication research)

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
- **BUC**  → `buk/buc_command.sh` (command utilities, buc_* functions)
- **BUD**  → `buk/bud_dispatch.sh` (dispatch utilities, bud_* functions)
- **BUG**  → `buk/bug_guide.sh` (guide utilities, bug_* functions - always-visible user interaction)
- **BUT**  → `buk/but_test.sh` (test utilities, but_* functions)
- **BUV**  → `buk/buv_validation.sh` (validation utilities, buv_* functions)
- **BUW**  → `buk/buw_workbench.sh` (workbench utilities, buw_* functions)
- **BURC** → `buk/burc_cli.sh`, `buk/burc_regime.sh` (regime configuration)
- **BURS** → `buk/burs_cli.sh`, `buk/burs_regime.sh` (regime secrets)

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
1. **Bash Development**: Start with relevant utility (BUC/BUD/BUT/BUV/BUW), check dependencies
2. **Requirements Writing**: Open spec file, review related documents in same directory

## Concept Model Kit Configuration

Concept Model Kit (CMK) is installed for managing concept model documents.

**Configuration:**
- Lenses directory: `lenses`
- Kit path: `Tools/cmk/README.md`
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

**Subagents:**
- `cmsa-normalizer` - Haiku-enforced MCM normalization (text, mapping, validation)

For full MCM specification, see `Tools/cmk/mcm-MCM-MetaConceptModel.adoc`.

**Important**: Restart Claude Code session after installation for new commands and subagents to become available.

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides
- Public project page: https://scaleinv.github.io/recipebottle

## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Heat**: Bounded initiative with coherent goals that are clear and present (3-50 sessions). Location: `current/` (active) or `retired/` (done).
- **Pace**: Discrete action within a heat; can be armed for autonomous execution via `/jja-pace-arm`
- **Itch**: Future work (any detail level), lives in jji_itch.md
- **Scar**: Closed work with lessons learned, lives in jjs_scar.md

- Target repo dir: `.`
- JJ Kit path: `Tools/jjk/README.md`

**Available commands:**
- `/jja-heat-saddle` - Saddle up on heat at session start, analyze and propose approach
- `/jja-heat-retire` - Move completed heat to retired with datestamp
- `/jja-pace-new` - Add a new pace
- `/jja-pace-arm` - Validate pace spec and arm for autonomous execution
- `/jja-pace-fly` - Execute an armed pace autonomously
- `/jja-pace-wrap` - Mark pace complete, analyze next pace, propose approach
- `/jja-itch-add` - Add a new itch to the backlog
- `/jja-notch` - JJ-aware git commit, push, and re-engage with current pace

**Important**: New commands are not available in this installation session. You must restart Claude Code before the new commands become available.
