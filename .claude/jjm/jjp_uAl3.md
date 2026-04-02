## Character
User experience design with iterative testing. Each role track must feel obvious to someone who has never seen Recipe Bottle before — but only shows them what's relevant to their credentials. Requires testing on fresh machines with different role profiles.

## Context

Spun off from ₣AU (rbk-mvp-3-release-finalize). The original ₢AUAAm pace attempted a single "role-aware onboarding dashboard" in `rbgm_onboarding()`. Testing on a fresh machine as a retriever-only user revealed the fundamental problem: a monolithic guide can't serve four different humans with four different credential sets. Showing a retriever the payor's OAuth ceremony is noise that erodes trust.

## Design Decision Record

**Problem**: A single onboarding script that probes all roles and shows all steps is too clumsy for any single user. Retrievers don't need to see payor steps. Directors don't need governor context. The guide must meet each user where they are.

**Decision**: Replace the monolithic `rbgm_onboarding()` with a triage guide plus per-role track guides.

**Tabtargets**:
- `tt/rbw-o.OnboardMAIN.sh` — Triage: probe credential presence, show detected roles with links to role tracks, link to docs for roles without credentials
- `tt/rbw-OR.OnboardRetriever.sh` — Retriever track
- `tt/rbw-OD.OnboardDirector.sh` — Director track
- `tt/rbw-OG.OnboardGovernor.sh` — Governor track
- `tt/rbw-OP.OnboardPayor.sh` — Payor track

**Colophon design**: `rbw-o` (lowercase) is the triage entry point, terminal. `rbw-O` (uppercase) prefixes role tracks with children `R`, `D`, `G`, `P`. Terminal exclusivity holds.

**Role model** (dependency flows downhill):
- **Payor** — owns GCP project and billing, manages multiple depots
- **Governor** — administers a single depot: SA creation, IAM grants, role management
- **Director** — builds and publishes vessel images into a depot
- **Retriever** — pulls and runs vessel images from a depot

**Triage behavior**: Only shows role tracks where credentials are detected. No guidance for absent roles — just a URL to the public docs where users learn what credentials to obtain. The CLI guide activates once you have something to probe against.

**Per-role track behavior**: Linear checklist. Fresh probe every run (no history). Shows first red step with action instructions. Green steps above get one-line confirmation. Something like:
```
Retriever Track
  [ok] Service account key installed
  [ok] GAR access verified
  [>>] Summon a consecration    ← you are here
       Run: tt/rbw-Rs.RetrieverSummons.sh tadmor
```

**URL root**: A kindle constant (e.g., `RBGC_PUBLIC_DOCS_URL`) in `rbgc_Constants.sh` pointing to the public project page. Initially set to fork repo. `buc_link` renders clickable terminal URLs back to README/getting-started docs.

**Explicitly deferred**: Theurge test cases for onboarding guides. That's ₣A1 territory if needed.

## Provenance
Emerged from testing ₢AUAAm's monolithic dashboard on a fresh machine as a retriever-only user. The guide couldn't help — too much irrelevant content, no clear path for a single role.

## References
- `Tools/rbk/rbgm_ManualProcedures.sh` — current `rbgm_onboarding()` (to be replaced)
- `Tools/rbk/rbgc_Constants.sh` — home for URL root constant
- `Tools/buk/buc_command.sh` — `buc_link` for terminal URLs
- `https://scaleinv.github.io/recipebottle` — public project page
- ₣AU paddock — MVP-3 release context