## Prefix Naming Discipline ("mint")

When asked to "mint" names, apply these rules. This is the universal minting
doctrine, CMK-homed; each host project keeps its own **Project Prefix Registry**
(the prefix → project table) in its CLAUDE.md. Deep-doctrine pointers below
(MCM, BUS0, BCG) resolve where those kits' veiled specs are present — they are
depth, not prerequisites. Full study (rbm-local provenance):
`Memos/memo-20260110-acronym-selection-study.md`

### Two Universes

**Primary Universe** — ANY persistent identifier, regardless of where it lives. This includes the obvious (code, docs, functions, variables, directories) AND the easy-to-miss: git refs, slash commands, environment variables, paths in target repos, configuration keys. **If it's a name that persists, it's in scope.** Prefixes must be globally unique and respect terminal exclusivity.

**Tabtarget Universe** — launchers in `tt/`. These are *colophons* referencing the primary universe. `rbw-` points to the `rbw` workbench; it doesn't consume new prefix space.

### Core Rules

**Rule 1 - Project Prefix**: Names start with a 2-4 char project ID registered in the host project's **Project Prefix Registry** (in its CLAUDE.md).

**Rule 2 - Terminal Exclusivity**: A prefix either IS a name or HAS children, never both.
- `rbg` has children (`rbga`, `rbgb`...) → `rbg` cannot name a thing
- `rbi` names Image module → `rbia`, `rbib` forbidden

### Primary Universe Patterns

| Domain | Pattern | Example |
|--------|---------|---------|
| Code files | `prefix_word.ext` (lowercase, prefer 1 word) | `rbga_registry.sh` |
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
| BURE tweak names | `buo{proj}_{name}` | `buorb_ensconce_stamp` |
| JSON wire keys | `{sprue}_{term}` (one sprue per RB-authored wire format; foreign schemas keep foreign keys) | `rblv_digest` |

This is not exhaustive. The principle: **any persistent name anywhere is in the mint universe.**

BURE tweak-name detail: the `buo` sprue is a reserved prefix for `BURE_TWEAK_NAME` values (the test-seam channel). BURE enforces the *shape* (`buo<segment>_…`) only — never specific names — so a typo'd/unregistered tweak fails loud instead of silently no-op'ing, and `grep buo` is the virtual registry (no central list). The segment after `buo` names the owning kit; `buost_` is BUK's own segment (BUK as consumer, since `buobu_` would be degenerate), homing both BUK-owned behavioral tweaks (the `buost_regime_poison` seam) and BUK self-test stubs (`buost_example`). Tweak *doctrine* — what a tweak is for, one-at-a-time by design, a suite reserving the slot for a standing guard — lives in BUS0 "Tweak Mechanism"; the live behavioral census is stamp (`buorb_ensconce_stamp`), poison (`buost_regime_poison`), the reveille-tier credless guard (`buorb_credless_guard`), the HTTP fault seam (`buorb_http_fault`), and the re-don cadence override (`buorb_redon_cadence`). A deliberate-rejection gate asserts its named exit code from the precision band, never bare nonzero — see BCG "Precision Exit-Code Band".

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

### Word Selection Discipline (MCM "Lapidary")

Prefix rules govern the left half of a name; these govern the word itself. When minting any persistent name — quoin word-parts, verbs, nouns, colophon words:

- **Semantic uniqueness** repo-wide: one meaning per word, ever. (Pre-MVP operator ruling: a retired word may be re-minted after a deliberate eviction sweep — the grep gate proves the sweep.)
- **Vocabulary isolation**: no *trodden words* (`mcm_trodden_word`) — vocabulary heavy in ambient software prose (open, close, run, lock, task, grant) is disqualified regardless of fit.
- **Grep gate**: repo-wide grep must land clean before adoption.
- **Rare but real**: distinctive words whose true meaning does semantic work; concrete over abstract; never abbreviations.
- **Mint into an asterism** (`mcm_asterism`): join a coherent metaphor family with an audible register (equestrian, ecclesiastical, diplomatic, civic).
- **Exposure ladder** (`mcm_ashlar`/`mcm_hearting`): hearting (interior, prefix-only) → quoin (catalogued default) → ashlar (operator-facing). An ashlar must be fair-faced (first-contact actionable; `mcm_cold_probe` tests it), draws from the coffer (bounded operator vocabulary), and registers on the project's broadside (RB's broadside is the README glossary). Words in error output are ashlar.
- **Revetment** (`mcm_revetment`/`mcm_spall`): the maintained public face presenting the veiled interior in dressed form; a leaked interior name on it is a *spall*. RB's revetment is README.md — the same file that is RB's broadside above. Concept home and the four-invariant face law: MCM "Lapidary", not restated here.

Full doctrine: MCM "Lapidary". Constrains births, not the living.

### Minting Workflow

Before minting new prefixes:
1. **Enumerate namespaces** — list every place this name will appear (code, refs, commands, env vars, target paths...)
2. **Check word selection** — apply MCM Lapidary to the word part (grep gate, no trodden words, asterism fit)
3. **Check reserved suffixes** — ensure the suffix matches intended type
4. **Verify terminal exclusivity** — search existing trees, check the memo
5. **Document the allocation** — add to prefix map in relevant heat/spec
