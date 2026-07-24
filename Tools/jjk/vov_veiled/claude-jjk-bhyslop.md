## JJK — bhyslop project specifics (veiled, not distributed)

This file holds the rbm-only scenery carved out of the public JJK guidance
(`Tools/jjk/claude-jjk-core.md`). It stays in this repo and is never
distributed to consumer projects.

### A. File Acronym Mappings — JJK Subdirectory (`Tools/jjk/`)

- **JJS0** → `jjk/vov_veiled/JJS0_JobJockeySpec.adoc` (Job Jockey specification - main file)
> **JJS\* sub-sheaf entries are intentionally not listed here.** The Job Jockey spec sheaves (aspirant `JJSA*`, command `JJSC*`, revision-control `JJSV*`, and the rest) load on demand, not always. Discipline: to reach any sheaf, read the SpecTop **JJS0** (`jjk/vov_veiled/JJS0_JobJockeySpec.adoc`) FIRST — it is the required entry point and indexes them; the sheaves live beside it as `jjk/vov_veiled/JJS*.adoc`.
- **JJW**  → `jjk/jjw_workbench.sh` (workbench)

### B. Foray Protocol — Fundus constants (rbm values)

**Fundus constants** (use these for `jjx_bind`):
- Default reldir: `projects/rbm_alpha_recipemuster`
- BURN alias: use the alias from the target's BUK Regime Node profile (e.g., `winhost-wsl`, `winhost-cyg`)

Hosts in play for this repo (full operator machine registry: §I Test Environments below):
- `winhost-wsl` / `winhost-cyg` — Windows host transports (WSL and Cygwin).
- `beast` — Windows 11 Pro host, Cygwin-based test box with Linux-shaped ssh
  (`ssh beast` — no cmd.exe layer; see §I).
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

**The trot** — a chunking pattern for delivering a dense, multi-part
deliverable to a tired reader (as horses are trotted out for inspection: one
animal at a time, in order, each set down clearly before the next). The
deliverable is broken into a pre-counted sequence of small chunks ("chunk 2 of
6"), and each chunk opens with a plain-language context reminder before its
content. **The default trot lands every chunk in one message** — the counting
and the per-chunk re-grounding are the whole value; the waiting is not.
Serializing across turns is a separate *gait*, opt-in only (below), never the
trot's nature. A chunk often leans on work the operator did not author — a
prior instance's, or a session they convened but did not line-read; there the
context reminder grounds the referenced artifact *from scratch* rather than
invoking it by name, because the operator's strategy of building across editions
holds them, by design, at a remove from the words, so a name that feels shared to
the trotter ("the normative-register clause," "the blessed form") frequently
names nothing they have read. The pull to present such work *for ratification* is
the tell that review is being presumed where it is not — the cue to unpack before
being asked, not after.

**The paced gait** — the opt-in, turn-by-turn form: the chunks serialize across
turns, each waiting on the operator's go-cry "gee" ("next" honored too) before
the next is set down, and the walk absorbs detours — a stray thought mid-trot
spawns its own discussion, then resumes where it stood. Enter it **only** on the
operator's request ("trot this slowly") or a mid-stream fatigue report — never
on the agent's own read that a response "looks dense." The default single-message
trot already avoids a stun; the paced gait exists for when the operator, not the
agent, wants to hold the rhythm.

Distinct from gradient delivery above, which layers depth on *one* deliverable
for the operator to pull: a trot chunks a *multi-part* deliverable, re-grounding
at every step. Use it both directions — the operator can ask to "trot the
findings," and the agent should offer a trot (the single-message default) when
it sees a dense multi-part response coming.

### E. Gallops schema changes are gated on a reprieve episode (JJK crate)

Changing the on-disk gallops shape — adding, removing, renaming, or retyping a
serialized field, or changing how one serializes — is a schema change, and a schema
change **must** register a reprieve episode. Skipping it silently breaks reading
of every older-format gallops on disk. These changes are rare and easy to get wrong;
do not improvise. Before touching the schema, read `jjdz_reprieve` in JJS0 and
follow its "Registering a new episode" procedure.

**Additive carve-out.** A new field riding `#[serde(default,
skip_serializing_if = …)]` — old stores read natively AND re-serialize
byte-identical — registers NO episode (JJSCRP's additive clause; the
bridle-revival determination). Byte-identity is required, not just
readability. §F branch delivery still applies.

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

Companion to the Unfurl directions in `Tools/jjk/claude-jjk-images.md`: in rbm a
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
  - Admin SSH: `ssh -i ~/.ssh/id_ed25519_winpc-admin bhyslop@rocket "<cmd>"` (cmd.exe
    default shell, so prepend `powershell -Command` / `bash -c` as the task needs).
    The former `tt/buw-jpS` tabtarget was removed when the jurisdiction apparatus was
    veiled; the bare key in `administrators_authorized_keys` is the live path
    (verified 260712).
  - Runtime posture (surveyed 260712): Windows Home — no RDP server, headless — so
    Docker Desktop cannot run here (its GUI needs a desktop session; the engine boots,
    then dies with the GUI — JJSAM-mews Palisade facts). The WSL distro's native
    dockerd is the standing container runtime: the ₣A--deferred configuration, serving
    the `wsl@rocket` container tests below. Never start both daemons concurrently.
  - Formal workload: `tt/buw-jws bujn-winpc` (as `bujuw_user`; garrison routes to
    WSL `rbtww-main`). Owned by the garrison ceremony, not hand-edits.
  - Ad-hoc test accounts (pubkey-only, independent of the formal garrison — safe
    scratch). Repo cloned at `~/projects/rbm_alpha_recipemuster`:
    - `ssh brad@rocket` — interactive Cygwin login shell (human use; ignores a passed command).
    - `ssh cygwin@rocket "<cmd>"` (one-shot) or `ssh -t cygwin@rocket` (interactive) — Cygwin, full shell semantics.
    - `ssh wsl@rocket "<cmd>"` (one-shot) or `ssh -t wsl@rocket` (interactive) — WSL Ubuntu 24.04 as root; **Docker daemon live — container tests run here**.
  - Legacy LAN aliases `winhost-{wsl,cyg,ps}` (192.168.86.27) are currently
    unreachable; use the `rocket` tailnet paths above.
- **beast** — Windows 11 Pro 24H2 desktop (i7-6700K, 64 GB), tailnet hostname
  `bhyslop-asrock-beast`. **The Cygwin-based Docker-Desktop test host** rocket
  structurally cannot be: the operator's persistent RDP logon keeps the DD
  engine alive across disconnect. Provisioned 260712 (heat ₣Bs; as-executed
  record + replay authority: `Memos/memo-20260712-beast-host-standup.md`).
  - **Access: `ssh beast`** (curia ssh-config alias; user `bhyslop`, the
    winpc-admin key). **Linux-shaped ssh, UNLIKE rocket**: registry
    `DefaultShell` is Cygwin bash with `-lc` one-shots, so there is NO cmd.exe
    layer — pipes, `;`, `$`-expansion, scp, and stdin cat-tricks all behave as
    against a Linux host. None of rocket's cmd.exe armor (EncodedCommand,
    pipe-avoidance) is needed on beast; it remains needed on rocket.
  - Substrate: Cygwin 3.6.9 at `C:\cygwin64` (rocket-matched package set + jq,
    python3), rustup stable windows-gnu, repo at `~/projects/rbm_alpha_recipemuster`
    (Cygwin HOME = Windows profile `C:\Users\bhyslop`), station-files sibling
    with secrets + burs.env (tincture `bhb`).
  - Runtime: Docker Desktop 4.81 (WSL2 backend, autostart, containerd
    snapshotter default — a delta from rocket's native WSL dockerd). Sole WSL
    distro is DD's own `docker-desktop`; never add a user distro (one-daemon
    cinch). **DD lifecycle acts (install/uninstall/settings) are console-bound
    — never script them over ssh** (memo §7).
  - Proven 260712: reveille green 145/145 over `ssh beast`; picket-capable
    (payor + sitting installed; credential-readiness and access-probe green).
- **mimic-bth-intel** — Windows machine, RDP-reachable, otherwise unprovisioned
  (as of 260712). Destined to replay the beast standup memo as its
  repeatability proof (heat ₣Bs plan).
- **cerebro** — Linux test host (Ubuntu 24.04). Direct access: `ssh cerebro`
  (user `bhyslop`, key `~/.ssh/id_ed25519`). Also the remote fundus for JJK
  scenario tests: tabtargets `tt/jjw-tfP2.ProvisionPhase2.cerebro.sh`,
  `tt/jjw-tfs.TestFundusScenario.cerebro.sh` (tests marked `#[ignore]`,
  `--ignored` required; fundus accounts must be provisioned on cerebro first).
- **brads-macbook-air** — macOS 26.5 laptop, Apple Silicon (arm64), local hostname
  `mac.lan`. **The registry's only arm64 host** — anything container-shaped built or
  run here is arm64, unlike every x86_64 machine above.
  - Access: `ssh brads-macbook-air` (user `bhyslop`, key `~/.ssh/id_ed25519`).
    **No ssh-config `Host` entry** — unlike beast/cerebro, the bare tailnet name
    resolves directly. Unix shell throughout: no transport armor of any kind.
  - Reachable but **unprovisioned** (surveyed 260713): no repo clone (no `~/projects/`),
    no rust toolchain, no container runtime (neither docker nor podman). System `git`,
    `jq`, and `python3` are present. Nothing here has been proven against a suite —
    stand it up before treating it as a test host.
- **localhost** — local fundus for JJK scenario tests via `jjfu-*` ssh aliases
  (`jjfu-full`, `jjfu-nogit`, `jjfu-nokey`, `jjfu-norepo`).

### J. Infield Resident Brands (`jjq_` / `jjy_`)

The dirname brands for JJ-owned infield residents. Concept home: `jjdw_yard` (JJSVC
cosmology) — this is the quick-lookup pointer, not the definition.

- **`jjq_`** — infield **directory** residents (has children; names no bare dir):
  - `jjqa_app` — the kit repo (engine + provisioned context; was `jjy_app`).
  - `jjqs_studbook` — the record repo (was `jjy_studbook`).
  - `jjqb_{catchword}_{coronet}` — a pace **billet** (per-pace worktree dir); `jjqb_{catchword}_{firemark}` a groom billet — one signet, the identity behind the serial typed by length.
  - `jjqd_scratch` — the dispatch-scratch container: per-billet BUK state (BURV output/temp/log roots) and session-scoped MCP config, keyed by billet dirname beneath it. Deliberately outside the `jjqb_` glob so the muck sweep can never match it.
- **`jjy_`** — infield **launcher scripts** (stiles): `jjy_saddle`, `jjy_lunge` (flat).

Two letters, not one: terminal-exclusivity forbids `jjy_`'s flat script names coexisting with a
sub-lettered billet container, and `jjq` was one of the last two free `jj`-letters (260707
census). The split is by resident kind — directories vs scripts.

**Billet naming: two surfaces, two owners.** The billet's **git branch** wears the *livery
badge* — `jjls_pace/{coronet}`, JJ's mark in a ref store JJ does not own. Its **dirname** is
`jjqb_{catchword}_{identity}`, in JJ's own yard: the leading serial is the catchword the
birth's dispatch record allocated, so a plain listing sorts by birth and concurrent groom
billets of one heat stay distinct. `git worktree list` ties branch to dirname, and it — never
the dirname — is what a dispatch rediscovers a standing billet by. The serial is a label and
never an identity: readers step over it to the tail token and never parse it. The muck sweep
globs `jjqb_*` — a positive match that structurally excludes the `jjqa_`/`jjqs_` repos it must
never reap. Constraint owed upstream: the pace-id re-gestalt must keep the coronet charset
git-ref- and dirname-safe (no leading `-`).

### K. Operator-only verb — jog (naming census)

**jog** is an operator-only Upper API verb. It drives the matricula census, which
never ships (VOr_q4f), so it homes here in the veiled context and never in the
delivered Quick Verbs table (`claude-jjk-core.md`). It is otherwise an ordinary
verb — same Upper/Lower register gap as every other.

**jog → `jjx_sift`** (no params; no officium — it reads the source tree, not the
gallops). Inspect the corpus of minted names for soundness: run the naming census
over the tracked source tree and surface every violation — exact collision and
terminal-exclusivity — plus the estray section. Reach for it at the grep-gate step
of a mint, and any time you want the corpus appraised.

- **Advisory where fallible (VOr_m7w):** the exact-collision and
  terminal-exclusivity presentments are authoritative; the sprue/rivet findings
  are advisory, never assertions — never gate a mint on them.
- **Degenerate for now:** until vestures land, every ours-cipher token reports as
  an estray. Expected — `jog` exercises the tool as it deepens (use-before-implement).
- **Absent binary:** the engine spawns the standalone census binary as a separate
  process; where it is absent the command simply fails. If `jjx_sift` reports it
  missing, build it with `tt/vow-mb.MatriculaBuild.sh`.
- Concept home: JJS0 "Census Operations" (`jjdo_sift`) and the `jjsuv_jog` verb
  definition; the tool itself is VOSMM-entity.adoc.
