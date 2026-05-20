# Claude Code Project Memory

## Directory Permissions
Full read and edit access is pre-approved for all files in:
- `Tools/`
- `Memos/`
- `../cnmp_CellNodeMessagePrototype/lenses/`

## File Acronym Mappings

Per-kit acronym mappings live in each kit's context file (loaded via `@` includes below).
- RBK: `@Tools/rbk/rbk-claude-acronyms.md`
- BUK: `@Tools/buk/buk-claude-context.md`
- CMK: `@Tools/cmk/vov_veiled/cmk-claude-context.md`
- JJK: `@Tools/jjk/vov_veiled/jjk-claude-context.md`
- VOK: `@Tools/vok/vok-claude-context.md`
- GAD: `Tools/gad/CLAUDE.md` (not `@`-included — loaded only when working in that kit)

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
| `service` | `tt/rbtd-s.TestSuite.service.sh` | GCP credentials | fast + access-probe (4), hallmark-lifecycle (1), batch-vouch (1) = 81 cases |
| `crucible` | `tt/rbtd-s.TestSuite.crucible.sh` | Container runtime | fast + tadmor-security (34), srjcl-jupyter (3), pluml-diagram (5) = 117 cases |
| `complete` | `tt/rbtd-s.TestSuite.complete.sh` | All of the above | All 8 fixtures = 122 cases |

**After code changes**, run the appropriate tier:
- Regime/validation changes → `fast`
- Foundry/credential changes → `service`
- Bottle/sentry/network changes → `crucible`
- Pre-release or decomposition sweep → `complete`

**Single fixture**: `tt/rbtd-r.FixtureRun.{name}.sh` (e.g., `tadmor`, `enrollment-validation`, `regime-smoke`)

**Single case**: `tt/rbtd-s.FixtureCase.sh <fixture> [case-name]` — run one case against an already-charged crucible (no charge/quench). Omit case name to list all cases for the fixture; omit fixture to list all fixtures. Workflow for crucible debugging: charge via `tt/rbw-cC.Charge.{nameplate}.sh`, run individual cases, quench via `tt/rbw-cQ.Quench.{nameplate}.sh` when done.

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

Tabtargets dispatch through `tt/z-launcher.sh` by passing a moorings-launcher *sprue* `{owner}ml_{launcher-id}` (`rbml_*` RBK-authored, `buml_*` BUK-hosted) — an underscore-universe dispatch token kept distinct from the hyphenated colophon. See BCG "Tabtarget Path Indirection" for the rationale.

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

- **Uniform shape `prefixXY_word` or `prefixXYZ_word`** — every quoin carries two or three sub-letters; pick one count per spec
- **Hard 3-letter ceiling** — never mint 4+ letter sub-prefixes
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

@Tools/vok/vok-claude-context.md

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides
- Public project page: https://scaleinv.github.io/recipebottle

## Test Environments

Operator-specific test machines reachable from this station.

- **bujn-winpc** — Windows host (tailnet hostname `rocket`). Formal BURN profile
  at `rbmm_moorings/rbmn_nodes/bujn-winpc/` for BUK caparison/garrison/invigilate work
  under heat ₣A-.
  - Admin SSH: `tt/buw-jpS bujn-winpc <cmd>` (as `bhyslop`).
  - Formal workload: `tt/buw-jws bujn-winpc` (as `bujuw_user`; current garrison
    routes to WSL `rbtww-main`).
  - **Ad-hoc hack — `ssh brad@rocket`** — unprivileged Cygwin login shell.
    Independent of the formal caparison/garrison machinery; safe scratch for
    ad-hoc Windows testing. Setup recipe, gotchas, and teardown live in
    `Memos/memo-20260516-windows-headless-account-anatomy.md`. Repo cloned at
    `~/projects/rbm_alpha_recipemuster` inside brad's session.
- **cerebro** — Linux test host used by JJK fundus scenario tests
  (`Tools/jjk/jjfp_fundus.sh:518`). Tabtargets:
  `tt/jjw-tfP2.ProvisionPhase2.cerebro.sh`,
  `tt/jjw-tfs.TestFundusScenario.cerebro.sh`. Scenario tests marked
  `#[ignore]`; `--ignored` is required to exercise them.

@Tools/rbk/rbk-claude-acronyms.md

@Tools/rbk/rbk-claude-tabtarget-context.md

For theurge/ifrit crucible testing work, read `Tools/rbk/rbk-claude-theurge-ifrit-context.md` — covers the iteration loop (kludge, charge, test, ordain), architecture of the two Rust binaries, and how to add new security test cases.

@Tools/jjk/vov_veiled/jjk-claude-context.md

@Tools/vvk/vov_veiled/vvk-claude-context.md

@Tools/apck/apck-claude-context.md
