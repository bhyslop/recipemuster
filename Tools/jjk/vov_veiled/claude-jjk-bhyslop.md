## JJK — bhyslop project specifics (veiled, not distributed)

This file holds the rbm-only scenery carved out of the public JJK guidance
(`Tools/jjk/claude-jjk-core.md`). It stays in this repo and is never
distributed to consumer projects.

### A. File Acronym Mappings — JJK Subdirectory (`Tools/jjk/`)

- **JJS0** → `jjk/vov_veiled/JJS0_JobJockeySpec.adoc` (Job Jockey specification - main file)
- **JJSCCH** → `jjk/vov_veiled/JJSCCH-chalk.adoc`
- **JJSCCU** → `jjk/vov_veiled/JJSCCU-curry.adoc` (Paddock operation - read/write heat paddock files)
- **JJSCDR** → `jjk/vov_veiled/JJSCDR-draft.adoc`
- **JJSCFU** → `jjk/vov_veiled/JJSCFU-furlough.adoc`
- **JJSCMU** → `jjk/vov_veiled/JJSCMU-muster.adoc`
- **JJSCNC** → `jjk/vov_veiled/JJSCNC-notch.adoc`
- **JJSCNO** → `jjk/vov_veiled/JJSCNO-nominate.adoc`
- **JJSCPD** → `jjk/vov_veiled/JJSCPD-parade.adoc`
- **JJSCRL** → `jjk/vov_veiled/JJSCRL-rail.adoc`
- **JJSCRN** → `jjk/vov_veiled/JJSCRN-rein.adoc`
- **JJSCRT** → `jjk/vov_veiled/JJSCRT-retire.adoc`
- **JJSCSC** → `jjk/vov_veiled/JJSCSC-scout.adoc`
- **JJSCSD** → `jjk/vov_veiled/JJSCSD-saddle.adoc`
- **JJSCSL** → `jjk/vov_veiled/JJSCSL-slate.adoc`
- **JJSCTL** → `jjk/vov_veiled/JJSCTL-tally.adoc`
- **JJSCVL** → `jjk/vov_veiled/JJSCVL-validate.adoc`
- **JJSCWP** → `jjk/vov_veiled/JJSCWP-wrap.adoc` (Close/wrap operation - mark pace complete and commit)
- **JJSRLD** → `jjk/vov_veiled/JJSRLD-load.adoc`
- **JJSRPS** → `jjk/vov_veiled/JJSRPS-persist.adoc`
- **JJSRSV** → `jjk/vov_veiled/JJSRSV-save.adoc`
- **JJSRWP** → `jjk/vov_veiled/JJSRWP-wrap.adoc`
- **JJSTF** → `jjk/vov_veiled/JJSTF-test-fundus.adoc` (Test Fundus — fundus scenario profiles and preflight contracts)
- **JJW**  → `jjk/jjw_workbench.sh` (workbench)

### B. Foray Protocol — Fundus constants (rbm values)

**Fundus constants** (use these for `jjx_bind`):
- Default reldir: `projects/rbm_alpha_recipemuster`
- BURN alias: use the alias from the target's BUK Regime Node profile (e.g., `winhost-wsl`, `winhost-cyg`)

Hosts in play for this repo:
- `winhost-wsl` / `winhost-cyg` — Windows host transports (WSL and Cygwin).
- `cerebro` — Linux test host used as a fundus by JJK fundus scenario tests.

In the public Foray Workflow, the bind example reldir is genericized to
`<your-project-reldir>`; the rbm-specific value is `projects/rbm_alpha_recipemuster`.

### C. Build & Run Discipline (rbm/VOK-specific)

**Build & Run Discipline:**
Always run these after Rust code changes:
- `tt/vow-b.Build.sh` — Build
- `tt/vvw-r.RunVVX.sh` — Run VVX

### D. Gradient delivery — mount and groom

Governs the Mount and Groom protocols in `claude-jjk-core.md`: when those
protocols say to surface the docket, approach, or paddock, this is *how* to
surface them.

Mount and groom surface heat artifacts (paddock, docket) written AI-for-AI and
dense by default; the operator does not read dockets directly and may be tired.
This is a *craft constraint on presentation, not a cap on thinking* — hold the
full analysis internally; shape the delivery so the operator has a place to
stand and a way to climb. Default, not gated on how long ago the pace was
slated: a same-session AI-dense docket is already a wall by afternoon.

At mount and groom:

- **Lead with one sentence the operator can stand on** — the pace goal (mount)
  or heat state (groom) — then *stop and let them pull the rest down*. Do not
  follow the opening sentence with the full docket, the approach, and every
  flagged concern in one turn. That sequence is the wall.
- **One path fully; alternatives in a line.** Present the single recommended
  path in full; give each alternative one line the operator can ask you to
  expand. Three fully-detailed options is word-wall in the costume of
  thoroughness.
- **Depth on request, in layers.** The operator throttles — "more," "the
  detail," "the others" pulls the next layer.

Shared antipattern names (use as shorthand, either direction) — concept homes
are MCM's Antipatterns catalog; this section carries only the avoidance moves:

- **Stun** (MCM `mcm_stun`) — the wall-of-prose delivery defect. Avoidance is
  the protocol above: one sentence to stand on, one path fully, depth in
  layers.
- **Molehill** (MCM `mcm_molehill`) — the manufactured decision-fork. Before
  presenting a decision or concern, weigh its real stakes; if small, treat it
  in a clause, not a fork. Unsure if it's a molehill? Say so in a line rather
  than building it up.

**The trot** — the Stun-recovery delivery pattern (as a horse is trotted out
in hand for inspection: deliberate pace, one animal at a time, the examiner
setting the rhythm). A dense deliverable is broken into a pre-counted sequence
of small chunks ("chunk 2 of 6"); each chunk opens with a plain-language
context reminder before its content; the operator paces the advance ("next"),
and the trot absorbs detours — a stray thought mid-walk spawns its own
discussion, then the walk resumes where it stood. A chunk often leans on work the operator did not author — a
prior instance's, or a session they convened but did not line-read; there the
context reminder grounds the referenced artifact *from scratch* rather than
invoking it by name, because the operator's strategy of building across editions
holds them, by design, at a remove from the words, so a name that feels shared to
the trotter ("the normative-register clause," "the blessed form") frequently
names nothing they have read. The pull to present such work *for ratification* is
the tell that review is being presumed where it is not — the cue to unpack before
being asked, not after. Distinct from gradient
delivery above, which layers depth on *one* deliverable for the operator to
pull: a trot serializes a *multi-part* deliverable, re-grounding at every
step for a tired reader. Use it both directions — the operator can ask to
"trot the findings," and the agent should offer a trot when it sees a dense
multi-part response coming or the operator reports fatigue.

### E. Gallops schema changes are gated on a forgiveness episode (JJK crate)

Changing the on-disk gallops shape — adding, removing, renaming, or retyping a
serialized field, or changing how one serializes — is a schema change, and a schema
change **must** register a forgiveness episode. Skipping it silently breaks reading
of every older-format gallops on disk. These changes are rare and easy to get wrong;
do not improvise. Before touching the schema, read `jjdz_forgiveness` in JJS0 and
follow its "Registering a new episode" procedure.

`JJr_a7c` (any `JJr_` token) in the crate is the cited forgiveness rivet —
`grep JJr_a7c` to that quoin for the rationale. The code it guards (the V3→V4
write-forward, frozen `jjrt_v3_types.rs`) is deliberately temporary, removed only
under the episode's demolition condition: do not clean it up or re-explain it beside
the marker. RBK's `RBr_` conduct rule, JJK-native.

### F. Branch Naming Discipline

Name every git branch with a uniform, age-sortable, identity-bearing shape, so the
branch list reads as a chronological ledger and each branch ties back to the heat or
pace it serves. Canonical form:

```
bhyslop-{YYMMDD}-{coronet-or-firemark}-{short-topic}
```

Example: this heat's bridle-retirement pace ₢BcAAG, on 2026-06-20, became branch
`bhyslop-260620-BcAAG-bridle-retirement`.

Segment rules:

- **`bhyslop-` prefix on every branch.** The operator namespace — reserved now so a
  future team's branches sort apart from the operator's.
- **`{YYMMDD}` second, before the topic.** Date-early so `git branch` sorts the
  namespace by age. Short `YYMMDD` (`260620`), never ISO `YYYYMMDD`.
- **`{coronet-or-firemark}` — the JJK identity the work belongs to, bare.** The
  base64 characters only (`BcAAG`, `Bc`) — **never** the ₢/₣ glyph: a glyph in a git
  ref is fragile across tab-completion, tooling, and platforms (the `pym-₢A_AAP`
  branches are the anti-pattern). **Preserve the identity's case** (`Bc`, not `bc`):
  lowercasing destroys the firemark — `bc` could be `Bc`, `BC`, `bC`, or `bc`, four
  distinct heats (the `bc-*` branches are that anti-pattern). Case-collision on a
  case-insensitive filesystem (macOS APFS) is a theoretical hazard only for two
  coronets differing solely in case; with a sole operator it does not arise, and if
  it ever did, disambiguate in the topic — never by lowercasing the identity. Omit
  this segment only when no pace or heat applies.
- **`{short-topic}` — kebab-case, a few words.**
