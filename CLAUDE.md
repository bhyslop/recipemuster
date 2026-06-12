# Claude Code Project Memory

## Directory Permissions
Full read and edit access is pre-approved for all files in:
- `Tools/`
- `Memos/`
- `../cnmp_CellNodeMessagePrototype/lenses/`

## File Acronym Mappings

Per-kit acronym mappings live in each kit's context file (loaded via `@` includes below).
- RBK: `@Tools/rbk/claude-rbk-acronyms.md`
- BUK: `@Tools/buk/claude-buk-core.md`
- CMK: `@Tools/cmk/claude-cmk-core.md`
- JJK: `@Tools/jjk/claude-jjk-core.md` (+ veiled `@Tools/jjk/vov_veiled/claude-jjk-bhyslop.md`)
- VOK: `@Tools/vok/claude-vok-context.md`
- GAD: `Tools/gad/CLAUDE.md` (not `@`-included — loaded only when working in that kit)

### CNMP Lenses Directory (`../cnmp_CellNodeMessagePrototype/lenses/`)
- **ANCIENT** → `a-roe-ANCIENT.md`
- **ANNEAL**  → `a-roe-ANNEAL-spec-fine.adoc`
- **CRAFT**   → `a-roe-CRAFT-cmodel-format.adoc`
- **METAL**   → `a-roe-METAL-sequences.adoc`
- **MIND**    → `a-roe-MIND-cmodel-semantic.adoc`
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

## Retired Memos

A memo whose work is fully dispositioned (every concern resolved into a pace, an
itch/RBSHR entry, or an explicit decline) moves to `Memos/retired/` with its
basename unchanged. A memo path that no longer resolves has retired — look for
the same basename under `Memos/retired/`. Retired memos are historical record:
read them freely, never resurrect work from them without operator direction.

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

Two Rust build targets. Always use the tabtarget, never raw cargo commands.

**VOW pipeline** (vvk/jjk/cmk kits — parceled for delivery):
- `tt/vow-b.Build.sh` — build vvr binary and install to VVK bin
- `tt/vow-t.Test.sh` — run all kit crate tests
- `tt/vvw-r.RunVVX.sh <cmd>` — run vvx binary with arguments

**Theurge** (rbk's own test infrastructure — dispatches through the unified rbw workbench):
- `tt/rbw-tb.Build.sh` — build theurge crate
- `tt/rbw-tt.Test.sh` — run theurge unit tests

### Test Execution

**Test suites** group fixtures by dependency tier. Composition is owned by
`RBTDRC_SUITES` (`Tools/rbk/rbtd/src/rbtdrc_crucible.rs`); this table summarizes it.
Run the broadest applicable suite:

| Suite | Tabtarget | Dependencies | What it covers |
|-------|-----------|-------------|----------------|
| `fast` | `tt/rbw-ts.TestSuite.fast.sh` | None | 10 fixtures: enrollment-validation, regime-validation, regime-smoke, podvm-resolve, handbook-render, dockerfile-hygiene, foundry-path, recipe-validation, cupel, conformance |
| `service` | `tt/rbw-ts.TestSuite.service.sh` | GCP credentials | fast + access-probe, hallmark-lifecycle, lode-lifecycle, reliquary-lifecycle, wsl-lifecycle, podvm-lifecycle, batch-vouch (17 fixtures) |
| `crucible` | `tt/rbw-ts.TestSuite.crucible.sh` | Container runtime | fast + tadmor, srjcl, pluml (13 fixtures) |
| `complete` | `tt/rbw-ts.TestSuite.complete.sh` | All of the above | service ∪ crucible (20 fixtures) |

**Release/probe suites** — ladders distinguished by project-churn × crucible ×
network posture, not dependency tier:

| Suite | Tabtarget | Precondition | What it covers |
|-------|-----------|-------------|----------------|
| `gauntlet` | `tt/rbw-ts.TestSuite.gauntlet.sh` | None (levies fresh projects) | Release-qualification ladder: marshal-zero state → canonical-establish → onboarding-sequence → fast fixtures → crucibles |
| `skirmish` | `tt/rbw-ts.TestSuite.skirmish.sh` | Canonical depot already levied | Mini-gauntlet: depot→build→crucible chain without project churn |
| `dogfight` | `tt/rbw-ts.TestSuite.dogfight.sh` | Canonical depot already levied | Cloud-build viability probe: ordain → summon → run, no crucible |
| `siege` | `tt/rbw-ts.TestSuite.siege.sh` | None (fully local) | Tadmor self-contained: kludge both vessels + security cases |
| `blockade` | `tt/rbw-ts.TestSuite.blockade.sh` | Depot levied + moriah hallmark ordained | Airgap moriah crucible with credential self-heal |

**After code changes**, run the appropriate tier:
- Regime/validation changes → `fast`
- Foundry/credential changes → `service`
- Bottle/sentry/network changes → `crucible`
- Pre-release or decomposition sweep → `complete`

**Single fixture**: `tt/rbw-tf.FixtureRun.sh <name>` (e.g., `tadmor`, `enrollment-validation`, `regime-smoke`)

**Single case**: `tt/rbw-tc.FixtureCase.sh <fixture> [case-name]` — run one case against an already-charged crucible (no charge/quench). Omit case name to list all cases for the fixture; omit fixture to list all fixtures. Workflow for crucible debugging: charge via `tt/rbw-cC.Charge.{nameplate}.sh`, run individual cases, quench via `tt/rbw-cQ.Quench.{nameplate}.sh` when done.

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

Tabtargets dispatch through `tt/z-launcher.sh`, naming their launcher in the `BURD_LAUNCHER` config line (a bare `launcher.<id>_workbench.sh` basename) and exec'ing the trampoline with a byte-identical, token-free exec line. See BCG "Tabtarget Path Indirection" for the rationale.

### Extended Namespace Checklist

When minting, enumerate ALL namespaces the system touches:

| Namespace | Pattern | Example |
|-----------|---------|---------|
| Git refs | `refs/{prefix}/...` | `refs/vvg/locks/*` |
| Slash commands | `/{prefix}-{noun}` | `/vvc-commit` |
| Command files | `.claude/commands/{cmd}.md` | `vvc-commit.md` |
| Environment vars | `{PREFIX}_NAME` | `VVG_SIZE_LIMIT` |
| Target repo paths | `Tools/{kit}/...` | `Tools/vvk/bin/vvx` |
| BURE tweak names | `buo{proj}_{name}` | `buorb_graft_image` |
| JSON wire keys | `{sprue}_{term}` (one sprue per RB-authored wire format; foreign schemas keep foreign keys) | `rblv_digest` |

This is not exhaustive. The principle: **any persistent name anywhere is in the mint universe.**

BURE tweak-name detail: the `buo` sprue is a reserved prefix for `BURE_TWEAK_NAME` values (the test-seam channel). BURE enforces the *shape* (`buo<segment>_…`) only — never specific names — so a typo'd/unregistered tweak fails loud instead of silently no-op'ing, and `grep buo` is the virtual registry (no central list). `buost_` is the reserved segment for BUK/test-stub placeholders (BUK's own, since `buobu_` is degenerate).

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

### Word Selection Discipline (MCM "Word Selection")

Prefix rules govern the left half of a name; these govern the word itself. When minting any persistent name — quoin word-parts, verbs, nouns, colophon words:

- **Semantic uniqueness** repo-wide: one meaning per word, ever. (Pre-MVP operator ruling: a retired word may be re-minted after a deliberate eviction sweep — the grep gate proves the sweep.)
- **Vocabulary isolation**: no *trodden words* (`mcm_trodden_word`) — vocabulary heavy in ambient software prose (open, close, run, lock, task, grant) is disqualified regardless of fit.
- **Grep gate**: repo-wide grep must land clean before adoption.
- **Rare but real**: distinctive words whose true meaning does semantic work; concrete over abstract; never abbreviations.
- **Mint into an asterism** (`mcm_asterism`): join a coherent metaphor family with an audible register (equestrian, ecclesiastical, diplomatic, civic).
- **Exposure ladder** (`mcm_ashlar`/`mcm_hearting`): hearting (interior, prefix-only) → quoin (catalogued default) → ashlar (operator-facing). An ashlar must be fair-faced (first-contact actionable; `mcm_cold_probe` tests it), draws from the coffer (bounded operator vocabulary), and registers on the project's broadside (RB's broadside is the README glossary). Words in error output are ashlar.

Full doctrine: MCM "Word Selection". Constrains births, not the living.

### Minting Workflow

Before minting new prefixes:
1. **Enumerate namespaces** — list every place this name will appear (code, refs, commands, env vars, target paths...)
2. **Check word selection** — apply MCM Word Selection to the word part (grep gate, no trodden words, asterism fit)
3. **Check reserved suffixes** — ensure the suffix matches intended type
4. **Verify terminal exclusivity** — search existing trees, check the memo
5. **Document the allocation** — add to prefix map in relevant heat/spec

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
- **ACG**: Allocation Discipline — reference the home, don't recreate (values → constants, concepts → quoin-refs) — see ACG
- **Specs**: Linked term structure earns its three-part form only when the concept warrants anchoring

When evaluating any new pattern, extraction, or structural choice, the litmus test is: "Does this element earn its existence?" If not, it doesn't belong.

<!-- Partnership rules of engagement (never distributed; hand-maintained outside
     the managed block). Salutation leads — the wake-up greeting — then the stance.
     Companion detail at Tools/cmk/claude-cmk-roe-detail.md is read on
     demand, not @-included. -->
@Tools/cmk/claude-cmk-salutation.md

@Tools/cmk/claude-cmk-roe.md

<!-- Distributable-kit guidance: managed @-include block (mirrors what consumer
     repos receive via vvx_emplace). Edit content in the @-targets, not here;
     `tt/vow-F.Freshen.sh` regenerates this block from the kit registry. -->
<!-- MANAGED:VVK-INCLUDES:BEGIN -->
@Tools/buk/claude-buk-core.md
@Tools/cmk/claude-cmk-core.md
@Tools/jjk/claude-jjk-core.md
@Tools/vvk/claude-vvk-core.md
<!-- MANAGED:VVK-INCLUDES:END -->

<!-- rbm-only veiled guidance (never distributed); hand-maintained outside the block -->
@Tools/jjk/vov_veiled/claude-jjk-bhyslop.md

@Tools/vok/claude-vok-context.md

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides
- Public project page: https://scaleinv.github.io/recipebottle

## Test Environments

Operator-specific test machines reachable from this station.

- **bujn-winpc** — Windows host, tailnet hostname `rocket`. Formal BURN profile
  at `rbmm_moorings/rbmn_nodes/bujn-winpc/` for BUK caparison/garrison/invigilate
  work under heat ₣A-. **Consolidated access reference + live account state:**
  `Memos/memo-20260516-windows-headless-account-anatomy.md`.
  - Admin SSH: `tt/buw-jpS bujn-winpc <cmd>` (as `bhyslop`; cmd.exe default shell,
    so prepend `powershell -Command` / `bash -c` as the task needs).
  - Formal workload: `tt/buw-jws bujn-winpc` (as `bujuw_user`; garrison routes to
    WSL `rbtww-main`). Owned by the garrison ceremony, not hand-edits.
  - Ad-hoc test accounts (pubkey-only, independent of the formal garrison — safe
    scratch). Repo cloned at `~/projects/rbm_alpha_recipemuster`:
    - `ssh brad@rocket` — interactive Cygwin login shell (human use; ignores a passed command).
    - `ssh cygwin@rocket "<cmd>"` (one-shot) or `ssh -t cygwin@rocket` (interactive) — Cygwin, full shell semantics.
    - `ssh wsl@rocket "<cmd>"` (one-shot) or `ssh -t wsl@rocket` (interactive) — WSL Ubuntu 24.04 as root; **Docker daemon live — container tests run here**.
  - Legacy LAN aliases `winhost-{wsl,cyg,ps}` (192.168.86.27) are currently
    unreachable; use the `rocket` tailnet paths above.
- **cerebro** — Linux test host (Ubuntu 24.04). Direct access: `ssh cerebro`
  (user `bhyslop`, key `~/.ssh/id_ed25519`). Also the remote fundus for JJK
  scenario tests: tabtargets `tt/jjw-tfP2.ProvisionPhase2.cerebro.sh`,
  `tt/jjw-tfs.TestFundusScenario.cerebro.sh` (tests marked `#[ignore]`,
  `--ignored` required; fundus accounts must be provisioned on cerebro first).
- **localhost** — local fundus for JJK scenario tests via `jjfu-*` ssh aliases
  (`jjfu-full`, `jjfu-nogit`, `jjfu-nokey`, `jjfu-norepo`).

@Tools/rbk/claude-rbk-acronyms.md

@Tools/rbk/claude-rbk-tabtarget-context.md

For theurge/ifrit crucible testing work, read `Tools/rbk/claude-rbk-theurge-ifrit-context.md` — covers the iteration loop (kludge, charge, test, ordain), architecture of the two Rust binaries, and how to add new security test cases.

@Tools/apck/claude-apck-context.md
