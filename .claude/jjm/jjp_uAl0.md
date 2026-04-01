## Gestalt

rbf_Foundry.sh is a ~4,600-line monolith — 19 public and ~25 private functions spanning four unrelated domains that share a single kindle/sentinel gate. This heat explodes it into five independently-sourceable modules with proper credential boundaries.

## Why now

The monolith forces every Foundry consumer to pass director credential checks even for operations that don't need them (retriever-only wrest/summon, zero-credential plumb). This is the director-gate problem surfaced in ₣AU (₢AUAAn). Decomposition resolves it structurally rather than with workarounds.

GitLab integration elimination in ₣Av left dead code throughout Foundry — cleaning that first reduces noise in the decomposition.

## Target architecture

| Module | File | Domain | Credentials |
|--------|------|--------|-------------|
| rbfc | rbfc_FoundryCore.sh | kindle, sentinel, GCB poll/wait, stitch, plumb | none |
| rbfd | rbfd_FoundryDirectorBuild.sh | ordain, conjure, enshrine, graft, kludge, mirror | director |
| rbfv | rbfv_FoundryVerify.sh | about, vouch, batch_vouch | director |
| rbfl | rbfl_FoundryLedger.sh | inscribe, tally, abjure, delete | director |
| rbfr | rbfr_FoundryRetriever.sh | wrest, summon | retriever |

All child modules source rbfc. rbf becomes a non-terminal parent prefix.

## Sequence rationale

Clean (gitlab purge) → Plan (dependency map) → Core first (shared infra) → Retriever (cleanest cut, no director deps) → Verify → Ledger → Director-Build (residual rename).

## References

- rbf_Foundry.sh — the monolith
- ₣Av — GitLab elimination heat (builds.create + pouch replaced trigger dispatch)
- ₢AUAAn in ₣AU — director-gate problem this decomposition resolves