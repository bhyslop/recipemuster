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
| `rbfls_seise.sh` (substrate-reliquary seise) | RBSDE |
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
