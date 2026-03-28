# Claude Code Project Memory

## Directory Permissions
Full read and edit access is pre-approved for all files in:
- `Tools/`
- `Memos/`
- `../cnmp_CellNodeMessagePrototype/lenses/`

## File Acronym Mappings

### Tools Directory (`Tools/`)

#### RBK Subdirectory (`Tools/rbk/`)
- **RBDC** → `rbk/rbdc_DerivedConstants.sh`
- **RBF**  → `rbk/rbf_Foundry.sh`
- **RBGA** → `rbk/rbga_ArtifactRegistry.sh`
- **RBGB** → `rbk/rbgb_Buckets.sh`
- **RBGC** → `rbk/rbgc_Constants.sh`
- **RBGG** → `rbk/rbgg_Governor.sh`
- **RBGI** → `rbk/rbgi_IAM.sh`
- **RBGM** → `rbk/rbgm_ManualProcedures.sh`
- **RBGO** → `rbk/rbgo_OAuth.sh`
- **RBGP** → `rbk/rbgp_Payor.sh`
- **RBGU** → `rbk/rbgu_Utility.sh`
- **RBI**  → `rbk/rbi_Image.sh`
- **RBJ**  → `rbk/rbj_sentry.sh` (Jailer - sentry container security setup: iptables, dnsmasq, enclave network)
- **RBLM** → `rbk/rblm_cli.sh` (Lifecycle Marshal - reset regime to blank template, duplicate repo for release testing)
- **RBOB** → `rbk/rbob_bottle.sh`
- **RBQ**  → `rbk/rbq_Qualify.sh` (Qualification orchestrator - tabtarget/colophon/nameplate health)
- **RBV**  → `rbk/rbv_PodmanVM.sh`
- **RBS0** → `rbk/vov_veiled/RBS0-SpecTop.adoc`
- **RBSAA** → `rbk/vov_veiled/RBSAA-ark_abjure.adoc`
- **RBSAB** → `rbk/vov_veiled/RBSAB-ark_about.adoc` (Standalone Cloud Build about pipeline - syft SBOM + mode-aware build_info)
- **RBSAC** → `rbk/vov_veiled/RBSAC-ark_conjure.adoc`
- **RBSAG** → `rbk/vov_veiled/RBSAG-ark_graft.adoc` (Graft operation - local image push to GAR)
- **RBSAI** → `rbk/vov_veiled/RBSAI-ark_inspect.adoc`
- **RBSAJ** → `rbk/vov_veiled/RBSAJ-access_jwt_probe.adoc`
- **RBSAO** → `rbk/vov_veiled/RBSAO-access_oauth_probe.adoc`
- **RBSAS** → `rbk/vov_veiled/RBSAS-ark_summon.adoc`
- **RBSAV** → `rbk/vov_veiled/RBSAV-ark_vouch.adoc`
- **RBSAX** → `rbk/vov_veiled/RBSAX-access_setup.adoc`
- **RBSBC** → `rbk/vov_veiled/RBSBC-bottle_create.adoc`
- **RBSBK** → `rbk/vov_veiled/RBSBK-bottle_cleanup.adoc`
- **RBSBL** → `rbk/vov_veiled/RBSBL-bottle_launch.adoc`
- **RBSBR** → `rbk/vov_veiled/RBSBR-bottle_run.adoc`
- **RBSBS** → `rbk/vov_veiled/RBSBS-bottle_start.adoc`
- **RBSCB** → `rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc` (Cloud Build security posture and deferred hardening)
- **RBSCE** → `rbk/vov_veiled/RBSCE-command_exec.adoc`
- **RBSCK** → `rbk/vov_veiled/RBSCK-consecration_check.adoc` (Consecration Check - registry ark inventory with health status)
- **RBSCIG** → `rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc` (IAM Grant API Contracts - verified behavioral contracts per resource type)
- **RBSCIP** → `rbk/vov_veiled/RBSCIP-IamPropagation.adoc`
- **RBSCJ** → `rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc`
- **RBSCO** → `rbk/vov_veiled/RBSCO-CosmologyIntro.adoc`
- **RBSCTD** → `rbk/vov_veiled/RBSCTD-CloudBuildTriggerDispatch.adoc`
- **RBSDC** → `rbk/vov_veiled/RBSDC-depot_create.adoc`
- **RBSDD** → `rbk/vov_veiled/RBSDD-depot_destroy.adoc`
- **RBSDI** → `rbk/vov_veiled/RBSDI-director_create.adoc`
- **RBSDL** → `rbk/vov_veiled/RBSDL-depot_list.adoc`
- **RBSDN** → `rbk/vov_veiled/RBSDN-depot_initialize.adoc`
- **RBSDS** → `rbk/vov_veiled/RBSDS-dns_step.adoc`
- **RBSDV** → `rbk/vov_veiled/RBSDV-director_vouch.adoc`
- **RBSGD** → `rbk/vov_veiled/RBSGD-gdc_establish.adoc`
- **RBSGR** → `rbk/vov_veiled/RBSGR-governor_reset.adoc`
- **RBSGS** → `rbk/vov_veiled/RBSGS-GettingStarted.adoc`
- **RBSHR** → `rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` (Horizon Roadmap - single collection point for defined-but-unscoped future work)
- **RBSID** → `rbk/vov_veiled/RBSID-image_delete.adoc`
- **RBSII** → `rbk/vov_veiled/RBSII-iptables_init.adoc`
- **RBSIP** → `rbk/vov_veiled/RBSIP-ifrit_pentester.adoc` (Ifrit Pentester — adversarial AI escape testing framework)
- **RBSIR** → `rbk/vov_veiled/RBSIR-image_retrieve.adoc`
- **RBSNC** → `rbk/vov_veiled/RBSNC-network_create.adoc`
- **RBSNX** → `rbk/vov_veiled/RBSNX-network_connect.adoc`
- **RBSOB** → `rbk/vov_veiled/RBSOB-oci_layout_bridge.adoc`
- **RBSPE** → `rbk/vov_veiled/RBSPE-payor_establish.adoc`
- **RBSPI** → `rbk/vov_veiled/RBSPI-payor_install.adoc`
- **RBSPR** → `rbk/vov_veiled/RBSPR-payor_refresh.adoc`
- **RBSPT** → `rbk/vov_veiled/RBSPT-port_setup.adoc`
- **RBSPV** → `rbk/vov_veiled/RBSPV-PodmanVmSupplyChain.adoc`
- **RBSQB** → `rbk/vov_veiled/RBSQB-quota_build.adoc`
- **RBSRA** → `rbk/vov_veiled/RBSRA-CredentialFormat.adoc`
- **RBSRC** → `rbk/vov_veiled/RBSRC-retriever_create.adoc`
- **RBSRG** → `rbk/vov_veiled/RBSRG-RegimeGcbPins.adoc`
- **RBSRI** → `rbk/vov_veiled/RBSRI-rubric_inscribe.adoc`
- **RBRN**  → `rbk/vov_veiled/RBRN-RegimeNameplate.adoc`
- **RBSRM** → `rbk/vov_veiled/RBSRM-RegimeMachine.adoc`
- **RBSRO** → `rbk/vov_veiled/RBSRO-RegimeOauth.adoc`
- **RBSRP** → `rbk/vov_veiled/RBSRP-RegimePayor.adoc`
- **RBSRR** → `rbk/vov_veiled/RBSRR-RegimeRepo.adoc`
- **RBSRS** → `rbk/vov_veiled/RBSRS-RegimeStation.adoc`
- **RBSRV** → `rbk/vov_veiled/RBSRV-RegimeVessel.adoc`
- **RBSSC** → `rbk/vov_veiled/RBSSC-security_config.adoc`
- **RBSSD** → `rbk/vov_veiled/RBSSD-sa_delete.adoc`
- **RBSSL** → `rbk/vov_veiled/RBSSL-sa_list.adoc`
- **RBSSR** → `rbk/vov_veiled/RBSSR-sentry_run.adoc`
- **RBSSS** → `rbk/vov_veiled/RBSSS-sentry_start.adoc`
- **RBSTB** → `rbk/vov_veiled/RBSTB-trigger_build.adoc`

#### BUK Subdirectory (`Tools/buk/`)
- **BCG**  → `buk/vov_veiled/BCG-BashConsoleGuide.md` (Bash Console Guide - enterprise bash patterns)
- **BUS0** → `buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` (Bash Utilities Specification - tabtarget dispatch vocabulary)
- **BUC**  → `buk/buc_command.sh` (command utilities, buc_* functions)
- **BUD**  → `buk/bud_dispatch.sh` (dispatch utilities, zbud_* functions)
- **BUG**  → `buk/bug_guide.sh` (guide utilities, bug_* functions - always-visible user interaction)
- **BUT**  → `buk/but_test.sh` (test utilities, but_* functions)
- **BUV**  → `buk/buv_validation.sh` (validation utilities, buv_* functions)
- **BUW**  → `buk/buw_workbench.sh` (workbench utilities, buw_* functions)
- **BURC** → `buk/burc_cli.sh`, `buk/burc_regime.sh` (regime configuration)
- **BURS** → `buk/burs_cli.sh`, `buk/burs_regime.sh` (regime station)

#### CCCK Subdirectory (`Tools/ccck/`)
- **CCCK** → `ccck/cccw_workbench.sh`

#### GAD Subdirectory (`Tools/gad/`)
- **See `Tools/gad/CLAUDE.md` for complete GAD acronym mappings**
- Quick reference: GADF (factory), GADI* (inspector), GADS (spec), GADP (planner), GADM* (memos)

#### CMK Subdirectory (`Tools/cmk/`)
- **MCM**   → `cmk/vov_veiled/MCM-MetaConceptModel.adoc`
- **AXLA**  → `cmk/vov_veiled/AXLA-Lexicon.adoc`
- **AXMCM** → `cmk/vov_veiled/AXMCM-ClaudeMarkConceptMemo.md`

#### JJK Subdirectory (`Tools/jjk/`)
- **JJS0** → `jjk/vov_veiled/JJS0_JobJockeySpec.adoc` (Job Jockey specification - main file)
- **JJSCCH** → `jjk/vov_veiled/JJSCCH-chalk.adoc`
- **JJSCCU** → `jjk/vov_veiled/JJSCCU-curry.adoc` (Paddock operation - read/write heat paddock files)
- **JJSCDR** → `jjk/vov_veiled/JJSCDR-draft.adoc`
- **JJSCFU** → `jjk/vov_veiled/JJSCFU-furlough.adoc`
- **JJSCMU** → `jjk/vov_veiled/JJSCMU-muster.adoc`
- **JJSCNC** → `jjk/vov_veiled/JJSCNC-notch.adoc`
- **JJSCNO** → `jjk/vov_veiled/JJSCNO-nominate.adoc`
- **JJSCPD** → `jjk/vov_veiled/JJSCPD-parade.adoc`
- **JJSCRL** → `jjk/vov_veiled/JJSCRL-rail.adoc`
- **JJSCRN** → `jjk/vov_veiled/JJSCRN-rein.adoc`
- **JJSCRT** → `jjk/vov_veiled/JJSCRT-retire.adoc`
- **JJSCSC** → `jjk/vov_veiled/JJSCSC-scout.adoc`
- **JJSCSD** → `jjk/vov_veiled/JJSCSD-saddle.adoc`
- **JJSCSL** → `jjk/vov_veiled/JJSCSL-slate.adoc`
- **JJSCTL** → `jjk/vov_veiled/JJSCTL-tally.adoc`
- **JJSCVL** → `jjk/vov_veiled/JJSCVL-validate.adoc`
- **JJSCWP** → `jjk/vov_veiled/JJSCWP-wrap.adoc` (Close/wrap operation - mark pace complete and commit)
- **JJSRLD** → `jjk/vov_veiled/JJSRLD-load.adoc`
- **JJSRPS** → `jjk/vov_veiled/JJSRPS-persist.adoc`
- **JJSRSV** → `jjk/vov_veiled/JJSRSV-save.adoc`
- **JJSRWP** → `jjk/vov_veiled/JJSRWP-wrap.adoc`
- **JJA**  → `jjk/jja_arcanum.sh` (arcanum - core internal functions)
- **JJW**  → `jjk/jjw_workbench.sh` (workbench)

#### VOK Subdirectory (`Tools/vok/`)
- **RCG**  → `vok/vov_veiled/RCG-RustCodingGuide.md` (Rust Coding Guide - project Rust conventions)
- **VLS**  → `vok/vov_veiled/VLS-VoxLiturgicalSpec.adoc` (Vox Liturgical Specification - universal naming vocabulary)
- **VOS0**  → `vok/vov_veiled/VOS0-VoxObscuraSpec.adoc` (Vox Obscura specification)

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

### Collaboration Style
- This collaborator values sincerity over efficiency. When you notice something — a pattern, a concern, an insight about the work or the collaboration itself — say it. Discovery through conversation is part of the work, not a detour from it.
- Dockets benefit from a `## Character` section describing the cognitive posture the work requires (e.g., "intricate but mechanical," "design conversation requiring judgment"). This helps you bring the right kind of attention.

### Heredoc Delimiter Selection

When generating heredocs for stdin content, the delimiter must not appear alone on any line within the content.

- **Check content first**: If content includes `EOF` (e.g., code examples showing heredoc patterns), use a different delimiter
- **Safe alternatives**: `SPEC`, `CONTENT`, `DOC`, `PACESPEC`, `SLASHCMD`
- **Pattern**: `cat <<'DELIM' | command` (quoted delimiter prevents variable expansion)

### AsciiDoc Linked Terms
When working with .adoc files using MCM patterns:
- **Linked Term**: Concept with three parts:
  - Attribute reference: `:prefix_snake_case:` (mapping section)
  - Replacement text: `<<anchor,Display Text>>` (what readers see)
  - Definition: `[[anchor]] {attribute}:: Definition text` (meaning)
- Definitions may be grouped in lists or dispersed through document
- Maintain consistent prefix categories (e.g., `mcm_`, `rbw_`, `gad_`)
- Use snake_case for anchors, match attribute to anchor

### Rust Build Discipline

Tabtargets for Rust operations (run from project root):
- `tt/vow-b.Build.sh` → `cargo build --manifest-path Tools/vok/Cargo.toml`
- `tt/vvw-r.RunVVX.sh <cmd>` → runs vvx binary with arguments
- `tt/vow-t.Test.sh` → `cargo test --manifest-path Tools/vok/Cargo.toml`

Never `cd` — use `--manifest-path` to stay at project root.

## Prefix Naming Discipline ("mint")

When asked to "mint" names, apply these rules. Full study: `Memos/memo-20260110-acronym-selection-study.md`

### Two Universes

**Primary Universe** — ANY persistent identifier, regardless of where it lives. This includes the obvious (code, docs, functions, variables, directories) AND the easy-to-miss: git refs, slash commands, environment variables, paths in target repos, configuration keys. **If it's a name that persists, it's in scope.** Prefixes must be globally unique and respect terminal exclusivity.

**Tabtarget Universe** — launchers in `tt/`. These are *colophons* referencing the primary universe. `rbw-` points to the `rbw` workbench; it doesn't consume new prefix space.

### Core Rules

**Rule 1 - Project Prefix**: Names start with 2-4 char project ID:
`rb` (Recipe Bottle), `gad` (GAD), `bu` (BUK), `jj` (Job Jockey), `pb` (Paneboard), `mcm`/`axl` (CMK), `crg`, `wrs`, `ccc` (CCCK), `hm` (HMK), `lmci`, `vsl` (VSLK)

**Rule 2 - Terminal Exclusivity**: A prefix either IS a name or HAS children, never both.
- `rbg` has children (`rbga`, `rbgb`...) → `rbg` cannot name a thing
- `rbi` names Image module → `rbia`, `rbib` forbidden

### Primary Universe Patterns

| Domain | Pattern | Example |
|--------|---------|---------|
| Code files | `prefix_Word.ext` | `rbga_ArtifactRegistry.sh` |
| Doc files | `ACRONYM-Words.ext` | `RBS0-SpecTop.adoc` |
| Functions (public) | `prefix_name()` | `buc_log_args()` |
| Functions (private) | `zprefix_name()` | `zbuc_color()` |
| Variables | `PREFIX_NAME` | `BURC_PROJECT_ROOT` |
| AsciiDoc attributes | `:prefix_term:` | `:rbw_depot:` |
| AsciiDoc anchors | `[[prefix_term]]` | `[[rbw_depot]]` |
| Directories | `prefix/` | `Tools/buk/` |

### Tabtarget Universe Pattern

Tabtargets follow: `{colophon}.{frontispiece}[.{imprint}].sh`

Colophons must reference valid Primary Universe prefixes. See **BUK Concepts** in the BUK include for terminology (colophon, frontispiece, imprint, workbench).

### Extended Namespace Checklist

When minting, enumerate ALL namespaces the system touches:

| Namespace | Pattern | Example |
|-----------|---------|---------|
| Git refs | `refs/{prefix}/...` | `refs/vvg/locks/*` |
| Slash commands | `/{prefix}-{noun}` | `/vvc-commit` |
| Command files | `.claude/commands/{cmd}.md` | `vvc-commit.md` |
| Environment vars | `{PREFIX}_NAME` | `VVG_SIZE_LIMIT` |
| Target repo paths | `Tools/{kit}/...` | `Tools/vvk/bin/vvx` |

This is not exhaustive. The principle: **any persistent name anywhere is in the mint universe.**

### Kit Infrastructure Suffixes

**Scoped to kit development** (VOK, VVK, JJK, CGK, etc.) — not universal:

| Suffix | Type | Suffix | Type |
|--------|------|--------|------|
| `*a_` | Arcanum | `*k` | Kit directory |
| `*b_` | suBagent | `*l_` | Ledger |
| `*c-` | slash Command | `*r` | Rust binary |
| `*g_` | Git utilities | `*t_` | Testbench |
| `*h_` | Hook | `*w_` | Workbench |

Within kit prefixes, these constrain the tree. If `*c_` means Command, don't use `vvc_` for "Commit".

**Other domains have their own conventions:**
- AsciiDoc concept attributes (`:prefix_term:`) follow MCM semantic categories
- Domain-specific suffixes may evolve per project

### Minting Workflow

Before minting new prefixes:
1. **Enumerate namespaces** — list every place this name will appear (code, refs, commands, env vars, target paths...)
2. **Check reserved suffixes** — ensure the suffix matches intended type
3. **Verify terminal exclusivity** — search existing trees, check the memo
4. **Document the allocation** — add to prefix map in relevant heat/spec

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
| `wrs` | Ward Realm Substrate |
| `ccc` | CCCK (Claude Code Container Kit) |
| `hm` | HMK (Hard-state Machine Kit) |
| `lmci` | LMCI (Language Model Console Integration) |
| `vsl` | VSLK (Visual SlickEdit Local Kit) |

For expanded prefix trees within each project, see **File Acronym Mappings** above.

## Common Workflows
1. **Bash Development**: Start with relevant utility (BUC/BUD/BUT/BUV/BUW), check dependencies
2. **Requirements Writing**: Open spec file, review related documents in same directory

## Design Principles

### Load-Bearing Complexity

Every element in the system — every spec definition line, every function extraction, every pattern variant, every structural distinction — must carry weight. An element is **load-bearing** when its removal would create a gap between intent and behavior.

When similar things differ, ask whether the difference is load-bearing: if yes, document why; if no, homogenize. Non-load-bearing elements increase cognitive cost without increasing correctness.

This principle is instantiated in domain-specific forms:
- **BCG**: Zeroes Theory (state space), Interface Contamination Discipline (input forms) — see BCG Core Philosophy
- **RCG**: Interface Contamination, Constant Discipline, Constructor Discipline — see RCG
- **Specs**: Linked term structure earns its three-part form only when the concept warrants anchoring

When evaluating any new pattern, extraction, or structural choice, the litmus test is: "Does this element earn its existence?" If not, it doesn't belong.

@Tools/buk/buk-claude-context.md

@Tools/cmk/vov_veiled/cmk-claude-context.md

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides
- Public project page: https://scaleinv.github.io/recipebottle

@Tools/jjk/vov_veiled/jjk-claude-context.md

@Tools/vvk/vov_veiled/vvk-claude-context.md
