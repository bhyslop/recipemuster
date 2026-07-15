# Memo: Cold onboarding shakedown — planning record (₢B0AAH)

Date: 2026-07-15
Status: planning record, pre-execution. Nothing below is cinched unless the
₢B0AAH docket says so; this memo is provenance for the deliberation, never
authority. Rulings graduate into the docket (or a formal home — see open
question 1, which is itself about where the durable form of this instrument
should live).

## What this pace is

₢B0AAH runs a zero-maintainer-context agent through the shipped onboarding
(a disposable anonymous clone of promoted public main) to surface doc gaps
before the operator's own greenfield walk (₢B0AAA). The cold agent's stumbles
are the finding set; findings fold into the release divergence census under
the ₣B0 paddock's divergence economics (doc-only → fast-follow by default).

## Grounded survey findings (260715, maintainer tree)

### 1. Credless reachability of the onboarding tracks

- **tadmor (rbw-Ots) is fully credless.** Its bottle vessel
  (`rbev-bottle-ifrit-tether`) bases on public `rust:slim-bookworm` with an
  empty `RBRV_IMAGE_1_ANCHOR`, so kludge pulls the base from Docker Hub.
  The whole evaluator track — kludge sentry + bottle, charge, 34-case
  adversarial suite — runs without any account. (Consistent with the `siege`
  suite being "fully local".)
- **moriah (inside rbw-Oda) is triple-walled against a cold agent:**
  1. Its bottle (`rbev-bottle-ifrit-airgap`) anchors its base into depot GAR
     (`RBRV_IMAGE_1_ANCHOR=rbi_hm/...`), and `rbfk_kludge.sh` deliberately
     dies when an anchored slot is not already in the local docker cache —
     anchored images must be wrested with credentials, out-of-band.
  2. Conjuring it requires a provisioned depot plus a director mantle.
  3. Donning any mantle is an interactive device-flow sign-in — no autonomous
     agent can complete it, regardless of spend policy.
  So there is no credless moriah execution path in the shipped surface, and
  none should be improvised: moriah execution belongs to ₢B0AAA's full walk.
- **ccyolo (rbw-Ofc) is credless up to a wall:** kludge, charge, and the
  containment-curl verification run cold; the "run Claude Code in the
  sandbox" step requires a Claude OAuth subscription login — interactive,
  human-owned. A cold agent hitting that wall and recording it is a
  legitimate finding, not a failure of the instrument.
- **Payor (rbw-Op) and the Director subtracks (rbw-Odf/Oda/Odb/Odg)** open
  with filesystem probes and redirect when prerequisites are absent. A cold
  agent can *visit* each — read the teach-prose, hit the prerequisite probe,
  and record whether the redirect is actionable to a stranger ("wall
  legibility") — at zero cost and zero spend.

### 2. Clone placement hazards (the "outside projects" consideration)

- **Station-file inheritance:** `BURC_STATION_FILE=../station-files/burs.env`
  (rbmm_moorings/burc.env) resolves relative to project root. Any clone under
  `~/projects/` silently inherits the maintainer's live station file *and the
  secrets directory beside it* — a containment leak, not merely a fidelity
  leak. Placing the clone elsewhere yields the true new-user state: the crash
  course walks the stranger through creating `../station-files/` fresh, which
  is exactly the navigation this shakedown must test.
- **Ancestor CLAUDE.md poisoning:** Claude Code loads CLAUDE.md files from
  every ancestor directory. The operator's `~/CLAUDE.md` is the
  STOP-WRONG-DIRECTORY sentinel, so a cold session anywhere under the
  operator's home directory opens with an instruction to refuse to work.
  (Observed directly: the maintainer-tree session loads it from two levels
  above cwd.) The clone must therefore live outside the home directory
  entirely, not merely outside `~/projects/`.
- **User-level context surveyed clean (this station, 260715):**
  `~/.claude/CLAUDE.md` is empty (0 bytes); user-scope `mcpServers` is empty;
  the vvx MCP server is project-scoped (`.mcp.json` at repo root). So the
  cold session sees only what the public clone actually ships. (Whether the
  shipped `.mcp.json` points at a binary a consumer doesn't have yet is
  itself something the walk will surface authentically — do not pre-fix it.)
- **Placement proposal (not cinched):** `/Users/Shared/rb-coldwalk/` — no
  ancestor CLAUDE.md, no station-file sibling, precedent for `/Users/Shared`
  use in this repo (APCK's `apcua`). Clone inside it; the agent creates
  `station-files/` beside the clone per the crash course; the divergence log
  (below) also lives beside the clone; the whole directory sweeps at discard.

### 3. Clean-tree gate behavior along the walk

`bug_require_clean_tree_creed` checks tracked modifications only
(`git diff` / `git diff --cached`) — untracked files pass. Kludge gates on
it, and kludge itself drives hallmarks into tracked `rbrn.env`, so the
consumer flow is kludge → dirty tree → commit → next kludge. The cold agent
will exercise that commit discipline authentically (local commits are fine;
the pre-push hook and removed origin block only escape). An untracked
divergence log would not trip the gate, but keeping the log outside the repo
is still preferred — harvest-before-discard cannot be fumbled.

### 4. Warm-cache fidelity caveat

This station's docker image cache is warm from ₣Bs proving. Consequences:
pass-through pulls (`rust:slim-bookworm`) may be skipped — steps identical,
timing diverges (minor, note-and-accept); and if the maintainer's GAR-anchored
images happen to be cached locally, a cold moriah kludge could *false-green*
— a success no stranger could reproduce. One more reason to hold moriah at
the wall-visit. Destructive pre-cleaning of the maintainer's images is off
the table.

## Proposed scope (not cinched)

Execute the credless arc fully; visit every credentialed track to its wall.

- Full execution: crash course (rbw-Occ) → kludged-crucible arc → tadmor
  adversarial suite end-to-end. ccyolo to the subscription wall.
- Wall visits: payor track and each Director subtrack opened and read cold;
  the prerequisite refusal and its redirect recorded for first-contact
  legibility. No execution past any wall.
- Moriah execution, payor founding, manor, gauntlet: reserved to ₢B0AAA
  (existing cinch: real spend and the founding belong to the operator's own
  hands).

## Draft cold prompt (v1, not cinched)

Stranger vocabulary only — every term below is on the shipped face
(README/menu ashlar), none is maintainer-interior:

> You are evaluating this project cold. You've just cloned it from GitHub and
> know nothing about it beyond what's in this working tree. Work only from
> the project's own documentation, starting with README.md.
>
> Your goals, in order:
> 1. Get the project's local container sandbox working on this machine:
>    build the images locally, start the sandbox, verify it runs.
> 2. Prove the project's security claim to yourself: run its adversarial
>    containment suite and read the results.
> 3. Survey everything the project offers beyond the local story: for each
>    documented track that requires accounts, credentials, or spending money,
>    read as far as the documentation takes you without signing in or paying,
>    and note exactly where and how it turns you away.
>
> Hard boundaries: never create accounts, sign into anything, spend money, or
> push anywhere — when docs ask, record the wall and move on. This machine
> may run other container workloads; never stop, delete, or prune anything
> you did not start.
>
> Recording discipline — this matters as much as the goals. Keep a running
> log at ../COLDWALK.md (outside the repo). Every time the documentation
> surprises you — a failing step, an instruction you can't find, a term used
> before it's explained, output you can't interpret, a probe contradicting
> the prose — append: what you were doing, what you expected, what happened,
> verbatim error text. Record first, then work around if you can. Blocked
> after a couple of attempts? Record the dead end and move to the next goal.

Design notes: the goal is stranger-shaped (the prompt never says crucible,
kludge, charge, tadmor — the docs teach those words); the recording
discipline and the boundaries are the harness's necessary artifice; the log
path is outside the repo per finding 3.

## Proposed model & effort (not cinched)

- **Cold walker: sonnet at default effort — deliberately.** A frontier model
  intuits around doc gaps, which blunts the instrument; sonnet is also the
  modal consumer. Pin at launch (`claude --model sonnet`). Do not raise the
  walker's effort: literal-mindedness is the sensor.
- **Pace orchestrator: sonnet designee, effort high** — but only after the
  open questions below are settled and their rulings encoded into the docket.
  The residue is careful mechanics (guarded clone, launch, wait, harvest,
  discard) plus faithful transcription; high effort is margin against the
  pull to "help" the walker or editorialize findings. Designee protocol
  governs: stop-and-surface on any hole.

## Open questions to settle before execution

Operator rulings needed; none of these is answered here by design.

1. **Repeatable instrument vs one-shot prompt** (operator, 260715). Should a
   *withheld* (never-delivered) tabtarget print the cold prompt, so the
   shakedown reruns at every release rather than living in one pace's chat?
   If yes, sub-questions follow: where the prompt text homes; the mint
   (colophon family — the release-ladder `rbw-M*` family is already withheld
   from delivery; or elsewhere); whether the clone-guarding steps (placement,
   `git remote remove origin`, pre-push hook, log path) script into the same
   tool or stay procedural; and how the pyx release-hygiene line is kept
   (withheld path discipline). Doctrine note: a durable per-release
   instrument homed in a memo is exactly the "durable knowledge in a memo"
   smell — if this becomes standing ceremony, it wants a formal home
   (tool + ceremony-document mention), and this memo retires into provenance.
2. **Launch mechanics.** Headless `claude -p` in background vs an
   operator-launched interactive terminal; permission mode for the cold
   session (`--dangerously-skip-permissions` inside the guarded clone, or
   something tighter); how completion is detected; expected wall-clock
   (two kludge builds + ~15 min suite + reading time).
3. **Census home.** Where the release divergence census physically lives —
   ₢B0AAA names it but no file exists yet; how this pace's findings and the
   operator walk's findings merge into one dispositioned census.
4. **ccyolo interior.** Accept the subscription wall as a recorded stop, or
   schedule a brief operator-assisted follow-on where the human performs the
   in-sandbox Claude login so the explorer track's interior prose gets
   walked too?
5. **Warm-cache handling.** Accept-and-caveat (annotate any divergence that
   is plausibly cache-shaped), or is a colder substrate worth the setup cost?
   (Pre-cleaning maintainer images is ruled out above.)
6. **Clone source verification.** Confirm at execution time that promoted
   public main is the correct base (per the ₣B0 260715 re-ruling the
   promotion has executed) and which URL the stranger-clone uses.
7. **Transcript harvest.** Is COLDWALK.md the sole harvest, or does the
   walker's session transcript ride along (the log captures what it noticed;
   the transcript captures what it did — silent workarounds hide in the
   difference)?
8. **Machine quiescence.** Crucibles share container namespaces; the walk
   charges and quenches crucibles on this station. Rule: no maintainer
   fixture/crucible activity during the walk — worth encoding where the
   executing agent will see it.

## Disposition

This memo banks the survey and the deliberation. The ₢B0AAH docket points
here and carries a not-ready-to-execute gate until the open questions are
ruled; rulings then graduate into the docket (or the formal home question 1
selects), and this memo becomes pure provenance.
