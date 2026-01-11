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
- Primary file: `Tools/buk/lenses/BCG-BashConsoleGuide.md`
- Context: Bash scripting and console development
- Related utilities: BUC, BUD, BUT, BUV, BUW in Tools/buk/

### adocrbags: Requirements writing with RBAGS
- Primary file: `lenses/RBAGS-AdminGoogleSpec.adoc`
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
- **RBAGS** → `RBAGS-AdminGoogleSpec.adoc`
- **RBRN**  → `RBRN-RegimeNameplate.adoc`
- **RBRR**  → `RBRR-RegimeRepo.adoc`
- **RBS**   → `RBS-Specification.adoc`
- **CRR**   → `CRR-ConfigRegimeRequirements.adoc`
- **RBSDC** → `RBSDC-depot_create.adoc`
- **RBSDD** → `RBSDD-depot_destroy.adoc`
- **RBSDI** → `RBSDI-director_create.adoc`
- **RBSDL** → `RBSDL-depot_list.adoc`
- **RBSGR** → `RBSGR-governor_reset.adoc`
- **RBSGS** → `RBSGS-GettingStarted.adoc`
- **RBSID** → `RBSID-image_delete.adoc`
- **RBSIR** → `RBSIR-image_retrieve.adoc`
- **RBSOB** → `RBSOB-oci_layout_bridge.adoc`
- **RBSPE** → `RBSPE-payor_establish.adoc`
- **RBSPI** → `RBSPI-payor_install.adoc`
- **RBSPR** → `RBSPR-payor_refresh.adoc`
- **RBSRC** → `RBSRC-retriever_create.adoc`
- **RBSSD** → `RBSSD-sa_delete.adoc`
- **RBSSL** → `RBSSL-sa_list.adoc`
- **RBSTB** → `RBSTB-trigger_build.adoc`
- **RBWMBX** → `RBWMBX-BuildxMultiPlatformAuth.adoc` (Memo: Buildx multi-platform authentication research)

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
- **RBOB** → `rbw/rbob_bottle.sh`
- **RBV**  → `rbw/rbv_PodmanVM.sh`

#### BUK Subdirectory (`Tools/buk/`)
- **BCG**  → `buk/lenses/BCG-BashConsoleGuide.md` (Bash Console Guide - enterprise bash patterns)
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
- **MCM**   → `cmk/MCM-MetaConceptModel.adoc`
- **AXLA**  → `cmk/AXLA-Lexicon.adoc`
- **AXMCM** → `cmk/AXMCM-ClaudeMarkConceptMemo.md`

#### Other Tools
- **RGBS** → `rgbs_ServiceAccounts.sh`

### CNMP Lenses Directory (`../cnmp_CellNodeMessagePrototype/lenses/`)
- **ANCIENT** → `a-roe-ANCIENT.md`
- **ANNEAL**  → `a-roe-ANNEAL-spec-fine.adoc`
- **CRAFT**   → `a-roe-CRAFT-cmodel-format.adoc`
- **METAL**   → `a-roe-METAL-sequences.adoc`
- **MIND**    → `a-roe-MIND-cmodel-semantic.adoc`
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

## Prefix Naming Discipline ("mint")

When asked to "mint" names, apply these rules. Full study: `Memos/memo-20260110-acronym-selection-study.md`

### Two Universes

**Primary Universe** — code, docs, functions, variables, attributes, anchors, directories. Prefixes must be globally unique and respect terminal exclusivity.

**Tabtarget Universe** — launchers in `tt/`. These are *colophons* referencing the primary universe. `rbw-` points to the `rbw` workbench; it doesn't consume new prefix space.

### Core Rules

**Rule 1 - Project Prefix**: Names start with 2-3 char project ID:
`rb` (Recipe Bottle), `gad` (GAD), `bu` (BUK), `jj` (Job Jockey), `pb` (Paneboard), `mcm`/`axl` (CMK), `crg`, `wrs`, `srf`

**Rule 2 - Terminal Exclusivity**: A prefix either IS a name or HAS children, never both.
- `rbg` has children (`rbga`, `rbgb`...) → `rbg` cannot name a thing
- `rbi` names Image module → `rbia`, `rbib` forbidden

### Primary Universe Patterns

| Domain | Pattern | Example |
|--------|---------|---------|
| Code files | `prefix_Word.ext` | `rbga_ArtifactRegistry.sh` |
| Doc files | `ACRONYM-Words.ext` | `RBAGS-AdminGoogleSpec.adoc` |
| Functions (public) | `prefix_name()` | `buc_log_args()` |
| Functions (private) | `zprefix_name()` | `zbuc_color()` |
| Variables | `PREFIX_NAME` | `BURC_PROJECT_ROOT` |
| AsciiDoc attributes | `:prefix_term:` | `:rbw_depot:` |
| AsciiDoc anchors | `[[prefix_term]]` | `[[rbw_depot]]` |
| Directories | `prefix/` | `Tools/buk/` |

### Tabtarget Universe Pattern

| Domain | Pattern | Example |
|--------|---------|---------|
| Launchers | `prefix-HumanName.sh` | `rbw-a.AccountInfo.sh` |

The hyphen is part of the colophon prefix (`rbw-`), not a separator.

### Minting Workflow

Before minting new prefixes, verify against existing trees via search or the memo to preserve terminal exclusivity.

### Project Prefix Registry

| Prefix | Project |
|--------|---------|
| `rb` | Recipe Bottle |
| `gad` | GAD (Google AsciiDoc Differ) |
| `bu` | BUK (Bash Utilities Kit) |
| `jj` | Job Jockey |
| `pb` | Paneboard |
| `mcm`, `axl` | CMK (Concept Model Kit) |
| `crg` | Config Regime |
| `wrs` | Ward Realm System |
| `srf` | Study Raft |

For expanded prefix trees within each project, see **File Acronym Mappings** above.

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

For full MCM specification, see `Tools/cmk/MCM-MetaConceptModel.adoc`.

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
