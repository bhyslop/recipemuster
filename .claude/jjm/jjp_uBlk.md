## Shape

This heat holds a single notion:
Job Jockey's own record — the gallops, the captured chat histories, the action-choices it commits — should eventually live in a git repository separate from the code being jockeyed, not co-resident in the work repo as it is today.

Stabled now to make the idea discoverable and revisable; today's act is capture, not construction.
The build waits until the operator chooses to resolve the design forks below and reconcile this heat against ₣Ba.

## Why — the married-model contamination

Co-residence makes JJ bookkeeping commits part of the work repo's history and advances its HEAD.
The recorded symptom is the pensum-seed eviction: JJ commits advanced HEAD past origin and tripped foray's curia-readiness guard.
A separate state repo dissolves that contamination class entirely — not one instance of it — which is the core case for decoupling, weighted above any merge-convenience relief.

## Relation to ₣Ba (the rig / fleet-provisioning heat)

₣Ba fuses two concerns under one charter: fleet provisioning (the rig — an officeplace of test machines) and an operator-scoped shared state home (its mews half), where the state-decoupling already appears as a cinch.
This heat carves out the pure standalone-state-repo concern so it can be reasoned about independently of whether the rig is ever built.

The open reconciliation — and it is the first thing to settle when this heat activates — is whether this heat subsumes ₣Ba's mews-state half, feeds it, or stands as a narrower sibling (record decoupling only, no fleet semantics).
Do not duplicate ₣Ba's cinched state decisions here; cite and reconcile them.

## Cinched

- This heat is the standalone-state-repo concern, deliberately carved out from ₣Ba's fused fleet-provisioning + state charter so the two can be reasoned about separately (operator decision, 260620).

## Inherited / related decisions (from ₣Ba — confirm, do not re-cinch here)

- Git stays source-of-truth and journal, never a transactional store (additive history, commit-as-intent, jjx_log reconstruction all survive).
- Decoupling is the goal; the contamination dissolution is the case, weighted above merge relief.

These live in ₣Ba and apply to this concern; whether they bind here or get restated is part of the reconciliation.

## Held (open — refine before cutting paces)

- Scope of "state." Gallops only, or also the captured chat histories and the action-choices? The framing is the whole record; the boundary is unset.
- Subsume / feed / sibling vs ₣Ba's mews (above) — the gating fork.
- Clone location and sync model — live in ₣Ba's Held; this heat inherits the same forks for whatever subset it owns.
- Naming. A standalone-state-repo concept may want its own register word; deferred until the boundary against the mews settles.
- Correlation strength between the captured chat histories and the commits they produced — elaborated in the chat↔commit correlation section; presumes the histories are in scope.

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

## Done when

The design is resolved and recorded: JJ's record lives in a git repo decoupled from the jockeyed code, the boundary against ₣Ba's mews is decided, and the scope-of-state question is answered.
No construction begins until that design settles and the operator prioritizes the heat off stabled.

## Character

Notional capture, stabled holding pen — design requiring judgment, not mechanical work.
Standing now to hold a settled-direction idea and its open forks until the operator can focus.
Resist building, and resist re-cinching ₣Ba's state decisions here, before the boundary between this heat and the rig/mews settles.

## Sources

- ₣Ba — the rig and mews heat; carries the fused fleet-provisioning + state-decoupling charter and the already-cinched state decisions this heat must reconcile against.