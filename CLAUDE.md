# Claude Code Project Memory

## Directory Permissions
Full read and edit access is pre-approved for all files in:
- `lenses/`
- `Tools/`
- `../cnmp_CellNodeMessagePrototype/lenses/`

## File Acronym Mappings

### Lenses Directory (`lenses/`)
- **RBRN**  → `RBRN-RegimeNameplate.adoc`
- **RBRR**  → `RBRR-RegimeRepo.adoc`
- **RBS0**  → `RBS0-SpecTop.adoc`
- **RBSRV** → `RBSRV-RegimeVessel.adoc`
- **CRR**   → `CRR-ConfigRegimeRequirements.adoc`
- **RBSDC** → `RBSDC-depot_create.adoc`
- **RBSDD** → `RBSDD-depot_destroy.adoc`
- **RBSDI** → `RBSDI-director_create.adoc`
- **RBSDL** → `RBSDL-depot_list.adoc`
- **RBSGR** → `RBSGR-governor_reset.adoc`
- **RBSGS** → `RBSGS-GettingStarted.adoc`
- **RBSAA** → `RBSAA-ark_abjure.adoc`
- **RBSAB** → `RBSAB-ark_beseech.adoc`
- **RBSAC** → `RBSAC-ark_conjure.adoc`
- **RBSAS** → `RBSAS-ark_summon.adoc`
- **RBSAX** → `RBSAX-access_setup.adoc`
- **RBSID** → `RBSID-image_delete.adoc`
- **RBSIR** → `RBSIR-image_retrieve.adoc`
- **RBSOB** → `RBSOB-oci_layout_bridge.adoc`
- **RBSPE** → `RBSPE-payor_establish.adoc`
- **RBSPI** → `RBSPI-payor_install.adoc`
- **RBSPR** → `RBSPR-payor_refresh.adoc`
- **RBSQB** → `RBSQB-quota_build.adoc`
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
- **RBOB** → `rbw/rbob_bottle.sh`
- **RBQ**  → `rbw/rbq_Qualify.sh` (Qualification orchestrator - tabtarget/colophon/nameplate health)
- **RBV**  → `rbw/rbv_PodmanVM.sh`

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
- **AXMCM** → `cmk/AXMCM-ClaudeMarkConceptMemo.md`

#### JJK Subdirectory (`Tools/jjk/`)
- **JJS0** → `jjk/vov_veiled/JJS0-GallopsData.adoc` (Gallops data model specification - main file)
- **JJSCCH** → `jjk/vov_veiled/JJSCCH-chalk.adoc`
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
- **Pace**: Discrete action within a heat; can be bridled for autonomous execution via `/jjc-pace-bridle`
- **Itch**: Future work (any detail level), lives in jji_itch.md
- **Scar**: Closed work with lessons learned, lives in jjs_scar.md
- **Spook**: Team infrastructure stumble — any workflow failure improvable with deft attention. Capture as a pace when encountered, don't lose the current thread.

**Identities vs Display Names:**
- **Firemark**: Heat identity (`₣AA` or `AA`). Used in CLI args and JSON keys.
- **Coronet**: Pace identity (`₢AAAAk` or `AAAAk`). Used in CLI args and JSON keys.
- **Silks**: kebab-case display name. Human-readable only — NOT usable for lookups.

When a command takes `<firemark>` or `<coronet>`, provide the identity, not the silks.

- Target repo dir: `.`
- JJ Kit path: `Tools/jjk/README.md`

**JJ Slash Command Reference:**

ALWAYS read the corresponding slash command before attempting JJ operations.

**CRITICAL**: JJK CLI syntax is non-standard. Do NOT guess based on common CLI conventions.
- Dockets go via stdin, not `--docket`
- Positioning uses `--move X --first`, not `--position first`
- **Verb names are NOT jjx_ subcommands**: there is no `jjx_slate`, `jjx_mount`, `jjx_bridle`, `jjx_notch`, `jjx_groom`, etc. Slash commands wrap differently-named jjx_ commands. Read the slash command to find the actual CLI invocation.
- Read the slash command to see the exact `./tt/vvw-r.RunVVX.sh jjx_*` invocation pattern.

**Heredoc for stdin**: Use `cat <<'DELIM' | jjx_*` pattern with quoted delimiter. The delimiter must not appear alone on any line in the content — if content shows heredoc examples with `EOF`, use a different delimiter like `DOCKET` or `PACESPEC`.

| When you need to... | Read first |
|---------------------|------------|
| Add a new pace | /jjc-pace-slate |
| Refine pace docket | /jjc-pace-reslate |
| Bridle for autonomous execution | /jjc-pace-bridle |
| Mark pace complete | `jjx_close` |
| Commit with JJ context | /jjc-pace-notch |
| Execute next pace | /jjc-heat-mount |
| Review heat plan | /jjc-heat-groom |
| Evaluate bridleable paces | /jjc-heat-quarter |
| Reorder paces | /jjc-heat-rail |
| Add steeplechase marker | /jjc-heat-chalk |
| Pause or resume heat | /jjc-heat-furlough |
| Draft paces between heats | /jjc-heat-restring |
| Retire completed heat | /jjc-heat-retire |
| View heat or pace | `jjx_show` |

**Quick Verbs** — When user says just the verb, invoke the corresponding command:

| Verb | Noun | Command |
|------|------|---------|
| muster | heats | `jjx_list` |
| parade | heat/pace | `jjx_show` |
| scout | heats | `jjx_search` |
| braid | heats | /jjc-heat-braid |
| nominate | heat | `jjx_create` |
| mount | heat/pace | /jjc-heat-mount |
| groom | heat | /jjc-heat-groom |
| quarter | heat | /jjc-heat-quarter |
| rail | heat | /jjc-heat-rail |
| furlough | heat | /jjc-heat-furlough |
| retire | heat | /jjc-heat-retire-dryrun |
| garland | heat | /jjc-heat-garland |
| restring | heat | /jjc-heat-restring |
| slate | pace | /jjc-pace-slate |
| reslate | pace | /jjc-pace-reslate |
| bridle | pace | /jjc-pace-bridle |
| notch | pace | /jjc-pace-notch |
| wrap | pace | `jjx_close` |

**CLI Command Reference:**

```
jjx_show [TARGET] [--detail] [--remaining]
jjx_list [--status racing|stabled|retired]
jjx_create --silks SILKS
jjx_enroll FIREMARK --silks SILKS [--first|--before C|--after C] <stdin
jjx_reorder FIREMARK --move C --first|--last|--before C|--after C
jjx_alter FIREMARK [--racing|--stabled] [--silks SILKS]
jjx_record CORONET FILE [FILE...] [--intent "msg"]
jjx_close CORONET
jjx_log FIREMARK [--limit N]
jjx_search PATTERN [--actionable]
jjx_archive FIREMARK [--execute] [--size-limit BYTES]
jjx_transfer FIREMARK --to FIREMARK <stdin
jjx_continue FIREMARK
jjx_mark CORONET --marker M --description "text"
jjx_paddock FIREMARK [<stdin for set]
jjx_relocate CORONET --to FIREMARK [--first|--before C|--after C]
jjx_orient [FIREMARK]
jjx_revise_docket CORONET <stdin
jjx_arm CORONET <stdin
jjx_relabel CORONET --silks "name"
jjx_drop CORONET
jjx_get_brief CORONET
jjx_get_coronets FIREMARK [--remaining] [--rough]
jjx_landing CORONET AGENT <stdin
jjx_validate [--file PATH]
```

**Composition Recipes:**

Common command combinations using `&&`:

```bash
# Revise docket and rename:
jjx_revise_docket C <docket_stdin && jjx_relabel C --silks "name"

# Bridle with revised docket:
jjx_revise_docket C <docket_stdin && jjx_arm C <warrant_stdin
```

**Key points:**
- `jjx_show` takes firemark OR coronet directly as sole argument — NO flags for target selection
  - Heat overview: `jjx_show AF`
  - Single pace: `jjx_show AFAAb`
  - Valid flags: `--detail`, `--remaining` only
- NEVER invent CLI flags — run `--help` first or read the slash command
- `jjx_orient` output includes `pace_coronet` (next actionable pace) — no separate show call needed to find what's next

**Commit Discipline:**
When working on a heat, ALWAYS use `/jjc-pace-notch` for commits. NEVER use `jjx_record` directly — the slash command handles context detection. Never bypass heat/pace affiliation and steeplechase tracking.

Notch handles two cases:
- **Pace-affiliated**: Active pace provides context (coronet in commit)
- **Heat-affiliated**: No active pace, but commit is part of heat work (firemark only)

When user says "notch", invoke `/jjc-pace-notch` — don't second-guess based on pace state.

**Multi-Officium Discipline:**
Multiple Claude officia (concurrent git-activity streams, not sessions — see VOS `vost_officium`) may work concurrently in the same repo. The explicit file list in `/jjc-pace-notch` enables orthogonal commits.

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
- Staging wrong? Just run `/jjc-pace-notch` with the correct file list — it handles staging
- Made a mistake? Make a new commit that fixes it — additive, not destructive
- Confused by repo state? ASK the user — another officium may be mid-work
- Need to undo something? Explain the situation to the user and let them decide

**Build & Run Discipline:**
Always run these after Rust code changes:
- `tt/vow-b.Build.sh` — Build
- `tt/vvw-r.RunVVX.sh` — Run VVX

**JJX Commands Are Self-Committing:**
`jjx_enroll`, `jjx_close`, `jjx_record`, and other state-mutating jjx commands commit gallops changes internally. Do NOT follow them with a separate `vvx_commit` — the tree will already be clean. If a commit command says "Nothing to commit", check `git status --short` and accept the result.

**Diagnose Before Escalating:**
When a command fails, check the simplest explanation first. "Nothing to commit" means the tree is clean — verify with `git status`, don't try creative workarounds. "Unexpected argument" means wrong CLI syntax — read `--help`, don't guess. One diagnostic command beats three speculative retries.

**Important**: New commands are not available in this installation session. You must restart Claude Code before the new commands become available.

### Bridleability Assessment

When evaluating whether a pace is ready for autonomous execution (bridled state), apply these criteria:

**Bridleable** (can bridle for autonomous execution) — ALL must be true:
- **Mechanical**: Clear transformation, not design work
- **Pattern exists**: Following established pattern, not creating new one
- **No forks**: Single obvious approach, not "we could do X or Y"
- **Bounded**: Touches known files, not "find where this should go"

**NOT bridleable** (needs human judgment):
- Language like "define", "design", "architect", "decide"
- Establishing new patterns others will follow
- Multiple valid approaches requiring human choice
- Scope unclear or requires judgment calls

**Examples:**
- ✓ Bridleable: "Rename function `getCwd` to `getCurrentWorkingDirectory` across codebase"
- ✓ Bridleable: "Add error handling to `fetchUser` following pattern in `fetchOrder`"
- ✗ Not bridleable: "Define KitAsset struct and registry pattern" (design decisions)
- ✗ Not bridleable: "Improve performance of dashboard" (unclear scope, many approaches)

### Wrap Discipline

**NEVER auto-wrap a pace.** Always ask the user explicitly: "Ready to wrap ₢XXXXX?" and wait for confirmation before running `jjx_close`. The user decides when work is complete, not the agent.

This applies to:
- Direct work on rough paces
- Spawned agents executing bridled paces
- Any situation where work appears "done"

When work is complete, report outcomes and ask. Do not wrap.

When wrapping (after user confirms), always pipe a one-line summary of the work via stdin:
`echo "Added bitmap displays to orient output" | jjx_close CORONET`
The agent always has context about what was accomplished — include it.

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
