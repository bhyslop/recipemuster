# Gauntlet rebaseline indeterminacy census — incidents, membranes, doctrine map

Date: 2026-07-11
Scope: pace ₢BsAAC (gauntlet-green), heat ₣Bs (rbk-10-rebaseline-all)
Status: standing record. Supersedes and retires
`memo-20260711-gauntlet-indeterminacy-incident-collection.md` (the collection
apparatus); the three incident reports it collected stand beside this memo as
provenance. Memos are provenance, never authority — every durable fact below
names its spec home, and the one class without a spec home is an open ruling.

## Why this memo exists

Eight gauntlet attempts produced a green rebaseline and, along the way, three
absorbed cloud-indeterminacy incidents — each diagnosed in its own chat, each
repaired with a Palisade membrane, each now carrying a first-person incident
report. This memo is the one place that remembers all three together: the
census, the classes they instantiate, the doctrine that governed the repairs,
and the rulings still open.

## Incident census

| # | Attempt | Incident report | Class | Membrane (site) | Budget | Rivets | Retirement stance |
|---|---------|-----------------|-------|-----------------|--------|--------|-------------------|
| 1 | 2 (2026-07-09) | `memo-20260711-gauntlet-attempt2-serviceusage-enable-internal.md` | Fresh-project provisioning race — serviceusage enable LRO completes `done:true` carrying INTERNAL (code 13) | `rbge_api_enable` inline whole-attempt retry (`Tools/rbk/rbge_rest.sh`) | `RBGC_API_ENABLE_RETRY_*` 3×/15s | `RBr_4e7` / `RBr_d21` at RBS0 `rbtoe_api_enable` | **Open ruling 1** — minted demolition clause vs Axis-B standing |
| 2 | 3 (2026-07-09) | `memo-20260711-admission-don-propagation-incident.md` | IAM propagation Class C — just-written admission grant unenforceable for ~minutes, no terminal-state signal (third operation type: `generateAccessToken`) | `rbtdrk_invoke_admission_settled`, fixture-side (the consumer downstream of the grant owns the wait) | `RBGC_PROPAGATION_*` 3s/2×/20s cap/420s | `RBr_7a9` / `RBr_3f4` at RBSCIP | Standing under Axis-B, deliberately not demolition-dated |
| 3 | 7 (2026-07-10) | `memo-20260711-gauntlet-attempt7-gar-token-timeout.md` | Foreign-client premature timeout — docker's hardcoded 15s registry-auth timeout, unretried pull resolve leg (moby/moby#44350) against one slow GAR token response | `zrbndb_registry_read` over `rbrd_check`'s pull + manifest-inspect (`Tools/rbk/rbndb_base.sh`) | `RBGC_HTTP_TRANSIENT_RETRY_*` 3×/3s | none — inline moby#44350 citation only | **Open ruling 2** — demolition-conditioned in principle, standing with the login membrane in practice; class may owe a spec home |

Non-incidents, for census completeness: attempts 1 (founding gap), 4 (billing
quota accounting), 5 (honest runway gate → ceremony), 6 (vacancy validation
contradiction) were deterministic; attempt 8 ran green with zero repairs.

## Membrane-exercised evidence

- **RBr_3f4 (admission settle):** attempt 8 absorbed 16 bends across
  brevet-director, don-director, and don-retriever — every one within budget,
  none escaping to a false failure. The membrane is field-proven at gauntlet
  scale.
- **RBr_d21 (enable retry):** one surveyed occurrence (attempt 2); no bend
  logged in attempts 3–8. Quiet, as a race that fires per fresh project would
  be.
- **zrbndb_registry_read:** unexercised since landing (attempt 8 saw no
  transient); its identical-shape sibling `rbgo_docker_login` /
  `zrbndb_docker_login` (2026-05-30, commit `40c429ddb`) is field-proven.

## Doctrine map

Layered, from conduct law down to constants:

1. **Conduct law — the Palisade.** `Tools/cmk/claude-cmk-roe.md` "The
   ungovernable, and the Palisade": characterize / contain / absorb only the
   surveyed signature / log the bend / retire with a removal condition. All
   three reports walk the five points explicitly.
2. **Project premise — Axis B.**
   `memo-20260521-hyperscaler-cost-cap-and-api-consistency-premise.md`:
   vendors withhold operation-completion contracts, so bounded
   retry/poll/timeout scaffolding is permanent budgeted engineering — honest
   poll-until-terminal, never blind sleeps. Elevation from memo to spec (an
   `rbsk_` Key Premise quoin) is already paced: **₢BsAAU**
   (axis-b-premise-mint).
3. **Domain spec homes.** `Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc` —
   the IAM propagation class taxonomy, call-context law (post-grant sites
   wait the budget, cold-call sites fail fast), admission-window siting rule,
   rivet census with evidence rows. `RBS0 rbtoe_api_enable` — the
   serviceusage enable membrane's spoor and budget. The moby#44350 class has
   **no spec home** (open ruling 2).
4. **Shared budgets.** `Tools/rbk/rbgc_constants.sh` — the three budget
   families above plus the surveyed-signature constants, each with its survey
   recorded in the adjacent comment block.

## Lineage — pre-pace incident record

The same doctrine absorbed earlier incidents outside this pace; cross-linked
here so the class record has one entry point:
`memo-20260513-iam-propagation-race-director-invest-gar.md`,
`memo-20260514-pristine-gauntlet-iam-flap.md`,
`memo-20260604-credential-churn-leak-and-propagation-races.md`,
`memo-20260604-dogfight-ordain-poll-connect-timeout.md`,
`memo-20260705-wifi-tls-timeout-incident.md`, and retired
`memo-20260611-heat-BH-class-c-setiampolicy-write-flap.md` (read-back-as-gate
falsified). Commit `40c429ddb` (2026-05-30) is the moby#44350 class origin —
its "pull/push retry internally" premise, falsified by attempt 7, lived only
in a commit message; that unrecorded-premise failure mode is exactly what the
incident-report pattern now guards against.

## Open rulings (operator)

1. **RBr_d21 retirement voice.** As minted it carries a demolition condition
   (drop when fresh-levy ladders stop logging the bend across a full
   rebaseline cycle); attempt 2's report argues Axis-B standing is the honest
   stance (a quiet cycle proves quiet, not healed). Either re-voice the rivet
   or keep the demolition clause as a deliberate census trigger — one ruling,
   applied at RBS0 `rbtoe_api_enable`.
2. **moby#44350 class home.** The class now has two membranes (login 2026-05-30,
   registry reads 2026-07-10) documented only in code comments and commit
   messages. Ruling: does it owe a spec home with rivets (parallel to
   RBSCIP/rbtoe_api_enable), and is its retirement standing-as-one-class or
   demolition-dated on an upstream docker fix?
3. **Unmake-guard recovery contradiction.** Twice observed friction (attempt
   2's report; attempt 7's chat): the live-depot unmake guard prescribes
   marshal-zero as recovery, but marshal-zero blanks the very regime fields
   unmake validation requires, forcing a hand-installed `torndown` dance.
   Candidate pace or itch — not an indeterminacy issue, recorded here so the
   observation isn't lost when the reports go quiet.

## Disposition

The collection memo retires with this memo's landing (roster fully
dispositioned: three reports collected, five rows classified non-incident).
The three incident reports stand as provenance. This memo itself retires when
its open rulings are dispositioned into spec homes or paces and a future
census supersedes it.

<!-- eof -->
