## JJK — bhyslop project specifics (veiled, not distributed)

This file holds the rbm-only scenery carved out of the public JJK guidance
(`Tools/jjk/claude-jjk-core.md`). It stays in this repo and is never
distributed to consumer projects.

### A. File Acronym Mappings — JJK Subdirectory (`Tools/jjk/`)

- **JJS0** → `jjk/vov_veiled/JJS0_JobJockeySpec.adoc` (Job Jockey specification - main file)
- **JJSAB** → `jjk/vov_veiled/JJSAB-breeze.adoc` (Breeze — aspirant sheaf (JJSA*: aspirant family, B: breeze), founding member of the full-prefix naming the legacy `JJS-aspirant-*` files predate: the attention-shaped execution pipeline recast from the retired V4 workup — longe/school/breeze/corral, school carving a beat map from the gait option-set, breeze parceling beats to variously configured LLM sessions, provender/nosebag informational flow, candidate vocabulary re-ratifying at graduation, and a standing reconciliation obligation against the studbook/farrier/blotter substrate. Mounted in JJS0's Aspirant Nucleations; mulling heat: jjk-13-aspirant-breeze.)
- **JJSCCU** → `jjk/vov_veiled/JJSCCU-curry.adoc` (Paddock operation - read/write heat paddock files)
- **JJSCDR** → `jjk/vov_veiled/JJSCDR-draft.adoc`
- **JJSCFU** → `jjk/vov_veiled/JJSCFU-furlough.adoc`
- **JJSCMU** → `jjk/vov_veiled/JJSCMU-muster.adoc`
- **JJSCNC** → `jjk/vov_veiled/JJSCNC-notch.adoc`
- **JJSCNO** → `jjk/vov_veiled/JJSCNO-nominate.adoc`
- **JJSCPD** → `jjk/vov_veiled/JJSCPD-parade.adoc`
- **JJSCRL** → `jjk/vov_veiled/JJSCRL-rail.adoc`
- **JJSCRN** → `jjk/vov_veiled/JJSCRN-rein.adoc`
- **JJSCRP** → `jjk/vov_veiled/JJSCRP-reprieve.adoc` (Reprieve — schema-change tolerance doctrine: the mechanism, the multi-install convergence model, schema-change delivery, and episode registration; `include::`d into JJS0 `== Serialization` at `jjdz_reprieve`)
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
- **JJSVB** → `jjk/vov_veiled/JJSVB-blotter.adoc` (Blotter — executory entity sheaf (B: blotter): the locked-store entity — linear, single-writer-under-lock, engine-driven git repo; the lock ref `refs/jjv/guidon` (interior shape settled 260706: one well-known ref per blotter, the store discriminator is the repo itself), the guidon, lockless-read posture, the studbook/mews instances, engine-known bootstrap config; methods summarized by citation to JJSVJ. Homes `jjdb_blotter`/`jjdb_guidon` and the `jjdk_lockless_reads` premise. Mounted in JJS0's Revision Control region.)
- **JJSVC** → `jjk/vov_veiled/JJSVC-cosmology.adoc` (Revision-Control Cosmology — executory cosmology sheaf: the world model of the revision-control substrate (infield peer ring, hippodrome, yard family, why-decouple), mounted in JJS0's Revision Control region; homes the `jjdw_` quoin category. First member of the **JJSV*** family (V: revision-control) — the destination-sheaf family of the studbook–farrier MVP, non-terminal, siblings landing as the aspirant nucleations drain.)
- **JJSVF** → `jjk/vov_veiled/JJSVF-farrier.adoc` (Farrier — executory entity sheaf (F: farrier): the polymorphic revision-control driver — one trait, one implementation per farrier kind, capability structural via the three facets (core/lock/billet: blotter carrier core+lock, hippodrome kind core+billet); homes `jjdf_farrier`/`jjdf_core`/`jjdf_lock`/`jjdf_billet`, the identify contract `jjdf_identify` (explicit probe path, claim-or-decline ground detection, the four resolutions root/upstream-key/seat/line-of-work, total for a claimed tree), the vocabulary Palisade `jjdf_palisade` (git words stop at the driver implementation boundary), the op census as method contracts under the census words (comb/lodge/glean/consign, guidon verbs stake/pluck/sight, billet ops billet_create/billet_remove/enfold, deferred counterfoil_verify), the rejection-kind taxonomy, and the never-force rivet homed at the consign contract. Mounted in JJS0's Revision Control region.)
- **JJSVJ** → `jjk/vov_veiled/JJSVJ-journal.adoc` (Journal Ceremony and Break Sequence — executory routine sheaf (J: journal): the blotter's shared write bracket — durable-first work half, stake/sight lock take, advance, mutate-and-lodge with counterfoil, atomic lease-bound consign, best-effort pluck — and the lease-guarded never-blind break sequence; the ONE home every blotter-writing operation cites. Homes `jjdb_journal`/`jjdb_break` and the `jjdk_sole_door` premise. Mounted in JJS0's Revision Control region.)
- **JJSVS** → `jjk/vov_veiled/JJSVS-studbook.adoc` (Studbook — executory entity sheaf (S: studbook): the regional record repo as the blotter's first instance — the gallops' physical vessel, one store serving every station and every jockeyed project. Homes `jjdb_studbook`, `jjdb_pedigree` (the per-upstream record: farrier kind + tackle bindings, keyed by origin URL), and `jjdb_counterfoil` (the journaled member→SHA stub, one-way reference, rebase soft spot); also the no-worktree-paths rivet home, journal-entries-as-commits, and scope at birth (gallops first tenant, chats the deferred second). Mounted in JJS0's Revision Control region.)
- **JJW**  → `jjk/jjw_workbench.sh` (workbench)

### B. Foray Protocol — Fundus constants (rbm values)

**Fundus constants** (use these for `jjx_bind`):
- Default reldir: `projects/rbm_alpha_recipemuster`
- BURN alias: use the alias from the target's BUK Regime Node profile (e.g., `winhost-wsl`, `winhost-cyg`)

Hosts in play for this repo (full operator machine registry: §I Test Environments below):
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
- **Close on a decision, not a menu.** End the mount on the single
  recommended call, stated as a decision you will execute unless the
  operator redirects — "I'll do X; say go or steer me" — never on a fork
  of expansions ("expand A? expand B? or proceed?"). The proceed-gate is
  one beat, not a multiple-choice. A docket-flagged mechanical choice is
  *made* in the close, with its one-line reason, and the operator
  overrides if they disagree — it is not handed back as a question.
  Offering to expand reasoning is a trailing half-clause at most, never
  the load-bearing close. A mount that ends on a question-mark menu has
  re-imported the Molehill it was supposed to avoid.
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
context reminder before its content; the operator paces the advance — the
cry is "gee" (the driver's go-command; "next" honored too) —
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

### E. Gallops schema changes are gated on a reprieve episode (JJK crate)

Changing the on-disk gallops shape — adding, removing, renaming, or retyping a
serialized field, or changing how one serializes — is a schema change, and a schema
change **must** register a reprieve episode. Skipping it silently breaks reading
of every older-format gallops on disk. These changes are rare and easy to get wrong;
do not improvise. Before touching the schema, read `jjdz_reprieve` in JJS0 and
follow its "Registering a new episode" procedure.

The full doctrine — the mechanism, the multi-install convergence model, how a
schema change is delivered, and how the clones converge — lives in the JJS0
subdocument `JJSCRP-reprieve.adoc` (the `jjdz_reprieve` quoin). The agent rule it
imposes: a schema change is delivered source-only on a date-and-identity branch
(per §F) and never commits a gallops conversion — the reprieve makes the new
binary tolerant of the old store, so the conversion is deferred to the single
coordinated convergence that forces and commits it across every install at once.
The branch is the quarantine; the episode is what makes holding it safe.

`JJr_a7c` (any `JJr_` token) in the crate is the cited reprieve rivet —
`grep JJr_a7c` to that quoin for the rationale. The code it guards (the V3→V4
write-forward, frozen `jjrt_v3_types.rs`) is deliberately temporary, removed only
under the episode's demolition condition: do not clean it up or re-explain it beside
the marker. RBK's `RBr_` conduct rule, JJK-native.

**Slate-time schema-impact check.** When slating a pace that touches the JJK crate,
check whether it changes the gallops on-disk schema (add, remove, rename, or retype a
serialized field, or change how one serializes). If it does, the docket must point the
mount agent at §E, §F, and `JJSCRP` (`jjdz_reprieve`) for the branch-delivery and the
reprieve-episode determination.

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

### G. Diagram review via unfurl (rbm-specific)

Companion to the public Unfurl Protocol in `claude-jjk-core.md`: in rbm a
committed diagram is reviewed by *unfurling* it onto the viewer and working it
by hand — never via an RB-side tool (the viewing is a generic JJK act; RB is
merely the first image producer). Unfurl the light `.svg` together with its
`-dark` sibling and the pair travels as one frame; then in the viewer `f` fits,
scroll/drag zoom and pan, and `d`/`l` toggle the variant at the held zoom+pan
(the backing flips with it). The viewer cannot derive dark, so both paths are
passed — the `rbdgX_name.svg` / `rbdgX_name-dark.svg` convention and its
`zrbtdrc_darken_svg` recolor are documented at the RBDG acronym entry
(`claude-rbk-acronyms.md`), referenced here, not restated. The toggle is an
in-tool proof of exactly what the README `<picture>` blocks render per theme.

### H. Notch before test (Rust test/commit discipline)

Operator standing preference, no carve-out — homed here because *notch* is
JJK-native; the rule governs every Rust test gate across BOTH build pipelines
(the VOW kits and RBK's theurge), so it has no single non-JJK owner.

**Always notch before you test.** Commit (notch) every pending change before
running any test — `vow-t`, `rbw-tt`, a suite (`rbw-ts.*`), or a fixture/case
(`rbw-tf`/`rbw-tc`) — so each result maps to a committed (if not yet pushed)
commit, and the interim notches stand as the durable record of what passed or
failed. Never run tests against a dirty tree; if `git status` isn't clean, notch
first.

### I. Test Environments

Operator-specific test machines reachable from this station. Cross-cutting
operator infrastructure — both BUK garrison/caparison and JJK foray/fundus reach
these — homed here because JJK foray/fundus is the dominant consumer; §B's
fundus-host list points here for the full registry.

- **bujn-winpc** — Windows host, tailnet hostname `rocket`. Formal BURN profile
  at `rbmm_moorings/rbmn_nodes/bujn-winpc/` for BUK caparison/garrison/invigilate
  work under heat ₣A-. **Consolidated access reference + live account state:**
  `Memos/memo-20260516-windows-headless-account-anatomy.md`.
  - Admin SSH: `tt/buw-jpS bujn-winpc <cmd>` (as `bhyslop`; cmd.exe default shell,
    so prepend `powershell -Command` / `bash -c` as the task needs).
  - Formal workload: `tt/buw-jws bujn-winpc` (as `bujuw_user`; garrison routes to
    WSL `rbtww-main`). Owned by the garrison ceremony, not hand-edits.
  - Ad-hoc test accounts (pubkey-only, independent of the formal garrison — safe
    scratch). Repo cloned at `~/projects/rbm_alpha_recipemuster`:
    - `ssh brad@rocket` — interactive Cygwin login shell (human use; ignores a passed command).
    - `ssh cygwin@rocket "<cmd>"` (one-shot) or `ssh -t cygwin@rocket` (interactive) — Cygwin, full shell semantics.
    - `ssh wsl@rocket "<cmd>"` (one-shot) or `ssh -t wsl@rocket` (interactive) — WSL Ubuntu 24.04 as root; **Docker daemon live — container tests run here**.
  - Legacy LAN aliases `winhost-{wsl,cyg,ps}` (192.168.86.27) are currently
    unreachable; use the `rocket` tailnet paths above.
- **cerebro** — Linux test host (Ubuntu 24.04). Direct access: `ssh cerebro`
  (user `bhyslop`, key `~/.ssh/id_ed25519`). Also the remote fundus for JJK
  scenario tests: tabtargets `tt/jjw-tfP2.ProvisionPhase2.cerebro.sh`,
  `tt/jjw-tfs.TestFundusScenario.cerebro.sh` (tests marked `#[ignore]`,
  `--ignored` required; fundus accounts must be provisioned on cerebro first).
- **localhost** — local fundus for JJK scenario tests via `jjfu-*` ssh aliases
  (`jjfu-full`, `jjfu-nogit`, `jjfu-nokey`, `jjfu-norepo`).
