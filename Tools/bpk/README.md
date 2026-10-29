# Bash Parcel Kit (BPK)

The starter kit for an estate that stands itself up from a parcel.

A parcel delivers kits into repositories that already exist. Before the first
door in those repositories can run, the estate receiving them needs a few things
no kit installs: a procedure to walk, a handful of documents, and the short
texts a first install needs in place. This kit carries them.

It names no receiving estate. Wherever a text needs something only the receiver
can say — its own name, its prefix, its hosts, the people a session will be
working with — the kit leaves a substitution for the receiver to fill.

## Copy once, then tune

Copy what you need out of `Tools/bpk/` into your own repositories and your own
studbook, and tune the copies. From that moment they are yours and take an
independent life; nothing in this kit reaches them again.

This directory is replaced whole by each parcel that carries the kit, so an edit
made here does not survive the next one. Read it instead as the edition your
copies were taken from: the base you compare a later edition against when you
decide what to carry forward.

## What this kit carries

No handbook: the estate handbook is the forge's, and its founding track walks
an estate to its first finished pace, its doors taking the templates below as
their inputs on the way.

Sixteen templates, each named for what it templates. Copy a template to the path
named beside it, fill its `«...»` substitutions, and delete its template header.
Guillemets mark the boundary between what this kit says and what only the
receiving estate can say; a guillemet left standing is an unfinished file.

Nine of them stand a repository up:

| template | copy to | what it is |
|---|---|---|
| `bpt_burc.env` | `«moorings»/burc.env` | the config regime, twelve variables, three of which the install door reads |
| `bpt_trampoline.sh` | `tt/z-launcher.sh` | the tabtarget trampoline — the one file that knows where the moorings stand |
| `bpt_station.sh` | `tt/buw-SI.StationInit.sh` | the standalone door that writes this machine's station regime |
| `bpt_launcher.sh` | `«moorings»/buml_launchers/launcher.buw_workbench.sh` | the bootstrap seed's launcher stub |
| `bpt_tabtarget.sh` | `tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh` | the bootstrap seed's tabtarget, the door that writes every other one |
| `bpt_claude.md` | into `CLAUDE.md` | the include block that carries kit context to a session |
| `bpt_gitignore` | into `.gitignore` | the rules the kits ask for |
| `bpt_licensing.md` | a root file such as `LICENSING.md` | the statement that the repository is proprietary and the kits are not |
| `bpt_header.txt` | into each source file | the header form a proprietary file wears |

Seven more are documents rather than scaffolding, and they land in the estate's
studbook rather than in any repository. Their paths below are studbook-relative:

| template | copy to | what it is |
|---|---|---|
| `bpt_python.md` | your corpus, under your guides directory | the Python authoring discipline code is written under |
| `bpt_abuttal_infield.md` | `estate/abuttal-infield.md` | the estate rules governing your infield, the directory every repository stands in |
| `bpt_salutation_default.md` | `salutations/default.md` | the letter a session reads on waking, where no tier is elected |
| `bpt_salutation_haiku.md` | `salutations/haiku.md` | the same letter, at the `haiku` stem |
| `bpt_salutation_sonnet.md` | `salutations/sonnet.md` | the same letter, at the `sonnet` stem |
| `bpt_salutation_opus.md` | `salutations/opus.md` | the same letter, at the `opus` stem |
| `bpt_salutation_fable.md` | `salutations/fable.md` | the same letter, at the `fable` stem |

Those seven are the founding's, not the install's. The engine's working doors
refuse until the infield text stands on the studbook's corpus trunk, and a
studbook missing any salutation stem is founded broken — so place them when you
found the store, before the first door runs anywhere. The guide waits on
nothing and can be placed whenever it suits.

The text emplaced above your infield is not among them. It is BUK's own,
`Tools/buk/buai0/buais_oikoumene.md`, the same in every estate, and the door
that emplaces it reads it from the kit; nothing of it is yours to copy or fill.

The salutation roster is the engine's rather than yours: one stem named
`default`, and one per model tier the engine can elect. An engine that later
elects a new tier owes you a new file under the new stem.

## The parcel engine

Beside the templates the kit carries the parcel engine, `bpe/`: the release half
that collects a target's declared kits into a parcel and records it, and the
install half that lays a parcel into its target. Its binary, `bpx`, rides every
parcel at the parcel's root beside the two bootstraps that exec it,
`bpi_install.sh` and `bpu_uninstall.sh`, so a parcel installs and uninstalls
through the engine it brought and never through a binary standing in the
target. The release writes the forge's behest, `bpee_behest.json`, beside the
parcel's record; the install and the uninstall read it there, and refuse a
parcel carrying none.

A repository that cuts parcels weans the engine's two collars from this kit's
`bkd_brood/`: `bpen_suite`, the engine's tests, and `bpen_app`, which converges
the binary its parcels carry.

## Standing a repository up

The order below is load-bearing in three places, each marked. Everything else may
be done whenever it suits.

Before the install:

1. The moorings directory, holding `burc.env` from the template and an
   `buml_launchers/` subdirectory beside it. That subdirectory's name is read by
   the kit as a literal and is not the receiver's to choose.
2. `tt/`, holding `z-launcher.sh` from the template.
3. **The directory `BURC_TOOLS_DIR` names, empty.** The install door writes into
   it and does not create it: it refuses a repository where that directory does
   not already stand. Git tracks no empty directory, so a fresh clone stands this
   one by hand again before its own first install.
4. The `.gitignore` rules and the root licensing statement.
5. **A commit.** The install door refuses a repository with anything uncommitted,
   so sealing the work above is part of the road rather than tidying after it.

The install, from a parcel:

6. Every kit directory the parcel carries and the repository declares, the record
   (`.vvm/vvmt_tamga.json`), and the harness config assets. Nothing else in the repository is
   written, at install or ever — the moorings layer above stays the receiver's.
   Nothing is committed either; sealing the delta is the receiver's act.

After the install:

7. **The station file, written by `tt/buw-SI.StationInit.sh`.** It reads the
   field list out of an installed kit, so it cannot run before step 6 however
   early the config regime names the file it writes.
   The station file is written once for an infield: every repository in it
   names the same file, and the door leaves one that already stands as it is
   unless you pass `--overwrite`.
8. Launcher stubs and tabtargets, every one written by the installed bash
   utility kit's creation doors, a kit's and your own alike.
   A first repository has a bootstrap to break — a door cannot be driven before
   a tabtarget exists to drive it — so the first launcher stub and the first
   tabtarget are placed by hand, from `bpt_launcher.sh` and `bpt_tabtarget.sh`,
   and made executable, which is a thing the generator does for you and a
   hand-placed file does not get. Neither carries a substitution: once its
   header is deleted, the launcher template is byte for byte what the bash
   utility kit's launcher door writes for its own workbench, and the tabtarget
   template what its batch + logging door writes for the seed's name. The
   kit's hurdle `butt_the_seed_templates_are_what_the_doors_emit`
   (`Tools/bkk/but0/tests/butt_template.rs`) holds those two templates against the
   doors' output, and compares nothing else.
   Every other door is written by the doors that seed reaches:
   `tt/buw-tt-cl.CreateLauncher.sh` writes a workbench's launcher stub, and
   `buw-tt-cbl`, `buw-tt-cbn` and `buw-tt-cbo` write tabtargets over it in the
   form each door takes. No test compares what they write with the copies
   standing in the repository that cut your parcel, and the two can differ —
   that repository's own launcher stubs carry a comment line the launcher door
   does not write. The door they write is the one your repository runs.
   Stand the roll call first, `buw-tt-rc.RollCall` over the bash utility kit's
   launcher: it lists every door the installed kits enroll that you have not
   stood, with what each does and how it takes its folio. Which doors take a
   form other than batch + logging, each kit's README says.
   A workbench routes whatever tabtargets stand, so stand the doors you use and
   no more. Run the roll call again after every install: it names in red any
   door left standing whose colophon the new kit edition no longer enrolls.
9. The `CLAUDE.md` include block, naming the context files the install landed.

A repository's own workbench, zipper and CLI modules are its own code, written
in its own tree against the module contract that kit's README states. No kit
ships them and no door generates them.

A kit may ask for more than this — the kennel asks for a toolchain store, a pin
and a lint file at the repository root, and states so in its own README. Read the
README of each kit you installed once, after step 8; what this kit carries is the
ground they all stand on, never the whole of what any one of them wants.
