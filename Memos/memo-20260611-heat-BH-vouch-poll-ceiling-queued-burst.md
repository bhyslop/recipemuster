# Vouch poll ceiling conflates QUEUED wait with execution — skirmish casualty

Date: 2026-06-11 (~04:45, skirmish ladder attempt 4)
Status: finding with trace evidence, no verdict — for the ₣BH terminal memo triage.
Shape-sibling of the progress-aware-convergence-deadline finding (RBSHR entry):
a fixed ceiling penalizing exactly the runs that queue.

## Phenomenon

`rbtdro_onboarding_ordain_conjure_jupyter` failed at the ordain's Vouch build:
the build sat **QUEUED for 47 of 50 polls (~235s)**, transitioned WORKING at
poll 48, and the host ceiling (`rbfcb` wait-build-completion, 50 polls x 5s)
expired at poll 50 — "Build timeout after 50 polls". The build itself was
healthy and was likely completing orphaned on GCP after the host abandoned it
(unverified — by the time of writing, the suite had moved on).

Attempt context: third ordain of the onboarding sequence that run; the two
prior ordains' builds ran promptly. The QUEUED burst is pool-capacity /
scale-up weather (vouch rides the airgap pool), not a property of the build.

## The shape

The poll ceiling charges queue-wait and execution against one budget. QUEUED
time measures the worker pool's capacity and scale-up clock — the neighbor's
weather, not our build. A ceiling sized for execution (vouch completes in
~30-60s once WORKING) gets eaten whole by a 4-minute queue burst.

## Repair question for triage (not decided here)

Candidate shapes: (a) start the execution budget at the QUEUED->WORKING
transition, with a separate (generous) queue allowance — the clean fix matching
what the two clocks actually measure; (b) raise the vouch ceiling outright
(blunt, re-conflates); (c) leave it and accept suite flakiness under pool
contention (rejected by tonight's evidence — it cost a full skirmish attempt).
Census note for (a): every `ZRBFC_BUILD_POLL_CEILING_*` consumer shares the
conflation; the fix belongs in the shared wait-build-completion loop
(`rbfcb_BuildHost.sh`), not per-kind.

## Night context (the full skirmish ledger, for the wrap)

Attempt 1: Payor RAPT expiry (operator reauth). Attempt 2: Class-C setIamPolicy
write flap (own memo). Attempt 3 + standalone: stale rbf_fact_reliquary test
reader (fixed, 2274b2ac2). Attempt 4: this QUEUED-burst timeout, 56 cases green
before it — past canonical-invest, conclave+yoke, both kludges, and two conjure
ordains. Four failures, four distinct causes, zero product-code defects among
them (one test-code defect, three environmental).
