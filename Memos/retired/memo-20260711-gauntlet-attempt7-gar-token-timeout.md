# Incident report: GAR token-endpoint timeout killed a one-shot tripwire pull (2026-07-10)

Status: absorbed with membrane (commit `be42ad698`); membrane unexercised since
(attempt 8 green with no transient), but identical-shape login membrane is
field-proven. Census home:
`Tools/rbk/vov_veiled/RBSWB-Watchbill.adoc` (the watchbill).

## Signature (grep anchors)

- `net/http: request canceled (Client.Timeout exceeded while awaiting headers)` — docker daemon stderr, on `Get "https://us-central1-docker.pkg.dev/v2/token?..."` inside a pull's manifest `Head`
- `docker pull failed for .../rbi_df/rbrd:tripwire` — the rbrd_check die that surfaced it
- Constant: `RBGC_DOCKER_LOGIN_TRANSIENT_SIGNATURE`; upstream: moby/moby#44350

## What happened

Gauntlet attempt 7 (pace ₢BsAAC), onboarding-sequence rung, case
`rbtdro_onboarding_ordain_airgap_chain`: the ordain of `rbev-bottle-ifrit-forge`
completed its conjure build green (6m19s wall clock), then died in the vouch
phase when `rbrd_check`'s tripwire `docker pull` hit one token-request timeout.
The `docker manifest inspect` seconds earlier — same endpoint, its own token
fetch — succeeded, and attempt 8 later ran the entire ladder against the same
backend without a blip: a single-request transient. Suite died 5-fixtures deep
(63 cases green, 1 failed). Evidence (station-local, operator-mutable):
trace dir `../temp-buk/temp-20260710-115901-2131730-480/rbtd/rbtdro_onboarding_ordain_airgap_chain/`,
pull stderr under the same run's `burv-temp/invoke-00055/.../rbndb_check_pull_stderr.txt`.

## Party analysis

- **Vendor (Google):** GAR's registry-auth backend answered one request slowly.
  Indeterminate latency, healthy service — not an outage, not a denial.
- **Foreign tool (docker):** hardcoded non-configurable 15s registry-auth client
  timeout (moby/registry/auth.go) fires prematurely against a
  healthy-but-slow backend, and the pull's initial resolve leg (token fetch +
  manifest HEAD) carries no internal retry — docker retries only layer
  downloads, after resolve. Both are moby/moby#44350 territory.
- **Ours (conduct gap):** both registry reads in `rbrd_check` were one-shot.
  Root cause of the gap: the 2026-05-30 survey (commit `40c429ddb`) wrapped
  `docker login` only, on the recorded premise "pull/push left untouched —
  docker retries them internally." That premise is false for the resolve leg,
  and it lived only in a commit message where nothing would re-test it.

## Repair

Commit `be42ad698` (2026-07-10): new `zrbndb_registry_read`
(`Tools/rbk/rbndb_base.sh`) — signature-gated bounded retry sharing the login
membrane's budget (`RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS`=3 /
`RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC`=3s) and signature constant. Wired at both
one-shot reads in `rbrd_check`: the pull that killed attempt 7, and the
manifest-inspect above it — where absorb-at-the-membrane is load-bearing
beyond flake-tolerance, because that site's failure path diagnoses
"tripwire absent" and prescribes depot re-levy, which must never fire on a
timeout. Non-matching failures return rc untouched to each caller's fail-fast;
exhausted budget dies naming the transient. The `rbgc_constants.sh` survey
comment now records the corrected scope. No rivets minted — class precedent is
the login membrane's inline moby#44350 citation; whether this class owes a spec
home is a superordinate-memo question.

## Doctrine linkage

Axis-B premise (`memo-20260521-hyperscaler-cost-cap-and-api-consistency-premise.md`):
bounded absorb scaffolding as permanent budgeted engineering, not a shameful
workaround — this membrane is that premise instantiated at the docker→GAR
boundary. No RBSCIP/RBS0 anchor applies (network/latency transient, not IAM
propagation, not serviceusage). RoE Palisade conduct
(`Tools/cmk/claude-cmk-roe.md`), point by point:
**characterize** — exact Go-stdlib string, version-stable, constant-ized;
**contain** — one membrane, in the module already owning this signature;
**absorb only the signature** — substring-gated, real auth failures
("unauthorized") never match; **log** — `buc_warn` per bend naming attempt
count and moby#44350, loud die on exhaustion; **retire** — below.

## Retirement

Demolition-conditioned in principle: unlike vendor-withheld capability, this
is a fixable docker-client defect — retire `zrbndb_registry_read`'s loop (and
the sibling login membranes) when the fleet's docker carries a moby#44350 fix.
No active tracking exists, so in practice it stands with the 2026-05-30 login
membrane and retires with it, as one class. Sibling incident reports of the
same run of gauntlet repairs: attempts 2 and 3, per the watchbill provenance.

<!-- eof -->
