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

## Done when

The design is resolved and recorded: JJ's record lives in a git repo decoupled from the jockeyed code, the boundary against ₣Ba's mews is decided, and the scope-of-state question is answered.
No construction begins until that design settles and the operator prioritizes the heat off stabled.

## Character

Notional capture, stabled holding pen — design requiring judgment, not mechanical work.
Standing now to hold a settled-direction idea and its open forks until the operator can focus.
Resist building, and resist re-cinching ₣Ba's state decisions here, before the boundary between this heat and the rig/mews settles.

## Sources

- ₣Ba — the rig and mews heat; carries the fused fleet-provisioning + state-decoupling charter and the already-cinched state decisions this heat must reconcile against.