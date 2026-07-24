# Incident report: serviceusage enable INTERNAL flake killed a fresh-project levy (2026-07-09)

Status: absorbed with membrane (commit `3cf94b248`); rivets minted at the spec
home, so the durable facts already outlive this memo. Census home:
`Tools/rbk/vov_veiled/RBSWB-Watchbill.adoc` (the watchbill).

## Signature (grep anchors)

- `encountered INTERNAL error: Help Token:` — the enable LRO's `error.message`, completed `done:true` with `error.code` 13
- `with failed services [containeranalysis.googleapis.com]` — same message's tail
- `PreconditionFailure` detail, violation type `googleapis.com`, subject `160009` — undocumented Google-internal token
- `API Enable containeranalysis: LRO completed with error — response saved:` — the buc_warn that surfaced it
- Fixture surface: `FAILED: rbtdrk_freehold_ensure`, `depot levy exit 1`
- Post-repair bend anchors: `transient INTERNAL (attempt`, exhaustion `INTERNAL persisted through`

## What happened

Gauntlet attempt 2 (pace ₢BsAAC), freehold-establish rung, 2026-07-09:

- 22:37:12 — depot-lifecycle's own fresh levy (`…100006`) enabled all seven
  depot APIs clean; its containeranalysis LRO completed in 3s.
- 22:43:44 — freehold levy of `…100007`: create, billing link, project number
  green; artifactregistry, cloudbuild (6s LRO), cloudresourcemanager enabled;
  the containeranalysis enable LRO completed `done:true` bearing INTERNAL at
  its first 3s poll. Levy died 22:44:15, fixture failed 34s in, suite exit 1
  (3 fixtures run, 51 cases passed, 1 failed).

Same code path, same station, same payor, 6½ minutes apart — a provisioning
race wholly inside Google. Durable logs: `logs-buk/hist-rbw-ts-gauntlet-20260709-223708-1946543-938.txt`
(suite), `logs-buk/hist-rbw-dL-sh-20260709-224344-1953301-309.txt` (levy).
Cleanup same chat: the half-founded `…100007` (billing linked, 3/7 APIs, no
terrier/SAs) unmade to DELETE_REQUESTED with billing unlinked. Friction found
there: the unmake guard's suggested recovery (marshal-zero, then retry) blanks
the very regime fields unmake validation requires — prefixes plus a `torndown`
placeholder moniker had to be hand-installed first.

## Party analysis

- **Vendor (Google):** serviceusage `services:enable` against a seconds-old
  project completed its LRO as *done with INTERNAL* — not a rejection, not an
  outage; the identical request succeeded in 3s on a sibling fresh project
  minutes earlier. Completion verdict handed back without a completion.
- **Foreign tool:** none — direct REST over rbuh/curl; transport healthy
  (HTTP 200s throughout, the LRO itself "completed").
- **Ours (conduct gap):** the enable path had no membrane while the codebase
  already carried sibling absorbs for adjacent Google indeterminacy (rbuh
  curl-transient retry, the SA-key-create flap streak, the RBSCIP IAM
  budgets). `rbge_api_enable` delegated its wait to the generic `rbge_lro_ok`,
  whose any-error crash-fast is right for its other eight callers but left a
  known flake class fatal on the one path that always runs seconds after
  project birth.

## Repair

Commit `3cf94b248` (2026-07-09): `rbge_api_enable` (`Tools/rbk/rbge_rest.sh`)
now runs its enable-and-await inline and retries the whole attempt only on the
surveyed signature (`error.code` 13), budget `RBGC_API_ENABLE_RETRY_ATTEMPTS`=3
/ `RBGC_API_ENABLE_RETRY_PAUSE_SEC`=15s, `buc_warn` per bend naming the saved
response and attempt count; any other LRO error, and exhaustion, die fast.
`rbge_lro_ok` untouched — its eight callers keep crash-fast. Both enable
callers (depot levy, manor instaurate) inherit. Rivets minted: `RBr_4e7`
(spoor — the exact signature and differential survey) and `RBr_d21` (membrane
— budget and retirement clause), homed at `RBS0 rbtoe_api_enable`.

## Doctrine linkage

Axis-B premise (`memo-20260521-hyperscaler-cost-cap-and-api-consistency-premise.md`):
this is the premise verbatim — an LRO that "completes" carrying a transient
INTERNAL leaves the completion contract in the customer's lap, so the bounded
absorb is budgeted permanent engineering. Spec anchor: `RBS0 rbtoe_api_enable`
(this incident's rivets); RBSCIP does not apply (serviceusage provisioning
race, not IAM propagation). RoE Palisade conduct, point by point:
**characterize** — exact code-13/Help-Token/160009 signature plus the
3s-sibling differential; **contain** — one membrane inside `rbge_api_enable`
alone; **absorb only the signature** — the retry gates on `error.code` 13,
everything else still dies clean; **log** — warn on every bend with the saved
response path; **retire** — below.

## Retirement

`RBr_d21` as minted names an explicit demolition condition: drop the loop when
fresh-levy ladders stop logging the bend across a full rebaseline cycle. That
clause was written before this chat read the Axis-B premise memo; under Axis B
the honest stance is *standing* — Google publishes no completion contract for
enable on fresh projects, so a quiet cycle proves quiet, not healed. Flagged
for the superordinate memo: either re-voice `RBr_d21`'s retirement clause to
Axis-B standing, or keep the demolition condition as a deliberate census
trigger. First-person frequency: one occurrence observed in this chat; later
attempts are other chats' evidence. Sibling incident reports of the same run
of gauntlet repairs: attempts 3 and 7, per the watchbill provenance.

<!-- eof -->
