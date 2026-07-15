# Transfer memo — ₢BcAAO pace-identity re-gestalt (station → cerebro)

**Provisional provenance, not authority.** This memo hands off a mounted-but-unstarted pace
across machines. Everything below is a *working map and set of proposals* from the slating
session — verify against the live tree at mount; the docket and the JJS* specs are the
authority. **Retire this memo when ₢BcAAO lands.**

- Pace: `₢BcAAO` (pace-identity-regestalt-impl), heat `₣Bc` (jjk-v4-0-isolated-schema-changes).
- Branch: `bhyslop-260715-BcAAO-pace-identity-regestalt`, based on `origin/main` @ `01cc55126`,
  main-tracking unset (a §F schema branch must never push to main until the coda).
- State at handoff: design fully mapped; **no source or spec bytes edited yet** — clean start.
- Standing: bridled opus, full ceremony (I mount, work, wrap myself — the mount, work, wrap
  is the executing session's, per the docket's standing).

## The one operator ruling that supersedes the docket

The docket (as originally written) homes the settlements in aspirant **JJSAS only**. The
operator ruled this session: **full JJS0 infusion now, and a full sweep of every affected
JJS\*.adoc** — the whole binding codex is made truthful about immutable-for-life coronets, not
just the aspirant sheaf. The reslate folds this in; this memo records the reasoning: landing
live code that makes JJS0's binding coronet definition false (its Reassignment paragraph says
relocate re-keys — the exact opposite of the new behavior) is the divergence this heat's whole
discipline exists to prevent, and the operator wants the codex kept honest.

## Settled design (from the docket cinches + this session)

- **Immutable-for-life pace ids from one global seed.** A coronet is minted once, from a single
  global seed on the gallops (under the commit lock), and never re-keyed. `restring`/`transfer`/
  `relocate` re-affiliate the pace to a new heat by *moving* it, never by minting a new id.
- **Grandfather verbatim.** Existing coronets keep their exact 5-char heat-embedded form as
  their immutable id; the embedded heat chars become frozen history, not live affiliation.
- **Founding value (operator ruling 260715):** the global seed founds at
  `max(highest grandfathered body + 1, CAAAA)`. All existing coronets lead with A/B (heats so
  far), so all decode below `CAAAA` (charset is url-safe: A=0, B=1, C=2), the floor dominates,
  and every new-era id visibly leads with C+. The `max()` keeps the one-comparison
  collision-freedom if legacy minting ever reached C-space. No reserved-set check.
- **Wire vs display.** Wire = the bare immutable id (`₢`-sigiled body, e.g. `₢CAAAB`) in every
  machine context: gallops keys, MCP params, git refs, billet branch names. Display =
  heat-qualified at emission only: `₢` + current-heat firemark body + interpunct `·` + immutable
  id (e.g. `₢Bc·CAAAB`; a grandfathered pace renders `₢Bc·BcAAO` — the heat appears twice, the
  embedded one being frozen history). Rendered from live affiliation, so transfer changes
  tomorrow's rendering, never the identity. Full-form never-abbreviate discipline carries over;
  firemark display untouched.
- **Ingest tolerance / halter typing.** Strip the glyph if present; a token containing the
  interpunct `·` is a qualified coronet — resolve the 5-char tail, ignore the qualifier;
  otherwise type by length exactly as today (2 = firemark, 5 = coronet). The `%`-sentinel
  pensum is untouched.

## Code map (entry points — verify live at mount; NO line numbers, they drift)

Crate root: `Tools/jjk/vov_veiled/src/` (source `jjr*`, test twins `jjt*`). A recurring
structural fact: **a coronet is stored as a `String` map-key in display form (with the `₢`
sigil), not a typed field**; the parent heat is *implicit* — whichever heat's `paces` map holds
the key. `jjrf_Coronet` (in `jjrf_favor.rs`) is a transient parse/encode newtype only.

1. **Minting** — `jjro_ops.rs` `jjrg_slate`: today `coronet = ₢ + firemark_body + heat.next_pace_seed`
   (per-heat 3-char seed), bumped by `jjru_util.rs` `zjjrg_increment_seed` (string-carry).
   **Change:** mint the whole 5-char id from a new *global* gallops seed; bump that.
2. **Types** — `jjrt_types.rs`: `jjrg_Gallops` holds `next_heat_seed` (`jjgrn_next_heat_seed`),
   `heat_order`, `heats` (BTreeMap keyed by `₣`-firemark), `retention_since` — **no global pace
   counter today**. `jjrg_Heat` holds `order` (Vec of coronet keys), `next_pace_seed`
   (`jjghn_next_pace_seed`, per-heat), `paces` (BTreeMap keyed by `₢`-coronet). `jjrg_Pace` = just
   `tacks`. **Change:** add `jjrg_Gallops.next_pace_seed` (`jjgrn_next_pace_seed`); retire
   `jjrg_Heat.next_pace_seed` (serde ignores it on old-store read; write-forward drops it).
3. **Alphabet / arithmetic** — `jjrf_favor.rs`: `JJRF_CHARSET` = url-safe
   `A-Za-z0-9-_` (standard order, A=0…). `jjrf_Coronet::jjrf_encode/decode/parent_firemark`,
   `JJRF_CORONET_LEN=5`. Two increment mechanisms coexist: string-carry `zjjrg_increment_seed`
   (live mint path) and arithmetic `jjrf_successor` (tests). Add the interpunct constant here.
4. **Re-affiliation** — `jjro_ops.rs` `jjrg_draft` (single move — **RE-KEYS today**: allocates a
   new coronet from the dest seed, drops the old key, no tombstone; also demotes bridled→rough)
   and `jjrg_restring` (bulk — loops `jjrg_draft`, collects `jjrg_RestringMapping` old→new). MCP
   routing in `jjrm_mcp.rs`: `jjx_relocate`→`jjrdr_run_draft`, `jjx_transfer`→`jjrrs_run`.
   **Change:** move the key (order + paces entry) to the dest heat under the SAME key; keep the
   bridle→rough revert (per the escalation rule); `RestringMapping` new == old.
5. **Display** — **no chokepoint exists.** Raw coronet keys are printed directly in
   `jjrgc_get_coronets.rs`, `jjrmt_mount.rs`, `jjrpd_parade.rs` (many sites), and the emblem
   paths in `jjrm_mcp.rs` (`zjjrm_normalize_identity`, `zjjrm_compose_emblem`).
   `jjrf_Coronet::jjrf_display` exists but is not on the listing surfaces. **Change:** introduce
   ONE qualified-display helper (`₢` + current-heat firemark body + `·` + coronet body) and route
   every raw-key print through it.
6. **Halter / ingest** — `jjrz_gazette.rs` `jjrz_parse_halter_input` returns raw ledes "typed
   downstream by length." Length-typing discriminators (all strip `₣`/`₢` then match
   `JJRF_FIREMARK_LEN`/`JJRF_CORONET_LEN`): `jjrmt_mount.rs` (orient), `jjrm_mcp.rs`
   (`zjjrm_normalize_identity`, `zjjrm_lede_firemark`, `zjjrm_resolve_emblem_marker`, the record
   designation gate), `jjrpd_parade.rs` (show). **Change:** interpunct-aware — a `·` token is a
   qualified coronet (5-char tail); lookup must stop inferring heat from the first 2 chars.
7. **Reprieve** — `jjri_io.rs`: `JJDZ_RIVET_REPRIEVE = "JJr_a7c"`, `ZJJDZ_REGISTRY` (3 live
   episodes: `V3→V4`, `schema_version drop`, `tack text→lines`), `zjjdz_Episode{label, is_live}`,
   `jjdz_probe` (read-only), `jjdz_write_forward`. Load funnels through `zjjdr_from_bytes`
   (probe → migration mode → write-forward). Nag: `jjrm_mcp.rs` `zjjrm_reprieve_nag`, emitted at
   open. Mirrored in `jjrvl_validate.rs`. `jjrt_v3_types.rs` = frozen V3 reference (do not edit).

**Cross-cutting invariant to relax:** `jjrv_validate.rs` `zjjrg_validate_pace` today *rejects* a
store where a pace key's suffix does not `starts_with(heat_id)` ("key must embed parent heat
identity"). Drop that rule; add cross-heat coronet uniqueness (a coronet appears in exactly one
heat's `paces`). Also audit every `jjrf_parent_firemark` / first-2-char heat inference in lookup
(`jjrg_resolve_pace` and the halter resolvers) → replace with a scan over heats' `paces` maps.

## Reprieve episode (register per JJSCRP; do NOT improvise — §E/§F/JJSCRP)

- **Registry entry** in `ZJJDZ_REGISTRY`. Proposed label: `pace-seed heat→global`.
- **is_live**: the global pace seed is absent (old store carries per-heat `jjghn_next_pace_seed`
  and no `jjgrn_next_pace_seed`). Cleanest: make the new field `Option`/empty-default and test
  emptiness, or byte-sniff — decide at implementation.
- **write-forward**: found `next_pace_seed = max(highest existing coronet body + 1, CAAAA)`
  computed over all pace keys across all heats (strip `₢`, decode 5-char body); drop per-heat
  seeds (serde omission on re-serialize). **Grandfathered keys are NOT rewritten** — they are
  already valid immutable ids; the write-forward only relocates the seed.
- Cite `JJr_a7c`; **never mint a per-episode rivet** (per-episode specifics are registry data).

## Delivery-sequencing hazard (resolve BEFORE running tt/vow-b)

§E/§F: a schema change is delivered source-only on the §F branch and **never commits a gallops
conversion**. But `tt/vow-b` builds AND installs the vvx binary; after install, any jjx
store-write (a notch, a wrap, an open with a convergence budget) run with the new-schema binary
would apply the write-forward and *persist the conversion* — the premature-conversion the
discipline forbids. Before running `tt/vow-b`: confirm the vvx exec/caching model (does the MCP
server hold the binary in-memory for the session, so install doesn't flip the live path until
the next session?), keep all jjx bookkeeping on the OLD binary (build LAST), and do not restart
the session between build and wrap. §H: **notch before every test.** This is the "do not
improvise this part" zone — read JJSCRP and CLAUDE.md §E/§F first.

## Spec scope — full infusion (all authority; verify sheaf list by re-grep at mount)

- **JJS0** (`JJS0_JobJockeySpec.adoc`): rewrite `jjdt_coronet` (drop the re-keying Reassignment
  paragraph; immutable-for-life, global seed, heat-qualified interpunct display); Types table row
  (structure "global index", capacity ~1.07B global, not "2 heat + 3 index / ~262K per heat"); the
  encode/decode algorithm section; the halter length rule (+ interpunct); add a `jjdgm_pace_seed`
  Gallops member near `jjdgm_seed` and retire `jjdhm_seed` (per-heat) to a "moved to gallops at
  the re-gestalt" note (pattern: `jjdhm_pensum_seed` / `jjdhm_paddock`); add the mapping-section
  attribute ref. The reprieve mechanism section is generic (registry = census), so the new episode
  rides it without a prose list edit.
- **JJSAS** (`JJSAS-state-repo.adoc`, aspirant): drain the "Pace identity re-gestalt" section to an
  INFUSED-→JJS0 pointer (sheaf convention); remove the wire-vs-display bullet from "Open forks".
  The revision-insignia sub-mint note stays (₣Br context — does NOT travel per the docket).
- **Sweep** every other JJS\* referencing coronet re-keying / heat-embedding / relocate. This
  session's grep hit (may drift — re-grep `coronet|re-key|reassign|relocate|embed.*firemark|
  next_pace_seed|pace index` over `JJS*.adoc`): JJSAB, JJSCCH, JJSCGC, JJSCDR, JJSCGZ, JJSCGS,
  JJSCMT, JJSCNC, JJSCPD, JJSCLD, JJSCRN, JJSCRS, JJSCRL, JJSCTL, JJSCRT, JJSRPS, JJSCSL, JJSCSC,
  JJSCWP, JJSRWP, JJSVC, JJSVD (plus JJS0/JJSAS above).

## Work breakdown (mirrors the slating session's task list — session-local, re-create if wanted)

1. JJS0 coronet normative rewrite (durable home). 2. Drain JJSAS + close its open fork.
3. Sweep the other JJS\*. 4. Schema: global seed + retire per-heat seed. 5. Reprieve episode.
6. Mint from global seed. 7. Re-affiliation without re-keying. 8. Lookup by scan + drop
embed-parent validation. 9. Qualified-display chokepoint + route sites. 10. Interpunct-aware
ingest. 11. jjt\* tests. 12. Build + test green (resolve the sequencing hazard first).

Do the specs first (durable-first, before code and before any provenance trims), commit as one
spec commit, then the code.

## Loose logistics

- Local `main` on the origin station carries two officium-invitatory markers diverged from
  origin/main — the operator's to reconcile; irrelevant to this branch.
- The origin station's officium for this session was `☉260715-1007-3nsb`; cerebro opens its own.
