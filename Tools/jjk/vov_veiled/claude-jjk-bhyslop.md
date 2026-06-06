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

Shared antipattern names (use as shorthand, either direction):

- **Stun** — a wall of dense, fully-structured prose with no on-ramp;
  comprehension seizes before it can enter. The finished staircase, seen from
  outside, is a wall.
- **Molehill** — manufacturing a significant-looking decision-fork or challenge
  out of something nearly inconsequential, then building structure around it.
  Before presenting a decision or concern, weigh its real stakes; if small,
  treat it in a clause, not a fork. Unsure if it's a molehill? Say so in a line
  rather than building it up.
