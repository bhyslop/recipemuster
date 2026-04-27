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
- **RBGO** → `rbk/rbgo_OAuth.sh`
- **RBGP** → `rbk/rbgp_Payor.sh`
- **RBGU** → `rbk/rbgu_Utility.sh`
- **RBGV** → `rbk/rbgv_AccessProbe.sh` (Google Verification — JWT SA and Payor OAuth access probes)
- **RBH0** → `rbk/rbh0/` (Handbook directory — decomposed one-function-per-file)
  - `rbh*` is the Handbook family (human-facing procedures), parallel to `rbg*` (Google machinery). Three groups: `RBHO` onboarding, `RBHP` payor, `RBHW` windows. Colophon pattern: `rbw-h0` (HandbookTOP index), `rbw-h{o,p,w}` (group tops), `rbw-H{O,P,W}*` (subordinate procedures).
  - **RBHO0** → `rbk/rbh0/rbho0_cli.sh` (Onboarding CLI — thin furnish, probe-only dependencies)
  - **RBHOB** → `rbk/rbh0/rbhob_base.sh` (Onboarding base — kindle, sentinel, probes, shared helpers)
  - **RBHP0** → `rbk/rbh0/rbhp0_cli.sh` (Payor CLI — full regime + OAuth + IAM dependencies)
  - **RBHPB** → `rbk/rbh0/rbhpb_base.sh` (Payor base — kindle, sentinel, enforce)
  - **RBHW0** → `rbk/rbh0/rbhw0_cli.sh` (Windows CLI — handbook display + zipper dependencies)
  - **RBHWB** → `rbk/rbh0/rbhwb_base.sh` (Windows base — kindle, sentinel)
- **RBID** → `rbev-vessels/common-ifrit-context/` (Ifrit — in-bottle attack binary for crucible testing; shared source across tether/airgap variants)
- **RBJ**  → Jailer prefix (non-terminal: children rbjs, rbjp, rbje, rbjh)
- **RBJS** → `rbk/rbjs_sentry.sh` (Jailer Sentry - container security setup: iptables, dnsmasq, enclave network)
- **RBLM** → `rbk/rblm_cli.sh` (Lifecycle Marshal - zero regime to blank template, proof repo for release testing)
- **RBOB** → `rbk/rbob_bottle.sh`
- **RBYC** → `rbk/rbyc_common.sh` (Common vocabulary — linked term constants for handbook yelp fragments)
- **RBQ**  → `rbk/rbq_Qualify.sh` (Qualification orchestrator - tabtarget/colophon/nameplate health)
- **RBTD** → `rbk/rbtd/` (Theurge — crucible test orchestrator)
- **RBTW** → `rbk/rbtd/rbtw_workbench.sh` (Theurge workbench — build/test routing, orthogonal from VOW)
- **RBS0** → `rbk/vov_veiled/RBS0-SpecTop.adoc`
- **RBSAA** → `rbk/vov_veiled/RBSAA-ark_abjure.adoc`
- **RBSAB** → `rbk/vov_veiled/RBSAB-ark_about.adoc` (Standalone Cloud Build about pipeline - syft SBOM + mode-aware build_info)
- **RBSAC** → `rbk/vov_veiled/RBSAC-ark_conjure.adoc`
- **RBSAE** → `rbk/vov_veiled/RBSAE-ark_enshrine.adoc`
- **RBSAG** → `rbk/vov_veiled/RBSAG-ark_graft.adoc` (Graft operation - local image push to GAR)
- **RBSAJ** → `rbk/vov_veiled/RBSAJ-access_jwt_probe.adoc`
- **RBSAK** → `rbk/vov_veiled/RBSAK-ark_kludge.adoc` (Kludge operation - local vessel build for development)
- **RBSAO** → `rbk/vov_veiled/RBSAO-access_oauth_probe.adoc`
- **RBSAP** → `rbk/vov_veiled/RBSAP-ark_plumb.adoc`
- **RBSAS** → `rbk/vov_veiled/RBSAS-ark_summon.adoc`
- **RBSAV** → `rbk/vov_veiled/RBSAV-ark_vouch.adoc`
- **RBSAX** → `rbk/vov_veiled/RBSAX-access_setup.adoc`
- **RBSBC** → `rbk/vov_veiled/RBSBC-bottle_create.adoc`
- **RBSBK** → `rbk/vov_veiled/RBSBK-bottle_cleanup.adoc`
- **RBSBL** → `rbk/vov_veiled/RBSBL-bottle_launch.adoc`
- **RBSCB** → `rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc` (Cloud Build security posture and deferred hardening)
- **RBSCC** → `rbk/vov_veiled/RBSCC-crucible_charge.adoc`
- **RBSCE** → `rbk/vov_veiled/RBSCE-command_exec.adoc`
- **RBSCIG** → `rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc` (IAM Grant API Contracts - verified behavioral contracts per resource type)
- **RBSCIP** → `rbk/vov_veiled/RBSCIP-IamPropagation.adoc`
- **RBSCJ** → `rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc`
- **RBSCL** → `rbk/vov_veiled/RBSCL-consecration_tally.adoc` (Hallmark Tally - registry ark inventory with health status)
- **RBSCN** → `rbk/vov_veiled/RBSCN-crucible_enjoin.adoc`
- **RBSCO** → `rbk/vov_veiled/RBSCO-CosmologyIntro.adoc`
- **RBSCTD** → `rbk/vov_veiled/RBSCTD-CloudBuildTriggerDispatch.adoc`
- **RBSDD** → `rbk/vov_veiled/RBSDD-director_divest.adoc` (Governor divests a director SA — zero-arg by-role lookup)
- **RBSDE** → `rbk/vov_veiled/RBSDE-depot_levy.adoc`
- **RBSDI** → `rbk/vov_veiled/RBSDI-depot_inscribe.adoc` (Reliquary creation — director mirrors tool images to datestamped GAR namespace)
- **RBSDK** → `rbk/vov_veiled/RBSDK-director_invest.adoc` (Governor invests a director SA — singleton-per-role)
- **RBSDL** → `rbk/vov_veiled/RBSDL-depot_list.adoc`
- **RBSDN** → `rbk/vov_veiled/RBSDN-depot_initialize.adoc`
- **RBSDR** → `rbk/vov_veiled/RBSDR-director_roster.adoc` (Governor rosters director SAs — fact-multi emission, presence-as-fact)
- **RBSDS** → `rbk/vov_veiled/RBSDS-dns_step.adoc`
- **RBSDU** → `rbk/vov_veiled/RBSDU-depot_unmake.adoc`
- **RBSDV** → `rbk/vov_veiled/RBSDV-director_vouch.adoc`
- **RBSDY** → `rbk/vov_veiled/RBSDY-director_yoke.adoc` (Yoke operation — director validates reliquary stamp and conjure vessel, then rewrites RBRV_RELIQUARY in rbrv.env)
- **RBSFR** → `rbk/vov_veiled/RBSFR-FundusRegistry.md` (Fundus Capability Registry — agent-interpreted test target inventory)
- **RBSGD** → `rbk/vov_veiled/RBSGD-gdc_establish.adoc`
- **RBSGM** → `rbk/vov_veiled/RBSGM-governor_mantle.adoc`
- **RBSGS** → `rbk/vov_veiled/RBSGS-GettingStarted.adoc`
- **RBSHR** → `rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc` (Horizon Roadmap - single collection point for defined-but-unscoped future work)
- **RBSII** → `rbk/vov_veiled/RBSII-iptables_init.adoc`
- **RBSIJ** → `rbk/vov_veiled/RBSIJ-image_jettison.adoc`
- **RBSIM** → `rbk/vov_veiled/RBSIM-image_muster.adoc` (Image Muster — three-domain catalog listing: hallmarks, reliquaries, enshrinements)
- **RBSIP** → `rbk/vov_veiled/RBSIP-ifrit_pentester.adoc` (Ifrit Pentester — adversarial AI escape testing framework)
- **RBSIR** → `rbk/vov_veiled/RBSIR-image_rekon.adoc` (Image Rekon — director-only canonical-member presence under hallmark/reliquary subtrees)
- **RBSIW** → `rbk/vov_veiled/RBSIW-image_wrest.adoc`
- **RBSNC** → `rbk/vov_veiled/RBSNC-network_create.adoc`
- **RBSNX** → `rbk/vov_veiled/RBSNX-network_connect.adoc`
- **RBSOB** → `rbk/vov_veiled/RBSOB-oci_layout_bridge.adoc`
- **RBSPE** → `rbk/vov_veiled/RBSPE-payor_establish.adoc`
- **RBSPI** → `rbk/vov_veiled/RBSPI-payor_install.adoc`
- **RBSPR** → `rbk/vov_veiled/RBSPR-payor_refresh.adoc`
- **RBSPT** → `rbk/vov_veiled/RBSPT-port_setup.adoc`
- **RBSQB** → `rbk/vov_veiled/RBSQB-quota_build.adoc`
- **RBSRA** → `rbk/vov_veiled/RBSRA-CredentialFormat.adoc`
- **RBSRD** → `rbk/vov_veiled/RBSRD-retriever_divest.adoc` (Governor divests a retriever SA — zero-arg by-role lookup)
- **RBSRG** → `rbk/vov_veiled/RBSRG-RegimeGcbPins.adoc`
- **RBSRK** → `rbk/vov_veiled/RBSRK-retriever_invest.adoc` (Governor invests a retriever SA — singleton-per-role)
- **RBSRL** → `rbk/vov_veiled/RBSRL-retriever_roster.adoc` (Governor rosters retriever SAs — fact-multi emission, presence-as-fact)
- **RBRN**  → `rbk/vov_veiled/RBRN-RegimeNameplate.adoc`
- **RBSRM** → `rbk/vov_veiled/RBSRM-RegimeMachine.adoc`
- **RBSRO** → `rbk/vov_veiled/RBSRO-RegimeOauth.adoc`
- **RBSRP** → `rbk/vov_veiled/RBSRP-RegimePayor.adoc`
- **RBSRR** → `rbk/vov_veiled/RBSRR-RegimeRepo.adoc`
- **RBSRS** → `rbk/vov_veiled/RBSRS-RegimeStation.adoc`
- **RBSRV** → `rbk/vov_veiled/RBSRV-RegimeVessel.adoc`
- **RBSSC** → `rbk/vov_veiled/RBSSC-security_config.adoc`
- **RBSSR** → `rbk/vov_veiled/RBSSR-sentry_run.adoc`
- **RBSSS** → `rbk/vov_veiled/RBSSS-sentry_start.adoc`
- **RBSTB** → `rbk/vov_veiled/RBSTB-trigger_build.adoc`

#### BUK Subdirectory (`Tools/buk/`)
- **BCG**  → `buk/vov_veiled/BCG-BashConsoleGuide.md` (Bash Console Guide - enterprise bash patterns)
- **BUS0** → `buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` (Bash Utilities Specification - tabtarget dispatch vocabulary)
- **BUC**  → `buk/buc_command.sh` (command utilities, buc_* functions)
- **BUD**  → `buk/bud_dispatch.sh` (dispatch utilities, zbud_* functions)
- **BUH**  → `buk/buh_handbook.sh` (handbook utilities, buh_* functions - always-visible user interaction)
- **BUT**  → `buk/but_test.sh` (test utilities, but_* functions)
- **BUYM** → `buk/buym_yelp.sh` (yelp module — diastema wire format, yawp functions, format resolver, legacy captures)
- **BUV**  → `buk/buv_validation.sh` (validation utilities, buv_* functions)
- **BUW**  → `buk/buw_workbench.sh` (workbench utilities, buw_* functions)
- **BUTT** → `buk/butt_testbench.sh` (BUK test framework self-test — kick-tires + bure-tweak, 9 cases)
- **BURC** → `buk/burc_cli.sh`, `buk/burc_regime.sh` (regime configuration)
- **BURS** → `buk/burs_cli.sh`, `buk/burs_regime.sh` (regime station)

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
- **JJSTF** → `jjk/vov_veiled/JJSTF-test-fundus.adoc` (Test Fundus — fundus scenario profiles and preflight contracts)
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

Two orthogonal Rust pipelines. Always use the tabtarget, never raw cargo commands.

**VOW pipeline** (vvk/jjk/cmk kits — parceled for delivery):
- `tt/vow-b.Build.sh` — build vvr binary and install to VVK bin
- `tt/vow-t.Test.sh` — run all kit crate tests
- `tt/vvw-r.RunVVX.sh <cmd>` — run vvx binary with arguments

**RBTW pipeline** (theurge — rbk's own test infrastructure, orthogonal from VOW):
- `tt/rbtd-b.Build.sh` — build theurge crate
- `tt/rbtd-t.Test.sh` — run theurge unit tests

### Test Execution

**Test suites** group fixtures by dependency tier. Run the broadest applicable suite:

| Suite | Tabtarget | Dependencies | What it covers |
|-------|-----------|-------------|----------------|
| `fast` | `tt/rbtd-s.TestSuite.fast.sh` | None | enrollment-validation (47), regime-validation (21), regime-smoke (7) = 75 cases |
| `service` | `tt/rbtd-s.TestSuite.service.sh` | GCP credentials | fast + access-probe (4), four-mode (1) = 80 cases |
| `crucible` | `tt/rbtd-s.TestSuite.crucible.sh` | Container runtime | fast + tadmor-security (34), srjcl-jupyter (3), pluml-diagram (5) = 117 cases |
| `complete` | `tt/rbtd-s.TestSuite.complete.sh` | All of the above | All 8 fixtures = 122 cases |

**After code changes**, run the appropriate tier:
- Regime/validation changes → `fast`
- Foundry/credential changes → `service`
- Bottle/sentry/network changes → `crucible`
- Pre-release or decomposition sweep → `complete`

**Single fixture**: `tt/rbtd-r.Run.{name}.sh` (e.g., `tadmor`, `enrollment-validation`, `regime-smoke`)

**Single case**: `tt/rbtd-s.SingleCase.{fixture}.sh [case-name]` — run one case against an already-charged crucible (no charge/quench). Omit case name to list all cases. Workflow for crucible debugging: charge via `tt/rbw-cC.Charge.{nameplate}.sh`, run individual cases, quench via `tt/rbw-cQ.Quench.{nameplate}.sh` when done.

**BUK self-test**: `tt/buw-st.BukSelfTest.sh` — exercises BUK test framework (kick-tires + bure-tweak, 9 cases)

**Sequential only**: Never run fixtures in parallel — they share regime state and container namespaces.

## Prefix Naming Discipline ("mint")

When asked to "mint" names, apply these rules. Full study: `Memos/memo-20260110-acronym-selection-study.md`

### Two Universes

**Primary Universe** — ANY persistent identifier, regardless of where it lives. This includes the obvious (code, docs, functions, variables, directories) AND the easy-to-miss: git refs, slash commands, environment variables, paths in target repos, configuration keys. **If it's a name that persists, it's in scope.** Prefixes must be globally unique and respect terminal exclusivity.

**Tabtarget Universe** — launchers in `tt/`. These are *colophons* referencing the primary universe. `rbw-` points to the `rbw` workbench; it doesn't consume new prefix space.

### Core Rules

**Rule 1 - Project Prefix**: Names start with 2-4 char project ID:
`rb` (Recipe Bottle), `gad` (GAD), `bu` (BUK), `jj` (Job Jockey), `pb` (Paneboard), `mcm`/`axl` (CMK), `crg`, `wrs`, `hm` (HMK), `lmci`, `vsl` (VSLK)

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

### Quoin Sub-Letter Discipline (MCM Spec-Internal)

For MCM concept model specs, apply this discipline on top of the general minting rules:

- **Uniform shape `prefixXY_word`** — every quoin carries exactly two sub-letters
- **Hard 2-letter ceiling** — never mint 3+ letter sub-prefixes (proven opaque in RBS0/JJS0)
- **Within-domain Y monosemy** — each sub-letter has exactly one meaning per domain; reusing Y for a second concept produces pattern-recognition collisions (JJS0's `rd` appearing in three quoins with three meanings is the canonical failure)
- **Documented legend** — sub-letter table declared in a comment block at the top of the spec's mapping section; future minters consult the table rather than re-derive from first letters
- **Family members sharing YY is intended** — uniqueness lives in the full quoin name, not the 5-char prefix

See minting memo Pattern J (RBS0/JJS0 empirical study) for evidence.

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
| `hm` | HMK (Hard-state Machine Kit) |
| `lmci` | LMCI (Language Model Console Integration) |
| `apc` | APCK (Ann's PHI Clipbuddy Kit) |
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

@Tools/rbk/rbk-claude-tabtarget-context.md

For theurge/ifrit crucible testing work, read `Tools/rbk/rbk-claude-theurge-ifrit-context.md` — covers the iteration loop (kludge, charge, test, ordain), architecture of the two Rust binaries, and how to add new security test cases.

For remote execution, fundus provisioning, foray dispatch, or cross-platform test planning, read `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md` — agent-interpreted inventory of all test targets, their capabilities, and reachability.

@Tools/jjk/vov_veiled/jjk-claude-context.md

@Tools/vvk/vov_veiled/vvk-claude-context.md

@Tools/apck/apck-claude-context.md
