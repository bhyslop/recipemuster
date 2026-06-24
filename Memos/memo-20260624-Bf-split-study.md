# Cross-heat split study — partition + decisions (₢BfAAk)

**Provenance memo.** Records the workflow-driven cross-heat parallelization study run 2026-06-24 over heats ₣Bf / ₣Bi / ₣Bl, the partition it produced, the three operator-deferred calls and their resolutions, and the execution it authorized. The durable shape lives in the heats' paddocks after restitch; this memo is the *how-we-got-there*, not authority.

## What the study did

The ₣Bf heat-integrity audit found that **file contention on a few shared spine files**, not dependency-chain depth, is the binding constraint on running heats in parallel across separate repo clones. This study tested that and acted on it. Two workflows: (A) footprinted all 39 remaining paces of ₣Bf/₣Bi/₣Bl (footprint → adversarial verify, 21 corrections) and aggregated them into a file-contention graph; (B) a 4-angle partition panel → synthesis → per-heat paddock-restitch → completeness critic. A third workflow deep-dived the three deferred calls. The study pace ₢BfAAk itself is excluded from the partition.

The contention concentrates on three spine files: `RBS0-SpecTop.adoc`, `rbtdrc_crucible.rs`, and the zipper trio (`rbz_zipper.sh` + regenerated `rbtdgc_consts.rs` + `claude-rbk-tabtarget-context.md`); plus the soft `claude-rbk-acronyms.md`.

## The partition (final — 16 / 16 / 7, decisions applied)

Three streams, each in its own repo clone, partitioned by file territory:

- **Stream-FED** — federation vertical (spec recast + Keycloak code + the genuinely ₣Bf-gated ₣Bi re-derivations). Anchor ₣Bf. Owns `RBS0-SpecTop.adoc` + `claude-rbk-acronyms.md`. **16 paces.**
- **Stream-THG** — theurge-crate refactor + zipper trio + foedus descry/instate code + cosmology spec. Anchor ₣Bl (relabelled + un-stabled). Owns `rbtdrc_crucible.rs` + the zipper trio. **16 paces.**
- **Stream-MVP** — RBS0-MVP spec + durable-leak drive-link + foundry/provenance cleanup. Anchor ₣Bi (narrowed). Owns no hot file; touches RBS0 (durable-leak + scry regions) and the zipper (`rbw-od`) through windows. **7 paces.**

**Chosen base:** dependency-honouring partition, grafted with two contention-purist moves (pull the lone crucible escapee and the colophon-relocation pace into THG so the crucible and zipper each have a single owning clone). ₣Bl reused as the third anchor — no fresh heat; its lone pace is superseded by a ₣Bf cosmology pace and wrapped.

### Pace assignment

- **FED (₣Bf, 16):** the federation spec recast, manor/org scrub, terrier finisher, foedus-reuse design, the Keycloak code arms (affiance mechanism-arm, orchestrator, accessor, realm, estate stub), the Entra guide, two isolated-clean paces — plus three ₣Bi federation-MVP re-derivations transferred in (account-state tolerance, OAuth fast-fail, payor-install next-step).
- **THG (₣Bl, 16):** the whole theurge-crate refactor cluster, the colophon-relocation pace, the cosmology pace, one ₣Bi crucible escapee transferred in, and ₣Bl's native (superseded) pace wrapped in place.
- **MVP (₣Bi, 7):** the two durable-leak spec paces (mint + cite), conjure-provenance design, Cloud Build spec repair, scry repair — plus the two re-routed cleanup paces (build-bucket scrub, clean-tree rationale) that stay put (see Decision 1).

## Hot-file serialization choreography

- **`RBS0-SpecTop.adoc` — FED owns.** 7 FED-internal writers (recast pace first, seating the foedus civic quoin + mechanism-discriminator), then three disjoint append-regions behind a single forward sync: MVP's durable-leak region (mint strictly before cite), MVP's scry region, THG's cosmology region. The 11th writer is THG's superseded pace — dropped, never writes RBS0.
- **`rbtdrc_crucible.rs` — THG owns.** All 8 writers in-clone (the escapee pulled in). The registry-relocation pace must bracket the cluster (first or last); one FED pace retiring two fixtures lands as a one-shot delta after the relocation settles.
- **Zipper trio — THG owns.** Single-writer baton THG → FED → MVP; each holder syncs, edits, rebuilds (`rbw-tb`), commits all three files together, pushes, hands off. (The Entra-guide colophon is a conditional FED baton step — only if the guide mints a colophon.)
- **`claude-rbk-acronyms.md` — FED owner-of-record, soft.** Append-distinct; union-on-conflict, never overwrite.
- **Soft non-spine cross-clone files** (name them, don't assume disjoint): `rbgp_payor.sh` (FED ↔ MVP after Decision 1), `rbcc_constants.sh` (MVP ↔ THG), `rbrf_regime.sh` (FED ↔ THG). All auto-merge (non-adjacent hunks).

## Heat realization (executed this pass)

**Transfers (`jjx_transfer`):**
1. ₣Bf → ₣Bl (14): the theurge cluster + colophon-relocation + cosmology pace.
2. ₣Bi → ₣Bl (1): the crucible escapee.
3. ₣Bi → ₣Bf (3): the three genuinely-gated federation-MVP re-derivations (NOT the two re-routed by Decision 1).

**Relabels / alters:**
- ₣Bl un-stabled to racing.
- Silks: ₣Bf → `rbk-14-mvp-federation-build`; ₣Bi → `rbk-14-mvp-loose-ends` (unchanged); ₣Bl → `rbk-15-mvp-theurge-refactor`.
- The study pace stays in ₣Bf, excluded. ₣Bl's native pace stays in ₣Bl, wrapped/dropped there.

## The three deferred calls — resolutions

### Decision 1 — `₢BiAAH` / `₢BiAAI` routing: **both stay in MVP** (not transferred to FED)
Grounded in the real `rbgp_payor.sh`: the four writers edit disjoint functions hundreds of lines apart (build-bucket block ~1556–1651, the clean-tree call at 1138, the install tail at 994/1028, the terrier region at 2171+). `₢BiAAH`'s build bucket is even a different constant (`RBDC_GCS_BUCKET`, depot-grain) from the federation terrier bucket — no federation code reads it. So the contention is soft (auto-merge), and neither pace carries a hidden federation wrap-gate. They were folded into FED only for single-clone hygiene, which costs more (bloats the heaviest stream, starves idle MVP) than the near-zero merge it avoids. Confirming contrast: `₢BiAAJ` *does* have a real gate (its install-tail prose must point at the manor-finisher `₢BfAAF` creates) and correctly stays in FED. **Net: rebalances 18/16/5 → 16/16/7.**

### Decision 2 — Suite-rename drain: **split the block**
The landed five-suite rename (reveille/picket/bivouac/echelon + keepers) shipped 2026-06-24 — dispositioned, so it **drains to `Memos/memo-20260623-Bf-heat-memories.md`** with its asterism rationale. The `parley` reservation is *live* (an unbuilt federation authentic-verb fixture word) — draining it would bury a live idea, so it **stays in the FED paddock** (parley names the federation real-admission-verb fixture — federation territory, not the THG suite-ladder). Note: when `parley` graduates to a built suite, its `RBTDRC_SUITES` registration becomes a THG-territory edit — a cross-stream touch to schedule then.

### Decision 3 — `occurrencesViewer` premise: **drop the clause; premise is false**
`₢BiAAH`'s docket justified a capability-spec correction by claiming RBSRK omits a retriever `occurrencesViewer` grant. False on three counts: (1) **RBSRK was deleted** 2026-06-19 (commit `7e72d7f19`, cult-verb-estate demolition) — no spec to correct; (2) the live spec that homes the fact, `RBSDC-depot_recognosce.adoc:45`, already states the retriever's `roles/containeranalysis.occurrences.viewer` grant correctly — no drift; (3) the camelCase `occurrencesViewer` is a docket mis-rendering of a real, correctly-granted dotted role (the code uses the constant, which is why it greps clean). The role is real and the retriever genuinely needs it (reads SLSA/DSSE provenance during vouch) — nothing is broken. **Dropped the clause from `₢BiAAH`; kept its genuine build-bucket-vestige-scrub spine.** The premise traces to SLSA-provenance work (heat ₣Al / a ₣BU build-bucket docket), not the Keycloak prototype — a passing thought from a different neighbour.

**Side-finding acted on:** the 2026-06-19 demolition's repo-wide sweep skipped `claude-rbk-acronyms.md`, leaving **7 dangling pointers** to the deleted cult-triad specs (RBSDK/RBSRK/RBSDD/RBSRD/RBSDR/RBSRL/RBSGM). Excised this pass; the successor-prose mentions in the RBSPA/RBSPB/RBSPU entries are preserved.

## Residual risks (carried into mount)

- **`₢BiAAa`** (heaviest pace) reaches FED's RBS0 + THG's zipper from a non-owner clone — rail it last, behind all owner pushes.
- **`₢BfAAJ`** (Entra guide, low confidence) is the conditional zipper-baton step — decide at mount whether it mints a colophon.
- **`₢BfAAT`** placeholder subdoc names (RBSDD/RBSDI collide) gate the one surviving cross-stream spec-first window (THG's descry/instate build on FED's foedus subdocs) — resolve the tails in FED first.
- Several footprints (`₢BfAAK`, `₢BfAAA`, `₢BiAAA`, `₢BiAAB`) are provisional but resolve inside their stream — no re-assignment forced.

## Cross-stream gates — deferred to a coordination-pace pass

Operator decision: encode the cross-stream/cross-clone dependencies and the hot-file serialization windows as **dedicated gate-only coordination paces** (not by reslating merge-mechanics into substantive dockets — keeps dockets clean and the gate visible in heat order). These are added *after* this restring/reslate pass, once every pace sits in its proper heat. The gates to encode: the zipper baton (THG → FED → MVP), the RBS0 forward-sync (FED first, then MVP/THG disjoint regions), the crucible delta (FED's fixture-retire after THG's relocation), and the FED-internal spec-first ordering (recast first; foedus subdocs before descry/instate).
