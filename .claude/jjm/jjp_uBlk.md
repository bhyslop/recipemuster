## Shape

This heat owns a single initiative:
Job Jockey's own record — the gallops, the captured chat histories, the action-choices it commits — lives in a git repository **separate from the code being jockeyed and global across stations**, not co-resident in the work repo as it is today.
Every state edit is a bracketed transaction — pull-rebase, acquire the lock, mutate, push (the push *is* the lock's compare-and-swap), confirm, commit, unwind — so concurrent masters working from different places never clobber the record.

Stabled now to make the initiative discoverable and revisable; today's act is capture, not construction.
The build waits until the operator chooses to resolve the design forks below.

This is its own strongly-justified initiative, pursued discretely from ₣Ba's fleet kit; the two were formerly fused and are now deliberately split (operator decision, 260625).

## Why — the married-model contamination

Co-residence makes JJ bookkeeping commits part of the work repo's history and advances its HEAD.
The recorded symptom is the pensum-seed eviction: JJ commits advanced HEAD past origin and tripped foray's curia-readiness guard.
A separate state repo dissolves that contamination class entirely — not one instance of it — which is the core case for decoupling.
The merge growing-pains across synced clones are the second, independently-sufficient case: today every gallops change is a 3-way merge of canonical JSON between clones, and a single global record edited under a lock has no cross-clone merge at all.

## Relation to ₣Ba (the fleet kit) — fork resolved: sibling

₣Ba formerly fused two concerns: fleet provisioning, and a shared state home (its "mews half") that reached in and claimed JJ-state ownership.
That fusion is dissolved (260625).
₣Ba's mews is now **fleet state only** — which nodes exist, of what type, who holds each; this heat owns **all** of JJ's record-in-a-repo.
The earlier open fork — whether this heat subsumes / feeds / or stands as a sibling to ₣Ba's mews-state half — is **resolved: sibling.** Neither heat depends on the other.
The two stores share an implementation pattern but not a home (see Sibling rhyme).

## Cinched

- This heat owns the standalone, global JJ state repo outright — the gallops, the captured chat histories, and the action-choices — decoupled from the jockeyed code (operator decision 260620; fully carved out of ₣Ba 260625).
- **Git stays source-of-truth and journal — never a transactional store.** Additive-only history, parseable commit-as-intent, jjx_log reconstruction, and free immutability all survive the move.
- **Global and central across stations, synced by a kit-owned pull-rebase-push loop.** The human never hand-manages the state repo's git. Git push is an atomic compare-and-swap on the remote; that CAS is the cross-station lock. Every state edit brackets as pull-rebase -> lock -> mutate -> push -> unlock.
- **Writers must be isolatable; single-blob storage is disqualifying.** A single canonical-JSON blob is the shared root of the merge pressure being felt today — every write contends on one file, every clone-sync is a 3-way merge. Per-heat sharding is the natural mechanism so two stations on different heats push without conflict; the exact representation is the git-database fork (Held).
- **Station-partitioned firemark/coronet minting.** Root state (next_heat_seed, heat_order) is the irreducible shared point even with per-heat shards; it needs a partition scheme so heat creation requires no cross-station coordination. The pensum-seed "cross-officium collision acceptable" stance is the template.
- **Exclusivity is soft.** No distributed-consensus machinery; an operator can administratively reclaim a stuck lock. Fits sole-operator.
- **Read path, not raw JSON.** Consumers reach gallops through a vvx read path (Exclusive JSON Ownership), never the raw files; reads take no lock and tolerate slight staleness.

## Held (open — refine before cutting paces)

- **Scope of "state."** Gallops only, or also the captured chat histories and the action-choices? The framing is the whole record; the boundary is unset.
- **State git-database representation — the design core that gates the build.** A peer set of JSON shards committed alongside, or something more primal (git objects/refs as the database directly). Git-stays-truth holds either way; sets the data shape and subsumes the per-heat sharding above. (Moved here from ₣Ba's mews-state half.)
- **Project dimension.** With many projects' records in one global repo: does a heat carry an explicit project, or is project implicit via the work-repo its paces' SHAs reference?
- **Clone location and sync model.** A gitignored subdirectory of each work repo (clones as disposable caches, remote CAS the only authority) vs one operator-scoped clone at a station path.
- **Conflict policy** on a genuine content conflict within one heat-file. The same-shard race is handled by CAS-reject + retry; a real content divergence on one heat is the open question.
- **Naming.** A standalone-state-repo concept may want its own register word; deferred until the design sets. ₣Ba's falconry "mews" is now fleet-only and not available for this.
- **Correlation strength** between the captured chat histories and the commits they produced — elaborated below; presumes the histories are in scope.

## Chat↔commit correlation — provenance of the action-choices (captured 260620)

A discussion about recording which model made each JJ commit collapsed into a question this heat owns:
once the captured chat histories live in the state repo, how strongly can a commit be tied back to the chat turn that produced it?

The model question dissolves into the captured histories.
The acting model is already recorded per-turn in the chat jsonl, so a durable model-per-commit fact needs no special capture — it is reachable by correlation, not stored twice.
This is the normalize/denormalize axis the whole discussion circled:
the captured transcript is the normalized source of truth — model, reasoning, context, all of it;
any field stamped on a commit is a denormalized cache of one slice, justified by query convenience alone, never by recoverability.

The correlation worth aiming for is a deterministic chain:
commit → its chat session → that session's transcript file → the exact turn that issued the commit → model and full context.
In today's co-resident world that chain is already half-built, worth knowing before the move:
- transcript → commit exists latently — the commit SHA is echoed into the record/close tool result, which lands in the jsonl — but unlabeled and fragile;
- commit → transcript is the missing half — the chat session id is written only into the open-ceremony commit, never into the record/close commits, so an arbitrary work commit can only guess its session by time, and concurrent sessions make that guess wrong.

Two small enabling changes close the loop, and they converge with the standing session-attribution gap:
- write the chat session id into every work commit — the commit→transcript link, and the same key that fixes concurrent-session attribution (one key, two payoffs);
- label the SHA the tool already emits — the within-transcript turn link.

A convention the design must pick:
a wrap writes two commits — the work commit and the chalk/state-transition commit — and only the work SHA is surfaced today;
decide which is the join anchor, or key both.

Whether this provenance ultimately lives as commit fields or purely as correlation queries over the state repo is itself the denormalize/normalize call — deferred until the scope-of-state boundary settles.

## Sibling rhyme (₣Ba — not a dependency)

This state repo and ₣Ba's fleet store are the **same implementation pattern** — git-backed, sharded, CAS-locked, journalled, synced across stations by pull-rebase-push, with atomic Rust manipulators.
The rhyme is real and acknowledged, but the two heats are deliberately decoupled: neither depends on the other, and each states its own store design.
The likely eventual reconciliation is at the vvx engine layer ("one engine, one lock"), the two domains — JJ planning record vs fleet — staying in separate homes; that convergence is future-chat work, not built speculatively here.
The one genuine seam is the (work-repo, SHA) cross-reference both sides need — here, the per-tack HEAD SHA a pace records for the work repo it touched.

## Done when

The design is resolved and recorded: JJ's record lives in a global git repo decoupled from the jockeyed code, the scope-of-state question is answered, the git-database representation is chosen, and the locked-edit transaction is specified.
The boundary against ₣Ba is already decided (sibling); what remains is this heat's own design core.
No construction begins until that design settles and the operator prioritizes the heat off stabled.

## Character

Notional capture, stabled holding pen — design requiring judgment, not mechanical work.
Standing now to hold a settled-direction initiative and its open forks until the operator can focus.
Resist building before the design core — the git-database representation and the locked-edit transaction — sets.

## Sources

- ₣Ba — the fleet kit (rig + mews); sibling sharing the git-CAS store pattern, not a dependency. Formerly fused with this concern; split 260625.
- `Memos/memo-20260615-chat-capture-and-cost-reconstruction.md` — the in-repo chat archive and "state gestalt" thread upstream of the chat↔commit correlation and the scope-of-state question.