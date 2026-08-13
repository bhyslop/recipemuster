# Claude Code Project Memory

## Directory Permissions
Full read and edit access is pre-approved for all files in:
- `Tools/`

Permission is not durability. `Tools/buk/` is a parcel drop-zone — see the next
section — and an edit made there is destroyed at the next install.

## Managed Kit Directories

`Tools/buk/` is a consumed copy, not a source. BUK's primary home is the
`jjqa_app` repository; this tree receives it as a minted parcel and installs it
by whole-directory replacement — the install verb (`emplace`) deletes the kit
directory outright, then recopies it from the parcel. Nothing standing in it
survives.

So an edit to any file beneath `Tools/buk/` is destroyed work, however small and
however correct. There is no merge and no conflict: the file is deleted, a fresh
copy from `jjqa_app` is written in its place, and the change is gone — no diff,
no warning, no trace that it ever stood. This has nearly happened here. A ruled,
censused, reveille-verified change to `Tools/buk/buv_validation.sh`
(`d6390f148`) was made in this tree after it stopped being BUK's home, and
survived only because it was hand-carried upstream ahead of the first install.
That rescue was a one-time act and has since retired; a second such edit has
nothing catching it.

A buk file is therefore fixed in `jjqa_app` and arrives here by parcel. That is
the only path in.

Which directories are drop-zones is declared by `BURC_MANAGED_KITS` in
`rbmm_moorings/burc.env` — today, `buk` alone. Every other directory under
`Tools/` is this repo's own and is edited here in the ordinary way. The release
and install procedures themselves are specified in
`jjqs_studbook/specs/vok/VOSO-distribution.adoc` (`vosor_release`,
`vosoi_install`); this tree carries no copy of them, by design.

## File Acronym Mappings

Per-kit acronym mappings live in each kit's context file (loaded via `@` includes below).
- RBK: `@Tools/rbk/claude-rbk-acronyms.md`
- BUK: `@Tools/buk/claude-buk-acronyms.md`
- APCK: `@Tools/apck/claude-apck-acronyms.md`

## Retired Memos

A memo whose work is fully dispositioned (every concern resolved into a pace, an
itch/RBSHR entry, or an explicit decline) moves to `jjqs_studbook/retired/memos/`
with its basename unchanged. A memo path under `jjqs_studbook/` that no longer
resolves has retired — look for the same basename under
`jjqs_studbook/retired/memos/`. Retired memos are historical record: read them
freely, never resurrect work from them without operator direction.

Memos are provenance, never authority: if a fact must still be true after the
memo retires, it needs a spec home. The temptation to home durable knowledge in
a memo is itself the signal that formal specification is due.

## Working Preferences
- When user mentions an acronym, immediately navigate to the corresponding file
- Assume full edit permissions under `Tools/`, bounded by the drop-zone limit
  above: a change beneath `Tools/buk/` is made in `jjqa_app`, never here
- For bash scripts, prefer functional programming style with clear error handling
- For .adoc files, maintain consistent AsciiDoc formatting
- For .claudex files, preserve the specific format requirements

### Heredoc Delimiter Selection

When generating heredocs for stdin content, the delimiter must not appear alone on any line within the content.

- **Check content first**: If content includes `EOF` (e.g., code examples showing heredoc patterns), use a different delimiter
- **Safe alternatives**: `SPEC`, `CONTENT`, `DOC`, `PACESPEC`, `SLASHCMD`
- **Pattern**: `cat <<'DELIM' | command` (quoted delimiter prevents variable expansion)

### AsciiDoc Linked Terms
When working with .adoc files using MCM patterns:
- **Linked Term**: Concept with three parts:
  - Attribute reference: `:prefix_snake_case:` (mapping section)
  - Replacement text: `<<anchor,Display Text>>` (what readers see)
  - Definition: `[[anchor]] {attribute}:: Definition text` (meaning)
- Definitions may be grouped in lists or dispersed through document
- Maintain consistent prefix categories (e.g., `mcm_`, `rbw_`)
- Use snake_case for anchors, match attribute to anchor

### Rust Build Discipline

Always use the tabtarget, never raw cargo commands.

**Theurge** (rbk's own test infrastructure — dispatches through the unified rbw workbench):
- `tt/rbw-tb.Build.sh` — build theurge crate
- `tt/rbw-tt.Test.sh` — run theurge unit tests

### Test Execution

**Test suites** group fixtures by dependency **stratum** (reveille/picket/bivouac; echelon = their union). The cosmology — the wrapper(inner) model, the three strata, and the freehold/leasehold substrate — is spec-homed in `RBSTC-theurge_cosmology.adoc`. Suite membership is owned by the hand-written `RBTDRA_SUITES` literal registry (`Tools/rbk/rbtd/src/rbtdra_almanac.rs`), the authoritative source; the table below summarizes strata, never member lists (which drift). Run the broadest applicable suite:

| Suite | Tabtarget | Dependencies | Stratum |
|-------|-----------|-------------|---------|
| `reveille` | `tt/rbw-ts.TestSuite.reveille.sh` | None | Credless base — no external dependency |
| `picket` | `tt/rbw-ts.TestSuite.picket.sh` | GCP credentials | reveille + GCP-credentialed fixtures |
| `bivouac` | `tt/rbw-ts.TestSuite.bivouac.sh` | Container runtime | reveille + container-runtime crucible fixtures |
| `echelon` | `tt/rbw-ts.TestSuite.echelon.sh` | All of the above | reveille ∪ picket ∪ bivouac |

`regime-poison` is the in-universe negative-validation fixture (real validate verbs against real regimes, one field corrupted via the regime-poison tweak, asserting a specific band code). It rides above reveille — reveille reserves the tweak slot for the credless guard — so a regime/validation change runs reveille (positives) plus this fixture (negatives) via `tt/rbw-tf.FixtureRun.sh regime-poison`. Its operator-local cases (station/oauth/auth/node/privilege) self-skip when the regime is not configured on the machine.

**Release/probe suites** — ladders distinguished by project-churn × crucible ×
network posture, not dependency tier:

| Suite | Tabtarget | Precondition | What it covers |
|-------|-----------|-------------|----------------|
| `gauntlet` | `tt/rbw-ts.TestSuite.gauntlet.sh` | None (levies fresh projects) | Release-qualification ladder: marshal-zero state → depot-lifecycle → freehold-establish → onboarding-sequence → reveille fixtures → crucibles |
| `skirmish` | `tt/rbw-ts.TestSuite.skirmish.sh` | Freehold depot already levied | Mini-gauntlet: depot→build→crucible chain without project churn |
| `dogfight` | `tt/rbw-ts.TestSuite.dogfight.sh` | Freehold depot already levied | Cloud-build viability probe: ordain → summon → run, no crucible |
| `siege` | `tt/rbw-ts.TestSuite.siege.sh` | None (fully local) | Tadmor self-contained: kludge both vessels + security cases |
| `blockade` | `tt/rbw-ts.TestSuite.blockade.sh` | Depot levied + moriah hallmark ordained | Airgap moriah crucible with credential self-heal |
| `parley` | `tt/rbw-ts.TestSuite.parley.sh` | Freehold depot levied + subject brevetted onto retriever (standing terrier) | Positive federation-admission round-trip: unseat → restore-brevet retriever, asserting via rehearse's manor roll that the muniment stands → vanishes → stands |

**After code changes**, run the appropriate tier:
- Regime/validation changes → `reveille` + `regime-poison` (`tt/rbw-tf.FixtureRun.sh regime-poison`)
- Foundry/credential changes → `picket`
- Bottle/sentry/network changes → `bivouac`
- Pre-release or decomposition sweep → `echelon`

**Single fixture**: `tt/rbw-tf.FixtureRun.sh <name>` (e.g., `tadmor`, `enrollment-validation`, `regime-smoke`)

**Single case**: `tt/rbw-tc.FixtureCase.sh <fixture> [case-name]` — run one case against an already-charged crucible (no charge/quench). Omit case name to list all cases for the fixture; omit fixture to list all fixtures. Workflow for crucible debugging: charge via `tt/rbw-cC.Charge.{nameplate}.sh`, run individual cases, quench via `tt/rbw-cQ.Quench.{nameplate}.sh` when done.

**BUK self-test**: `tt/buw-st.BukSelfTest.sh` — exercises the BUK test framework and core modules: kick-tires, band-survival, bure-tweak, burx-exchange, fact-chaining, buh-link, dispatch-color, buym-yelp (8 fixtures, 58 cases)

**Sequential only**: Never run fixtures in parallel — they share regime state and container namespaces.

<!-- Universal minting doctrine is CMK-homed, and CMK homes in jjqa_app: this
     repo no longer carries the kit, so the doctrine is not loaded here. What
     stays is this repo's own Project Prefix Registry, which the doctrine's
     Rule 1 defers to the host project for. -->

## Project Prefix Registry (rbm-local)

| Prefix | Project |
|--------|---------|
| `rb` | Recipe Bottle |
| `bu` | BUK (Bash Utilities Kit) |
| `jj` | Job Jockey |
| `pb` | Paneboard |
| `mcm`, `axl` | CMK (Concept Model Kit) |
| `crg` | Config Regime |
| `wrs` | Ward Realm Substrate |
| `hm` | HMK (Hard-state Machine Kit) |
| `lmci` | LMCI (Language Model Console Integration) |
| `apc` | APCK (Ann's PHI Clipbuddy Kit) |
| `vsl` | VSLK (Visual SlickEdit Local Kit) |

For expanded prefix trees within each project, see **File Acronym Mappings** above.

## Common Workflows
1. **Bash Development**: Start with relevant utility (BUC/BUD/BUT/BUV/BUW), check dependencies
2. **Requirements Writing**: Open spec file, review related documents in same directory

## Design Principles

### Load-Bearing Complexity

An element is **load-bearing** when its removal would create a gap between intent and behavior. The litmus for any new pattern, extraction, or structural choice: "Does this element earn its existence?" If not, it doesn't belong.

Concept home: MCM `mcm_load_bearing` — the headwater the Antipatterns catalog instantiates. Domain forms: BCG (Zeroes Theory, Interface Contamination), RCG (Constant and Constructor Discipline), ACG (Allodial Discipline).

### Zeroes Theory

Every tolerance, alias, fallback, or alternative path multiplies the enumerated state space. The litmus: **"How many zeroes did this choice add to the enumerated state space?"** If the answer isn't zero, it needs explicit justification. The multiplication runs along several axes — the input forms accepted at one moment, the formats tolerated across time, and the interior representation chosen for data.

Concept home: BCG **Zeroes Theory** — the built form, where each axis carries its own discipline. Instantiates MCM `mcm_load_bearing`: the litmus is the load-bearing question sharpened to state space.

<!-- The wake-up greeting (never distributed; hand-maintained outside the
     managed block). The file is gitignored and billet-local, materialized at
     billet preparation by the saddle and tier-matched from the studbook; if
     the include dangles, run a fresh saddle. The partnership rules of
     engagement it used to lead into are CMK-homed and CMK homes in jjqa_app,
     so they are not loaded here. -->
@.claude/claude-salutation.md

<!-- Distributable-kit guidance: hand-maintained @-include block, and the
     curation record for it — each line is a kit guidance file curated for
     launch-time load (kits ship more claude-*.md than belong here; on-demand
     files stay out). Which files load is decided here; their content is edited
     in neither this file nor the targets themselves — these targets stand in
     the Tools/buk parcel drop-zone (see Managed Kit Directories above), so a
     content change is made in jjqa_app and arrives by parcel. -->
@Tools/buk/claude-buk-core.md
@Tools/buk/claude-buk-acronyms.md

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides
- Public project page: https://scaleinv.github.io/recipebottle

@Tools/rbk/claude-rbk-core.md
@Tools/rbk/claude-rbk-acronyms.md

<!-- rbm-only veiled guidance (never distributed); hand-maintained outside the block -->
@Tools/rbk/vov_veiled/claude-rbk-veiled.md
@Tools/rbk/vov_veiled/claude-rbk-veiled-acronyms.md

@Tools/rbk/claude-rbk-conduct.md

@Tools/rbk/claude-rbk-tabtarget-context.md

For theurge/ifrit crucible testing work, read `Tools/rbk/claude-rbk-theurge-ifrit-context.md` — covers the iteration loop (kludge, charge, test, ordain), architecture of the two Rust binaries, and how to add new security test cases.

@Tools/apck/claude-apck-context.md
@Tools/apck/claude-apck-acronyms.md
