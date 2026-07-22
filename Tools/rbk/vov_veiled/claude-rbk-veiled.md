## File Acronym Mappings — RBK Veiled (`Tools/rbk/vov_veiled/`)

The veiled half of the RBK agent context. Its delivered siblings map the shipped
surface and travel with the distribution:

- `Tools/rbk/claude-rbk-acronyms.md` — the prefix map for shipped code
- `Tools/rbk/claude-rbk-conduct.md` — the behavioral rules for shipped code
- `Tools/rbk/claude-rbk-theurge-ifrit-context.md` — the crucible-testing context

This file maps what stays behind: the guides, the specification, the shelved
handbook, the derived diagram, and the contract pointers stripped from the
delivered rows. **Nothing named here — by path, by basename, or by acronym — may
appear in a delivered file.** The delivered half must not even acknowledge that
this material exists: a consumer agent is told nothing about an upstream record it
cannot read. The pyx release-hygiene fixture holds the path-and-basename half of
that line mechanically; the acronym half is authorial for now.

### Guides

- **CBG**  → `rbk/vov_veiled/CBG-CloudBuildGuide.md` (Cloud Build Guide — step-body discipline for Google Cloud Build steps; polyglot bash/sh + python; foreign-environment sibling to BCG/RCG/WSG. Rule families CBi_/CBb_/CBp_/CBh_.)
- **JDG**  → `rbk/vov_veiled/JDG-JailerDialectGuide.md` (Jailer Dialect Guide — the zero-dependency in-vessel POSIX `sh` baked into vessel images and run inside the security envelope (`rbjs_sentry.sh` type specimen, `rbjp_pentacle.sh` sibling); foreign-environment sibling to BCG/CBG/WSG. v1 bless-and-name. Rule families JDo_ (observability contract: phase anchors, exit-code families, `|| exit N`) and JDp_ (parameter-transport Palisade). The type specimen's phase-label and FATAL-shape deviations are citable against JDo_101/JDo_103.)
- **PUCG**  → `rbk/vov_veiled/PUCG_PlantUmlCodingGuide.md` (PlantUml Coding Guide — diagram source + render discipline for the committed `.puml`/`.svg` pairs; foreign-environment sibling to CBG/WSG. Palisade conduct at the PlantUML render membrane; cited-rule family PUCr_. Founded on PUCr_101 (charset transport).)

### Specification

- **RBS0** → `rbk/vov_veiled/RBS0-SpecTop.adoc`

> **RBS\* sheaf entries are intentionally not listed here.** The Recipe Bottle spec sheaves (RBSAA…RBSYC — every operation and concept subdoc) load on demand, not always. Discipline: to reach any sheaf, read its SpecTop **RBS0** (`rbk/vov_veiled/RBS0-SpecTop.adoc`) FIRST — it is the required entry point and indexes them; the sheaves live beside it as `rbk/vov_veiled/RBS*-*.adoc`.

- **RBRN**  → `rbk/vov_veiled/RBRN-RegimeNameplate.adoc`

### Shelved code

- **RBHW0** → `rbk/vov_veiled/rbhw0_*.sh` (Windows 0-prefix — VEILED: the windows handbook (`rbhw0_cli.sh`, `rbhw0_top.sh`, `rbhw0_windows.sh`, `rbhwht_handbook_top.sh`, `rbhwcd_docker_context_discipline.sh`, `rbhwdd_docker_desktop.sh`) was shelved off the surface into `rbk/vov_veiled/`. Inert; enrollments + tabtargets removed, not rewired for local execution. It is the `RBHW` **windows** group — the third `rbh*` group, alongside the delivered `RBHO` onboarding and `RBHP` payor.)

### Veiled diagram

- **RBDGP** → `rbk/vov_veiled/diagrams/rbdgp_provenance-tale.{puml,svg}` (the provenance tale — a derived presentation of RBSYP's marker skeleton, veiled 2026-07-13 because its rendered title named its closed source in a shipped image, and because no README `<picture>` block ever consumed it. Frozen: it sits outside the `diagrams/*.puml` glob the pluml crucible case renders, so it is not re-rendered. Its intended re-gestation is chapbook-driven puml authoring; the delivered `rbdg*` set is the three federation diagrams.)

### Rivet doctrine (the veiled half of the `RBr_` conduct rule)

The delivered `claude-rbk-conduct.md` carries the *behavior* — never restate a
marked decision, never simplify the code it guards — and names no home for the
reasoning, because the delivered face must not disclose that a closed record
exists. Here is what it withholds:

An `RBr_<tail>` token is a **rivet**: a cited constraint, defined and explained
once in an `.adoc` spec. The ID is opaque on purpose — it carries no meaning, so
it leaks none into released code. To understand one, `grep RBr_<tail>` and read
the spec. The code carries the pointer; the spec carries the prose; one home,
always. Doctrine: ACG "residue" (the rule), MCM `mcm_rivet` (the concept). In a
jailer script (no comments by dialect) the citation rides the execution-time
announcement — see JDG.

### Contract map (stripped from the delivered rows)

Which spec sheaf contracts each delivered module. Kept here so the delivered
prefix map can name the module without naming its contract.

| Module | Contract |
|--------|----------|
| `rba_auth.sh` (sitting lifecycle: novate, espy) | RBS0 `rbtf_novate`, `rbtf_espy` |
| `rbfb_beckon.sh` (per-fact signpost) | RBS0 `rbch_beckon`, Chaining-Fact Roles |
| `rbflf_feoff.sh` (vessel feoff) | RBSDF |
| `rbgft_terrier.sh` (muniment access) | RBSTR |
| `rbgp_payor.sh` (polity admission) | RBSPB / RBSPU / RBSPA / RBSPO; gird → RBSPG |
| `rbgjs/` (composed-snippet library) | RBSCJ "Composed-snippet library (rbgjs)" |
| `rblds_spine.sh` (capture-assembly spine) | RBSCJ "Capture Composition Contract" |
| `rbld*` (Lode capture family) | spec family `RBSL*` |
| `rbnnh_*` (per-nameplate charge hooks) | RBSCH |
| `rbof_foedus.sh` (foedus descry, instate) | RBSFD, RBSFI |
| `rbmf_foedera/` (federation regime storage) | RBSRF |
| tabtarget path indirection (`BURD_LAUNCHER`) | BCG "Tabtarget Path Indirection" |
| shellcheck suppressions / inline directives | BCG § Shellcheck Integration |
| BURE tweak slot-reservation (reveille guard) | BUS0 "Tweak Mechanism" |
