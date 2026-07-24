# GitHub SSH Flap and the JJX Remote-Op Stall Phenomena

2026-07-24, station `bhyslop-macmini-pym`. Recorded from the ₢B2AAB billet session
(officium `☉260724-1028-3jys`, billet clone `jjqb_200196_B2AAB`) during and after the
attempted wrap of pace ₢B2·B2AAB (readme-revetment-census). Several concurrent sessions
on this station stalled the same morning, though not all. This memo is the field record:
the observed phenomena, the forensic evidence, the hypothesized mechanisms, the state
they left behind, and the observability gap that made all of it hard to see.

Companion work: a docket for a JJ git-Palisade vedette pace (₣B9 candidate,
suggested silks `git-palisade-vedette`) was drafted in a parallel session; the
assessment at the end of this memo maps these phenomena onto it.

## The environment

- Six concurrent JJ billet sessions live on the station at the time, each with its own
  vvx MCP server (per-server `cwd` confirmed via `lsof`): billets for ₢B2AAB, ₢BsAAW,
  ₢CAABW, ₢CAABY, plus groom/lunge billets for ₣B9 and ₣B0. Each runs jjx machinery
  that reaches `github.com` over SSH: dispatch gleans, record pushes, refits, studbook
  syncs, wrap ceremonies.
- SSH local configuration is bare and deterministic: `ssh-add -l` reports "The agent
  has no identities" (auth comes from default on-disk identity files), and there is no
  `~/.ssh/config` (no ControlMaster, no per-host settings). Nothing local varies
  per-connection.

## Phenomenon 1 — transient GitHub SSH auth denial (the underlying foreign behavior)

An identical remote git operation intermittently fails with
`git@github.com: Permission denied (publickey). / fatal: Could not read from remote
repository.` and succeeds on immediate retry with the environment unchanged.

Field measurements, all same-morning, same station:

- Controlled probe: six back-to-back `ssh -o BatchMode=yes -T git@github.com` from one
  shell → five "successfully authenticated", one `Permission denied (publickey)`, all
  within seconds. Reproduced the same 1-in-N rate on a later 3-probe run (3/3 clean).
- `git ls-remote origin` from the billet clone: denied on first invocation, clean on the
  immediate second (66 refs returned).
- Operator terminal, trunk clone: `git pull --no-ff` denied, identical retry seconds
  later merged cleanly.

Since the local key path is deterministic, the per-connection variance is server-side:
GitHub's SSH edge is a load-balanced fleet, and the burst profile of six billets'
machinery hitting it concurrently from one IP is the classic shape for tripping
per-connection throttling/abuse tolerance. Whether this is a GitHub-side fault or a
tolerance ceiling for this use model, the conduct conclusion is the same: this is a
Palisade neighbor returning verdicts that do not match reality, and it must be absorbed
by signature, not trusted.

## Phenomenon 2 — silent indefinite stall of `jjx_close` (the serious one)

Four `jjx_close` attempts were made for ₢B2AAB with a correct, committed, pushed,
fast-qualify-green billet. Their behaviors:

- **Attempt 1**: no response for ~8 minutes (harness backgrounded it at 120 s); killed.
- **Attempt 2**: no response for ~10 minutes; killed. Forensics later showed this
  attempt (or possibly attempt 1, late) **half-completed the wrap** — see Phenomenon 4.
- **Attempt 3**: answered *promptly* — with the staleness-gate INTERDICTUM
  (Phenomenon 4 explains why). Proof that the gate path, which runs before remote
  mutation, was never the slow part.
- **Attempt 4** (after an operator-directed refit): silent again for 6+ minutes; killed
  on operator instruction.

The forensic signature during every silent period, checked repeatedly:

- The session's vvx MCP server process alive, state `S`, essentially zero CPU
  accrual (0:00.20 → 0:00.25 over ~20 minutes), **no child processes at all** — so no
  hung `git push` subprocess, no credential prompt; the remote transport is in-process.
- Zero side effects on the ground while silent: no new commits in the billet clone, no
  studbook journal entry, clean working tree.
- No locks anywhere: no lock refs and no `.lock` files locally or on either remote
  (project origin and studbook origin both enumerated clean during the stall).
- Every *other* jjx verb answered instantly through the same server, before, between,
  and even *during* a pending close (`jjx_open`, `jjx_record`, `jjx_refit`,
  `jjx_brief`). The stall is specific to the close path's remote-op sequence.

**Hypothesized mechanism.** A denial (Phenomenon 1) fails fast and loud; it cannot
produce a multi-minute silence. The stall shape — zero CPU, no children, no error, and
in one case *eventual progress after ~10 minutes* — fits a TCP-level stall: a connection
that completes the TCP handshake and then hangs (throttled/tarpitted or a dying
frontend), with the in-process transport sitting in a socket wait with **no application
deadline**. Kernel retransmission gives up on the order of 9–15 minutes, after which the
operation either errors or — as observed once — simply succeeds late and the ceremony
continues to its next remote op, where the lottery draws again. Under a flap, a
multi-connection ceremony like wrap (trunk push + studbook journal + push + billet ops)
gets multiple draws per attempt and stalls with high probability, while single-shot
verbs usually get through. This would also explain why *some* sessions on the station
stalled and others did not: exposure is proportional to remote connections per ceremony.

Implication for the fix: **a retry membrane alone cannot catch this face.** The stalled
operation never returns a verdict to classify. The membrane must be
deadline → classify → retry → guided-fail, in that order; without the per-op deadline,
the retry logic is unreachable exactly when it is needed most.

## Phenomenon 3 — transient denial misclassified as "offline"

Two consecutive `jjx_refit` invocations from the ₢B2AAB session returned the graceful
verdict:

> jjx_refit: offline — merged the last-gleaned position of main into
> personal/bhyslop/jjls_pace/B2AAB; nothing pushed. Re-run refit once the remote is
> reachable.

Direct probes in the same minutes showed the remote fully reachable (3/3 SSH auth
clean, `ls-remote` clean). So a transient failure is *already* being absorbed somewhere
on this path — into the wrong category. The offline verdict's named remedy ("wait for
reachability") is false for this signature, and it does not retry even though an
immediate retry demonstrably succeeds. A classifier that will grow a transient category
must distinguish it not only from the panic/unclassified pole but from this existing
offline verdict, or the transient keeps masquerading as an outage.

(An earlier refit in the same session, ~30 minutes prior, succeeded normally —
"refitted, merged and pushed" — confirming the path works when the lottery is kind.)

## Phenomenon 4 — crash-mid-wrap split state and the self-referential staleness refusal

The killed attempt 2 did not die cleanly-before-everything as its silence suggested. It
**landed the trunk half of the ceremony and not the rest**:

- The W chalk commit for ₢B2AAB (`106c06352`, chalk header
  `jjb:1019-be3d20a63:₢B2AAB:W:`) exists as the tip of `refs/heads/main` on origin —
  committed and pushed by the dying attempt.
- The studbook has **no** ₶ journal entry for the wrap (its tip remained the prior
  session's entry throughout), and the pace remained open.
- The billet branch on origin remained at its pre-wrap tip (`be3d20a63`).

Attempt 3 then refused with (verbatim):

> INTERDICTUM — wrap staleness gate: jjx_wrap refuses; billet branch
> 'personal/bhyslop/jjls_pace/B2AAB' is behind the remote counterpart of trunk 'main',
> so trunk carries work this billet has never enfolded. … Remedy: refit …

The gate was factually right and semantically wrong: the "work this billet has never
enfolded" was **the pace's own W commit**, landed by the interrupted predecessor. The
subsequent operator-directed refits then merged the pace's own chalk back into its
billet. Wrap has no recognition of an already-landed chalk for its own coronet, so a
crash between the trunk push and the studbook journal leaves a state that reads as
ordinary trunk drift and steers the operator into a refit-of-own-work loop.

As of this memo the split stands: W chalk on main; pace open; no studbook entry; billet
holding unpushed enfold merges of its own chalk. Reconciliation is pending operator
direction — deliberately not improvised from inside the session.

## The observability gap (the meta-finding)

Every fact above was recovered *forensically*: process tables, `lsof`, repeated
`git log`/`ls-remote` sampling on three repos, and diffing ground state between
attempts. The machinery itself emitted nothing during the stalls — no step
announcements, no journal of which ceremony phase was in flight, no record afterward of
how far a dead attempt got. The single most expensive consequence: attempt 2's
half-landed wrap was invisible for ~20 minutes and two further attempts, because
"nothing observable happened" and "the trunk push already succeeded" produce identical
silence.

**Recommendation: a JJ step journal.** Every jjx ceremony should journal each step —
timestamp, officium, verb, target, step name, outcome (or entry-without-exit) — to an
append-only local sink that does **not** depend on the network path being observed
(the failure domain under diagnosis must not be the journal's transport). The officium
scratch directory is the natural home; per-officium files, append-only, survive the
process. With such a journal, each phenomenon above collapses from a forensic
reconstruction to a single read: the stall shows its exact blocked step and elapsed
time; the half-landed wrap shows "trunk push: ok / studbook journal: never entered";
the offline misread shows the raw error it classified. This is the same observability
contract the RB side already imposes on its own foreign edges (transcripts, BURX
liveness facts, announced log paths) — JJ's ceremonies have outgrown running dark.

## Mapping onto the proposed git-palisade-vedette docket

The parallel session's docket (transient category in `jjrfg_plaingit`, retry with
bounded backoff, guided `buc_die` on exhaustion, JJSVF sheaf-first, census of
remote-reaching farrier ops) is confirmed by this session's evidence **for the denial
face** — Phenomenon 1 is exactly its cinched transient, now with a measured 1-in-6
same-second rate.

It does not, as drafted, cover:

1. **The stall face (Phenomenon 2)** — needs per-op deadlines ahead of the classifier;
   a panic-to-retry conversion never fires on an op that never returns. "No remote
   flake panics anywhere in dispatch" can be fully green while wrap still goes dark for
   ten minutes.
2. **The offline misread (Phenomenon 3)** — the transient category must be carved away
   from the existing offline verdict, not only from the panic pole.
3. **Crash-mid-wrap chalk detection (Phenomenon 4)** — a separate pace: wrap (or its
   staleness gate) recognizing its own coronet's already-landed chalk and resuming the
   back half instead of interdicting against it.
4. **The step journal (observability)** — a separate pace; arguably the one to land
   first, since it makes every other fix verifiable in the field.

Near-term station-side mitigations, independent of the paces: ControlMaster
multiplexing for `github.com` in `~/.ssh/config` (collapses the six-billet burst into
one persistent authenticated connection — directly shrinks the trigger profile);
loading the key into the agent is hygiene but likely immaterial to the intermittency.
