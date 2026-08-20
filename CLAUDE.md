# Recipe Bottle (`rb`) — sire context

The kit context this tree contributes to a kraal session's composed context, and
its gather manifest. No session sits in this tree: it is prepared as a billet
standing beside the session's own outspan, and the outspan — which carries no
repository — is where that session's shell starts. Every path below is therefore
repo-relative and names a file in the billet, reached by joining the billet root
the composed frame above names.

Tabtarget lines are written the same way, `tt/rbw-…`, and are run by their
absolute path from wherever the shell happens to stand: `tt/z-launcher.sh`
normalizes the working directory to the repo root itself, so a tabtarget invoked
from outside the tree behaves exactly as one invoked inside it. Nothing here
needs a `cd`, and the log paths a dispatch announces are the ones to read back —
never a path rebuilt from a remembered layout.

## Current Context
- Primary focus: Recipe Bottle infrastructure and tooling
- Architecture: Bash-based CLI tools with Google Cloud integration
- Documentation format: AsciiDoc (.adoc) for specs, Markdown (.md) for guides
- Public project page: https://scaleinv.github.io/recipebottle

## Managed Kit Directories

`Tools/buk/` is a consumed copy, not a source. BUK's primary home is the JJ
application clone, `../jjqa_app` — a peer of the outspan, not a tree inside this
one; this tree receives BUK as a minted parcel and installs it by
whole-directory replacement — the install verb (`emplace`) deletes the kit
directory outright, then recopies it from the parcel. Nothing standing in it
survives.

So an edit to any file beneath `Tools/buk/` is destroyed work, however small and
however correct. There is no merge and no conflict: the file is deleted, a fresh
copy from `../jjqa_app` is written in its place, and the change is gone — no
diff, no warning, no trace that it ever stood. This has nearly happened here. A
ruled, censused, reveille-verified change to `Tools/buk/buv_validation.sh`
(`d6390f148`) was made in this tree after it stopped being BUK's home, and
survived only because it was hand-carried upstream ahead of the first install.
That rescue was a one-time act and has since retired; a second such edit has
nothing catching it.

A buk file is therefore fixed in `../jjqa_app` and arrives here by parcel. That
is the only path in — including a buk file's own carried prose, so a correction
owed to `Tools/buk/claude-buk-core.md` is made at the BUK home and rides the
next parcel.

Which directories are drop-zones is declared by `BURC_MANAGED_KITS` in
`rbmm_moorings/burc.env` — today, `buk` alone. Every other directory under
`Tools/` is this repo's own and is edited in the billet in the ordinary way. The
release and install procedures themselves are specified in
`jjqs_studbook/specs/vok/VOSO-distribution.adoc` (`vosor_release`,
`vosoi_install`); this tree carries no copy of them, by design.

## File Acronym Mappings

Each kit's acronym rows ride that kit's core context file, annotated with the
per-row descriptions and family topology — RBK's in
`Tools/rbk/claude-rbk-core.md`, BUK's in `Tools/buk/claude-buk-core.md`, and the
veiled half's in `Tools/rbk/vov_veiled/claude-rbk-veiled.md`. Those cores load
below.

Beside each core stands a bare index (`claude-{kit}-acronyms.md`) restating the
same rows without their annotations. Those indexes are not loaded — see the
curation record under **Kit context** — and each core is a strict superset of
its own index, so nothing is lost. They stand for the lints that read them as
rosters, and for a reader who wants the bare table.

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
- When an acronym is mentioned, navigate to the corresponding file
- Everything under `Tools/` is this repo's own and is edited in the billet,
  bounded by the drop-zone limit above: a change beneath `Tools/buk/` is made in
  `../jjqa_app`, never here
- For bash scripts, prefer functional programming style with clear error handling
- For .adoc files, maintain consistent AsciiDoc formatting

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

<!-- Universal minting doctrine is CMK-homed, and CMK homes in ../jjqa_app: this
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
| `vsl` | VSLK (Visual SlickEdit Local Kit) |

For expanded prefix trees within each project, see **File Acronym Mappings** above.

## Design Principles

### Load-Bearing Complexity

An element is **load-bearing** when its removal would create a gap between intent and behavior. The litmus for any new pattern, extraction, or structural choice: "Does this element earn its existence?" If not, it doesn't belong.

Concept home: MCM `mcm_load_bearing` — the headwater the Antipatterns catalog instantiates. Domain forms: BCG (Zeroes Theory, Interface Contamination), RCG (Constant and Constructor Discipline), ACG (Allodial Discipline).

### Zeroes Theory

Every tolerance, alias, fallback, or alternative path multiplies the enumerated state space. The litmus: **"How many zeroes did this choice add to the enumerated state space?"** If the answer isn't zero, it needs explicit justification. The multiplication runs along several axes — the input forms accepted at one moment, the formats tolerated across time, and the interior representation chosen for data.

Concept home: BCG **Zeroes Theory** — the built form, where each axis carries its own discipline. Instantiates MCM `mcm_load_bearing`: the litmus is the load-bearing question sharpened to state space.

## Salutation

<!-- The wake-up greeting (never distributed; hand-maintained outside the
     managed block). The file is gitignored and billet-local, materialized at
     billet preparation by the saddle and tier-matched from the studbook; if
     the include dangles, run a fresh saddle. The partnership rules of
     engagement it used to lead into are CMK-homed and CMK homes in ../jjqa_app,
     so they are not loaded here. -->
@.claude/claude-salutation.md

## Kit context

<!-- THE GATHER MANIFEST, and the curation record for it. Every `@` line below
     is a whole file spliced into the composed context of every session this
     tree is dispatched to, so each line is standing launch-time weight. Which
     files load is decided here; their content is edited in the files
     themselves, and a target standing in the Tools/buk parcel drop-zone (see
     Managed Kit Directories above) is edited at the BUK home and arrives by
     parcel.

     Three rulings govern what stands here, each applied to every kit alike:

     1. ANNOTATED CORE IN, BARE INDEX OUT. Each kit ships a core context file
        carrying its acronym rows WITH the per-row descriptions and family
        topology, and beside it a bare index restating the same rows without
        them. Every token in each index is present in its core with the same
        path, and the rbk core carries eight tokens its index lacks, so each
        index is a strict subset and adds no fact at launch time. The cores
        load; the indexes stand as pull doors and as lint rosters.

     2. A GENERATED LOOKUP TABLE IS A PULL DOOR, NOT LAUNCH-TIME WEIGHT. The
        tabtarget command reference is regenerated by the build from the zipper
        registry, is the largest single file this manifest could carry, and is
        consulted a handful of rows at a time — while `ls tt/` already lists the
        same set. It stands named under Pull doors below.

     3. AN INCLUDE SITS UNDER THE HEADING THAT CLAIMS IT. The composition
        demotes each gathered file to nest under the heading in effect where its
        line sits, so placement decides both the composed outline and how much
        of a gathered file's own outline survives the depth budget. Every
        include therefore stands directly under this H2, never trailing whatever
        section happened to precede it. -->

<!-- Distributable kit guidance. -->
@Tools/buk/claude-buk-core.md

@Tools/rbk/claude-rbk-core.md

@Tools/rbk/claude-rbk-conduct.md

<!-- rbm-only veiled guidance (never distributed). -->
@Tools/rbk/vov_veiled/claude-rbk-veiled.md

## Pull doors

Named, not loaded — read when the work calls for one.

- `Tools/rbk/claude-rbk-tabtarget-context.md` — the full tabtarget command
  reference: every colophon, its folio channel, and its purpose. Build-generated
  from the zipper registry; `ls tt/` lists the same set by filename.
- `Tools/rbk/claude-rbk-theurge-ifrit-context.md` — theurge/ifrit crucible
  testing: the iteration loop (kludge, charge, test, ordain), the architecture of
  the two Rust binaries, and how to add new security test cases.
- `Tools/rbk/claude-rbk-acronyms.md`, `Tools/buk/claude-buk-acronyms.md`,
  `Tools/rbk/vov_veiled/claude-rbk-veiled-acronyms.md` — the bare acronym
  indexes, each a subset of its loaded core.
