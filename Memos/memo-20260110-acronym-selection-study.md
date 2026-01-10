# Prefix Naming System Memo

## Status
Draft v0.6 — Ten assessments complete, all anchored (2026-01-10)

## Purpose
Codify the naming convention system used across projects for human-AI collaboration. The system creates unique, prefix-free identifiers that enable unambiguous reference to concepts, files, and modules in conversation.

## Core Rules (Hypothesized)

### Rule 1: Project Prefix
Every named entity begins with a 2-3 character project identifier.

| Prefix | Domain |
|--------|--------|
| `rb` | Recipe Bottle |
| `gad` | Google AsciiDoc Differ |
| `pb` | Paneboard |
| `bu` | Bash Utilities (Kit) |
| `jj` | Job Jockey |
| `wrs` | Ward Realm System |
| `mcm` | Meta Concept Model |
| `axl` | Axiom Lexicon |
| `crg` | Config Regime |
| `srf` | Study Raft |

### Rule 2: Prefix-Free (Terminal Exclusivity)
A prefix either **IS a name** or **HAS children**, never both.

- If `rbg` has children (`rbga`, `rbgb`, `rbgc`...), then `rbg` itself cannot name a specific thing
- If `rbi` names a specific thing (Image module), then `rbia`, `rbib` are forbidden
- This ensures unambiguous parsing: any prefix uniquely identifies one entity

### Rule 3: Hierarchical Semantics
Letters after the project prefix form a collapsed namespace hierarchy.

```
rb   = Recipe Bottle (project root)
├─ rbg  = rb + Google
│  ├─ rbga = rb + Google + Artifact Registry
│  ├─ rbgb = rb + Google + Buckets
│  ├─ rbgc = rb + Google + Constants
│  ├─ rbgg = rb + Google + Governor
│  ├─ rbgi = rb + Google + IAM
│  ├─ rbgm = rb + Google + Manual Procedures
│  ├─ rbgo = rb + Google + OAuth
│  ├─ rbgp = rb + Google + Payor
│  └─ rbgu = rb + Google + Utility
├─ rbh  = rb + GitHub (h for Hub)
│  ├─ rbha = rb + GitHub + Actions
│  ├─ rbhcr = rb + GitHub + Container Registry
│  ├─ rbhh = rb + GitHub + Host
│  └─ rbhr = rb + GitHub + Remote
├─ rbi  = rb + Image (TERMINAL)
├─ rbk  = rb + Coordinator (TERMINAL)
├─ rbl  = rb + Locator (TERMINAL)
├─ rbob = rb + Bottle (TERMINAL)
└─ rbv  = rb + PodmanVM (TERMINAL)
```

### Rule 4: Case Convention by Domain

| Domain | Pattern | Example |
|--------|---------|---------|
| Code files (bash) | `prefix_word.sh` | `rbga_ArtifactRegistry.sh` |
| Code files (rust) | `prefix_word.rs` | `pbgc_core.rs` |
| Documentation | `prefix-ACRONYM-Words.ext` | `rbw-RBAGS-AdminGoogleSpec.adoc` |
| AsciiDoc attributes | `:prefix_term:` | `:rbw_depot:` |

### Rule 5: CAPS Indicate Mnemonic Acronym
The ALL-CAPS portion in documentation filenames is a human-memorable acronym.

- `RBAGS` = Recipe Bottle Admin Google Spec
- `GADM` = Google AsciiDoc Differ Memo
- `BCG` = Bash Console Guide

These appear only in documentation/specification files, not in code modules.

### Rule 6: Acronym Embeds Project Prefix (in docs)
Documentation acronyms typically embed the project prefix:

- `RB`AGS, `RB`RN, `RB`S — all start with RB
- `GAD`M, `GAD`S, `GAD`P — all start with GAD
- Exception: Some cross-cutting concepts use different prefixes (e.g., `BCG` under `bpu-`)

## Terminology

- **Prefix**: The lowercase identifier that uniquely names an entity
- **Terminal prefix**: A prefix that names a specific thing (has no children)
- **Non-terminal prefix**: A prefix that only groups children (does not name a thing itself)
- **Project ID**: The 2-3 character root prefix identifying a project/domain
- **Acronym**: The ALL-CAPS mnemonic appearing in documentation filenames

## Application Domains

Where this pattern is (or should be) applied:

1. **File naming** — both code and documentation
2. **AsciiDoc attribute references** — `:prefix_term:` mappings
3. **Bash function naming** — `prefix_function()` pattern
4. **Rust module naming** — `prefix_description.rs`
5. **Directory naming** — `prefix/` subdirectories (e.g., `Tools/buk/`, `Tools/gad/`)
6. **Git branch naming** — TBD
7. **AsciiDoc anchors** — `[[prefix_term]]` definitions

---

## Health Assessments

### Assessment 1: MCM Self-Application
- **Private**: `https://github.com/bhyslop/recipemuster.git` · `Tools/cmk/mcm-MCM-MetaConceptModel.adoc`
- **Local**: `/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk/mcm-MCM-MetaConceptModel.adoc`
- **Pattern expected**: Single category `mcm_`, consistent anchor↔attribute alignment
- **Conformance**: HIGH

**Observations:**
- Single category prefix `mcm_` used throughout
- All 50+ attributes follow `:mcm_term:` pattern
- Anchors match: `[[mcm_concept_model]]` ↔ `:mcm_concept_model:`
- Variant suffixes consistent: `_s` (plural), `_p` (possessive), `_ed`, `_ing`
- Column alignment at multiples of 10
- Category group header: `// mcm_: Meta Concept Model`

**Anomalies:**
- `mcm_category_ies` uses non-standard `_ies` suffix (for "categories")
- `mcm_task_lens_es` uses `_es` suffix (for "lenses")
- These are valid English plurals but deviate from simple `_s` pattern

**Notes:** MCM is the spec for the pattern. It demonstrates what conformance looks like. The irregular plurals (`_ies`, `_es`) suggest Rule: variant suffixes match English morphology, not rigid `_s`.

---

### Assessment 2: RBAGS AsciiDoc Attributes
- **Private**: `https://github.com/bhyslop/recipemuster.git` · `lenses/rbw-RBAGS-AdminGoogleSpec.adoc`
- **Local**: `/Users/bhyslop/projects/rbm_alpha_recipemuster/lenses/rbw-RBAGS-AdminGoogleSpec.adoc`
- **Pattern expected**: Multiple category prefixes, hierarchical structure
- **Conformance**: HIGH

**Category Prefixes Declared (16 distinct):**
```
rbtr_   RB Term Role (role identities)
rbtgi_  RB Term Google Instance (specific instances)
rbtgo_  RB Google Operation (workflows)
rbtoe_  RB Term Orchestration Embodiment (patterns)
gcp_    Google Cloud Platform (core concepts)
gar_    Google Artifact Registry
gcs_    Google Cloud Storage
gcb_    Google Cloud Build
giam_   Google IAM
rbrr_   Repository Regime
rbra_   Account Regime
rbrp_   Payor Regime
rbro_   OAuth Regime
rbrv_   Vessel Regime
rbev_   Environment Variables Regime
rbbc_   Bash Console Control
rbhg_   Human Guide Control
at_     Architectural Term (cross-reference)
```

**Observations:**
- ~180 attribute references, all conformant to declared categories
- Anchors consistently match attribute prefixes
- Strachey bracket annotations `// ⟦...⟧` used systematically
- Column alignment maintained within category groups
- Sub-documents included with `include::rbw-RBS*-*.adoc[]`

**Anomalies:**
- `at_` prefix is legacy cross-reference (not `rb*`)
- Mixed operation prefixes: `rbtgo_` vs `rbtoe_` — semantic distinction exists but naming overlap (`tgo` vs `toe`) is subtle
- Some anchors like `[[xref_AXLA]]` referenced but not defined in this file (external reference)

**Notes:** RBAGS demonstrates multi-category management at scale. The `rbtr_`/`rbtgi_`/`rbtgo_`/`rbtoe_` family shows 4-letter prefixes within `rb` namespace. Pattern: `rb` + `t` (term) + category letter.

---

### Assessment 3: RBM File Tree Naming
- **Private**: `https://github.com/bhyslop/recipemuster.git` · `/` (root)
- **Local**: `/Users/bhyslop/projects/rbm_alpha_recipemuster/`
- **Pattern expected**: `prefix_word.sh` for code, `prefix-ACRONYM-Words.ext` for docs
- **Conformance**: MEDIUM-HIGH

**Code Files (conformant):**
```
rbf_Foundry.sh, rbga_ArtifactRegistry.sh, rbgb_Buckets.sh
rbgc_Constants.sh, rbgg_Governor.sh, rbgi_IAM.sh
rbgm_ManualProcedures.sh, rbgo_OAuth.sh, rbgp_Payor.sh
rbgu_Utility.sh, rbi_Image.sh, rbk_Coordinator.sh
rbl_Locator.sh, rbob_bottle.sh, rbv_PodmanVM.sh
gadf_factory.py, gadib_base.js, gadic_cascade.css
gadie_engine.js, gadiu_user.js, gadiw_webpage.html
buc_command.sh, bud_dispatch.sh, bug_guide.sh
but_test.sh, buv_validation.sh, buw_workbench.sh
```

**Documentation Files (conformant):**
```
rbw-RBAGS-AdminGoogleSpec.adoc, rbw-RBS-Specification.adoc
rbw-RBRN-RegimeNameplate.adoc, rbw-RBRR-RegimeRepo.adoc
GADM-DualViewImplementationSpec.md, GADS-GoogleAsciidocDifferSpecification.adoc
bpu-BCG-BashConsoleGuide.md, mcm-MCM-MetaConceptModel.adoc
```

**Anomalies/Non-conformant:**
- `bhyslop-nopasswd` — personal config, no prefix
- `bottle_*.recipe` — uses `bottle_` not `rb*`
- `env-prop.sh`, `entrypoint.sh` — generic names, no prefix
- `machine_setup_PROTOTYPE_rule.sh` — no prefix
- `netdiag.py`, `namegenie.py` — no prefix
- `spd.strip-podman-docs.py` — `spd` prefix but not in declared family
- Various `Snnp-*.sh` files — `Snnp` prefix (Study network namespace pod?)
- `vsp-*.vpj`, `vsw-*.vpw` — SlickEdit project files with `vsp`/`vsw` prefix
- `ttc.CreateTabtarget.sh`, `ttx.FixTabtargetExecutability.sh` — `tt*` prefix
- `podman-*.md` — external documentation, no RB prefix

**Terminal vs Non-Terminal Analysis:**
- `rbg` is non-terminal → children `rbga`, `rbgb`, `rbgc`, `rbgg`, `rbgi`, `rbgm`, `rbgo`, `rbgp`, `rbgu`
- `rbh` is non-terminal → children `rbha`, `rbhcr`, `rbhh`, `rbhr`
- `rbi`, `rbk`, `rbl`, `rbv`, `rbf`, `rbob` are all TERMINAL (no children) ✓
- `gadi` is non-terminal → children `gadib`, `gadic`, `gadie`, `gadiu`, `gadiw`
- `gadf` is TERMINAL ✓

**Notes:** Core RB files are highly conformant. Anomalies fall into categories:
1. External/imported files (podman docs)
2. Personal/local config files
3. Study/experimental files (`Snnp-*`)
4. Tool-specific files (SlickEdit `vs*-*`)
5. Older files predating convention

---

### Assessment 4: Job Jockey (JJK) Prefix System
- **Private**: `https://github.com/bhyslop/recipemuster.git` · `Tools/jjk/`
- **Local**: `/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/`
- **Pattern expected**: `jj*` prefix family, portable kit pattern
- **Conformance**: HIGH (mature, purposeful design)

**JJ Prefix Hierarchy (3-level):**
```
jj (non-terminal)
├─ jja  Action (slash commands, installation)
├─ jjb  Brand (version tracking)
├─ jjc  Chase (steeplechase logs)
├─ jjg  aGent (future)
├─ jjh  Heat (bounded initiatives)
├─ jji  Itch (future work backlog)
├─ jjk  sKill (future)
├─ jjl  Ledger (version registry)
├─ jjm  Memory (state directory)
├─ jjn  Notch (JJ-aware commits)
├─ jjs  Scar (closed work)
├─ jjt  Test (test suite)
├─ jju  Utility (studbook ops, CLI)
└─ jjw  Workbench (command routing)
```

**Key Observations:**
- **`jj` is non-terminal**, with 13+ terminal children
- **No sub-prefixes** (no `jjaa_`, `jjab_`) — flat at second level
- **`z` prefix for private functions**: `zjjw_*`, `zjju_*` (follows BUK pattern)
- **Cross-domain isolation**: JJ* completely separate from RB*, BU*, GAD*, CMK*

**File Naming Patterns:**
- Code: `jj{cat}_{role}.sh` (e.g., `jja_arcanum.sh`, `jjw_workbench.sh`)
- Data: `jj{cat}_{content}.md/.json` (e.g., `jji_itch.md`, `jjl_ledger.json`)
- Commands: `jja-{action}.md` (kebab-case for Claude slash commands)

**Function Naming:**
- Public: `{prefix}_{function}()` (e.g., `jjw_install()`)
- Private: `z{prefix}_{function}()` (e.g., `zjjw_compute_source_hash()`)

**Notes:** JJK demonstrates a portable kit pattern — same prefixes work across any repo where installed. The `z` prefix for private/internal functions is a cross-kit convention shared with BUK.

---

### Assessment 5: RBS vs RBAGS Evolution
- **Private**: `https://github.com/bhyslop/recipemuster.git` · `lenses/rbw-RBS-Specification.adoc`
- **Local**: `/Users/bhyslop/projects/rbm_alpha_recipemuster/lenses/rbw-RBS-Specification.adoc`
- **Pattern expected**: Earlier iteration of RBAGS patterns
- **Conformance**: MEDIUM (Phase 1 maturity)

**RBS Declared Prefixes (9):**
```
at_     Architectural Term
st_     Support Term
ua_     User Action
ops_    Sentry startup sequence
opbs_   Sessile Bottle startup
opbr_   Agile Bottle startup
rbb_    Base configuration
rbrn_   Nameplate configuration
cmk_    Console Makefile elements
```

**Additional undeclared prefixes found:** `mkr_`, `mkc_`, `scr_`, `cfg_` (ad-hoc additions)

**Comparison: RBS → RBAGS Evolution:**

| Dimension | RBS (Phase 1) | RBAGS (Phase 2) |
|-----------|---------------|-----------------|
| Prefix structure | Simple (`at_`, `st_`) | Three-part (`rbtr_`, `rbtgi_`) |
| Regime concept | Implicit (`rbb_`, `rbrn_`) | Systematic (RBRR, RBRP, RBRA, RBRO, RBRV, RBEV) |
| Anchor pattern | `term_*` indirection | Direct prefix match |
| Semantic metadata | None | Strachey brackets `⟦axl_voices⟧` |
| External domains | Minimal | Extensive (gcp_, gar_, gcs_, gcb_, giam_) |

**Key Anomaly in RBS:**
```
:rbtr_consumer:  <<term_consumer,Consumer Role>>  ✗
Expected:        <<term_rbtr_consumer,Consumer Role>>
```
The `rbtr_consumer` anchor omits the prefix, breaking pattern consistency.

**Evolution Insight:** RBAGS systematizes what RBS did ad-hoc:
- Undeclared `mkr_`, `scr_`, `cfg_` in RBS become managed regime layers in RBAGS
- Configuration hints (`rbb_`, `rbrn_`) evolve into six formal regime categories
- Simple operational categories become domain-stratified prefixes

**Notes:** RBS represents foundational vocabulary; RBAGS integrates it with Google Cloud primitives while introducing regime abstraction. This is healthy evolution — backward compatible while more systematic.

---

### Assessment 6: AXLA Lexicon — Canonical Ontology Pattern
- **Private**: `https://github.com/bhyslop/recipemuster.git` · `Tools/cmk/axl-AXLA-Lexicon.adoc`
- **Local**: `/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk/axl-AXLA-Lexicon.adoc`
- **Pattern expected**: Shared vocabulary hub
- **Conformance**: HIGH (exemplary cross-reference hub)

**Purpose:** AXLA is not a pure definitions file but a **canonical vocabulary reference** that other specs selectively import. It defines "motifs" (abstract patterns) that specs "voice" (concretely implement).

**15 Primary Prefix Families:**
```
axc_    Axial Control (orchestration)
axe_    Axial Environment (execution context)
axo_    Axial Operation (operation patterns)
axt_    Axial Type (type system)
axr_    Axial Record (structures)
axj_    Axial JSON (JSON-specific)
axa_    Axial Argument (command args)
axi_    Axial Interface
axk_    Axial Key Premise (constraints)
axl_    Axial Lexicon (motif vocabulary)
axd_    Axial Dimension (modifiers)
axf_    Axial Format (serialization)
axrg_   Axial Regime (config systems)
xref_   External spec references
axig_   Axial Infrastructure Google
```

**Extended Types:** `axtu_` (universal), `axtg_` (Google), `axtw_` (AWS reserved), `axta_` (Azure reserved)

**Key Pattern — Motif-Voicing:**
```
Specs annotate implementations:
// ⟦axl_voices axc_call axe_bash_interactive⟧
```
This declares that a term "voices" the `axc_call` motif in the `axe_bash_interactive` environment.

**Notes:** AXLA is the ontological foundation — it provides the vocabulary that RBAGS, RBS, and other specs import. The prefix design prioritizes "LLM interpretability over human visual parsing."

---

### Assessment 7: GAD Prefix Family — 4-Level Hierarchy
- **Private**: `https://github.com/bhyslop/recipemuster.git` · `Tools/gad/`
- **Local**: `/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/gad/`
- **Pattern expected**: `gad*` prefix family
- **Conformance**: HIGH (mature, component-oriented)

**GAD Prefix Hierarchy (4 levels):**
```
gad (non-terminal)
├─ gadf (TERMINAL) — Factory (Python backend)
├─ gadi (non-terminal) — Inspector
│  ├─ gadib (terminal) — Base infrastructure
│  ├─ gadic (terminal) — Cascade CSS
│  ├─ gadie (terminal) — Engine (diff computation)
│  ├─ gadiu (terminal) — User (UI handlers)
│  └─ gadiw (terminal) — Webpage (HTML container)
├─ gads (TERMINAL) — Specification
├─ gadp (TERMINAL) — Planner
└─ gadm (non-terminal) — Memos (18+ children)
   ├─ gadmcr, gadmdd, gadmdr... (4-char codes)
   └─ gadmrha5, gadmdug... (extensible suffixes)
```

**Key Difference from JJK:** GAD is 4-level (GADI has children), JJK is 3-level flat.

**Function Naming by Module:**
- `gadib_*()` in gadib_base.js (infrastructure)
- `gadie_*()` in gadie_engine.js (computation)
- `gadiu_*()` in gadiu_user.js (UI)
- `gadfl_*()` in gadf_factory.py (logging)

**CSS Class Pattern:** `gads-*` (links to spec document)

**Notes:** GAD shows component-layered architecture naming. The GADM memo space is extensible — codes can grow without breaking hierarchy (GADMCR → GADMRHA5).

---

### Assessment 8: BUK — Portable Kit with Z-Prefix
- **Private**: `https://github.com/bhyslop/recipemuster.git` · `Tools/buk/`
- **Local**: `/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/`
- **Pattern expected**: `bu*` prefix family, portable kit
- **Conformance**: HIGH (enterprise-grade bash infrastructure)

**BU Prefix Hierarchy:**
```
bu (non-terminal)
├─ buc (TERMINAL) — Command (output formatting)
├─ bud (TERMINAL) — Dispatch (environment setup)
├─ bug (TERMINAL) — Guide (user interaction)
├─ bur (non-terminal)
│  ├─ burc (terminal) — Config Regime (project-level)
│  └─ burs (terminal) — Config Regime (station-level)
├─ but (TERMINAL) — Test (testing framework)
├─ buu (non-terminal)
│  └─ buut (terminal) — TabTarget (launcher creation)
├─ buv (TERMINAL) — Validation (type system)
└─ buw (TERMINAL) — Workbench (BUK self-management)
```

**Z-Prefix Convention CONFIRMED:**
```bash
Public:   buc_log_args(), buc_die(), buc_step()
Private:  zbuc_color(), zbuc_make_tag(), zbuc_print()
```

**Inclusion Guard Pattern:**
```bash
test -z "${ZBUC_INCLUDED:-}" || return 0
ZBUC_INCLUDED=1
```

**Variable Naming:**
- Config: `BURC_*`, `BURS_*`, `BUD_*` (upper-case prefix)
- Private state: `ZBUC_*`, `ZBUD_*` (Z-prefix upper-case)

**Portable Kit Evidence:**
- No project-specific logic in `Tools/buk/*.sh`
- Configuration external via BURC/BURS regimes
- Copy-paste installation documented in README

**Notes:** BUK confirms Pattern D (z-prefix) and Pattern E (portable kit isolation). The 3-letter/4-letter split (buc vs burc) mirrors JJK's pattern.

---

### Assessment 9: WRC — Ward Realm Concepts
- **Private**: `https://github.com/bhyslop/cnmp_CellNodeMessagePrototype.git` · `lenses/wrs-WRC-WardRealmConcepts.adoc`
- **Local**: `/Users/bhyslop/projects/cnmp_CellNodeMessagePrototype/lenses/wrs-WRC-WardRealmConcepts.adoc`
- **Pattern expected**: `wrs*` prefix family, MCM patterns
- **Conformance**: HIGH (193+ defined terms, perfect MCM alignment)

**Domain:** Distributed state machine substrate using feudal/organizational metaphors (Ward, Fief, Steward, Vassal, Gerent, Hearth).

**Six Primary Category Prefixes:**
```
ftc_    Form Type Concept (data structures)
smc_    Swarm Model Concept (runtime/organization)
cmb_    Concept Model Basics (builder classes)
pmp_    Protocol Machinery Patterns (binary protocol)
hdm_    Handwritten Methods (directly coded)
mcd_    Messaging Communication Definition
```

**Additional Prefixes:** `cbi_` (API verbs), `gcm_` (generated code), `toc_` (testing ops), `noc_` (notices), `csc_` (documentation refs)

**Key Stats:**
- 169 attribute references in mapping section
- Perfect anchor↔attribute alignment
- MCM-compliant: Strachey brackets, linked terms, proper tag delimiters

**Cross-References:** Links to MCM, Task Lens Guide, Model-to-Claudex

**Notes:** WRC demonstrates domain-specific vocabulary with cognitive scaffolding (feudal metaphors for distributed systems). High term density (~193 terms over 3000+ lines).

---

### Assessment 10: PB Rust Modules — 5-Level Hierarchy
- **Private**: `https://github.com/bhyslop/pb_paneboard02.git` · `poc/src/`
- **Public**: `https://github.com/scaleinv/paneboard.git` · `poc/src/`
- **Local**: `/Users/bhyslop/projects/pb_paneboard02/poc/src/`
- **Pattern expected**: `pb*` prefix family for Rust modules
- **Conformance**: HIGH (well-designed platform-aware architecture)

**PB Prefix Hierarchy (4-5 levels):**
```
pb (non-terminal: Paneboard)
├─ pbg (non-terminal: General/Global)
│  ├─ pbgc (TERMINAL) — Core
│  ├─ pbgf (non-terminal: Form)
│  │  ├─ pbgfc (terminal) — Config
│  │  ├─ pbgfp (terminal) — Parse
│  │  ├─ pbgfr (terminal) — Resolve
│  │  └─ pbgft (terminal) — Types
│  ├─ pbgk (TERMINAL) — Keylog
│  └─ pbgr (TERMINAL) — Retry
└─ pbm (non-terminal: Mac-specific)
   ├─ pbmb (non-terminal: Mac bindings)
   │  ├─ pbmba (terminal) — AX (Accessibility)
   │  ├─ pbmbd (terminal) — Display
   │  ├─ pbmbe (terminal) — EventTap
   │  ├─ pbmbk (terminal) — Keymap
   │  ├─ pbmbo (terminal) — Observer/Overlay
   │  └─ pbmbs (terminal) — Sandbox
   ├─ pbmcl (TERMINAL) — Clipboard
   ├─ pbmp (TERMINAL) — Pane
   └─ pbms (non-terminal: Mac services)
      ├─ pbmsa (terminal) — AltTab
      ├─ pbmsb (terminal) — Browser
      └─ pbmsm (terminal) — MRU
```

**Level Semantics:**
| Level | Meaning | Examples |
|-------|---------|----------|
| L1 `pb` | Project | All modules |
| L2 `g`/`m` | Tier (General vs Mac) | `pbg*` vs `pbm*` |
| L3 | Domain/Feature | `pbgf` (Form), `pbmb` (Bindings) |
| L4 | Specific module | `pbgfc`, `pbmba` |
| L5 | Semantic name | `_config`, `_eventtap` |

**Key Design Feature:** L2 fork cleanly separates cross-platform (`g`) from platform-specific (`m`) code.

**Notes:** PB shows deepest hierarchy (5 levels) but only 2 real fork points. Strong semantic coupling — all `pbmb*` are C FFI wrappers. Platform-aware architecture pattern.

---

## Candidates for Future Assessment

### All Priority Candidates COMPLETE
- ~~`Tools/jjk/`~~ — Assessment 4
- ~~`lenses/rbw-RBS-Specification.adoc`~~ — Assessment 5
- ~~`lenses/axl-AXLA-Lexicon.adoc`~~ — Assessment 6
- ~~`Tools/gad/` file tree~~ — Assessment 7
- ~~`Tools/buk/` bash utilities~~ — Assessment 8
- ~~`wrs-WRC-WardRealmConcepts.adoc`~~ — Assessment 9
- ~~`pb_paneboard02/poc/src/`~~ — Assessment 10

### Lower Priority (older patterns, not assessed)
- `lens-*` files in cnmp — pre-date strict convention?
- `../recipebottle-admin/` — mixed patterns observed

---

## Open Questions

1. **2-char vs 3-char project prefix?** — Appears to be based on collision avoidance and distinctiveness. `rb` works when children disambiguate; `gad` needed 3 chars to be memorable.

2. **Cross-repo references?** — RBAGS uses `at_` prefix for legacy cross-references. May need formal cross-reference category convention.

3. **Acronym embedding rule?** — Observed: acronyms in `rb*` family embed `RB`; acronyms in `gad*` embed `GAD`. Exception: `BCG` under `bpu-` (Bash Programming Utility) doesn't embed `BPU`. Needs clarification.

4. **Version suffixes?** — `bpu-PCG-ProcedureCurationGuide-005.md` suggests pattern `prefix-ACRONYM-Words-NNN.ext` for versioned docs.

5. **Deferred/experimental items?** — `wrs-xDeferred*` uses `x` prefix within the name portion (not the category). Suggests `x` = experimental/deferred.

6. **Operation category collision** — `rbtgo_` vs `rbtoe_` emerged through evolution without collision detection. This memo should help prevent similar issues. Example of why codification matters.

Note: Variant suffixes (`_s`, `_ies`, `_es`) are AsciiDoc conventions, out of scope for prefix study.

## Discovered Patterns (from assessments)

### Pattern A: Category Nesting Within Project
RBAGS shows nested categories within `rb`:
```
rb
├─ rbtr_   (term + role)
├─ rbtgi_  (term + google + instance)
├─ rbtgo_  (term + google + operation)
├─ rbtoe_  (term + orchestration + embodiment)
├─ rbrr_   (regime + repo)
├─ rbra_   (regime + account)
├─ rbrp_   (regime + payor)
├─ rbro_   (regime + oauth)
├─ rbrv_   (regime + vessel)
├─ rbev_   (environment + variables)
├─ rbbc_   (bash + console)
└─ rbhg_   (human + guide)
```

### Pattern B: External Domain Prefixes
RBAGS imports external domain prefixes that don't start with `rb`:
- `gcp_`, `gar_`, `gcs_`, `gcb_`, `giam_` — Google services
- `oauth_` — OAuth concepts
- `at_` — legacy architectural terms

These are declared in the mapping section header but represent external concepts.

### Pattern C: File vs Attribute Prefix Relationship
- File prefix: `rbw-` (workbench layer)
- Attribute prefixes: `rbtr_`, `rbtgi_`, etc.
- The file prefix (`rbw`) differs from attribute category prefixes (`rbtr_`)
- File prefix indicates which module/workbench owns the document
- Attribute prefix indicates semantic category of the term

### Pattern D: Z-Prefix for Private Functions (from JJK, BUK)
Internal/private functions use `z` prefix before the module prefix:
```
Public:   jjw_install()           buc_doc_exists()
Private:  zjjw_compute_source_hash()   zbuc_internal_helper()
```
- `z` = "infrastructure" / "internal"
- Consistent across JJK and BUK kits
- Enables grep filtering: `^jjw_` finds public API, `^zjjw_` finds internals

### Pattern E: Portable Kit Prefixes (from JJK, BUK, CMK)
Kits designed for installation across multiple repos use isolated prefix families:
- **JJK**: `jj*` — no overlap with host repo prefixes
- **BUK**: `bu*` — no overlap with RB, GAD, etc.
- **CMK**: `cm*`, `mcm*`, `axl*` — concept model vocabulary

This enables a single repo to host multiple kits without collision.

### Pattern F: Evolution Phases (from RBS→RBAGS comparison)
Naming systems mature through observable phases:
1. **Phase 1**: Simple category prefixes (`at_`, `st_`, `ops_`)
2. **Phase 2**: Domain-stratified prefixes (`rbtr_`, `rbtgi_`, `rbtgo_`)
3. **Phase 3**: Semantic annotation layer (`// ⟦axl_voices axo_identity⟧`)

Backward compatibility maintained — Phase 2 absorbs Phase 1 vocabulary.

### Pattern G: Motif-Voicing Ontology (from AXLA)
AXLA defines abstract "motifs" that concrete specs "voice":
```
Motif:   axc_call (abstract: synchronous invocation)
Voicing: rbbc_call (concrete: curl REST call in bash)
         Annotated: // ⟦axl_voices axc_call axe_bash_interactive⟧
```
- Motifs live in AXLA with `ax*_` prefixes
- Voicings live in domain specs with domain prefixes (`rbbc_`, `rbhg_`)
- Annotations link voicing back to motif + execution environment

### Pattern H: Hierarchy Depth Varies by Domain
| System | Levels | Structure | Notes |
|--------|--------|-----------|-------|
| JJK | 3 | jj → jja/jjw → terminal | Flat at L2 |
| BUK | 3-4 | bu → buc/bur → burc/burs | 3-letter core, 4-letter specialized |
| GAD | 4 | gad → gadi → gadib/gadie/gadiu | Component layers |
| RB | 4+ | rb → rbg → rbga/rbgb/rbgc | Domain-stratified |
| WRC | 3 | wrs → ftc/smc/cmb/pmp | Category-per-domain |
| PB | 5 | pb → g/m → f/b → c/p/r/t | Platform-aware fork |

Depth is determined by domain complexity, not a fixed rule. PB shows deepest (5) but only 2 real fork points.

## Future Work

- [x] Complete 2-3 deep assessments (actually completed 10)
- [x] Document Pattern C formally (file prefix vs attribute prefix)
- [x] Document Pattern D (z-prefix for private functions)
- [x] Document Pattern E (portable kit prefixes)
- [x] Document Pattern F (evolution phases)
- [x] Document Pattern G (motif-voicing ontology)
- [x] Document Pattern H (hierarchy depth varies)
- [x] Assess all priority candidates (10/10 complete)
- [ ] Refine rules based on exceptions found
- [ ] Create validation tooling (lint script?)
- [ ] Document migration path for non-conforming files
- [ ] Add to CLAUDE.md as enforceable convention
