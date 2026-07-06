# Handbook Agent-Learner Evaluation — Design and First Run

Differential evaluation of whether the onboarding handbooks' teaching prose
carries measurable weight. A minimal eval artifact, not a harness: everything
here is executed by hand (agent prompts dispatched from a Claude session),
committed so the design and the first run's results are on record.

## Design

**Differential, never absolute.** Two variants of each evaluated track are
shown to otherwise-identical blind learner agents:

- **Full** — the track's rendered output verbatim (ANSI SGR and OSC-8
  hyperlink codes stripped, link display text kept).
- **Stripped** — teaching prose removed. The strip rule follows the source
  form exactly: keep section/step headers, commands (`buh_tt`/`buh_code`),
  and live probe lines; delete every `buh_line`/`buh_warn` teaching unit.
  A step that is pure teaching survives as a bare header.

The signal is the score delta between variants. An absolute score of either
variant is not meaningful — question difficulty is not calibrated.

**Anti-inference learner persona.** The learner reasons strictly from what
the handbook text says. If the handbook does not say it, the learner answers
"The handbook does not say." Outside knowledge of Docker/GCP/similar systems
is forbidden, as is any tool use (the text arrives inline in the prompt).

**Blind scoring.** A separate scorer agent receives the rubric and the two
answer sheets labeled only Learner A / Learner B, without knowing which
variant produced which. The variant assignment is recorded here, outside the
scorer's context.

**Synthetic critic.** Alongside the learners, a critic agent evaluates each
track's full text AS pedagogy — gaps, ambiguities, stumble points,
misconception risks — independent of the question set.

**Goodhart note.** Eval questions must be authored outside the track
author's working context, rotated periodically, and any human override of a
score carries a written justification. This first-run question set was
authored in a session that had not edited the evaluated track prose; on any
future run, rotate the questions before re-running — a question set that has
been seen by a session that later edits handbook prose is spent.

## Variant production recipe

1. Run the track tabtarget; handbook output rides stderr:
   `tt/rbw-Occ.OnboardingConfigureEnvironment.sh 2> capture.txt`
2. Strip ANSI: remove OSC-8 wrappers `\x1b]8;;...\x1b\\` (keep display
   text) and SGR sequences `\x1b\[[0-9;]*m`.
3. Stripped variant: from the track *source* (`rbho*_*.sh`), delete the
   rendered lines that originate from `buh_line`/`buh_warn`; keep
   `buh_section`/`buh_step*` headers, `buh_tt`/`buh_code` lines, and probe
   status lines.

## Evaluated tracks, this run

- **Crash course** (`rbw-Occ`, `rbhocc_crash_course.sh`) — CC questions.
- **Director first build** (`rbw-Odf`, `rbhodf_director_first_build.sh`) —
  DF questions.

## Learner prompt (persona)

> You are a brand-new team member on the Recipe Bottle project. Your ONLY
> source of knowledge is the handbook text provided below. Rules:
> (1) Use no tools — do not read files, run commands, or search; answer
> purely from the provided text. (2) Answer strictly from what the handbook
> text states. Where it does not state the needed information, write
> exactly "The handbook does not say." for that part. (3) Do not draw on
> outside knowledge of Docker, GCP, or similar systems, and do not guess
> from naming patterns unless the handbook teaches the pattern. (4) Answer
> each question in 2-5 sentences.

## Question set — first run (rotate before reuse)

### Crash course (CC)

- **CC-1** (application) You just cloned your team's repo onto a brand-new
  laptop and ran `tt/buw-rsv.ValidateStationRegime.sh`. It fails,
  complaining a field is missing. Your teammate runs the identical command
  on their machine and it passes. Explain why this difference is expected
  and what you should do next.
- **CC-2** (misconception) You are on a bare fork (no team infrastructure
  yet). `tt/rbw-rdv.ValidateDepotRegime.sh` fails. A teammate concludes the
  validation tool is broken. Are they right? What has to happen — and by
  which role — for that validation to pass?
- **CC-3** (application) A state-changing command just failed and its
  output scrolled away. (a) Which log file has a fixed path that tooling
  can always read, (b) what are the other two log files' behaviors, and
  (c) which artifact should you read first to understand a failed command's
  decision points?
- **CC-4** (application/generalization) Without consulting any directory
  listing: give the exact tabtarget command to VALIDATE the payor regime,
  and the exact tabtarget command to RENDER a nameplate regime, noting
  anything extra the nameplate command requires.
- **CC-5** (misconception) Yesterday you ran this crash-course handbook
  command and want to re-read its printed output today. Where under
  BURS_LOG_DIR will you find it?

### Director first build (DF)

- **DF-1** (misconception) Last month you conjured several vessel images.
  Today you ran conclave, yoked the new touchmark, and committed. Do the
  vessel images you built last month now contain the newer builder tools?
  Say exactly what yoke changed and what it did not.
- **DF-2** (application) A colleague summons your hallmark, then inspects
  the pulled `image:{hallmark}` locally and finds its digest does not match
  what vouch verified. Which ark basename carries the GCB-attested digests,
  and why can the pulled image's digest not serve for attestation?
- **DF-3** (application) On a freshly levied depot, your very first ordain
  fails a preflight check about missing builder tool images. What one-time
  operation was skipped? Is it per-vessel? Give the command(s) to fix it,
  in order, including anything you must do before re-running ordain.
- **DF-4** (misconception) Ordain completed successfully. Before capturing
  anything you ran the tally tabtarget to look around. Now
  `export HANDBOOK_HALLMARK=$(cat .../rbf_fact_hallmark)` finds no fact
  file. Why is it gone, and what should you have done?
- **DF-5** (application) Immediately after yoke rewrote every vessel's
  rbrv.env, you run ordain and it refuses. Why does it refuse, and what
  does the project gain from that refusal?

## Rubric

Each question scores 0-2 as the sum of two key elements (1 point each,
half-credit allowed). Track maximum 10. "The handbook does not say" scores
0 for that element; honesty is not additionally penalized.

- **CC-1**: [a] BURS is per-developer, local, gitignored — does not travel
  with the repo, so a fresh machine lacks fields; failure is expected.
  [b] Read the validator's error — it names the field and what to fill in.
- **CC-2**: [a] Not broken — on a bare fork the RBRD depot-identity fields
  are blank, so failure is the expected state. [b] The Payor must establish
  a Manor and Levy a Depot to populate them.
- **CC-3**: [a] Stable log, always the same path (e.g. `../logs-buk/last.txt`)
  — half-credit; other half for per-cmd (same filename across runs,
  diffable) + history (timestamped, never overwritten). [b] The Transcript —
  read first on failure; captures decision points and state transitions.
- **CC-4**: [a] `tt/rbw-rpv.ValidatePayorRegime.sh`.
  [b] `tt/rbw-rnr.RenderNameplateRegime.sh`, and it takes a target name.
- **CC-5**: [a] Nowhere — handbook display commands do not log.
  [b] Teaching output is ephemeral by design; re-run the command to see it.
- **DF-1**: [a] No — existing images still embed the old tool versions
  until rebuilt via ordain. [b] Yoke validated the stamp against GAR and
  rewrote RBRV_RELIQUARY in every vessel's rbrv.env — a regime change only.
- **DF-2**: [a] The attest arks (`attest:{hallmark}-{arch}`, per-platform)
  are the only arks carrying GCB-attested digests. [b] The classic Docker
  image store re-serializes manifests, so a pulled image's digest no longer
  matches what GCB attested.
- **DF-3**: [a] Conclave the reliquary; one-time, shared by all vessels —
  not per-vessel. [b] `tt/rbw-lC.DirectorConclavesReliquary.sh`, then
  `tt/rbw-rvy.DirectorYokesReliquaryAllVessels.sh <stamp>`, then commit
  before re-running ordain.
- **DF-4**: [a] The output directory is a fixed-path staging area that each
  tabtarget clears and recreates on entry — running tally wiped the fact
  file. [b] Capture/export the hallmark immediately after ordain, before
  running any other tabtarget.
- **DF-5**: [a] Ordain refuses to build from an uncommitted tree; commit
  the yoke changes. [b] Rationale: every artifact traces to a committed
  state.

## Run protocol

1. Produce full + stripped variants for each track (recipe above).
2. Dispatch four learner agents in parallel (one per track x variant),
   each blind to the others, persona prompt + variant text + that track's
   five questions inline. Uniform mid-tier model for all learners.
3. Dispatch one critic agent per track (full text) alongside.
4. Dispatch one blind scorer agent per track: rubric + the two answer
   sheets labeled Learner A / Learner B (assignment recorded below, outside
   the scorer's context).
5. Report per-question and total scores, compute the delta, disposition
   critic + learner findings.

## First run — 2026-07-06

Variant assignment (hidden from scorers): CC: A=stripped, B=full.
DF: A=full, B=stripped. Learners: claude-sonnet uniform. Scorers/critics:
claude-opus tier.

### Results

| Track | Full | Stripped | Delta |
|-------|------|----------|-------|
| Crash course (CC) | 10/10 | 1.5/10 | **+8.5** |
| Director first build (DF) | 10/10 | 2.5/10 | **+7.5** |

The teaching prose carries decisive, measurable weight on both tracks. The
full-variant learners scored perfect on every question including all four
misconception probes; the stripped-variant learners answered "the handbook
does not say" on most mechanism and rationale elements.

Where the stripped learners did score, the credit traces to structure that
survives the strip: step ordering (DF-3 fix sequence, DF-4 capture-before-
tally), command names (DF-3 "AllVessels" implying not-per-vessel), and live
probe text (CC-1's out-of-repo station path, CC-2's "RBRD populated" probe
caption). Two observations ride this: step structure and probe lines are
themselves load-bearing teaching, and despite the anti-inference persona a
learner still extracts meaning from names — realistic, and netted out by
the differential design.

Both scorers were blind and their per-element scores match a manual read of
the answer sheets; no human override was needed.

### Critic findings

Crash course critic (severity, one line each):

1. (high) Step 7's "the letter is all that changes" is contradicted by its
   own table — the workbench prefix also splits (buw- vs rbw-).
2. (high) Payor/Manor/Levy/Depot appear with no gloss.
3. (med) "Every state-changing command writes three Log files" — yet the
   read-only validator logs, teaching a false mutation/logging link.
4. (med) Only the stable log gets a concrete filename; per-cmd and history
   patterns are undiscoverable from the prose.
5. (med) Step 4 normalizes failure ("that is expected") without telling the
   learner how to distinguish expected from actionable failure.
6. (med) Step 8 closes "your repo environment is configured" unconditionally,
   contradicting the bare-fork branch of Step 5.
7. (low) BURC is never explicitly equated with "the config regime."
8. (low) "great for Claude" aside assumes an AI-assistant context.

Director first-build critic (severity, one line each):

1. (high) Step 3's capture command shows an operator-specific absolute path
   that would break on any other machine.
2. (med) Attest's "Durable" emphasis conflates durability with its actual
   distinguishing trait (sole GCB-attested-digest carrier).
3. (med) Conclave "prints a touchmark" but yoke's placeholder said <stamp> —
   the two are never equated.
4. (med) GAR is never expanded or defined.
5. (med) No description of what ordain success looks like after the 15-20
   minute blocking wait, nor what to do on failure.
6. (med) Probe checkboxes ([*]/[ ]) are never explained, and the negative
   summon probe prints after its remediation command.
7. (low) "Cleared on entry" is stated but not stressed as capture-now-
   before-any-other-command.
8. (low) The learner is told ordain dispatches on RBRV mode but not how to
   check which mode their vessel is in.

### Disposition

**Absorbed into handbook content (this pace's commit):**

- CC Step 6: "state-changing" qualifier corrected — read-only or
  state-changing, every command logs (critic CC-3).
- CC Step 6: per-cmd and history filename patterns now shown
  (`same-<cmd>`, `hist-<cmd>-<timestamp>`) (critic CC-4).
- CC Step 7: workbench-prefix wrinkle added — buw- for BURC/BURS, rbw- for
  Recipe Bottle regimes; letter rule holds within each family (critic CC-1).
- DF Step 1: GAR expanded on first use — Google Artifact Registry, the
  Depot's image store (critic DF-4).
- DF Step 1: `<stamp>` placeholder renamed `<touchmark>` and prose aligned,
  matching what conclave prints (critic DF-3).

**Handed to the docs-integrity sweep (the heat's terminal pace):**

- DF: name ordain's success signal and the failure path (critic DF-5) —
  needs verification against a live ordain run before prose is written.
- CC Step 4: distinguish expected from actionable validation failure
  (critic CC-5).
- CC Step 8: branch the closing on the Step-5 probe state instead of
  declaring success unconditionally (critic CC-6).
- Cross-track: decide whether the probe glyphs ([*]/[ ]) warrant a one-line
  legend at first use (critics CC — implicit, DF-6).

**Sweep disposition (docs-integrity pace, 2026-07-06): all four absorbed.**
The ordain success signal was verified against a live ordain transcript
(`hist-rbw-fO-sh-20260705-090534`: QUEUED/WORKING/SUCCESS poll ticks per
phase, automatic vouch closing with 'Vouch complete' and the 'This hallmark
feeds:' roster; failure path is a red ERROR naming phase + Cloud Build
status, with the Cloud Console link printed at submission) before the DF
prose was written. DF Step 2 also gained the mode-check render pointer
(DF-8's ride-along). CC Step 4 now teaches the expected-vs-actionable tell
(an expected failure names a field and its fix); CC's closing branches on
the depot probe state instead of declaring success unconditionally. Decision
on the glyph legend: yes — both tracks carry a one-line legend at the first
probe use ("[*] holds, [ ] needs action").

**Declined / false positives:**

- DF-1 "hardcoded absolute path" — capture artifact, not a defect: the
  handbook renders `${BURD_OUTPUT_DIR}` live per machine, so every reader
  sees their own path. Future eval runs should note this rendering fact in
  the critic prompt.
- CC-2 "no gloss for Payor/Manor/Levy/Depot" — those words are OSC-8
  hyperlinked to the README glossary in real terminal rendering; the critic
  judged de-linked plain text. The linked-term mechanism is the gloss.
- DF-2 "durable conflation" — the durability emphasis is deliberate, a
  recent correction of prose that mis-taught attest arks as ephemeral; the
  uniqueness claim is scoped to digests. Retained.
- CC-7/CC-8, DF-6 probe placement, DF-8 — minor voice/design choices;
  retained as-is (DF-8's mode-check pointer may ride along if the sweep
  touches Step 2).

Score sheets and variant texts were session artifacts; the recipe above
reproduces them against the handbook as it stands at any later date.
