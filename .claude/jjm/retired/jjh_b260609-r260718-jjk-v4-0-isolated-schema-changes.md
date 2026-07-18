# Heat Trophy: jjk-v4-0-isolated-schema-changes

**Firemark:** ₣Bc
**Created:** 260609
**Retired:** 260718
**Status:** retired

## Paddock

## Shape
This heat grew past its original framing of two isolated schema edits into three layers.

A reusable schema-migration tolerance mechanism — "forgiveness" — is the spine.
It is a cited rivet (an inert token in shipped code, rationale in the veiled spec), a single read-only detection probe shared by the loader and a new open-time status line, a per-episode registry, and a self-reporting nag at officium open.
The nag tells the operator, per install, whether each forgiveness is still load-bearing or has gone dormant.

Two wire-format-breaking schema changes ride that spine.
The first turns tack docket text into a line array and moves tack history out of the JSON into git, keeping one edited-in-place current tack.
The second turns validate into a normalize-and-report pass that canonicalizes the store and carries its verdict in the exit code.
Each registers an episode against the mechanism rather than re-plumbing detection.
A third schema change — pace bridling — is additive-only and rides no episode; its determination is recorded in Cinched below.
The heat has since adopted further schema-change paces:
the pace-original-intent capture — additive-only like bridling, riding no episode —
and (adopted 260708 from the studbook–farrier heat ₣Br)
the pace-identity re-gestalt — immutable-for-life pace ids from one global seed,
relocated here because it is an isolated gallops schema change ₣Br does not depend on;
its 260706 design cinches travel in its docket, and it must land and converge before the revision-control cutover heat.
(A gallops-key glyph-strip pace was adopted then abandoned when the minted-mark re-cut inverted its law.)

A coda then propagates the changes to every install and retires the spent episodes — but not the mechanism.
Multi-install schema drift is the steady state of this tool, not a transition to be ended: there is always a leading clone where a schema-impacting change lands first and lagging clones not yet converged.
So the registry, the shared probe, and the open-time nag are permanent infrastructure; only individual episodes — the per-version tolerances and their frozen legacy types — retire once every clone converges.

## Cinched
The forgiveness mechanism is the spine; the two schema changes register episodes against it, they do not each invent a trigger.
The rivet stays inert in shipped code; its rationale lives once in the veiled spec — the RBr_ rivet doctrine made JJK-native, because the rust ships and the specs do not.
The shared probe is the single source of "what counts as old-format"; the loader keeps no second copy.
The mechanism is per-episode from the start — more than one episode is live at once, so an episode's removal gate is "dormant on every operated clone", never a single boolean.
The open-time probe is read-only, best-effort, and never gating; officium open must still always succeed, which is a deliberate, logged softening of "open touches no persistent state" down to "open mutates nothing".
Mechanism and episode are distinct lifetimes: the registry, probe, and nag are permanent because drift is the steady state; the per-version tolerances and frozen legacy types are transient and retire on convergence.
The coda retires the spent episodes — this heat's tack-rework tolerance and the original V3-to-V4 tolerance with its frozen reference, consolidated here rather than left in a stale placeholder heat — and leaves the mechanism standing.
An episode's demolition condition — no pre-its-schema gallops survives in any operated clone — is checkable rather than an act of faith because the operator is sole, so the operated clones are a finite, enumerable set.
The durable "deep pattern" is not a torn-down cleverness but the standing mechanism plus its governing why, stated inline here: we minimize the schemas we keep mutually readable so the migration mechanism converges rather than accumulating (Zeroes Theory's version axis). The general cross-language statement of that why is a separate, independent formalization; this heat does not depend on it.
Schema-change paces are delivered source-only on a date-and-identity-named branch and never commit a gallops conversion; the forgiveness episode makes the new binary tolerant of the old store, so the conversion stays deferred until the coda — the sole coordinated convergence — forces and commits it across every install at once.
Keeping schema-change source off the shared main until that convergence is what prevents a stray gallops-saving command on a freshly-built binary from persisting the conversion prematurely, ahead of the cross-install coordination.
Bridle revival (260706): the third schema change is additive-only and registers NO episode — forgiveness exists to tolerate old stores, and an additive change leaves every old store natively readable, so a probe would have no signature; §F branch delivery is retained.
Its strip-first step retires the bridle-retirement episode ahead of the coda — the sole-instance ruling makes that episode's demolition condition already met — freeing the retired token for clean re-mint under the grep gate.
The provisional designation-tier vocabulary is the vendor family words carried by the model wire param; the session-tier design pace in the studbook–farrier heat (₣Br) may re-mint rung words later, a cheap value migration while one instance operates.

## Done when
Every install runs a binary whose on-disk gallops is canonical at this heat's final schema and whose open-time nag reports every episode dormant;
every spent episode's tolerance and frozen reference is stripped and its registry entry removed (the registry is the census — the V3-to-V4, schema_version-drop, and tack-rework tolerances today, plus any the remaining schema paces register);
the mechanism — registry, probe, open-time nag — stands as permanent infrastructure;
the forgiveness quoin describes that permanent mechanism with its per-episode lifecycle rather than a self-demolishing whole;
the crate builds and tests green.

## Paces

### apostille-command-rename (₢BcAAN) [complete]

**[260712-2300] complete**

## Character
Mechanical rename plus spec registration; word cinched, no design latitude.

## Shape
Repair the Upper/Lower vocabulary-isolation breach shipped with pace bridling:
the lower MCP command carries the vivid equestrian word (jjx_bridle),
and the upper verbs were never registered in JJS0's Upper API catalog.
Cinched: the lower command renames to jjx_apostille
(operator word choice 260707, grep-gated clean);
upper vocabulary — bridle, unbridle, the bridled state, jjdpe_bridled, wire jjgte_bridled — is untouched;
no serialized field changes, so no reprieve episode (§E does not fire).
Rust: rename the command constant, params struct, dispatch arm, schemars command list,
gate comments, and the designation-gate remedy text;
old name rejected loudly, no alias.
Discovery recipe: grep jjx_bridle and BRIDLE across Tools/jjk.
JJS0: mint the lower-tool quoin (jjdo_apostille) in the tool census;
add jjsuv_bridle and jjsuv_unbridle rows to Upper API Verbs
(two verbs onto one tool, chivvy/cantle precedent).
claude-jjk-core.md: verb table, command reference, three-bucket gate text, Bridle Protocol —
the command string only; equestrian upper prose stays.

## Done when
grep jjx_bridle lands clean outside historical/archive prose;
vow-b and vow-t green;
JJS0 carries the two upper-verb rows and the lower-tool quoin.

### bridle-verb-and-mount-guard (₢BcAAM) [complete]

**[260707-1125] complete**

## Character

JJK feature build; the judgment core is operator-settled (260706 design review) and this docket is written for mechanical execution at opus tier under the current global frontier gate.
Mid-execution posture: on any hole or surprise, surface the mechanism plus one proposed repair and stop — do not improvise design.
Gallops schema change, additive-only; the reprieve determination is recorded in Cinched — read §E and §F in the veiled JJK context first, and deliver on a §F date-and-identity branch with no gallops conversion committed.
The former cross-heat gate on the session-tier-launch design pace in ₣Br is disposed by operator ruling:
the tier vocabulary here is provisionally the vendor family words extractable from the model wire param;
that design pace has since ratified the vendor-family vocabulary and folded the optional effort field below into this docket (operator ruling 260707);
a future re-mint, if any, costs a value migration over stored tiers — cheap while exactly one instance operates.

## Docket

Add pace bridling: a frontier-only verb records that a frontier agent judged a pace mechanically defined and designates its execution tier; the server enforces from there.
The evaluator is always the calling frontier agent, never the server:
protocol text directs it to read the docket, refuse designation on ambiguities or lurking design-level decisions (reporting the gaps), and otherwise designate.

Strip first, then re-mint.
The bridle-retirement reprieve episode is dormant with the store canonical on the sole operating instance, so its demolition condition is met:
remove the jjgte_bridled serde alias, the episode probe, its registry entry, and the legacy-load test.
The V3-era primed alias and the frozen v3 Bridled variant stay with the V3-to-V4 episode — they demote to Rough through the conversion path and collide with nothing, since the new sense never writes those bytes; verify with the existing v3 tests.
Then reuse jjgte_bridled as the new state's wire token, with repo-wide grep proving the sweep.

Schema, additive only:
new pace state bridled (wire jjgte_bridled; display label joins the existing display-constant home);
a tier field beside the state on the current tack, serialized only when present so an untouched store stays byte-canonical and the load round-trip gate never trips;
an optional effort field beside the tier, same serialized-only-when-present posture — a designation without effort is byte-identical to the tier-only form;
tier and effort persist through close as provenance.
Tier is a typed enum, never naked strings (RCG Constant Discipline — follow the pace-state pattern: wire token per variant, one display home); mint tier wire tokens under the gallops token scheme behind a grep gate.
Effort is likewise a typed enum with its own wire tokens under the same scheme, anchored transparently on Anthropic's effort classification as the product surfaces it (low, medium, high, xhigh, max at authoring — verify against the current product surface and record what you find):
JJ mints no effort vocabulary of its own, and effort never rides the MCP wire, so it is recorded here and consumed only by the ₣Br dispatch layer when it lands — validation is recognized-word only; per-family pair ceilings arrive with that design's tier table.
Designable tiers: haiku, sonnet, opus, fable.
Recognized-but-refused: the gpt/codex and gemini families — named in the extractor for fair-faced diagnostics, outside both the frontier and designable sets;
gpt-5.5 is demoted OUT of the frontier gate (operator-sanctioned): frontier = fable + opus.
The tier extractor returns the enum; the naked tier string literals go.

Command surface — one new MCP command; the verb table maps both bridle and unbridle onto it:
{coronet, tier, effort?} designates — effort optional, recorded beside the tier — and only a rough pace may be bridled;
{coronet, release: true} un-bridles — bridled to rough, tier and effort wiped;
tier and release are mutually exclusive, exactly one required; effort rides only beside tier, never beside release;
frontier-only; self-committing with heat affiliation like the other state mutations.

Guard policy — the single global frontier gate becomes a three-bucket per-command policy:
OPEN to every tier: jjx_open and the read-only commands (list, show, brief, coronets, log, search).
DESIGNATION-GUARDED: orient — the resolved target pace must be bridled at the caller's tier; both directions hold (a frontier caller is refused on a sub-frontier-bridled pace, allowed on rough; a sub-frontier caller is refused on rough — undesignated work is judgment work);
the match is strict tier equality with no frontier carve-out — a fable caller is refused on an opus-bridled pace and vice versa, keeping the persisted tier honest provenance of the executing session; the frontier remedy is one command away (release or re-designate);
the refusal fires after resolution and never skips — orient does not pass over a tier-mismatched pace to a later one, because pace order is the dependency tree;
record and landing — a sub-frontier caller only against a coronet whose pace is bridled at its tier (firemark-affiliated record stays frontier); frontier callers unrestricted, as today.
The guard reads tier alone — effort is designation-and-dispatch data, invisible on the wire, never guarded.
FRONTIER-ONLY: everything else — all docket-authoring and state-mutating verbs, close, validate, the new bridle command, and the remote family.
Refusal messages are fair-faced: name the pace's designated tier, the caller's tier, and the remedies (a matching-tier session, or frontier release/re-designation).

Revert triggers — a designation is void when its judgment inputs change:
redocket of a bridled pace reverts it to rough and wipes the tier and effort (single and mass forms);
transfer and relocate likewise (the paddock context judged against changed);
relabel and reorder revert nothing.

State-filter semantics — bridled is a distinct open state:
the rough filter (jjx_coronets and the bridle precondition) excludes it;
remaining includes it, as it does every open pace;
next-actionable resolution lands on it, so the orient guard judges the resolved pace — resolution itself never skips;
the default coronets listing tags it with state and tier, coronet staying the first token, mirroring the abandoned tag.

Protocol text in the kit source claude-jjk-core.md:
verb-table rows for bridle and unbridle;
the bridling protocol for the frontier agent (read via brief, refuse-and-report gaps, else designate tier and, where the docket's depth is judged, effort);
the designee-session protocol (orient, work, record, landing, never wrap, stop-and-surface on holes);
the escalation paths (edit-revert and deliberate release).

Tests follow the crate's existing conventions:
tier extraction, each guard bucket including the cross-frontier orient refusals, every revert trigger, bridle/release transitions, the state-filter semantics,
and serde round-trips — old store byte-canonical, bridled store round-trips, effort-absent designation byte-identical to tier-only, effort recorded and wiped through the transitions, v3 demotion still green.
Notch before every test run (§H); build tt/vow-b.Build.sh, test tt/vow-t.Test.sh.

## Cinched

- Designation is by TIER, never model ID — IDs drift, tiers hold.
- Tier vocabulary: the vendor family words as extracted from the required MCP model param — the only thing the server can enforce from the wire. Ratified by the ₣Br session-launch design pace (operator ruling 260707).
- Effort designation is in scope (operator ruling 260707, folding the ₣Br design forward): an optional typed field on Anthropic's own effort words, recorded beside the tier; effort consumption, enforcement, and per-family ceilings stay with the ₣Br dispatch design — effort is not on the wire, so the guard never reads it.
- Wrap stays frontier-only, always: a sub-frontier designee finishes with record plus landing, and a frontier session reviews and wraps.
- Read commands stay open to every tier.
- Orient matching is strict tier equality across the whole designable set — no frontier carve-out; frontier oversight flows through show/brief and wrap, none of which are orient-gated.
- No reprieve episode: the change is additive — the new binary reads every old store natively, there is nothing to tolerate, and a probe would have no signature; the stale-binary-reads-new-store hazard is covered by §F branch delivery and the sole-instance reality. §E stands for shape changes; this records the additive carve-out with operator sanction (260706).
- Word re-mint sanctioned strip-then-reuse; the grep gate proves the eviction sweep (MCM re-mint rule).

## Done when

State, tier and effort fields, command with release, guard policy, revert rules, and fair-faced refusals land with tests green on a §F branch with no gallops conversion committed;
the bridle-retirement episode is stripped and repo-wide grep shows jjgte_bridled in the new sense only;
no reprieve episode is registered — this docket's rationale is the record;
claude-jjk-core.md carries the verb-table rows and the bridling, designee, and escalation protocol texts;
merge to main follows green tests with operator sanction.

### home-reprieve-doctrine (₢BcAAI) [complete]

**[260621-0917] complete**

## Character
A vocabulary rename wrapped around one piece of new doctrine prose — mostly mechanical.
Not a schema change: no on-disk gallops shape moves, so it lands on main via jjx_record without the branch quarantine or a reprieve episode.
The judgment is the subdoc's doctrine prose; the rest is a grep-sweep.

## Goal
Rename the schema-tolerance mechanism forgiveness -> reprieve across spec and code,
and home the schema-change delivery and convergence doctrine —
today stranded in the ₣Bc paddock and the coda docket —
in a new JJS0-included subdoc,
so the policy outlives heat retirement and stays in-context during future schema work.

## Cinched
- The word is reprieve: it encodes an episode's temporary-stay-meant-to-end lifecycle and reads fair-faced in the open nag.
- The doctrine lives as a JJS0 include:: subdoc (the gazette-concept-subdoc precedent),
  spliced where the mechanism block sits in == Serialization;
  it absorbs the delivery posture and the two-pass cross-clone convergence protocol from their perishable paddock and coda homes,
  beside the mechanism and the register-an-episode procedure lifted from JJS0.
- Not a schema change: pure identifier rename plus doc move, zero on-disk impact —
  this pace is the doctrine's own counter-example, the rule it documents does not bind it.
- The opaque rivet JJr_a7c is untouched by the readable-name change.

## Done when
The reprieve subdoc is authored and included in JJS0;
every forgiveness site (spec prose and code identifiers) reads reprieve;
the delivery posture and convergence protocol live in the subdoc rather than only the paddock and coda;
the conduct file cites the subdoc and the new word;
tt/vow-b and tt/vow-t green.

### forgiveness-rivet-and-probe (₢BcAAC) [complete]

**[260618-1426] complete**

## Character
Infrastructure — lift the schema-migration tolerance into a shared, cited, self-reporting mechanism ahead of the two changes that consume it.
Judgment lives in the probe shape; mechanical once shaped.

## Done when
- A named rivet constant (RCG constant discipline) holds the inert forgiveness token, and every legacy site cites it in place of the firemark-and-silks now hand-copied across code and spec.
- The migration-detection predicate inlined in jjdr_load (jjri_io.rs) is extracted to one pure read-only probe returning per-episode status — which forgivenesses are live versus dormant for the on-disk gallops; jjdr_load and the new open-time chatter are its only callers.
- Detection is generalized from "schema_version absent" to "schema_version differs from current", driven by a per-episode registry, and the existing V3 to V4 tolerance becomes the first registered episode.
- jjx_open emits a per-episode forgiveness status line carrying the rivet token: pending means still load-bearing here, dormant means candidate for removal once all clones agree.
  The peek is read-only, best-effort, and never gating — jjx_open must still always succeed.
- JJS0 seats a forgiveness quoin in the jjdz_ serialization category describing a PERMANENT mechanism (registry, probe, nag) with a per-episode lifecycle: the concept, each episode's demolition condition (no pre-its-schema gallops survives in any operated clone, gated on jjdk_sole_operator), the open-probe contract, the inline rationale for permanence (multi-install schema drift is the steady state, so mutually-readable schemas are driven to a floor rather than accumulating), and the standard procedure a future schema change follows to register a new episode — the durable operating manual that outlives this heat's V3-legacy removal, so the next change converges through the mechanism instead of reinventing it.
  The scattered V3-legacy markers (jjrt_v3_types.rs, the JJS0 V3 Legacy Schema Reference section, jjdpe_bridled, jjdcm_direction, the jjdgm_version and jjdgm_order migration notes) re-cite the quoin in place of the firemark.
- JJS0's "open touches no persistent state" property records the deliberate bend: a read-only diagnostic peek, non-gating.
- Build tt/vow-b, test tt/vow-t.

## Cinched
- The rivet stays inert in shipped code — opaque token in the binary, rationale in the veiled spec; this is the RBr_ rivet doctrine made JJK-native, since JJK rust ships and the specs do not.
- The probe is the single source of "what counts as old-format"; jjdr_load keeps no second copy.
- Per-episode from the start — at strip time two forgivenesses are live (the V3 to V4 case and the tack rework), so an episode's gate is "dormant on every clone", never a single boolean.
- The mechanism is permanent infrastructure, not scaffolding — multi-install drift is the tool's steady state, so the registry, probe, and nag stand; only episodes retire. The quoin describes a standing mechanism with a per-episode lifecycle, never a self-demolishing whole.
- One rivet, mechanism-level and permanent: a single inert token names the forgiveness mechanism and is cited by every participating site — mechanism machinery and per-episode tolerance code alike — and never retires. Per-episode specifics (old shape, schema number, demolition condition) live as registry data, never as per-episode rivets, so a recurring schema change registers a registry entry and cites the existing token rather than minting and later retiring its own. This is the cited-constraint anchor pattern: one anchor, many citers. The mount-time freedom is only the token's inert string and whether the legible machinery carries the cite too (the odd-looking tolerance code must).

## Sources
JJSRLD-load.adoc (migration mode), the jjx_open handler and jjdr_load / jjdr_save in the crate, RCG (constant discipline), ACG residue and MCM rivet (the cite-once pattern), CMK ROE Palisade membrane retirement (the demolition-date discipline).

### drop-gallops-schema-version (₢BcAAF) [complete]

**[260619-0749] complete**

## Character
Implementation — remove a serialized field and reframe detection as structural, plus one spec cinch.
Mechanical deletions; the load-bearing part is the doctrine, not the edits.
A schema-change window — it drops a gallops field and is itself a (self-describing) forgiveness episode.

## Cinched
Remove the schema_version field.
It is a redundant second source of truth — the on-disk shape already carries the version, and the V3→V4 live-test already detects structurally rather than trusting the number.
Detection becomes per-episode structural: each live-test sniffs the actual old shape, not a version compare.
That fits concurrent episodes — independent, order-free — better than a single monotonic integer.
Cinch in JJS0 jjdz_forgiveness: every schema change must be self-detectable — shape-changing or self-marked; an idempotent normalization needs no episode (normalize at point of use); a non-idempotent shape-preserving change must carry its own marker, since there is no global version oracle.
Its own removal is a self-describing episode: serde ignores the now-unknown key on old files.
Bring the uncited primed alias (jjrt_types.rs) under the mechanism in the same motion: if any operated clone still holds a "primed" pace state on disk, register it as a tracked episode with the rivet cited at the code site; if dormant everywhere, delete the alias. Either way it stops being an untracked tolerance.

## Consumer reconciliation
Every reader/writer of the version. Discovery: grep schema_version and JJDZ_CURRENT_SCHEMA across the crate.
The V3→V4 live-test loses its version clause — confirm its remaining structural signals still distinguish V3.

## Done when
The schema_version field is gone from the type, the validate gate, and all construction sites; the V3→V4 episode detects structurally without it; the primed alias is tracked or removed (no untracked tolerance remains); JJS0 cinches the self-detectable discipline; build tt/vow-b, test tt/vow-t green.

### tack-data-model-rework (₢BcAAA) [complete]

**[260619-1201] complete**

## Character
Implementation — wire-format-breaking tack-text change plus a one-time live-gallops conversion, riding the forgiveness mechanism.
Mechanical once registered; the conversion is the load-bearing risk, not the type edits.
A schema-change window — it changes the tack-text wire shape and rewrites the live gallops in place, so it is not a grab-anytime pace.

## Precondition (hard gate — operator confirmation required)
This pace rewrites the live gallops in place and is not atomic across clones.
The operator runs multiple repo clones in parallel; each clone carries its own gallops, synced through git.
Before any conversion work begins, the mount agent must obtain explicit operator confirmation that every clone is committed and pushed — no un-pushed gallops commits at the old schema in flight anywhere.
A lagging clone with un-pushed old-schema work would diverge from or collide with the converted store, and the forgiveness read-tolerance does not undo that collision.
This is an operator-only confirmation: this clone's clean-and-pushed tree is checkable by the agent but says nothing about the parallel clones, so do not infer the gate from it — ask, and wait for an explicit yes before touching the gallops.

## Cinched
Two transforms, one structural episode, one migration.
1. Tack text becomes a line array (newline-split) — pretty-JSON decomposes the docket one element per physical line.
2. Tack history leaves the JSON — the pace keeps a single current tack edited in place (redocket replaces, not prepends); evolution lives in git per JJS0 Git-as-Journal.
History stays preserve-the-tack-struct (keep the five fields ts/state/text/silks/basis).
Newline-split, not sentence-split — git's line-level merge granularity is the durable rationale; paragraph breaks fall out as empty elements.
Keep pretty JSON.
Do not reconcile against any other heat's data-model design — that design has not converged and is out of scope.

## Registering the episode (structural, no version bump)
The preceding pace removed schema_version, so detection is structural per JJS0 jjdz_forgiveness — there is no schema number to bump.
- Tolerate both shapes at the deserialize boundary: a custom deserialize accepting either a JSON string or an array of lines, normalizing to the line array, so the parse succeeds and the probe and write-forward run instead of fatalling first.
- The episode's live-test sniffs the old string-text shape in the raw bytes (post-normalization structs look identical either way); when live, migration mode skips the round-trip equality gate a string-to-array reserialization would otherwise fatal.
- Migration collapses any existing multi-tack array to its newest element and splits string text to lines; conversion is automatic on next load, persists at the next save/commit, idempotent on the load after.

## Consumer reconciliation
- Every reader of tack text now handles a line array, not a string. Discovery: grep the crate for `.text` on a tack.
- Two readers iterate the full tack history (the detail parade, the trophy builder); collapsing to one tack degrades both to current-only — intended (history goes to git). Confirm the framing; do not re-source from git. Readers of the current tack are unaffected.

## Weigh at mount
- Keep the field a length-1 Vec of tacks (replace-at-0) — least-breaking; first()/[0] readers stay unchanged, only the two iterators change behavior. Confirm versus collapsing the field to a single object.
- The redocket diff computes old-versus-new from the pre-edit current tack in-flight; verify it reaches for no prior tack.
- Text currency: carry the line array from parse through the tack and flatten only at gazette emit (deletes the existing split-join-split sandwich), or fall back to splitting only inside the tack if the gazette public surface proves too wide.
- Recheck the gazette spec still does not pin the internal text type (verified silent now).

## Scope honesty
Even paired, this only turns same-pace concurrent redockets from always-conflict to conflict-only-on-overlap.
It does NOT fix the structural merge collisions (concurrent seed/order allocation) — those stay on the merge-driver track.
Land before the gallops-normalize pace later in this heat, which canonicalizes the store — this changes what canonical is.

## Done when
Tack text serializes one element per physical line; each pace holds a single edited tack with no in-JSON history; redocket overwrites; the episode detects the old string shape structurally and converts the live gallops on next load, idempotent thereafter; round-trip passes; build tt/vow-b, test tt/vow-t.

### remove-tack-direction-field (₢BcAAG) [complete]

**[260620-1353] complete**

## Character
Schema change — remove the tack `direction` field.
Mechanical; the one judgment is whether it needs a forgiveness episode at all.

## Why (slate-time inspection)
`direction` — the bridled-state "warrant" — has no live producer: every production `jjrg_tally` caller (relabel, drop, wrap) passes `direction: None`, and no MCP-exposed verb sets state=bridled (the `jjx_arm` that orient/saddle recommends is unimplemented, not in the dispatch's command set).
It survives only in the type, validate Rule 9, the tally bridled-branch, and display readers (parade/scout/saddle/trophy), exercised only by unit tests — the spine of an unbuilt bridle/warrant surface.

## On-disk footprint — decide first
The field is `Option<String>` + `skip_serializing_if = Option::is_none`, so a never-set field leaves nothing on disk: removal likely round-trips clean with no episode.
Confirm whether `jjgtn_direction` appears in any on-disk gallops: none → no episode (serde already omits it); a stray key → serde ignores it on parse but the round-trip gate then mismatches, so register a structural episode sniffing `jjgtn_direction` per JJS0 jjdz_forgiveness.

## Weigh at mount
Removing the field makes validate Rule 9 and the tally bridled-branch vacuous — decide whether the bridled *state*, the warrant display, and the dangling `jjx_arm` recommendation also retire or stay as harmless vestige (out of scope unless trivially entailed).
Discovery: `grep -n direction *.rs`.

## Done when
The `direction` field is gone from the tack type and all readers; the bridled-direction logic it fed is resolved; build tt/vow-b and test tt/vow-t green; and — only if an on-disk key was found — an episode detects and converts it idempotently.

### validate-normalize-and-report (₢BcAAB) [complete]

**[260620-1546] complete**

## Character
Intricate but mechanical — rewiring existing load/save machinery, not new validation logic.

## What done looks like
`jjx_validate` becomes a deliberate normalize-and-report pass over the gallops, replacing today's read-only behavior (which fatals on any non-canonical-but-valid file via the load round-trip check).
It leans on the same canonicalization the forgiveness mechanism drives in migration mode.

Tri-state outcome, exit code carries the verdict so a caller branches without scraping stdout:
- exit 0 / clean — valid and already canonical; no write.
- exit 2 / normalized — valid but non-canonical; validate rewrote it to canonical form and committed.
- exit 1 / broken — could not load/validate (parse or invariant failure); file untouched.

## Cinched
- Fix scope is normalization only — key order, whitespace, heat_order population. Never invents or repairs missing/contradictory data; structural breakage is exit 1, never a silent fix.
- Idempotent — re-running after a 2 yields 0.
- Commit-on-normalize (operator decision, additive-safe). validate is gallops-wide, affiliated with neither heat nor pace, so the standard identity-keyed persist path does not fit — needs its own commit path (action code plus gallops-level or marker-style primitive).
- Correct under in-progress merge — committing with MERGE_HEAD present must finalize the merge (two parents), not leave it dangling. This finalization is intended: validate is the post-merge gallops cleanup step.
- Self-describing stdout — every outcome names its bucket and prints a census, so green is positive evidence, not mere silence.
- Reuse the shared forgiveness detection probe the infra pace extracts; validate-normalize and the open-time probe are the two consumers of that one predicate. Do not reimplement migration-mode detection.
- Delivered source-only per the heat's branch-delivery posture: build and unit-test the canonicalizer against temp fixtures; never run it against a live gallops. Forcing real conversions across installs is the coda's job, not this pace's.

## Canonicalize against the current heat schema
The canonical form has already moved twice in this heat: the tack data-model rework (docket text to a line array, tack history collapsed to one) and bridle retirement (the `bridled` pace state retired, the `jjgtn_direction` key dropped — `jjgte_bridled`/`primed` demote to rough).
Write the canonicalizer against that current form; never normalize toward an earlier shape.

## Also in scope
Update the JJK command-reference wording (CLAUDE.md JJK section / jjk core) so the entry states the three outcomes and the residual: normalized is not semantically correct; eyeball the heat/pace inventory against both branches yourself.

## Sources
JJSCVL-validate.adoc (rules), JJSRLD-load.adoc (round-trip plus the migration rewrite the probe now governs), JJSRSV-save.adoc (load-back validation), JJSRPS-persist.adoc (identity-keyed commit path this cannot reuse).

### chat-retention-config (₢BcAAH) [complete]

**[260621-1354] complete**

## Character
Schema change plus a small open-time read — mostly mechanical.
The one judgment is whether the new field needs a forgiveness episode.

## Goal
Make chat-history retention opt-in rather than compulsory:
add a gallops field that gates capture, surfaced through a monitum at open.
This is the precedent the chat-capture pace in ₣BD (₢BDAAY) consumes —
without it, capture cannot be made conditional.

## Cinched
- A top-level gallops field, optional; absent or empty means off.
That default is load-bearing, not convenient:
it lets the binary be shared with friends without absorbing their chat history.
- Gallops field deliberately — it rides clone-sync as one global policy applied
locally to each machine's own transcripts, which is what makes it a schema change
and homes it in this heat.
- The field is surfaced by a monitum at open: the generalized forgiveness-nag —
read-only, best-effort, never gating — in three states:
off (quiet), on-since-DATE, and MALFORMED/capture-disabled (loud).
- The monitum is the fail-loud: a typo'd date is a loud line every open, never a
hard gate, because bad config never makes the gallops illegitimate.
- Scope is field + read accessor + date validation + the monitum.
Scan-and-copy and the operator-facing setter stay with ₢BDAAY —
setting the date is inert until capture consumes it.
- Name the monitum concept in JJS0 near jjdz_forgiveness, with forgiveness its
first instance.
No forgiveness-code refactor, no monitum registry, no gate-side (interdict) word —
those wait for a third instance.

## Weigh at mount
Whether the field needs a forgiveness episode
(lean: no — an optional field absent on old gallops round-trips clean,
the mirror of this heat's direction-field removal),
and the matching old-binary clone-sync tolerance (does the parse reject unknown fields?).
Field and quoin identifiers are a slate-time mint left to mount.

## Sources
Monitum exemplar: zjjrm_forgiveness_nag (jjrm_mcp.rs); gate sibling zjjrm_managed_clean.
Concept home: JJS0 jjdz_forgiveness (registry/probe/nag);
the ambient/gate determination umbrella in ₣Bg's paddock.
Provenance: Memos/memo-20260615-chat-capture-and-cost-reconstruction.md.

## Done when
The gallops carries the optional retention field;
open emits the monitum in all three states;
a malformed date reads loud and disables capture without gating open;
the monitum concept is homed in JJS0 with forgiveness named its first instance;
capture itself stays untouched (₢BDAAY's);
tt/vow-b and tt/vow-t green.

### jjk-dealias-cipher-imports (₢BcAAJ) [complete]

**[260715-0559] complete**

## Character
Mechanical and crate-wide — a pure, behavior-preserving rename.
Per-file independent, so it fans out cleanly to parallel sonnet/haiku agents.

## Goal
Bring the JJK crate into conformance with RCG's new Alias Discipline:
de-alias every `use … as` that strips a project cipher prefix, rewriting the body use sites back to the canonical prefixed name.

## Cinched
- Project-owned (jjr*/JJR*) identifiers only.
Foreign-crate boundary aliases stay — RCG permits them (`rmcp::ErrorData as McpError`, `std::io::Read as IoRead`).
- Pure rename: no logic, signature, or test-semantics change; each file stays behaviorally identical.
- The fan-out unit is one source file; there is no cross-file coupling.

## Sources
The rule and its rationale: RCG "Alias Discipline".
Offender census (the work list): `rg ' as ' Tools/jjk/vov_veiled/src --type rust`, filtered to a jjr*/JJR* left-hand side.

## Done when
No `use` alias in the crate strips a project cipher prefix — the census grep returns only foreign-boundary cases.
tt/vow-b clean and tt/vow-t green; the rename is behavior-preserving, so the existing suite is the proof.

### pace-original-intent (₢BcAAK) [complete]

**[260715-0657] complete**

## Character
Design plus an additive-only gallops schema change; judgment work, not mechanical.

## Goal
Give a pace a frozen-at-slate "original intent" capture, surfaced at mount above the docket,
so the slate-time understanding is read before any later-reconstructed docket rationale.
It exists because a docket's rationale can be reconstructed over time
and later becomes indistinguishable from genuine intent;
an immutable, slate-frozen capture is the fix.
Chat-UUID / transcript linkage stays OUT of scope — a future problem, not this pace.

## Cinched
Capture BOTH, as two immutable fields frozen once at slate and never rewritten by reslate or redocket:
  - the operator's raw words/context, verbatim — the authenticity anchor;
  - the slating LLM's distillation of that context — the readable intent, attributed as LLM-authored.
The raw field is what strips the distillation of sole authority:
a future reader audits the distillation against the operator's own words,
so a confidently-wrong distillation always has recourse.
No ratification beat at slate — the operator's "paint context, move on" workflow is preserved;
read-time auditability (both fields present) replaces slate-time correction.
A redocket-count field increments on each reslate/redocket — the drift signal —
folded into this same schema change since the shape is changing anyway.
Mount surfaces the capture above the docket with:
a STANDING staleness caveat (a frozen field over a mutable docket is always possibly-stale),
the redocket count, a date annotation (age annotates, never adjudicates),
and an attribution label marking the distillation as LLM-authored from editor context.
Rejected, not to be reopened: show-on-demand (the operator can't predict when to ask);
a churn metric (invites false precision — the raw redocket count is honest);
elapsed time as the staleness adjudicator (false-alarms old-but-untouched intents, trains dismissal).

## Schema change
Additive-only: two frozen fields plus a counter, and no existing store migrates —
per the heat's bridle-revival cinch and JJSCRP's additive clause this registers NO reprieve episode
(forgiveness tolerates old stores, and an additive change leaves every old store natively readable via serde defaults).
§F branch delivery is retained: deliver source-only on a date-and-identity branch, commit no gallops conversion;
the branch merges at the coda's coordinated convergence.
Do not improvise this part.

## Guidance update
Update the JJK CLAUDE guidance (`claude-jjk*.md`) to direct the slating ceremony
to populate both fields correctly and only during slating — not reslate, not later.
In the same act, amend §E's blanket must-register rule with the additive carve-out
(citing the bridle-revival determination and JJSCRP's additive clause),
so additive changes stop re-litigating the episode question.

## Done when
A newly slated pace carries both immutable fields (raw + distilled) plus the redocket counter;
mount surfaces them above the docket with the staleness caveat, count, date annotation, and attribution label;
the `claude-jjk*.md` guidance directs correct slate-only population and §E carries the additive carve-out;
the change is delivered per §F with no episode registered (additive-only); older on-disk gallops read natively.

## opus mount provided
Scout findings from an opus mount on 260715 (read-only; nothing was built or committed).
Banked at operator request so the executing session need not re-scout.
This section is scaffolding for the build, not a re-opening of the cinches above — where it names a call it is direction, not a fork.

### Structural findings (verified by reading the crate)
The JJK crate lives at `Tools/jjk/vov_veiled/src/` (source files `jjr*`, test files `jjt*`).
A pace's `tacks` field is typed as a vector but the code holds exactly ONE tack: `jjrg_set_tack` does `pace.tacks = vec![tack]` on every write, and tack history lives in git, not the JSON.
So every reslate/bridle/release REPLACES the single tack — a field hung on the tack is wiped on the next docket edit.
Therefore the frozen intent and the counter must live on the `jjrg_Pace` struct itself (which survives tack replacement), NOT on the tack.
Copy the existing additive pattern verbatim: `tier`, `effort` (on `jjrg_Tack`) and `retention_since` (on `jjrg_Gallops`) already ride as `#[serde(default, skip_serializing_if = "…", rename = "…")]` — that is the blessed "additive, no reprieve episode, old stores byte-identical" mechanism this pace needs.

### Baked answers (collapsing the mount's forks — do not re-fork these)
- Field set: THREE frozen-at-slate values — verbatim, distillation, and the slate date — plus ONE mutable counter. The slate-date is an addition beyond the docket's literal "two fields + counter": mount's required date annotation needs a stored source, and nothing today persists a pace's slate time (the tack ts is overwritten by later edits). Storing it frozen keeps mount a pure gallops read. (Operator confirmed this elaboration.)
- Transport: the operator's verbatim words and the distillation arrive as TWO new gazette slugs written into `gazette_in.md` beside the `jjezs_slate` notice, bound to it by a shared lede. NOT MCP params — docket-shaped content routes through the gazette by law.
- Counter increment site: in `jjrg_revise_docket` ONLY. It is the single funnel both single and mass reslate pass through, so one increment there covers both. Bridle and release also call `jjrg_set_tack` but must NOT increment; keep the bump in `jjrg_revise_docket`, not in `jjrg_set_tack`.
- Immutability across relocation: draft and restring build a fresh `jjrg_Pace` with a new coronet — carry the three frozen fields AND the counter forward (immutable provenance travels with the work). They are relocations, not reslates, so they never re-freeze and never increment.
- Wire strictness: the two intent notices are OPTIONAL at the tool (additive, graceful, old flows and non-ceremony slates still work) and REQUIRED by the written ceremony/guidance (the slating LLM always populates them). This matches the optional-field precedent and preserves "paint context, move on".

### Scope
Wire the primary slate path fully: `jjx_enroll` (MCP `JJRM_CMD_NAME_ENROLL` dispatch) and thus the slate/chivvy/cantle verbs.
Batch-born paces via `jjx_redocket` (its folded-in slate notices) may carry no intent for now — leave that a DOCUMENTED, deliberate follow-on, not a silent gap. Do not expand the batch parser this pace.

### Naming (operator's domain — confirm or re-mint at bridle; run the grep gate before adoption)
Recommended so fable has a runnable default, not a mandate:
- verbatim field/slug: `dictation` — `jjgpn_dictation` (pace field), `jjezs_dictation` (slug)
- distillation field/slug: `gloss` — `jjgpn_gloss` (pace field), `jjezs_gloss` (slug)
- frozen slate date: `jjgpn_slated` (or per operator mint), same `jjgpn_` pace-field prefix
- counter: `jjgpn_redocket_count`
- mount block label: "Original intent"; distillation carries an "LLM-authored from editor context" attribution tag.
`dictation`/`gloss` sit in the existing records asterism (paddock, steeplechase, silks, halter); each is one evocative word carried identically wire→store. The two field-key words are PERSISTED, so a later rename is itself a schema change — settle them before writing the struct.

### Implementation map (durable symbol anchors — line numbers deliberately omitted)
- Schema: `jjrt_types.rs` — add the four fields to `struct jjrg_Pace` (derive `Default` and add a zero-predicate helper for the counter's `skip_serializing_if`); extend `jjrg_SlateArgs` to carry the two intent strings + the slate stamp.
- Slate set-once: `jjro_ops.rs` `jjrg_slate` (constructs the `jjrg_Pace`); carry-forward in `jjro_ops.rs` `jjrg_draft` and the restring path.
- Increment: `jjrg_gallops.rs` `jjrg_revise_docket`.
- Transport: `jjrz_gazette.rs` — extend `enum jjrz_Slug` (+ the wire consts, `jjrz_from_str`, `jjrz_as_str`, `jjrz_direction`, `JJRZ_ALL_SLUGS`), and extend `jjrz_parse_slate_input` to also return the two intent bodies; thread through `jjrsl_slate.rs` `jjrsl_run_slate` and the `JJRM_CMD_NAME_ENROLL` arm in `jjrm_mcp.rs`.
- Mount render: `jjrmt_mount.rs` `jjrmt_run_mount` — emit the intent block just before the `Docket:` block; both the specific-coronet and first-actionable resolution paths must read the pace-level fields.
- Tests: many `jjrg_Pace { tacks: … }` literals across `jjt*` fixtures need `..Default::default()` once the struct grows; add coverage in `jjtg_gallops.rs`, `jjtz_gazette.rs`, `jjtm_mcp.rs`.
- Guidance: edit the KIT SOURCE `Tools/jjk/claude-jjk-core.md` and veiled `Tools/jjk/vov_veiled/claude-jjk-bhyslop.md` §E — NOT the installed `.claude/` copy (per `Tools/jjk/CLAUDE.md`).

### Delivery under §F (do not improvise)
Branch `bhyslop-260715-BcAAK-pace-original-intent`; all work source-only there.
Do NOT rebuild/reinstall the live `jjx` binary (a freshly-built new-schema binary that saves a gallops would bake the shape in ahead of the coda's coordinated convergence).
Prove behavior with the crate's unit/integration tests (`tt/vow-t.Test.sh`), which run against in-memory fixtures and never touch the live gallops — a live end-to-end slate demo is deferred to the coda by design.
Notch before every test run (§H).

### gallops-key-glyph-strip (₢BcAAL) [abandoned]

**[260711-1433] abandoned**

## Character
Design plus a gallops schema change — reprieve machinery in scope, so judgment work, not mechanical.
Bounded: the target form is spec-mandated and the bare-form render home already exists; the work is the read-tolerance + write-forward and its episode.

## Goal
Migrate the on-disk gallops identity keys from their glyph-prefixed form (`₣BD`, `₢BDAAn`) to the spec-mandated bare form (`BD`, `BDAAn`).
AXLA `axd_insignia` / JJS0 `jjdt_insignia` forbid the render glyph in machine contexts — JSON keys among them — but the gallops `heats`/`paces` maps and the `order` vectors still carry it.

## Cinched
- The target is the bare body (glyph stripped); it is spec-mandated, not a design choice.
The insignia-ADT render home already produces it (`jjrf_as_str` in `jjrf_favor.rs`); this pace routes the gallops key-production sites off `jjrf_display` onto the bare form, leaving operator-output rendering glyphed.
- Wire-format-breaking: it registers an episode against the forgiveness spine (read old glyph-keyed stores, write bare) — it does not invent its own trigger.
Retirement follows the standard demolition condition (dormant on every operated clone), governed by the coda, not presumed into any specific coda pass.

## Schema change
Changes the gallops on-disk key shape — follow §E (reprieve-episode gate), §F (branch naming), and JJSCRP (`jjdz_reprieve`): register a reprieve episode, deliver source-only on a date-and-identity branch, commit no gallops conversion.
Do not improvise this part.

## Discovery
Key-production sites: `grep -rn 'jjrf_display' Tools/jjk/vov_veiled/src` — separate the gallops-key uses (jjrg_gallops resolve/set, jjrm_mcp key-normalize, jjro_ops coronet-string construction) from operator-output uses, which keep the glyph.
Read-tolerance + loader home: JJSCRP (`jjdz_reprieve`) and `jjri_io.rs`.
Officium-local ephemeral JSON keys (pensum seeds, saddle marker) also carry glyphs but need no episode — rewritten per session; mount decides whether to fold them into the same branch.

## Done when
The gallops `heats`/`paces` map keys and the `heat_order`/`order` vectors serialize glyph-stripped;
a reprieve episode is registered per JJSCRP and older glyph-keyed gallops still load (read-tolerance + write-forward);
build tt/vow-b and test tt/vow-t green.

### pace-identity-regestalt-impl (₢BcAAO) [complete]

**[260717-1845] complete**

## Character
Gallops schema change — reprieve-gated; judgment work, not mechanical.
Relocated (260708) from the studbook–farrier heat (₣Br), which does not depend on it:
its billet layer needs only the git-ref- and dirname-safe id shape, which grandfathering preserves,
and its scratch rehearsal exercises no transfer or restring.

## Session findings
Slated across a station→cerebro transfer (260715): the code map, the reprieve-episode plan, and a
delivery-sequencing hazard are banked as provisional provenance in
`Memos/memo-20260715-pace-identity-regestalt-transfer.md` — verify against the live tree at mount;
the docket and the JJS* specs are the authority, the memo is not.

## Goal
Pace identities become immutable for life:
minted atomically from one global seed under the store's single-writer lock
(the seed is store data — it lives in the gallops now and migrates with the store when the studbook becomes authoritative);
restring and transfer re-affiliate without re-keying;
display is heat-qualified at emission only, rendered from live affiliation at output time.

## Cinched
Authority: JJSAS "Pace identity re-gestalt" (`Tools/jjk/vov_veiled/JJSAS-state-repo.adoc`),
plus the ₣Br paddock's 260706 settlements, which travel with this pace as its contract:
- Pace-id shape: five charset characters from the one global seed;
  existing coronets grandfather verbatim — immutable-for-life includes the already-living —
  and the seed founds above the highest grandfathered body,
  so collision with the legacy heat-embedded space is structurally impossible
  (one founding-time comparison, no reserved-set check).
- Founding value (operator ruling 260715): the seed founds at max(highest grandfathered body + 1, CAAAA).
  The CAAAA floor is a legibility landmark, not a safety need:
  every grandfathered id leads with A or B, so every new-era id visibly leads with C or later.
  The max() keeps the one-comparison guarantee intact if legacy minting ever reaches C-space before conversion;
  no reserved-set check enters.
- Wire-vs-display: the bare immutable id is the identity in every machine context
  (gallops keys, MCP params, git refs; billet branch names when the ₣Br surface consumes it);
  heat qualification is emission-only — glyph + current heat + interpunct + immutable id —
  so transfer changes tomorrow's rendering, never the identity;
  input tolerates emitted forms (glyph and qualifier stripped on ingest, bare id looked up);
  frozen emissions keep the qualified form as record-of-the-moment, never a live reference;
  full-form never-abbreviate display discipline carries over, firemark display untouched.
- Halter typing: strip the glyph if present;
  a token containing the interpunct is a qualified coronet — the five-character tail resolves, qualifier ignored;
  otherwise type by length exactly as today (2 firemark, 5 coronet);
  the %-sentinel pensum untouched.
Spec scope (operator ruling 260715, supersedes the docket's original bank-into-JSSAS-only framing):
infuse the re-gestalt into normative JJS0 as the settlements' durable home —
the coronet quoin, its Types-table row, the encode/decode algorithm, the halter rule,
and the pace-seed member relocated from heat to gallops —
drain the JJSAS "Pace identity re-gestalt" section to a pointer and close its wire-vs-display open fork,
and sweep every other affected JJS*.adoc so the whole binding codex reads truthfully about immutable-for-life ids.
Durable-first: the specs commit before the code and before any provenance trims;
the founding-value ruling banks with them.
The revision-insignia sub-mint (catchword/varvel) does NOT travel — it stays ₣Br design context;
it shared only the design pace, not this surface.

## Schema change
MANDATORY: CLAUDE.md §E and §F and JJSCRP (jjdz_reprieve) govern delivery —
reprieve episode registered, source-only date-and-identity branch, no gallops conversion committed.
Do not improvise this part.

## Sequencing
Must land and converge before the JJ revision-control conversion/cutover heat:
live billet branches embed the immutable id and must not rot on transfer.

## Done when
Pace identities are immutable for life, minted atomically under the store lock from the global seed;
restring and transfer re-affiliate without re-keying;
display renders heat-qualified with the interpunct and ingest tolerates emitted forms per the cinches above;
the coronet re-gestalt is infused into JJS0 and every affected JJS* sheaf reads truthfully (the JJSAS open fork drained closed);
a reprieve episode is registered and older on-disk gallops still read;
build tt/vow-b, test tt/vow-t green.

### propagate-and-retire-episodes (₢BcAAD) [complete]

**[260718-0910] complete**

## Character
Operations plus cleanup — merge this heat's standing §F schema branches to main, converge every operated clone, then retire the spent episodes — every episode the registry lists dormant at execution time — leaving the mechanism standing.
The load-bearing risk is stripping an episode before every operated clone has actually converted.

## First act — end the quarantine
Merge every standing §F date-and-identity schema branch of this heat to main and build the episode-bearing vvx from the merged tree;
the branches were the quarantine, and this coordinated convergence is the sole act that ends it.

## The operated set
Take the install census at mount — the operated-clone set is whatever it is that day, never this docket's memory.
As of the 260715 reslate the set is this repo's clones alone, sharing one origin,
so convergence collapses to: merge, build, force conversion via the normalize-and-commit jjx_validate, push, and each clone pulls;
the former binary-only remotes (paneboard, obsidian/jupyter) are no longer operated.
If any binary-only install exists at mount, its convergence is two passes with a gate between:
first deliver the episode-bearing vvx, force conversion, and confirm every episode dormant via the open-time probe;
only then deliver a vvx with the converged episodes' tolerances stripped.
Delivering the stripped binary before conversion is confirmed leaves a binary that cannot read a still-old store — the failure to avoid.

## Cinched
- This coda owns the retirement of every spent episode the registry lists at execution time — the registry and its open-time nag are the census, never a hand list — and removes their registry entries; the mechanism (registry, probe, open-time nag) stays standing.
- The retire gate is "the episode dormant on every operated clone" — never retire on a single dormant reading; an episode not yet dormant everywhere stays registered and rides to a later convergence.
- Per jjdk_sole_operator the operated clones are a finite, enumerable set, which is what makes "dormant on every instance in the world" a checkable condition rather than an act of faith.
- Mechanism is permanent — this pace strips per-version tolerances, frozen types, and their registry rows, never the registry, probe, or nag.

## Done when
Every operated clone runs a binary whose on-disk gallops is canonical at this heat's final schema and whose open-time probe reports every episode dormant;
every spent episode's tolerance and frozen reference is stripped from source and its registry entry removed (the registry at execution time is the census);
the mechanism — registry, probe, open-time nag — stands;
the forgiveness quoin stays as the permanent mechanism's home with the spent episodes marked discharged;
build tt/vow-b, test tt/vow-t green after removal.

### retire-v3-legacy-arg-quoins (₢BcAAE) [complete]

**[260717-2118] complete**

## Character
Cleanup — mechanical, with a verify-dead gate before each removal.

## Done when
The three remaining V3-legacy MCP-argument quoins — jjda_state, jjda_pace, jjda_created — and their residual Rust arg-struct fields are removed, eliminating the last hand-copied ₣An firemark references in JJS0.
(jjda_direction was already retired with the tack `direction` field in the bridle-retirement pace.)
Each is confirmed unexposed as a jjx_tool parameter and unread before removal;
a field still wired to a live V3-compat path stays until that path is gone.
Build tt/vow-b, test tt/vow-t green.

## Cinched
These are dead parameters, not schema tolerances — removable independent of the forgiveness episodes' demolition condition.
This pace does not touch the forgiveness mechanism, its episodes, or the V3→V4 frozen reference; the coda owns those.
Lands after the coda so any bridled/direction entanglement the coda resolves is already settled.

## Sources
Discovery: grep '₣An' Tools/jjk/vov_veiled/JJS0_JobJockeySpec.adoc (the survivors after the forgiveness re-cite);
the jjda_* quoins in the JJS0 Arguments section; their arg-struct fields in jjrt_types.rs.

## Commit Activity

```

File touches (file: the paces whose commits touched it):

  N  ₢BcAAN  apostille-command-rename
  M  ₢BcAAM  bridle-verb-and-mount-guard
  I  ₢BcAAI  home-reprieve-doctrine
  C  ₢BcAAC  forgiveness-rivet-and-probe
  F  ₢BcAAF  drop-gallops-schema-version
  A  ₢BcAAA  tack-data-model-rework
  G  ₢BcAAG  remove-tack-direction-field
  B  ₢BcAAB  validate-normalize-and-report
  H  ₢BcAAH  chat-retention-config
  J  ₢BcAAJ  jjk-dealias-cipher-imports
  K  ₢BcAAK  pace-original-intent
  O  ₢BcAAO  pace-identity-regestalt-impl
  D  ₢BcAAD  propagate-and-retire-episodes
  E  ₢BcAAE  retire-v3-legacy-arg-quoins

  JJS0_JobJockeySpec.adoc                            N M I C F A O D E
  jjri_io.rs                                         M I C F A B O D
  jjrm_mcp.rs                                        N M I C B J K O
  jjtg_gallops.rs                                    M I F A B K O D
  jjrg_gallops.rs                                    N M F A J K O
  jjrt_types.rs                                      M I F A K O D
  jjtq_query.rs                                      M F A J K O
  jjro_ops.rs                                        M A J K O
  jjtfu_furlough.rs                                  M F A K O
  jjtm_mcp.rs                                        N M J K O
  jjtpd_parade.rs                                    M F A K O
  jjtrl_rail.rs                                      M F A K O
  jjtrs_restring.rs                                  M F A K O
  jjtsc_scout.rs                                     M F A K O
  lib.rs                                             I C F B D
  claude-jjk-core.md                                 N M B K
  jjrpd_parade.rs                                    M A J O
  jjrv_validate.rs                                   M F A O
  JJSCVL-validate.adoc                               I B O
  jjrgc_get_coronets.rs                              M J O
  jjrgs_get_spec.rs                                  A J O
  jjrmt_mount.rs                                     J K O
  jjrno_nominate.rs                                  F J O
  jjrt_v3_types.rs                                   I C D
  jjru_util.rs                                       M A J
  jjrwp_wrap.rs                                      M J O
  claude-jjk-bhyslop.md                              I K
  jjrch_chalk.rs                                     J O
  jjrdr_draft.rs                                     J O
  jjrf_favor.rs                                      O D
  jjrnc_notch.rs                                     J O
  jjrrs_restring.rs                                  M J
  jjrsd_saddle.rs                                    M A
  jjrsl_slate.rs                                     J K
  jjrtl_tally.rs                                     J O
  jjrvl_validate.rs                                  I B
  jjtcu_curry.rs                                     J O
  jjtdm_muck.rs                                      K O
  jjtds_spine.rs                                     K O
  jjtvb_blotter.rs                                   O D
  JJSAS-state-repo.adoc                              O
  JJSCCH-chalk.adoc                                  O
  JJSCDR-draft.adoc                                  O
  JJSCGZ-gazette.adoc                                O
  JJSCNO-nominate.adoc                               O
  JJSCPD-parade.adoc                                 O
  JJSCRP-reprieve.adoc                               I
  JJSCRS-restring.adoc                               O
  JJSCSL-slate.adoc                                  O
  JJSCTL-tally.adoc                                  O
  MCM-MetaConceptModel.adoc                          M
  claude-cmk-core.md                                 C
  jjrcu_curry.rs                                     J
  jjrds_spine.rs                                     O
  jjrfu_furlough.rs                                  J
  jjrld_landing.rs                                   J
  jjrmu_muster.rs                                    J
  jjrn_notch.rs                                      J
  jjrnm_markers.rs                                   B
  jjrrl_rail.rs                                      J
  jjrrn_rein.rs                                      J
  jjrrt_retire.rs                                    J
  jjrs_steeplechase.rs                               J
  jjrsc_scout.rs                                     A
  jjrz_gazette.rs                                    K
  jjtf_favor.rs                                      O
  jjtn_notch.rs                                      J
  jjtz_gazette.rs                                    K
  memo-20260715-coronet-regestalt-code-handoff.md    O
  memo-20260715-pace-identity-regestalt-transfer.md  O
  vvcp_probe.rs                                      C

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 126 commits)

  1 K pace-original-intent
  2 O pace-identity-regestalt-impl
  3 E retire-v3-legacy-arg-quoins
  4 D propagate-and-retire-episodes

123456789abcdefghijklmnopqrstuvwxyz
xx·································  K  2c
·······x··x·xxxxxxxxxxxx···········  O  14c
························xxxxx······  E  5c
·····························xxxx·x  D  5c
```

## Steeplechase

### 2026-07-18 09:10 - ₢BcAAD - W

Ended the ₣Bc schema-migration quarantine (coda). Frontier-reviewed the sonnet designee's landing (all four episodes stripped, compile fallout fixed, spec propagated) and confirmed it correct and appropriately scoped. Verified the forgiveness mechanism stands — registry shell &[], jjdz_probe, jjdz_write_forward shell, jjx_open nag, JJr_a7c rivet — with the frozen jjrt_v3_types.rs gone and no live references. Confirmed the discharge ledger in the code registry comment; JJS0/JJSCRP left episode-generic. jjx_validate clean/canonical, zero episodes registered. Census: two operated clones (this gamma station + the alpha sibling clone), both already carrying the strip in source over a shared canonical gallops; no binary-only remotes, so single-pass convergence. tt/vow-b green (exit 0), tt/vow-t green (617 passed, 0 failed).

### 2026-07-18 08:56 - Heat - T

released ₢BcAAD to rough

### 2026-07-17 22:04 - ₢BcAAD - L

claude-sonnet-5 landed

### 2026-07-17 22:02 - ₢BcAAD - n

Propagate the code-level episode strip into the normative spec JJS0_JobJockeySpec.adoc, which the code change made factually stale in several places. Removed the '== V3 Legacy Schema Reference' section (jjrt_v3_types.rs mirror) per its own explicit instruction: 'Do NOT modify -- removed under jjdz_reprieve once that episode is dormant on every operated clone' -- now true everywhere. Corrected three now-false present-tense claims to historical past-tense notes: the primed-token demotion sentence under jjdcm_state, the heat_order populate-on-load write-forward under jjdgm_order, and the next_pace_seed founding write-forward (both the jjdgm_pace_seed member and the jjghn_next_pace_seed old-format-store paragraph under jjdhm_seed). Left the jjdz_reprieve quoin definition itself, the Reprieve doctrine subdocument, the jjdr_hark reach description, and the jjdcm_text/jjdcm_tier members untouched -- all already mechanism-generic with no episode-specific claims needing correction. This extends beyond the explicitly-named Rust source files in the docket; flagged in the landing report for frontier review.

### 2026-07-17 21:54 - ₢BcAAD - n

Fix compile-guided fallout from the episode strip: jjtvb_found_instance_is_immediately_ready_for_the_journal_ceremony wrote its seed gallops via compact serde_json::to_string, which only passed jjdr_load's round-trip canonical check because the now-removed V3->V4 episode's heat_order.is_empty() check also fired on this fixture's empty heat_order, forcing migration mode and standing the round-trip gate down as an unrelated side effect. Switched the seed writer to to_string_pretty to match the canonical on-disk form jjdr_save always produces -- no live V4 behavior changed, only the test fixture now writes what a real gallops file actually looks like.

### 2026-07-17 21:51 - ₢BcAAD - n

Strip all four spent reprieve episodes (V3->V4, schema_version drop, tack text->lines, pace-seed heat->global), confirmed dormant on every operated clone by jjx_open at mount. Removed each episode's ZJJDZ_REGISTRY entry and is_live fn, its write-forward body (registry now &[], write-forward now an empty shell), and its old-shape tolerance: the primed serde alias and the frozen jjrt_v3_types.rs reference module (V3->V4), the zjjrg_deserialize_text custom deserializer (tack text->lines), and the next_pace_seed serde default (pace-seed heat->global). Removed the now-orphaned zjjdz_found_pace_seed helper and the four episode-specific tests. Mechanism (registry shell, probe, write-forward shell, jjx_open nag, JJr_a7c rivet citation) left standing. Updated stale doc comments referencing the removed logic.

### 2026-07-17 21:18 - ₢BcAAE - W

Retired the V3-legacy arg-quoin residue in JJS0. Of the three docket targets, only jjda_pace was genuinely dead (no backing Rust field, no spec citation, no CLI flag) and was removed. jjda_state and jjda_created back live, essential fields (jjrg_TallyArgs.state: close->complete/drop->abandoned/relabel->unset; jjrg_NominateArgs.created: procedure-layer I/O capture, YYMMDD-validated, cited by JJSCNO-nominate.adoc) and were correctly retained — the docket's dead-parameter premise held for 1 of 3. Their now-false 'V3-legacy./Removal deferred to An.' framing was corrected in place to describe them accurately as permanent internal-only arguments. Net effect: zero hand-copied An references remain in JJS0 (the docket's actual goal); the two live fields stay under the docket's own 'field wired to a live path stays' escape valve. Build tt/vow-b + test tt/vow-t green (620 passed/0 failed) throughout. Adjacent staleness flagged for a separate itch: JJSCNO-nominate.adoc lists jjda_created as a required Argument beside the externally-supplied jjda_silks without marking it internal-only.

### 2026-07-17 21:17 - ₢BcAAE - n

Opus re-review correction: the prior jjda_state rewrite (authored at sonnet tier during a sonnet-reviews-sonnet window) was factually wrong — it called the transition 'tally-family' while citing jjdo_close (which is NOT in the tally family, only shares the jjrg_tally core) and claimed 'redocket passes none to inherit', but redocket routes through jjrg_revise_docket and never touches state. Corrected to the code-verified writers of jjrg_TallyArgs.state: jjdo_close→complete, jjdo_drop→abandoned, jjdo_relabel→unset(hold current). jjda_created definition confirmed accurate, left as-is.

### 2026-07-17 21:09 - ₢BcAAE - n

Frontier review fix: correct the now-false 'V3-legacy. Removal deferred to ₣An.' framing on jjda_state and jjda_created's own definitions — sonnet's designee investigation proved both back live, essential Rust fields (state-transition mechanism in jjrg_tally; YYMMDD-validated creation_time in jjrg_nominate), so the stale removal-candidate text is corrected to describe them as permanent internal-only arguments.

### 2026-07-17 21:02 - ₢BcAAE - L

claude-sonnet-5 landed

### 2026-07-17 21:02 - ₢BcAAE - n

Remove the jjda_pace quoin (mapping ref + definition) from JJS0 — verified fully dead: zero backing Rust field anywhere in the crate, zero citation from any operation spec sheaf beyond its own definition. jjda_state and jjda_pace are NOT touched: their backing Rust fields (jjrg_TallyArgs.state, jjrg_NominateArgs.created) are live and essential (wrap/drop state transitions; the deliberate I/O-boundary-capture pattern shared with jjrg_SlateArgs.slated), contradicting the docket's dead-parameter premise for those two.

### 2026-07-17 18:45 - ₢BcAAO - W

Coronet re-gestalt code phase complete, delivered §F source-only. jjtrs_restring reworked to move-under-same-key (immutable coronet unchanged across transfer; obsolete draft-note/dest-seed tests morphed to carries-tack-verbatim + leaves-global-seed-unchanged; the wrong-heat-decode case became source-membership). Two straggler compile fixes the mechanical sweep missed (jjrno_nominate inline test literal; jjtg validate test repurposed into cross-heat-uniqueness). vow-t swept to green. Step 4 display/ingest: jjrf_bare — the single ingest-normalization home (JJS0 jjdz_encoding) — strips glyph + interpunct qualifier; routed through it are jjrf_Coronet::jjrf_parse, the dispatch spine, and the five glyph-only length-typers (mount/parade/chalk/notch/mcp normalize-lede-emblem-apostille). Emission: jjrg_qualify_coronet scans live affiliation to render the heat-qualified ₢Bc·CAAAB, fail-soft to bare, routed through the primary glance surfaces (jjx_coronets, parade Pace/table/Next, orient Next, wrap Next, groom emblem); machine contexts (gallops keys, gazette wire, git refs, MCP params) stay bare. Four new unit tests + updated jtq output tests; final vow-t 608 passed / 0 failed / 6 ignored. The Done-when 'build tt/vow-b green' clause is DISCHARGED AT THE CODA ₢BcAAD per this docket's MANDATORY ## Schema change section (reprieve episode registered, source-only, no gallops conversion committed) — NOT run here; the live binary is untouched (confirmed old-schema, mtime 2026-07-15 15:44), matching the ₢BcAAK source-only precedent. This §F branch stands for the coda's coordinated merge.

### 2026-07-16 00:37 - ₢BcAAO - n

Step 4 tests. jjtq_query: update the two jjx_coronets output tests to the heat-qualified renderings (₢AC·ACAAD etc.) the emission change now produces; the default-tags test asserts the qualified tagged+bare lines, and the remaining-exclusion test checks the bare body ACAAD so it holds regardless of display form. jjtf_favor: new jjrf_bare round-trip test (glyph + interpunct stripped across every emitted form, grandfathered heat-twice included) and a coronet-parse tolerance test proving the qualified form ingests to the same bare identity. jjtg_gallops: new jjrg_qualify_coronet tests — renders ₢AB·ABAAA from live affiliation (accepting bare/glyphless/already-qualified input) and fails soft to bare when no heat harbours the coronet.

### 2026-07-16 00:34 - ₢BcAAO - n

Step 4 of the coronet re-gestalt — heat-qualified display and interpunct-aware ingest (JJS0 jjdt_coronet 'Display and ingest' + jjdz_encoding 'Input flexibility'). INGEST consolidation: new jjrf_bare(token) in jjrf_favor strips the glyph and any interpunct qualifier to the bare body — the single normalization home the spec's 'consolidate to jjdz_encoding' calls for. Routed through it: jjrf_Coronet::jjrf_parse (so every op tolerates the qualified form), the spine's jjrds_type_target (folded its inline rsplit logic in), and the five glyph-only length-typers that previously gapped — mount, parade (zjjrpd_strip_glyph removed), chalk, notch, and the MCP normalize/lede/emblem-resolve helpers plus the record designation gate. EMISSION: new jjrg_Gallops::jjrg_qualify_coronet(coronet) scans live affiliation and renders the heat-qualified form (glyph + heat firemark + interpunct + body), fail-soft to bare when no heat harbours it. Routed the primary glance surfaces through it: the jjx_coronets listing (coronet stays first whitespace token), the parade/show Pace line + table cell + Next, the orient Next line, the wrap AGENT_RESPONSE Next, and the groom emblem identity (qualified where its heat resolves; normalize_identity now preserves an already-qualified stored marker so the per-engagement re-display never strips it). Machine contexts (gallops keys, gazette wire, git refs, MCP params, file lookups) and error strings stay bare per the spec. Tests next.

### 2026-07-15 18:54 - ₢BcAAO - n

Sweep the lone runtime failure to green: jjtg_validate_pace_key_wrong_heat_identity asserted the retired 'must embed parent heat identity' validate rule, which the re-gestalt dropped (a flat Coronet embeds no heat), so validate now returns Ok and the test's unwrap_err panicked. Repurpose it into jjtg_validate_coronet_cross_heat_uniqueness — the invariant the source comment names as the replacement: the same immutable Coronet in two heats is rejected ('appears in more than one heat'). Fills a real coverage gap (no uniqueness test existed) while retiring dead coverage.

### 2026-07-15 18:52 - ₢BcAAO - n

Compile fix: the mechanical next_pace_seed sweep missed the inline test module in jjrno_nominate.rs — add the global next_pace_seed (CAAAA) to its jjrg_Gallops literal. Repo-wide scan confirms this was the last struct literal missing the field and no stale per-heat-seed access or removed-coronet-parent-helper references remain in source or tests.

### 2026-07-15 18:49 - ₢BcAAO - n

Rework the last untouched test file to the coronet re-gestalt's move-under-same-key model. jjtrs_restring tests the bulk jjrg_restring wrapper; the source now re-affiliates each pace under its SAME immutable coronet (no re-key, no per-heat seed) and carries non-bridled tacks verbatim (the old 'Drafted from' draft-note is gone). Helpers: make_valid_gallops carries the global next_pace_seed (CAAAA floor); make_heat_with_paces drops the retired per-heat jjrg_Heat.next_pace_seed field and its unused local. Error cases: the retired heat-embedding decode check is gone, so the former wrong-heat-identity test becomes coronet_in_dest_not_source (a live coronet living in the dest heat, rejected as absent from source) asserting the new 'not found in heat' message. Success cases: single/multiple now assert new_coronet==old_coronet and the dest order holds the same immutable keys in transfer order; the obsolete draft-note test becomes carries_tack_verbatim (single current tack, original text/silks, no injected note); the obsolete dest-pace-seed-increment test becomes leaves_global_seed_unchanged (a move must not advance the mint cursor). Draft-level bridle->rough revert stays covered in jjtg_gallops, so jjtrs stays focused on restring-specific concerns.

### 2026-07-15 18:37 - ₢BcAAO - n

Banner the transfer memo as superseded for current state: the code phase has begun, so its 'no bytes edited yet' is historical; point readers to the code-handoff memo. Prevents the resumption chat from mistaking the transfer memo's clean-start framing for the live state, without reslating the pace (which would wipe the opus bridle).

### 2026-07-15 18:33 - ₢BcAAO - n

Handoff memo for the coronet re-gestalt code phase across a chat restart: commit sequence, what source/spec is done (compiles), remaining work (jjtrs_restring rework, vow-t to green, step-4 display/ingest, vow-b last), and the standing sequencing hazard. Provenance; retires when BcAAO lands.

### 2026-07-15 18:32 - ₢BcAAO - n

WIP test-fixture + API/behavior rework for the coronet re-gestalt. Mechanical fixtures (9 files, via subagent): removed the retired per-heat jjrg_Heat.next_pace_seed and added the new global jjrg_Gallops.next_pace_seed to every struct literal. jjtf_favor: reworked coronet encode/decode/successor tests to the flat API (positional digit-position test, no magic numbers per RCG; dropped the parent_firemark test). jjtg_gallops: helpers carry the global seed, dropped the per-heat seed setup/asserts, slate test asserts the flat CAAAA mint and global-seed advance, validate test now checks the 5-char global seed. jjtm_mcp: guard_gallops fixture holds the reslate coronets so jjrm_resolve_batch_firemark(&b, &gallops) scans them. Remaining: jjtrs_restring (untouched — has re-key behavior asserts to rewrite for move-under-same-key), then vow-t to green (runtime test failures not yet swept).

### 2026-07-15 18:15 - ₢BcAAO - n

Compile fixes: reconstruct parade's firemark from the scanned heat_key (it is used downstream for files-for-pace and paddock emission); qualify jjrf_Firemark by full path in tally and wrap where it is not imported.

### 2026-07-15 18:13 - ₢BcAAO - n

Route every display/MCP-layer parent_firemark call site through the jjrg_heat_key_of_coronet paces-scan (JJS0 jjdt_coronet Resolution): get-spec, parade, mount, tally, wrap, the draft MCP wrapper (source firemark captured BEFORE the move, since post-move the pace lives in the destination), the apostille commit-affiliation firemark, the two groom-emblem helpers (lede-firemark now loads gallops fail-soft; resolve-emblem-marker scans the already-loaded gallops), and jjrm_resolve_batch_firemark (now takes gallops and scans each reslate coronet to its live heat, with the dispatch loading gallops once for the scan). jjrf_parent_firemark on Coronet is fully retired from source; library compiles. Test fixtures next.

### 2026-07-15 18:05 - ₢BcAAO - n

WIP: coronet re-gestalt core (schema + reprieve + mint + re-affiliate + lookup-by-scan). jjrf_favor: jjrf_Coronet is now a flat 5-char global index (flat encode/decode, successor; jjrf_parent_firemark removed), plus the interpunct qualifier and CAAAA seed-floor constants. jjrt_types: retired per-heat jjghn_next_pace_seed, added global jjgrn_next_pace_seed (serde default so old stores parse). jjri_io: registered the pace-seed heat→global reprieve episode (JJr_a7c) and the write-forward founding at max(highest+1, CAAAA). jjro_ops: mint from the global seed; draft/restring re-affiliate under the same immutable key (no re-key, no per-heat seed), keeping the bridle→rough revert; lookup via new jjrg_heat_key_of_coronet scan. jjrg_gallops: added the scan helper, resolve_pace scans. jjrv_validate: dropped the embed-parent-heat rule and per-heat seed check, added the global-seed root check and cross-heat coronet uniqueness. Display/MCP-layer parent_firemark sites still pending (compile-guided next).

### 2026-07-15 17:39 - Heat - n

Align the operation sheaves to the current tack schema — pre-existing drift found during the coronet sweep, unrelated to the re-gestalt. The tack record and members were referenced as jjdkr_tack/jjdkm_* across draft/restring/slate/mount/validate, but are defined as jjdcr_tack/jjdcm_*; those refs rendered literally. Rename all to the defined names and correct the format annotations the renames exposed against the current tack model: jjdcm_ts is an ISO 8601 timestamp (was refined/YYMMDD-HHMM), jjdcm_text is a line array (was a string), jjdcm_basis is the basis commit SHA (was jjdkm_commit). Also fix a latent validate bug: the tack-state rule now admits jjdpe_bridled, which the current jjdcm_state enumeration allows but validate would previously have rejected as broken. Left the deeper tack-model completeness (optional tier/effort members) untouched — separate concern.

### 2026-07-15 17:34 - ₢BcAAO - n

Infuse the pace-identity re-gestalt into the JJS binding codex (durable-first, before code). JJS0: rewrite the jjdt_coronet quoin as immutable-for-life flat global ids — global jjdgm_pace_seed mint under the commit lock, retired per-heat jjdhm_seed, heat-qualified interpunct (·) display, ingest tolerance, seed-floor grandfathering at max(highest+1, CAAAA); rewrite the flat encode/decode algorithm and Types row (~1.07B global); home the paces-scan Resolution rule and the CAAAA fresh-gallops default in the quoin. validate: drop the must-embed-parent-heat rule, add cross-heat coronet uniqueness and the global seed as a canonical root member. Behavior rewrites (draft/restring/slate/tally/nominate): re-affiliate by moving the entry under the same immutable key (never re-key), mint from the global seed, resolve source heat by scan; restring keeps old/new_coronet equal to preserve its wire schema. Drain JJSAS to a terse JJS0 pointer and close its wire-vs-display open fork. Consolidate ingest normalization to jjdz_encoding (chalk/parade/gazette cite it). Present-tense throughout — no skidmarks.

### 2026-07-15 08:50 - Heat - T

bridled ₢BcAAO at opus

### 2026-07-15 08:50 - Heat - d

batch: 1 reslate

### 2026-07-15 08:49 - ₢BcAAO - n

Transfer memo for the pace-identity re-gestalt (station -> cerebro): provisional code map (seven subsystems, entry points), the reprieve-episode plan (pace-seed heat->global, found at max(highest body+1, CAAAA)), the tt/vow-b delivery-sequencing hazard, and the operator's full-JJS*-infusion scope ruling. Provenance only; retires when BcAAO lands.

### 2026-07-15 07:54 - Heat - T

bridled ₢BcAAO at opus

### 2026-07-15 07:54 - Heat - d

batch: 1 reslate

### 2026-07-15 07:41 - Heat - T

bridled ₢BcAAD at sonnet

### 2026-07-15 07:41 - Heat - d

batch: 1 reslate

### 2026-07-15 07:41 - Heat - d

batch: 1 reslate

### 2026-07-15 06:57 - ₢BcAAK - W

Original-intent capture landed per §F on branch bhyslop-260715-BcAAK-pace-original-intent: four additive jjrg_Pace fields (jjgpn_dictation/jjgpn_precis/jjgpn_slated frozen at slate, jjgpn_redocket_count drift counter) riding default+skip serde — no reprieve episode, old gallops read natively and re-serialize byte-identical (test-proven). New Input slugs jjezs_dictation/jjezs_precis stage beside the slate notice with a shared lede, optional at the tool, rejected loud by reslate and batch; counter bumps in jjrg_revise_docket only; draft/restring carry the capture forward. Mount renders an Original-intent block above Docket: with staleness caveat, slated date, redocket count, and LLM-authored attribution. Guidance: core.md slating rule + Mount step 3 sentence, veiled §E additive carve-out. Distillation word re-minted gloss→precis after the grep gate found MCM owns gloss. vow-t green: 620 passed / 0 failed / 6 ignored (12 new tests). Live binary untouched; end-to-end slate demo deferred to the coda by design.

### 2026-07-15 06:53 - ₢BcAAK - n

Trimmed the original-intent guidance to operative instruction only: the slating block cut to 7 lines (stage jjezs_dictation verbatim + jjezs_precis distillation with the slate's lede, frozen at slate, enroll-only, tool-optional/slate-required), §E additive carve-out cut to 5 lines (default+skip field with byte-identical old stores registers no episode; §F retained), Mount step 3 addition reduced to one sentence, table row and command-reference annotations shortened.

### 2026-07-15 06:47 - ₢BcAAK - n

Guidance for the original-intent capture (kit source, on the §F branch): claude-jjk-core.md gains the Slating ceremony block — every ceremony slate stages jjezs_dictation (operator verbatim, quote never paraphrase) and jjezs_precis (LLM distillation) beside the jjezs_slate notice with the same lede, slate-only, never on reslate or batch slates (batch rejects loud); wire-format law, enroll table row, command reference, and Mount Protocol step 3 (read Original-intent above the docket, audit precis against dictation, docket is the living authority) updated to match. Veiled §E gains the additive carve-out: a default+skip_serializing_if field that leaves old stores natively readable AND byte-identical registers NO reprieve episode, citing the ₣Bc bridle-revival determination and JJSCRP's additive clause, §F branch delivery retained.

### 2026-07-15 06:43 - ₢BcAAK - n

Original-intent capture, first cut (on §F branch bhyslop-260715-BcAAK-pace-original-intent): four additive jjrg_Pace fields (jjgpn_dictation/jjgpn_precis/jjgpn_slated frozen at slate + jjgpn_redocket_count) with serde default/skip pattern — no reprieve episode; gloss re-minted to precis after grep-gate collision with MCM's gloss. Two new Input gazette slugs jjezs_dictation/jjezs_precis bound to the slate notice by shared lede, optional at the tool; batch vocabulary deliberately excludes them (loud not-in-vocabulary). Slate freezes the capture, jjrg_revise_docket alone bumps the counter, draft/restring carry all four forward. Mount renders an Original-intent block above Docket: with standing staleness caveat, redocket count, slated date, and LLM-authored attribution. Tests: intent freeze/bump/carry/serde-additive coverage + gazette companion parsing; all jjrg_Pace literals gain ..Default::default(). Untested as of this notch.

### 2026-07-15 06:24 - Heat - n

Strengthened RCG Alias Discipline to a zero-exception bright line: no `use ... as` anywhere — project-owned, foreign, or `as _`. Removed the foreign-boundary carve-out; foreign types now use their declared name, foreign-name collisions resolve by fully-qualified path at use sites, trait-in-scope imports use the bare name. Retired the repo's three remaining aliases: jjrm_mcp.rs McpError -> ErrorData (import + 9 sites), jjru_util.rs dead IoRead rename -> bare `use std::io::Read;`, apcnsa_main.rs JsonValue -> fully-qualified serde_json::Value (genuine collision with ort::value::Value). Census `rg '^\s*use .* as '` now returns zero repo-wide.

### 2026-07-15 06:23 - Heat - d

batch: 1 reslate

### 2026-07-15 06:20 - Heat - T

released ₢BcAAK to rough

### 2026-07-15 05:59 - ₢BcAAJ - W

De-aliased every jjr*/JJR* project-cipher-stripping `use ... as` import across the JJK crate (29 files, symmetric 268/268 pure rename), rewriting body use-sites back to canonical prefixed names per RCG Alias Discipline; the two RCG-permitted foreign-boundary aliases (rmcp::ErrorData as McpError, std::io::Read as IoRead) retained. Frontier-verified independently: census grep returns only the foreign pair, no logic/string-literal changes in the diff, vow-b clean, vow-t 635 passed / 0 failed (JJK crate 608/0/6).

### 2026-07-15 05:51 - ₢BcAAJ - L

claude-sonnet-5 landed

### 2026-07-15 05:51 - ₢BcAAJ - n

De-alias every jjr*/JJR*-stripping `use ... as` in the JJK crate per RCG Alias Discipline; foreign-boundary aliases (rmcp::ErrorData as McpError, std::io::Read as IoRead) retained. Pure rename — build clean, 608 tests pass.

### 2026-07-14 20:33 - Heat - T

bridled ₢BcAAE at sonnet

### 2026-07-14 20:33 - Heat - T

bridled ₢BcAAD at sonnet

### 2026-07-14 20:33 - Heat - T

bridled ₢BcAAK at opus

### 2026-07-14 20:33 - Heat - T

bridled ₢BcAAJ at sonnet

### 2026-07-12 23:00 - ₢BcAAN - W

Verified already-landed, no work remained: the jjx_apostille rename shipped 260707 — the sole surviving jjx_bridle is JJS0's permitted historical note; the jjdo_apostille lower-tool quoin and the jjsuv_bridle/jjsuv_unbridle Upper API rows stand in JJS0; claude-jjk-core.md speaks jjx_apostille throughout (verb table, command reference, gate text, Bridle Protocol); vow-b and vow-t green (543 jjk-crate tests, 0 failed). Closure records the verification.

### 2026-07-12 22:50 - Heat - d

paddock curried: groom 260712: glyph-strip abandonment recorded, intent-capture rider noted, episode retirement re-phrased to registry census

### 2026-07-12 22:49 - Heat - r

moved ₢BcAAE after ₢BcAAD

### 2026-07-12 22:49 - Heat - r

moved ₢BcAAD after ₢BcAAO

### 2026-07-12 22:48 - Heat - d

batch: 2 reslate

### 2026-07-11 14:33 - Heat - T

gallops-key-glyph-strip

### 2026-07-08 10:52 - Heat - d

paddock curried: adopt pace-identity re-gestalt from ₣Br (₢BcAAO)

### 2026-07-08 10:51 - Heat - T

bridled ₢BcAAO at opus

### 2026-07-08 10:51 - Heat - S

pace-identity-regestalt-impl

### 2026-07-07 12:05 - ₢BcAAN - n

Rename the designation command jjx_bridle to jjx_apostille, repairing the Upper/Lower vocabulary-isolation breach (the vivid equestrian upper word had shipped as the lower tool name). Rust: command constant, registry, params struct (jjrm_ApostilleParams), dispatch arm, schemars command list, guard-bucket comments, and the three designation-gate remedy texts; interior jjrg_bridle/jjrg_release methods kept as hearting tied to the sanctioned bridled state noun. JJS0: registered the missing Upper API rows jjsuv_bridle and jjsuv_unbridle (mapping attrs + verb definitions mapping to the lower tool), and minted the jjdo_apostille operation quoin inline under Write Operations with a provenance line naming the rename. claude-jjk-core.md: verb table, command reference, three-bucket gate text, and Bridle Protocol now cite jjx_apostille; all upper-register equestrian prose (bridle, unbridle, bridled state) unchanged. No serialized field changes — no reprieve episode.

### 2026-07-07 11:58 - Heat - S

apostille-command-rename

### 2026-07-07 11:25 - ₢BcAAM - W

Pace bridling landed and live-verified: jjgte_bridled re-minted as the Bridled state after stripping the dormant bridle-retirement episode (probe, registry entry, alias, legacy test); typed jjrg_Tier/jjrg_Effort with jjgde_ wire tokens and serialize-when-present jjgtn_tier/jjgtn_effort tack fields keep untouched stores byte-canonical, so no reprieve episode; jjx_bridle designates (rough only) and releases; three-bucket per-command guard (open reads, designation-guarded orient/record/landing with strict tier equality and post-resolution no-skip refusal, frontier-only rest, frontier = fable + opus); revert triggers on redocket/transfer/relocate, provenance through close; coronets tags [bridled tier]; designation-coherence validation; Bridle Protocol and verb rows in claude-jjk-core.md; jjdpe_bridled, jjdcm_tier, jjdcm_effort recorded in JJS0. 442 jjk + 27 vvc tests green; full guard matrix exercised live against the real store via a bridle/release trial on BcAAL.

### 2026-07-07 11:20 - Heat - T

released ₢BcAAL to rough

### 2026-07-07 11:18 - Heat - T

bridled ₢BcAAL at sonnet high

### 2026-07-07 11:13 - ₢BcAAM - n

Bridle protocol text and schema record: claude-jjk-core.md gains the bridle/unbridle verb rows, the jjx_bridle command reference, the three-bucket gate description on the model param, the bridled coronets tag, and a Bridle Protocol section (frontier bridling judgment, designee session, escalation paths); JJS0 gains jjdpe_bridled with predicate rows, jjdcm_tier and jjdcm_effort tack members, the state member values list, and the primed demotion note re-homed to the V3-to-V4 episode; jjri_io registry comment corrected on the jjgtn_direction tolerance retirement

### 2026-07-07 11:09 - ₢BcAAM - n

Test-compile repairs: tier/effort fields in jjtq_query tack literals; bridled round-trip test inspects via deserialization since the Canonical appraisal variant carries the census string, not the struct

### 2026-07-07 11:07 - ₢BcAAM - n

Bridle test battery: wire tokens for state and both designation vocabularies, recognized-word parsing, untouched-store byte-canonical guarantee, bridled-store canonical round-trip, effort-absent tier-only bytes, bridle/release transitions and their preconditions, revert triggers (redocket, draft) vs preserving paths (relabel, close, drop), designation-coherence validation rejections, v3 primed demotion through the V3-to-V4 episode, caller-tier extraction across vendor families, frontier set, guard-bucket partition, and strict-equality designation judgment in both directions

### 2026-07-07 11:03 - ₢BcAAM - n

Bridle core lands on the §F branch: strip the dormant bridle-retirement reprieve episode (probe, registry entry, serde alias, legacy-load test) and re-mint jjgte_bridled as the live Bridled pace state; typed jjrg_Tier/jjrg_Effort enums with jjgde_ wire tokens and optional jjgtn_tier/jjgtn_effort tack fields (serialize-when-present, additive no-episode); bridle/release composed methods; jjx_bridle MCP command; three-bucket per-command guard (open/designation/frontier, frontier=fable+opus) with post-resolution orient guard and sub-frontier record/landing designation checks; revert triggers on redocket and draft; bridled joins next-actionable, remaining, and tagged coronets listing; designation-coherence validation rules

### 2026-07-07 10:18 - ₢BcAAM - n

Seed provenance and census into the MCM hallowed-word census by term-of-art reservation (oracle precedent, hoisted shared clause), surfaced during the bridle pace's reslate review

### 2026-07-07 10:10 - Heat - d

batch: 1 reslate

### 2026-07-07 09:19 - Heat - d

batch: 1 reslate

### 2026-07-06 10:28 - Heat - d

batch: paddock, 1 reslate

### 2026-07-06 09:49 - Heat - d

batch: 1 reslate

### 2026-07-06 09:24 - Heat - d

batch: 1 reslate

### 2026-07-06 07:51 - Heat - S

bridle-verb-and-mount-guard

### 2026-07-05 09:20 - Heat - S

gallops-key-glyph-strip

### 2026-06-25 09:51 - Heat - d

batch: 1 reslate

### 2026-06-23 22:56 - Heat - S

pace-original-intent

### 2026-06-21 13:54 - ₢BcAAH - W

Added the optional chat-retention gallops field jjgrn_retention_since — a raw ISO date with default + skip_serializing_if, so an off store is byte-identical and needs no reprieve episode (the easy mirror of a field removal). Added the jjri_retention_state classifier (Off/On/Malformed, validated at read) and the open-time retention monitum zjjrm_retention_monitum, an independent sibling to the reprieve nag. Homed the jjdz_monitum umbrella concept in JJS0 (reprieve named its first instance) plus the jjdgm_retention member. 6 new tests. Capture itself and the operator-facing setter stay with BDAAY. Delivered source-only on a §F branch per §E, then merged into the converged main; vow-b clean, vow-t 390, gallops validated clean.

### 2026-06-21 11:28 - Heat - n

Add a slate-time schema-impact check to conduct §E: when slating a JJK-crate pace, check for a gallops on-disk schema change and, if so, the docket must point the mount agent at §E/§F/JSCRP for branch-delivery and the reprieve-episode determination. Puts the recognition in the docket (the execution artifact) rather than only the paddock, the gap that let this heat's schema pace get worked on main before branching.

### 2026-06-21 10:12 - Heat - n

Add RCG Alias Discipline: a use-import must never alias a project-owned cipher-prefixed identifier to a prefix-stripped name (the use-site twin of Minting Discipline's grep-census — an as-rename keeps the declaration greppable but makes every body use site invisible to grep {cipher}). New ## Alias Discipline section after Import Discipline (Rule + the-strip-bit-us rationale + foreign-crate boundary carve-out for ErrorData/Read aliases + Smell Test), plus a Module Maturity Checklist subsection. Surfaced during the retention-field work when an aliased Gallops construction site escaped a `jjrg_Gallops {` grep and nearly green-lit a non-compiling change; founds the crate-wide de-alias pace BcAAJ.

### 2026-06-21 10:12 - Heat - n

Add RCG Alias Discipline: a use-import must never alias a project-owned cipher-prefixed identifier to a prefix-stripped name (the use-site twin of Minting Discipline's grep-census — an as-rename keeps the declaration greppable but makes every body use site invisible to grep {cipher}). New ## Alias Discipline section after Import Discipline (Rule + the-strip-bit-us rationale + foreign-crate boundary carve-out for ErrorData/Read aliases + Smell Test), plus a Module Maturity Checklist subsection. Surfaced during the retention-field work when an aliased Gallops construction site escaped a `jjrg_Gallops {` grep and nearly green-lit a non-compiling change; founds the crate-wide de-alias pace BcAAJ.

### 2026-06-21 10:09 - Heat - S

jjk-dealias-cipher-imports

### 2026-06-21 09:17 - ₢BcAAI - W

Renamed the schema-tolerance mechanism forgiveness -> reprieve across spec and code, and homed its full doctrine in the new JJS0 subdoc JJSCRP-reprieve.adoc: the mechanism (registry/probe/nag), the multi-install convergence model, the schema-change delivery posture (source-only branch, never commit a conversion), the two-pass cross-clone convergence protocol, and the register-an-episode procedure now live in one durable home, include::'d at jjdz_reprieve in == Serialization; the open-ceremony contract stays in Serialization as its own subsection. The reprieve metaphor is seated in the definition (a temporary stay for the old shape, retired on convergence). Conduct claude-jjk-bhyslop.md §E gains the branch-delivery agent rule and cites the subdoc; §A registers JJSCRP. Verified not a schema change: the only renamed string value is the jjx_open nag's display label (stdout, not serialized), no serde field/variant touched, the on-disk gallops is byte-identical, and the opaque rivet JJr_a7c is unchanged -- so it landed on main with no episode and no branch quarantine, the doctrine's own counter-example. Implementing work notched at b5203ba3; build tt/vow-b clean, tt/vow-t 391 passed.

### 2026-06-21 09:02 - ₢BcAAI - n

Rename the schema-tolerance mechanism forgiveness -> reprieve across spec and code, and home its full doctrine in a new JJS0 subdocument JJSCRP-reprieve.adoc. The mechanism (registry/probe/nag), the multi-install convergence model, the schema-change delivery posture (source-only on a date+identity branch, never commit a gallops conversion), the two-pass cross-clone convergence protocol, and the register-an-episode procedure now live in one durable home, include::'d at jjdz_reprieve in == Serialization; the open-ceremony contract stays in Serialization as its own subsection. The reprieve metaphor is seated in the definition (a temporary stay for the old shape, granted until every clone converges and then retired). Conduct claude-jjk-bhyslop.md gains the branch-delivery agent rule in section E and cites the subdoc; section A registers JJSCRP. Pure identifier rename plus doc move (consts JJDZ_LABEL_REPRIEVE/JJDZ_RIVET_REPRIEVE, zjjrm_reprieve_nag, all comments/labels), zero on-disk gallops impact -- not a schema change, no reprieve episode; the opaque rivet JJr_a7c is unchanged. Build tt/vow-b clean, tt/vow-t 391 passed.

### 2026-06-21 08:35 - Heat - S

home-reprieve-doctrine

### 2026-06-20 15:46 - ₢BcAAB - W

validate is now a normalize-and-report pass over the gallops, replacing the read-only check that fataled on any valid-but-non-canonical store. Tri-state verdict in the exit code (JJSCVL exit-enumerated): 0 clean (already canonical, no write), 2 normalized (rewrote to canonical and committed), 1 broken (parse/invariant failure, file untouched, never a silent fix). Reuses the shared forgiveness probe; extracts jjdr_load's migration write-forward into shared jjdz_write_forward (single canonicalizer source); adds gallops-wide jjri_consign (sibling to identity-keyed jjri_persist) that saves+commits the gallops alone under budget, reverts on failure, and finalizes any in-progress merge (plain git commit picks up MERGE_HEAD). Pure zjjrvl_appraise seam split from the effectful commit so the canonicalizer is unit-tested on byte fixtures, never a live store. New 'v'/Validate chalk marker; MCP maps 0/2 to success, 1 to error. Spec JJSCVL migrated to the tri-state contract; command-reference updated with the normalized-is-structural-not-semantic residual. 8 unit tests green within 370 lib tests; release build clean under deny(warnings). Implementing work notched at 6d58fe0df; source-only per the heat branch-delivery posture, not run against the live gallops.

### 2026-06-20 14:47 - ₢BcAAB - n

validate becomes a normalize-and-report pass over the gallops, replacing the read-only check that fataled on any valid-but-non-canonical store. Tri-state verdict carried in the exit code (JJSCVL exit-enumerated): 0 clean (already canonical, no write), 2 normalized (valid-but-non-canonical -> rewrote to canonical form and committed), 1 broken (parse/invariant failure, file untouched, never a silent fix). Reuses the shared forgiveness probe jjdz_probe; extracts jjdr_load's inline migration write-forward into the shared jjdz_write_forward canonicalizer (loader keeps no second copy); adds the gallops-wide commit primitive jjri_consign (sibling to identity-keyed jjri_persist) which saves+commits the gallops alone under a budget, reverts to HEAD on a blocked commit, and finalizes any in-progress merge (two parents) since plain git commit picks up MERGE_HEAD. Splits a pure zjjrvl_appraise seam from the effectful commit so the canonicalizer is unit-tested on byte fixtures, never a live store. Adds the 'v'/Validate chalk marker (gallops-wide, no identity); MCP maps 0/2->success, 1->error. Spec JJSCVL migrated to the tri-state contract and command-reference wording updated with the normalized-is-structural-not-semantic residual. 8 new unit tests (clean/normalize/broken/idempotent/legacy-collapse + exit-0/exit-1 wiring), 370 lib tests green, release build clean under deny(warnings). Source-only per the heat branch-delivery posture; not run against the live gallops.

### 2026-06-20 13:53 - ₢BcAAG - W

Retired the tack direction field and the bridled pace state. The direction field is gone from the live tack type; loading a pre-retirement gallops now demotes bridled->rough and drops the jjgtn_direction key via a structural forgiveness tolerance (DIRECTION_KEY probe in jjri_io.rs), covered by the jjtg_load_legacy_bridle_demotes_and_drops_direction test. The bridled state, warrant display, and dangling jjx_arm recommendation all retired rather than persisting as vestige. The frozen V3 reference (jjrt_v3_types.rs) deliberately retains its direction field for the coda BcAAD to retire. Implementing work landed in plain commit 5378fe09f; this wrap reconciles the JJK ledger, which had no record of that non-jjx-affiliated commit.

### 2026-06-20 09:07 - Heat - d

paddock curried: add branch-delivery posture to Cinched

### 2026-06-20 08:38 - Heat - S

chat-retention-config

### 2026-06-19 12:01 - ₢BcAAA - W

Converted tack docket text from String to a line array (one element per physical line) with a both-shape-tolerant custom deserializer, and collapsed each pace to a single current tack edited in place (redocket/tally replace, not prepend; tack history lives in git). Registered the structural 'tack text->lines' forgiveness episode (rivet JJr_a7c). Reconciled every reader via the lossless jjrg_text_to_lines/jjrg_lines_to_text pair; validate's empty-check is now all-lines-empty; JSO0 schema description updated to the single-tack/line-array model. 369 kit tests green, deny(warnings) clean. The one-time live conversion was performed on authoritative main via the jjx_open size_limit convergence hook seated in this heat, leaving the store canonical and idempotent thereafter.

### 2026-06-19 11:58 - Heat - S

remove-tack-direction-field

### 2026-06-19 11:23 - Heat - n

JJS0: rewrite the jjdz_forgiveness open-probe contract into an open-ceremony contract — the size_limit convergence budget (0 = read-only invitatory; >0 = budget-gated convergence commit that self-persists pending conversions as the invitatory), the over-budget hard-fail (required size reported, store reverted to HEAD, officium rolled back, no internal retries), and the always-gate (open refuses unless every managed file is pristine). So open 'always succeeds' becomes 'succeeds iff the managed store is clean'; the nag alone still never causes failure. Code: surface the guard's detailed breakdown (the required byte count) in the over-budget error, so the required size is shown before the revert.

### 2026-06-19 11:12 - Heat - n

Fix jjx_open size_limit extraction. The open dispatch reads p.params before the main dispatcher's stringified-params normalization, so when params arrive stringified (the documented MCP quirk) size_limit was silently read as 0 and the convergence path never triggered (open ran the default read-only path instead). Normalize the stringified-params case — parse the JSON string to a Value — before reading size_limit, mirroring the main dispatcher.

### 2026-06-19 11:06 - Heat - n

Add a size_limit-gated convergence hook to jjx_open. size_limit (default 0) is the convergence budget: 0 keeps the lockless read-only open (empty invitatory marker — open mutates nothing); >0 opts the ceremony into a bulk-authorized commit that loads (running any pending forgiveness conversion), saves, and commits the gallops under the budget as the invitatory. An always-gate refuses open if any jj-managed file (gallops now; chat store later) is staged or conflicted, fail-safe on git-command error so a plumbing failure never bricks open. Over budget hard-fails: prints the required size, reverts the store to HEAD, rolls back the freshly-claimed officium (no officium delivered). Reuses vvc machine_commit + guard + lock; no internal retries (lock-wedge breaks via officium-free vvx vvx_unlock). Anticipates autonomous chat-history capture riding the same path.

### 2026-06-19 09:49 - ₢BcAAA - n

JJS0: bring the tack schema description to the single-current-tack, line-array model. jjdpm_tacks now holds one current tack edited in place (evolution is git history, not an in-JSON array); jjdcm_text is a line array (axd_repeated, was axt_string), one element per physical line; jjdcr_tack replaces its single tack rather than prepending. Rename the shared write primitive quoin jjsgmpt_prepend_tack -> jjsgmst_set_tack (replace semantics) across the mapping section, anchor, definition, and the revise-docket composition step. Frozen V3 reference section left untouched.

### 2026-06-19 09:46 - ₢BcAAA - n

Convert tack docket text from String to a Vec<String> line array (one element per physical line) with a custom deserializer tolerating the legacy string shape; collapse a pace's tack history to a single current tack edited in place — redocket and tally now replace rather than prepend (jjrg_prepend_tack renamed jjrg_set_tack), evolution lives in git. Register the structural 'tack text->lines' forgiveness episode (rivet JJr_a7c): the load probe sniffs the legacy string-valued jjgtn_text in raw bytes, stands down the round-trip gate, and the write-forward splits text to lines and truncates multi-tack history to the newest tack, idempotent on reload. Flatten at every read boundary (PaceContext, get_spec, parade, scout, saddle, trophy, draft note, restring) via the lossless jjrg_text_to_lines/jjrg_lines_to_text pair; validate's empty-check becomes all-lines-empty. Update test tack constructions/assertions to the Vec shape and single-tack model; add deserializer both-shape, lossless round-trip, and file-load convert+collapse+idempotency tests. 369 kit tests green, tt/vow-b clean under deny(warnings).

### 2026-06-19 11:23 - Heat - n

JJS0: rewrite the jjdz_forgiveness open-probe contract into an open-ceremony contract — the size_limit convergence budget (0 = read-only invitatory; >0 = budget-gated convergence commit that self-persists pending conversions as the invitatory), the over-budget hard-fail (required size reported, store reverted to HEAD, officium rolled back, no internal retries), and the always-gate (open refuses unless every managed file is pristine). So open 'always succeeds' becomes 'succeeds iff the managed store is clean'; the nag alone still never causes failure. Code: surface the guard's detailed breakdown (the required byte count) in the over-budget error, so the required size is shown before the revert.

### 2026-06-19 11:12 - Heat - n

Fix jjx_open size_limit extraction. The open dispatch reads p.params before the main dispatcher's stringified-params normalization, so when params arrive stringified (the documented MCP quirk) size_limit was silently read as 0 and the convergence path never triggered (open ran the default read-only path instead). Normalize the stringified-params case — parse the JSON string to a Value — before reading size_limit, mirroring the main dispatcher.

### 2026-06-19 11:06 - Heat - n

Add a size_limit-gated convergence hook to jjx_open. size_limit (default 0) is the convergence budget: 0 keeps the lockless read-only open (empty invitatory marker — open mutates nothing); >0 opts the ceremony into a bulk-authorized commit that loads (running any pending forgiveness conversion), saves, and commits the gallops under the budget as the invitatory. An always-gate refuses open if any jj-managed file (gallops now; chat store later) is staged or conflicted, fail-safe on git-command error so a plumbing failure never bricks open. Over budget hard-fails: prints the required size, reverts the store to HEAD, rolls back the freshly-claimed officium (no officium delivered). Reuses vvc machine_commit + guard + lock; no internal retries (lock-wedge breaks via officium-free vvx vvx_unlock). Anticipates autonomous chat-history capture riding the same path.

### 2026-06-19 09:49 - ₢BcAAA - n

JJS0: bring the tack schema description to the single-current-tack, line-array model. jjdpm_tacks now holds one current tack edited in place (evolution is git history, not an in-JSON array); jjdcm_text is a line array (axd_repeated, was axt_string), one element per physical line; jjdcr_tack replaces its single tack rather than prepending. Rename the shared write primitive quoin jjsgmpt_prepend_tack -> jjsgmst_set_tack (replace semantics) across the mapping section, anchor, definition, and the revise-docket composition step. Frozen V3 reference section left untouched.

### 2026-06-19 09:46 - ₢BcAAA - n

Convert tack docket text from String to a Vec<String> line array (one element per physical line) with a custom deserializer tolerating the legacy string shape; collapse a pace's tack history to a single current tack edited in place — redocket and tally now replace rather than prepend (jjrg_prepend_tack renamed jjrg_set_tack), evolution lives in git. Register the structural 'tack text->lines' forgiveness episode (rivet JJr_a7c): the load probe sniffs the legacy string-valued jjgtn_text in raw bytes, stands down the round-trip gate, and the write-forward splits text to lines and truncates multi-tack history to the newest tack, idempotent on reload. Flatten at every read boundary (PaceContext, get_spec, parade, scout, saddle, trophy, draft note, restring) via the lossless jjrg_text_to_lines/jjrg_lines_to_text pair; validate's empty-check becomes all-lines-empty. Update test tack constructions/assertions to the Vec shape and single-tack model; add deserializer both-shape, lossless round-trip, and file-load convert+collapse+idempotency tests. 369 kit tests green, tt/vow-b clean under deny(warnings).

### 2026-06-19 09:01 - Heat - S

remove-tack-direction-field

### 2026-06-19 07:49 - ₢BcAAF - W

Removed the gallops schema_version field; schema-change detection is now per-episode structural. The field is gone from the type, the validate gate (Rule 0), and all 12 construction sites. The V3-to-V4 episode now rides on heat_order-absent and stale-pensum-seed alone. A new schema_version-drop episode (live-test: the jjgrn_schema_version key in the raw bytes; no custom write-forward, serde omits the absent field on save) migrates existing canonical stores forward on their first structural write without tripping the round-trip gate. The primed alias is cited under rivet JJr_a7c and rides the V3-to-V4 episode rather than standing as an untracked tolerance. JJS0 cinches the self-detectable discipline (no global version oracle; idempotent-normalize-at-use; empty-write-forward case) and drops the jjdgm_version quoin, its mapping entry, and the version-set migration step. Build and tests green (366+27, 0 fail); detection and tolerated-load verified live; this wrap is the converging write that drops the key from the store.

### 2026-06-19 07:43 - ₢BcAAF - n

Remove the gallops schema_version field and reframe schema-change detection as per-episode structural sniffing. Drop the field from the type, the validate gate (Rule 0), and all 12 construction sites. Strip the version clause from the V3-to-V4 episode so heat_order-absent and stale-pensum-seed carry it. Add a schema_version-drop forgiveness episode whose live-test is the jjgrn_schema_version key in the raw bytes, with no custom write-forward: serde omits the now-absent field on save, so an existing canonical store converges on its first write without tripping the round-trip gate. Cite rivet JJr_a7c at the primed alias so it rides the V3-to-V4 episode instead of standing as an untracked tolerance. In JJS0, rewrite the Registering-a-new-episode procedure to the self-detectable doctrine (no global version oracle; idempotent-normalize-at-use; empty-write-forward case), remove the jjdgm_version quoin and its mapping entry, and drop the version-set step from the migration path.

### 2026-06-19 06:46 - Heat - r

moved BcAAF before BcAAA

### 2026-06-19 06:43 - Heat - S

drop-gallops-schema-version

### 2026-06-19 06:32 - Heat - n

Complete the forgiveness mechanism's deserialize-boundary documentation. JJS0 'Registering a new episode' procedure gains the missing step: new types must tolerate the old on-disk shape at the deserialize boundary or the parse rejects the old file before the probe and write-forward can run (additive/removed fields ride serde defaults and ignored-unknowns; a retyped field needs an explicit tolerance — a custom deserialize accepting either shape, or a frozen reference type converted in a second step). The demolition-condition line generalizes 'frozen reference' to 'old-shape tolerance (a custom deserialize, a frozen reference type, or both)'. New claude-jjk-bhyslop.md section E gates gallops schema changes on registering a forgiveness episode and routes to jjdz_forgiveness in JJS0 — a pointer, not a restatement (rivet doctrine applied to itself; the public claude-jjk-core stays clean since consumers do not build the crate).

### 2026-06-18 14:26 - ₢BcAAC - W

Seated the permanent forgiveness mechanism — the spine the heat's two schema changes ride. A single read-only per-episode probe is the sole old-format detector (jjdr_load keeps no second copy; the jjx_open nag is its only other caller); detection generalized to schema-below-current, driven by a one-entry registry whose first episode is the V3→V4 tolerance. jjx_open emits a best-effort, non-gating per-episode status line. JJS0 gained the jjdz_forgiveness quoin as the durable operating manual (permanent registry/probe/nag, per-episode lifecycle, demolition condition gated on jjdk_sole_operator, register-an-episode procedure, open-probe contract); six V3-legacy markers re-cite it off the firemark; the open-touches-no-persistent-state property softened to a read-only peek. Two correctives surfaced and landed: the model-ID probe now extracts the canonical claude-… signature (killing a flaky test and a latent jjx_open commit-pollution bug), and the rivet — first malformed as the legible JJr_forgiveness — was corrected to the opaque JJr_a7c per MCM mcm_rivet, the const renamed JJDZ_RIVET_FORGIVENESS, the nag split into legible-label + opaque-token, and the CMK MCM vocabulary table given a Rivet row to prevent recurrence. Build tt/vow-b and full tt/vow-t green throughout.

### 2026-06-18 14:25 - Heat - S

retire-v3-legacy-arg-quoins

### 2026-06-18 14:23 - ₢BcAAC - n

Correct the malformed forgiveness rivet: a readable token leaks meaning into shipped code, which is the quoin property a rivet is defined against (MCM mcm_rivet). Revert the token to the opaque tail JJr_a7c, rename the const JJDZ_RIVET -> JJDZ_RIVET_FORGIVENESS (RCG Identity Rule; terminal exclusivity bars JJDZ_FORGIVENESS_RIVET against the jjdz_forgiveness quoin) plus parallel JJDZ_LABEL_FORGIVENESS, and split the open-time nag into a legible label beside the opaque token (the jjx_open echo of the jailer rivet riding its phase announcement, JDG JDo_101). Re-cite all code/spec sites to the opaque token. Document the rivet as a peer census category in the CMK MCM Vocabulary table — format {proj}r_<opaque-tail>, opacity contrast with the quoin, grep-based uniqueness, axvc_ kind voicing, and the formal home (MCM mcm_rivet / ACG cited-constraint anchor) — closing the doc gap that let the misnaming through.

### 2026-06-18 13:58 - ₢BcAAC - n

Harden the model-ID probe against chatty/refusing model replies: extract the canonical claude-… token (optional [1m]-style context suffix preserved, trailing sentence period dropped) instead of trusting raw stdout, falling back to unavailable. Fixes a latent bug where a prose reply polluted the jjx_open invitatory commit body and broke the fixed 5-line probe-output contract (the intermittent test_probe_output_format failure). Adds a unit test covering bare/bracketed/prose-wrapped/dotted/dated/absent inputs. Pattern confirmed across the full Claude model-ID history; source URLs cited in the extractor doc comment.

### 2026-06-18 13:39 - ₢BcAAC - n

Seat the forgiveness mechanism ahead of the schema changes that ride it: named rivet JJr_forgiveness (single-def const), one pure read-only per-episode probe as the sole old-format detector (jjdr_load keeps no second copy), a one-entry registry with the V3-to-V4 tolerance as the first episode, detection generalized to schema-below-current, and a best-effort non-gating jjx_open dormancy nag carrying the rivet token. JJS0 seats the jjdz_forgiveness quoin as the permanent operating manual (registry/probe/nag, per-episode lifecycle, demolition condition gated on jjdk_sole_operator, register-an-episode procedure, open-probe contract); six V3-legacy markers re-cited off the firemark onto the quoin, and the open-touches-no-persistent-state property softened to a read-only peek. Rivet named rather than opaque per operator steer (RCG String Boundary Discipline).

### 2026-06-18 14:26 - ₢BcAAC - W

Seated the permanent forgiveness mechanism — the spine the heat's two schema changes ride. A single read-only per-episode probe is the sole old-format detector (jjdr_load keeps no second copy; the jjx_open nag is its only other caller); detection generalized to schema-below-current, driven by a one-entry registry whose first episode is the V3→V4 tolerance. jjx_open emits a best-effort, non-gating per-episode status line. JJS0 gained the jjdz_forgiveness quoin as the durable operating manual (permanent registry/probe/nag, per-episode lifecycle, demolition condition gated on jjdk_sole_operator, register-an-episode procedure, open-probe contract); six V3-legacy markers re-cite it off the firemark; the open-touches-no-persistent-state property softened to a read-only peek. Two correctives surfaced and landed: the model-ID probe now extracts the canonical claude-… signature (killing a flaky test and a latent jjx_open commit-pollution bug), and the rivet — first malformed as the legible JJr_forgiveness — was corrected to the opaque JJr_a7c per MCM mcm_rivet, the const renamed JJDZ_RIVET_FORGIVENESS, the nag split into legible-label + opaque-token, and the CMK MCM vocabulary table given a Rivet row to prevent recurrence. Build tt/vow-b and full tt/vow-t green throughout.

### 2026-06-18 14:25 - Heat - S

retire-v3-legacy-arg-quoins

### 2026-06-18 14:23 - ₢BcAAC - n

Correct the malformed forgiveness rivet: a readable token leaks meaning into shipped code, which is the quoin property a rivet is defined against (MCM mcm_rivet). Revert the token to the opaque tail JJr_a7c, rename the const JJDZ_RIVET -> JJDZ_RIVET_FORGIVENESS (RCG Identity Rule; terminal exclusivity bars JJDZ_FORGIVENESS_RIVET against the jjdz_forgiveness quoin) plus parallel JJDZ_LABEL_FORGIVENESS, and split the open-time nag into a legible label beside the opaque token (the jjx_open echo of the jailer rivet riding its phase announcement, JDG JDo_101). Re-cite all code/spec sites to the opaque token. Document the rivet as a peer census category in the CMK MCM Vocabulary table — format {proj}r_<opaque-tail>, opacity contrast with the quoin, grep-based uniqueness, axvc_ kind voicing, and the formal home (MCM mcm_rivet / ACG cited-constraint anchor) — closing the doc gap that let the misnaming through.

### 2026-06-18 13:58 - ₢BcAAC - n

Harden the model-ID probe against chatty/refusing model replies: extract the canonical claude-… token (optional [1m]-style context suffix preserved, trailing sentence period dropped) instead of trusting raw stdout, falling back to unavailable. Fixes a latent bug where a prose reply polluted the jjx_open invitatory commit body and broke the fixed 5-line probe-output contract (the intermittent test_probe_output_format failure). Adds a unit test covering bare/bracketed/prose-wrapped/dotted/dated/absent inputs. Pattern confirmed across the full Claude model-ID history; source URLs cited in the extractor doc comment.

### 2026-06-18 13:39 - ₢BcAAC - n

Seat the forgiveness mechanism ahead of the schema changes that ride it: named rivet JJr_forgiveness (single-def const), one pure read-only per-episode probe as the sole old-format detector (jjdr_load keeps no second copy), a one-entry registry with the V3-to-V4 tolerance as the first episode, detection generalized to schema-below-current, and a best-effort non-gating jjx_open dormancy nag carrying the rivet token. JJS0 seats the jjdz_forgiveness quoin as the permanent operating manual (registry/probe/nag, per-episode lifecycle, demolition condition gated on jjdk_sole_operator, register-an-episode procedure, open-probe contract); six V3-legacy markers re-cited off the firemark onto the quoin, and the open-touches-no-persistent-state property softened to a read-only peek. Rivet named rather than opaque per operator steer (RCG String Boundary Discipline).

### 2026-06-18 12:33 - Heat - n

Seat durable schema-convergence design intent in JJS0 as insurance ahead of the forgiveness operating manual: schema changes recur and converge to a floor, not accumulate

### 2026-06-18 12:22 - Heat - T

propagate-and-retire-episodes

### 2026-06-18 12:20 - Heat - d

paddock curried: decouple from Bb: inline the principle, drop the cross-heat citation

### 2026-06-18 12:07 - Heat - d

paddock curried: reframe: mechanism is permanent infrastructure, only episodes retire; prune stale silks

### 2026-06-17 19:41 - Heat - f

racing, silks=jjk-v4-0-isolated-schema-changes

### 2026-06-17 19:29 - Heat - d

paddock curried: rewrite to capture forgiveness spine + coda shape

### 2026-06-17 16:40 - Heat - S

propagate-and-retire-forgiveness

### 2026-06-17 16:39 - Heat - S

forgiveness-rivet-and-probe

### 2026-06-09 08:38 - Heat - n

Land the restring of the two schema-entangled paces (tack-data-model-rework, validate-normalize-and-report) into this heat; the transfer applied the move to the gallops on disk but the size guard blocked its own commit

### 2026-06-09 08:34 - Heat - N

jjk-v4-1-isolated-schema-changes

