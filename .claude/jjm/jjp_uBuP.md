## Shape

Grab-bag cleanup heat. Loose theme: consolidate to the single rightful owner — remove capability and constants that are redundantly held in more than one place. Two strands so far, open to other post-cutover cleanups:

- **ifrit / ccyolo separation** — ifrit becomes purely scripted-adversary (rbid binary + sortie adjutant); ccyolo owns the live-agent role via its SSH model. Retire the redundant `rbw-Ic` IfritClient (dead — execs `claude` with no `claude` installed; redundant with `rbw-cS` SshTo, the more honest transport). Tier-2 spec reframe + code/tabtarget/zipper removal + generated-doc regen. (Authored in a separate session — slated there.)
- **cross-language constant projection** — replace hand-maintained bash↔Rust constant mirrors with build-time generation from the canonical bash source.

## Locked decisions

- **Gated on ₣BK (moorings cutover) closing.** The projection strand reads RBCC and the moorings layout; both are reshaped throughout ₣BK. No projection work starts until ₣BK retires — stable RBCC, settled layout, RBBC aliases gone. The ifrit strand is not gated but rides here by choice: one grab-bag over two heats.
- **Ordering: ifrit before codegen.** The ifrit strand removes `rbw-Ic` from the rbz registry; the codegen projects rbz. Clean the registry before generating its mirror.
- **Projection scope rule: single canonical bash author, or out.** Colophons (rbz) and RBCC flat tinder qualify. Distributed-identity classes (fixtures, operations, container roles — held across bash arrays, disk imprints, and Rust) are excluded unless a separate canonical-author decision is taken.

## What done looks like

- ifrit is scripted-adversary only; ccyolo owns the live-agent role; `rbw-Ic` and its enrollment/tabtargets are gone.
- The rbtd colophon mirror and RBCC-derived Rust constants are generated, checked in, regenerated before each Rust build, and gated by a diff check; the hand-maintained mirror and its runtime verify are retired.

## Out of scope

- Unifying fixture/operation/container-role identity (separate canonical-author decision; not pulled in unless deliberately scoped).
- Any work that depends on RBCC or the moorings layout before ₣BK closes.