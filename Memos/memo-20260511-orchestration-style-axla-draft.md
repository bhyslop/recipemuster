# Orchestration Style — Proposed AXLA Additions (Draft)

Working draft of the AXLA-level additions emerging from the orchestration
style-guide design discussion under heat `₣A-`. Not committed to AXLA
proper; codification deferred pending a second exemplar (likely RBSAC,
Cloud Build conjure) for stress-test on a different transport (cloud
REST vs Windows WSp-105).

The concrete exemplar driving this draft: `BUSJGW-GarrisonWsl.adoc`
(Windows workload garrison via WSL).

## Proposed Dimension

```asciidoc
[[axd_orchestrating]]
{axd_orchestrating}::
A
{axd_dimension}
indicating that a procedure or method composes external systems whose
timing and failure modes are not under the spec's control -- especially
where authoritative API specifications are absent, sparse, or actively
misleading.
The shape of the procedure is empirically discovered through repeated
contact with reality; the spec accumulates as truth over time, with
failure modes encoded, retries dialed in, and recovery paths named.
Valid on
{axvo_procedure}
and
{axvo_method}.
+
When present, detail-site lint enforces:
+
* Every {axhos_step} body contains >=1 {axc_*} control voicing.
* {axc_call} / {axc_submit} / {axc_poll} timing parameters appear
  literally in step prose, not derived at implementation time.
* Rationale narrative does not appear in step bodies; load-bearing
  rationale lives in a trailing Rationale section, citations in a
  trailing References section.
* Step granularity matches the operation's transport contract (e.g.,
  WSG WSp-105 for Windows transport; REST atomicity for cloud APIs).
* {axc_fatal} default scope is the surrounding operation;
  cross-procedure recovery scope is annotated explicitly when the
  failure routes to a sibling procedure or operator handbook.
```

## Anti-pattern: False Branches for Pretty Error Reports

A recurring overdesign in orchestration specs is the
**probe-then-conditional dispatch** where the dispatch is idempotent:
probe whether X exists, then conditionally remove X. When the dispatched
cmdlet has an idempotent absent-state flag
(`-ErrorAction SilentlyContinue` in PowerShell, `|| true` in POSIX,
exit-code-tolerance via specific accepted codes), the branch buys
nothing -- it inflates state space without buying diagnostic value.

**Principle:** a failed step with a unique `{axc_fatal}` (voiced
locally as `buc_die` / `rbbc_fatal` / `busc_fatal` / etc.) carrying a
step-identifying string is sufficient diagnostic for most orchestration
failures. The operator finds the failure site by searching the unique
string in logs; elaborate pre-flight probes for diagnostic prettiness
are not load-bearing.

**Reserve real branching for:**

1. **Dynamic discovery + iteration** -- where the target list is emitted
   by a probe and unknown ahead of time (e.g., `Get-CimInstance` with
   `LIKE` patterns emitting a variable number of profile rows; bash
   iterates and dispatches per row).
2. **Probe result consumed by something other than the dispatch** --
   where the result populates a variable used in a later step.
3. **Failure modes that genuinely differ in recovery** -- where "X
   absent" and "X exists but unremovable" route to different recovery
   procedures.

In all other cases, the simpler shape is one idempotent dispatch call.

## Worked Examples from BUSJGW

| Pattern                              | Phase | Treatment                                                            |
|--------------------------------------|-------|----------------------------------------------------------------------|
| `Get-CimInstance` + iterate rows     | 2     | **Branch retained** -- dynamic discovery of profile paths            |
| `Get-LocalUser` + conditional remove | 3     | **Collapsed** -- `Remove-LocalUser -ErrorAction SilentlyContinue` (¹)|
| `Get-ChildItem` + iterate rows       | 4     | **Branch retained** -- dynamic discovery of profile dirs             |
| `Test-Path` + conditional remove     | 4     | **Branch retained** -- arbitrary filesystem path, fails WSp-108 (¹)  |
| `wsl id` + conditional userdel       | 5     | **Collapsed** -- `userdel` with accepted absent-state exit code 6    |

(¹) PowerShell destructive-cmdlet rows are subordinate to **WSp-108**.
The false-branches principle is necessary but not sufficient for those
rows; the WSG carve-out's three conditions (narrow failure spectrum at
the callsite, downstream verification, single named suppression point)
must also hold. Phase 3 qualifies; Phase 4's Cygwin remove does not
(arbitrary filesystem path lacks a narrow failure spectrum). Non-PS
rows (POSIX `|| true`, exit-code-tolerance) are governed by this memo
alone -- WSp-108 is PowerShell-scoped.

## Notes

- **Naming.** The dimension is proposed as `axd_orchestrating` (present
  participle, adjective-shaped, parallel to peer dimensions
  `axd_attended` / `axd_internal` / `axd_grouped`). Alternatives:
  `axd_composing`, `axd_compositional`. Decision deferred until the
  RBSAC stress-test validates the dimension shape on a second
  transport.

- **Conditional control verb (deferred).** The "On X, call Y" pattern
  appeared in the original BUSJGW draft; the false-branches principle
  largely eliminates it. If a residual conditional pattern emerges
  from RBSAC or sibling orchestrations that cannot be flattened with
  an idempotent-dispatch substitution, a motif like `axc_branch` or
  `axc_when` may be warranted. Held back from minting now to avoid
  premature vocabulary growth.

- **Runtime variables.** The `«FOO»` convention plus `{axc_store}` /
  `{axc_use}` control verbs are sufficient -- no `axvo_variable` and
  no Runtime Values table needed. Variables are declared at their
  first STORE in sequence order; readers encounter them linearly. The
  control verb IS the declaration. A separate table would be a second
  source of truth subject to silent drift on step renumbering.

- **Trailing sections.** Two structural sections below
  `axhoc_completion`:
  - `Rationale` -- strictly load-bearing entries that prevent future
    "simplification" from regressing correctness (e.g., why a probe
    pattern is retained when it might look removable). Not narrative;
    not history.
  - `References` -- external citations as epistemic provenance, the
    trail back to ground truth when something breaks.

- **No invigilate-guarantee binding.** Tempting to specify that each
  `axhog_guarantee` has a matching observable probe in the paired
  invigilate procedure. Resisted: orchestration specs use a deliberately
  small vocabulary in a deliberately bad language (bash); turning the
  spec into a programming-language fragment harms readability more
  than it buys verification.

- **Migration scope.** Eventually all eight sibling orchestrations
  (BUSJGB / BUSJGC, BUSJCW / BUSJCM / BUSJCL, BUSJIW / BUSJIM / BUSJIL)
  plus the GCB orchestrations (RBSAC, RBSTB, possibly RBSAG, RBSAS)
  want the same discipline. Staged migration in a dedicated heat after
  Windows garrison is solid and the AXLA dimension is committed.
