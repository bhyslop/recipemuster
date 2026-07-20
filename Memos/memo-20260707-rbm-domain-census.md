# The RB home-repo domain census (tackle's worked example)

Design-session record, 2026-07-07 (Fable session, operator Brad); refreshed 2026-07-20.
Provenance, never authority.

This memo is the *living* census: the concrete inventory of this repo's domains and the
relations between them, kept separate from the tackle concept itself (the aspirant sheaf,
`Tools/jjk/vov_veiled/JJSAT-tackle.adoc`) so the concept stays general while the
example stays real.
Operator framing: the two "fly in formation" throughout tackle development — the sheaf
carries a small frozen illustration; this memo is where the census grows.

## The boxes (domains)

Rows the operator named in session, plus rows found by reading.
Paths are best-current-knowledge, not verified claims; this is a census in progress.

| Domain | Files (approximate scope) | Governed by |
|---|---|---|
| JJ specs | `Tools/jjk/vov_veiled/**/*.adoc` | MCM/AXLA authoring + JJS0 doctrine |
| RB specs | `Tools/rbk/vov_veiled/**/*.adoc` (incl. the hierophant sheaves RBSHC/RBSHE/RBSHH) | MCM/AXLA authoring + RBS0 |
| CMK specs | `Tools/cmk/vov_veiled/**/*.adoc` (MCM, AXLA themselves) | MCM's own rules, self-hosting |
| VOK specs | `Tools/vok/vov_veiled/**/*.adoc` | MCM/AXLA + VOS0 |
| Guides | the guide family (BCG, RCG, WSG, CBG, JDG, PCG, ACG, HCG, GMG) | GMG (the guide for guides) |
| JJ rust | the VOW-pipeline crates: `jjk` (`Tools/jjk/vov_veiled/`), `vvr` (`Tools/vok/`), `vvc`, `vof` | RCG |
| JJ tests | JJ crate test modules | RCG + test conduct |
| Matricula | `Tools/vok/vom/**` — operator-only, never ships (VOr_q4f) | RCG + VOSMM |
| Theurge | `Tools/rbk/rbtd/**/*.rs` | RCG + RBSTC + theurge-ifrit context |
| Hierophant | `Tools/rbk/vov_veiled/rbthd/**` — veiled, withheld from delivery; zero third-party deps by construction | RCG + RBSHC (normative) |
| Ifrit | `rbmm_moorings/rbmv_vessels/common-ifrit-context/` + `common-ifrit-forge-context/` (one crate `rbid`, two vessel build contexts) | RCG + RBSIP posture |
| BUK bash | `Tools/buk/**/*.sh` | BCG |
| RB bash | `Tools/rbk/*.sh` (flat — the decomposed modules) | BCG + RBK conduct |
| Cloud steps | `Tools/rbk/rbgj*/` step scripts | CBG |
| Jailer sh | sentry/pentacle in-vessel scripts (`rbjs_sentry.sh` kin) | JDG |
| Tabtargets | `tt/*.sh` | BUS0 dispatch discipline (no business logic) |
| Launchers | `rbmm_moorings/rbml_launchers/*.sh` | BUK launcher discipline |
| Regimes | the `*.env` regime carriers under moorings (incl. `rbmf_foedera/`) | the RBSR* regime spec family |
| Diagrams | `diagrams/*.puml` | PCG |
| Rendered SVGs | `diagrams/*.svg` (+ dark siblings) | generated — regenerate via pluml case, never hand-edit |
| Memos | `Memos/**/*.md` | provenance-never-authority |
| Retired memos | `Memos/retired/**` | historical record, never resurrect without direction |
| Claude context | `CLAUDE.md`, `claude-*.md` | index-of-record; some generated (do-not-hand-edit) |
| Generated consts | `rbtdgc_consts.rs`, `claude-rbk-tabtarget-context.md` | regenerate via build, never hand-edit |
| Public face | `README.md` | MCM revetment law (ashlar-or-prose only) |
| APCK | `Tools/apck/**` | RCG + APCS0/APCPS, no-JavaScript rule |
| Studies | `Study/**` (e.g. `study-model-prompt-tuning/`, crate `smpt`) | none declared — a live specimen of the unclaimed-in-bounds verdict (or of a bounds decision, if Studies sit outside bounds) |
| Aspirant sheaves | `Tools/jjk/vov_veiled/JJSA*.adoc` + aspirant-lintel members elsewhere (VOSMM, RBSCV, RBSPC…) | MCM/AXLA + AWG (Aspirant Writing Guide) |

Notes on fuzziness (deliberate, recorded):

- Domains are fluid during development (operator, 260707).
  They generally follow directory-tree lines, but that is a tendency, not a law.
- A file may legitimately sit in two domains at once (operator, 260707):
  the standing example is workstation bash that, with slight modification, also runs in the
  cloud environment — BCG and CBG both apply.
  Tackle stays simple rather than over-fitted: overlap means *both apply* (union of
  bundles), the audit report makes every overlap visible, and a genuine contradiction
  between two bundles is surfaced to the operator, never auto-resolved.

## The builds (independent compilation units and their verbs)

The evidence base for tackle's check-member design: what "build/test/etc." concretely
means per domain today.
Nine independent Rust builds (eleven `Cargo.toml`s; the two ifrit contexts share one
crate, and Studies sit outside the tabtarget system).

| Build | Source | Build | Test | Other verbs |
|---|---|---|---|---|
| VOW pipeline (`vvr` + `jjk` + `vvc` + `vof`) | `Tools/vok/`, `Tools/jjk/vov_veiled/`, `Tools/vvc/` | `tt/vow-b` | `tt/vow-t` | run `vow-r`, clean `vow-c`, freshen `vow-F`, parcel-release `vow-R.*` |
| Matricula (`vom`) | `Tools/vok/vom/` | `tt/vow-mb` | `tt/vow-mt` | run `vow-mr` (operator-only) |
| Theurge (`rbtd`) | `Tools/rbk/rbtd/` | `tt/rbw-tb` | `tt/rbw-tt` | suites `rbw-ts.*`, fixture `rbw-tf`, case `rbw-tc`, qualify `rbw-tq`/`rbw-tr` |
| Hierophant (`rbthd`) | `Tools/rbk/vov_veiled/rbthd/` | `tt/rbthw-b` | `tt/rbthw-t` | ceremony acts: docimasy `rbthw-d`, essai `rbthw-e`, ostend `rbthw-o`, harbinger `rbthw-h` |
| APCK (`apcd`) | `Tools/apck/apcd/` | `tt/apcw-b` | `tt/apcw-t` | run `apcw-r`, deploy `apcw-D`, dictionary-refresh `apcw-dr`, batch-assay `apcw-ba`, container lifecycle `apcw-c*` |
| Ifrit (`rbid`, two contexts) | `rbmm_moorings/rbmv_vessels/common-ifrit-*-context/` | inside vessel image builds (kludge `rbw-cK*`, ordain `rbw-fO`) | exercised, not unit-tested: sortie `rbw-Is`, siege suite | — |
| Study (`smpt`) | `Study/study-model-prompt-tuning/` | raw cargo, no tabtarget | — | exploratory |

Cross-domain scenario harnesses (rust-driven, drive major scenarios and evaluate):
the theurge suite ladder — dependency strata reveille/picket/bivouac/echelon, release/probe
ladders gauntlet/skirmish/dogfight/siege/blockade/parley — plus BUK self-test `buw-st`.
These belong to no single fileset domain; they are consumers of many.

Verb-census observation (feeds the fixed-members vs. open-actions fork): *build*, *test*,
and *lint* recur across every domain; beyond them the tail is idiosyncratic — run, deploy,
refresh, assay, render, charge/quench, clean, freshen, parcel-release, and hierophant's
ceremony acts.
The recurring three fit fixed typed members; the tail does not fit any small closed enum.

## The arrows (relations between domains)

The second half of the tackle charter.
Each is declared, never inferred; some carry their own governing discipline:

| From | To | Relation | Governing rule |
|---|---|---|---|
| RB bash | RB specs | cites its spec home | ACG (reference-the-home, rivet residue) |
| JJ rust | JJ specs | cites its spec home | ACG + rivet discipline (`JJr_` tails) |
| Theurge | RB bash + crucible | tests | RBSTC cosmology, notch-before-test |
| Ifrit | crucible containment | attacks (adversarial test) | RBSIP |
| JJ tests | JJ rust | tests | RCG test conduct |
| BUTT | BUK bash | tests | BUK self-test frame |
| Hierophant | RB bash (the `rblm_` marshal workers) | sequences as subprocesses | RBSHC (worker-never-authority; structural incapacity) |
| Hierophant | release tree state | proves + shows, keyed by tree hash (the cachet) | RBSHC reversible/irreversible seam |
| JJS0/RBS* specs | their implementations | specifies | MCM/AXLA (executory→normative ladder) |
| Zipper registry | generated consts + tabtarget context | generates | build-freshness gate (`rbq_qualify`) |
| Diagrams (.puml) | rendered SVGs | generates | pluml crucible case |
| Tackle table (future) | claude context files | projects | the tackle sheaf's consumer story |

## Why this memo exists

The tackle sheaf's census table is a frozen illustration and deliberately does not grow.
This memo is where new domains and new arrows get recorded as they are noticed, so the
concept document never silently drifts into being an RB inventory.
The builds table above exists for the same reason at command grain: it is the evidence the
check-member design decision (fixed members vs. open actions) reads.
When tackle's real table exists (pedigree-side), this memo retires into provenance — the
table becomes the census and the matricula's audit becomes the freshness check.
