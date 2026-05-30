# Heat Trophy: rbk-13-mvp-redundancy-cleanup

**Firemark:** ₣BP
**Created:** 260520
**Retired:** 260529
**Status:** retired

## Paddock

## Shape

Grab-bag cleanup heat. Theme: consolidate to the single rightful owner — remove capability and constants that are redundantly held in more than one place. Collects post-cutover RBK cleanups of that nature.

The constant-projection thread is the bulk of the heat. A survey found the co-maintained set is broader than colophons alone — colophons, credential roles, .env filenames, operation verbs, and container roles all live in bash and are hand-copied into the rbtd Rust manifest or lib.rs, kept in sync only by a runtime drift check. The work points every co-maintained group through bash→Rust generation, retiring the runtime drift check in favor of a build-time diff gate.

## Locked decisions

- **Work that reads RBCC or the moorings layout is gated on ₣BK (moorings cutover) closing** — stable RBCC, settled layout, RBBC aliases retired. Cleanups touching neither are free to run.
- **Constant-projection scope: single canonical bash author, or out.** A constant is projectable to another language only if exactly one bash module canonically owns it. Constants whose identity is distributed across bash, disk, and Rust are excluded unless a separate canonical-author decision is taken first.
- **One generated landing file, one minted prefix `RBTDGC_`, fed per-pair.** All four co-maintained groups (colophons, RBCC set, operation verbs, container roles) project into a single generated Rust file in the rbtd crate — new file, not new crate — under the `RBTDGC_` prefix. The consts never carry a bash module prefix (`RBZ_`, `RBCC_`, … are bash-only namespaces; stamping them on a Rust const is a false ownership claim). The cross-language correspondence is mechanical and fully generated: Rust const = `RBTDGC_` + the bash const's stem (its bash-module prefix stripped, stem carried verbatim) — one uniform rule, no per-entry mapping, hence no drift. A shared RBK-ignorant BUK primitive formats one `pub const` line; each group's canonical bash home loops its own data through it (bash 3.2 has no clean way to pass a pair-list to a function, but per-pair scalar calls are trivial). The legacy curated names (`RBTDRM_COLOPHON_*`, the lib.rs path macros, etc.) carry no preservation obligation — each is retired as its group's generated consts land.
- **Co-maintained constant set in scope** — not just colophons. Groups with a single bash home are projected directly: colophons (rbz), the RBCC tinder set incl. credential roles and .env filenames (rbcc). Groups scattered across bash today — operation verbs and container roles — earn a canonical-author tidy first (one bash home in rbcc), then project. This honors the single-author rule rather than weakening it.
- **Fixture names are Rust-authored — out of the projection.** On inspection, fixtures are defined in Rust (the `RBTDRC_FIXTURES` registry; fixture structs name themselves), and bash holds only redundant copies in its `ZRBTE_SUITE_*` suite arrays — there is no canonical bash author, so fixtures fail the single-author rule. The redundancy is removed in the other direction: suite definitions relocate into theurge (Rust) and bash becomes a pure forwarder. This is an ownership relocation, not a bash→Rust constant projection — a future pace must not re-attempt projecting fixture names.

## What done looks like

The redundancies in scope are gone: no capability that has a better-owned home elsewhere survives in two places, and no co-maintained constant group is hand-maintained on both sides of the bash↔Rust boundary. The `RBTDGC_` generated file feeds the rbtd manifest's former hand-mirror groups and the lib.rs path macros; the runtime drift check is retired in favor of a build-time diff gate (the same fresh-generate-and-diff staleness check the qualifier already applies to the generated markdown context).

## Paces

### scattered-const-bash-home (₢BPAAI) [complete]

**[260527-0946] complete**

## Character

Pure bash refactor — depot-free, emitter-independent, do-now-able. Judgment lives in choosing the single home; no behavior change.

## Goal

Give operation verbs and container roles each a single canonical bash home, so they become projectable under the heat's single-author rule. No Rust touched here.

## Shape

- Today these are implicit/scattered: operation verbs across tabtarget/zipper definitions (rbz), container roles across `rbob_*.sh`. Neither has one owner.
- Settle a single bash module that canonically declares each group as a named list — the home, not new behavior. Existing call sites reference the new home.
- Terms to home (verify against live files first): operation verbs — inscribe, yoke, ordain, enshrine, kludge; container roles — sentry, bottle.
- Prerequisite for the projection pace; touches no Rust and no manifest.

## Sources

- `Tools/rbk/rbz_zipper.sh` — operation verbs (implicit in tabtarget defs)
- `Tools/rbk/rbob_*.sh` — container roles (implicit)

## What done looks like

- Operation verbs and container roles each declared once in a canonical bash home; call sites reference it; bash behavior unchanged; fast suite green.

**[260520-2035] rough**

## Character

Pure bash refactor — depot-free, emitter-independent, do-now-able. Judgment lives in choosing the single home; no behavior change.

## Goal

Give operation verbs and container roles each a single canonical bash home, so they become projectable under the heat's single-author rule. No Rust touched here.

## Shape

- Today these are implicit/scattered: operation verbs across tabtarget/zipper definitions (rbz), container roles across `rbob_*.sh`. Neither has one owner.
- Settle a single bash module that canonically declares each group as a named list — the home, not new behavior. Existing call sites reference the new home.
- Terms to home (verify against live files first): operation verbs — inscribe, yoke, ordain, enshrine, kludge; container roles — sentry, bottle.
- Prerequisite for the projection pace; touches no Rust and no manifest.

## Sources

- `Tools/rbk/rbz_zipper.sh` — operation verbs (implicit in tabtarget defs)
- `Tools/rbk/rbob_*.sh` — container roles (implicit)

## What done looks like

- Operation verbs and container roles each declared once in a canonical bash home; call sites reference it; bash behavior unchanged; fast suite green.

### ifrit-retire-claude-code (₢BPAAA) [complete]

**[260520-1731] complete**

## Character
Cross-cutting removal: spec reframe requiring judgment, then mechanical code/registry deletion + regen.

Retire Claude Code from the ifrit entirely. The ifrit becomes a purely scripted
adversary (the `rbid` binary + the sortie adjutant); the live-agent role belongs
to `rbev-bottle-ccyolo`, which hosts `@anthropic-ai/claude-code` and is reached
via the SSH model (`rbw-cS`). The ifrit Dockerfiles already install no node/npm/
claude — this pace makes the spec and the launcher machinery match that reality.

Two parts:

- **Spec reframe (judgment).** `Tools/rbk/vov_veiled/RBSIP-ifrit_pentester.adoc`.
  The SYSTEM CONCEPT currently defines an Ifrit *as* an adversarial AI agent with
  Anthropic-API connectivity. Reframe to scripted-adversary; drop the Claude Code
  runtime package rows and the "API connectivity for Claude Code" framing. While
  there, reconcile the package-list drift: scapy and git are listed in the spec
  but absent from the actual Dockerfiles — decide at mount whether they return to
  the Dockerfiles or leave the spec (the sortie adjutant does run python3).

- **Rip `rbw-Ic` IfritClient (mechanical).** It's dead (`exec ... claude` with no
  claude installed) and redundant with `rbw-cS`. Remove the launcher, its zipper
  enrollment, the per-nameplate tabtargets, and the generated context-doc row.
  Discovery: `grep -rin "ifrit_client\|IfritClient\|rbw-Ic" Tools/rbk/ tt/`.
  Regenerate the tabtarget context via the marshal-generate path after removal.

## Done looks like
ifrit is scripted-adversary only; ccyolo owns the live-agent role; `rbw-Ic` and
its enrollment/tabtargets/doc-row are gone; the regenerated context reflects it.

**[260520-1018] rough**

## Character
Cross-cutting removal: spec reframe requiring judgment, then mechanical code/registry deletion + regen.

Retire Claude Code from the ifrit entirely. The ifrit becomes a purely scripted
adversary (the `rbid` binary + the sortie adjutant); the live-agent role belongs
to `rbev-bottle-ccyolo`, which hosts `@anthropic-ai/claude-code` and is reached
via the SSH model (`rbw-cS`). The ifrit Dockerfiles already install no node/npm/
claude — this pace makes the spec and the launcher machinery match that reality.

Two parts:

- **Spec reframe (judgment).** `Tools/rbk/vov_veiled/RBSIP-ifrit_pentester.adoc`.
  The SYSTEM CONCEPT currently defines an Ifrit *as* an adversarial AI agent with
  Anthropic-API connectivity. Reframe to scripted-adversary; drop the Claude Code
  runtime package rows and the "API connectivity for Claude Code" framing. While
  there, reconcile the package-list drift: scapy and git are listed in the spec
  but absent from the actual Dockerfiles — decide at mount whether they return to
  the Dockerfiles or leave the spec (the sortie adjutant does run python3).

- **Rip `rbw-Ic` IfritClient (mechanical).** It's dead (`exec ... claude` with no
  claude installed) and redundant with `rbw-cS`. Remove the launcher, its zipper
  enrollment, the per-nameplate tabtargets, and the generated context-doc row.
  Discovery: `grep -rin "ifrit_client\|IfritClient\|rbw-Ic" Tools/rbk/ tt/`.
  Regenerate the tabtarget context via the marshal-generate path after removal.

## Done looks like
ifrit is scripted-adversary only; ccyolo owns the live-agent role; `rbw-Ic` and
its enrollment/tabtargets/doc-row are gone; the regenerated context reflects it.

### cross-language-constant-codegen (₢BPAAB) [abandoned]

**[260520-1738] abandoned**

## Character

Design-locked from an extended design session; multi-step — decompose into sub-paces at groom. Mechanical once the locked decisions are held. Gated: do not start until ₣BK closes.

## Goal

Replace the hand-maintained bash↔Rust constant mirrors with build-time generation from the canonical bash source, so cross-language drift becomes structurally impossible for everything projected. (Today the colophons are a hand-copied Rust mirror guarded only by a runtime assertion — `rbtdrm_verify` against the zipper's manifest string — which detects drift but does not prevent it.)

## Precondition

₣BK closed: RBCC stable, moorings layout settled, RBBC aliases retired. Building the projector against an in-flight RBCC is the one thing the design explicitly forbids.

## Locked shape

- **Two manifests, two projectors.** rbz (registry/colophons) → a generic BUK emitter over the zipper rolls; RBCC (flat tinder: moorings dir, `.env` filenames, roles) → an RBK projector. BUK provides mechanism and stays RBK-ignorant; RBK owns domain content. Dependency points RBK→BUK only. The proven precedent is `buz_emit_context`, which already generates the markdown command reference from the same kindled rolls — the Rust emitter is a sibling target, not new machinery.
- **Naming: carry the enroll varname verbatim.** The rbz enroll varname is the cross-language symbol; the generated Rust const takes that exact name, so BUK names nothing. Retires the curated `RBTDRM_COLOPHON_*` names (a rename across their reference sites).
- **Scope gate: single canonical bash author, or out.** Colophons and RBCC tinder qualify. Fixtures / operations / container-roles are out — no single bash author (held across `rbte_engine.sh` arrays, disk imprints, and Rust).
- **Engine change (BUK):** `buz_enroll` must retain the enroll varname in a roll; today it is `printf -v`'d into the caller and discarded, so the registry has no stable symbol for the emitter to name consts after.
- **Wiring:** generated file checked in; regenerated at the BUK-available layer before every Rust build; a build inside a minimal container consumes the committed artifact (never regenerates); a `git diff --exit-code` qualify gate makes a stale commit fail loud. This supersedes `rbtdrm_verify`'s drift role with exact equality.

## What done looks like

- The rbtd colophon consts and the RBCC-derived consts are generated, not hand-written; `rbtdrm_verify` is retired; inline colophon literals (e.g. in the fast suite) reference the generated consts; the diff-gate is wired into qualify.

## Sources

- `Tools/buk/buz_zipper.sh` — `buz_emit_context` is the bash→artifact template; engine that needs the varname roll
- `Tools/rbk/rbz_zipper.sh` — colophon registry (one source)
- `Tools/rbk/rbcc_Constants.sh` — RBCC flat tinder (the other source; already self-described as projection inventory)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — current hand-mirror + `rbtdrm_verify` (the target to replace)
- `Tools/rbk/rbtd/src/lib.rs` — `rbtd_moorings_dir!` macro (already-landed single-source precedent)

**[260520-1019] rough**

## Character

Design-locked from an extended design session; multi-step — decompose into sub-paces at groom. Mechanical once the locked decisions are held. Gated: do not start until ₣BK closes.

## Goal

Replace the hand-maintained bash↔Rust constant mirrors with build-time generation from the canonical bash source, so cross-language drift becomes structurally impossible for everything projected. (Today the colophons are a hand-copied Rust mirror guarded only by a runtime assertion — `rbtdrm_verify` against the zipper's manifest string — which detects drift but does not prevent it.)

## Precondition

₣BK closed: RBCC stable, moorings layout settled, RBBC aliases retired. Building the projector against an in-flight RBCC is the one thing the design explicitly forbids.

## Locked shape

- **Two manifests, two projectors.** rbz (registry/colophons) → a generic BUK emitter over the zipper rolls; RBCC (flat tinder: moorings dir, `.env` filenames, roles) → an RBK projector. BUK provides mechanism and stays RBK-ignorant; RBK owns domain content. Dependency points RBK→BUK only. The proven precedent is `buz_emit_context`, which already generates the markdown command reference from the same kindled rolls — the Rust emitter is a sibling target, not new machinery.
- **Naming: carry the enroll varname verbatim.** The rbz enroll varname is the cross-language symbol; the generated Rust const takes that exact name, so BUK names nothing. Retires the curated `RBTDRM_COLOPHON_*` names (a rename across their reference sites).
- **Scope gate: single canonical bash author, or out.** Colophons and RBCC tinder qualify. Fixtures / operations / container-roles are out — no single bash author (held across `rbte_engine.sh` arrays, disk imprints, and Rust).
- **Engine change (BUK):** `buz_enroll` must retain the enroll varname in a roll; today it is `printf -v`'d into the caller and discarded, so the registry has no stable symbol for the emitter to name consts after.
- **Wiring:** generated file checked in; regenerated at the BUK-available layer before every Rust build; a build inside a minimal container consumes the committed artifact (never regenerates); a `git diff --exit-code` qualify gate makes a stale commit fail loud. This supersedes `rbtdrm_verify`'s drift role with exact equality.

## What done looks like

- The rbtd colophon consts and the RBCC-derived consts are generated, not hand-written; `rbtdrm_verify` is retired; inline colophon literals (e.g. in the fast suite) reference the generated consts; the diff-gate is wired into qualify.

## Sources

- `Tools/buk/buz_zipper.sh` — `buz_emit_context` is the bash→artifact template; engine that needs the varname roll
- `Tools/rbk/rbz_zipper.sh` — colophon registry (one source)
- `Tools/rbk/rbcc_Constants.sh` — RBCC flat tinder (the other source; already self-described as projection inventory)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — current hand-mirror + `rbtdrm_verify` (the target to replace)
- `Tools/rbk/rbtd/src/lib.rs` — `rbtd_moorings_dir!` macro (already-landed single-source precedent)

### buz-rust-const-emitter (₢BPAAC) [complete]

**[260527-1119] complete**

## Character

Novel BUK mechanism plus the first feeder. The design choice is settled: a shared RBK-ignorant per-pair emit primitive, colophons landing in the `RBTDGC_` file. Additive, low-risk.

## Goal

Build the shared bash→Rust const-emit primitive and wire the zipper's colophons as its first feeder, landing them in the generated `RBTDGC_` file. The primitive names nothing domain-specific; colophons are the proving case.

## Shape

- `buz_enroll` must retain the enroll varname in a new co-indexed roll — today it is `printf -v`'d into the caller and discarded, and it is the source of the const stem. Additive: `printf -v` and the existing rolls stay, callers untouched.
- Add an RBK-ignorant BUK primitive that formats one `pub const NAME: &str = "VALUE";` line from a name and value, modeled on `buz_emit_context`'s emit shape. Per-pair scalar interface — no pair-list passing (bash 3.2).
- The zipper is the first feeder: walk the co-indexed rolls; for each colophon the const name is `RBTDGC_` + the enroll varname with its `RBZ_` prefix stripped (stem verbatim), the value is the colophon string.
- Wire the generation command to write the `RBTDGC_` file and give it the same fresh-generate-and-diff staleness gate the qualifier applies to the generated markdown context.

## Sources

- `Tools/buk/buz_zipper.sh` — `buz_enroll` (varname discard), `buz_emit_context` (emit template), `zbuz_kindle` (roll init)
- `Tools/rbk/rblm_cli.sh` — `rblm_generate` (markdown emit site to extend)
- `Tools/rbk/rbq_Qualify.sh` — the fresh-vs-committed staleness gate to mirror

## What done looks like

A varname roll exists; an RBK-ignorant per-pair emit primitive exists; the generation command writes a `rbtdgc_*` file whose colophon consts (`RBTDGC_*`) match the enrolled colophons, gated stale-on-drift. Consumers are NOT migrated here — that is the next pace. The BUK self-test is a no-regression check, not emitter verification.

**[260527-1034] rough**

## Character

Novel BUK mechanism plus the first feeder. The design choice is settled: a shared RBK-ignorant per-pair emit primitive, colophons landing in the `RBTDGC_` file. Additive, low-risk.

## Goal

Build the shared bash→Rust const-emit primitive and wire the zipper's colophons as its first feeder, landing them in the generated `RBTDGC_` file. The primitive names nothing domain-specific; colophons are the proving case.

## Shape

- `buz_enroll` must retain the enroll varname in a new co-indexed roll — today it is `printf -v`'d into the caller and discarded, and it is the source of the const stem. Additive: `printf -v` and the existing rolls stay, callers untouched.
- Add an RBK-ignorant BUK primitive that formats one `pub const NAME: &str = "VALUE";` line from a name and value, modeled on `buz_emit_context`'s emit shape. Per-pair scalar interface — no pair-list passing (bash 3.2).
- The zipper is the first feeder: walk the co-indexed rolls; for each colophon the const name is `RBTDGC_` + the enroll varname with its `RBZ_` prefix stripped (stem verbatim), the value is the colophon string.
- Wire the generation command to write the `RBTDGC_` file and give it the same fresh-generate-and-diff staleness gate the qualifier applies to the generated markdown context.

## Sources

- `Tools/buk/buz_zipper.sh` — `buz_enroll` (varname discard), `buz_emit_context` (emit template), `zbuz_kindle` (roll init)
- `Tools/rbk/rblm_cli.sh` — `rblm_generate` (markdown emit site to extend)
- `Tools/rbk/rbq_Qualify.sh` — the fresh-vs-committed staleness gate to mirror

## What done looks like

A varname roll exists; an RBK-ignorant per-pair emit primitive exists; the generation command writes a `rbtdgc_*` file whose colophon consts (`RBTDGC_*`) match the enrolled colophons, gated stale-on-drift. Consumers are NOT migrated here — that is the next pace. The BUK self-test is a no-regression check, not emitter verification.

**[260527-1006] rough**

## Character

Mechanical, additive, low-risk. The one design choice is settled: the emit lives in the zipper as a twin of `buz_emit_context`, scoped to colophons. RBK-ignorant — the zipper names nothing domain-specific.

## Goal

Make the zipper project its enrolled colophons to Rust consts on demand, mirroring how `buz_emit_context` projects them to markdown. Colophons only — other constant groups are later paces.

## Shape

- `buz_enroll` must retain the enroll varname in a new co-indexed roll. Today the varname is `printf -v`'d into the caller and discarded; it is the symbol the Rust const is named after. Additive: `printf -v` and the existing rolls stay, existing callers untouched.
- Add a sibling emit function modeled line-for-line on `buz_emit_context`: same index walk over the co-indexed rolls, printing `pub const VARNAME: &str = "colophon";` to stdout. It reads its own rolls — no accessor, no generic pair-passing. Varname roll supplies the const name, colophon roll the value.
- Wire the generation command to emit the Rust file alongside the markdown, and give that file the same fresh-generate-and-diff staleness gate the qualifier already applies to the generated markdown context.

## Sources

- `Tools/buk/buz_zipper.sh` — `buz_enroll` (varname discard), `buz_emit_context` (emit template), `zbuz_kindle` (roll init)
- `Tools/rbk/rblm_cli.sh` — `rblm_generate` (the markdown emit site to extend)
- `Tools/rbk/rbq_Qualify.sh` — the fresh-vs-committed staleness gate to mirror

## What done looks like

A varname roll exists and the new emit walks the co-indexed rolls to produce `pub const` lines for every enrolled colophon. The Rust file is generated by the generation command and gated stale-on-drift by the qualifier, the same as the markdown. Acceptance is the emitted output matching the enrolled colophons; the BUK self-test is a no-regression check, not emitter verification.

**[260520-2033] rough**

## Character

Novel BUK mechanism — design judgment lives in the emitter shape; the engine change is additive and low-risk. RBK-ignorant: BUK provides mechanism only, names nothing of its own.

## Goal

Give the zipper registry a stable cross-language symbol per enrolled colophon, and a generic emitter that projects name/value pairs to Rust consts — mirroring the existing bash-to-artifact pattern of `buz_emit_context`.

## Shape

- `buz_enroll` must retain the enroll varname in a new roll. Today it `printf -v`'s the colophon into the caller and discards the varname, so the registry has no stable symbol for an emitter to name consts after. Additive: `printf -v` stays, existing callers untouched.
- Add a generic Rust-const emitter (sibling target to `buz_emit_context`). DECIDED: the emitter is generic — it takes name/value pairs, not zipper rolls only. The later RBCC, fixture, and scattered-group projectors all feed this one mechanism. For the colophon case the pairs come from the kindled rolls with const name = enroll varname verbatim; other callers supply their own pairs.
- The emitter emits `pub const NAME: &str = "...";` lines and is RBK-ignorant — it names nothing domain-specific itself.

## Sources

- `Tools/buk/buz_zipper.sh` — `buz_enroll` (varname discard site) and `buz_emit_context` (the bash-to-artifact template)

## What done looks like

- A roll carries enroll varnames; a generic BUK emitter turns name/value pairs into `pub const NAME: &str = "...";` lines.
- Acceptance is the emitter's own output: feed it the rbz rolls and confirm the emitted consts match the expected colophons. The BUK self-test is a no-regression check, not emitter verification.

**[260520-1743] rough**

## Character

Novel BUK mechanism — design judgment lives in the emitter shape; the engine change is additive and low-risk. RBK-ignorant: BUK provides mechanism only, names nothing of its own.

## Goal

Give the zipper registry a stable cross-language symbol per enrolled colophon, and a generic emitter that projects those symbols to Rust consts — mirroring the existing bash-to-artifact pattern of `buz_emit_context`.

## Shape

- `buz_enroll` must retain the enroll varname in a new roll. Today it `printf -v`'s the colophon into the caller and discards the varname, so the registry has no stable symbol for an emitter to name consts after. Additive: `printf -v` stays, existing callers untouched.
- Add a generic Rust-const emitter (sibling target to `buz_emit_context`) over the kindled rolls. The generated const name is the enroll varname verbatim.
- Open design question to settle here: does this emitter stay zipper-roll-only (the RBCC projector in the next paces being separate machinery), or is it shaped to accept arbitrary name/value pairs so the RBK RBCC projector can reuse it? The original locked shape says "BUK provides mechanism, RBK owns domain content" but also frames them as two distinct projectors — resolve before building.

## Sources

- `Tools/buk/buz_zipper.sh` — `buz_enroll` (varname discard site) and `buz_emit_context` (the bash-to-artifact template)

## What done looks like

- A roll carries enroll varnames; a BUK emitter turns the rolls into `pub const NAME: &str = "...";` lines.
- Acceptance is the emitter's own output: run it against the rbz rolls and confirm the emitted consts match the expected colophons. The BUK self-test is a no-regression check, not emitter verification.

**[260520-1737] rough**

## Character

Novel BUK mechanism — design judgment lives in the emitter shape; the engine change is additive and low-risk. RBK-ignorant: BUK provides mechanism only, names nothing of its own.

## Goal

Give the zipper registry a stable cross-language symbol per enrolled colophon, and a generic emitter that projects those symbols to Rust consts — mirroring the existing bash-to-artifact pattern of `buz_emit_context`.

## Shape

- `buz_enroll` must retain the enroll varname in a new roll. Today it `printf -v`'s the colophon into the caller and discards the varname, so the registry has no stable symbol for an emitter to name consts after. Additive: `printf -v` stays, existing callers untouched.
- Add a generic Rust-const emitter (sibling target to `buz_emit_context`) over the kindled rolls. The generated const name is the enroll varname verbatim.
- Generic and domain-agnostic — emits whatever rolls it is handed. Dependency points RBK to BUK only.

## Sources

- `Tools/buk/buz_zipper.sh` — `buz_enroll` (varname discard site) and `buz_emit_context` (the bash-to-artifact template)

## What done looks like

- A roll carries enroll varnames; a BUK emitter turns the rolls into `pub const NAME: &str = "...";` lines; the BUK self-test stays green.

### colophon-codegen-projection (₢BPAAD) [complete]

**[260527-1141] complete**

## Character

Mechanical but broad — a consumer rename across the rbtd Rust sources.

## Goal

Migrate every colophon consumer off the hand-mirror `RBTDRM_COLOPHON_*` and inline colophon literals onto the generated `RBTDGC_*` names, retiring the hand-written colophon consts. The runtime drift check stays alive here.

## Shape

- The generated colophon consts already exist (emitter pace). This pace retires the curated `RBTDRM_COLOPHON_*` names and points consumers at `RBTDGC_*`.
- Discover sites with `grep -rln "RBTDRM_COLOPHON" Tools/rbk/rbtd/src/` plus the inline colophon literals (e.g. the fast suite).
- Repoint `rbtdrm_required_colophons` to the generated names but keep `rbtdrm_verify` running — do NOT retire verify here; that belongs with the diff-gate landing so drift protection is continuous.

## Sources

- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — `RBTDRM_COLOPHON_*` mirror, `rbtdrm_verify`, `rbtdrm_required_colophons`
- the generated `rbtdgc_*` colophon consts (from the emitter pace)

## What done looks like

No `RBTDRM_COLOPHON_*` consts remain; consumers and inline literals reference `RBTDGC_*`; `rbtdrm_verify` still runs against the generated consts; Rust builds and the fast suite is green.

**[260527-1034] rough**

## Character

Mechanical but broad — a consumer rename across the rbtd Rust sources.

## Goal

Migrate every colophon consumer off the hand-mirror `RBTDRM_COLOPHON_*` and inline colophon literals onto the generated `RBTDGC_*` names, retiring the hand-written colophon consts. The runtime drift check stays alive here.

## Shape

- The generated colophon consts already exist (emitter pace). This pace retires the curated `RBTDRM_COLOPHON_*` names and points consumers at `RBTDGC_*`.
- Discover sites with `grep -rln "RBTDRM_COLOPHON" Tools/rbk/rbtd/src/` plus the inline colophon literals (e.g. the fast suite).
- Repoint `rbtdrm_required_colophons` to the generated names but keep `rbtdrm_verify` running — do NOT retire verify here; that belongs with the diff-gate landing so drift protection is continuous.

## Sources

- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — `RBTDRM_COLOPHON_*` mirror, `rbtdrm_verify`, `rbtdrm_required_colophons`
- the generated `rbtdgc_*` colophon consts (from the emitter pace)

## What done looks like

No `RBTDRM_COLOPHON_*` consts remain; consumers and inline literals reference `RBTDGC_*`; `rbtdrm_verify` still runs against the generated consts; Rust builds and the fast suite is green.

**[260520-1743] rough**

## Character

Mechanical once the emitter exists, but broad — a consumer rename across the rbtd Rust sources.

## Goal

Generate the rbtd colophon consts from the rbz registry via the BUK emitter and migrate every consumer to the generated names, retiring the hand-mirror. The runtime drift check stays alive here and is retired only when the diff-gate replaces it.

## Shape

- Wire the rbz colophon rolls through the BUK emitter to a checked-in generated Rust file.
- Generated const names carry the enroll varname verbatim — this retires the curated `RBTDRM_COLOPHON_*` names, a rename across their reference sites.
- Migrate the Rust consumers off `RBTDRM_COLOPHON_*` and the inline colophon literals (e.g. the fast suite) onto the generated consts. Discover sites with `grep -rln "RBTDRM_COLOPHON" Tools/rbk/rbtd/src/`.
- Repoint `rbtdrm_required_colophons` to the generated names but keep `rbtdrm_verify` running. Do NOT retire verify here — that belongs with the diff-gate landing, so drift protection is continuous (no gap between this pace and the gate).
- Out of scope, left hand-written: `RBTDRM_FIXTURE_*`, `RBTDRM_OPERATION_*`, `RBTDRM_CONTAINER_*` — no single canonical bash author.

## Sources

- `Tools/rbk/rbz_zipper.sh` — colophon registry
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — hand-mirror + `rbtdrm_verify` + `rbtdrm_required_colophons`

## What done looks like

- Colophon consts are generated, not hand-written; consumers and inline literals reference the generated names; `rbtdrm_verify` still runs (against generated consts); Rust builds and the fast suite is green.

**[260520-1737] rough**

## Character

Mechanical once the emitter exists, but broad — an 11-site consumer rename plus a retirement.

## Goal

Generate the rbtd colophon consts from the rbz registry via the BUK emitter, migrate every consumer to the generated names, and retire the hand-mirror plus its runtime drift check.

## Shape

- Wire the rbz colophon rolls through the BUK emitter to a checked-in generated Rust file.
- Generated const names carry the enroll varname verbatim — this retires the curated `RBTDRM_COLOPHON_*` names, a rename across their reference sites.
- Migrate the Rust consumers off `RBTDRM_COLOPHON_*` and the inline colophon literals (e.g. the fast suite) onto the generated consts.
- Retire `rbtdrm_verify` — exact-equality generation supersedes its drift-detection role.
- Out of scope, left hand-written: `RBTDRM_FIXTURE_*`, `RBTDRM_OPERATION_*`, `RBTDRM_CONTAINER_*` — no single canonical bash author.

## Sources

- `Tools/rbk/rbz_zipper.sh` — colophon registry
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — hand-mirror + `rbtdrm_verify` (target to replace)
- `grep -rln "RBTDRM_COLOPHON\|rbtdrm_verify" Tools/rbk/rbtd/src/` — consumer sites

## What done looks like

- Colophon consts are generated, not hand-written; consumers and inline literals reference the generated names; `rbtdrm_verify` is gone; Rust builds and the fast suite is green.

### rbcc-tinder-codegen-projection (₢BPAAE) [complete]

**[260527-1823] complete**

## Character

RBK projector over the single-homed RBCC constants; supersedes the lib.rs path macros, the manifest credential-role mirror, and the scattered .env-filename literals. Covers the full RBCC-owned co-maintained set.

## Goal

Project every co-maintained constant that `rbcc_Constants.sh` canonically owns to generated `RBTDGC_*` consts via the shared emit primitive, retiring the hand-maintained mirrors.

## Shape

- Scope (full single-homed RBCC set): moorings dir, vessels dir, credential roles, .env filenames. Roles and .env names are hand-copied into Rust today — generation removes a live drift risk, it does not create a new home.
- Feed each as a name/value pair through the shared primitive into the `RBTDGC_` file; the const name is `RBTDGC_` + the RBCC stem (bash prefix stripped), value verbatim.
- Supersede: `rbtd_moorings_dir!` / `rbtd_vessels_dir!` macros and `RBTD_MOORINGS_DIR` in lib.rs; the credential-role consts in `rbtdrm_manifest.rs`; the .env-filename literals in `rbtdrp_pristine.rs` / `rbtdrk_canonical.rs`.
- Constraint: the macro form exists because `concat!` needs a literal token, not a const identifier (see the lib.rs comment). Preserve literal-composability for any `concat!` path sites, or migrate those to runtime join.
- Note: two credential roles (assay, mason) live only in bash with no Rust mirror — projecting brings them across cleanly; confirm consumers don't already hardcode them.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — flat tinder: moorings/vessels dirs, credential roles, .env filenames
- `Tools/rbk/rbtd/src/lib.rs` — `rbtd_moorings_dir!` / `rbtd_vessels_dir!` macros
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — credential-role mirror
- `Tools/rbk/rbtd/src/rbtdrp_pristine.rs`, `rbtdrk_canonical.rs` — scattered .env-filename literals

## What done looks like

The single-homed RBCC consts are generated `RBTDGC_*`, not hand-written; the lib.rs macros, manifest role mirror, and scattered .env literals reference generation or are retired; Rust builds; moorings paths resolve unchanged; the fast suite is green.

**[260527-1034] rough**

## Character

RBK projector over the single-homed RBCC constants; supersedes the lib.rs path macros, the manifest credential-role mirror, and the scattered .env-filename literals. Covers the full RBCC-owned co-maintained set.

## Goal

Project every co-maintained constant that `rbcc_Constants.sh` canonically owns to generated `RBTDGC_*` consts via the shared emit primitive, retiring the hand-maintained mirrors.

## Shape

- Scope (full single-homed RBCC set): moorings dir, vessels dir, credential roles, .env filenames. Roles and .env names are hand-copied into Rust today — generation removes a live drift risk, it does not create a new home.
- Feed each as a name/value pair through the shared primitive into the `RBTDGC_` file; the const name is `RBTDGC_` + the RBCC stem (bash prefix stripped), value verbatim.
- Supersede: `rbtd_moorings_dir!` / `rbtd_vessels_dir!` macros and `RBTD_MOORINGS_DIR` in lib.rs; the credential-role consts in `rbtdrm_manifest.rs`; the .env-filename literals in `rbtdrp_pristine.rs` / `rbtdrk_canonical.rs`.
- Constraint: the macro form exists because `concat!` needs a literal token, not a const identifier (see the lib.rs comment). Preserve literal-composability for any `concat!` path sites, or migrate those to runtime join.
- Note: two credential roles (assay, mason) live only in bash with no Rust mirror — projecting brings them across cleanly; confirm consumers don't already hardcode them.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — flat tinder: moorings/vessels dirs, credential roles, .env filenames
- `Tools/rbk/rbtd/src/lib.rs` — `rbtd_moorings_dir!` / `rbtd_vessels_dir!` macros
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — credential-role mirror
- `Tools/rbk/rbtd/src/rbtdrp_pristine.rs`, `rbtdrk_canonical.rs` — scattered .env-filename literals

## What done looks like

The single-homed RBCC consts are generated `RBTDGC_*`, not hand-written; the lib.rs macros, manifest role mirror, and scattered .env literals reference generation or are retired; Rust builds; moorings paths resolve unchanged; the fast suite is green.

**[260520-2033] rough**

## Character

RBK projector over the single-homed RBCC constants; supersedes the lib.rs path macros, the manifest's credential-role mirror, and the scattered .env-filename literals. Broader than originally scoped — covers the full RBCC-owned co-maintained set.

## Goal

Project every co-maintained constant that `rbcc_Constants.sh` canonically owns to generated Rust consts via the generic BUK emitter, retiring the hand-maintained mirrors.

## Shape

- Scope DECIDED (full single-homed RBCC set): moorings dir, vessels dir, credential roles, and .env filenames. Roles and .env names are hand-copied into Rust today — bringing them under generation removes a live drift risk, it does not create a new home.
- Feeds the generic emitter from the emitter pace as name/value pairs — no standalone RBK codegen.
- Supersedes: `rbtd_moorings_dir!` / `rbtd_vessels_dir!` macros and `RBTD_MOORINGS_DIR` in lib.rs; the credential-role consts in `rbtdrm_manifest.rs`; the .env-filename literals scattered in `rbtdrp_pristine.rs` / `rbtdrk_canonical.rs`.
- Constraint: the macro form exists because `concat!` needs a literal token, not a const identifier (see the lib.rs comment). Preserve literal-composability for any `concat!` path-composition sites, or migrate those sites to runtime join.
- Note: two credential roles (assay, mason) live only in bash with no Rust mirror — projecting brings them across cleanly; confirm consumers don't already hardcode them.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — flat tinder: moorings/vessels dirs, credential roles, .env filenames
- `Tools/rbk/rbtd/src/lib.rs` — `rbtd_moorings_dir!` / `rbtd_vessels_dir!` macros
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — credential-role mirror
- `Tools/rbk/rbtd/src/rbtdrp_pristine.rs`, `rbtdrk_canonical.rs` — scattered .env-filename literals

## What done looks like

- The single-homed RBCC consts (moorings, vessels, roles, .env names) are generated, not hand-written; the lib.rs macros, manifest role mirror, and scattered .env literals reference generation or are retired; Rust builds; moorings paths resolve unchanged; the fast suite is green.

**[260520-1743] rough**

## Character

Self-contained RBK projector; supersedes one existing macro hand-mirror. Carries an unresolved scope question — settle it before projecting.

## Goal

Project the in-scope RBCC flat tinder to generated Rust consts via an RBK-owned projector, replacing the hand-maintained `rbtd_moorings_dir!` macro mirror.

## Shape

- Scope question to resolve first: the heat's theme is removing constants *redundantly held in two places*. Today only `RBCC_moorings_dir` has a Rust mirror (the macro); `.env` filenames and roles live only in bash and are already single-homed. The original locked shape names "moorings dir, .env filenames, roles" as the projector input — but projecting the single-homed ones would *create* a second home, against the heat theme. Decide at mount: project only constants with a Rust mirror today, or the full named tinder set.
- Relationship to the BUK emitter (decided in the emitter pace): either the RBK projector reuses that mechanism by feeding RBCC values as name/value pairs, or it is standalone RBK codegen. This pace does not depend on the colophon-projection pace — independent sibling, ordered after only for sequential convenience.
- Supersedes the `rbtd_moorings_dir!` / `rbtd_vessels_dir!` macros and `RBTD_MOORINGS_DIR` in `lib.rs`.
- Constraint: the macro form exists because `concat!` needs a literal token, not a const identifier (see the `lib.rs` comment). Preserve literal-composability for any `concat!` path-composition sites, or migrate those sites to runtime join.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — flat tinder (self-described projection inventory)
- `Tools/rbk/rbtd/src/lib.rs` — `rbtd_moorings_dir!` macro (current hand-mirror to supersede)

## What done looks like

- The in-scope RBCC consts are generated; the moorings macro mirror is retired or fed from generation; Rust builds; moorings paths resolve unchanged.

**[260520-1737] rough**

## Character

Self-contained RBK projector; supersedes one existing macro hand-mirror. Watch the `concat!`-composability constraint.

## Goal

Project the in-scope RBCC flat tinder (moorings dir, `.env` filenames, roles) to generated Rust consts, replacing the hand-maintained `rbtd_moorings_dir!` macro mirror.

## Shape

- An RBK projector (RBK owns domain content) emits the in-scope RBCC tinder to a checked-in generated Rust file.
- Supersedes the `rbtd_moorings_dir!` / `rbtd_vessels_dir!` macros and `RBTD_MOORINGS_DIR` in `lib.rs`.
- Constraint: the macro form exists because `concat!` needs a literal token, not a const identifier (see the `lib.rs` comment). Preserve literal-composability for any `concat!` path-composition sites, or migrate those sites.
- Scope: single-canonical-author source-time literals only. Composed/kindled values lacking a clean single-author projection stay as-is.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — flat tinder (self-described projection inventory)
- `Tools/rbk/rbtd/src/lib.rs` — `rbtd_moorings_dir!` macro (current hand-mirror to supersede)

## What done looks like

- RBCC tinder consts are generated; the moorings macro mirror is retired or fed from generation; Rust builds; moorings paths resolve unchanged.

### fixture-codegen-projection (₢BPAAG) [abandoned]

**[260529-1254] abandoned**

## Character

Mechanical projection over a single-homed group; mirrors the colophon shape against a different bash owner.

## Goal

Generate the rbtd fixture-name consts from their canonical bash owner via the shared primitive and migrate consumers off the hand-mirror in `rbtdrm_manifest.rs`.

## Shape

- Fixture names are canonically authored in `Tools/rbk/rbtd/rbte_engine.sh` and hand-copied into `rbtdrm_manifest.rs`. Single home exists — projectable without a tidy step.
- Feed the names through the shared primitive into the `RBTDGC_` file (const name `RBTDGC_` + fixture stem); migrate manifest consumers and any inline fixture literals onto the generated consts.
- Keep the runtime drift check alive until the diff-gate lands.

## Sources

- `Tools/rbk/rbtd/rbte_engine.sh` — fixture-name canonical author
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — fixture hand-mirror

## What done looks like

Fixture consts are generated `RBTDGC_*`, not hand-written; consumers reference the generated names; Rust builds and the fast suite is green.

**[260528-1040] rough**

## Character

Intricate test-infrastructure refactor, premise-corrected: Rust authors fixtures; bash only mirrors their names in suite arrays. This is an ownership relocation, NOT a bash→Rust projection — there is no codegen and no diff-gate here (those belong to the sibling projection paces). Study has de-risked it: the bash surface is tiny, the work is almost entirely Rust, and the one design fork is now locked. Verification is behavioral, not just a green build — this touches the spine of every `rbw-ts` run.

## Goal

Make theurge (Rust) the sole owner of test vocabulary: relocate suite definitions into theurge, derive fixture names from Rust identifiers, and retire both the bash suite arrays and the `RBTDRM_FIXTURE_*` const family.

## Locked design

- **Fixture name = Rust identifier.** Add a `fixture!` `macro_rules!` deriving `name` via `stringify!` of an identifier token — the `case!` macro in `rbtdre_engine.rs` is the exact precedent. No `paste`, no concat crate.
- **Nameplate monikers stay verbatim.** `rbtdri_invocation.rs` documents that crucible fixture names ARE nameplate monikers (they drive `rbw-cC.Charge.{name}.sh`). `tadmor`, `moriah`, `srjcl`, `pluml`, `dogfight` are single tokens already — their derived names MUST equal today's strings. Only multi-word kebab names become underscore identifiers; that wire-name shift for `rbw-tf`/`rbw-tc` is accepted.
- **`required_colophons` folds into the `rbtdre_Fixture` struct** (operator-accepted). Each fixture declares its own; shared sets are named `&'static [&'static str]` statics referenced by multiple fixtures (the crucible four, the empty fast/calibrant group, etc. — preserve today's groupings as single-source). `rbtdrm_required_colophons` collapses to a registry lookup returning the field; `rbtdrm_verify` keeps its signature and behavior. Delete the entire `RBTDRM_FIXTURE_*` family.
- **Suites move to a new Rust module, minted prefix `rbtdrs`.** A suite is compile-checked fixture refs (`&'static rbtdre_Fixture`), never name strings. Express composition as base-suite + extras concatenated at resolve time (runtime Vec chaining), so a base like `fast` stays single-source. Each suite carries a human description + optional precondition, PRINTED at runtime at suite start (superseding the buried bash precondition comments).
- **Binary gains a `suite` dispatch mode** beside `single`. It resolves the suite, prints description/precondition, runs the tree-clean guard ONCE at suite start, then loops fixtures (per-fixture verify + setup/charge + cases + teardown/quench), stopping at the first fixture that reports failures — preserving the cross-fixture fail-fast the bash `set -e` loop gives gauntlet/skirmish today.
- **`rbte_suite` (bash) collapses** to: build binary, forward the suite name to the binary's `suite` mode. Delete `ZRBTE_SUITE_*` and `zrbte_resolve_suite`; update the suite-name list in the `RBZ_THEURGE_SUITE` enroll description in `rbz_zipper.sh`.

## Out of scope / leave alone

- Operation-verb, container-role, and module/probate consts in `rbtdrm_manifest.rs` are NOT fixture names — leave them. `rbtdrm_credential_check_colophon` (role→probe) untouched.
- Calibrants are never added to a `rbw-ts` suite — they intentionally Fail/Skip and are asserted only under `rbw-tt`. Document, don't suite.
- No generated file, no `rbq` diff-gate added. Do not edit memos or retired `jjh_*` records.

## Discovery (sites drift — use these, don't enumerate)

- `grep -rn RBTDRM_FIXTURE_ Tools/rbk/rbtd/src` — every consumer to repoint: source fixture statics (`name:` field → macro) and test lookups/asserts.
- Suites + forwarder live in `rbte_engine.sh`; preserve each suite's exact membership and order from the current `ZRBTE_SUITE_*` arrays (they are the behavioral contract).

## Sources

- `Tools/rbk/rbtd/src/rbtdre_engine.rs` — `rbtdre_Fixture` struct (add `required_colophons`), `case!` precedent for `fixture!`
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` — `RBTDRC_FIXTURES` registry + `rbtdrc_lookup_fixture`
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — retire `RBTDRM_FIXTURE_*`; collapse `rbtdrm_required_colophons`
- `Tools/rbk/rbtd/src/main.rs` — add `suite` mode
- `Tools/rbk/rbtd/src/lib.rs` — register the `rbtdrs` module
- `Tools/rbk/rbtd/rbte_engine.sh` + `Tools/rbk/rbz_zipper.sh` — bash forwarder + suite-name description
- `CLAUDE.md` (Test Execution), `Tools/buk/claude-buk-core.md`, `Tools/rbk/rbk-claude-theurge-ifrit-context.md` — nav docs; add a "calibrant" definition and the `rbw-tt` vs `rbw-ts` split

## What done looks like

`rbw-ts fast` runs the identical fixture set in identical order as today; every suite resolves to its current membership. No `ZRBTE_SUITE_*`/`zrbte_resolve_suite`; no `RBTDRM_FIXTURE_*` anywhere. `rbw-tb` builds deny-warnings clean, `rbw-tt` green (calibrants included), `rbw-tq` clean. Nav docs describe the new model and define "calibrant" + the `rbw-tt`/`rbw-ts` split.

**[260528-1032] rough**

## Character

Intricate test-infrastructure refactor carrying a premise correction: the prior docket had ownership backwards. Rust authors fixtures (the `RBTDRC_FIXTURES` registry, fixture structs naming themselves); bash only holds redundant copies in its suite arrays. This is not a bash→Rust projection — it is a consolidation of test vocabulary into its rightful owner. Touches the spine of every `rbw-ts` run, so verification is behavioral, not just a green build.

## Goal

Make theurge (Rust) the sole owner of test vocabulary: relocate suite definitions from bash into theurge, and collapse fixture-name identity onto Rust identifiers — retiring every hand-maintained name string on both sides of the boundary.

## Shape

One end state ("Rust owns what fixtures exist, how they group, and what they're named"), reached along two strands:

- **Suites into theurge.** The `ZRBTE_SUITE_*` arrays and `zrbte_resolve_suite` in `rbte_engine.sh` are the redundant copy of Rust-owned fixture names. Move suite definitions into a new Rust module (minted prefix `rbtdrs`, "suites"). A suite is compile-checked **fixture refs** (the statics `RBTDRC_FIXTURES` already holds), never name strings. `rbte_suite` collapses to: build binary, forward the suite name; the binary grows a `suite` dispatch mode beside its existing `single` mode. Each suite carries a human description + optional precondition **printed at runtime** at suite start, so a failure shows its context — superseding the buried bash precondition comments.

- **Fixture identity = Rust identifier.** Retire the `RBTDRM_FIXTURE_*` kebab consts; a fixture's name derives from its Rust identifier (the `case!` macro in `rbtdre_engine.rs` is the stringify precedent). Repoint every consumer (`grep RBTDRM_FIXTURE_ Tools/rbk/rbtd/src/`) and `rbtdrm_required_colophons` onto the derived names. Operator wire-names for `rbw-tf`/`rbw-tc` shift kebab→identifier; accepted, since individual fixture runs are rare.

## Locked constraints

- **Zero new crates.** The suite registry and name derivation use native `macro_rules!` only — no `paste`, no concat crate. Derive names via `stringify!`; express composition as base-suite + extras concatenated at resolve time, so a base like `fast` stays single-source (no flat re-enumeration, no multi-place edits).
- **Calibrants stay out of `rbw-ts` suites.** Their cases intentionally Fail/Skip to calibrate the harness, asserted under `rbw-tt` (`rbtdtl_calibrant.rs`). They are documented, not suited.
- **Navigational context is part of done.** Update the CLAUDE.md "Test Execution" section and `Tools/rbk/rbk-claude-theurge-ifrit-context.md` so a restarted chat navigates the new model without rediscovery — including a definition of "calibrant" and the `rbw-tt` vs `rbw-ts` split.

## Sources

- `Tools/rbk/rbtd/rbte_engine.sh` — suite arrays + `rbte_suite` (→ forwarder)
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` — `RBTDRC_FIXTURES` registry + lookup
- `Tools/rbk/rbtd/src/rbtdre_engine.rs` — `Fixture` struct + `case!` stringify precedent
- `Tools/rbk/rbtd/src/main.rs` — binary dispatch (add `suite` mode)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — `RBTDRM_FIXTURE_*` hand-mirror + `rbtdrm_required_colophons`
- `Tools/rbk/rbtd/src/rbtdtl_calibrant.rs` — calibrant meta-tests (held separate)

## What done looks like

`rbw-ts fast` runs the identical fixture set in identical order as today, and every suite name resolves to the membership it has now. No `ZRBTE_SUITE_*`/`zrbte_resolve_suite` in bash; no `RBTDRM_FIXTURE_*` strings anywhere. `rbw-tb` builds deny-warnings clean, `rbw-tt` green (calibrants included), `rbw-tq` clean. CLAUDE.md and the theurge context file describe the new test model and define "calibrant."

**[260528-1026] rough**

## Character

Intricate test-infrastructure refactor carrying a premise correction: the prior docket had ownership backwards. Rust authors fixtures (the `RBTDRC_FIXTURES` registry, fixture structs naming themselves); bash only holds redundant copies in its suite arrays. This is not a bash→Rust projection — it is a consolidation of test vocabulary into its rightful owner. Touches the spine of every `rbw-ts` run, so verification is behavioral, not just a green build.

## Goal

Make theurge (Rust) the sole owner of test vocabulary: relocate suite definitions from bash into theurge, and collapse fixture-name identity onto Rust identifiers — retiring every hand-maintained name string on both sides of the boundary.

## Shape

One end state ("Rust owns what fixtures exist, how they group, and what they're named"), reached along two strands:

- **Suites into theurge.** The `ZRBTE_SUITE_*` arrays and `zrbte_resolve_suite` in `rbte_engine.sh` are the redundant copy of Rust-owned fixture names. Move suite definitions into a new Rust module (minted prefix `rbtdrs`, "suites"). A suite is compile-checked **fixture refs** (the statics `RBTDRC_FIXTURES` already holds), never name strings. `rbte_suite` collapses to: build binary, forward the suite name; the binary grows a `suite` dispatch mode beside its existing `single` mode. Each suite carries a human description + optional precondition **printed at runtime** at suite start, so a failure shows its context — superseding the buried bash precondition comments.

- **Fixture identity = Rust identifier.** Retire the `RBTDRM_FIXTURE_*` kebab consts; a fixture's name derives from its Rust identifier (the `case!` macro in `rbtdre_engine.rs` is the stringify precedent). Repoint every consumer (`grep RBTDRM_FIXTURE_ Tools/rbk/rbtd/src/`) and `rbtdrm_required_colophons` onto the derived names. Operator wire-names for `rbw-tf`/`rbw-tc` shift kebab→identifier; accepted, since individual fixture runs are rare.

## Locked constraints

- **Zero new crates.** The suite registry and name derivation use native `macro_rules!` only — no `paste`, no concat crate. Derive names via `stringify!`; express composition as base-suite + extras concatenated at resolve time, so a base like `fast` stays single-source (no flat re-enumeration, no multi-place edits).
- **Calibrants stay out of `rbw-ts` suites.** Their cases intentionally Fail/Skip to calibrate the harness, asserted under `rbw-tt` (`rbtdtl_calibrant.rs`). They are documented, not suited.
- **Navigational context is part of done.** Update the CLAUDE.md "Test Execution" section and `Tools/rbk/rbk-claude-theurge-ifrit-context.md` so a restarted chat navigates the new model without rediscovery — including a definition of "calibrant" and the `rbw-tt` vs `rbw-ts` split.

## Sources

- `Tools/rbk/rbtd/rbte_engine.sh` — suite arrays + `rbte_suite` (→ forwarder)
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` — `RBTDRC_FIXTURES` registry + lookup
- `Tools/rbk/rbtd/src/rbtdre_engine.rs` — `Fixture` struct + `case!` stringify precedent
- `Tools/rbk/rbtd/src/main.rs` — binary dispatch (add `suite` mode)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — `RBTDRM_FIXTURE_*` hand-mirror + `rbtdrm_required_colophons`
- `Tools/rbk/rbtd/src/rbtdtl_calibrant.rs` — calibrant meta-tests (held separate)

## What done looks like

`rbw-ts fast` runs the identical fixture set in identical order as today, and every suite name resolves to the membership it has now. No `ZRBTE_SUITE_*`/`zrbte_resolve_suite` in bash; no `RBTDRM_FIXTURE_*` strings anywhere. `rbw-tb` builds deny-warnings clean, `rbw-tt` green (calibrants included), `rbw-tq` clean. CLAUDE.md and the theurge context file describe the new test model and define "calibrant."

**[260528-0856] rough**

## Character

Mechanical projection over a bash-authored group; mirrors the colophon/rbcc shape against the fixture-name owner. Carries a premise correction from the prior docket.

## Goal

Project the bash-authored fixture names into generated `RBTDGC_*` consts via the shared `buz_emit_const` primitive, and repoint their consumers off the hand-mirror in `rbtdrm_manifest.rs`.

## Shape

- Fixture names live spread across the `ZRBTE_SUITE_*` arrays in `rbte_engine.sh` — there is no single canonical array today. A minimal tidy is required: declare the fixture set once as a flat emit list; the suite arrays stay literal subsets (they select fixtures, they do not declare them). Suite-vs-list drift is already caught — an unknown fixture name fails resolution.
- Only the bash-authored fixtures project (those named in the suite arrays). The calibrant fixtures are Rust-only synthetic (`rbtdrl_calibrant.rs`, no bash home) and stay out of scope per the heat's single-bash-author rule — their `RBTDRM_FIXTURE_CALIBRANT_*` consts remain hand-written in the manifest. The hand-mirror shrinks; it does not vanish.
- Feed the names through the shared primitive into the `RBTDGC_` file (const name `RBTDGC_` + fixture stem, hyphens to underscores, uppercased); migrate the bash-authored manifest consumers and any inline fixture literals onto the generated consts. Mirror `rbcc_emit_consts` for the emit shape and `rbz_emit_consts` for the combiner wiring.
- Keep the runtime drift check alive until the diff-gate lands.

## Sources

- `Tools/rbk/rbtd/rbte_engine.sh` — fixture-name author (suite arrays)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — fixture hand-mirror
- `Tools/rbk/rbtd/src/rbtdrl_calibrant.rs` — Rust-only fixtures (out of scope)

## What done looks like

The bash-authored fixture consts are generated `RBTDGC_*`, not hand-written; their consumers reference the generated names; the calibrant consts remain in the manifest; Rust builds deny-warnings clean and the fast suite is green.

**[260527-1034] rough**

## Character

Mechanical projection over a single-homed group; mirrors the colophon shape against a different bash owner.

## Goal

Generate the rbtd fixture-name consts from their canonical bash owner via the shared primitive and migrate consumers off the hand-mirror in `rbtdrm_manifest.rs`.

## Shape

- Fixture names are canonically authored in `Tools/rbk/rbtd/rbte_engine.sh` and hand-copied into `rbtdrm_manifest.rs`. Single home exists — projectable without a tidy step.
- Feed the names through the shared primitive into the `RBTDGC_` file (const name `RBTDGC_` + fixture stem); migrate manifest consumers and any inline fixture literals onto the generated consts.
- Keep the runtime drift check alive until the diff-gate lands.

## Sources

- `Tools/rbk/rbtd/rbte_engine.sh` — fixture-name canonical author
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — fixture hand-mirror

## What done looks like

Fixture consts are generated `RBTDGC_*`, not hand-written; consumers reference the generated names; Rust builds and the fast suite is green.

**[260520-2034] rough**

## Character

Mechanical projection over a single-homed group; mirrors the colophon pace's shape against a different bash owner.

## Goal

Generate the rbtd fixture-name consts from their canonical bash owner via the generic emitter and migrate consumers off the hand-mirror in `rbtdrm_manifest.rs`.

## Shape

- Fixture names are canonically authored in `Tools/rbk/rbtd/rbte_engine.sh` and hand-copied into `rbtdrm_manifest.rs`. Single home exists — projectable without a tidy step.
- Feed the names through the generic emitter as name/value pairs to a checked-in generated file; migrate manifest consumers and any inline fixture literals onto the generated consts.
- Keep the runtime drift check alive until the diff-gate lands (same handover discipline as the colophon pace).

## Sources

- `Tools/rbk/rbtd/rbte_engine.sh` — fixture-name canonical author
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — fixture hand-mirror

## What done looks like

- Fixture consts are generated, not hand-written; consumers reference the generated names; Rust builds and the fast suite is green.

### verb-container-project (₢BPAAH) [complete]

**[260529-0627] complete**

## Character

Mechanical projection over two now-single-homed groups; the bash-home prerequisite landed in the chivvied front pace.

## Goal

Project operation verbs and container roles to generated `RBTDGC_*` consts via the shared primitive, retiring their hand-mirror in `rbtdrm_manifest.rs`.

## Shape

- Assumes operation verbs and container roles each have one canonical bash declaration — landed by the front bash-home pace, now in `rbcc_Constants.sh` (`RBCC_verb_*`, `RBCC_container_*`).
- Feed them through the shared primitive into the `RBTDGC_` file (const name `RBTDGC_` + stem); migrate the manifest consumers onto the generated consts.
- Keep the runtime drift check alive until the diff-gate lands.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — canonical bash home for operation verbs (`RBCC_verb_*`) and container roles (`RBCC_container_*`)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — operation/container hand-mirror

## What done looks like

Operation-verb and container-role consts are generated `RBTDGC_*` from their bash home; the manifest hand-mirror is retired for these groups; Rust builds and the fast suite is green.

**[260529-0621] rough**

## Character

Mechanical projection over two now-single-homed groups; the bash-home prerequisite landed in the chivvied front pace.

## Goal

Project operation verbs and container roles to generated `RBTDGC_*` consts via the shared primitive, retiring their hand-mirror in `rbtdrm_manifest.rs`.

## Shape

- Assumes operation verbs and container roles each have one canonical bash declaration — landed by the front bash-home pace, now in `rbcc_Constants.sh` (`RBCC_verb_*`, `RBCC_container_*`).
- Feed them through the shared primitive into the `RBTDGC_` file (const name `RBTDGC_` + stem); migrate the manifest consumers onto the generated consts.
- Keep the runtime drift check alive until the diff-gate lands.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — canonical bash home for operation verbs (`RBCC_verb_*`) and container roles (`RBCC_container_*`)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — operation/container hand-mirror

## What done looks like

Operation-verb and container-role consts are generated `RBTDGC_*` from their bash home; the manifest hand-mirror is retired for these groups; Rust builds and the fast suite is green.

**[260527-1034] rough**

## Character

Mechanical projection over two now-single-homed groups; the bash-home prerequisite landed in the chivvied front pace.

## Goal

Project operation verbs and container roles to generated `RBTDGC_*` consts via the shared primitive, retiring their hand-mirror in `rbtdrm_manifest.rs`.

## Shape

- Assumes operation verbs and container roles each have one canonical bash declaration — landed by the front bash-home pace, now in `rbcc_Constants.sh` (`RBCC_verb_*`, `RBCC_container_*`).
- Feed them through the shared primitive into the `RBTDGC_` file (const name `RBTDGC_` + stem); migrate the manifest consumers onto the generated consts.
- Keep the runtime drift check alive until the diff-gate lands.

## Sources

- `Tools/rbk/rbcc_Constants.sh` — canonical bash home for operation verbs (`RBCC_verb_*`) and container roles (`RBCC_container_*`)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — operation/container hand-mirror

## What done looks like

Operation-verb and container-role consts are generated `RBTDGC_*` from their bash home; the manifest hand-mirror is retired for these groups; Rust builds and the fast suite is green.

**[260520-2035] rough**

## Character

Mechanical projection over two now-single-homed groups; the bash-home prerequisite landed in the chivvied front pace.

## Goal

Project operation verbs and container roles to Rust consts via the generic emitter, retiring their hand-mirror in `rbtdrm_manifest.rs`. Assumes their bash home already exists.

## Shape

- The bash-home step is done separately at the front of the heat. This pace assumes operation verbs and container roles each have one canonical bash declaration.
- Feed them through the generic emitter as name/value pairs to the generated file; migrate the manifest consumers onto the generated consts.
- Keep the runtime drift check alive until the diff-gate lands.

## Sources

- the canonical bash home for operation verbs and container roles (from the front bash-home pace)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — operation/container hand-mirror

## What done looks like

- Operation-verb and container-role consts are generated from their bash home; the manifest hand-mirror is retired for these groups; Rust builds and the fast suite is green.

**[260520-2034] rough**

## Character

The one pace that does bash tidying before projecting — judgment lives in choosing the single home. Honors the heat's single-author rule rather than weakening it.

## Goal

Give operation verbs and container roles a single canonical bash home, then project them to Rust consts via the generic emitter, retiring their hand-mirror in `rbtdrm_manifest.rs`.

## Shape

- Operation verbs and container roles are co-maintained today but lack a single bash owner — operations are implicit in tabtarget/zipper definitions, container roles implicit across `rbob_*.sh`. The heat's locked rule forbids projecting a constant without one canonical author.
- First settle a single bash home for each group (a small refactor — the home, not new behavior), then feed them through the generic emitter like the other groups.
- The bash-home step is depot-free and emitter-independent; only the projection step needs the emitter pace.
- Migrate the manifest consumers onto the generated consts; keep the runtime drift check alive until the diff-gate lands.

## Sources

- `Tools/rbk/rbz_zipper.sh` — operation verbs (implicit in tabtarget defs)
- `Tools/rbk/rbob_*.sh` — container roles (implicit)
- `Tools/rbk/rbtd/src/rbtdrm_manifest.rs` — operation/container hand-mirror

## What done looks like

- Operation verbs and container roles each have one canonical bash home; their consts are generated from it; the manifest hand-mirror is retired for these groups; Rust builds and the fast suite is green.

### codegen-build-wiring-and-diff-gate (₢BPAAF) [complete]

**[260529-0642] complete**

## Character

Touches qualify infrastructure and build ordering; subtle — the regenerate-vs-consume distinction is load-bearing. Closes out the drift-protection handover for the whole projected set.

## Goal

Wire generation into the build lifecycle so every generated const file is regenerated at the BUK-available layer before each Rust build, never regenerated inside the container, and a stale commit fails qualify loud — then retire the runtime drift check the gate replaces.

## Shape

- Regenerate all generated files (colophons, RBCC set, scattered groups) before every Rust build, at the layer where BUK is available.
- A build inside the minimal container consumes the committed artifacts and never regenerates (no BUK present there).
- Add a `git diff --exit-code` qualify gate so a stale committed artifact fails loud.
- Retire `rbtdrm_verify` here, once the gate is live — exact equality supersedes its drift role. Retiring verify likely strands `rbtdrm_required_colophons` (its only consumer); remove that map too if nothing else reads it.

## Sources

- `Tools/rbk/rbtd/rbtw_workbench.sh` and the rbtd build tabtargets — Rust build entry
- `tt/rbw-tf.QualifyFast.sh`, `tt/rbw-tr.QualifyRelease.sh`, `tt/rbw-tP.QualifyPristine.sh` — qualify gates
- Any existing diff-gate around the `buz_emit_context`-generated markdown context — precedent for the regeneration wiring

## What done looks like

- Every generated artifact is regenerated pre-build outside the container; the container build consumes the commit; qualify fails on a deliberately-staled artifact; `rbtdrm_verify` is gone.

**[260529-1254] rough**

## Character

Touches qualify infrastructure and build ordering; subtle — the regenerate-vs-consume distinction is load-bearing. Closes out the drift-protection handover for the whole projected set.

## Goal

Wire generation into the build lifecycle so every generated const file is regenerated at the BUK-available layer before each Rust build, never regenerated inside the container, and a stale commit fails qualify loud — then retire the runtime drift check the gate replaces.

## Shape

- Regenerate all generated files (colophons, RBCC set, scattered groups) before every Rust build, at the layer where BUK is available.
- A build inside the minimal container consumes the committed artifacts and never regenerates (no BUK present there).
- Add a `git diff --exit-code` qualify gate so a stale committed artifact fails loud.
- Retire `rbtdrm_verify` here, once the gate is live — exact equality supersedes its drift role. Retiring verify likely strands `rbtdrm_required_colophons` (its only consumer); remove that map too if nothing else reads it.

## Sources

- `Tools/rbk/rbtd/rbtw_workbench.sh` and the rbtd build tabtargets — Rust build entry
- `tt/rbw-tf.QualifyFast.sh`, `tt/rbw-tr.QualifyRelease.sh`, `tt/rbw-tP.QualifyPristine.sh` — qualify gates
- Any existing diff-gate around the `buz_emit_context`-generated markdown context — precedent for the regeneration wiring

## What done looks like

- Every generated artifact is regenerated pre-build outside the container; the container build consumes the commit; qualify fails on a deliberately-staled artifact; `rbtdrm_verify` is gone.

**[260520-2033] rough**

## Character

Touches qualify infrastructure and build ordering; subtle — the regenerate-vs-consume distinction is load-bearing. Closes out the drift-protection handover for the whole projected set.

## Goal

Wire generation into the build lifecycle so every generated const file is regenerated at the BUK-available layer before each Rust build, never regenerated inside the container, and a stale commit fails qualify loud — then retire the runtime drift check the gate replaces.

## Shape

- Regenerate all generated files (colophons, RBCC set, fixtures, scattered groups) before every Rust build, at the layer where BUK is available.
- A build inside the minimal container consumes the committed artifacts and never regenerates (no BUK present there).
- Add a `git diff --exit-code` qualify gate so a stale committed artifact fails loud.
- Retire `rbtdrm_verify` here, once the gate is live — exact equality supersedes its drift role. Retiring verify likely strands `rbtdrm_required_colophons` (its only consumer); remove that map too if nothing else reads it.

## Sources

- `Tools/rbk/rbtd/rbtw_workbench.sh` and the rbtd build tabtargets — Rust build entry
- `tt/rbw-tf.QualifyFast.sh`, `tt/rbw-tr.QualifyRelease.sh`, `tt/rbw-tP.QualifyPristine.sh` — qualify gates
- Any existing diff-gate around the `buz_emit_context`-generated markdown context — precedent for the regeneration wiring

## What done looks like

- Every generated artifact is regenerated pre-build outside the container; the container build consumes the commit; qualify fails on a deliberately-staled artifact; `rbtdrm_verify` is gone.

**[260520-1743] rough**

## Character

Touches qualify infrastructure and build ordering; subtle — the regenerate-vs-consume distinction is load-bearing. Closes out the drift-protection handover.

## Goal

Wire generation into the build lifecycle so the committed artifact is regenerated at the BUK-available layer before every Rust build, never regenerated inside the container, and a stale commit fails qualify loud — then retire the runtime drift check the gate replaces.

## Shape

- Regenerate the generated file(s) before every Rust build, at the layer where BUK is available.
- A build inside the minimal container consumes the committed artifact and never regenerates (no BUK present there).
- Add a `git diff --exit-code` qualify gate so a stale committed artifact fails loud.
- Retire `rbtdrm_verify` here, once the gate is live — exact equality supersedes its drift role. Retiring verify likely strands `rbtdrm_required_colophons` (its only consumer); remove that map too if nothing else reads it.

## Sources

- `Tools/rbk/rbtd/rbtw_workbench.sh` and the rbtd build tabtargets — Rust build entry
- `tt/rbw-tf.QualifyFast.sh`, `tt/rbw-tr.QualifyRelease.sh`, `tt/rbw-tP.QualifyPristine.sh` — qualify gates
- Any existing diff-gate around the `buz_emit_context`-generated markdown context — precedent for the regeneration wiring

## What done looks like

- The generated artifact is regenerated pre-build outside the container; the container build consumes the commit; qualify fails on a deliberately-staled artifact; `rbtdrm_verify` is gone.

**[260520-1738] rough**

## Character

Touches qualify infrastructure and build ordering; subtle — the regenerate-vs-consume distinction is load-bearing.

## Goal

Wire generation into the build lifecycle so the committed artifact is regenerated at the BUK-available layer before every Rust build, never regenerated inside the container, and a stale commit fails qualify loud.

## Shape

- Regenerate the generated file(s) before every Rust build, at the layer where BUK is available.
- A build inside the minimal container consumes the committed artifact and never regenerates (no BUK present there).
- Add a `git diff --exit-code` qualify gate so a stale committed artifact fails loud. This supersedes the drift role that `rbtdrm_verify` formerly held, with exact equality.

## Sources

- `Tools/rbk/rbtd/rbtw_workbench.sh` and the rbtd build tabtargets — Rust build entry
- `tt/rbw-tf.QualifyFast.sh`, `tt/rbw-tr.QualifyRelease.sh`, `tt/rbw-tP.QualifyPristine.sh` — qualify gates
- Any existing diff-gate around the `buz_emit_context`-generated markdown context — precedent for the regeneration wiring

## What done looks like

- The generated artifact is regenerated pre-build outside the container; the container build consumes the commit; qualify fails on a deliberately-staled artifact.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 I scattered-const-bash-home
  2 A ifrit-retire-claude-code
  3 C buz-rust-const-emitter
  4 D colophon-codegen-projection
  5 E rbcc-tinder-codegen-projection
  6 H verb-container-project
  7 F codegen-build-wiring-and-diff-gate

IACDEHF
···xxxx rbtdrm_manifest.rs
·xx·x·x rbz_zipper.sh
x·x·xx· rbcc_Constants.sh
···xx·x rbtdtm_manifest.rs
···xxx· rbtdro_onboarding.rs
··x·xx· rbtdgc_consts.rs
···x··x main.rs
···xx·· rbtdrc_crucible.rs, rbtdrd_dogfight.rs, rbtdrf_fast.rs, rbtdrk_canonical.rs, rbtdrp_pristine.rs
··x···x rbte_engine.sh
··x·x·· buz_zipper.sh, lib.rs
·xx···· rbk-claude-tabtarget-context.md
xx····· rbob_bottle.sh
····x·· rbtdtk_canonical.rs, rbtdtp_pristine.rs
···x··· rbtdrf_handbook.rs, rbtdri_invocation.rs, rbtdti_invocation.rs
··x···· rblm_cli.sh, rbq_Qualify.sh, rbte_cli.sh, rbw-MG.MarshalGenerate.sh
·x····· RBSIP-ifrit_pentester.adoc, rbw-Ic.IfritClient.moriah.sh, rbw-Ic.IfritClient.tadmor.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 37 commits)

  1 A ifrit-retire-claude-code
  2 I scattered-const-bash-home
  3 C buz-rust-const-emitter
  4 D colophon-codegen-projection
  5 E rbcc-tinder-codegen-projection
  6 H verb-container-project
  7 F codegen-build-wiring-and-diff-gate

123456789abcdefghijklmnopqrstuvwxyz
·····xx····························  A  2c
················xx·················  I  2c
····················xx·············  C  2c
······················xx···········  D  2c
························xx·········  E  2c
·······························xx··  H  2c
·································xx  F  2c
```

## Steeplechase

### 2026-05-29 06:42 - ₢BPAAF - W

Retired the rbtdrm_verify runtime colophon drift check, superseded by const projection + the build-time diff gate that landed in the prior projection paces. The required-colophon map now references the generated RBTDGC_* consts directly (dropped/renamed colophon breaks compilation) and rbq's regenerate-and-diff freshness gate catches staleness — together superseding the runtime check. Deleted rbtdrm_verify and both main.rs call sites; full-excised the now-dead manifest CLI arg from both runners (arg indices shifted, usage + stale diagnostic updated) and removed the bash plumbing that fed it (ZRBZ_COLOPHON_MANIFEST dropped from all three rbte_engine invocations, definition deleted from rbz_zipper). rbtdrm_required_colophons retained — rbtdtl_calibrant still asserts calibrant fixtures declare empty colophons; only the 17 verify-specific tests in rbtdtm_manifest.rs removed. The docket's other three Shape items (pre-build regeneration, container-consumes-commit, diff gate) were already live; this pace was solely the verify retirement the gate enabled. Verified: rbw-tb clean, rbw-tt 100 tests, rbw-tq fast qualify green, rbw-ts fast suite 5 fixtures green. Closes the constant-projection thread and the heat (7/7).

### 2026-05-29 06:39 - ₢BPAAF - n

Retire the rbtdrm_verify runtime colophon drift check, superseded by const projection + the build-time diff gate. The required-colophon manifest now references the generated RBTDGC_* consts directly, so a dropped or renamed colophon breaks compilation; rbq's regenerate-and-diff freshness gate (rbq_qualify_rust_consts/_context) catches staleness — together these supersede the runtime 'is this colophon in the live manifest string' check. Deleted rbtdrm_verify and both main.rs call sites; the now-dead manifest CLI arg is excised from both the suite and single runners (arg indices shifted, usage strings and the stale 'manifest verification accepted the name' diagnostic updated), and the bash plumbing that fed it removed (the ZRBZ_COLOPHON_MANIFEST positional dropped from all three rbte_engine invocations, its definition deleted from rbz_zipper). rbtdrm_required_colophons is retained — rbtdtl_calibrant still asserts calibrant fixtures declare empty colophons; only the 17 verify-specific tests in rbtdtm_manifest.rs are removed (vessels-dir pin test survives, file recommented). The build wiring and diff gate the docket also named were already live from the prior projection paces; this pace is solely the verify retirement that the gate enabled. Verified: rbw-tb deny-warnings clean, rbw-tt 100 unit tests (was 117, minus the 17 retired verify tests), rbw-tq fast qualify green (freshness gates + shellcheck 181 files).

### 2026-05-29 06:27 - ₢BPAAH - W

Projected operation verbs and container roles into generated RBTDGC_VERB_*/RBTDGC_CONTAINER_* consts from their canonical bash home (rbcc_Constants.sh), via the shared buz_emit_const primitive. Retired the hand-written RBTDRM_OPERATION_*/RBTDRM_CONTAINER_* mirror in rbtdrm_manifest.rs; rbtdro_onboarding.rs (sole consumer) repointed onto the generated consts with the OPERATION->VERB name shift. Full bash groups projected (8 verbs, 3 containers), not just the manifest subset, per the loop-its-own-data locked decision. rbtdrm_verify colophon drift check left live until the diff-gate pace. Verified: rbw-tb deny-warnings clean, rbw-tt 117 tests, rbw-tq fast qualify (freshness gate + shellcheck 181 files), rbw-ts fast suite all 5 fixtures green. Completes the constant-projection thread (colophons, rbcc set, verbs+containers).

### 2026-05-29 06:25 - ₢BPAAH - n

Project operation verbs and container roles into generated RBTDGC_* consts from their canonical bash home in rbcc_Constants.sh, retiring the hand-written RBTDRM_OPERATION_*/RBTDRM_CONTAINER_* mirror in rbtdrm_manifest.rs. Extended rbcc_emit_consts to loop the eight RBCC_verb_* and three RBCC_container_* names through the shared buz_emit_const primitive (stem-verbatim, uppercased) producing RBTDGC_VERB_* and RBTDGC_CONTAINER_*. The five RBTDRM_OPERATION_* and two RBTDRM_CONTAINER_* consts are deleted; rbtdro_onboarding.rs (the sole consumer) repoints all imports and usages onto the generated consts, with the OPERATION->VERB name shift applied at every site. rbtdrm_verify (the colophon runtime drift check) is untouched, staying live until the diff-gate pace. Verified: rbw-tb deny-warnings clean, rbw-tt 117 unit tests pass, rbw-tq fast qualify green (consts freshness gate + shellcheck 181 files).

### 2026-05-29 06:21 - Heat - T

verb-container-project

### 2026-05-28 10:32 - Heat - T

test-vocab-into-theurge

### 2026-05-28 10:27 - Heat - d

paddock curried: premise correction: fixture names are Rust-authored, out of bash to Rust projection; suites relocate into theurge

### 2026-05-28 09:10 - Heat - n

Clarify the gazette wire-format lede rule after a reslate misfire. The lede is now stated as exactly one whitespace-free token (silks/coronet/firemark), uniform across every input slug, with nothing following it on the # line — appending extra text folds it into the identity and fails validation. Replaces the prior ambiguous 'slug and lede' framing that, alongside the jjezs_slate <silks> example, primed appending a pace silks after the coronet on a jjezs_reslate notice.

### 2026-05-29 12:54 - Heat - T

fixture-codegen-projection

### 2026-05-27 18:23 - ₢BPAAE - W

Projected the rbcc single-homed constant set into generated RBTDGC_* consts. New rbcc_emit_consts loops the set (moorings/vessels dirs, 6 credential roles incl. assay/mason, 9 .env filenames) through the shared buz_emit_const primitive, uppercasing each stem mechanically (bash stays mixed-case, Rust SCREAMING). The file banner moved out of buz_emit_colophon_consts (now body-only) into rbz_emit_consts, which became the single combiner emitting one banner + colophon section + RBCC section — shared by the theurge build and rbq's freshness gate so they never diverge. Retired lib.rs rbtd_moorings_dir! macro + RBTD_MOORINGS_DIR const, the rbtdrm RBTDRM_ROLE_* mirror, and the scattered RBTDRK_*/RBTDRP_* .env-literal locals; all six consumer files repoint directly to RBTDGC_* with no aliases (per operator steer — only RBTDRP_RBRV_FILE survives as a genuine local, no rbcc home). Hardcoded "assay" literals in rbtdrk/rbtdrp now use RBTDGC_ROLE_ASSAY. The surviving rbtd_vessels_dir! macro (one compile-time literal for the Rust-local rbev-* concat sites — concat! can't consume a const without a new dependency, and the codebase's established zero-dep idiom is the literal-producing macro) is test-pinned to RBTDGC_MOORINGS_DIR/VESSELS_SUBDIR by rbtdtm_vessels_dir_matches_generated, so any drift from the bash source is a test failure. Verified: rbw-tb deny-warnings clean, rbw-tt 117 unit tests pass, rbw-tq fast qualify green (consts freshness gate + shellcheck 181 files), rbw-ts fast suite 112/112 across 5 fixtures.

### 2026-05-27 18:22 - ₢BPAAE - n

Project the rbcc single-homed constant set (moorings/vessels dirs, credential roles, .env filenames) into generated RBTDGC_* consts via the shared buz_emit_const primitive. New rbcc_emit_consts loops the set through buz_emit_const, uppercasing each stem (bash stays mixed-case, Rust SCREAMING); the file banner moved out of buz_emit_colophon_consts into rbz_emit_consts, which is now the single combiner (one banner + colophon section + RBCC section) shared by the theurge build and rbq's freshness gate. Retired lib.rs rbtd_moorings_dir! macro + RBTD_MOORINGS_DIR const, the rbtdrm RBTDRM_ROLE_* mirror, and the scattered RBTDRK_*/RBTDRP_* .env-literal locals; all six consumer files repoint directly to RBTDGC_* with no aliases (only RBTDRP_RBRV_FILE survives — no rbcc home). Hardcoded "assay" literals in rbtdrk/rbtdrp now use RBTDGC_ROLE_ASSAY. The surviving rbtd_vessels_dir! macro (one compile-time literal for the Rust-local rbev-* concat sites) is test-pinned to RBTDGC_MOORINGS_DIR/VESSELS_SUBDIR by rbtdtm_vessels_dir_matches_generated. Verified: rbw-tb deny-warnings clean, rbw-tt 117 unit tests pass, rbw-tq fast qualify green (consts freshness gate + shellcheck 181 files).

### 2026-05-27 11:41 - ₢BPAAD - W

Retired the hand-written RBTDRM_COLOPHON_* mirror; repointed all theurge consumers onto the build-generated RBTDGC_* colophon consts. Deleted 59 colophon consts from rbtdrm_manifest.rs; rbtdrm_required_colophons and rbtdrm_credential_check_colophon now reference RBTDGC_*; rbtdrm_verify retained (runtime drift check stays live until the diff-gate landing). 164 consumer references across 11 files repointed, imports split so colophon names come from rbtdgc_consts while role/fixture/operation/container/module names stay in rbtdrm_manifest. Per operator direction symbolization is maximal: inline invocation literals AND runtime diagnostic-prose Fail() messages (rbtdrf_fast.rs 10+2, rbtdrp_pristine.rs 3) now format!-interpolate the consts; rbtdti_invocation.rs test fixtures interpolate bark/writ consts into filenames+search-args, near-miss expressed as format!("{}b...", RBTDGC_CRUCIBLE_BARK). Only doc/code comments (cannot interpolate) and fixture-name imprints (tadmor/srjcl/testplate, project in a later pace) remain literal. Verified: rbw-tb deny-warnings clean; rbw-tt 116 unit tests pass including rbtdtm_manifest verify-against-generated; fast suite green (47+32+9+15+9), including rbtdrf_rs_unmake_empty_arg_refusal exercising the converted diagnostic prose at runtime. Code committed 8603ea29b (size_limit 95000). Loose end noted for operator: doc/code comments still name colophons literally — symbolizing them needs rewording, candidate for its own pace.

### 2026-05-27 11:39 - ₢BPAAD - n

Retire the hand-written RBTDRM_COLOPHON_* mirror; repoint all theurge consumers onto the build-generated RBTDGC_* colophon consts. Deleted 59 colophon consts from rbtdrm_manifest.rs; rbtdrm_required_colophons and rbtdrm_credential_check_colophon now reference RBTDGC_*; rbtdrm_verify retained (runtime drift check stays live until the diff-gate landing). 164 consumer references across 11 files repointed, with imports split so colophon names come from rbtdgc_consts while role/fixture/operation/container/module names stay in rbtdrm_manifest. Per operator direction, symbolization is maximal: inline invocation literals AND runtime diagnostic-prose Fail() messages in rbtdrf_fast.rs (10 invocation + 2 prose) and rbtdrp_pristine.rs (3 prose) now format!-interpolate the consts; rbtdti_invocation.rs test fixtures interpolate the bark/writ consts into filenames and search-args, with the near-miss expressed as format!("{}b...", RBTDGC_CRUCIBLE_BARK). Only doc/code comments and fixture-name imprints (tadmor/srjcl/testplate) remain literal — comments cannot interpolate consts; fixture names project in a later pace. Verified: rbw-tb builds deny-warnings clean; rbw-tt 116 unit tests pass including rbtdtm_manifest verify-against-generated and all rewritten rbtdti_invocation tests.

### 2026-05-27 11:19 - ₢BPAAC - W

Bash→Rust const-emit primitive + colophons as first feeder, landing in generated RBTDGC_ file. buz: buz_emit_const (RBK-ignorant one-line primitive) + buz_emit_colophon_consts (generic walker over the new co-indexed varname roll). rbz: rbz_emit_consts binds the RBTDGC_/RBZ_ projection (single site), rbz_generate_consts + rbz_generate_context are write-on-change producers. rbcc: RBCC_rbtdgc_consts_file + RBCC_tabtarget_context_file (sole shared paths). Architecture evolved from a standalone generate command to build-is-sole-producer: zrbte_codegen runs before every cargo invocation (build/test/build_binary), write-on-change so unchanged files keep mtime (no Cargo churn). Retired rbw-MG entirely — rblm_generate, the colophon enrollment, the tabtarget, and stale doc_env removed; rbq's two gates (context + consts) verify committed freshness and point users at rbw-tb. lib.rs gains pub mod rbtdgc_consts (107 colophon consts, deny-warnings clean). Consumers stay on RBTDRM_COLOPHON_* — migration and removing rbte's legacy ZRBZ_COLOPHON_MANIFEST runtime drift check are the next pace. Verified: bash -n x7, rbw-tb regenerates+compiles, write-on-change mtime-stable, gate passes fresh/trips on drift, rbw-tq both gates + colophon qualification, rbw-tt 116 tests, buw-st 36/36.

### 2026-05-27 11:19 - ₢BPAAC - n

Fold tabtarget codegen into theurge build; add colophon→Rust const projection

### 2026-05-27 10:33 - Heat - d

paddock curried: groom: lock RBTDGC_ landing-file/prefix; supersede the no-generic-emitter bullet with shared per-pair primitive + mechanical stem mapping

### 2026-05-27 10:05 - Heat - d

paddock curried: revise emitter decision: no pre-built generic emitter; each owner emits its own consts (bash 3.2 plumbing reality)

### 2026-05-27 09:46 - ₢BPAAI - W

Gave the two scattered constant groups a single canonical bash home in rbcc. Operation verbs: extended RBCC_verb_* with an alphabetized image/build-lifecycle sub-block (enshrine/inscribe/kludge/ordain/yoke) alongside the re-sorted SA-management verbs (divest/invest/roster), two commented sub-blocks preserving the SA-management scoping. Container roles: new RBCC_container_* family (bottle/pentacle/sentry, alphabetized), kept distinct from credential RBCC_role_* for word monosemy. Verify-against-live-files corrected the docket: crucible is sentry+pentacle+bottle, three not two. Wired rbob_bottle.sh container-name derivations and the charged-predicate service loop to the new constants (behavior-identical; rbob already sources rbcc). Verbs are declare-only here per docket intent; consumption is the later projection pace. bash -n clean both files; fast suite 112/112 green.

### 2026-05-27 09:45 - ₢BPAAI - n

Give the two scattered constant groups a single canonical bash home in rbcc. Operation verbs: extend RBCC_verb_* with the image/build-lifecycle sub-block (enshrine/inscribe/kludge/ordain/yoke) alongside the existing SA-management verbs, both sub-blocks alphabetized; previously these were implicit only in command-function names and tabtarget descriptions across rbfd_/rbfl_/rbfk_/rbob_. Container roles: new RBCC_container_* family (bottle/pentacle/sentry, alphabetized) — verified against rbob_bottle.sh that the crucible is sentry+pentacle+bottle, three not the docket's two; kept distinct from credential RBCC_role_* to preserve word monosemy. Wired rbob_bottle.sh container-name sites (ZRBOB_SENTRY/PENTACLE/BOTTLE and the charged-predicate service loop) to the new constants — behavior-identical, rbob already sources rbcc. bash -n clean both files.

### 2026-05-20 20:35 - Heat - S

scattered-const-bash-home

### 2026-05-20 20:34 - Heat - S

scattered-const-canonicalize

### 2026-05-20 20:34 - Heat - S

fixture-codegen-projection

### 2026-05-20 20:33 - Heat - d

paddock curried: groom: settle generic-emitter + full-set scope decisions

### 2026-05-20 17:38 - Heat - T

cross-language-constant-codegen

### 2026-05-20 17:38 - Heat - S

codegen-build-wiring-and-diff-gate

### 2026-05-20 17:37 - Heat - S

rbcc-tinder-codegen-projection

### 2026-05-20 17:37 - Heat - S

colophon-codegen-projection

### 2026-05-20 17:37 - Heat - S

buz-rust-const-emitter

### 2026-05-20 17:31 - ₢BPAAA - W

Retired Claude Code from the ifrit. Spec reframe (RBSIP): SYSTEM CONCEPT now describes a scripted adversary (rbid binary + python3 sortie adjutant), not an AI agent; dropped API-connectivity framing; rewrote Ifrit bottle package table to match Dockerfile.tether (no nodejs/npm, no scapy, no git); corrected nameplate network policy to test-target-only, naming Anthropic egress as ccyolo's property. Drift decision: scapy/git leave the spec rather than return to Dockerfiles — git's only justification was Claude Code; scapy's raw-packet capability is owned by the rbid binary (cap_net_raw+ep), so re-adding it would reintroduce the redundancy this heat removes. Mechanical removal of rbw-Ic IfritClient: deleted rbob_ifrit_client function, zipper enrollment, both per-nameplate tabtargets, regenerated tabtarget context. Verified: clean live-tree sweep, rbw-tf QualifyFast passed (incl. context-freshness).

### 2026-05-20 17:30 - ₢BPAAA - n

Retire Claude Code from the ifrit: reframe RBSIP SYSTEM CONCEPT to scripted-adversary (rbid binary + python3 sortie adjutant), rewrite Ifrit bottle package table to match Dockerfile.tether (drop nodejs/npm, scapy, git), correct nameplate network policy (test target only; Anthropic egress is ccyolo's property). Rip rbw-Ic IfritClient: remove rbob_ifrit_client function, zipper enrollment, both per-nameplate tabtargets, and regenerate tabtarget context.

### 2026-05-20 13:12 - Heat - f

racing

### 2026-05-20 10:19 - Heat - d

paddock curried: strip pace descriptions — paddock carries shape + locked constraints only

### 2026-05-20 10:19 - Heat - S

cross-language-constant-codegen

### 2026-05-20 10:18 - Heat - S

ifrit-retire-claude-code

### 2026-05-20 10:16 - Heat - d

paddock curried: seed paddock — redundancy-cleanup grab-bag, ifrit + codegen strands, gated on BK

### 2026-05-20 10:15 - Heat - f

stabled

### 2026-05-20 10:15 - Heat - N

rbk-13-mvp-redundancy-cleanup

