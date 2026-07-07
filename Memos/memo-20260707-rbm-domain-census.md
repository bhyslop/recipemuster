# The RB home-repo domain census (tackle's worked example)

Design-session record, 2026-07-07 (Fable session, operator Brad).
Provenance, never authority.

This memo is the *living* census: the concrete inventory of this repo's domains and the
relations between them, kept separate from the tackle concept itself (the aspirant sheaf,
`Tools/jjk/vov_veiled/JJS-aspirant-tackle.adoc`) so the concept stays general while the
example stays real.
Operator framing: the two "fly in formation" throughout tackle development — the sheaf
carries a small frozen illustration; this memo is where the census grows.

## The boxes (domains)

Rows the operator named in session, plus rows found by reading.
Paths are best-current-knowledge, not verified claims; this is a census in progress.

| Domain | Files (approximate scope) | Governed by |
|---|---|---|
| JJ specs | `Tools/jjk/vov_veiled/**/*.adoc` | MCM/AXLA authoring + JJS0 doctrine |
| RB specs | `Tools/rbk/vov_veiled/**/*.adoc` | MCM/AXLA authoring + RBS0 |
| CMK specs | `Tools/cmk/vov_veiled/**/*.adoc` (MCM, AXLA themselves) | MCM's own rules, self-hosting |
| VOK specs | `Tools/vok/vov_veiled/**/*.adoc` | MCM/AXLA + VOS0 |
| Guides | the guide family (BCG, RCG, WSG, CBG, JDG, PCG, ACG, HCG, GMG) | GMG (the guide for guides) |
| JJ rust | the JJ/vvx crates (VOW pipeline kits) | RCG |
| JJ tests | JJ crate test modules | RCG + test conduct |
| Theurge | `Tools/rbk/rbtd/**/*.rs` | RCG + RBSTC + theurge-ifrit context |
| Ifrit | `rbev-vessels/common-ifrit-context/` | RCG + RBSIP posture |
| BUK bash | `Tools/buk/**/*.sh` | BCG |
| RB bash | `Tools/rbk/*.sh` (flat — the decomposed modules) | BCG + RBK conduct |
| Cloud steps | `Tools/rbk/rbgj*/` step scripts | CBG |
| Jailer sh | sentry/pentacle in-vessel scripts (`rbjs_sentry.sh` kin) | JDG |
| Tabtargets | `tt/*.sh` | BUS0 dispatch discipline (no business logic) |
| Launchers | `rbmm_moorings/rbml_launchers/*.sh` | BUK launcher discipline |
| Regimes | the `*.env` regime carriers under moorings | the RBSR* regime spec family |
| Diagrams | `diagrams/*.puml` | PCG |
| Rendered SVGs | `diagrams/*.svg` (+ dark siblings) | generated — regenerate via pluml case, never hand-edit |
| Memos | `Memos/**/*.md` | provenance-never-authority |
| Retired memos | `Memos/retired/**` | historical record, never resurrect without direction |
| Claude context | `CLAUDE.md`, `claude-*.md` | index-of-record; some generated (do-not-hand-edit) |
| Generated consts | `rbtdgc_consts.rs`, `claude-rbk-tabtarget-context.md` | regenerate via build, never hand-edit |
| Public face | `README.md` | MCM revetment law (ashlar-or-prose only) |
| APCK | `Tools/apck/**` | RCG + APCS0/APCPS, no-JavaScript rule |
| Aspirant sheaves | `JJS-aspirant-*.adoc` + aspirant-lintel members elsewhere (VOSMM, RBSCV, RBSPC…) | MCM/AXLA + a future Aspirant Refinement Guide (unwritten; this session is its type specimen) |

Notes on fuzziness (deliberate, recorded):

- Domains are fluid during development (operator, 260707).
  They generally follow directory-tree lines, but that is a tendency, not a law.
- A file may legitimately sit in two domains at once (operator, 260707):
  the standing example is workstation bash that, with slight modification, also runs in the
  cloud environment — BCG and CBG both apply.
  Tackle stays simple rather than over-fitted: overlap means *both apply* (union of
  bundles), the audit report makes every overlap visible, and a genuine contradiction
  between two bundles is surfaced to the operator, never auto-resolved.

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
| JJS0/RBS* specs | their implementations | specifies | MCM/AXLA (executory→normative ladder) |
| Zipper registry | generated consts + tabtarget context | generates | build-freshness gate (`rbq_qualify`) |
| Diagrams (.puml) | rendered SVGs | generates | pluml crucible case |
| Tackle table (future) | claude context files | projects | the tackle sheaf's consumer story |

## Why this memo exists

The tackle sheaf's census table is a frozen illustration and deliberately does not grow.
This memo is where new domains and new arrows get recorded as they are noticed, so the
concept document never silently drifts into being an RB inventory.
When tackle's real table exists (pedigree-side), this memo retires into provenance — the
table becomes the census and the matricula's audit becomes the freshness check.
