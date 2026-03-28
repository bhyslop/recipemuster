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

### Test Execution Discipline

Run test fixture tabtargets **sequentially, never in parallel**. Test fixtures share regime state and container/network namespaces — parallel execution causes resource conflicts and false failures.

```
# Correct: run one at a time
tt/rbw-tf.TestFixture.regime-validation.sh
tt/rbw-tf.TestFixture.nsproto-security.sh

# Wrong: never run fixtures concurrently
tt/rbw-tf.TestFixture.regime-validation.sh & tt/rbw-tf.TestFixture.nsproto-security.sh &
```

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

### Forbidden Shell Operations

**Never use `cd` in Bash commands — NO exceptions.**

The working directory persists between Bash tool calls. A single `cd` corrupts ALL subsequent commands that use relative paths, including every `./tt/` tabtarget.

- Use absolute paths instead of cd'ing
- Use `--manifest-path` or equivalent flags
- This applies to ALL work — not just Rust builds

**There is no safe cd.** Do not reason that "I'll cd back" — the next tool call may be yours or another officium's, and it will break.

### Rust Build Discipline

Tabtargets for Rust operations (run from project root):
- `tt/vow-b.Build.sh` → `cargo build --manifest-path Tools/vok/Cargo.toml`
- `tt/vvw-r.RunVVX.sh <cmd>` → runs vvx binary with arguments
- `tt/vow-t.Test.sh` → `cargo test --manifest-path Tools/vok/Cargo.toml`

See **Forbidden Shell Operations** above — never `cd`, use `--manifest-path` instead.
When running cargo directly, use `--manifest-path` to stay at project root.

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

Colophons must reference valid Primary Universe prefixes. See **BUK Concepts** below for terminology (colophon, frontispiece, imprint, workbench).

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

## Bash Utility Kit (BUK) Concepts

BUK provides tabtarget/launcher infrastructure. Key vocabulary:

| Term | Definition |
|------|------------|
| **Tabtarget** | Launcher script in `tt/` that delegates to a workbench |
| **Colophon** | Routing identifier (includes hyphen): `rbw-B`, `buw-tt-ll` |
| **Frontispiece** | Human-readable description (PascalCase): `ConnectBottle` |
| **Imprint** | Optional target parameter: `nsproto`, `srjcl` |
| **Zipper** | BCG-compliant module kindling colophon→module→command array constants |
| **Workbench** | Routes commands: `{prefix}w_workbench.sh` |
| **Testbench** | Routes tests: `{prefix}t_testbench.sh` |

**Tabtarget pattern**: `{colophon}.{frontispiece}[.{imprint}].sh`

- The `.` is the delimiter between parts
- The hyphen is part of the colophon (not a separator)
- Colophon naming follows **Prefix Naming Discipline** above

Full spec: `Tools/buk/README.md`

<!-- MANAGED:CMK:BEGIN -->
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

<!-- MANAGED:CMK:END -->

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides
- Public project page: https://scaleinv.github.io/recipebottle

<!-- When editing MANAGED:JJK content, also update: Tools/jjk/vov_veiled/vocjjmc_core.md -->
<!-- MANAGED:JJK:BEGIN -->
## Job Jockey Configuration

Job Jockey (JJ) is installed for managing project initiatives.

**Concepts:**
- **Heat**: Bounded initiative with coherent goals that are clear and present (3-50 officia). Status: `racing` (active execution) or `stabled` (paused for planning). Location: `current/` or `retired/` (done).
- **Pace**: Discrete action within a heat.
- **Itch**: Future work (any detail level), lives in jji_itch.md
- **Scar**: Closed work with lessons learned, lives in jjs_scar.md
- **Spook**: Team infrastructure stumble — any workflow failure improvable with deft attention. Capture as a pace when encountered, don't lose the current thread.

**Identities vs Display Names:**
- **Firemark**: Heat identity (`₣AA` or `AA`). Used in command params and JSON keys.
- **Coronet**: Pace identity (`₢AAAAk` or `AAAAk`). Used in command params and JSON keys.
- **Silks**: kebab-case display name. Human-readable only — NOT usable for lookups.

When a command takes a firemark or coronet, provide the identity, not the silks.

**Case sensitivity**: Firemarks and coronets are case-sensitive. `Av` ≠ `AV` ≠ `av`. Passing the wrong case produces a confusing "not found" error. Copy identities exactly as displayed — the final letter's case distinguishes heats (e.g., `₣Av` vs `₣AV` are different heats).

- Target repo dir: `.`
- JJ Kit path: `Tools/jjk/README.md`

**MCP Tool Usage:**

All JJK commands are accessed via the single `mcp__vvx__jjx` MCP tool with three parameters:
- `command`: string selecting the operation — always the canonical `jjx_*` name (e.g., `"jjx_show"`, `"jjx_enroll"`, `"jjx_record"`)
- `params`: JSON object with command-specific fields (see reference below)
- `officium`: officium identity string from `jjx_open` (required on all commands except `jjx_open` — see Officium Protocol below)

**`params` must be a JSON object, never a string.** If params is accidentally stringified (e.g., `"{\"key\": \"val\"}"` instead of `{"key": "val"}`), deserialization will fail. The server has a defensive fallback for this, but always pass a native object.

**Verb names are NOT command names**: there is no `jjx_slate`, `jjx_mount`, `jjx_notch`, `jjx_groom` command. The verb table below maps horse vocabulary to actual MCP commands.
NEVER invent param fields — check the reference below first.

**Quick Verbs** — When user says just the verb, invoke the corresponding command:

| Verb | Noun | MCP command |
|------|------|-------------|
| muster | heats | `jjx_list` |
| parade | heat/pace | `jjx_show` |
| scout | heats | `jjx_search` |
| nominate | heat | `jjx_create` |
| mount | heat/pace | See Mount Protocol below |
| groom | heat | See Groom Protocol below |
| slate | pace | `jjx_enroll` |
| reslate | pace | `jjx_redocket` |
| notch | pace | See Commit Discipline below |
| wrap | pace | `jjx_close` |
| rail | heat | `jjx_reorder` |
| furlough | heat | `jjx_alter` |
| retire | heat | `jjx_archive` |
| restring | heat | `jjx_transfer` |

**MCP Command Reference:**

All params are JSON objects. `?` = optional, `[]` = array. Booleans default to false.

```
jjx_open           {}
jjx_show           {target?, detail?, remaining?}
jjx_list           {status?}
jjx_orient         {firemark?}
jjx_create         {silks}
jjx_enroll         {firemark, silks, docket, before?, after?, first?}
jjx_reorder        {firemark, move?, before?, after?, first?, last?}
jjx_alter          {firemark, racing?, stabled?, silks?}
jjx_record         {identity, files[], size_limit?, intent?}
jjx_close          {coronet, summary?, size_limit?}
jjx_log            {firemark, limit?}
jjx_search         {pattern, actionable?}
jjx_archive        {firemark, size_limit?}
jjx_transfer       {firemark, to, coronets}
jjx_continue       {firemark}
jjx_paddock        {firemark?, content?, note?}
jjx_relocate       {coronet, to, before?, after?, first?}
jjx_redocket  {coronet?, docket?}
jjx_relabel        {coronet, silks}
jjx_drop           {coronet}
jjx_brief      {coronet}
jjx_coronets   {firemark, remaining?, rough?}
jjx_landing        {coronet, agent, content?}
jjx_validate       {}
```

**Key points:**
- `jjx_show` takes firemark OR coronet in the `target` param
  - Heat overview: `{"target": "AF"}`
  - Single pace: `{"target": "AFAAb"}`
  - Additional params: `detail`, `remaining` only
- `jjx_orient` output includes next actionable pace — no separate show call needed
- `jjx_enroll` takes `docket` as a string param (not stdin)
- `jjx_close` takes `summary` as a string param (not stdin pipe)
- `jjx_record` takes `files` as a native JSON array: `["file1.rs", "file2.rs"]`
- `jjx_transfer` takes `coronets` as a JSON-encoded string (not a native array): `"[\"AYAAA\", \"AYAAB\"]"`

### Officium Protocol

Each chat session must open an officium before using any jjx commands.

1. **At chat start**, call `jjx_open` (no params, no officium field). It returns a ☉-prefixed identity string (e.g., `☉260327-1000`).
2. **On every subsequent jjx call**, pass the returned identity as the `officium` field on the MCP tool (sibling to `command` and `params`).
3. **Self-healing**: If any jjx command fails with "Officium directory not found", call `jjx_open` again to create a fresh officium, then retry.

The ☉ (U+2609 SUN) prefix parallels ₣/₢ for firemarks/coronets. Pass it exactly as returned — the dispatcher strips it.

Gazette file exchange uses two directional files in the officium exchange directory. Every jjx MCP call unconditionally deletes both gazette files on entry (read+delete `gazette_in.md`, delete `gazette_out.md`). Gazette content has single-MCP-call lifetime — it is a parameter or a return value, not persistent state.

- **`gazette_in.md`** (agent → server): write before calling a setter command. Getter commands (`jjx_orient`, `jjx_show`, `jjx_paddock` getter) write `gazette_out.md` after returning.
- **`gazette_out.md`** (server → agent): read after a getter command returns. The next jjx call of any kind deletes it.

**Gazette wire format (setter commands):**
Each notice is a `#`-header line with slug and lede, followed by content body. Write `gazette_in.md`, then call the command.

| Command | Write to `gazette_in.md` | Then call with params |
|---------|--------------------------|----------------------|
| `jjx_enroll` | `# slate <silks>` + docket body | `{"firemark": "XX"}` |
| `jjx_redocket` | `# reslate <coronet>` + docket body | `{"coronet": "XXXXX"}` |
| `jjx_paddock` (set) | `# paddock <firemark>` + content body | `{}` |

Gazette paths: `.claude/jjm/officia/<officium-id>/gazette_in.md` and `gazette_out.md`.

Example — reslate a pace docket:
1. Write `gazette_in.md`: `# reslate AvAAH\n\n## Character\nNew docket content...`
2. Call: `jjx_redocket` with `{"coronet": "AvAAH"}`

**Read-modify-write workflow** (paddock editing):
1. Call `jjx_paddock` getter → reads `gazette_out.md`
2. Rename `gazette_out.md` → `gazette_in.md`, edit content
3. Call `jjx_paddock` setter

### Mount Protocol

When user says "mount" or you need to engage the next pace:

1. Run `jjx_orient` command (with optional firemark) to get context
2. Parse output: Racing-heats table, Heat/Next/Docket/Recent-work sections. Read paddock and pace docket from the gazette file written to the officium exchange directory.
3. **Read the paddock before the docket.** The paddock tells you the shape of the work and what's been learned; the docket tells you what to do next. Orientation before action.
4. Display context to user: racing heats, heat silks, paddock summary, recent work, current pace and docket
5. **Name assessment**: If pace silks doesn't fit docket, offer rename via `jjx_relabel`
6. Analyze docket, propose approach (2-4 bullets), assess execution strategy:
   - Model tier: haiku (mechanical), sonnet (standard dev), opus (architectural)
   - Parallelization: file independence, task decomposability
   - State recommendation explicitly (e.g., "Sequential sonnet — single file")
7. Ask to proceed, then begin work

### Groom Protocol

When user says "groom":

1. Run `jjx_show` command with `{target: FIREMARK, detail: true, remaining: true}`
2. Display overview: heat silks, progress, remaining paces with dockets
3. Enter planning mode: suggest structural operations (slate new paces, rail to reorder, reslate to refine dockets, paddock review)

### Commit Discipline

When working on a heat, use `jjx_record` for commits with heat/pace affiliation.

**Pace-affiliated commit** (active pace provides context):
Use `jjx_record` with `{identity: "CORONET", files: ["file1", "file2"], intent: "description"}`

**Heat-affiliated commit** (no active pace, but part of heat work):
Use `jjx_record` with `{identity: "FIREMARK", files: ["file1", "file2"], intent: "description"}`

Synthesize intent from the conversation — describe *what* was accomplished, not *how*.

**Size guard — ALL commands (record, close, archive)**: If ANY jjx command fails due to size limits, STOP. Report the byte count, limit, and per-file breakdown to the user. Ask: "Raise the limit, or split?" Then WAIT for the user's explicit answer before taking any action. NEVER auto-override `size_limit` — the guard exists to force a human decision.

When user says "notch", determine context (pace or heat affiliated) and invoke `jjx_record` with the appropriate identity and explicit file list.

**Multi-Officium Discipline:**
Multiple Claude officia (concurrent chat sessions, each with its own ☉-prefixed officium ID) may work concurrently in the same repo. The explicit file list in `jjx_record` enables orthogonal commits.

- Claude is **additive only** — make commits, never discard changes
- "Unexpected" uncommitted changes are likely another officium's work
- If something looks wrong, ASK — do not "fix" by discarding
- Commit only YOUR files; ignore everything else

**Forbidden Git Commands — NO exceptions, NO "safe" variants:**
- `git reset` — ALL forms: `--hard`, `--soft`, `--mixed`, with paths, without paths. Even `git reset HEAD <file>` (unstaging) is forbidden — it's too close to destructive variants and Claude will reason its way into worse forms.
- `git restore` — ALL forms: working tree, staged, with `--source`, without
- `git checkout <file>` — when used to discard changes (navigating branches is fine)
- `git clean` — ALL forms
- `git stash` — ALL forms

**What to do instead:**
- Staging wrong? Run `jjx_record` with the correct file list — it handles staging
- Made a mistake? Make a new commit that fixes it — additive, not destructive
- Confused by repo state? ASK the user — another officium may be mid-work
- Need to undo something? Explain the situation to the user and let them decide

**Build & Run Discipline:**
Always run these after Rust code changes:
- `tt/vow-b.Build.sh` — Build
- `tt/vvw-r.RunVVX.sh` — Run VVX

**JJX Commands Are Self-Committing:**
`jjx_enroll`, `jjx_close`, `jjx_record`, and other state-mutating jjx commands create git commits internally. **`jjx_close` (wrap) commits ALL uncommitted changes** — code files and gallops state together in one commit. Do NOT follow `jjx_record` or `jjx_close` with another commit command — the tree will already be clean. If a commit command says "Nothing to commit", check `git status --short` and accept the result.

**Diagnose Before Escalating:**
When a command fails, check the simplest explanation first. "Nothing to commit" means the tree is clean — verify with `git status`, don't try creative workarounds. "Invalid params" means wrong field names — check the MCP Command Reference above, don't guess. One diagnostic command beats three speculative retries.

### Wrap Discipline

**NEVER auto-wrap a pace.** Always ask the user explicitly: "Ready to wrap ₢XXXXX?" and wait for confirmation before running `jjx_close`. The user decides when work is complete, not the agent.

When work is complete, report outcomes and ask. Do not wrap.

When wrapping (after user confirms), always include a summary of the work:
Use `jjx_close` with `{coronet: "CORONET", summary: "Added bitmap displays to orient output"}`
The agent always has context about what was accomplished — include it.

**Wrap commits everything.** `jjx_close` stages and commits all dirty files (code edits + gallops state) in one commit. Do NOT notch before or after wrapping — the wrap IS the final commit. If you want separate commits for intermediate code milestones, notch during work; remaining uncommitted changes are captured by wrap.

<!-- MANAGED:JJK:END -->

<!-- MANAGED:BUK:BEGIN -->
## Bash Utility Kit (BUK)

BUK provides tabtarget/launcher infrastructure for bash-based tooling.

**Key files:**
- `Tools/buk/buc_command.sh` — command utilities
- `Tools/buk/bud_dispatch.sh` — dispatch utilities
- `Tools/buk/buw_workbench.sh` — workbench

**Tabtarget pattern:** `{colophon}.{frontispiece}[.{imprint}].sh`

For full documentation, see `Tools/buk/README.md`.

<!-- MANAGED:BUK:END -->

<!-- MANAGED:VVK:BEGIN -->
## Voce Viva Kit (VVK)

VVK provides core infrastructure for Claude Code kits.

**Key commands:**
- `/vvc-commit` — Guarded git commit with size validation

**Key files:**
- `Tools/vvk/bin/vvx` — Core binary
- `.vvk/vvbf_brand.json` — Installation brand file

For installation/uninstallation, use `vvi_install.sh` and `vvu_uninstall.sh`.

<!-- MANAGED:VVK:END -->
