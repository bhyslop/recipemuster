# Heat Trophy: rbk-01-mvp-review-loose-ends

**Firemark:** ₣Bi
**Created:** 260618
**Retired:** 260705
**Status:** retired

## Paddock

## Character

Junk-drawer of small pre-MVP loose ends —
the residual engineering that accreted under the release heats
once the release-qualification machinery shipped.
As the project approaches MVP the remaining issues trend small and self-contained;
this heat is their shared home,
with no pretense of a single goal beyond settling them before release.

What remains after the partition is a spec-and-cleanup remainder,
the lighter robustness band having departed with the federation re-derivations it belonged to.
One coherent sub-initiative anchors the rest:
the chained-fact / durable-config consolidation —
the two coarse chaining-role quoins minted into the spec top — rbch_enchase (the durable-config link) and rbch_palpate (the read-side consumer) —
and the unified nameplate-hallmark drive-link that cites them onto the durable-config surface.
Around it sit five genuinely independent cleanups —
a conjure-provenance design-only pass,
a mechanical two-reference Cloud Build spec repair,
a network-diagnostic (scry) repair,
a build-bucket-vestige scrub of the GCS bucket superseded by pouch-context delivery,
and a clean-tree-rationale gate parameter that gives each gate call site a locale-specific reason while BUG stays kit-agnostic.

The consolidation sub-initiative is not loose:
its two spec paces author the discipline laid out below,
and they share a citation order — the coarse quoins mint strictly before the drive-link cites them.
The five cleanups stand alone;
their order is loose priority, not a dependency chain.
The conjure-provenance pass is design-only and the lightest tail —
if the heat needs thinning rather than finishing, that is the pace to lift to an itch, not the scry or spec-repair work, which are genuine MVP cleanup that should land.

## Chaining-fact discipline — the shape the cluster cinched

One coherent sub-initiative inside the junk-drawer: the verbs that pass a value forward between tabtargets are split by role, and the split is load-bearing for safety.

Chain HEADS — the build and capture verbs (conjure, ensconce, conclave, underpin, immure, kludge, mirror) — only ever WRITE a fact; a head never reads a prior fact.
Conjure violated this by embedding the base-anchor election, which is being extracted into a standalone link (feoff) so conjure becomes pure output and builds only from committed config; that also closes a live provenance hole.

Chain LINKS read one chained value and write one durable config field.
Only four verbs write durable config from a resolved value — feoff (the base anchor), anoint (the graft image), yoke (the reliquary), and the nameplate-hallmark drive (the bottle/sentry hallmark) — and they are the only sanctioned members of the durable-config surface (durable-config, not durable-leak: "leak" names only the hazard the no-relay invariant eliminates, never the sanctioned mechanism).

The leak-elimination invariant is no-relay: the express-or-chain resolver is depth-1 and terminally consumed, never forwarded.
The git clean-tree gate is an ergonomic backstop only — never the safety mechanism.
Instance-binding (stamping a producing-vessel identity into the fact) was deliberately weighed and DROPPED; the safety is operator-trust plus loud-on-typecheck output plus the commit-review gate.

Read-side chain consumers (augur, summon, plumb, rekon) resolve the same express-or-chain fact but write no durable config; rekon was the last brought onto the band, leaving vouch the lone read consumer not yet chained.
A broken resolve now rejects with the named chaining band (BUBC_band_chain) — reused, NOT a new band: a read and a durable-config write never share one tabtarget, so 105 stays unambiguous per tabtarget without a second code.
This reverses the earlier loud-die cinch: the read/write distinction no longer lives in the exit code but in the EFFECT — a wrong read corrupts only a transient action, never a regime file.
They must still never be extended to write durable config without joining the durable-config surface and its no-relay discipline.

The read side carries two requirements the durable-config verbs' coverage does not vouch for it, and they need two different nets — one runtime, one static — because the exit code cannot tell them apart.
First, furnish: any CLI whose source closure reaches a buf_* fact caller must source buf_fact, or the helper is undefined and the verb dies command-not-found at the resolve.
This is furnish-what-you-LOAD, not just furnish-what-you-dispatch: the static check caught rbfh and rbfv, which only transitively load rbfcp_plumb through the rbfc0_core 0-trick.
Second, coverage: the reveille net is named-band exit-code cases (folio-less drives asserting the band — the resolve-logic net) PLUS a static furnish-invariant case (the transitive source-closure scan).
The furnish gap needs the static case because a command-not-found exits the SAME band 105 as a real broken chain, so a runtime exit-code case cannot distinguish it; an end-to-end cloud fixture (dogfight) proves the pull against real artifacts.
A coding error like the furnish gap is flushed out statically and NEVER given a named runtime band — naming command-not-found (127) would mix tiers, dressing a defect as a deliberate rejection.

This heat owns the spec paces that author the band-citing prose — the coarse quoins and the drive-link that cite the discipline into the durable-config surface.
Verification of the discipline, however, now lives in a sibling stream: it is theurge-homed — the theurge crate drives the real verbs through the full tabtarget exec path and asserts the named chain-rejection band, regime-poison the type specimen — and the theurge crate relocated out of this heat into the theurge-refactor stream.
That is a stream boundary, not an in-heat one: the spec work here and the band-asserting fixtures there must stay consistent on the band matrix, but they sit in different clones.
The band fires at both the durable-config LINKS (feoff/yoke via buc_reject) and the read consumers (summon/plumb/augur/rekon), all RBK; the BUK footing resolver and decoder still return a bare 1.
It is deliberately NOT homed in the BUK bash self-test: the band fires only at the RBK consumer, and the self-test sources no RBK — so it can prove the footing primitives' return-1 shape but never the band itself.
The footing primitives keep their existing BUK self-test cases; the band, wrong-kind, furnish-invariant, and fact-intact matrix is the theurge stream's.

## Provenance

Restrung from ₣BB and ₣BU.
₣BB built and shipped the release-qualification machinery — its goal is complete;
these paces were ordinary engineering that had collected under it, never release ceremony.
₣BU holds the actual release runs and the RELEASE.md revision,
parked stabled as paddock-memory until release time —
that work is not here and is not reopened by this heat.
Both source heats keep the git history of any dropped dockets.

A later cross-heat partition narrowed this heat further:
the federation-MVP re-derivations that had collected here (the credential-flap tolerance, the OAuth terminal fail-fast, and the payor-install next-step) restrung out into the federation-build stream, where they re-derive against the now-stable federation auth surface;
the read-side named-band crucible case restrung out into the theurge-refactor stream alongside the theurge crate it asserts against.
The heats those paces moved to keep the git history of their dockets.

## References

- ₣BB — shipped the release-qualification machinery; holds the design record of what was built
- ₣BU — the deferred release runs and doc revision (stabled until release time)

## Paces

### bind-digest-gate-vouch-budget (₢BiAAq) [complete]

**[260701-1503] complete**

## Character
Mechanical: a bind-gated regime-validation check plus a one-constant poll-budget bump. No new band. Code landed.

Fail-fast gate so a bind vessel whose RBRV_BIND_IMAGE lacks an `@sha256:` digest is rejected at regime-validation time (rbw-rvv / reveille),
not eight minutes into a cloud build at the vouch's verify-provenance step ("no @ in BIND_SOURCE").
zrbrv_enforce carries the bind-gated check (rbnve_bind ⇒ RBRV_BIND_IMAGE matches @sha256:<64-hex>), rejecting via `buc_reject "${BUBC_band_regime}"`;
modeled on zrbrn_enforce.
RBSRV homes the bind ⇒ digest-pinned invariant.
Folded in: ZRBFC_BUILD_POLL_CEILING_VOUCH raised 50→150 (a large multi-arch bind vouch runs ~8 min; 50 ≈ 4 min false-timed-out the local poller even on GCB success).

## Cinched
- Reuse BUBC_band_regime=100 — mint no new band (kept us clear of the parallel bubc_constants.sh band work).
- Bind-gated only; matches every existing valid bind config (pluml is digest-pinned).
- The regime-poison case is OUT of this pace — band-asserting fixtures live in the theurge stream (Bl), not here (paddock). Hand the bind-digest poison case (assert band 100 on a tag-only bind RBRV_BIND_IMAGE) to Bl; spec and fixture must stay consistent on band 100.
- Do NOT fix nineveh here — the gate correctly rejecting nineveh's committed tag-only config is the intended proof; that fix belongs to the nineveh pace.

## Done when
- rbw-rvv rejects a tag-only bind RBRV_BIND_IMAGE with band 100 and passes a digest-pinned one.
- RBSRV specs the bind ⇒ digest invariant.
- ZRBFC_BUILD_POLL_CEILING_VOUCH raised.

### nineveh-kroki-trial-crucible (₢BiAAo) [complete]

**[260701-1604] complete**

## Character
Resume — the config landed and a cloud mirror already ran; what remains is a digest correction, a re-vouch of the existing hallmark, and the charge/POST. Mount fresh in a separate chat.

Standing state:
- nineveh config is committed (bind vessel rbev-bottle-kroki + nameplate nineveh), but its RBRV_BIND_IMAGE is tag-only, which the new bind-digest gate now rejects at rbw-rvv (band 100).
- A cloud ordain-bind already ran: hallmark `b260701131017-r260701201021` is in GAR with image + about present, vouch ABSENT — the vouch failed on the tag-only source ("no @ in BIND_SOURCE" at verify-provenance).
- The vouch poll ceiling is now 150, so a re-vouch will not false-timeout.

Remaining:
- Fix rbev-bottle-kroki RBRV_BIND_IMAGE to tag@digest: `docker.io/yuzutech/kroki:0.31.0@sha256:c16303ecd8ae840a6e3a76efa53468836c6297eeb7b7316845c3b24e8dbd0398` — the digest the tag mirrored to, so it matches the image already in GAR. rbw-rvv then passes.
- Re-vouch the EXISTING hallmark (no re-mirror): rbw-fV sweeps image+about-present/vouch-absent; confirm it is the only pending hallmark first, then verify the vouch ark lands (rekon).
- Set nineveh RBRN_BOTTLE_HALLMARK to that hallmark (replacing PENDING-ORDAIN-BIND), notch.
- Charge nineveh, POST a graphviz sample to /graphviz/svg (workstation port 8006), get a well-formed <svg>; the post-charge hook prints the usage note.

## Cinched
- Bind/mirror, not conjure/kludge; core engines only, companions excluded; no dark-mode/darken, no suite wiring.
- tag@digest pin, not a bare digest — legible and satisfies the vouch's digest-pin requirement.

## Done when
Charge nineveh, POST a graphviz sample to the kroki /graphviz/svg endpoint on the nameplate's port, get a well-formed <svg>; the post-charge hook prints the usage note.

### charge-note-formalization (₢BiAAp) [complete]

**[260701-1635] complete**

## Character
Design + normative RBSCH change — judgment on the note/hook boundary and the BCG-clean emit; the migration is mechanical.

Formalize on-charge usage documentation as a declarative affordance: any nameplate gains a usage note by dropping one file.
Mint a third RBNNH member — a per-nameplate usage note (working name `rbnnh_charge_note`) — sibling to `rbnnh_compose_fragment` and `rbnnh_post_charge_hook` in RBSCH.
The note is data; charge core surfaces it at charge's end.
Migrate the three existing cases: srjcl's JupyterLab URL -> note (its ANTHROPIC_API_KEY check stays in a slimmed hook); pluml, which has no on-charge affordance today, gains a note; nineveh's trial freeform hook -> note.

## Cinched
- Note/hook boundary is conditionality: unconditional/templated display -> note; anything conditional or enforcing -> the post_charge hook.
- Interpolation is bash-native `${!name}` indirect expansion after name-validation — NOT envsubst (off the BCG POSIX allowlist; a builtin does it), never eval. See BCG Eval Policy + Command Dependency Discipline.
- Closed, documented placeholder vocabulary; an unresolved/typo'd placeholder is buc_die, never silent passthrough (BCG Interface Contamination / Zeroes Theory).
- The charge-core emit is kit code -> BCG governs it: route through the kit messaging surface (buc_*/BUH), never raw echo. (The self-contained hook can't use buc_*, so display belongs on the charge-core surface — BCG argues for the split.)
- rbw-ni surfacing-without-charge is deferred — the reuse this unlocks, proven later.

## Done when
RBSCH carries the new quoin; charge surfaces a nameplate's note BCG-clean at charge end; srjcl, pluml, and nineveh all render usage via notes; a typo'd placeholder fails the charge loud.

### chaining-roles-selection-adjacency (₢BiAAk) [abandoned]

**[260627-1047] abandoned**

## Spec of needed change
The rbch_enchase / rbch_palpate quoins (RBS0 "Chaining-Fact Roles") are scoped to "the role a verb plays toward a chained fact" — a HEAD-produced express-or-chain touchmark/hallmark.
A sibling pattern arrives from the federation work in ₣Bf: durable-config writes and transient reads whose resolved input is an operator selection from a discoverable set, NOT a chained fact.
These share the enchase / palpate discipline (one durable field, no self-gate, operator commits / transient use, no durable write) but are not chaining roles — there is no express-or-chain and no capture, so the depth-1 no-relay leak-elimination invariant is vacuous for them.
Align the spec text so the boundary is explicit rather than leaving the adjacent case to be mis-voiced later.

## Done when
RBS0 "Chaining-Fact Roles" names the enchase-adjacent / palpate-adjacent case and the distinguishing test — resolved input is a HEAD-produced chained fact versus an operator selection; only the former is an enchase or palpate.
The prose states the band rule: a selection-driven durable-config write or transient read uses its own failure band, never BUBC_band_chain (which is the chain-resolve band).
A future selection-driven verb can cite this boundary to classify itself without re-litigating.

## Cinched
Align the spec text; do NOT broaden the quoins to cover operator selections (operator decision).

## Character
Small, focused spec edit; vocabulary-boundary clarification, mechanical once the boundary text is agreed.

### mvp-rides-last-syncs-owners (₢BiAAe) [abandoned]

**[260624-2332] abandoned**

Cross-clone coordination gate for the MVP-cleanup stream (its own repo clone).
No code change of its own — this stream owns no hot spine file and rides LAST behind both owner streams.
Pace order within each heat is fixed and encodes every in-clone wait; this gate names only the CROSS-clone waits.

## Spec of needed change
Before landing its hot-file-touching paces, this stream syncs the owners:
- RBS0-SpecTop (₣Bf-owned): ₢BiAAc mints the coarse durable-leak quoins and ₢BiAAG amends the scry region — both append as disjoint regions behind ₣Bf's federation-block push (led by ₢BfAAO); rebase onto ₣Bf first. The in-clone mint-before-cite order holds — ₢BiAAc strictly before ₢BiAAa.
- Zipper trio: ₢BiAAa's ordain-drive colophon (rbw-od) takes the zipper baton LAST, after ₣Bf. Sync, edit rbz_zipper.sh, rebuild, commit the three-file unit.
- Theurge crate (₣Bl-owned): ₢BiAAa's onboarding-probe touch into the crate syncs ₣Bl's crate push before landing.
₢BiAAa is the heaviest pace and reaches all three owners — rail it LAST, behind every owner push. Its durable-leak enumeration also reconciles with the out-of-scope ₣Bb work as a standing external note, never pulling ₣Bb in.

## Done when
Every MVP write into an owner-stream file (the RBS0 regions, the zipper trio, the theurge crate) rebased onto that owner's push first, with no owner work overwritten.

## Cinched
Coordination-only; MVP is the last clone to land on every shared file.

## Character
Cross-clone coordination gate; mechanical.

### theurge-commit-seam (₢BiAAV) [complete]

**[260623-2115] complete**

## Character
Framework consolidation with a real safety upgrade — not mere de-duplication. Code-only, mechanical; no GCP, no cloud.

## Why this is more than DRY
The two drifted write-side commit helpers each take a generic file list, which can sweep SURPRISE edits — another officium's work, an operator's half-finished change (the wrap-sweeps-everything hazard).
Replacing them with scoped, intent-named verbs makes a fixture structurally incapable of committing anything but the thing it names.
The READ side already set this precedent: rbtdre_tree_clean is the engine-owned, documented single clean-tree primitive; this finishes the pattern on the write side.

## The console — scoped test-action utilities
- One scoped COMMIT verb per forward-evolved config class — vessel (rbrv), nameplate (rbrn), regime (rbrd/rbrr) — each deriving and staging ONLY its own path. These replace the two generic helpers (rbtdro_git_commit in rbtdro_onboarding.rs; rbtdrk_git_add_and_commit[_paths] in rbtdrk_freehold.rs); migrate both call sites (onboarding commits vessels + nameplates; freehold commits regime).
- CONFIG-ZERO discipline: named field-zeroing that FAILS LOUD if the field name is absent. The embryo exists — rbtdro_write_vessel_env already errs on a missing var, and zero = write-empty — so generalize it as one validated zero seam, NOT a function-per-field farm. The fail-on-absent doubles as a schema-drift catch.
- Home the seams in the engine beside rbtdre_tree_clean; framework-neutral error type; case-side callers keep their thin verdict adapter.

## Doctrine to document
Fixtures forward-evolve config in exactly these classes (vessel, nameplate, regime), each through its own scoped commit seam; nothing else a fixture touches is ever committed.
Config-zero exists so an in-place tracked-config reset starts from a known state — defeating the stale-value false-green — and fails loud on a renamed field.
Home it where findable (the theurge context doc the likely place).

## Done when
- The scoped commit verbs + the validated config-zero seam live in the engine; both existing commit call sites are migrated; the doctrine is documented.
- The theurge crate builds; the suites exercising the migrated sites pass; shellcheck clean if any bash is touched (expected: none).

## Cinched
- Scoped-by-class, never a generic file list — the can't-sweep-surprises property is the point.
- Config-zero is for in-place TRACKED config (the marshal-zero family); the temp-vessel test cases construct known bytes instead and need no zero call.
- Mint any new names per Lapidary; the engine prefix + the rbtdre_tree_clean sibling constrain them.

### build-poll-two-clock-split (₢BiAAC) [complete]

**[260622-1412] complete**

Drafted from ₢BBABY in ₣BB.

## Character
Single-chokepoint bash change in the shared build wait loop;
mechanical once the loop is read.
Sonnet-delegable body with driver verification.

## Goal
Split the build-completion poll budget in the shared wait loop
(`Tools/rbk/rbfcb_BuildHost.sh`, `zrbfc_wait_build_completion`) into two bounded clocks:
a shared, generous queue ceiling counting PENDING/QUEUED polls before the first WORKING,
and the existing per-kind execution ceilings counting only from the QUEUED→WORKING transition.
Every op still terminates itself with success or failure:
worst-case wall clock = queue ceiling + execution ceiling.

## Cinched
- Queue ceiling is one shared constant across kinds, sized ~15 minutes
  (exact value and constant name are mount-time);
  queue exhaustion dies loud as its own message — pool never took the build —
  and keeps the quota-review tabtarget hint.
- Per-kind execution ceilings (`ZRBFC_BUILD_POLL_CEILING_*`) keep current values;
  their meaning narrows to execution-only.
- The existing poll-20 queue-weather `buc_warn` survives
  and repeats every ~20 queued polls instead of firing once.
- One info line at the QUEUED→WORKING transition records the queued poll count,
  so future log mining gets the queue/execution split for free.

## Evidence
`Memos/memo-20260611-heat-BH-vouch-poll-ceiling-queued-burst.md` —
trace plus the 2026-06-12 incidence addendum
(2 timeouts + 1 near-miss in 531 builds over 8 weeks, all pure queue weather,
confined to the small-budget kinds).
Shape-sibling: the RBSHR progress-aware-convergence-deadline entry.

## Done when
Two clocks live in the shared loop;
queue death and execution death are distinct loud messages;
the transition info line and repeating weather warning are observable in a log;
`fast` is green and a service-tier run exercises at least one real build wait.

### yoke-touchmark-fact-fallback (₢BiAAK) [complete]

**[260623-1525] complete**

Yoke gains the express-or-chain fallback for its reliquary touchmark, and lays the two shared footings the sibling verbs will reuse.

## Character
First-consumer pace: it introduces shared hearting plumbing (a fact resolver and a touchmark-kind decoder) and makes yoke its first caller — modeled on zrbfc_resolve_vessel, the established one-resolver-many-verbs precedent.
Mechanical once the footings' contracts are set; the footings are the reusable part, yoke is one thin client.

## The footings (cinched plumbing — hearting, never a parent)
An express-or-chain resolver: a BUK fact primitive in buf_fact.sh, BCG _capture shape (stdout-once / return-1, caller guards with || buc_die), generic over any fact constant — returns the folio if set, else the chained fact, fails on a broken chain.
A touchmark-kind decoder: Lode hearting (z-prefixed), the single home for the RBGC_LODE_KIND_* decode, handling 1-char prefixes (b/r/w) and 2-char (vw/vn).
Both are parameterized so each verb stays a unique caller; final names are a mint to settle.

## BCG conformance (required)
buf_read_fact is read via $() at every call site but lacks the _capture/_recite suffix BCG keys that permission to, and no fact exception is documented — suffix it (a mint) or document the exception so the fact stack reads clean.
All bash holds BCG (kindle/sentinel, prefix rules, no inline shellcheck suppressions without cause) and lands shellcheck-clean via rbw-tl.

## Yoke itself
Express touchmark wins; absent → the resolver reads the chained reliquary touchmark; then assert kind=reliquary via the decoder.
This closes a kind-typing gap: yoke validates its touchmark's existence against live GAR today but never asserts its KIND, so a non-reliquary touchmark fails late at cohort validation; the decoder adds the kind assertion up front.
Yoke's reliquary write stays leak-safe under the chain fallback — the reliquary fans to every vessel (global, idempotent), so there is no per-vessel wrong-instance exposure.
Relax spec RBSDY from hard-require to optional-with-fallback alongside the code.

## Cinched
Express identifier always overrides the fact; the fact is the no-arg fallback only.
The resolver and decoder are shared hearting parameterized per verb — never a parent verb, never spanning bondstones.
Kind discrimination is by touchmark kind-prefix, matched as a prefix, never char[0].
The resolver NEVER relays — the chain stays depth-1, terminally consumed; this no-relay rule is the leak-elimination invariant (its rationale already lives in the base-anchor election's no-buf_relay comment).
Kind and broken-chain rejections surface a named precision-band exit code via buc_reject, never bare nonzero — mint a chain-rejection code in the open BUBC band, per the regime-poison precedent and BCG's Precision Exit-Code Band.

## Done when
The resolver and touchmark-kind decoder exist as reusable helpers with the contracts above, and the resolver buc_rejects the named chain-rejection band code on a wrong-kind or broken chain.
Yoke resolves express-or-chain, asserts reliquary, and refuses a non-reliquary or broken chain cleanly; an express call behaves as today.
buf_read_fact satisfies BCG's _capture/_recite suffix rule (suffix or documented exception).
RBSDY reflects optional-with-fallback.
rbw-tl clean and the fast suite green.

### feoff-base-anchor-split (₢BiAAQ) [complete]

**[260623-1631] complete**

Restore conjure to a pure chain HEAD — output only, never reads a prior fact — by extracting its base-anchor election into a standalone feoff link verb.

## Character
Behavior change (own cinch, distinct from the refactor-only convergence pace). The correctness fix of the heat: it closes a live provenance hole and restores the head-never-reads invariant. Contract-first, then intricate-but-bounded code.

## Contract first (RBS0)
RBK is contract-before-code, and this is a contract REVERSAL: RBSAC (ark_conjure) currently sanctions the election as "the one sanctioned post-gate write." Before the code:
Reverse RBSAC — remove the base-anchor election from conjure's contract; state conjure reads no chain and builds only from committed config.
Carve the election's contract into a NEW feoff subdoc (acronym mint, Fable-gated like the verb name).
Land the no-relay and head-never-reads invariants in that spec (and the footings' contract), not only in pace cinches that evaporate on wrap.
RBSDY's optional-with-fallback relax belongs to the footings pace, not this one.

## The split
Conjure (rbfd_build) embeds zrbfd_elect_base_anchor: it reads the chained bole touchmark, writes RBRV_IMAGE_n_ANCHOR into the vessel's rbrv.env after its clean-tree gate, then builds FROM that uncommitted value — so the artifact's git provenance names a commit whose rbrv.env still holds the OLD anchor.
Extract the election into feoff <vessel> (own colophon, express vessel arg like anoint): reads the chained bole touchmark through the shared footings, writes the vessel's anchor, creates no fact, no clean-tree gate, prints a commit reminder.
The election's call site AND its function body leave rbfd_director.sh entirely — relocated into feoff, not merely bypassed. Conjure then reads no chain and builds only from committed config.
Ceremony becomes: ensconce -> feoff -> commit -> conjure.

## Existing consumer to re-home
The election's chain consumer is the documented chain_next_invoke pairing (ensconce writes the touchmark, the following ordain reads it); the split breaks it.
Discovery: grep -rn chain_next_invoke Tools/rbk/rbtd/src/. Re-home any election-asserting chain to an ensconce -> feoff invoke (feoff before ordain), so the test proves the new ceremony rather than shipping red or being "repaired" by restoring conjure's read.

## Loud on typecheck
feoff and anoint both emit bold output on a passing typecheck — the value written and its source (chain vs express) — so a wrong build shows at the moment of action, not just in the eventual git diff. This replaces feoff's silent no-op branches.

## Cinched
Conjure is a pure head — it never reads a prior fact (the head-never-reads invariant).
feoff takes its vessel from an express arg; the chain supplies only the value.
The derived-pull affordance is preserved, relocated — not retired.
The no-relay leak-elimination rationale (today a comment in the election) relocates into feoff's body, so the invariant survives the wrap.
Extracting the election here is also its convergence onto the footings, so the later convergence pace carries only anoint.
Per the gird precedent, the feoff mint is operator-sanctioned mid-heat; flag for Fable's terminal review.

## Done when
grep zrbfd_elect_base_anchor returns zero hits in rbfd_director.sh; a grep for chain reads (buf_read_fact / BURD_PREVIOUS) in head bodies returns only feoff; conjure has none.
Conjure builds green from an EMPTY previous/ (no hard chain dependency survived), building only from committed config.
feoff resolves the bole touchmark, writes the anchor, refuses a non-bole or broken chain cleanly via the named band code, and is loud on success; anoint gains the same loud output — the written value and its source appear in output, asserted.
RBSAC no longer carries the election; a feoff subdoc holds its contract.
Provenance closure is proven by the chaining-tests pace's provenance case.
rbw-tl clean; fast suite green plus a crucible/onboarding tier exercising the re-homed election chain and the anchor path.

### augur-chaining-fact-fallback (₢BiAAM) [complete]

**[260623-1643] complete**

Augur gains the express-or-chain fallback for its lode touchmark — the second consumer of the shared footings.

## Character
Thin adopter of the shared footings; mechanical.
Also tightens augur's existing shape-only check into a real kind assertion.

## Shape
Express touchmark wins; absent -> the shared resolver reads the chained touchmark of any Lode kind; augur accepts any known kind via the shared decoder.
Augur today shape-validates the touchmark with a regex but never asserts a known kind — replace that with the decoder's known-kind assertion.
The footings are introduced earlier in this heat; reuse them, do not reimplement.

## Cinched
Reuses the shared resolver and touchmark-kind decoder — no second implementation.
Accepts any known Lode kind, unlike yoke's reliquary-only.
Read-side fallback — reads the chain for a transient action only (inspect/display), writes no durable config; dies loud on a broken chain via the shared resolver. Categorically lower severity than feoff/anoint; MUST NOT be extended to write rbrv.env without joining the durable-leak surface and its no-relay plus named-band-reject discipline.

## Done when
A no-arg augur after any capture inspects the just-captured Lode; an express touchmark behaves as today; an unknown-kind or broken chain refuses cleanly.
rbw-tl clean and the fast suite green.

### hallmark-verbs-chaining-fact-fallback (₢BiAAN) [complete]

**[260623-1657] complete**

Summon, plumb, and rekon each gain the express-or-chain fallback for their hallmark, reusing the shared resolver.

## Character
Thin, near-identical adopter applied to three hallmark verbs; mechanical.
Bundled because the edit is the same across them — they remain three distinct toothings, not a family.

## Shape
Each verb: express hallmark wins; absent -> the shared resolver reads the chained hallmark fact written by the prior build (ordain or kludge).
Hallmarks carry no sub-kind these verbs discriminate, so the typecheck is a shape check only — no kind decode; keep it minimal, shared only if a hallmark-shape predicate earns it.
The footings are introduced earlier in this heat; reuse the resolver, do not reimplement per verb.

## Cinched
Three distinct verbs, one shared resolver, no parent.
Hallmark needs shape-validation only, never kind discrimination.
Read-side fallback — reads the chain for a transient action only (pull/display/verify), writes no durable config; dies loud on a broken chain via the shared resolver. Categorically lower severity than feoff/anoint; MUST NOT be extended to write rbrv.env without joining the durable-leak surface and its no-relay plus named-band-reject discipline.

## Done when
A no-arg summon, plumb, or rekon after a build acts on the just-built hallmark; an express hallmark behaves as today; a broken chain refuses cleanly.
rbw-tl clean and the fast suite green.

### retire-vestigial-build-id-fact (₢BiAAO) [complete]

**[260623-1803] complete**

Retire the vestigial build-id fact RBF_FACT_BUILD_ID.

## Character
Retirement deletion — removing dead state, a false pointer, and a false spec claim.
Mechanical across a small set of sites, but contract-first: the quoin lives in RBS0 and is cross-referenced from two sibling specs.

## Shape
Retire the build-id fact RBF_FACT_BUILD_ID entirely — it is the orphaned half of a dismantled conjure↔vouch cross-check, made redundant when vouch became cryptographic DSSE verification (the rbgjv vouch step family).
Full provenance in backout commit 22e1c019ec (and its ancestor 9a2c90a96, branch fix-consecration-discovery-strong-tie, which built the original test-time tie).
Obsolescence verified clean — zero value-readers, collateral-safe — by a multi-agent census plus adversarial refutation; this pace is the deletion, not a re-investigation.

Delete, by durable handle (line numbers drift):
- the constant RBF_FACT_BUILD_ID in rbgc_constants.sh, and strike the now-false "read by tests" clause in its preceding block comment;
- in rbfd_director.sh (conjure path, rbfd_build): the sole writer buf_write_fact_single "${RBF_FACT_BUILD_ID}", its now-false "for cross-check with vouch provenance" comment, and the paired buc_info "Output: …${RBF_FACT_BUILD_ID}" echo;
- the RBS0 quoin in full — mapping-section attribute ref, the [[rbf_fact_build_id]] anchor, its axpof_fact voicing, and the definition body;
- the two sibling-spec typed-output cross-refs that would dangle once the quoin is gone: the {rbf_fact_build_id} axhoot_typed_output in RBSAC-ark_conjure.adoc (which itself carries the stale "Used by ark_vouch for provenance verification" prose) and the one in RBSAG-ark_graft.adoc ("Empty for graft … uniform fact coverage").

## Must not touch
Three build-id mechanisms are tangled in rbfd_director.sh; only the fact is dead.
Leave intact: ZRBFC_BUILD_ID_FILE (the rbfc_build_id.txt console-URL + build-polling file), _RBGA_BUILD_ID and the build_info.json build_id field (the Cloud Build native $BUILD_ID), and about's conjure_build_id parameter.
None shares a line with the deletion set; the shared local z_build_id is benign — the fact write is a downstream leaf nothing reads back.

## Cinched
Retire, do not restore — a separate build-id equality assertion buys little on top of cryptographic provenance bound to the image digest (a different build yields a different digest).
about's conjure_build_id parameter stays exactly as-is: explicit-only, optional, empty when omitted.
NO chain fallback is wired into it — that was the backed-out approach (25ec5400bd, reverted by 22e1c019ec).

## Done when
RBF_FACT_BUILD_ID is gone from the constant, the conjure write+echo, the RBS0 quoin, and the RBSAC/RBSAG cross-refs.
A repo-wide grep for the constant and for rbf_fact_build_id lands clean, with no dangling {rbf_fact_build_id} AsciiDoc attribute.
rbw-tl clean and the fast suite green.

### chaining-fact-footing-convergence (₢BiAAP) [complete]

**[260623-1812] complete**

Retrofit the remaining hand-rolled fact consumer — anoint — onto the shared footings, retiring the divergent implementation.

## Character
Drift-killing cleanup; mechanical. The footings exist precisely so there is one implementation; this folds the last consumer that predates them.
The base-anchor election is converged separately, as part of its extraction into feoff, so this pace no longer touches conjure.

## Shape
anoint reads three build facts (hallmark, gar-root, ark-stem) with its own hand-rolled buf_read_fact + buc_die phrasing. Route them through the shared resolver.
Discovery: grep buf_read_fact across Tools/rbk/.

## Cinched
After this pace, every fact consumer routes through the shared resolver; no hand-rolled buf_read_fact + buc_die dance remains outside the footing (the BUK self-test fact case and feoff's own use excepted).
Behavior is unchanged — a refactor onto the footings, not a contract change.
anoint's loud-on-typecheck output is a separate behavior change, owned by the feoff pace.

## Done when
anoint uses the shared resolver; a grep for hand-rolled buf_read_fact dances returns only the footing, feoff, and the tests.
anoint behaves identically, verified by the relevant fixtures.
rbw-tl clean; the fast suite green.

### chaining-fact-band-tests (₢BiAAR) [complete]

**[260623-1906] complete**

Chaining-fact band matrix — permutation and negative coverage with named precision-band exit codes, proving a bad chain attempt cannot corrupt a good one. The local matrix, verified under theurge.

## Character
Test infrastructure; one pace covering the local band matrix (feoff/yoke negatives, the good election, express-beats-chain precedence, and fact-intact). The named-band assertions are the only subtlety. The cloud integration + provenance cases split to a follow-on pace.

## Home — theurge, not the BUK self-test
The chain-rejection band fires only at the RBK consumer (feoff/yoke via buc_reject); the footing resolver and decoder return a bare 1, and the BUK bash self-test sources no RBK — so it can never drive the band.
All chaining verification therefore lives under theurge, regime-poison the type specimen: drive the real verb through the tabtarget exec path and assert the SPECIFIC band code, never bare nonzero.
RBTDGC_BAND_CHAIN=105 is projected from BUBC_band_chain in rbcc_emit_consts.
The footing primitives keep their existing BUK self-test cases — they test BUK's own code and are not this pace's work.

## Matrix tier (local, creds-free)
Drive feoff and yoke directly with a seeded input and assert the chain-rejection band:
wrong-kind touchmark (a non-bole expressed into feoff, a non-reliquary into yoke) -> the band code;
broken/absent chain (empty express and empty previous/) -> the band code;
good input -> success (the anchor is elected);
express present beats a present chained fact (precedence).
And the fact-intact proof — the operator's original worry, specified to defend it: with a GOOD fact seeded in previous/, drive feoff to a band REJECT, then assert BOTH that the band fired AND that the files that must survive (the seeded previous/ fact and the target rbrv.env) are byte-identical to their pre-call state — rejection precedes any destructive write. Not the tautological "a read left the file alone."
feoff resolves a staged temp vessel by path, so no tracked config is touched; yoke negatives reject before auth and the fan-out write.

## Cinched
All chaining self-tests are verified under theurge; the BUK bash self-test is not the home, because it cannot reach the RBK band gate.
Negative cases assert the named precision-band code (buc_reject), never bare nonzero.
The fact-intact case asserts rejection-before-write on named files, not a trivial read-left-it-alone.
No relay anywhere — the depth-1 terminal-consume contract is what these tests defend.

## Done when
The band matrix (wrong-kind, broken, good, precedence, fact-intact-via-rejection-before-write) passes via the band-code assertions, under theurge.
rbw-tl clean; the fast suite green.

### chaining-fact-cloud-code (₢BiAAT) [complete]

**[260623-2150] complete**

## Character
Service-tier test infrastructure, authored but NOT run here — coding is local and fast; the live GCP run is deferred. Standard dev with one mint (the fixture + case names).

## Scope (narrowed)
Author ONLY the cloud INTEGRATION case — the genuine cloud sibling of the local band matrix (rbtdrh_chain.rs).
The provenance case is folded out: the conjure-built-from-committed-base invariant is accepted as structural for now, and its proper fix is its own end-of-heat design pace.

## The case
Real ensconce -> chain_next_invoke -> real feoff, driven through the tabtarget exec path — proving the real producer->consumer chain the local band matrix can only simulate.
- feoff writes against a STAGED TEMP vessel resolved by path (band-matrix discipline, rbtdrh_chain.rs the model) — no tracked config is touched, and the temp vessel is constructed with known bytes so a feoff no-op is caught.
- Assert the real ensconce touchmark matches the band matrix's synthetic bole-seed SHAPE (proving the synthetic stand-in was faithful) AND feoff elects the expected anchor (the Lode locator for that touchmark).
- Read via the structured BURV fact (rbtdri_read_burv_fact) and a config-file read of the temp vessel's anchor — never a printout scrape.

## Reset + cleanup (setup establishes baseline)
- SETUP ensures the case's Lode is absent before it runs (idempotent banish-if-present), so a prior crashed run cannot poison it; pin the touchmark deterministically via the existing ensconce-stamp tweak so "ensure mine absent" has a stable handle.
- TEARDOWN banish is best-effort, not load-bearing — setup re-establishes the baseline next run.

## Naming (this pace owns the mint)
Mint the fixture name + the case name here, the sole owner.
First VERIFY which theurge file/family the fixture actually registers in (it may live in rbtdrc with the other cloud fixtures, NOT be an rbtdrh sibling) and fit the asterism to that real neighborhood; apply Lapidary (grep gate, no trodden words).

## Cinched
- Code-only: the case compiles, registers, and lists; runtime is UNVERIFIED here — the live GCP run is a deferred follow-on.
- Temp vessel, no commit, no tracked-config touch; rides service/complete (the committing objections do not apply).
- A negative, where asserted, names the precision band (buc_reject), never bare nonzero.

## Done when
- The integration case is authored on a service-tier fixture; the theurge crate builds; the fixture is registered (service/complete); shellcheck clean; rbw-tc lists the new case name.
- Runtime explicitly UNVERIFIED — the run is a follow-on.

### chaining-fact-cloud-run (₢BiAAU) [complete]

**[260623-2204] complete**

Run and verify the chaining-fact cloud cases (integration + provenance) on a service-tier session — the slow GCP half, split from the authoring pace.

## Character
Execution-only: no new code expected (the cases were authored upstream). A live GCP session running real cloud builds (ensconce, conjure), minutes-long. If a case fails, the fix loops back to authoring — fix-and-rerun.

## Done when
The chaining-fact cloud fixture runs green on a service (or dogfight) session: the integration case proves the named band survives the real exec path with a real ensconce upstream of feoff; the provenance case proves the committed-anchor stamp.

### mantle-access-probe-rename (₢BiAAX) [complete]

**[260623-2224] complete**

## Character
Mechanical operator-facing rename, low risk — an ashlar-tier vocabulary fix.

Reframe the rbw-acm probe's operator face from "don a mantle" to its true subject — mantle access — without disturbing the interior `don` act or the colophon.
The probe actively compears, dons (real Leg-3 token mint), calls Artifact Registry, and writes an attributed Data-Access audit entry;
its face must name that access-reach verdict, not the mechanism, and must not under-promise the real side effects.

## Cinched
- Name: CheckMantleAccess (frontispiece), sibling of CheckFederatedAccess / CheckPayorCredential.
  Rejected CheckMantleAdmission — the handler's 403 branch tests AR-reach (capability-set / artifactregistry.reader), which outlives mere admission.
- Colophon rbw-acm is UNCHANGED (m = mantle).
  This is what makes the rename cheap: every colophon cross-ref and the generated rbtdgc colophon const stay untouched.
- Interior `don` STAYS: rba_don_capture, rba_token_capture, the handler's Leg-3 step/die/success narration, and rba_auth's "Leg 3 (don)" text all faithfully name the real act.
  Only the intent-layer ashlar strings change.

## Done when
- Tabtarget renamed tt/rbw-acm.CheckMantleDon.sh -> tt/rbw-acm.CheckMantleAccess.sh (content byte-identical; trampoline routes on the colophon).
- The zipper purpose (RBZ_CHECK_MANTLE in rbz_zipper.sh) and the handler's buc_doc_brief + headline buc_step (rbgv_check_mantle in rbgv_cli.sh) reframed to the access verdict, keeping the active-don + AR-exercise + audit-entry honesty.
- claude-rbk-tabtarget-context.md regenerated via build (rbtdgc_consts.rs shows no diff — colophon stable).
- rbw-tq + rbw-tl green; rbw-acm <mantle> still routes. No spec edit (grep finds no reference); colophon cross-refs (rbgp_payor, rbtdrk_depot) untouched.

### reconcile-and-push-chaining-fixes (₢BiAAS) [abandoned]

**[260623-2239] abandoned**

Operator checkpoint — NOT agent work. The reminder, positioned at the chain-cluster terminus on purpose: this is where the chaining-fact changes get reconciled and pushed downstream.

## Character
Operator-owned, cross-repo. The agent cannot do this — it lives in the operator's multi-repo push/reconcile workflow and judgment.

## Why it lands here
The cluster's reusable footings (`buf_elect_fact_capture`, the `BUBC_band_chain` precision band) live in BUK, a distributed kit — so the work reaches the repos that vendor BUK. The RBK-side changes (feoff, the brand-fact collapse) are local to this repo. The operator owns the actual reconcile scope and steps.

## Done when
The operator has reconciled and pushed the chaining-fact changes downstream by their usual cross-repo workflow. Closed by operator judgment, not an automated gate.

### buk-colophon-projection-division (₢BiAAL) [complete]

**[260623-2347] complete**

## Goal
buwz (BUK workbench) colophons become generated Rust consts — first-class peers of RB's RBTDGC_ set —
retiring the hand-minted BUK colophon consts and bare "buw-" literals in the theurge crate.

## Cinched
- New "division" tier above buz_group: a start-index marker recording which zipper owns each run of the
  shared registry roll (buz_enroll itself is unchanged — same trick buz_group uses).
- The colophon-consts emitter iterates divisions, each carrying its own add/strip prefix; this retires the
  single-zipper signature of buz_emit_colophon_consts (whose sole caller is rbz_emit_consts).
- The context emitter scopes to one named division, so BUK colophons never leak into RB's
  tabtarget-context.md (the hazard: buz_emit_context emits one row per whole-roll entry).
- Project ALL buwz colophons (parity with RB's total projection), not just the few theurge consumes.
- Mints, operator may override: buz_tome (the division), BUWGC_ (the BUK const prefix).
- All new/edited bash (buz_tome, the two emitters, the rbz/buwz tome declarations) holds BCG discipline
  — kindle/sentinel shape, prefix rules, shellcheck clean via rbw-tl (no inline suppressions without cause).

## Done when
A build regenerates rbtdgc_consts.rs with the existing RBTDGC_ lines byte-identical and a new BUWGC_ block;
the theurge crate carries no bare "buw-" literals and no hand-mint BUK colophon consts
(grep '"buw-' over Tools/rbk/rbtd/src/ is clean); rbq is green; tabtarget-context.md is unchanged.

## Character
Intricate but mechanical — a shared-buz refactor with a clear verification gate and one concept mint.

### rbrs-relocate-to-future (₢BiAAE) [complete]

**[260624-0002] complete**

Drafted from ₢BBABT in ₣BB.

## Character
Intricate but mechanical.

## Goal
Pull the RBK station regime (RBRS) off the active MVP surface. RBRS is dormant podman/VM
station config (RBRS_PODMAN_ROOT_DIR / _VMIMAGE_CACHE_DIR / _VM_PLATFORM) that self-skips in
the fast suite under the Docker MVP — noise for first MVP users. Preserve the payload intact
for the podman build-out (heat ₣BW).

## Shape
Relocate intact into Tools/rbk/vov_veiled/FUTURE/ (beside rbv_PodmanVM.sh): rbrs_regime.sh,
rbrs_cli.sh, RBSRS-RegimeStation.adoc. Delete the active-surface hooks: rbw-rsr / rbw-rsv
tabtargets, the rbtdrf_rs_rbrs test plus its rbrs-missing-platform negative case, the
RBTDRM_MODULE_RBRS manifest entry, the RBTDGC_RBRS_* constants, the RBS0 TOC reference.
Discovery: grep -rn "RBRS|RENDER_STATION|VALIDATE_STATION" Tools/ (exclude burs, buk/).

## Locked
Do NOT touch the BUK station regime (BURC_STATION_FILE / BURD_STATION_FILE / burs) — it is
load-bearing for every BUD dispatch and is a distinct regime from RBRS.

## Done
Verify rbv_cli.sh is not behind a live tabtarget before relocating it. Regenerate the
generated context and rebuild theurge. Fast suite green with 0 skipped; no rbw-rs*
tabtargets remain; RBRS payload intact under FUTURE/.

### shellcheck-cover-moorings-and-jailer (₢BiAAD) [complete]

**[260624-0128] complete**

Drafted from ₢BBABU in ₣BB.

## Character

Mechanical lint cleanup with a security-boundary caveat. Most fixes are
behavior-neutral; the risk is concentrated in the jailer, where a quoting or
word-splitting "fix" can change runtime behavior. Each jailer touch must be
gated by appropriate crucible re-testing — not forbidden, but verified.

## Goal

Bring the bash that lives outside `Tools/` — and is therefore un-gated today,
since the shellcheck root scans `Tools/` only — to shellcheck-clean, then widen
the release-tier shellcheck (`rbq_qualify_shellcheck`) to cover it so it stays
clean by construction.

## Scope

Discovery recipe (these are the currently-uncovered `.sh` files):

  find . -name '*.sh' -type f -not -path './Tools/*' -not -path './tt/*' \
    -not -path './.git/*' -not -path '*/target/*'

Include the load-bearing ones: the moorings vessel jailer scripts under
`rbmm_moorings/rbmv_vessels/common-sentry-context/` (sentry + pentacle),
`MBS.STATION-reference.sh`, and the `rbnnh_post_charge.sh` charge hooks.
Exclude the same classes the release-tier gate already excludes: `Study/`
scratch scripts and `rbmm_moorings/rbml_launchers/launcher.*.sh` trivial shims.

## Jailer risk assessment — required, gating

The sentry and pentacle scripts run inside the crucible and define the security
boundary (iptables, dnsmasq, enclave network). Before landing edits to them,
classify each finding as behavior-neutral vs behavior-affecting. For anything
not provably neutral, re-run the crucible security testing appropriately — at
minimum the tadmor security fixture (`tt/rbw-ts.TestSuite.crucible.sh`, or the
tadmor fixture directly), charging and quenching a crucible as the change
demands. RBSIP / the tadmor suite is the relevant containment contract.

## Done looks like

- The included outside-`Tools/` scripts are clean under `busc_shellcheckrc`.
- `rbq_qualify_shellcheck` covers them (root widened beyond `Tools/` — multi-root
  parameter vs explicit extra-file list is a mount-time call).
- Jailer behavior verified intact by appropriate tadmor-tier testing after any
  sentry/pentacle edit; the risk assessment is recorded in the commit trail.
- The marshal-zero lint gate consequently refuses a dirty tree across the
  widened set too.

## Out of scope

- `Study/` scratch and launcher shims.
- Parallelizing shellcheck — settled out: serial is fine at release-prep cadence,
  and `xargs -P` is non-POSIX / outside the allowed-command set.

### prune-legacy-build-cycle-tabtargets (₢BiAAY) [complete]

**[260624-0159] complete**

## Character
Mechanical removal gated on a coverage check:
confirm the chaining / per-container pattern fully covers the convenience before deleting it.

## Goal
Retire the redundant build-cycle convenience tabtargets — rbw-tO (OrdainCycle / rbob_ordain) and rbw-tK (KludgeCycle / rbob_kludge).
They are fixture-uncovered, superseded, and rbw-tO is broken (rbuh_json not sourced — see the Spook in Memos/memo-20260624-jailer-shellcheck-coverage-risk.md).

## Replacement coverage (verify, then delete)
Local: rbw-cKB / rbw-cKS (rbob_kludge_bottle / rbob_kludge_sentry) — per-container kludge + drive hallmark into nameplate, fixture-covered (siege).
Cloud: rbw-fO (DirectorOrdainsHallmark) plus the onboarding ordain->propagate flow in rbtdro_onboarding.rs, which propagates the hallmark into the nameplate.
Confirm onboarding and any operator-facing convenience is already served by these; wire in the chaining op only if a real gap is found.

## Done when
- rbw-tO / rbw-tK tabtargets, the rbob_ordain / rbob_kludge functions, their CLI dispatch cases, and their rbz_zipper enrollments are gone; rbob_kludge_bottle / rbob_kludge_sentry retained.
- grep for the removed colophons lands clean across onboarding, handbook, and specs.
- Build green; rbw-tq + rbw-tl green (regenerated consts / context re-derived).

## Out of scope
The dogfight cloud-build summon failure (its own pace).
The chaining-nature common quoin in RBS0 — a separate spec finding: the chain-head / chain-link / durable-leak discipline is repeated near-verbatim across ~7 subdocs and was never lifted into one quoin.

### read-side-chaining-furnish-coverage (₢BiAAZ) [complete]

**[260624-1339] complete**

## Character
Diagnosis complete this session; remaining work is mechanical fix + audit + a reveille-tier guard.
The original "retriever-mantle / credential" hypothesis was wrong — root cause is a furnish-wiring gap.

## Root cause (found this session — do not re-derive)
The read-side chaining consumers were half-built:
the verb bodies call buf_elect_fact_capture, but their furnishing CLIs do not source buf_fact.sh,
so the helper is undefined at runtime (command-not-found) and the `|| buc_die "No hallmark"` fires.
Confirmed broken: summon and plumb.
Unaffected: the write-side durable-leak verbs (feoff/anoint/yoke) and the lode read consumer (augur) —
their CLIs already source buf_fact, which is the fix template.

## Fix (full lode precedent — no new pattern)
Make every CLI that dispatches a buf_elect/buf_read caller source buf_fact.sh, the way the lode and ledger CLIs already do.
Audit, do not trust a list: grep the callers (buf_elect_fact_capture / buf_read_fact_capture under Tools/rbk), resolve each to its furnishing CLI, confirm the source line.
vouch and rekon are not yet chained (no buf_elect call) — confirm and leave them.

## Coverage (both guards approved)
- Reveille-tier credless guard (primary): invoke each read-side verb with no folio; assert it dies with the NAMED no-fact message, not command-not-found.
  The resolve precedes any auth/network, so no cloud is needed — this is the net that would have caught the gap in CI.
- End-to-end (secondary): drive summon/plumb against real artifacts in a cloud-bearing tier, mirroring how augur is already driven end-to-end by its lifecycle fixture.

## Cinched
- Read-side stays loud-die, never band-reject (paddock chaining-fact discipline).
- The fix is the lode-precedent furnish source; no new pattern on the fix side.

## Done when
- Every read-side consumer CLI sources buf_fact; the audit recipe lands clean.
- The reveille-tier credless guard covers the read-side verbs and is green.
- summon and plumb are driven end-to-end; dogfight green is the cloud confirmation.
- Root cause recorded where the fix lives.

### anoint-band-conformance (₢BiAAd) [complete]

**[260624-1352] complete**

## Character
Small code pace mirroring the read-side band conversion on the last write-side holdout. Mechanical.

## Why
After the read-side reversal, anoint is the only chaining consumer — read or write — still on buc_die, not the named band.
It is a durable-leak writer (rewrites RBRV_GRAFT_IMAGE), so the named-band-reject discipline should cover it; the buc_die is a gap, not a decision.

## Scope
- Convert anoint's three broken-chain buc_die (hallmark, gar_root, ark_stem in rbfla_anoint.sh) to buc_reject BUBC_band_chain (105), matching feoff/yoke.
- Keep anoint chain-only — no express override (deliberate: it pulls the just-built facts forward, three facts, an override would be awkward).
- Add a reveille case in rbtdrh_chain.rs: drive anoint folio-less against an empty BURV root, assert exit 105.
  anoint resolves a graft vessel BEFORE the fact reads, so the case must stage a graft-mode vessel (RBRV_VESSEL_MODE=rbnve_graft) — adapt rbtdrh_drive_feoff, which already stages a vessel.
- Update anoint's comment if it claims loud-die.

## Done when
- anoint's broken-chain rejects with band 105; feoff/yoke/anoint are uniform on the band.
- A reveille case asserts it by exit code; chaining-fact-band green.

## Cinched
- Express override stays feoff/yoke-only by design; anoint is chain-only.
- Coding errors stay flushed-out statically, never named runtime bands.

### await-fed-rbs0-block (₢BiAAf) [complete]

**[260626-1059] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until ₢BfAAO (heat ₣Bf — the federation spec recast that seats the RBS0-SpecTop federation block) is complete, pushed to origin, and origin merged back into this clone.
Only then may the next pace — the durable-leak coarse quoins — append its disjoint RBS0 region behind FED's federation block. The in-clone mint-before-cite order still holds for the coarse quoins and the drive-link that cites them.
(The earlier "scry-region amend" mention was struck by the integrity audit: the scry pace ₢BiAAG makes no RBS0 write — the interface-by-role model it adopts is already in RBS0 at the at_sentry_container quoin — so no scry RBS0 append rides this barrier.)

## Done when
₢BfAAO is wrapped and pushed to origin, and this clone has pulled origin so the RBS0 federation block is present here.

## Character
Cross-clone barrier; hold here until the named pace is merged through.

### rbs0-chaining-style-quoins (₢BiAAc) [complete]

**[260627-0813] complete**

## Character
RB-local interim consolidation — mostly mechanical, with light judgment on the two quoins.
No AXLA, no Fable: the full cross-model consolidation is ₣Bb's parked ₢BbAAT, and this pace neither awaits nor invokes it.

## Scope — RBS0-local placeholder (Half-1 only)
1. Correct the now-false read-side prose: RBSLA, RBSAS, RBSAP (and RBSIR, which is not even chained) still say the read side "dies loud … categorically lower severity."
   It is now band-reject; fix to band-reject + distinction-by-effect.
2. Define the pattern on two coarse RBS0 quoins —
   a read-style consumer (reads a chained fact, writes no durable config, band-reject on broken resolve), and
   a write-style durable-leak link (reads a chained fact, writes one durable field, band-reject, no-relay).
   Reconcile with the existing rbf_fact_lode; generalize beyond the Lode touchmark to any chained fact (the hallmark chain too).
3. Subdocs cite the two quoins instead of restating; verb code comments shrink to cite, not restate.

## Out of scope — left for ₣Bb's ₢BbAAT
- Any AXLA motif: the home stays RB-local.
- The fine-grained quoin breakdown (chain-head, express-or-chain, no-relay, distinction-by-effect as separate quoins) — minting that set is the regret risk; the coarse read/write pair is the stable dichotomy.
- The chain-HEAD/producer role earns prose here; a quoin only if it falls out clean.

## Done when
- The false read-side prose is corrected everywhere it appears.
- Two RBS0 quoins define the read/write styles; the listed subdocs and verb comments cite them rather than restate.
- No AXLA construct introduced; the grep gate is clean on the two new names.

## Cinched
- Read-side rejects with the named chaining band (reused 105, not a new band); the read/write distinction is by effect, not exit code.
- Coding errors stay flushed-out statically, never named runtime bands.
- RB-local and Fable-free; the AXLA consolidation is ₢BbAAT's.

### cloudbuild-spec-stale-refs (₢BiAAF) [complete]

**[260627-0836] complete**

Drafted from ₢BBABV in ₣BB.

## Character
Two reference corrections in the Cloud Build specs; the RBSCJ item is mechanical, the RBSCB item is a semantic judgment — its original "broken acronym" premise is inverted at mount-time (see below). Verify-then-edit, low risk.

## Goal
The Cloud Build specs carry two reference questions. The original drafting (Cloud Build spec audit) called both "dangling symbols that no longer exist," but the RBSCB half's premise is INVERTED and must be re-derived, not acted on mechanically.

## Items
- **RBSCB authoritative-specs list — re-derive semantically, do NOT substitute.** The list names `RBSAC, RBSAV, RBSDC, RBSDN`. All four resolve to live files (`ls Tools/rbk/vov_veiled/RBSD*-*.adoc` shows RBSDC-depot_recognosce.adoc present). **`RBSDC` is NOT a dangle** — the original "has no file → likely RBSDI" suggestion is backwards and would replace a live referent with a dangling one. The real question is purely semantic: is `RBSDC` (depot_recognosce, read-only founding-survey, colophon `rbw-dr`) the right "authoritative operational spec" for this Cloud Build posture sentence, or was a now-retired inscribe spec intended? Correct ONLY if recognosce is genuinely the wrong referent for what the sentence asserts. The actual dangle — `RBSDI` → `RBSDI-depot_inscribe.adoc` in claude-rbk-acronyms.md (no such file) — is a real defect but is ₣Bf-of-record (soft-shared registry) territory: flag it, do NOT fix it here.
- **RBSCJ dead function name.** The trigger-era Decision section names `zrbf_stitch_build_json` (`grep zrbf_stitch_build_json Tools/rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc`); the live function is `zrbfd_stitch_build_json` (renamed in the foundry decomposition; RBSCB already cites it correctly). Repoint to the live name. Scope is the name only — a broader refresh of the trigger-era Decision section is a separate question, not this pace.

## Done when
RBSCJ no longer names `zrbf_stitch_build_json` (now `zrbfd_stitch_build_json`); the RBSCB authoritative-specs sentence is either confirmed correct as-is (RBSDC is live) or repointed to a genuinely-correct referent — never blind-substituted to the RBSDI dangle. The RBSDI acronyms.md dangle is left flagged for ₣Bf, not fixed here.

### scry-diagnostic-repairs (₢BiAAG) [complete]

**[260627-0917] complete**

Drafted from ₢BBABW in ₣BB.

## Character
Standalone tooling fix + one design addition; no dependency on this heat's release-qual gestalt. Goal: make scry a trustworthy, scriptable network-diagnostic primitive.

scry (`rboo_observe.sh`, colophon `rbw-cs`) proved wanting during the ₣BV Docker-Desktop containment investigation (see `Memos/memo-20260604-conntrack-spoofed-ack-adjudication.md`). Four changes:

- **Latent correctness bug — interface roles are hardcoded.** scry assumes sentry eth0=uplink / eth1=enclave. Docker's interface ordering is not guaranteed; on the WSL native-docker host the two are reversed, so scry captures the wrong sentry interface and mislabels its output. Discover interfaces by IP/role the way `rbjs_sentry.sh` already does (`ip -o addr`), never by hardcoded name.
- **Add `-e`** to the tcpdump options so L2 source MACs show. Without MACs, off-path delivery (the whole DD finding) can only be inferred from absence, not demonstrated.
- **Capture both sentry legs on docker** (currently enclave-only; the bridge leg is podman-only), so the full enclave↔uplink path is visible in one run.
- **Bound-duration / non-interactive mode.** Today scry traps SIGINT and `wait`s forever, so nothing can drive it programmatically. Add an optional duration and optional tcpdump filter expression: when a duration is supplied, run the parallel captures under `timeout <dur>` (already present in the vessels), collect, and exit 0 — a scriptable capture primitive a fixture or operator one-liner can call. When no duration is given, preserve today's run-until-Ctrl+C behavior unchanged. The filter argument scopes the capture (e.g. one host) so output stays legible.

## Done
scry resolves interfaces by role (correct under either ordering), shows MACs, captures both sentry legs on docker, and supports a bounded non-interactive capture (duration + optional filter) that returns cleanly for scripted use — interactive behavior unchanged when no duration is passed.

### clean-tree-rationale-param (₢BiAAI) [complete]

**[260627-1002] complete**

Drafted from ₢BBABZ in ₣BB.

## Character
Reconfirm-then-mechanical: independently re-derive the design from sources, then a multi-site string + parameter edit.
The originating chat ran below Opus quality and manufactured a non-issue before being corrected — treat every claim in this docket as a hypothesis to verify, not as authority.

Surface, per call site, the precise rationale for why `bug_require_clean_tree` (BUG, `Tools/buk/bug_git.sh`) demands a committed tree at that locale.
Rationale strings live as RB constants in `Tools/rbk/rbcc_constants.sh`; each call site passes its own as a new gate argument; BUG stays kit-agnostic and only displays the string it is handed.

## Reconfirm first (origin chat was unreliable)
Verify from sources, not from the claims here:
- Enumerate the call sites and read each local why: `grep -rn --include='*.sh' 'bug_require_clean_tree' Tools/` (one definition lives in BUG; the rest are calls).
- Confirm how many distinct rationales actually exist — the chat claimed two: a build/capture "HEAD-as-provenance" family and the depot-tripwire "inscribed bytes must be a committed reference" case. Correct it if the sites disagree.
- Confirm BCG tinder vs kindle for the new constants (rbcc is tinder-heavy; these are pure string literals).
- Settle what the chat left open: whether the rationale displays on the failure path only, or always.
- Re-judge whether this needs anything beyond constants + param. The chat concluded the principle is already spec-homed (`rbtgo_lode_ensconce`, `rblv_git_commit` in RBS0, plus the tripwire statement in RBSRT/RBSMF) and warrants no new quoin or RBr_ rivet — an implementation detail, not a concept. Confirm or overturn.

## Cinched
Rationale strings live in rbcc (RB-space), never inside BUG.
The gate gains a parameter; every call site supplies its locale's rationale.

## Done when
The re-verified set of rationale constants exists in rbcc.
Every `bug_require_clean_tree` call passes its locale's rationale and the gate surfaces it to the operator.
Tripwire aside (separable, not required): its rationale is stated near-verbatim in both RBSRT and RBSMF — fold to one if you touch it.
`tt/rbw-tl.Shellcheck.sh` clean and the fast suite green.

### build-bucket-vestige-scrub (₢BiAAH) [complete]

**[260627-1120] complete**

Drafted from ₢BUAAB in ₣BU.

Rigorously determine whether the GCS build bucket is fully superseded by pouch-based build-context delivery, and retire the vestige if so.

Build context now ships to Cloud Build as the pouch — a FROM SCRATCH OCI image pushed to GAR before `builds.create`.
See RBS0's `{rbtga_ark_pouch}` definition and the pouch push in `rbfd_director.sh`; RBSCTD records the trigger-dispatch supersession that introduced it.
The older design staged source into the GCS build bucket; a kit-wide grep for `storageSource` and `objectCreator` lands empty today, so that staging path appears gone.

Candidate residue, if the verdict is retire:
the build-bucket creation and its one-day lifecycle rule at depot levy in `rbgp_payor.sh`;
Mason's `roles/storage.objectViewer` grant on that bucket at levy;
bucket-only constants in `rbgb_buckets.sh` / `rbgd_depot.sh` / `rbdc_derived.sh`;
and — the integrity audit found the original residue list understated this — the RBS0 `rbtgi_build_bucket` quoin (definition + mapping ref), its citations in RBSMF / RBSDU / RBSGS, and the Mason "reads from the build bucket" prose. RBS0 is written under BOTH verdicts (retire the quoin, or correct it to match).

## Cross-clone note (surfaced by the integrity audit)
The RBS0 edit lands in the `[[rbtgi_depot]]` / `[[rbtgi_build_bucket]]` definition cluster, which BRACKETS FED's `[[rbtgi_manor]]` manor/org scrub (₢BfAAE).
This pace is gated (via the ₣Bi RBS0 barrier) only on FED's federation-block anchor, NOT on ₢BfAAE, and FED never pulls MVP's RBS0 edits — so at final convergence these two near-adjacent, uncoordinated edits to one definition cluster may textually conflict.
It is EXPECTED-and-resolvable (blank-line-separated hunks under a default 3-line-context merge), NOT a defect or a missing barrier: resolve it by hand at convergence; do not add a heavyweight barrier for it.

## Done when
Every reader and writer of the build bucket is enumerated from the live kit, yielding a verdict.
Either a live consumer is found — keep the bucket and correct the specs to match — or none is, in which case the bucket, its levy creation, Mason's grant, and the RBS0 quoin + its spec citations retire together with the suites green.

## Cinched
Pouch is the build-context delivery mechanism; this pace does not reopen that decision.
No cult-spec drift to fix here: the RBSDK director-enrobe storage line and the RBSRK retriever occurrencesViewer drift this docket once carried are both moot — those specs were deleted in the 260619 cult-verb-estate demolition, and the live retriever occurrences grant is already correct in RBSDC (split-study Decision 3, memo-20260624-Bf-split-study).

## Character
Investigation-first ACG simplification — a hidden live consumer is the one risk, so the study gates the cut; mechanical once the verdict is in.

### resolved-base-provenance (₢BiAAW) [complete]

**[260627-1458] complete**

## Character
Design conversation requiring judgment — re-express a known soundness gap and work out the proper fix.
NOT implementation; the deliverable is a ratified design plus a named follow-on path.

## The problem, re-expressed (the nutrition label)
A conjure build is meant to build FROM the base its committed config names; we want to verify it actually did, to catch a regression where the build re-derives its own base and ignores the committed one.
We cannot today: the build stamps a metadata label (the about ark / build_info) recording the git commit, builder, and timestamps — but no line naming the base it built from.
Bind and graft modes record a `.source`; conjure does not — that asymmetry is the gap.
With no recorded base, a test has nothing to read back and compare, so no honest provenance assertion exists.
This is the successor the fold deferred: the structural-acceptance decision parked the invariant; this pace designs the real fix.

## The crux — do not lose this
The recorded base must be the base AS ACTUALLY USED — captured from the build's real resolution (the actual resolved FROM digest), never echoed from the committed config.
An emitter that copies RBRV_IMAGE_n_ANCHOR from config is circular: it restates the committed value no matter what the build consumed, and the false-green survives.
Record the flour that went in the mixer, not the recipe card.

## Design questions to settle
- WHERE to record: (1) the SLSA provenance the build already emits — the resolved base is conceptually a SLSA material/resolvedDependency, so the standard slot may be the right home before minting anything bespoke; (2) the about ark as a first-class field, symmetric with bind/graft `.source`; (3) the vouch ark, only if the resolved base should be a SIGNED, consumer-verifiable provenance claim, not just descriptive. Weigh standard-slot vs bespoke; decide vouch in/out by whether a consumer verifies it.
- HOW to read back: surface it host-side as an rbf_fact_* fact file after the build (structured, no printout scraping) — the channel a future regression test consumes. Confirm a post-build host step can extract it, as the touchmark/hallmark facts already do.
- Mint any new JSON wire key under the sprue + Lapidary discipline.

## Done when
- The problem and the proper fix are written up clearly enough to ratify, the nutrition-label framing carried through.
- The fix is specified: where the resolved base is recorded (SLSA/about/vouch), captured-from-actual-resolution honored, and how it is surfaced for read-back.
- A follow-on implementation path is named (its own pace or a spec entry) — this pace is design, not build.

## Cinched
- Captured-from-actual-resolution, never echoed-from-config — non-negotiable.
- This is the home for the regression test the fold deferred; the test is downstream of recording the base.

### resolved-base-readback-and-verify (₢BiAAm) [complete]

**[260630-1423] complete**

## Character
Build-side change plus a spec sheaf, a foreign-behavior rivet, and a theurge regression case.
The design is settled and grounded against the live depot; the work is mechanical against hooks that already exist.

## Goal
Capture a conjure hallmark's resolved base image as a signed digest, by writing it as an image label at build time —
so the recorded base is captured-from-actual-resolution and rides the existing google-worker signature, never a config echo.
The original "read the signed attest resolvedDependencies" approach is dead: that field carries only the build-step tooling, never the FROM base.
Full grounding and the five verified links: Memos/memo-20260630-conjure-resolved-base-provenance-location.md.

## Done when
- The conjure build resolves each populated base slot's ref to an @sha256: digest in a cloud step, pins the build-arg to that digest so buildx provably builds from exactly it, and emits it as a sprued image label (one per RBF_IMAGE_n slot; a scratch slot gets none).
- plumb reads the resolved-base label back from the signed attest image's config blob (registry-raw, no gcloud) and surfaces it as a fact.
- RBSAC specs the digest-pin and the label emission; RBSAP specs the label readback; an RBr_ rivet cites the two foreign-behavior reliances — buildx labels survive the per-platform pullback into the attest image, and the google-worker DSSE signs that attest image — each with a demolition condition.
- A theurge regression case drives a conjure ordain, reads the resolved-base label, and fails loud on divergence from committed config.

## Cinched
- Approach is b2 (build-time signed label), not provenance readback — the memo holds the why.
- The base tag to digest resolve happens in a cloud step (GCP egress), reusing the gcrane-fingerprint shape; not host-side.
- The new label key takes a unique sprue (sprue_resolved_base_n form); the existing rbgjb03 labels stay unsprued by deliberate divergence.
- The fact is a transient cache; the durable signed home is the attest-image label.
- A FROM-direct Dockerfile cannot reach this — rbfh hygiene already forces every FROM to a RBF_IMAGE_n slot or scratch before any build.

### await-fed-zipper-and-thg-crate (₢BiAAg) [complete]

**[260702-1131] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until BOTH of the following are complete, pushed to origin, and origin merged back into this clone:
- ₢BfAAz (heat ₣Bf — the terrier-scaffold-retirement, ₣Bf's LAST zipper + RBS0 write under the 260630 rework; retargeted from ₢BfAAF, which the rework's reorder moved off "last") — so the zipper baton has passed ₣Bl → ₣Bf → here and this stream takes it LAST for the ordain-drive colophon and its RBS0 drive-verb quoin;
- ₢BlAAL (heat ₣Bl — the onboarding-reliquary-probe helper) — so the theurge crate's onboarding area is settled before the drive-link's onboarding-probe touch.
Only then may the next pace — the unified nameplate-hallmark drive-link, the heaviest and most cross-cutting pace — proceed; it rails last by design, behind every owner push.

## Done when
Both ₢BfAAz and ₢BlAAL are wrapped and pushed to origin, and this clone has pulled origin so the full federation zipper+RBS0 state and the settled onboarding crate are both present here.

## Character
Cross-clone barrier (MVP rides last); retargeted 260630 from ₢BfAAF to ₢BfAAz (₣Bf's actual last zipper/RBS0 writer after the rework). Hold until both named paces are merged through.

### nameplate-hallmark-drive-chain-link (₢BiAAa) [complete]

**[260702-1222] complete**

## Character
Larger structural + spec change — code and spec together, above the ₣Bi loose-ends norm.
May be deferred, but scope is settled.

## Goal
Give "drive a freshly-built hallmark into a nameplate's RBRN_*_HALLMARK" ONE disciplined chain-link home — read one resolved value, write one durable config field — shared by BOTH the local kludge and cloud ordain paths,
and bring that home onto the formal durable-leak surface with the same discipline feoff/yoke carry.
Replaces the three ad-hoc forms that exist today.

## Cinched
The nameplate-hallmark drive JOINS the durable-leak surface — full treatment: no-relay, named-band-reject, theurge verification, and a spec home that enumerates it alongside feoff/anoint/yoke.
NOT implementation-unification only.
Rationale: pattern consistency is critical for healthy LLM pattern reuse —
a durable config write that looks like the others must BE one of them, or it teaches the wrong pattern to every future instance.

## The three forms it replaces (grep, don't trust a list)
grep -rnE 'drive_hallmark|RBRN_(BOTTLE|SENTRY)_HALLMARK' Tools/rbk/
Today the same durable write lives as: bash zrbob_drive_hallmark fused into the local kludge wrappers (rbw-cKB/cKS); a Rust reimplementation rbtdro_drive_hallmark for onboarding; and a handbook step that tells the cloud operator to hand-edit rbrn.env.
The cloud path has no operator-facing drive verb at all.

## Boundary
Keep the local one-shot: rbw-cKB/cKS stay single-command, composing build-head + the new drive-link.
Unify the underlying operation, not the operator gesture — do not tax the local dev loop with a second command.

## Cross-clone surface (surfaced by the integrity audit — barriers cover it, this is for legibility)
This is the heaviest cross-cutting pace; it writes into BOTH sibling clones' territory, all of it covered by the ₢BiAAg dual-wait barrier (which waits on ₢BfAAz and ₢BlAAL) — recorded here so a mount agent sees the surface, not just the ₢BbAAT coupling below.
- **Same-symbol collision with ₢BlAAK (₣Bl, theurge crate).** ₢BiAAa REMOVES the Rust reimplementation rbtdro_drive_hallmark (callers compose the unified primitive); ₢BlAAK REFACTORS that same function (routes it through the engine config-console). Whichever lands second reconciles — ₢BiAAa's removal supersedes ₢BlAAK's reroute. Both ₢BlAAK and ₢BlAAL have since LANDED and pushed, so the refactored form is already in this clone's ancestry once ₢BiAAg merges through — the reconcile is informed, not a surprise.
- **RBS0 write (₣Bf-owned).** "Enumerate alongside feoff/anoint/yoke" means a new rbtgo_ drive-verb quoin + include into RBS0; gated because ₢BiAAg waits on ₢BfAAz, FED's last RBS0 writer under the 260630 rework (retargeted from ₢BfAAF, which the reorder moved off "last").
- **Zipper-trio write (₣Bl-owned).** The new cloud drive verb mints a colophon (beside rbw-rvf/rva/rvy), editing the zipper trio; gated by the zipper baton arriving last at MVP (₢BiAAg on ₢BfAAz, whose ancestry carries ₣Bl's last zipper write ₢BlAAb and every ₣Bf zipper write through the scaffold retirement).

## Cross-heat coupling — ₢BbAAT (₣Bb)
That pace consolidates the chaining discipline-prose into one home and enumerates the durable-leak-link as exactly three verbs writing RBRV_*.
This pace adds a fourth member writing RBRN_*, so whichever lands second reconciles: the consolidated enumeration and the "writes one durable config field" wording must cover the nameplate write.
Same for its companion memo (memo-20260624-chaining-pattern-axla-consolidation.md).

## Done when
- One drive-link primitive is the single home; the Rust reimplementation and the handbook hand-edit are gone — both compose the primitive.
- The drive is on the durable-leak surface: no-relay, named-band-reject, and theurge verification matching the feoff/yoke pattern.
- The spec home enumerates the nameplate-hallmark drive alongside feoff/anoint/yoke.
- Local kludge stays one-shot; the cloud path gains a real operator drive verb.
- Build green; fast + siege green.

### fact-next-tabtarget-emitter (₢BiAAn) [complete]

**[260702-1243] complete**

## Character
Small self-contained mechanism pace — one helper family plus success-path wiring.
Mechanical once the shape is set; the only judgment is the mint and the spec home.

## Goal
A per-fact-type "next tabtargets" emitter:
keyed on the fact a chain HEAD just wrote, it prints the tabtargets that consume that fact, composing the existing buc_tabtarget primitive.
The producing HEAD calls it on the success path, so the operator sees what the fresh fact unlocks.
This replaces the hand-placed one-off next-step hints with one home per fact.

## Done when
- One emitter per fact type owns its consumer list; today that is exactly one — the hallmark fact (RBF_FACT_HALLMARK) — and its producing HEAD calls it on success.
- The hallmark emitter lists every current consumer, including the unified nameplate-hallmark drive verb; this pace therefore lands after that verb exists.
- The list composes buc_tabtarget with only light context filtering (enough to fill the folio argument), never a predicate farm.
- An RBS*.adoc spec home documents the emitter mechanism; which subdoc is chosen at mount.
- Build green; shellcheck clean; reveille green.

## Cinched
- Per-fact-type, never per-call-site: the consumer list has ONE home per fact, not the drift the hand-coded form would grow.
- RBK home: the buc_tabtarget primitive and the buf_ resolver stay generic BUK; the RBK colophon content lives RBK-side.
- The emitter's consumer set must agree with the durable-leak / chaining enumeration; cite it so the two encodings cannot silently drift.

## Sources
buc_tabtarget in buc_command.sh; RBF_FACT_HALLMARK in rbgc_constants.sh; prior-art hand-placed next-step hints in rbgp_payor.sh and rbfd_director.sh.

### diffuse-codex-vocabulary-to-rbs-bus (₢BiAAh) [complete]

**[260626-0328] complete**

## Character
Diffusion — apply the settled document-composition vocabulary to the RBS0 and BUS0 codices. Mechanical, with per-sheaf status judgment; two specs.

## Done when
RBS0 and BUS0 each define a self-naming identity quoin sealed by //axvd_codex; every included
sheaf is marked at its head and at its include-site with //axvd_sheaf and exactly one axd_
normativeness status; each tendon binding resolves (the sheaf names a real codex).

## Source
Vocabulary home: MCM == Document Composition (mcm_codex/sheaf/pandect/canon, mcm_tendon) and
AXLA == Axial Voicing Document Terms (axvd_codex/axvd_sheaf + the two emplacement templates)
plus the axd_ Normativeness Dimensions. Minted 260625.

### veil-jurisdiction-and-windows-apparatus (₢BiAAi) [complete]

**[260701-1943] complete**

## Goal
Unpublish the BUK jurisdiction apparatus (BURN/BURP regimes + caparison/invigilate/garrison) and the Windows documentation guides from the distributed/canonical surface, pending re-gestation through the mews aspirant sheaf (`Tools/jjk/vov_veiled/JJS-aspirant-mews.adoc`).

## Cinched
- Veil, do not delete: relocate the code + guides to a veiled directory so they are unpublished but still function locally. Deletion stays off the table while these are the only working fleet apparatus — it waits until the mews supersedes them.
- Preserve the hard-won Windows-transport Palisade knowledge; veiling retains it.
- Record the disposition (where each artifact went) at wrap, and cross-note it in the mews aspirant sheaf so the re-gestation knows its raw material.

## Done when
The jurisdiction apparatus and Windows guides no longer sit on the distributed/canonical surface, every sourcing and launcher reference still resolves, and the disposition is recorded.

## Discovery
grep the apparatus: `burn_`/`burp_` regimes + CLIs, `bujb_jurisdiction.sh`, the `BUSJ*` specs, `WSG`, and the windows handbook guides. Relocation means updating every reference in the same move.

## Character
Mechanical relocation + reference updates, gated on the veil-not-delete decision. Caution: live load-bearing paths (BURN/BURP machinery, any Linux/cerebro provisioning) must keep functioning.

### drain-jurisdiction-heat-into-mews-sheaf (₢BiAAj) [complete]

**[260701-2006] complete**

## Goal
Reduce heat ₣A-'s jurisdiction design substance into the mews aspirant sheaf
(Tools/jjk/vov_veiled/JJS-aspirant-mews.adoc), then archive ₣A-. The apparatus is
already veiled and ₣A-'s rough paces already abandoned; what remains is to drain
the KNOWLEDGE and retire the heat.

## Character
The tricky reduction of a detailed PLAN into a DESIGN CONCEPT — mulling, not
construction; Fable-tier deliberation. The ₣A- paddock is an operational plan
(three verbs, two regimes, ceremonies, tabtargets); the sheaf wants durable design
concept. The judgment: what re-gestates intact vs. is redesigned vs. is disposable
operational detail — the sheaf's own "Open forks" section frames the live question.

## Raw material
- ₣A- paddock — the design substance (groom ₣A-; after archive, in retired/).
- Windows-transport Palisade memos: `ls Memos/*windows*` plus
  memo-20260511-orchestration-style-axla-draft.md.
- Shelved implementation for reference: Tools/{buk,rbk}/vov_veiled/
  (BUSJ*/WSG specs; burn/burp/bujb/buhj/bujp + rbhw* code).

## Done when
₣A-'s design substance lives in the mews sheaf as design concept, the veiling
disposition is cross-noted there, and ₣A- is archived.

### clean-tree-gate-callsite-migration (₢BiAAl) [complete]

**[260702-0348] complete**

Migrate the existing bug_require_clean_tree call sites to the well-formed precision-band variant, then retire the malformed original.

## Spec of needed change
Once the well-formed bug_require_* variant exists (slated on the federation-build heat), migrate the existing malformed-gate call sites to it and remove the original.
Discovery (grep, do not trust a count): grep -rn 'bug_require_clean_tree' Tools/ — today ~9 RBK call sites (kludge, immure, conclave, conjure, mirror, underpin, ensconce, affiance, inscribe) each passing an RBCC_verb_* context string, plus ~4 spec references (RBSAC, RBSLE, RBSMF, RBSRT) and the BUK core doc naming the old gate.
Each call site gains the error-condition constant + RB rationale the new variant wants; the spec references and the BUK doc update to name the new gate.

## Done when
Every bug_require_clean_tree call site uses the well-formed variant, the malformed original is gone, the spec/doc references name the new gate, and the suites the affected verbs ride stay green.

## Cinched
Gated on the federation-build heat's variant pace landing first (the variant must exist before sites migrate to it).
This is the deferred mass-migration half deliberately split off so the federation work did not churn ~13 sites mid-stream.

## Character
Mechanical call-site migration across ~9 RBK verbs + spec/doc reference updates; the judgment is per-site error-condition/rationale selection.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 q bind-digest-gate-vouch-budget
  2 o nineveh-kroki-trial-crucible
  3 p charge-note-formalization
  4 V theurge-commit-seam
  5 C build-poll-two-clock-split
  6 K yoke-touchmark-fact-fallback
  7 Q feoff-base-anchor-split
  8 M augur-chaining-fact-fallback
  9 N hallmark-verbs-chaining-fact-fallback
  10 O retire-vestigial-build-id-fact
  11 P chaining-fact-footing-convergence
  12 R chaining-fact-band-tests
  13 T chaining-fact-cloud-code
  14 U chaining-fact-cloud-run
  15 X mantle-access-probe-rename
  16 L buk-colophon-projection-division
  17 E rbrs-relocate-to-future
  18 D shellcheck-cover-moorings-and-jailer
  19 Y prune-legacy-build-cycle-tabtargets
  20 Z read-side-chaining-furnish-coverage
  21 d anoint-band-conformance
  22 f await-fed-rbs0-block
  23 c rbs0-chaining-style-quoins
  24 F cloudbuild-spec-stale-refs
  25 G scry-diagnostic-repairs
  26 I clean-tree-rationale-param
  27 H build-bucket-vestige-scrub
  28 W resolved-base-provenance
  29 m resolved-base-readback-and-verify
  30 g await-fed-zipper-and-thg-crate
  31 a nameplate-hallmark-drive-chain-link
  32 n fact-next-tabtarget-emitter
  33 h diffuse-codex-vocabulary-to-rbs-bus
  34 i veil-jurisdiction-and-windows-apparatus
  35 j drain-jurisdiction-heat-into-mews-sheaf
  36 l clean-tree-gate-callsite-migration

qopVCKQMNOPRTUXLEDYZdfcFGIHWmganhijl
··x···x··x······x·····x···x···xx···· RBS0-SpecTop.adoc
······x·······xxx·x···········x··x·· rbz_zipper.sh
······x····x···xx·x···········x··x·· rbtdgc_consts.rs
······x·······x·x·x···········x··x·· claude-rbk-tabtarget-context.md
·····xx··x··················x··x···x rbfd_director.sh
···········x·······xx·x·······x····· rbtdrh_chain.rs
······x···············x·······xx·x·· claude-rbk-acronyms.md
·····xx···x·········x·x············· rbfla_anoint.sh
···········x····x·············x····x rbcc_constants.sh
···········xx·········x··········x·· rbtdrm_manifest.rs
········x··········x··x·····x······· rbfcp_plumb.sh
······x··x··················x······x RBSAC-ark_conjure.adoc
x··············xx················x·· rbtdrs_poison.rs
··················x············x···x rbfk_kludge.sh
··················x·····x·····x····· rbob_cli.sh
········x·············x·····x······· RBSAP-ark_plumb.adoc
········x··········x··x············· rbfr_retriever.sh
·······x···········x··x············· rbldl_lifecycle.sh
······x··x··················x······· rbgc_constants.sh
···x··x·······················x····· rbtdro_onboarding.rs
··x···············x···········x····· rbob_bottle.sh
·································x·x claude-buk-core.md
··························x········x RBSMF-depot_levy.adoc, RBSRT-RegimeDepot.adoc, rbgp_payor.sh
···············x·················x·· buwz_zipper.sh
···············x·x·················· rbq_qualify.sh
···············xx··················· rbtdrf_fast.rs
···········xx······················· rbtdrc_crucible.rs
·········x··················x······· RBSAB-ark_about.adoc
········x·············x············· RBSAS-ark_summon.adoc, RBSIR-image_rekon.adoc, rbfln_inventory.sh
·······x··············x············· RBSLA-lode_augur.adoc
······x····························x RBSLE-lode_ensconce.adoc, rbldb_bole.sh, rbldr_reliquary.sh, rblds_spine.sh, rbldv_immure.sh, rbldw_underpin.sh
······x···············x············· RBSDF-director_feoff.adoc
·····x················x············· RBSDY-director_yoke.adoc
·····xx····························· rbfl0_ledger.sh, rbfly_yoke.sh, rbldk_kind.sh
···x··························x····· claude-rbk-theurge-ifrit-context.md
·xx································· rbnnh_post_charge.sh
x···x······························· rbfc0_core.sh
xx·································· rbrv.env
···································x ACG-AllocationCodingGuide.md, bug_git.sh, rbndb_base.sh, vob_build.sh
··································x· JJS-aspirant-mews.adoc
·································x·· buh_handbook.sh, buhj_cli.sh, buhj_jurisdiction.sh, bujb_cli.sh, bujb_jurisdiction.sh, bujp_preflight.sh, burn_cli.sh, burn_regime.sh, burp_cli.sh, burp_regime.sh, buw-hj0.HandbookJurisdictionTop.sh, buw-hjl.HandbookJurisdictionLinux.sh, buw-hjm.HandbookJurisdictionMacos.sh, buw-hjw.HandbookJurisdictionWindows.sh, buw-jpCL.CaparisonLinux.sh, buw-jpCM.CaparisonMacos.sh, buw-jpCW.CaparisonWindows.sh, buw-jpGb.GarrisonBash.sh, buw-jpGc.GarrisonCygwin.sh, buw-jpGw.GarrisonWsl.sh, buw-jpS.PrivilegedSsh.sh, buw-jwc.WorkloadCommandFile.sh, buw-jwk.WorkloadKnock.sh, buw-jws.WorkloadInteractiveSession.sh, buw-rnl.ListNodeRegime.sh, buw-rnr.RenderNodeRegime.sh, buw-rnv.ValidateNodeRegime.sh, buw-rpl.ListPrivilegeRegime.sh, buw-rpr.RenderPrivilegeRegime.sh, buw-rpv.ValidatePrivilegeRegime.sh, rbhw0_cli.sh, rbhw0_top.sh, rbhw0_windows.sh, rbhwcd_docker_context_discipline.sh, rbhwdd_docker_desktop.sh, rbhwht_handbook_top.sh, rbtdrf_handbook.rs, rbw-HWdc.DockerContextDiscipline.sh, rbw-HWdd.DockerDesktop.sh, rbw-h0.HandbookTOP.sh, rbw-hw.HandbookWindows.sh
·······························x···· rbfb_beckon.sh, rbfd_cli.sh, rbfk_cli.sh
······························x····· RBSND-nameplate_drive.adoc, memo-20260624-chaining-pattern-axla-consolidation.md, rbhoda_director_airgap.sh, rbhodb_director_bind.sh, rbrn_cli.sh, rbrn_drive.sh, rbw-nd.DriveNameplateHallmark.sh
····························x······· memo-20260630-conjure-resolved-base-provenance-location.md, rbfcg_gar.sh, rbgjb03-buildx-push-image.sh, rbgjb03-resolve-base-digests.sh, rbgjb04-buildx-push-image.sh, rbgjb04-per-platform-pullback.sh, rbgjb05-per-platform-pullback.sh, rbgjb05-push-per-platform.sh, rbgjb06-push-diags.sh, rbgjb06-push-per-platform.sh, rbgjb07-push-diags.sh, rbtdrd_dogfight.rs, rbtdrv_patrol.rs
··························x········· RBSDU-depot_unmake.adoc, RBSGS-GettingStarted.adoc, rbdc_derived.sh, rbgb_buckets.sh, rbgd_depot.sh, rbrd.env, rbrd_regime.sh, rbtdrp_attest.rs, rbtdtk_freehold.rs
························x··········· claude-rbk-conduct.md, rboo_observe.sh
·······················x············ RBSCJ-CloudBuildJson.adoc
···················x················ rbfc0_cli.sh, rbfh_cli.sh, rbfr_cli.sh, rbfv_cli.sh
··················x················· rbw-tK.KludgeCycle.tadmor.sh, rbw-tO.OrdainCycle.tadmor.sh
·················x·················· MBS.STATION-reference.sh, buq_qualify.sh, memo-20260624-jailer-shellcheck-coverage-risk.md, rbjp_pentacle.sh, rbjs_sentry.sh, rbk-prep-release.md
················x··················· CLAUDE.consumer.md, RBSRS-RegimeStation.adoc, rbrs_cli.sh, rbrs_regime.sh, rbv_cli.sh, rbw-rsr.RenderStationRegime.sh, rbw-rsv.ValidateStationRegime.sh
···············x···················· buz_zipper.sh, rbq_cli.sh, rbte_cli.sh
··············x····················· README.md, rbgv_cli.sh, rbw-acm.CheckMantleAccess.sh, rbw-acm.CheckMantleDon.sh
···········x························ lib.rs
·········x·························· RBSAG-ark_graft.adoc, rbfv_verify.sh
······x····························· RBSLC-lode_conclave.adoc, RBSLI-lode_immure.adoc, RBSLU-lode_underpin.adoc, rbflf_feoff.sh, rbw-rvf.DirectorFeoffsVessel.sh
·····x······························ BUS0-BashUtilitiesSpec.adoc, bubc_constants.sh, buf_fact.sh, butcfc_facts.sh, butt_testbench.sh, rbld0_lode.sh
····x······························· rbfcb_host.sh
···x································ rbtdre_engine.rs, rbtdrk_freehold.rs, rbtdte_engine.rs
··x································· RBSCH-charge_hook.adoc, rbnnh_charge_note.txt
·x·································· rbrn.env, rbw-cC.Charge.nineveh.sh, rbw-cQ.Quench.nineveh.sh
x··································· RBSRV-RegimeVessel.adoc, rbrv_regime.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 179 commits)

  1 m resolved-base-readback-and-verify
  2 o nineveh-kroki-trial-crucible
  3 q bind-digest-gate-vouch-budget
  4 p charge-note-formalization
  5 i veil-jurisdiction-and-windows-apparatus
  6 j drain-jurisdiction-heat-into-mews-sheaf
  7 l clean-tree-gate-callsite-migration
  8 g await-fed-zipper-and-thg-crate
  9 a nameplate-hallmark-drive-chain-link
  10 n fact-next-tabtarget-emitter

123456789abcdefghijklmnopqrstuvwxyz
·xxxxx·····························  m  5c
·········x······xx·················  o  3c
···········x·xxx···················  q  4c
··················xx···············  p  2c
······················xx·x·········  i  3c
··························xx·······  j  2c
····························xx·····  l  2c
······························x····  g  1c
·······························xx··  a  2c
·································xx  n  2c
```

## Steeplechase

### 2026-07-02 12:43 - ₢BiAAn - W

Added the per-fact next-tabtargets emitter (RBS0 rbch_beckon). New guard-free rbfb_beckon.sh (Foundry Beckon, fresh rbf child) holds rbfb_beckon_hallmark: keyed on RBF_FACT_HALLMARK, composes buc_tabtarget over the fact's full consumer roster — rbch_palpate readers (summon/plumb-compact/plumb-full/rekon, folio filled with the just-written hallmark) plus rbch_enchase writers resolving the same fact (anoint/drive, folio placeholder). Light fill-or-placeholder filtering, no per-mode predicate farm. Wired on the success path of both hallmark producers: rbfd_ordain tail (one call covering conjure/bind/graft) and rbfk_kludge tail (charge hint kept). Both CLIs' furnishes source the module. Spec: minted rbch_beckon in RBS0 Chaining-Fact Roles beside enchase/palpate, roster pinned to the two role enumerations directly above it (drift-cite made local), beckon framed as a courtesy emission that resolves no chained value and carries no band. RBFB registered in the acronym map. Word 'beckon' operator-chosen; 'blazon' kept furrowed. shellcheck 228 clean; theurge build green; reveille 117/0/0 (furnish-invariant passed — no furnish gap). Closes heat rbk-01-mvp-review-loose-ends at 36/36. Code at 020c6c148.

### 2026-07-02 12:40 - ₢BiAAn - n

Add the per-fact next-tabtargets emitter (RBS0 rbch_beckon): after a chain HEAD writes a fact, one emitter for that fact announces the tabtargets that consume it, so the operator sees what the fresh fact unlocks — replacing hand-placed one-off next-step hints with one home per fact. New guard-free rbfb_beckon.sh (Foundry Beckon, prefix rbfb — a fresh rbf child) holds rbfb_beckon_hallmark: keyed on RBF_FACT_HALLMARK, it composes the BUK buc_tabtarget primitive over the fact's full consumer roster — the rbch_palpate readers (summon rbw-fs, plumb compact+full rbw-fpc/fpf, rekon rbw-irh) with the just-written hallmark filled as folio, plus the rbch_enchase writers that resolve the same fact (anoint rbw-rva, drive rbw-nd) printed with a folio placeholder. Light context filtering (fill-or-placeholder), never a per-mode predicate farm. Wired on the success path of both hallmark producers: the director ordain tail (rbfd_ordain, covering conjure/bind/graft in one call with z_hallmark already resolved) and the kludge success tail (rbfk_kludge, keeping its orthogonal rbw-cKB/cKS charge hint). Both CLIs' furnishes source the new module. Spec: minted the rbch_beckon quoin in RBS0 Chaining-Fact Roles beside rbch_enchase/rbch_palpate, with mapping attribute and a definition that pins the roster to the two role enumerations directly above it (cite-so-they-cannot-drift cinch made local) and distinguishes beckon as a courtesy emission that resolves no chained value and carries no chaining band. Registered RBFB in the acronym map and added rbfb to the RBF children list. Word 'beckon' minted (operator-chosen; 'blazon' kept furrowed). Shellcheck 228 clean; theurge build green; no zipper change so no generated-file diffs.

### 2026-07-02 12:22 - ₢BiAAa - W

Unified the nameplate-hallmark drive into one durable-config chain-link, rbrn_drive — the fourth member on the rbch_enchase surface beside feoff/anoint/yoke, writing the rbrn_regime family. New guard-free rbrn_drive.sh resolves the hallmark express-or-chain (buf_elect_fact_capture), band-rejects BUBC_band_chain on broken resolve, rewrites one RBRN_{BOTTLE,SENTRY}_HALLMARK line addressed by moniker WITHOUT loading the regime (like feoff never loads its vessel — so a still-blank field the drive fills does not block enforce); no clean-tree gate, no-relay, loud on success. Replaced all three ad-hoc forms: (1) the local kludge rbw-cKB/cKS recomposed onto it (build-head + drive-link, one-shot preserved), retiring zrbob_drive_hallmark + ZRBOB_DRIVE_PREFIX; (2) the onboarding Rust reimpl rbtdro_drive_hallmark deleted, its 4 cloud propagation sites now invoke the real rbw-nd tabtarget via helper rbtdro_drive_nameplate; (3) the handbook hand-edits (airgap/moriah + bind/pluml tracks) now invoke rbw-nd instead of instructing a manual rbrn.env edit. Cloud path gained the operator drive verb it never had: colophon rbw-nd, folio=nameplate + bottle|sentry field + optional express hallmark. Spec: minted rbtgo_nameplate_drive quoin + RBS0 === section + RBSND-nameplate_drive.adoc subdoc beside feoff/yoke, upgraded the rbch_enchase enumeration from the descriptive phrase to the quoin; new RBCC_verb_drive tinder -> generated RBTDGC_VERB_DRIVE. Theurge verification: new rbtdrh_drive_broken_chain reveille case (anoint-shaped — real tadmor folio, empty BURV, asserts band 105 + byte-identical rbrn.env under reject). Cross-heat reconcile: ₢BbAAT had already pre-anticipated the drive as the fourth member in BOTH the RBS0 enumeration and the companion memo, so no reconcile was needed. All Done-when met: build green; shellcheck 227 clean; fast qualify green; reveille 117/0/0 (drive band case + furnish-invariant pass, no feoff/yoke/anoint regression); siege 62/0/0 (recomposed local kludge built+drove tadmor's vessels end-to-end, charged, 61 containment cases). Pace code committed at 42ee06b1a; the two later commits (tadmor kludge hallmarks) are ordinary siege test residue. The 'drive' verb/colophon is a mid-heat ashlar-surface mint (operator-ratified) flagged for Fable's terminal review per the gird/jilt precedent.

### 2026-07-02 12:08 - ₢BiAAa - n

Unify the nameplate-hallmark drive into one durable-config chain-link (rbrn_drive), joining feoff/anoint/yoke on the rbch_enchase surface. New guard-free rbrn_drive.sh primitive resolves the hallmark express-or-chain (buf_elect_fact_capture), band-rejects BUBC_band_chain on broken resolve, and rewrites one RBRN_{BOTTLE,SENTRY}_HALLMARK line in a target nameplate addressed by moniker (never loaded — like feoff never loads its vessel — so a still-blank field the drive fills does not block enforce); no clean-tree gate, no-relay, loud on success. Sourced by rbrn_cli (new operator verb, colophon rbw-nd, folio=nameplate + bottle|sentry field + optional express hallmark) and by rbob (local kludge rbw-cKB/cKS recomposed to build-head + drive-link, staying one-shot; retired the ad-hoc zrbob_drive_hallmark + ZRBOB_DRIVE_PREFIX). rbrn_cli furnish given a lean drive path (rbgc for RBF_FACT_HALLMARK + buf_fact) and skips the nameplate auto-load for the drive. Cloud path: onboarding Rust now invokes the real rbw-nd tabtarget (helper rbtdro_drive_nameplate) at all 4 propagation sites instead of the removed rbtdro_drive_hallmark reimpl + orphaned RBTDRO_HALLMARK_VAR_* consts. Theurge verification: new rbtdrh_drive_broken_chain reveille case (anoint-shaped: real tadmor folio, empty BURV, asserts band 105 + byte-identical rbrn.env under reject). Spec: minted rbtgo_nameplate_drive quoin + RBS0 === section + RBSND-nameplate_drive.adoc subdoc beside feoff/yoke, upgraded the rbch_enchase enumeration from the descriptive phrase to the quoin. New RBCC_verb_drive tinder -> generated RBTDGC_VERB_DRIVE. Handbook hand-edits gone: airgap (moriah) + bind (pluml) tracks now invoke rbw-nd instead of instructing a manual rbrn.env edit. Regenerated rbtdgc_consts.rs + tabtarget-context.md via the build; acronym registry + theurge-ifrit context doc updated.

### 2026-07-02 11:31 - ₢BiAAg - W

Barrier satisfied and verified — no code work of its own. Both named upstream paces are wrapped and confirmed in this clone's HEAD ancestry via merge-base --is-ancestor: ₢BfAAz (₣Bf terrier-scaffold-retirement, ₣Bf's last zipper+RBS0 writer under the 260630 rework) at W commit 767968601, and ₢BlAAL (₣Bl onboarding-reliquary-probe helper) at W commit 10e782cce. Both reached this clone through the origin/main merge 50dbeabec; the full federation zipper+RBS0 state and the settled onboarding crate are present here. Done-when fully met — mirrors the earlier sibling barrier ₢BfAAq. The gate now lifts on the next pace, the unified nameplate-hallmark drive-link, which rails last by design.

### 2026-07-02 03:48 - ₢BiAAl - W

Migrated all 10 clean-tree gate call sites to the well-formed precision-band variant bug_require_clean_tree_creed and retired the malformed bug_require_clean_tree. Sites now pass a creed (rationale) instead of a verb-name context: 3 image builds (conjure/mirror/kludge) share RBCC_creed_clean_build; 4 Lode captures (ensconce/conclave/immure/underpin) share the new RBCC_creed_clean_capture; inscribe and affiance got their own new creeds; the VOK parcel-release site (10th, absent from the docket's 9-RBK list but forced in by the removal criterion) carries an inline creed. Removed the now-dead 2nd verb-constant group (RBCC_verb_affiance..underpin, gate-only); left the first-group registry untouched. Updated 4 spec .adoc refs (RBSAC/RBSLE/RBSMF/RBSRT), the BUK core doc (acronym + Tool Git Discipline prose), the ACG analogy, and the spine's stale gate-name comment. Verified: shellcheck 224 clean, reveille 116/0/0, and the fire path proven live end-to-end (kludge on a dirty tree rejects exit 108 with the clean_build creed). Band-108 formal fixture remains theurge-stream territory per the paddock. Code notched at 7e4fb20f7.

### 2026-07-02 03:45 - ₢BiAAl - n

Migrate all clean-tree gate call sites to the well-formed precision-band variant bug_require_clean_tree_creed and retire the malformed bug_require_clean_tree. Ten live call sites now pass a creed (rationale) instead of a verb-name context: 3 image builds (conjure/mirror/kludge) share RBCC_creed_clean_build; 4 Lode captures (ensconce/conclave/immure/underpin) share the new RBCC_creed_clean_capture; inscribe and affiance each get their own new creed (RBCC_creed_clean_inscribe, RBCC_creed_clean_affiance); the lone VOK site (vob_build parcel release) carries an inline creed. Retired the malformed bug_require_clean_tree function (BUG) and the now-orphaned 2nd verb-constant group (RBCC_verb_affiance..underpin, minted solely for that gate); left the first-group registry (anoint/inscribe/kludge/ordain/yoke) untouched as a pre-existing catalog. Updated the creed-tinder comment, the spine's stale gate-name comment, 4 spec .adoc refs (RBSAC/RBSLE/RBSMF/RBSRT), the BUK core doc (acronym entry + Tool Git Discipline prose), and the ACG analogy line. Fire path proven end-to-end: kludge on a dirty tree rejects exit 108 with the clean_build creed. Shellcheck 224 files clean.

### 2026-07-01 20:06 - ₢BiAAj - W

Drained heat ₣A-'s jurisdiction design substance into the mews aspirant sheaf (JJS-aspirant-mews.adoc) as a new '== The jurisdiction inheritance' section, sorting the operational plan by fate: invariants that re-gestate intact (two-records/one-identity split, three-verb division of labor, destructive ownership, operator-owned keys, handbook boundary, self-contained nodes, transport posture), surveyed Windows Palisade facts that must not be re-derived (two-phase SSH trust, admin-no-WSL workload-owns-distro, reboot-as-state-reset, headless-account absolute-path requirement, one-daemon-per-host, plus the two parked defects from the paddock's standing notes), and the residual forks reopened at mews-time (store carrier, dispatch surface, multi-station trust, RB-side handbook lineage). Veiling disposition cross-noted in its own subsection; Open-forks bullet narrowed to point at the itemized residual forks; Sources updated with the orientation-map memo, the parked orchestration-style AXLA draft, and the Windows transport memo family. ₣A- archived (trophy jjh_b260412-r260701-rbk-30-win-windows-remote-control.md) with the size guard raised to 800000 on operator confirm for the 71-pace heat record.

### 2026-07-01 20:04 - ₢BiAAj - n

Drained heat ₣A-'s jurisdiction design substance into the mews aspirant sheaf as design concept. New '== The jurisdiction inheritance' section reduces the operational plan by fate: the invariants that re-gestate intact (the two-records/one-identity BURN-BURP split, the three-verb establish/verify/provision division of labor, destructive ownership with the rejected account-lifecycle verb zoo, operator-owned keys with their held corollaries, the handbook boundary for security-sensitive edits, self-contained nodes with converge-against-known-host demoted to a sequencing choice, and the bash-conducts transport posture); the surveyed Windows Palisade facts that must not be re-derived (structural two-phase SSH trust, workload-owns-its-WSL-distribution admin-no-WSL shape, reboot as the canonical Windows state reset, the GetUserProfileDirectoryW headless-account absolute-path requirement, one container daemon per Windows host, and the two parked defects — UTF-16LE workload stdout without $env:WSL_UTF8=1, orphan profile-suffix minting — that lived only in the paddock's standing notes); and the residual forks reopened at mews-time (store carrier, dispatch surface, multi-station trust via the deferred adopt/user-regime pair, the RB-side handbook lineage, and the still-deferred PowerShell letter / localhost variants / per-purpose keypair). Veiling disposition cross-noted in its own subsection (apparatus inert in Tools/{buk,rbk}/vov_veiled/, enrollments and tabtargets removed, re-gestation through the sheaf never by re-enrolling). Open-forks bullet narrowed to point at the itemized residual forks; Sources updated with the orientation-map memo, the parked orchestration-style AXLA draft, and the Windows transport memo family, with the ₣A- entry flipped to archived/drained. Ceremony step lists, directive sets, and the colophon inventory deliberately not carried — the veiled tree and git history hold them.

### 2026-07-01 19:43 - ₢BiAAi - W

Shelved the BUK jurisdiction apparatus (BURN/BURP regimes + caparison/invigilate/garrison) and the RBK Windows handbook off the distributed/canonical surface. Reframed mid-pace from the docket's preserve-execution reading to shelve-entirely per operator: git mv 9 BUK + 6 RBK modules into vov_veiled/ (inert, git-recoverable, NOT rewired for local execution — the mews re-gestates); removed all jurisdiction/windows zipper enrollments (buwz + rbz) and 23 tabtargets. Surviving-surface repairs: dropped orphaned buh_index_buk(); pruned the 2 regime-poison cases + 4 handbook-render cases + their manifest coverage entries that referenced the shelved regimes/colophons; regenerated rbtdgc_consts.rs + claude-rbk-tabtarget-context.md; annotated the veiled disposition in claude-buk-core.md + claude-rbk-acronyms.md (bubc jurisdiction constants left inert, flagged). rbw-o remains the handbook entry. Green: fast qualify, BUK self-test 49, reveille 116, regime-poison 29, shellcheck 224. Commits 742707e5b + 2e0cd08d9. (Sibling BiAAj prepped for a Fable drain: 4 rough ₣A- paces abandoned + docket reslated to the plan→design-concept reduction.)

### 2026-07-01 19:41 - Heat - d

batch: 1 reslate

### 2026-07-01 19:32 - ₢BiAAi - n

Remove the orphaned rbw-h0 HandbookTOP tabtarget — the master handbook index accelerator, whose colophon was de-registered when the windows handbook module (its host) was veiled. Straggler caught by fast qualify (colophon-not-registered). rbw-o remains the handbook entry point. Fast qualify now green.

### 2026-07-01 19:32 - ₢BiAAi - n

Shelve the BUK jurisdiction apparatus + RBK Windows handbook off the distributed/canonical surface. Veil-not-delete: git mv the 9 BUK jurisdiction modules (burn/burp regimes+CLIs, bujb/buhj jurisdiction, bujp preflight) into Tools/buk/vov_veiled/ and the 6 RBK windows-handbook modules (rbhw* — incl. the cross-group rbw-h0 HandbookTOP, which was misfiled inside the windows module) into Tools/rbk/vov_veiled/. Inert, git-recoverable; NOT rewired for local execution (execution deliberately not preserved per operator; the mews aspirant sheaf re-gestates). Removed every jurisdiction zipper enrollment (buwz Node/Privileged-regime, Handbook-jurisdiction, workload/caparison/privileged-ssh/garrison blocks; rbz handbook group) and the 23 jurisdiction/windows tabtargets. Surviving-surface repairs: dropped orphaned buh_index_buk(); removed the 2 regime-poison cases that validate the now-shelved BURN/BURP regimes + the 4 windows cases in the handbook-render fixture + their manifest coverage entries; regenerated rbtdgc_consts.rs + claude-rbk-tabtarget-context.md via the theurge build (node/privilege BUWGC_ + RBK handbook RBTDGC_ consts dropped). Annotated the veiled disposition in claude-buk-core.md + claude-rbk-acronyms.md; bubc jurisdiction constants left inert on the core surface (flagged). rbw-o remains the handbook entry point. Theurge crate compiles clean.

### 2026-07-01 16:55 - Heat - d

batch: 1 reslate

### 2026-07-01 16:54 - Heat - d

batch: 1 reslate

### 2026-07-01 16:35 - ₢BiAAp - W

Formalized on-charge usage documentation as a declarative rbnnh_charge_note affordance — any nameplate earns a usage note by dropping one file. Minted the third RBNNH quoin in RBSCH (+ RBS0 mapping attribute) with its own completion clause making an unknown/malformed placeholder fatal. Charge core (rbob) gained zrbob_render_charge_note: reads the note as data (never sourced), resolves {{NAME}} tokens against a closed one-member vocabulary (RBRN_ENTRY_PORT_WORKSTATION) via bash-native ${!name} indirect expansion after vocab validation, buc_die on unknown token or residual doubled-brace, BCG-clean emit via buc_step/buc_info at the charge tail AFTER the post-charge hook (so a hook precondition failure suppresses the note). Migrated all three cases: srjcl URL->note with hook slimmed to the conditional ANTHROPIC_API_KEY check; pluml gained a note (no prior affordance); nineveh hook->note, hook deleted (was all unconditional display). Gates green: shellcheck 221 clean, nameplate audit clean, a verbatim-logic render harness proved typo/unknown/malformed placeholders all buc_die loud (exit 3) while good/nineveh notes resolve with single-brace {a->b} safe, and a live nineveh charge surfaced the note end-to-end at the real tail (placeholder resolved to 8006, no hook step, BCG-clean). Vocabulary deliberately seeded with the single used member per Load-Bearing Complexity; widening is a one-line arm + RBSCH line.

### 2026-07-01 16:20 - ₢BiAAp - n

Formalize on-charge usage documentation as a declarative affordance (rbnnh_charge_note). Mint the third RBNNH member in RBSCH — a per-nameplate plain-text usage note at «moniker»/rbnnh_charge_note.txt, sibling to the compose fragment and post-charge hook — with its own completion clause (malformed/unknown placeholder is fatal). Charge core (rbob) gains zrbob_render_charge_note: reads the note as data (never sourced), resolves {{NAME}} tokens against a closed one-member vocabulary (RBRN_ENTRY_PORT_WORKSTATION) via bash-native ${!name} indirect expansion after vocab validation, buc_die on unknown token or residual doubled-brace, emits BCG-clean via buc_step/buc_info; invoked at charge tail AFTER the post-charge hook so a hook precondition failure suppresses the note. Migrated all three cases: srjcl's JupyterLab URL -> note (hook slimmed to the conditional ANTHROPIC_API_KEY check only); pluml gains a note (no on-charge affordance before); nineveh's freeform hook -> note, hook deleted (was all unconditional display). RBS0 mapping registers the rbnnh_charge_note attribute.

### 2026-07-01 16:04 - ₢BiAAo - W

Nineveh kroki render crucible proven end-to-end. Re-vouched the already-mirrored hallmark b260701131017-r260701201021 (cloud vouch build succeeded 1m19s under the raised poll ceiling; rekon confirms image+about+vouch present), seated it in nineveh RBRN_BOTTLE_HALLMARK replacing the PENDING-ORDAIN-BIND placeholder, revalidated the nameplate, and notched (f4b6d8a7c). The docket's first remaining item — the tag@digest pin on RBRV_BIND_IMAGE — was already landed by sibling pace BiAAq, so work started at the re-vouch. Charged nineveh (all three containers healthy, image auto-summoned, post-charge hook printed the usage note); POSTed 'digraph { a -> b }' to /graphviz/svg on workstation port 8006 and got a well-formed SVG (graphviz 14.1.3). Also unfurled that SVG to the paneboard viewer. Crucible left charged.

### 2026-07-01 16:01 - ₢BiAAo - n

Seat the real bottle hallmark in nineveh. The ordain-bind cloud mirror ran and the hallmark b260701131017-r260701201021 is now fully vouched in GAR (image+about+vouch present, confirmed via rekon after re-vouch against the digest-pinned RBRV_BIND_IMAGE), so RBRN_BOTTLE_HALLMARK replaces the PENDING-ORDAIN-BIND placeholder. Comment updated to past-tense (mirror done). Nameplate regime revalidates clean.

### 2026-07-01 15:03 - ₢BiAAq - W

Bind-digest fail-fast gate landed (canonical upstream form after twin-strand reconciliation): zrbrv_enforce rejects a bind RBRV_BIND_IMAGE lacking an end-anchored @sha256:<64-hex> digest with band 100 at rbw-rvv time, instead of ~8 min into the cloud build at the vouch's verify-provenance. RBSRV homes the invariant at rbrv_bind_image and censuses it in the rbkro_validate block. regime-poison gained rbrv-bind-image-tag-only (tag-pinned pluml FQIN clears fqin enroll, trips the gate, asserts exact band 100) + RBTDRS_VESSEL_PLANTUML const. ZRBFC_BUILD_POLL_CEILING_VOUCH 50->150. Operator-directed beyond the docket: digest-pinned the kroki bind vessel (multi-arch index digest, tag retained for legibility), superseding BiAAo's tag-only deviation and clearing the standing reveille red. Verified: reveille 119/119 (first fully-green since the gate), regime-poison 31/31, shellcheck 222, jjx_validate clean, pushed.

### 2026-07-01 15:01 - ₢BiAAq - n

Reconcile the twin BiAAq executions: this clone's duplicate gate/ceiling/spec commit was dropped in rebase (upstream 6009125de is canonical — its 64-hex end-anchored regex is stricter than the beta glob and BlAAP/BoAAC stack on it), keeping only the two additive pieces. This commit re-lands the unique remainder: the regime-poison bind-digest case rbrv-bind-image-tag-only (tag-pinned pluml FQIN clears buv_fqin enroll, trips the canonical zrbrv_enforce digest regex, asserts band 100) with the RBTDRS_VESSEL_PLANTUML const extracted now that the pluml folio is referenced by 2+ cases — per the docket's Done-when, deliberately overriding upstream's defer-to-Bl reading since the paddock's stream boundary covers chaining-band verification, not this fixture's own RBRV band cases. Plus the RBSRV rbkro_validate operation block gains the bind digest-pin line (upstream homed the invariant at rbrv_bind_image but left the validate census stale). The kroki digest-pin (27d24693a) rode the rebase intact and satisfies the stricter regex. Theurge crate compiles.

### 2026-07-01 14:20 - ₢BiAAq - n

Digest-pin the kroki bind vessel to satisfy the new bind=>digest gate (operator-directed reconciliation). RBRV_BIND_IMAGE gains the multi-arch index digest @sha256:c16303... (resolved daemon-free from the Docker registry v2 API; content-type oci.image.index confirms the whole manifest list), retaining the :0.31.0 tag for legibility. Supersedes BiAAo's operator-approved tag-only deviation: the gate reddened reveille's rbrv_all_vessels sweep, and the operator chose to pin now rather than carve out a mirror exception. Comment rewritten to describe the tag@digest form per the RBSRV invariant.

### 2026-07-01 14:00 - Heat - d

batch: 2 reslate

### 2026-07-01 13:54 - ₢BiAAq - n

Bind-digest fail-fast gate + vouch poll-budget bump. zrbrv_enforce now rejects a bind vessel whose RBRV_BIND_IMAGE lacks an @sha256:<64-hex> digest with buc_reject BUBC_band_regime (100) — catching at rbw-rvv/reveille config time what previously only surfaced ~8 min into a cloud build at the vouch's verify-provenance ('no @ in BIND_SOURCE'). Modeled on zrbrn_enforce custom checks; reuses band 100 (regime-module custom enforce), no new band. RBSRV homes the bind-implies-digest-pinned invariant. Bumped ZRBFC_BUILD_POLL_CEILING_VOUCH 50->150: a large multi-arch bind vouch runs ~8 min, and 50 (~4 min) false-timed-out the local poller even when the GCB build succeeded. Verified: rbw-rvv rejects kroki (exit 100), passes pluml (0); full vessel sweep isolates kroki as the sole tag-only offender; shellcheck 222 clean. Regime-poison case deliberately NOT added here — band-asserting fixtures are theurge-stream (Bl) territory per the paddock.

### 2026-07-01 13:46 - Heat - S

bind-digest-gate-vouch-budget

### 2026-07-01 13:08 - ₢BiAAo - n

Stand up nineveh kroki-core render crucible config (trial). Bind vessel rbev-bottle-kroki mirrors docker.io/yuzutech/kroki:0.31.0 — pinned by version tag, not a bare @sha256 digest: the mirror gcrane-copies the whole multi-arch manifest list and the GAR hallmark is digest-locked regardless, so the committed selector stays legible (operator-approved deviation from the pluml bare-digest exemplar; docket edit skipped by operator). Nameplate nineveh clones pluml posture: ws 8006 -> enclave 8000, subnet 10.242.6.0/24, reused rbev-sentry-deb-tether sentry hallmark, bottle hallmark PENDING-ORDAIN-BIND placeholder (real b...-r... comes from the cloud ordain-bind, deferred). rbnnh_post_charge.sh prints kroki core-engine paths + a graphviz POST sample (srjcl hook exemplar). Charge/quench tabtargets, no theurge fixture (ccyolo posture). All static validators green: vessel/nameplate regime, cross-nameplate audit, fast qualify, shellcheck 222 clean.

### 2026-06-30 20:41 - Heat - S

charge-note-formalization

### 2026-06-30 20:29 - Heat - S

nineveh-kroki-trial-crucible

### 2026-06-30 15:36 - Heat - d

batch: 1 reslate

### 2026-06-30 14:23 - ₢BiAAm - W

Conjure resolved-base provenance via build-time signed label (b2). Four legs across six commits (cf333f54f→6cbc0124c): (1) write — new gcrane cloud step rbgjb03-resolve-base-digests resolves each populated base slot's tag to an @sha256: digest, rbgjb04 pins the build-arg to it (buildx builds FROM exactly that) and emits a sprued rbi_resolved_base_n image label; steps renumbered 04-07; director shebang baker gains a busybox entrypoint; RBGC_IMAGE_LABEL_RESOLVED_BASE constant. (2) read — new registry-raw zrbfc_image_config_fetch helper; plumb surfaces the resolved base from the SIGNED attest image's config blob in the Base Image section. (3) specs+rivet — RBSAC specs pin+label, RBSAP specs readback, RBr_b4e (axvc_spoor) defines the two foreign-behavior reliances (label survives the per-platform pullback into the attest ark; google-worker DSSE signs that attest image) each with a demolition condition. (4) regression — dogfight case reads rbi_resolved_base_1 off the summoned image and fails loud on divergence from committed RBRV_IMAGE_1_ORIGIN. Host bash made BCG-compliant per operator directive (stderr to temp files, helper not _capture, null-safe jq); cloud steps follow CBG. Fully live-verified against the depot: dogfight passed, manual ordain->plumb->abjure showed plumb rendering 'Resolved base 1 (signed): busybox@sha256:fd8d9aa6...' from the attest image; that digest cross-confirmed three ways (memo's buildkit attestation, dogfight consumer-image read, plumb attest-image read). Shellcheck clean (221 files), theurge 156 unit tests pass, RBr_b4e grep-registry consistent (1 definition, 5 citations).

### 2026-06-30 13:55 - ₢BiAAm - n

Resolved-base provenance leg 4 (regression case): extends the dogfight fixture's ordain->summon lifecycle to read the rbi_resolved_base_1 label off the summoned image and fail loud on divergence from committed config. New rbtdrv_docker_config_label helper (docker inspect --format index .Config.Labels, no serde dependency). Asserts the label is present, @sha256:-pinned with a well-formed 64-hex digest, and its ref-portion equals the vessel's committed RBRV_IMAGE_1_ORIGIN with its tag stripped (the exact ${origin%:*} transform rbgjb03 applies) — a mismatch is a provenance lie. Reuses the existing cloud build rather than spending a second on a separate case. Crate compiles (rbw-tb clean); the case itself runs live (dogfight is credless:false, operator-gated).

### 2026-06-30 13:48 - ₢BiAAm - n

Resolved-base provenance leg 3 (specs + rivet): RBSAC specs the cloud digit-pin (host resolves tag-level, cloud step resolves to @sha256:, build-arg pinned so buildx builds FROM exactly it) and the sprued rbi_resolved_base_n label emission; RBSAP specs the readback (new attest-image config-blob fetch step + resolved-base in the conjure Base Image section). Mints rivet RBr_b4e (axvc_spoor, foreign-behavior signature) defined in RBSAC: the label's trust rests on two surveyed behaviors at the GCB/buildx Palisade — buildx labels survive the per-platform pullback into the attest ark byte-identically, and the google-worker DSSE signs that attest image (not the consumer image) — each with its demolition condition. Cited at the write site (rbgjb04) and the read site (plumb, RBSAP); one definition, grep is the registry.

### 2026-06-30 13:39 - ₢BiAAm - n

Resolved-base provenance leg 2 (read side): plumb reads the rbi_resolved_base_n labels back from the SIGNED attest image's config blob and surfaces them in the conjure Base Image section as the actual digest-pinned base (vs the generic FROM template). New registry-raw helper zrbfc_image_config_fetch (manifest -> .config.digest -> blob, single-platform-resolve, returns 1 on 404) parallels zrbfc_gar_extract_artifact; plumb derives the attest tag from the first built platform. Cites rivet RBr_b4e (labels survive the pullback; google-worker DSSE signs the attest image). BCG-compliant: helper is not _capture (writes to a file), all jq stderr to temp files (no 2>/dev/null), null-safe (.config.Labels // {}) read, graceful buc_warn with permission comment.

### 2026-06-30 13:25 - ₢BiAAm - n

Resolved-base provenance leg 1 (write side): conjure resolves each populated base slot's tag to an @sha256: digest in a new gcrane cloud step (rbgjb03-resolve-base-digests), writing the pinned ref to .resolved_base_n; the buildx step uses each pinned ref twice — as the RBF_IMAGE_n build-arg (so buildx provably builds FROM exactly the resolved digest) and as the sprued rbi_resolved_base_n image label. Renumbered buildx/pullback/push/diags steps to rbgjb04-07; director shebang baker gains a busybox entrypoint for the gcrane :debug builder; RBGC_IMAGE_LABEL_RESOLVED_BASE homes the sprued label-key prefix. Shellcheck clean (221 files).

### 2026-06-30 12:56 - Heat - d

batch: 1 reslate

### 2026-06-30 12:56 - ₢BiAAm - n

Capture the conjure resolved-base provenance grounding: live-depot survey proving the signed google-worker attest provenance carries only build-step tooling (not the FROM base), the FROM base lives in the unsigned buildx-native attestation, and the settled b2 way forward (build-time signed image label, cloud-resolve + pin + label, verified to survive the per-platform pullback into the signed attest image)

### 2026-06-30 12:02 - Heat - S

fact-next-tabtarget-emitter

### 2026-06-30 17:32 - Heat - d

batch: 1 reslate

### 2026-06-27 14:58 - ₢BiAAW - W

Design ratified for the conjure resolved-base provenance gap. Finding: conjure's resolved base is ALREADY recorded — in the BuildKit provenance attestation on the -image ark (.SLSA.buildDefinition.resolvedDependencies: uri + digest.sha256), confirmed present live on a real hallmark (260627). It is captured-from-actual-resolution (BuildKit's own record, never config-echo) and distinct from the GCB GoogleHostedWorker provenance vouch verifies, which carries no base. Fix: plumb interrogates that attestation and emits a scalar rbf_fact_ resolved base; a theurge regression asserts built-base vs committed config. Vouch untouched; sprue-free (foreign source, scalar sink); one canonical home (the GAR attestation, the fact a transient cache); no build-pipeline change. Actionable spec lives in the cantled build pace BiAAm docket — pointed to in lieu of a separate artifact, per operator direction.

### 2026-06-27 14:57 - Heat - S

resolved-base-readback-and-verify

### 2026-06-27 14:09 - Heat - S

clean-tree-gate-callsite-migration

### 2026-06-27 11:20 - ₢BiAAH - W

Retired the superseded GCS build bucket. A three-way enumeration (code / spec / adversarial-supersession) confirmed no live consumer: no storageSource anywhere, both build-submission paths (rbfd_director conjure, rblds_spine capture) submit builds.create with no source field, context delivered solely via the GAR pouch. Cut 14 files across three commits (two notches + this wrap): bucket creation + 1-day lifecycle and Mason's storage.objectViewer grant at depot levy (rbgp_payor); the RBDC_GCS_BUCKET / RBGD_GCS_BUCKET derivations + the three dead RBGD_API_GCS_BUCKET_{OPS,OBJECTS,IAM} constants (keeping the live generic _CREATE used by governor/terrier); the RBS0 rbtgi_build_bucket quoin + all four citations; the RBSMF «BUILD_BUCKET» store step, Create-Build-Bucket levy step, Mason grant line, and two prose mentions; RBSDU/RBSGS composition mentions; the RBRD regime meaning strings + derivation descriptions in rbrd_regime/RBSRT/rbrd.env (surfaced by the depot render, which the grep-based enumeration missed); and two stale theurge doc comments (rbtdrp_attest, rbtdtk_freehold). rbtgi_build_bucket now resolves to zero references; axig_bucket / gcs_bucket / rbtoe_iam_grant_bucket all survive via other quoins; generic terrier/governor bucket refs (RBSCB, RBS0 setIamPolicy) correctly retained, and depot storage-API enablement kept (governor still creates a depot-project bucket via _CREATE). Verified green within the change's blast radius: shellcheck 217 clean, depot regime render (rbdc/rbgd kindle clean post-removal), all RBRD depot-poison cases, handbook-render 12/12, conformance 5/5, recipe-validation 10/10. The cross-clone RBS0 adjacency the docket predicted holds exactly (depot + build_bucket edits bracket FED's rbtgi_manor) — resolve by hand at convergence, no barrier, per the cinch.

### 2026-06-27 11:20 - ₢BiAAH - n

Build-bucket residue in theurge crate comments, the same prose-string class the depot-regime sweep missed (enumeration searched constants, not doc-comment text): drop the now-dead bucket mention from the two RBDC-derivation comments — "and bucket" from RBTDRP_RBRD_BLANK_FIELDS' site-identity list (rbtdrp_attest.rs, project ID/GAR repo/pool stem still derive from CLOUD_PREFIX + DEPOT_MONIKER), and "GCS buckets" from the freehold dual-station disjoint-derivation comment (rbtdtk_freehold.rs). Comment-only; no code or behavior change.

### 2026-06-27 11:10 - ₢BiAAH - n

Build-bucket residue the depot-regime render surfaced (the enumeration agents searched constants/storageSource, not prose meaning strings): drop 'GCS bucket'/'and bucket' from the RBRD_CLOUD_PREFIX and RBRD_DEPOT_MONIKER meaning strings in rbrd_regime.sh, the matching RBSRT derivation-tuple sentence (parallel to the line already fixed), and the same derivation comment in the live rbrd.env. Generic-bucket mentions (RBSCB control-plane deletes, RBS0 setIamPolicy 400) correctly left — those name terrier/governor buckets, not the build bucket.

### 2026-06-27 11:06 - ₢BiAAH - n

Retire the superseded GCS build bucket. A three-way enumeration (code, spec, adversarial supersession check) confirmed the bucket is fully dead: no storageSource anywhere, both build-submission paths (rbfd_director conjure, rblds_spine capture) submit builds.create with no source field, context delivered solely via the GAR pouch. CODE: removed the build-bucket creation block + 1-day lifecycle and Mason's storage.objectViewer grant at depot levy (rbgp_payor.sh), the RBDC_GCS_BUCKET derivation (rbdc_derived.sh), the RBGD_GCS_BUCKET alias + the three dead RBGD_API_GCS_BUCKET_{OPS,OBJECTS,IAM} constants (rbgd_depot.sh, keeping the live generic _CREATE used by governor/terrier), and the two contrast-comments (rbgb_buckets.sh). SPECS: removed the RBS0 rbtgi_build_bucket quoin (anchor+def+both mapping refs), its four citations (Mason prose, rbtgi_depot constituent, RBSDU unmake cascade, RBSGS composition), and dropped RBDC_GCS_BUCKET from the regime-derivation line in RBS0 + RBSRT; removed the «BUILD_BUCKET» store step, the Create Build Bucket levy step, the Mason bucket-grant line, and two prose mentions in RBSMF. Orphan-checked: axig_bucket, gcs_bucket, rbtoe_iam_grant_bucket all survive via other quoins; rbtgi_build_bucket goes to zero references. Depot storage-API enablement retained (governor still creates a depot-project bucket via _CREATE).

### 2026-06-27 10:47 - Heat - T

chaining-roles-selection-adjacency

### 2026-06-27 10:02 - ₢BiAAI - W

Concluded by OVERTURN (the docket authorized confirm-or-overturn; reconfirm was the deliverable). Re-verified from sources: bug_require_clean_tree (BUG) already takes a context param and already surfaces the verb name ('commit before conjure') — the actionable part. The per-gate 'why' is already canonically spec-homed by the audit-driven BHAAR pass: RBSAC (conjure), RBSLE + three sibling pointers (the four captures), RBSAG (graft's deliberate ungated exception), RBSDE/RBSRT (tripwire), RBSMA (affiance). The nine gating sites resolve to two rationale families (provenance-by-construction; committed-reference-anchor: tripwire+affiance) plus graft's non-provenance exception. Surfacing the non-actionable why as rbcc string constants would duplicate spec-homed rationale into a drift-prone second runtime home (ACG one-home), on a gate the paddock pins as an ergonomic backstop — never the safety mechanism (that is the no-relay chaining discipline). Verdict: no rbcc rationale constants, no second gate param — design does not warrant implementation. Affiance's flat rbcc grouping needs no correction absent a rationale taxonomy we are not building; the RBSRT/RBSMF tripwire near-duplication stays as the docket marked it (separable, not required). No code changed.

### 2026-06-27 09:30 - Heat - S

chaining-roles-selection-adjacency

### 2026-06-27 09:17 - ₢BiAAG - W

Made scry (rboo_observe.sh, rbw-cs) a trustworthy scriptable network-diagnostic primitive. Four docket repairs: (1) resolve sentry interface roles by IP inside the container (ip -o addr, like rbjs_sentry) instead of hardcoded eth0/eth1; (2) add -e for L2 MACs; (3) capture BOTH sentry legs (enclave + uplink), not enclave-only; (4) optional bounded duration + tcpdump filter that runs captures under timeout and returns 0 for scripted use, interactive run-until-Ctrl+C unchanged when no duration. Cleanups: collapsed the three copy-paste prefix fns into one parametrized zrboo_prefix; fixed a latent podman-bridge bug (${ARR} -> ${ARR[*]} was passing only -U). Live-verified on a real charged tadmor crucible: discovery resolved enclave=eth1/uplink=eth0 correctly, both sentry legs + pentacle captured enclave ICMP with MACs, bounded 6s run returned cleanly. Added an RBK conduct membrane so future instances reach for scry now that the bounded form makes it agent-drivable (and avoid the bare-form-hangs trap). Shellcheck 217 clean.

### 2026-06-27 09:06 - ₢BiAAG - n

Add an RBK conduct membrane for scry now that it is agent-drivable: reach for it to debug a charged crucible's traffic (bounded <duration> [filter], both sentry legs + pentacle, L2 MACs, exits 0), and never call the bare no-duration form from a tool call (it hangs until timeout). Closes the loop the pace opened — the prior avoidance was correct for the run-forever tool; this nudges future instances now that the bounded form makes it usable.

### 2026-06-27 08:58 - Heat - n

Re-blank tadmor's sentry/bottle hallmarks — restore the deliberate marshal-baseline (empty, per 7cee441ec on ₣Bl) after the two transient local-kludge hallmarks were committed only to charge a crucible for live verification of the scry repairs. tadmor returns to the honest interim state; the local dev images remain on this machine but the repo no longer references them.

### 2026-06-27 08:55 - Heat - n

TRANSIENT crucible-charge state (not scry work): bottle kludge drove RBRN_BOTTLE_HALLMARK=k260627085530-28d186710 into tadmor's rbrn.env. Committed only to satisfy the charge nameplate-clean gate for live verification of the ₢BiAAG scry repairs. tadmor to be re-zeroed (rbw-MZ) after.

### 2026-06-27 08:55 - Heat - n

TRANSIENT crucible-charge state (not scry work): sentry kludge drove RBRN_SENTRY_HALLMARK=k260627085245-8a48270ca into tadmor's rbrn.env. Committed only to satisfy the clean-tree gate so the bottle kludge + charge can proceed for live verification of the ₢BiAAG scry repairs. tadmor to be re-zeroed (rbw-MZ) after verification.

### 2026-06-27 08:47 - ₢BiAAG - n

scry: resolve sentry interfaces by role inside the container (ip -o addr, never hardcoded eth0/eth1), capture BOTH sentry legs (enclave + uplink) so the full path shows in one run, add -e for L2 MACs, and add an optional bounded duration + tcpdump filter for scriptable non-interactive capture (interactive run-until-Ctrl+C unchanged when no duration). Parametrized the three copy-paste prefix fns into one zrboo_prefix; fixed the latent podman-bridge opts bug (${ARR} -> ${ARR[*]} was passing only -U).

### 2026-06-27 08:36 - ₢BiAAF - W

RBSCJ: repointed both stale `zrbf_stitch_build_json` references (lines 17 prose + 55 What-Survives) to the live `zrbfd_stitch_build_json` (renamed in the foundry decomposition; confirmed at rbfd_director.sh:391); grep gate clean. RBSCB: re-derived the authoritative-specs sentence and confirmed RBSDC (depot_recognosce) correct AS-IS — a live operational spec and the coherent referent for a build posture that rests on depot-founding (mantle SAs, capability-sets, AR audit config); did NOT blind-substitute to the RBSDI dangle per the inverted-premise warning. The real RBSDI dangle (claude-rbk-acronyms.md:94 -> nonexistent RBSDI-depot_inscribe.adoc) left flagged for Bf, not fixed here.

### 2026-06-27 08:36 - ₢BiAAF - n

RBSCJ: refresh stale `zrbf_stitch_build_json` citations to `zrbfd_stitch_build_json` — track the function to its rbfd_director home after the Foundry decomposition (two sites: Decision rationale + What Survives).

### 2026-06-27 08:13 - ₢BiAAc - W

Minted rbch_enchase (Durable-Config Link) and rbch_palpate (Read-Side Chain Consumer) chaining-role quoins in RBS0, fact-general (no AXLA). Cited them from the four read subdocs (band-reject + distinction-by-effect) and the feoff/yoke/anoint write-side; converted rekon to band-reject for read-consumer consistency; retired durable-leak->durable-config naming, keeping leak only in the hazard-sense leak-elimination invariant; swept theurge comments + acronyms entry. Slated the rekon band-fixture in the theurge stream (Bl) and repaired the Bi paddock to match. Verified: grep gate clean, shellcheck 217 clean, theurge build green.

### 2026-06-27 08:13 - Heat - d

paddock curried: durable-leak->durable-config terminology + valence principle; rekon now band-converted (vouch the lone holdout); members three->four (nameplate drive); name the minted quoins

### 2026-06-27 08:08 - ₢BiAAc - n

Mint rbch_enchase/rbch_palpate chaining-role quoins in RBS0 (fact-general, no AXLA); cite them from the four read subdocs (band-reject + distinction-by-effect) and the feoff/yoke/anoint write-side; convert rekon to band-reject for read-consumer consistency; retire durable-leak->durable-config naming, keeping leak only in the hazard-sense leak-elimination invariant

### 2026-06-27 05:53 - Heat - f

racing

### 2026-06-26 10:55 - Heat - f

silks=rbk-01-mvp-review-loose-ends

### 2026-06-26 10:59 - ₢BiAAf - W

Cross-clone barrier lifted: verified ₢BfAAO (federation spec-first recast) wrapped on origin (commit 585f3afc2) and pulled into this clone, with the RBS0 federation block now present here (Federation Civics, Federation Regime RBRF, plus the new RBSFE/RBSFA contract subdocs). No code work of its own — a gate that has now passed; the durable-leak coarse quoins may append their disjoint RBS0 region behind the federation block.

### 2026-06-26 07:20 - Heat - f

stabled, silks=rbk-41-fbl-review-loose-ends

### 2026-06-26 07:05 - Heat - f

silks=rbk-04-mvp-loose-ends

### 2026-06-26 03:28 - ₢BiAAh - W

Extended the pace from RBS0+BUS0 to FOUR codices (added VOS0 and APCS0). Minted each codex identity quoin — rbs0_imprimatur / bus0_imprimatur / vos0_imprimatur / apcs0_imprimatur — as mapping-section attribute + Overview [[anchor]] + //axvd_codex voicing + definition. Marked all 105 sheaves with //axvd_sheaf <codex>_imprimatur axd_<status> at the head and //axvd_sheaf axd_<status> at each include-site, using the repaired comment-token mechanism (no rendered lookahead line): 103 normative + 2 informative (RBSHR HorizonRoadmap and RBSGS GettingStarted, each carrying a fair-faced informative banner). BUSJH0 treated as an ordinary BUS0 sheaf, not a codex (operator ruling). Verified: each codex quoin defined exactly once and sealed by axvd_codex, all 105 tendons resolve, prefixes grep-clean. NOT render-verified — no asciidoctor on this machine. The work is committed and correct in 66e3ad58a; it rode a concurrent operator commit (bundled with an unrelated memo) rather than a dedicated jjx_record, which the operator accepted.

### 2026-06-25 15:23 - Heat - S

drain-jurisdiction-heat-into-mews-sheaf

### 2026-06-25 15:22 - Heat - S

veil-jurisdiction-and-windows-apparatus

### 2026-06-25 14:33 - Heat - S

diffuse-codex-vocabulary-to-rbs-bus

### 2026-06-25 16:29 - Heat - d

paddock curried: integrity-audit fix: count four→five (5 cleanups listed); drop phantom 'build-poll clock split' from the ₣Bi→₣Bf drain list (no referent, FED received 3)

### 2026-06-25 16:25 - Heat - d

batch: 4 reslate

### 2026-06-24 23:34 - Heat - S

await-fed-zipper-and-thg-crate

### 2026-06-24 23:33 - Heat - S

await-fed-rbs0-block

### 2026-06-24 23:32 - Heat - T

mvp-rides-last-syncs-owners

### 2026-06-24 23:12 - Heat - S

mvp-rides-last-syncs-owners

### 2026-06-24 22:58 - Heat - d

paddock curried: restitch into the narrowed MVP-cleanup stream (split study): Character re-derived to the spec-and-cleanup remainder, robustness band + coupling-to-watch drained to the federation stream, chaining-fact discipline kept with its verification cross-ref re-pointed to the sibling theurge stream

### 2026-06-24 22:46 - Heat - d

batch: 1 reslate

### 2026-06-24 13:52 - ₢BiAAd - W

Converted anoint's three broken-chain buc_die (hallmark, gar_root, ark_stem in rbfla_anoint.sh) to buc_reject BUBC_band_chain (105), making the last write-side holdout uniform with feoff/yoke; structural dies (vessel-required, not-graft, write-failures) left as buc_die per the chain-vs-structural split feoff keeps; comment updated off the loud-die claim. Added reveille case rbtdrh_anoint_broken_chain in rbtdrh_chain.rs asserting band 105 plus tracked-rbrv byte-identity. anoint takes the strict zrbfc_load_vessel (demands the canonical RBRR_VESSEL_DIR location), so unlike feoff it cannot use a temp vessel — drove the one real graft vessel rbev-graft-demo as folio against an empty BURV root, the yoke reject-before-write precedent. chaining-fact-band 14/14 green; shellcheck 217 files clean. Code committed in 55ae8610.

### 2026-06-24 13:50 - ₢BiAAd - n

Convert anoint's three broken-chain buc_die (hallmark, gar_root, ark_stem) to buc_reject BUBC_band_chain (105), making the last write-side holdout uniform with feoff/yoke — anoint is a durable-leak writer (rewrites RBRV_GRAFT_IMAGE), so a bad resolve is a deliberate rejection, not a bare die. Comment updated off the loud-die claim. Add a reveille case rbtdrh_anoint_broken_chain in rbtdrh_chain.rs asserting band 105: anoint takes the strict load, so unlike feoff it cannot use a temp vessel — it drives the one real graft vessel (rbev-graft-demo) as its folio against an empty BURV root; the broken-chain reject precedes the rewrite, so the tracked rbrv.env stays byte-identical (asserted). Harness comment documents anoint's real-vessel approach alongside feoff/yoke.

### 2026-06-24 13:39 - ₢BiAAZ - W

Read-side chaining furnish + named-band conversion. Sourced buf_fact in the read-consumer CLIs (rbfr, rbfc0) plus two transitive-loader CLIs the new static check caught (rbfh, rbfv — they load rbfcp_plumb via rbfc0_core). Converted summon/plumb/augur broken-chain + augur wrong-kind from buc_die to buc_reject BUBC_band_chain (105), reversing the read-side loud-die cinch to named-band (105 reused — read and durable-leak write never share a tabtarget). Added to rbtdrh_chain: four reveille exit-code cases (folio-less drives assert 105, resolve-logic) and a static furnish-invariant case (transitive CLI source-closure must reach buf_fact — the no-cloud net the runtime cases cannot provide, since command-not-found exits the same 105). reveille 118/0 green; dogfight green (cloud confirmation of summon/plumb end-to-end). Paddock curried to the reversed discipline. Spec-subdoc correction deferred to the placeholder pace; anoint's matching write-side gap cantled as a follow-on.

### 2026-06-24 13:38 - Heat - S

anoint-band-conformance

### 2026-06-24 13:32 - Heat - T

rbs0-chaining-style-quoins

### 2026-06-24 13:32 - Heat - d

batch: 1 reslate

### 2026-06-24 13:24 - Heat - d

batch: 1 reslate

### 2026-06-24 13:21 - Heat - d

paddock curried: curry chaining-fact section to the reversed read-side discipline (band-reject, distinction-by-effect, two-net coverage, furnish-what-you-load, tier-separation cinch)

### 2026-06-24 13:09 - ₢BiAAZ - n

Close two furnish gaps surfaced by the new static furnish-invariant check: rbfh_cli (hygiene) and rbfv_cli (vouch) transitively load rbfcp_plumb (a buf_elect caller) through rbfc0_core without sourcing buf_fact. Add the furnish line, consistent with the other rbfc0_core consumers — furnish what you load. The manual audit missed these; the static check caught them on first run.

### 2026-06-24 12:58 - ₢BiAAZ - n

Read-side chaining furnish + named-band conversion. Furnish fix: source buf_fact in rbfr_cli and rbfc0_cli so summon/plumb resolve instead of dying command-not-found. Convert summon/plumb/augur broken-chain (+ augur wrong-kind) from buc_die to buc_reject BUBC_band_chain, reversing the read-side loud-die cinch to named-band (reused 105; read and durable-leak write never share a tabtarget). rbtdrh_chain: reveille exit-code cases driving the three read verbs folio-less (assert 105, resolve-logic) plus a static furnish-invariant case (transitive CLI source-closure must reach buf_fact) as the no-cloud net for the furnish gap, which the runtime cases cannot distinguish.

### 2026-06-24 12:47 - Heat - S

chaining-discipline-spec-quoins

### 2026-06-24 12:42 - Heat - S

read-existence-failure-bands

### 2026-06-24 12:07 - Heat - d

paddock curried: chaining-fact note: fast-tier -> reveille-tier, tracking the merged suite rename

### 2026-06-24 12:06 - Heat - d

batch: 1 reslate

### 2026-06-24 12:01 - Heat - d

paddock curried: chaining-fact section: read-side consumers carry their own furnish + coverage requirements (surfaced by the summon/plumb furnish-gap diagnosis)

### 2026-06-24 12:00 - Heat - d

batch: 1 reslate

### 2026-06-24 12:00 - Heat - T

read-side-chaining-furnish-coverage

### 2026-06-24 02:12 - ₢BiAAa - n

Backlink the chaining-consolidation memo (₢BbAAT's source) to this pace. Added a 'Coupled pace' section carrying ₢BiAAa's coronet: it warns the consolidation that the durable-leak link's membership grows from three to four (the nameplate-hallmark RBRN_*_HALLMARK drive joins feoff/anoint/yoke), now spanning two regime families (RBRV_* vessel + RBRN_* nameplate), so the consolidated motif/enumeration must generalize beyond a fixed three-RBRV_*-verb roster. Closes the cross-heat loop both ways — ₢BiAAa's docket points at ₢BbAAT, the memo now points back.

### 2026-06-24 02:11 - Heat - d

batch: 1 reslate

### 2026-06-24 02:08 - Heat - S

nameplate-hallmark-drive-chain-link

### 2026-06-24 01:59 - ₢BiAAY - W

Retired the redundant build-cycle convenience tabtargets rbw-tO (OrdainCycle/rbob_ordain) and rbw-tK (KludgeCycle/rbob_kludge) after verifying the coverage gate. Local kludge is served by the retained per-container rbw-cKB/rbw-cKS (rbob_kludge_bottle/_sentry, siege + onboarding-sequence covered); cloud ordain by onboarding's own rbtdro_ordain_capture + rbtdro_drive_hallmark plus operator-facing rbw-fO/rbfd_ordain. Inlined the bottle-kludge body into rbob_kludge_bottle (was a thin delegate to the deleted rbob_kludge) so it is now self-contained and symmetric with rbob_kludge_sentry. Removed both rbz_zipper enrollments + the orphaned z_mod=rbob_cli.sh line; simplified the two differential case statements in rbob_cli.sh (enforce-gate + kindle-gate) by dropping the rbob_ordain branch and rbob_kludge from the kludge list, trimming their now-moot comments; removed the now-dead rbfd_director.sh source (rbob_ordain was its only consumer). Repointed the rbfk_kludge.sh operator help hint from the deleted rbw-tK to the retained rbw-cKB/rbw-cKS. Regenerated rbtdgc_consts.rs + claude-rbk-tabtarget-context.md via the theurge build. Verified no Rust consumers of the dropped RBTDGC_THEURGE_KLUDGE/ORDAIN consts before deletion. Green: build, rbw-tq, rbw-tl (217 files clean), and a full-repo grep sweep clean of all removed identifiers (rbw-tK, rbw-tO, rbob_ordain, RBZ_THEURGE_*, KludgeCycle, OrdainCycle).

### 2026-06-24 01:59 - ₢BiAAY - n

The diff retires the superseded `rbob_kludge`/`rbob_ordain` cycle tabtargets — exactly the follow-up cantled as ₢BiAAY.

### 2026-06-24 01:28 - ₢BiAAD - W

Widened the release-tier shellcheck gate past its Tools/-only root to cover load-bearing outside-Tools/ bash, and brought it clean. buq_shellcheck (BUK) gained an explicit trailing extra-file parameter (validated, fails loud on a missing extra); rbq_qualify_shellcheck (RBK) enumerates the vessel security-boundary jailers (rbjs_sentry.sh, rbjp_pentacle.sh) and globs the per-nameplate charge hooks — single chokepoint, so rbw-tl/rbw-tr/marshal-zero all inherit. 34 jailer findings resolved: 32 behavior-neutral direct fixes (read -r on ip/IP-data reads; double-quoting single-token interface-name expansions) and 2 behavior-preserving rewrites of the sentry RBJp0 compose-quoting probe (deliberate whitespace tokenization rewritten to capture for-loop tokens, since the no-inline-directive policy bars suppression). Deleted the vestigial MBS.STATION-reference.sh (no consumer; was release-stripped) and its rbk-prep-release.md reference. Verified: rbw-tl 217 files clean; siege 62/0/0 pre- AND post-change (jailer containment proven intact). Risk assessment + the nsproto->tadmor->ifrit/theurge lineage captured in Memos/memo-20260624-jailer-shellcheck-coverage-risk.md. Two follow-ups surfaced and cantled next: prune the superseded rbob_ordain/rbob_kludge cycle tabtargets (₢BiAAY), and debug the dogfight cloud-build summon failure (₢BiAAZ).

### 2026-06-24 01:27 - Heat - S

debug-cloud-build-summon

### 2026-06-24 01:27 - Heat - S

prune-legacy-build-cycle-tabtargets

### 2026-06-24 01:11 - ₢BiAAD - n

Update the jailer-coverage memo: record the post-change siege result (GREEN 62/0/0, identical to baseline, commit 1111f5901; rbw-tl green 217 clean), correct the cloud-verification section per operator (tadmor is local-only/kludge, NOT cloud-built — moriah/blockade is the cloud-airgap sentry path, dogfight is the cloud-pipeline smoke test), and capture a Spook: the rbob_ordain (rbw-tO OrdainCycle) path is broken — rbfd_director.sh:200 calls rbuh_json but rbob_cli.sh furnish sources rbfd_director.sh without rbuh_http.sh (exit 127, command-not-found); the sibling rbw-fO/rbfd_cli path sources rbuh correctly, so only rbw-tO is affected; pre-existing, unrelated to jailer work, candidate itch.

### 2026-06-24 00:58 - ₢BiAAD - n

Bring the security-boundary jailer scripts (sentry, pentacle) to shellcheck-clean under the widened gate — 34 findings resolved, all classified behavior-neutral or rewritten to preserve behavior, none suppressed (the no-inline-directive policy bars suppression). Behavior-neutral direct fixes (32): added -r to four read calls (sentry lines 37/42/59, pentacle 38 — all read ip-command output or an IPv4 string, data that never carries backslashes); double-quoted 28 single-token interface-name expansions (${RBJ_ENCLAVE_IF}/${RBJ_UPLINK_IF} across the iptables/proc commands, ${RBJP_ENCLAVE_IF} on the pentacle ARP-flush) — single tokens from ip output, so quoting a space/glob-free value changes nothing, and the file already quoted its ${RBRN_*} IPs. Behavior-PRESERVING rewrite (2): the sentry RBJp0 compose-quoting probe used unquoted `echo ${RBJE_PROBE} | cut` to tokenize on whitespace — quoting would have changed multi-space behavior, so instead of suppressing (barred) the two cut-based comparisons now read the first/second tokens captured in the already-clean `for z_word in ${RBJE_PROBE}` loop; validation is identical (count==2, first==alpha, second==bravo) with the same failure modes. rbw-tl now green: 217 files clean (gate covers the jailers via the extras added in 3b794bc88). Post-change siege re-run follows to prove containment intact; risk assessment recorded in Memos/memo-20260624-jailer-shellcheck-coverage-risk.md.

### 2026-06-24 00:54 - ₢BiAAD - n

Record the jailer shellcheck-coverage risk assessment memo: the discovery (40 outside-Tools/ .sh files curated to a 3-file covered set with every exclusion's rationale), the mechanism decision (explicit extra-file list on buq_shellcheck, not directory-root, because the covered jailers share rbmm_moorings/ parents with excluded launcher shims), the MBS.STATION-reference.sh deletion (vestigial, release-stripped), the full 34-finding classification (32 behavior-neutral direct fixes — read -r and single-token interface quoting — vs 2 behavior-affecting rewrites where the sentry RBJp0 quoting-probe's unquoted echo is deliberate whitespace tokenization that the no-inline-directive policy forces us to rewrite rather than suppress), the gating discipline (siege before+after), the GREEN pristine-jailer siege baseline (62/0/0 on commit 3b794bc88), and the nsproto->tadmor->ifrit/theurge lineage establishing that 'cloud build and run nsproto' means running the tadmor security crucible against cloud-built (ordained) sentry+ifrit images.

### 2026-06-24 00:44 - ₢BiAAD - n

Widen the release-tier shellcheck gate past the Tools/ root and cut the vestigial MBS station reference (baseline-prep, jailer untouched). buq_shellcheck (BUK) gains a trailing explicit extra-file parameter: args beyond the third are validated load-bearing .sh files lints alongside the Tools/ tree, each must exist (a moved/renamed extra fails loud rather than silently dropping coverage). rbq_qualify_shellcheck (RBK) enumerates the extras: the vessel security-boundary jailers (rbjs_sentry.sh, rbjp_pentacle.sh, shipped in every vessel build context) plus a glob of the per-nameplate charge hooks (rbmm_moorings/*/rbnnh_post_charge.sh) — an explicit list, not a directory-root, so the deliberately-excluded launcher shims, Study/ scratch, fdkyclk POC drivers, and example-vessel scripts sharing those parents stay out. Single chokepoint: rbw-tl/rbw-tr/marshal-zero all route through rbq_qualify_shellcheck, so this one function widens all three. Deleted MBS.STATION-reference.sh (vestigial: no consumer anywhere, explicitly git-rm'd at release prep) and removed its rbk-prep-release.md 9e reference. The sentry/pentacle scripts have 34 not-yet-fixed findings, so rbw-tl is intentionally red on this commit until the jailer fixes land next; this commit exists to clean the tree for a pristine-jailer siege baseline.

### 2026-06-24 00:02 - ₢BiAAE - W

Pulled the RBK station regime (RBRS) off the active MVP surface, payload intact for the later podman build-out. Relocated rbrs_regime.sh, rbrs_cli.sh, RBSRS-RegimeStation.adoc, and rbv_cli.sh (verified not behind any live tabtarget) into Tools/rbk/vov_veiled/FUTURE/ beside rbv_podvm.sh. Deleted the active-surface hooks: rbw-rsr/rbw-rsv tabtargets; the rbz_zipper RBZ_RENDER_STATION/VALIDATE_STATION enrollments; RBCC_rbrs_file (def + emit, which drove RBTDGC_RBRS_FILE); the rbtdrf_rs_rbrs fast case and rbtdrs_rbrs_missing_platform poison case (fns + imports + case! registrations); the RBS0 Station Regime section + include::RBSRS + rbrs mapping attrs + rbrs_prefix anchor. Pruned the RBRS regime row/list-entry/bullet from CLAUDE.consumer.md (left cinched BUK BURS buw-* rows). Regenerated rbtdgc_consts.rs + claude-rbk-tabtarget-context.md via build (both re-derived RBRS-free). BUK station regime (burs/BURS) deliberately untouched; RBSPV left in place (podman-supply-chain spec, BW territory, inert prose refs only). Green: build, shellcheck 214 clean, fast suite 113 passed/0 failed/0 skipped (Done criterion met), regime-poison 30/0/0. No rbw-rs* tabtargets remain.

### 2026-06-24 00:00 - ₢BiAAE - n

Pull the RBK station regime (RBRS) off the active MVP surface, preserving the payload intact for the later podman build-out (heat BW). Relocated four dormant files into Tools/rbk/vov_veiled/FUTURE/ beside rbv_podvm.sh: rbrs_regime.sh, rbrs_cli.sh, RBSRS-RegimeStation.adoc, and rbv_cli.sh (verified not behind any live tabtarget). Deleted the active-surface hooks: rbw-rsr/rbw-rsv tabtargets; the rbz_zipper.sh RBZ_RENDER_STATION/RBZ_VALIDATE_STATION enrollments; RBCC_rbrs_file (def + emit entry), which drove the generated RBTDGC_RBRS_FILE; the theurge rbtdrf_rs_rbrs fast case (fn + comment + case! + 3 RBTDGC imports) and the rbtdrs_rbrs_missing_platform regime-poison case (fn + case! + import); and the RBS0 Station Regime section, its include::RBSRS, the rbrs mapping attrs, and the rbrs_prefix anchor. Pruned the RBRS regime row/list-entry/bullet from CLAUDE.consumer.md so first-MVP users no longer see it (left the cinched BUK BURS buw-* rows untouched). Regenerated rbtdgc_consts.rs + claude-rbk-tabtarget-context.md via the build (both re-derived RBRS-free). The BUK station regime (burs/BURS) was deliberately not touched. Build green, shellcheck 214 files clean. Fast-suite verification follows on the now-clean tree.

### 2026-06-23 23:47 - ₢BiAAL - W

Added a zipper division tier (buz_tome, one level above buz_group) so BUK workbench colophons project into generated Rust consts as a BUWGC_ peer block alongside RB's RBTDGC_ set. buz_emit_colophon_consts lost its single-zipper prefix args and now iterates tomes (each carrying its own add/strip prefix); buz_emit_context takes a tome name and scopes to that division, so BUK colophons never leak into RB's tabtarget-context.md. RB declares tome 'rbz', BUK declares tome 'buwz'; rbte_cli and rbq_cli source+kindle buwz after rbz, keeping the RB tome at roll index 0 so RBTDGC_ regenerates byte-identical. rbtdgc_consts.rs gained a 35-const BUWGC_ block (all buwz colophons); the theurge crate's 9 bare 'buw-' literals (rbtdrs_poison.rs, rbtdrf_fast.rs) now reference the generated BUWGC_ consts. RBTDGC_ byte-identical, tabtarget-context.md unchanged, regeneration idempotent (clean tree on rerun). Green: shellcheck 214 files, build, rbw-tq (all freshness+colophon+nameplate gates), 163/163 theurge unit tests. New/edited bash holds BCG discipline.

### 2026-06-23 23:46 - ₢BiAAL - n

Project BUK workbench colophons into generated Rust consts via a new zipper division tier. buz_tome (division marker, one level above buz_group) records which zipper owns each run of the shared registry roll; buz_emit_colophon_consts loses its single-zipper signature and iterates tomes, each carrying its own add/strip prefix; buz_emit_context takes a tome name and scopes to that division so BUK colophons never leak into RB's tabtarget-context.md. RB declares tome 'rbz' (RBTDGC_/RBZ_), BUK declares tome 'buwz' (BUWGC_/BUWZ_); rbte_cli and rbq_cli source+kindle buwz AFTER rbz so the RB tome stays at roll index 0 and the RBTDGC_ block regenerates byte-identical. rbtdgc_consts.rs gains a 35-const BUWGC_ block (all buwz colophons, parity with RB's total projection); the theurge crate swaps 9 bare 'buw-' literals for the generated BUWGC_ consts in rbtdrs_poison.rs and rbtdrf_fast.rs. All new/edited bash holds BCG discipline (((  )) arithmetic, explicit if over test&&assign, (( ${#..[@]} )) || buc_die guard). Verified: shellcheck 214 clean, build green, rbw-tq green (RBTDGC_ byte-identical, tabtarget-context.md unchanged), 163/163 theurge unit tests pass.

### 2026-06-23 22:39 - Heat - T

reconcile-and-push-chaining-fixes

### 2026-06-23 22:24 - ₢BiAAX - W

Renamed the rbw-acm probe's operator face from 'don a mantle' to mantle access. Tabtarget CheckMantleDon -> CheckMantleAccess (byte-identical content, colophon rbw-acm unchanged); zipper purpose (RBZ_CHECK_MANTLE) + handler buc_doc_brief + headline buc_step reframed to the access-reach verdict while keeping the active-don + AR-exercise + audit-entry honesty; claude-rbk-tabtarget-context.md regenerated via build (rbtdgc_consts.rs unchanged); README broadside glossary propagated (anchor + inbound link renamed, entry reframed to access verdict with AR/audit honesty) -- the one item beyond the docket, operator-approved. Interior don narration and comments left intact per the cinch. rbw-tq + rbw-tl green (214 files clean), rbw-acm routes, CheckMantleDon grep-clean repo-wide.

### 2026-06-23 22:13 - ₢BiAAX - n

Rename the rbw-acm probe's operator face from 'don a mantle' to mantle access. Renamed tabtarget CheckMantleDon -> CheckMantleAccess (byte-identical content; colophon rbw-acm unchanged); reframed the zipper purpose (RBZ_CHECK_MANTLE) and the handler's buc_doc_brief + headline buc_step to the access-reach verdict while keeping the active-don + AR-exercise + audit-entry honesty; regenerated claude-rbk-tabtarget-context.md via build (rbtdgc_consts.rs unchanged, colophon stable); propagated the rename to the README broadside glossary (anchor CheckMantleDon -> CheckMantleAccess + its one inbound link, entry reframed to the access verdict with AR/audit honesty). Interior don narration and comments left intact per the cinch. rbw-tq + rbw-tl green (214 files clean); rbw-acm routes to rbgv_check_mantle; CheckMantleDon grep-clean repo-wide.

### 2026-06-23 22:04 - ₢BiAAU - W

Ran the chaining-fact cloud fixture (chaining-fact-livery / rbtdrc_chaining_livery) live on a service-tier session: PASSED, 1/1, ~3.5 min of real Cloud Build. Discharges the runtime-UNVERIFIED cinch from authoring pace BiAAT. The one consolidated case proves both halves of the Done-when — integration (real bole ensconce upstream -> feoff -> emitted touchmark equals pin b260623000000 + bole-seed shape) and provenance (staged vessel RBRV_IMAGE_1_ANCHOR bears the committed <touchmark>:rbi_bole stamp). No code changes; execution-only as scoped.

### 2026-06-23 21:54 - Heat - S

mantle-access-probe-rename

### 2026-06-23 21:50 - ₢BiAAT - W

Authored chaining-fact-livery, the service-tier cloud sibling of the local chaining-fact band matrix (rbtdrh_chain.rs). Single case rbtdrc_chaining_livery drives real ensconce -> chain_next_invoke -> real feoff against a staged temp vessel (no tracked config, no commit), proving the live producer->consumer handoff the synthetic matrix only simulates; asserts the chained touchmark's pin+bole shape and the elected RBRV_IMAGE_1_ANCHOR locator. Self-contained divine-then-banish reset baseline + best-effort cleanup. Pivoted from the docket's setup/teardown hooks to inline reset (main.rs reads setup.is_some() as a crucible-charge signal, which fataled rbw-tc) — matching the lode-lifecycle bare-cloud precedent, which also makes cleanup run on mid-case failure and in single-case mode. Registered in service+complete suites; name const in rbtdrm_manifest.rs. Code-only per the cinch: build green under deny(warnings), 163/163 unit tests, shellcheck 214 clean, rbw-tc lists the case; the live GCP run is the deferred follow-on. Mint chaining-fact-livery via Lapidary (grep-clean, feudal asterism with feoff).

### 2026-06-23 21:48 - ₢BiAAT - n

Author the cloud-integration chaining-fact fixture chaining-fact-livery — the genuine cloud sibling of the local band matrix (rbtdrh_chain.rs). Single service-tier case rbtdrc_chaining_livery drives a real bole ensconce (busybox base, touchmark pinned to b260623000000 via the ensconce-stamp tweak) -> chain_next_invoke -> real feoff against a STAGED TEMP vessel resolved by path: no tracked config touched, nothing committed. Proves the live producer->consumer succession the synthetic matrix can only simulate — catches drift between what a live ensconce WRITES to current/ and what a live feoff READS from previous/. Asserts the emitted touchmark (via rbtdri_read_burv_fact) equals the pin and carries the bole-seed shape ('b' + 12 digits), then a config-file read of the temp vessel's RBRV_IMAGE_1_ANCHOR bears '<touchmark>:rbi_bole' (proving feoff fired, no no-op, and elected the chained value). feoff itself makes no GAR call (RBSDF) — the registry confirmation is conjure's at a later build; the live ensconce is what makes the chained touchmark real. The case self-contains a divine-then-banish reset baseline (load-bearing; the cloud collision guard would trip on a leaked prior Lode at the pinned touchmark) plus a best-effort cleanup that runs on every path. Reset lives in the case body, NOT a setup hook: main.rs reads setup.is_some() as 'crucible fixture, verify charged', which fataled rbw-tc — so this follows the lode-lifecycle bare-cloud precedent (setup/teardown None), which also makes cleanup run on mid-case failure and work in single-case mode. Registered in RBTDRC_FIXTURES + the service and complete suites (not fast/crucible); name const RBTDRM_FIXTURE_CHAINING_LIVERY in rbtdrm_manifest.rs. Mint chaining-fact-livery applies Lapidary (grep-clean, feudal asterism: livery of seisin is the ceremony making a feoff a real conveyance). Build green under deny(warnings); 163/163 theurge unit tests pass; shellcheck 214 files clean; rbw-tc lists rbtdrc_chaining_livery. Runtime UNVERIFIED per the pace cinch — the live GCP service run is the deferred follow-on.

### 2026-06-23 21:15 - ₢BiAAV - W

Consolidated the two drifted write-side commit helpers (rbtdro_git_commit, rbtdrk_git_add_and_commit) into a scoped 'fixture config-evolution console' in rbtdre_engine.rs beside rbtdre_tree_clean: a private rbtdre_commit_paths core (empty-paths guard + scoped status no-op) plus class-typed verbs commit_nameplates/commit_vessels/commit_vessels_all/commit_regime that derive their own paths from a class identifier, so a fixture is structurally incapable of sweeping a surprise edit. Added the rbtdre_config_set_field/rbtdre_config_zero seam (config_zero = set_field with empty value; rbtdro_write_vessel_env now delegates to the shared core). Migrated all 10 onboarding + 2 freehold call sites; removed 6 now-dead helpers and the unused Command import. Documented the console doctrine (three config classes, no-free-form-list property, config-zero, dispersed siblings flagged) in claude-rbk-theurge-ifrit-context.md. Added 4 unit tests incl. a git-backed no-sweep proof and config-zero fail-on-absent. Build green under deny(warnings), 163/163 unit tests pass; no bash touched. Adversarial review (4 dimensions + per-finding skeptic): 1 low doc-fidelity nit fixed (regime row basenames -> full <moorings>/ paths), and the review confirmed the consolidation incidentally fixes a latent freehold bug (old unconditional commit spuriously failed a fixture on idempotent re-run). GCP-tier suites (onboarding-sequence/freehold/skirmish) that exercise the sites at runtime remain the operator's release-qualification step.

### 2026-06-23 21:15 - ₢BiAAV - n

Consolidate fixture config commits into a scoped config-evolution console

### 2026-06-23 20:39 - Heat - d

batch: 2 reslate

### 2026-06-23 20:36 - Heat - S

resolved-base-provenance

### 2026-06-23 19:51 - Heat - S

theurge-commit-seam

### 2026-06-23 19:12 - Heat - T

chaining-fact-cloud-code

### 2026-06-23 19:12 - Heat - d

batch: 1 reslate, 1 slate

### 2026-06-23 19:06 - ₢BiAAR - W

Built and verified the chaining-fact-band theurge fixture (rbtdrh_chain.rs): 8 cases driving the durable-leak chain LINKS feoff/yoke through the real tabtarget exec path, asserting the named chain-rejection band RBTDGC_BAND_CHAIN=105 (newly projected from BUBC_band_chain in rbcc_emit_consts — the mirror was absent, skipping 104->115). Coverage: feoff wrong-kind/unknown-prefix/broken-chain each reject with the band; feoff good (bole express) elects the ANCHOR; feoff precedence proves an express bole beats a seeded reliquary chain; feoff fact-intact proves rejection-before-write (rbrv.env AND the seeded previous/ fact byte-identical under a band reject); yoke wrong-kind/unknown-prefix reject before auth and the fan-out write. feoff resolves a staged temp vessel by path so no tracked config is touched; the chained fact is seeded into the BURV root current/ which bud promotes to previous/. Registered in fast/service/crucible/complete; credless. Verified: full fixture 8/8 green, fast suite 113 passed / 0 failed / 1 expected station skip, shellcheck 214 files clean, zero tracked vessel rbrv.env mutated. Mid-flow the pace was reslated from a BUK-self-test home to theurge and the verification-home cinch added to the paddock; the cloud integration + provenance cases split to follow-on pace BiAAT.

### 2026-06-23 19:05 - Heat - d

batch: 1 reslate, 1 slate

### 2026-06-23 19:02 - ₢BiAAR - n

Add the chaining-fact-band theurge fixture (rbtdrh_chain.rs) — the band matrix for the durable-leak chain LINKS feoff/yoke, driven through the real tabtarget exec path and asserting the SPECIFIC chain-rejection band, never bare nonzero. Eight cases: feoff wrong-kind (reliquary express), unknown-prefix, and broken-chain (empty express + empty previous/) each reject with RBTDGC_BAND_CHAIN (105); feoff good (bole express) elects the ANCHOR; feoff precedence proves an express bole beats a seeded reliquary chain (express short-circuits the chain read); feoff fact-intact proves rejection-before-write — a wrong-kind chained fact rejects with the band while the staged rbrv.env AND the seeded previous/ fact stay byte-identical; yoke wrong-kind/unknown-prefix reject before auth and the fan-out write. Newly projected RBTDGC_BAND_CHAIN=105 from BUBC_band_chain in rbcc_emit_consts (the mirror was absent, skipping 104->115). feoff resolves a staged temp vessel by path so no tracked config is touched; the chained fact is seeded into the BURV root current/ which bud promotes to previous/. Registered in RBTDRC_FIXTURES + the fast/service/crucible/complete suites; credless (nothing mints a token). 8/8 cases pass in single-case mode; shellcheck 214 files clean; no tracked vessel rbrv.env mutated by the yoke negatives. The band-asserting tests are homed under theurge per the pace reslate — the BUK self-test cannot reach the RBK band gate.

### 2026-06-23 18:39 - Heat - d

paddock curried: homed chaining-fact verification under theurge (not BUK self-test) — the intent prior models had to guess at

### 2026-06-23 18:38 - Heat - d

batch: 1 reslate

### 2026-06-23 18:12 - ₢BiAAP - W

Converged anoint's three build-fact reads (hallmark, gar-root, ark-stem) onto the shared buf_elect_fact_capture resolver — folding the last hand-rolled buf_read_fact + buc_die consumer onto the footing every sibling (yoke, feoff, plumb, summon, inventory, lifecycle) already uses. anoint carries no express path, so each resolve passes an empty express, falling straight through to buf_read_fact_capture: byte-identical chain-read behavior. Per the docket cinch, the chain-only source labeling and per-read buc_die phrasing stayed untouched (anoint's loud-on-typecheck output remains the feoff pace's to change). Grep gate: buf_read_fact across Tools/rbk/ now empty; repo-wide it survives only in the footing (buf_fact.sh + BUS0) and the BUK self-test. rbw-tl clean (214 files); fast suite 105/105 (1 expected operator-local station-regime skip). Committed c1cd42f60.

### 2026-06-23 18:09 - ₢BiAAP - n

Converge anoint's three build-fact reads (hallmark, gar-root, ark-stem) onto the shared buf_elect_fact_capture resolver — folding the last hand-rolled buf_read_fact + buc_die consumer onto the footing every other fact consumer (yoke, feoff, plumb, summon, inventory, lifecycle) already uses. anoint carries no express path, so each resolve passes an empty express, which falls straight through to buf_read_fact_capture — byte-identical chain-read behavior. The per-read buc_die phrasing stays, and the chain-only source labeling is untouched (anoint's loud-on-typecheck output is the feoff pace's to change, per the docket cinch). Repo-wide grep for buf_read_fact now returns only the footing (Tools/buk/buf_fact.sh + BUS0 spec) and the BUK self-test; shellcheck 214 files clean.

### 2026-06-23 18:03 - ₢BiAAO - W

Retired the vestigial build-id fact RBF_FACT_BUILD_ID at all five sites: the constant + its 'read by tests' comment clause (rbgc_constants.sh), the sole conjure-side writer + comment + Output echo (rbfd_director.sh), the RBS0 quoin in full (mapping attr, anchor, axpof_fact voicing, body), and the two dangling {rbf_fact_build_id} cross-refs in RBSAC/RBSAG. Must-not-touch set (ZRBFC_BUILD_ID_FILE, native _RBGA_BUILD_ID/build_info.json, about's conjure_build_id, shared local z_build_id) verified untouched and z_build_id still in active use. Grep gate clean, shellcheck 214 files clean, fast suite 105/105 green. Committed 937b92c.

### 2026-06-23 18:02 - ₢BiAAO - n

Retire the vestigial build-id fact RBF_FACT_BUILD_ID — the orphaned half of a conjure↔vouch cross-check made redundant when vouch became cryptographic DSSE verification. Deleted at every site: the constant in rbgc_constants.sh (and struck the now-false 'read by tests' clause from its block comment); the sole conjure-side writer buf_write_fact_single plus its 'for cross-check with vouch provenance' comment and the paired buc_info Output echo in rbfd_director.sh; the RBS0 quoin in full (mapping-section attr ref, [[rbf_fact_build_id]] anchor, axpof_fact voicing, definition body); and the two dangling {rbf_fact_build_id} axhoot_typed_output cross-refs in RBSAC-ark_conjure.adoc (with its stale 'Used by ark_vouch for provenance verification' prose) and RBSAG-ark_graft.adoc ('Empty for graft … uniform fact coverage'). Left intact per docket: ZRBFC_BUILD_ID_FILE, _RBGA_BUILD_ID / build_info.json native $BUILD_ID, about's conjure_build_id parameter, and the shared local z_build_id (still feeds the console-URL + build-polling file). Repo-wide grep for the constant and rbf_fact_build_id lands clean; shellcheck 214 files clean.

### 2026-06-23 17:58 - Heat - d

batch: 1 reslate

### 2026-06-23 17:36 - Heat - T

retire-vestigial-build-id-fact

### 2026-06-23 17:35 - Heat - d

batch: 1 reslate

### 2026-06-23 17:32 - ₢BiAAO - n

Back out the build-id chain-fallback (commit 25ec5400bd) — additive revert to the pre-pace state, byte-exact. Investigation determined the change decorated a vestige: RBF_FACT_BUILD_ID is the orphaned half of a build-id cross-check deliberately built at AlAAb (branch fix-consecration-discovery-strong-tie) as a TEST-time assertion comparing a conjure-written fact against a vouch-written fact, to compensate for a then-non-cryptographic vouch (the RBS0 of that day: 'weaker than cryptographic verification ... Integration of slsa-verifier into the vouch path is deferred'). When vouch became real DSSE envelope signature verification (rbgjv02-verify-provenance.py, against Google attestor keys), the compensation was dismantled — vouch's writer of the fact and the comparison test (rbtcsl_SlsaProvenance.sh) were both removed — leaving only conjure's write as an orphan with a now-false comment 'for cross-check with vouch provenance'. The current verifier references no build-id; nothing read the fact until this pace's commit. So the fallback gave a dead fact its first reader, in the wrong verb (about's recipe-stamping) for the wrong reason. Reverts both the rbfv_verify.sh conjure-branch resolution + doc_param + local-r drop and the RBSAB _RBGA_BUILD_ID rework + read-side-NOTE. Pace to be reslated from 'add fallback' to 'retire the vestigial build-id fact'.

### 2026-06-23 17:07 - ₢BiAAO - n

Vouch's standalone-about conjure build-id falls back to the chained build-id fact when omitted — the smallest and last read-side adopter of the shared chaining footings. rbfv_about's already-optional conjure_build_id (3rd arg) now resolves express-or-chain via buf_elect_fact_capture against RBF_FACT_BUILD_ID (written by the prior conjure in rbfd_director) inside the rbnve_conjure branch of zrbfv_about_submit: an explicit argument behaves exactly as today; omitted, it falls back to the build-id the prior conjure chained forward, so a standalone about immediately after a conjure stamps the just-built id into build_info.json. Mode-gated by construction — the resolve lives in the conjure case only, so bind/graft (which carry no build-id and where empty is correct per RBSAB) never reach it; dropped local -r on z_conjure_build_id so the branch can reassign. Read-side severity held: a broken chain dies loud via buc_die, NOT the durable-leak surface's band-reject — about writes no durable config and carries neither the named band nor a clean-tree gate; the verb comment and the spec NOTE both mark the categorical distinction from feoff/anoint/yoke. Contract-first: RBSAB _RBGA_BUILD_ID row reworked to document the express-or-chain resolve, plus a read-side-chain-consumer NOTE. Entry-point doc_param advertises the now-conjure-only fallback. shellcheck 214 files clean.

### 2026-06-23 16:57 - ₢BiAAN - W

Summon, plumb, and rekon gained the express-or-chain hallmark fallback — the read-side hallmark cluster, third consumer of the shared chaining footings after yoke (link) and augur (read-side). Each resolves via buf_elect_fact_capture against RBF_FACT_HALLMARK (express folio wins; absent, falls back to the hallmark a prior build — ordain or kludge — chained forward, depth-1 no-relay). Per the docket's keep-it-minimal cinch: NO kind decode and NO new shape predicate — hallmarks carry no sub-kind these verbs discriminate and the chain value is build-written, so the resolve is the whole typecheck. Read-side severity held: broken chain dies loud via buc_die, NOT the durable-leak surface's BUBC_band_chain reject; the verb comments and spec NOTEs mark the categorical distinction from feoff/anoint/yoke. Plumb resolves once in zrbfc_plumb_core (shared by full+compact); entry-point doc_params advertise the now-optional arg. Contract-first: RBSAS/RBSAP/RBSIR input steps reworked to Resolve the Hallmark (express-or-chain) with a read-side-chain-consumer NOTE each — the rework incidentally retired stale vessel-argument descriptions (RBSAS two-arg vessel+hallmark; RBSAP Two-invocation-modes NOTE + hallmark-only-mode labels) that no longer matched the single-param code. Verified: shellcheck 214 files clean, fast suite 105 passed / 0 failed / 1 skipped. The live service path (GCP+registry) is service-tier and not exercised by fast — same caveat as the augur commit; resolver reachability confirmed by source (bud_dispatch global source + each CLI's rbgc_constants source).

### 2026-06-23 16:54 - ₢BiAAN - n

Summon, plumb, and rekon gain the express-or-chain hallmark fallback — the third read-side consumer cluster of the shared chaining footings, after yoke (link) and augur (read-side). Each verb now resolves its hallmark via buf_elect_fact_capture (express folio wins; absent, falls back to the RBF_FACT_HALLMARK fact a prior build — ordain or kludge — chained forward, depth-1 no-relay), so a no-arg summon/plumb/rekon immediately after a build acts on the just-built hallmark. Unlike augur/feoff/yoke there is NO kind decode and NO new shape predicate: hallmarks carry no sub-kind these verbs discriminate and the chained value is build-written, so the express-or-chain resolve is the whole typecheck (per the docket's keep-it-minimal cinch). Read-side severity held: a broken chain dies loud via buc_die, NOT the durable-leak surface's buc_reject BUBC_band_chain — these verbs write no durable config and carry neither the named band nor a clean-tree gate; the comment in each verb and the spec NOTE both mark the categorical distinction from feoff/anoint/yoke. Plumb resolves once in zrbfc_plumb_core (shared by full+compact); both entry-point doc_params now advertise the optional arg. Contract-first: RBSAS/RBSAP/RBSIR input steps reworked to Resolve the Hallmark (express-or-chain) with a read-side-chain-consumer NOTE each; the rework incidentally retired stale vessel-argument descriptions in RBSAS (two-arg vessel+hallmark) and RBSAP (Two invocation modes NOTE + hallmark-only-mode labels) that no longer matched the single-param code. shellcheck 214 files clean.

### 2026-06-23 16:43 - ₢BiAAM - W

Augur became the second read-side consumer of the shared chaining footings. rbld_augur now resolves its lode touchmark express-or-chain via buf_elect_fact_capture (express folio wins; absent, falls back to the touchmark any capture chained forward, depth-1 no-relay) and asserts a known kind via the shared zrbld_decode_touchmark_kind_capture decoder — accepting ANY known Lode kind, unlike feoff's bole-only / yoke's reliquary-only gates, and replacing augur's former format-shape regex. Read-side severity held: broken chain / unknown kind die loud via buc_die, not the durable-leak surface's buc_reject BUBC_band_chain — augur writes no durable config and carries neither the named band nor a clean-tree gate. Contract-first: RBSLA input step reworked to Resolve the Touchmark (express-or-chain) with the any-known-kind decode, plus a read-side-chain-consumer NOTE marking the categorical severity distinction from feoff/yoke. Verified: shellcheck 214 files clean, fast suite 105 passed / 0 failed / 1 skipped. The augur runtime path (Director auth + GAR) is service-tier and not exercised by fast.

### 2026-06-23 16:40 - ₢BiAAM - n

Augur gains the express-or-chain touchmark fallback — the second read-side consumer of the shared chaining footings. rbld_augur now resolves its lode touchmark via buf_elect_fact_capture (express folio wins; absent, falls back to the touchmark any capture chained forward, depth-1 no-relay), so a no-arg augur immediately after a capture inspects the just-captured Lode. Replaced augur's former regex shape-check with the shared zrbld_decode_touchmark_kind_capture known-kind assertion — augur accepts ANY known Lode kind, unlike feoff's bole-only / yoke's reliquary-only gates. Read-side severity: a broken chain or unknown kind dies loud via buc_die, NOT the durable-leak surface's buc_reject BUBC_band_chain — augur writes no durable config and carries neither the named band nor a clean-tree gate. Contract-first: RBSLA input step reworked to Resolve the Touchmark (express-or-chain) with the any-known-kind decode, plus a new read-side-chain-consumer NOTE marking the categorical severity distinction from feoff/yoke. shellcheck 214 files clean.

### 2026-06-23 16:31 - Heat - S

reconcile-and-push-chaining-fixes

### 2026-06-23 16:31 - ₢BiAAQ - W

Extracted conjure's base-anchor election into the new feoff link verb — conjure is now a pure chain head (reads no fact, builds only from committed config), closing the provenance hole where the election wrote the anchor after the clean-tree gate. Folded in elimination of the redundant brand chaining fact: the touchmark is the sole kind carrier, decoded from its prefix via the single decoder footing (feoff + yoke its consumers); brand strings survive only as immure's family vocabulary + display labels. Contract-first: RBSAC reversed, RBSDF-director_feoff subdoc minted + RBS0 wiring, Lode capture specs (RBSLE/RBSLC/RBSLU/RBSLI + rbf_fact_lode quoin) collapsed to one fact. New rbflf_feoff.sh (vessel express + touchmark express-or-chain via buf_elect_fact_capture, bole gate via zrbld_decode_touchmark_kind_capture, buc_reject BUBC_band_chain on broken chain/non-bole, loud-on-success value+source, no token/clean-tree/fact); colophon rbw-rvf wired (zipper RBZ_FEOFF_BOLE + ledger source + tt launcher). anoint gained loud value+source output. Re-homed the airgap onboarding test chain to ensconce->feoff->commit->ordain with a new rbtdro_feoff invoke helper. Acronyms RBFLA/RBFLF/RBSDF. Local verification all green: rbw-tl 214 files clean, theurge build + 159/0 unit tests, fast suite 105/0/1. Un-run here (no cloud creds): the crucible/onboarding tier exercising the re-homed chain; the provenance proof is delegated by this docket to BiAAR.

### 2026-06-23 16:25 - ₢BiAAQ - n

Extract conjure's base-anchor election into a new feoff link verb so conjure is a pure chain head (reads no fact, builds only from committed config) — closes the provenance hole where the election wrote the anchor after the clean-tree gate. Fold in elimination of the now-redundant brand chaining fact: the touchmark is the sole kind carrier, decoded from its prefix via the single decoder footing (feoff + yoke its consumers); brand strings survive only as immure's family vocabulary + display labels. Contract-first: reverse RBSAC (election sanction removed), new RBSDF-director_feoff subdoc + RBS0 wiring, Lode capture specs (RBSLE/RBSLC/RBSLU/RBSLI + rbf_fact_lode quoin) now one fact. Code: new rbflf_feoff.sh (vessel express + touchmark express-or-chain via buf_elect_fact_capture, bole gate via zrbld_decode_touchmark_kind_capture, loud-on-success with value+source, buc_reject BUBC_band_chain on broken chain/non-bole, no token/clean-tree/fact), wired colophon rbw-rvf (zipper RBZ_FEOFF_BOLE + ledger source + tt launcher). Eliminated RBF_FACT_LODE_BRAND (constant + spine/bole writes + the election reader); spine brand param demoted to display label. anoint gains loud value+source output. Re-homed the airgap onboarding test chain (rbtdro_onboarding.rs) to ensconce->feoff->commit->ordain with a new rbtdro_feoff invoke helper. Regenerated rbtdgc_consts.rs + tabtarget-context.md from the zipper. Acronyms: RBFLA, RBFLF, RBSDF. Verified: theurge build green, shellcheck 214 files clean, theurge unit tests 159/0.

### 2026-06-23 15:25 - ₢BiAAK - W

Laid the two reusable chaining footings and made yoke their first client. New BUK fact primitive buf_elect_fact_capture (express-or-chain, generic, depth-1 no-relay) in buf_fact.sh; renamed buf_read_fact -> buf_read_fact_capture for BCG _capture conformance across all call sites + BUS0 vocab. New Lode kind-decode hearting zrbld_decode_touchmark_kind_capture (rbldk_kind.sh) as the single home for the RBGC_LODE_KIND_* prefix decode (whole letter-run, not char[0]; 1-char b/r/w + 2-char vw/vn), sourced by both rbld0_lode and rbfl0_ledger with its own inclusion guard + zrbfc_sentinel shared-context pattern. Minted BUBC_band_chain=105. Yoke resolves express-or-chain and asserts reliquary kind up front, buc_rejecting band 105 on broken chain / unknown prefix / wrong kind; express call unchanged. RBSDY relaxed to optional-with-fallback + kind gate. Added 2 BUK self-test cases. Verified: shellcheck 213 clean, BUK self-test 49/49, fast suite 105 passed/0 failed.

### 2026-06-23 15:23 - ₢BiAAK - n

Lay the two reusable chaining footings and make yoke their first client. New BUK fact primitive buf_elect_fact_capture (express-or-chain, depth-1 no-relay) in buf_fact.sh; renamed buf_read_fact -> buf_read_fact_capture for BCG _capture-suffix conformance across all call sites (anoint, director conjure election, BUK self-test) and BUS0 vocab. New Lode kind-decode hearting zrbld_decode_touchmark_kind_capture in rbldk_kind.sh (single home for the RBGC_LODE_KIND_* prefix decode, matched as a whole letter-prefix not char[0], 1-char b/r/w + 2-char vw/vn), sourced by both rbld0_lode and rbfl0_ledger (own inclusion guard, zrbfc_sentinel shared-context pattern). Minted BUBC_band_chain=105 precision-band code. Yoke now resolves express-or-chain and asserts reliquary kind up front via the decoder, buc_rejecting band 105 on broken chain / unknown prefix / wrong kind; express call unchanged. RBSDY relaxed from hard-require to express-or-chain with the kind gate. Added 2 BUK self-test cases (elect express-wins + chain-fallback). shellcheck clean (213 files); BUK self-test 49/49 green.

### 2026-06-22 12:22 - Heat - d

batch: paddock, 5 reslate

### 2026-06-22 12:05 - Heat - S

chaining-fact-band-tests

### 2026-06-22 12:05 - Heat - d

batch: 2 reslate, 1 slate

### 2026-06-22 14:12 - ₢BiAAC - W

Split the build-completion poll budget into two bounded clocks in zrbfc_wait_build_completion (rbfcb_host.sh): a shared queue ceiling ZRBFC_BUILD_POLL_CEILING_QUEUE=180 (~15min) counting pre-WORKING PENDING/QUEUED polls, and the existing per-kind ceilings now counting execution only from the QUEUED→WORKING transition. Queue exhaustion dies loud ('pool never took the build') with the quota tabtarget hint; execution timeout message narrows to 'execution polls'; one info line at QUEUED→WORKING records the queue poll count; the poll-20 queue-weather warning now repeats every 20 queue polls; per-poll displays show phase budget. Verified live in hallmark-lifecycle: Conjure/Abjure/Vouch all transitioned correctly (8/8/11 queue polls charged to the 180 ceiling, exec counter fresh from the transition) — the memo's casualty (Vouch's 50-poll budget eaten by a queue burst) can no longer occur. fast green; shellcheck clean. Queue-death and repeating-warning paths verified by inspection+shellcheck only (no real queue burst today).

### 2026-06-22 13:50 - ₢BiAAC - n

Split build-completion poll budget into two bounded clocks in zrbfc_wait_build_completion: a shared queue ceiling (ZRBFC_BUILD_POLL_CEILING_QUEUE=180, ~15min) counting pre-WORKING PENDING/QUEUED polls, and the existing per-kind ceilings counting execution only from first WORKING. Queue exhaustion dies loud (pool never took the build) with quota tabtarget hint; execution timeout message narrows to execution polls; poll-20 queue-weather warning now repeats every 20 queue polls; one info line at QUEUED->WORKING records the queue poll count; per-poll displays show phase budget.

### 2026-06-22 13:39 - Heat - r

moved BiAAA to last

### 2026-06-22 13:39 - Heat - d

batch: 1 reslate

### 2026-06-22 13:25 - Heat - n

Record stale-binary root cause + resolution for the recurring jjx_orient 'missing field firemark' spook: source/docs forward at e78aa537c (gazette-halter migration), deployed vvx binary lagged; fix is rebuild + restart MCP server

### 2026-06-22 13:01 - Heat - r

moved BiAAP after BiAAO

### 2026-06-22 13:01 - Heat - r

moved BiAAO after BiAAN

### 2026-06-22 13:01 - Heat - r

moved BiAAN after BiAAM

### 2026-06-22 13:00 - Heat - d

batch: 1 reslate, 4 slate

### 2026-06-22 12:35 - Heat - d

batch: 1 reslate

### 2026-06-22 12:15 - Heat - r

moved BiAAH after BiAAI

### 2026-06-22 12:15 - Heat - r

moved BiAAB after BiAAF

### 2026-06-22 12:15 - Heat - r

moved BiAAD after BiAAE

### 2026-06-22 12:15 - Heat - r

moved BiAAL after BiAAK

### 2026-06-22 12:15 - Heat - r

moved BiAAK after BiAAC

### 2026-06-22 12:15 - Heat - r

moved BiAAJ to last

### 2026-06-22 11:56 - Heat - f

racing

### 2026-06-21 16:08 - Heat - S

buk-colophon-projection-division

### 2026-06-21 13:47 - Heat - f

silks=rbk-14-mvp-loose-ends

### 2026-06-21 11:58 - Heat - S

yoke-touchmark-fact-fallback

### 2026-06-19 10:07 - Heat - S

payor-install-finish-manor-guidance

### 2026-06-18 15:28 - Heat - D

restring 1 paces from ₣BB

### 2026-06-18 14:02 - Heat - d

paddock curried: seat loose-ends paddock

### 2026-06-18 13:59 - Heat - D

restring 1 paces from ₣BU

### 2026-06-18 13:58 - Heat - D

restring 7 paces from ₣BB

### 2026-06-18 13:58 - Heat - N

rbk-15-mvp-loose-ends

### 2026-06-18 14:02 - Heat - d

paddock curried: seat loose-ends paddock

### 2026-06-18 13:59 - Heat - D

restring 1 paces from ₣BU

### 2026-06-18 13:58 - Heat - D

restring 7 paces from ₣BB

### 2026-06-18 13:58 - Heat - N

rbk-15-mvp-loose-ends

