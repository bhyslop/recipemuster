# GADS Repair Proposal Memo

This memo provides concrete proposals for repairing and simplifying GADS, based on our guiding principles:

* Partial credit is better than fragile perfection.
* Failures should be visible (orange markers), not silently “fixed.”
* Absolutely no mutable DOMs.
* Telemetry = structured JSON artifacts (debug files), not logs.

---

## 1. Partial Credit

**Proposal:**

* Any operation that cannot be placed exactly must produce a **Quarantine Marker (QM)**.
* QM is an inline `<span>` with `class="gads-deletion-unplaced"`, styled orange, and must carry `data-gad-reason`.
* QMs count toward invariants: `expected = placed + quarantined`.

**Engineer note:** Replace all fallback placement branches with QM creation. Never silently drop.

---

## 2. Failure Markers

**Proposal:**

* Define a formal spec term: **Quarantine Marker (QM)**.
* Repurpose “quarantine” terminology from existing unresolved-op handling.
* QMs become mandatory output when placement fails.

---

## 3. No Mutable DOM Trees

**Proposal:**

* Phase 1 and Phase 2 must produce **immutable DOMs**.
* Every later phase works on clones only.
* Remove all functions that traverse and mutate simultaneously (`locate_target` + insertion in one pass, etc.).
* Resolution = pure lookup; application = separate clone.

**Engineer note:** Review `gadie_locate_target` and ensure it never mutates. Audit Phase 6/7 code paths.

---

## 4. Telemetry vs Logs

**Proposal:**

* **Logs**: terse console for human debugging (`gadib_logger_*`).
* **Telemetry**: JSON payloads written by `gadib_factory_ship`, consumed as debug files.
* Spec states invariants must be enforced in telemetry, not logs.
* Example telemetry file fields:

  * `expected_deletions`
  * `placed`
  * `quarantined`
  * `invariant_satisfied`

---

## 5. Simplify Phases

**Delete Phases:**

* Delete Phase 8 (coalescing) — no more merging adjacent annotations.
* Delete Phase 9 (serialize) — rendering is just “output clone.innerHTML”.

**Retain / Adjust:**

* Phase 1: Immutable input.
* Phase 2: Detached working DOM + anchors.
* Phase 3: DFK capture.
* Phase 4: Diff operations.
* Phase 5: Semantic classification.
* Phase 6: Resolution + annotation (no fallbacks, produce QMs).
* Phase 7: Telemetry + invariants only (no extra logic).

Result: Phases 1–7 remain, but with gaps where 8 and 9 are removed.

---

## 6. Engineer Checklist

* [ ] Replace fallback placement with QM insertion.
* [ ] Ensure all route resolution is read-only.
* [ ] Remove anchor fallback code paths.
* [ ] Emit `phase6_quarantine_unresolved_ops.json` for all QMs.
* [ ] Remove Phase 8/9 from spec and code.
* [ ] Validate invariants: `expected == placed + quarantined`.
* [ ] Update spec docs: add **Quarantine Marker (QM)** definition.

---

## 7. Expected Outcomes

* Simpler pipeline (Phases 1–7 only).
* Visible, explicit failure markers (orange QMs).
* Deterministic, immutable processing.
* Telemetry that guarantees correctness checks.

This memo will guide both GADS repairs and parallel GADIE simplifications.
