# Bash Kennel (BKK) — the receiving station's road to the whistle

This kit is delivered as source. The parcel carries `Tools/bkk/` whole, and the
station you install it on builds the kennel itself, through its own whistle.

This is the road from an installed parcel to a whistle that answers. It is four
acts. Walk them in order, from the repository root.

The parcel install writes kit directories and nothing else: it does not touch
your moorings layer, so the launcher stub and the doors are yours to mint. That
is one act apiece, through the substrate's own emitters.

---

## What the kennel asks of the station

**A tackroom.** The kennel homes the pinned Rust toolchain and the cargo
registry in a station-shared store, apart from the station user's own rustup and
cargo homes. One store serves every clone on the station.

**The pinned channel present in it.** The kennel states its channel on every
cargo invocation rather than inheriting whatever the store defaults to. A
routine invocation verifies and refuses; it never downloads unasked.

**A committed repository.** Every kennel door reads the repository a collar
stands in and refuses while anything is uncommitted — a source file, a test
file, an untracked file alike. This is what makes each verdict map to a
position, and it fires during the kennel's own build, so the acts below end with
a commit before the whistle is blown.

**Kits under `Tools/`.** The kennel's own election file names its roots as
`Tools/bkk/...`, so `BURC_TOOLS_DIR=Tools` in your `burc.env`.

**A pin at the repository root.** The kennel pins its own channel for its own
crate, but every other crate it launches is answered for by the pin standing at
your root:

```toml
[toolchain]
channel = "1.90.0"
```

A crate with no pin above it is refused by name rather than built against
whatever the store defaults to. This bites at the first collar you launch, not
at the whistle.

**A `clippy.toml` at the repository root.** A collar names the directory whose
lint governance it wears, and the substrate's own suite collar names `.` — your
root. The lint list is yours, not the kit's; an empty one is a valid answer:

```toml
disallowed-methods = []
```

A muzzle directory holding no lint list is refused as a muzzle nothing wears.

**The substrate already seated.** Act 2 is driven through the substrate's own
emitter door, `tt/buw-tt-cl.CreateLauncher.sh`; a station that has not yet
minted it mints it the same way, from `Tools/buk/README.md`. Act 3 drives two
more of its creation doors, `buw-tt-cbl` and `buw-tt-cba`, minted the same way.

**Every exergue a collar declares struck once first.** A collar's own
`BKRR_STRIKE` field names the door that writes the generated file its
`BKRR_EXERGUE` field points at, and a fresh seat's first derby refuses at the
first collar whose exergue no door has struck yet — the refusal names that
door. Run it once, ahead of the first derby.

---

## Act 1 — Declare the tackroom

Add the tackroom to your station file — the one your `burc.env` names in
`BURC_STATION_FILE`:

```bash
BURS_TACKROOM=«/absolute/path/to/tackroom»
```

It may be absolute, or relative to the project root as the log directory is.
It must not be the station user's home directory, nor anything inside that
user's `.rustup` or `.cargo` — keeping the kennel's store apart from the
station user's own toolchain is the whole point of the redirection, and the
kennel refuses a declaration that collapses it.

Install the pinned channel into that store. The channel is the one named in
`Tools/bkk/bk0/rust-toolchain.toml`:

```bash
RUSTUP_HOME=«tackroom»/rustup rustup toolchain install «channel» \
  --profile minimal --component clippy --component rustfmt
```

You may skip this and let the kennel refuse instead: its refusal prints this
command with your own paths already filled in.

---

## Act 2 — Mint the launcher stub

The stub delegates the kit's tabtargets to the kennel's workbench. It goes in
your moorings launcher directory, `«moorings»/buml_launchers/` — create that
directory first if this is the first kit you have minted for.

```bash
tt/buw-tt-cl.CreateLauncher.sh Tools/bkk/bk0/bkw_workbench.sh bkw_workbench
```

That writes `«moorings»/buml_launchers/launcher.bkw_workbench.sh` — four lines,
sourcing the substrate's launcher and handing it the workbench path.

---

## Act 3 — Mint the door roster

Twelve doors, each written by the substrate's creation doors over the launcher
stub Act 2 made. A kennel tabtarget carries nothing of the repository it stands
in, so the door these write is the door the kennel ships.

Nine take the plain form:

```bash
tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh «moorings»/buml_launchers/launcher.bkw_workbench.sh \
  bkw-w.Whistle bkw-l.Lineup bkw-t.Tattoo bkw-h.Heel bkw-z.Muzzle \
  bkw-g.Gangline bkw-s.Scoop bkw-n.Delouse bkw-k.Kennelman
```

Three — mush, derby and futurity — take the form whose command writes its own
record:

```bash
tt/buw-tt-cba.CreateTabTargetBatchAmanuensis.sh «moorings»/buml_launchers/launcher.bkw_workbench.sh \
  bkw-m.Mush bkw-d.Derby bkw-f.Futurity
```

That form declares `BURD_AMANUENSIS=1`: each of the three consumes its tenant's
streams and renders the record itself, and under that line the dispatch
composes the three log names and creates no file, leaving the kennel to write
all three members or none — without it the dispatch curates the log line by
line and destroys the interleaving the record exists to hold. A door minted in
the plain form and left without that line still runs; it is simply a poorer
record than the kennel writes.

The workbench routes whatever doors stand, so mint the ones you use; the whole
twelve is the ordinary choice. The substrate's roll call,
`tt/buw-tt-rc.RollCall.sh`, lists any kennel door you have not stood, and names
in red any that a later kennel edition no longer enrolls.

Then ignore the kennel's build directory, if your repository does not already:

```
**/target/
```

The kennel builds into `Tools/bkk/bk0/target/`, and the door law reads untracked
files exactly as it reads modified ones. Without this line the first whistle
refuses on the output of its own build — it names the directory, so the refusal
is legible, but it is cheaper not to meet it.

Now commit the moorings, the doors and that ignore line. The kennel's build
refuses a dirty tree, so this commit is part of the road rather than
housekeeping after it.

---

## Act 4 — Blow the whistle

```bash
tt/bkw-w.Whistle.sh
```

It takes no argument. It stands on the tackroom fence, verifies the pinned
channel is present, builds the kennel binary where the seat has outrun it, and
then reports the kennel's own making in three lines: the position it was built
at, the channel its pin asked for, and the compiler that answered.

The first run compiles the kennel and takes a few minutes. Later runs rebuild
only when the source has moved.

A whistle that answers is the road walked. Everything below is what you can now
do with it.

---

## The doors

Each door wears its own tabtarget and takes a collar as its argument. No door is
reached through another.

| Door | Argument | What it does |
|---|---|---|
| `bkw-w.Whistle` | none | Report the kennel's own making, building it first where the seat has outrun it |
| `bkw-m.Mush` | collar | Launch what one collar names — a suite under its declared runner, an app under the binary election |
| `bkw-d.Derby` | none | Run every suite collar the weaned set holds, sequentially, running through every red course and naming each in the set verdict |
| `bkw-f.Futurity` | patterns | Run the hurdles the patterns name across every suite collar the weaned set holds, running through every red course and naming each red course and each course not launched |
| `bkw-l.Lineup` | collar | List a suite's hurdles as its runner names them, running nothing |
| `bkw-t.Tattoo` | collar | Proclaim one collar and report every conformance finding, changing nothing on disk |
| `bkw-h.Heel` | collar | Build a collar's launchable current through the leash and install it at its residence |
| `bkw-z.Muzzle` | none | Lint every collar the weaned set holds at the muzzle directory it declares |
| `bkw-g.Gangline` | collar | Re-derive one collar's lock — the one act in which the leash lifts the lock flag |
| `bkw-s.Scoop` | none | Remove every collar's target directory, on your word alone |
| `bkw-k.Kennelman` | filter | Run this kit's own hurdles outside the kennel, so a broken kennel can still be told it is broken |

A collar is a declaration, one per launchable, named by the directory it sits
in; "Declaring a collar" below is how to write one. The kennel runs only the
declarations your moorings' weaned set holds — `«moorings»/bkd_weaned/`, one
instance directory per declaration — and a kit ships the declarations it offers
in its brood, `Tools/«kit»/bkd_brood/`, which no door reads. You activate one by
copying it, never by writing it:

```bash
mkdir -p «moorings»/bkd_weaned
cp -r Tools/buk/bkd_brood/bun_suite «moorings»/bkd_weaned/
```

Commit the copy. It must stay byte for byte what the brood carries: a door that
meets a copy differing from its brood refuses before it reads anything else,
naming both files and the copy that repairs it. A collar of your own, declaring
a crate of yours, has no brood and is written straight into the weaned set.

This kit's brood carries its own two suites, the collars of the exemplar crates
and projects it ships (each introduced in its section below), and the two
kibbles its doors spawn: `bki_nextest`, which a collar whose runner is nextest needs, and
`bki_uv`, which a python collar needs. Wean the kibbles your collars reach the
same way:

```bash
cp -r Tools/bkk/bkd_brood/bki_nextest «moorings»/bkd_weaned/
```

With the substrate's suite weaned, the shortest proof that the kennel works on
your station is:

```bash
tt/bkw-t.Tattoo.sh bun_suite
tt/bkw-m.Mush.sh bun_suite
```

The first reads the collar and reports; the second runs it.

---

## Declaring a collar

This is the whole contract for a rust collar. It is read off the kennel's own
collar reader, which is the authority: where this text and a finding disagree,
the finding is right and this text is the defect.

The kit ships a rust hello crate, `Tools/bkk/bke0/bker`, with an app and a suite
collar in the kit's brood (`bkern_app`, `bkern_suite`): std alone, no
dependencies, every field in its plainest lawful shape, and the one obligation a
reader app owes shown in a build script and a few lines of its output module.
Read it beside this section, and wean it to prove the rust tenant on your
station.

### Where it stands

A collar is a file named `bkrr.env` in a directory of its own. That directory's
name is the collar's name, and `BKRR_COLLAR` must repeat it. The directory
stands in your moorings' weaned set, `«moorings»/bkd_weaned/`, and the kennel
reads that set and nothing else: a `bkrr.env` anywhere else in the repository
declares nothing to any door. One instance directory holds one family file, so
a name stands once across the set whatever its family; one crate may carry an
app collar and a suite collar under two names. A collar a kit ships stands in
the kit's `bkd_brood/` and is weaned by copying, as above; a name two kits'
broods both carry refuses.

Every path a collar carries is relative to the repository root, whatever
directory the collar itself stands in.

### The file's syntax

The file is bash-readable, and the kennel reads it without a shell, so it takes
a strict subset:

- A line is blank, a comment whose first non-whitespace character is `#`, or an
  assignment `KEY=value`. Nothing else, and no `export`.
- A value with no whitespace may stand bare. Anything else is double-quoted.
  Single quotes are refused.
- A value of several elements is double-quoted and continued across lines with
  a space and a backslash ending every line but the last. The space matters: it
  is what keeps two elements apart.
- No `$`, no backticks, and no backslash inside a value except the one ending a
  continued line. Values are stated literally.
- Nothing follows a closing quote on its line.

```bash
BKRR_ROOTS="Tools/mine/src \
  Tools/mine/Cargo.toml \
  Tools/mine/Cargo.lock"
```

### The fields

Seventeen fields exist. Every collar declares the first twelve. An app declares
two more, a suite declares a different two, and the seventeenth stands only
beside a generated file. Each field the collar owes must be declared even where
its value is empty, since a door cannot tell an empty declaration from an
omission; a field that belongs to the other kind is a finding.

| Field | Owed by | Admits |
|---|---|---|
| `BKRR_COLLAR` | every collar | the collar's name, equal to its directory's name |
| `BKRR_KIND` | every collar | `bknre_app` or `bknre_suite` — closed |
| `BKRR_MANIFEST` | every collar | the crate's `Cargo.toml`, which must stand |
| `BKRR_TARGET` | every collar | an app: the cargo binary target's name. A suite: `bknre_manifest` for every test of the manifest, or one test target's name |
| `BKRR_ROOTS` | every collar | the elected roots: git pathspecs, one or more (see below) |
| `BKRR_FEATURES` | every collar | cargo feature names separated by whitespace; empty for none |
| `BKRR_PROFILE` | every collar | a cargo profile name — `dev`, `test`, `release`, `bench` or a custom one. A suite may leave it empty for cargo's default; an app names one, since the built binary is found by it |
| `BKRR_SPEND` | every collar | `bknre_reader` or `bknre_writer` — closed (see below) |
| `BKRR_TAMED_CRATES` | every collar | the manifest's direct dependencies by name (see below) |
| `BKRR_FERAL_CRATES` | every collar | `bknre_tamed`, and nothing else |
| `BKRR_MUZZLE` | every collar | a directory holding a `clippy.toml`; never empty, `.` being the usual value |
| `BKRR_EXERGUE` | every collar | `bknre_unstruck` where the compile reads no generated file; otherwise every generated file it reads |
| `BKRR_RESIDENCE` | an app | the directory the built binary is installed into and launched from |
| `BKRR_BYNAME` | an app | the binary's file name inside the residence, usually the target's name |
| `BKRR_RUNNER` | a suite | `bknre_cargo` (`cargo test`) or `bknre_nextest` (`cargo nextest run`) — closed |
| `BKRR_TONGUE` | a suite | the output shape the kennel reads: `bknre_harness` for cargo's harness, `bknre_nextest` for nextest — closed. Match it to the runner |
| `BKRR_STRIKE` | a collar whose `BKRR_EXERGUE` names a file | the tabtargets that write those files |

**The roots** say what the launchable is made from, and the kennel measures
whether a binary is current by the newest commit touching them. An element may
be a directory, a file, or a `:(exclude)` pathspec carving part of an earlier
element back out. The set may not be empty, some commit must touch it, and it
must reach every crate of yours that is compiled into the launchable — the
crate itself and every local path dependency, followed through theirs. A suite
is compiled with its own crate's dev-dependencies and an app is not, so the two
kinds reach different sets. Reaching a crate means naming its directory, a
directory above it, or paths inside it.

**The crate lists** part the manifest's direct dependencies, local and registry
alike. Every one is named in `BKRR_TAMED_CRATES`, except that an app does not
name its dev-dependencies, since they are never compiled into it. Only direct
edges are named, since the lock pins everything beyond them. A crate whose
build script fetches during compilation is feral and is refused outright, so
`BKRR_FERAL_CRATES` carries `bknre_tamed` alone: naming a crate there is a
finding, and the remedy is to take the crate out of the closure.

**The spend** says whether the launchable writes shared durable state. A
`bknre_reader` borrows an installed binary wherever that binary is current with
the roots. A `bknre_writer` is always launched from the installed binary at its
residence, whatever any position says, so a stale build never writes to a live
store.

**A reader app owes one answer.** Run with `--exergue` as its first argument, it
prints the commit it was built at, forty hex characters on a line, and nothing
else. The kennel asks before every launch and runs the binary only where the
tree under the roots is the same at that commit as at your checkout; a binary
that gives no answer is never launched, and rebuilding the same crate cannot
make it answer. The kennel's doors state the commit into every build as
`BKCH_COLLAR_POSITION`, so the crate needs a build script that refuses where
the variable is unset, hands it to the compile with
`cargo:rustc-env=BKCH_COLLAR_POSITION=…`, and asks cargo to rebuild when it
changes with `cargo:rerun-if-env-changed=BKCH_COLLAR_POSITION`; the app prints
it with `env!`. No generated file stands in that compile, so `BKRR_EXERGUE`
stays `bknre_unstruck`. A writer is never asked, and a suite is compiled from
the source your checkout holds, so neither owes the answer.

**The exergue** is a generated source file a compile reads and nobody commits.
Where a collar's compile reads one, including one read by a path dependency,
`BKRR_EXERGUE` names each file and `BKRR_STRIKE` names the doors that write
them. The two are declared together or not at all. A seat on which an exergue
has not been struck is refused before cargo runs, naming the doors to run.

### Two collars

A suite over a library crate's whole test set:

```bash
BKRR_COLLAR="mine_suite"
BKRR_KIND="bknre_suite"
BKRR_MANIFEST="Tools/mine/Cargo.toml"
BKRR_TARGET="bknre_manifest"
BKRR_ROOTS="Tools/mine/src \
  Tools/mine/tests \
  Tools/mine/Cargo.toml \
  Tools/mine/Cargo.lock"
BKRR_FEATURES=""
BKRR_PROFILE="test"
BKRR_SPEND="bknre_reader"
BKRR_TAMED_CRATES="regex"
BKRR_FERAL_CRATES="bknre_tamed"
BKRR_MUZZLE="."
BKRR_RUNNER="bknre_cargo"
BKRR_TONGUE="bknre_harness"
BKRR_EXERGUE="bknre_unstruck"
```

An app over the same crate's binary target `mine`:

```bash
BKRR_COLLAR="mine_app"
BKRR_KIND="bknre_app"
BKRR_MANIFEST="Tools/mine/Cargo.toml"
BKRR_TARGET="mine"
BKRR_ROOTS="Tools/mine/src \
  Tools/mine/build.rs \
  Tools/mine/Cargo.toml \
  Tools/mine/Cargo.lock"
BKRR_FEATURES=""
BKRR_PROFILE="release"
# A reader: the crate's build script compiles in the commit, and the app
# prints it when asked --exergue.
BKRR_SPEND="bknre_reader"
BKRR_TAMED_CRATES="regex"
BKRR_FERAL_CRATES="bknre_tamed"
BKRR_MUZZLE="."
BKRR_RESIDENCE="Tools/mine/target/release"
BKRR_BYNAME="mine"
BKRR_EXERGUE="bknre_unstruck"
```

Commit the collar into the weaned set, then drive `tt/bkw-t.Tattoo.sh` over it. Tattoo reports every
finding at once and changes nothing, so a collar is done when tattoo reports
none.

---

## Declaring a python collar

A python collar points at a uv project as a rust collar points at a crate. Its
file is `bkrp.env`, read under the same syntax and standing in the weaned set
the same way. The kit ships four projects under `Tools/bkk/bke0/`, each with
its collars in the kit's brood: a hello project with an app and a suite
(`bkepn_app`, `bkepn_suite`), an OpenMM suite (`bkeon_suite`), a torch suite
(`bketn_suite`), and a declared project with an app and a suite (`bkeun_app`,
`bkeun_suite`) whose manifest names the one platform it runs on. Read them
beside this section, and wean one to prove the python tenant on your station.

### The project

A uv project is three files standing together in one directory:

- `pyproject.toml`, which the collar names.
- `.python-version`, the exact interpreter as three numbers, such as `3.12.7`.
  A bare `3.12` is refused.
- `uv.lock`, written by gangline and never by hand.

The kennel states uv's settings itself on every call, so the project's
`[tool.uv]` table carries only what is the project's own: its sources and its
indexes. A non-default index is `explicit = true` and is drawn from by a
`[tool.uv.sources]` entry, and `index-strategy` is never set. The build and
binary keys (`no-build`, `no-build-package`, `no-binary`, `no-binary-package`)
are never set either: the kennel states the bar itself. A flat index, in
either spelling (`find-links`, `format = "flat"`), is refused, because over
one uv proves no coverage at the lock. A path, workspace or git source is
refused: one project stands per environment, and a sibling project is
reached as a wheel or not at all. Metadata is static, never `dynamic`.

**The kennel never builds a dependency.** Every dependency arrives as a wheel
the lock hashes. Gangline resolves fresh under uv's build bar at every drive,
so a package with no wheel for a platform the project declares in
`required-environments`, or no wheel anywhere, refuses the lock, naming the
package and the environment in uv's words. Heel names every package the lock
carries, save the project's own, as one uv may not build, so a station whose
platform no wheel covers refuses rather than compiling. A project with an app
declares uv's own backend, `uv_build`, in a range the kennel's uv satisfies,
because the project's own build is the one build heel performs and its
requirements stand in no lock.

Keep `requires-python` to the pin's minor, such as `>=3.12,<3.13`. The lock
records every wheel the range admits on every platform, so an open range locks
wheels no station of yours will take.

The kennel runs every suite with warnings as errors. A warning a case raises
fails that case, and one raised while the suite is collected fails the course;
the repair is in the project, never a filter.

**A project may declare the platforms its lock covers**, in uv's own
`environments` key of its `[tool.uv]` table, such as
`environments = ["sys_platform == 'linux'"]`. uv resolves for those platforms
alone and records them in the lock. On a station outside them, every door
reports the project's collars as **unhitched**: the derby and the futurity name
each such course in their verdict, launch nothing of it, write it no record and
stay green; heel and mush refuse, naming the declaration; tattoo says so beside
its clean verdict. Nothing on that station is missing, so there is nothing to
converge. A station that lacks a tool is never unhitched: its course stays red,
naming the tool. Name the platform in the project's manifest and nowhere else,
never in a collar. The declared exemplar (`bkeun_app`, `bkeun_suite`) shows the
shape.

A project that only runs a suite declares no build system and is never built.
A project with an app declares its entry point under `[project.scripts]` and a
build system; uv's own backend, `uv_build` in the range the kennel's uv
carries, builds it with nothing fetched.

### The fields

| Field | Owed by | Admits |
|---|---|---|
| `BKRP_COLLAR` | every collar | the collar's name, equal to its directory's name |
| `BKRP_KIND` | every collar | `bknre_app` or `bknre_suite` — closed |
| `BKRP_MANIFEST` | every collar | the project's `pyproject.toml`, which must stand |
| `BKRP_ROOTS` | every collar | the project's directory |
| `BKRP_SPEND` | every collar | `bknre_reader` or `bknre_writer` — closed |
| `BKRP_BAKED_PACKAGES` | every collar | the manifest's direct dependencies by name, as the lock spells them (see below) |
| `BKRP_CRUDE_PACKAGES` | every collar | `bknre_baked`, and nothing else |
| `BKRP_BYNAME` | an app | the entry point's key in `[project.scripts]` |
| `BKRP_TARGET` | a suite | `bknre_manifest` for the project's whole suite, as the runner's own configuration finds it |
| `BKRP_RUNNER` | a suite | `bknre_pytest` — closed |
| `BKRP_TONGUE` | a suite | `bknre_pytest` — closed |

No residence is declared: an app's entry point stands only in the environment
the kennel builds for the project, outside your tree.

**The package lists** part the project's direct dependencies by how they
arrive. Every direct dependency the lock's own account of the manifest carries
is named in `BKRP_BAKED_PACKAGES`, the dev group's included for an app and a
suite alike, since one environment per project carries both; a name is
spelled as the lock spells it, so `Jinja2` is `jinja2`. Only direct edges are
named, since the lock pins everything beyond them, and tattoo proves each
named package stands in the lock with a wheel. A package that would be
compiled from its source on a station the lock covers would be named in
`BKRP_CRUDE_PACKAGES`, which carries `bknre_baked` alone: naming a package
there is a finding, and the remedy is to take the package out of the closure
or serve a wheel for it.


### The first lock

A new project has no lock. Take the doors in this order:

```bash
tt/bkw-h.Heel.sh «collar»
tt/bkw-g.Gangline.sh «collar»
```

Commit the lock gangline wrote, then:

```bash
tt/bkw-h.Heel.sh «collar»
tt/bkw-m.Mush.sh «collar»
```

The first heel installs the pinned interpreter and stops, naming gangline,
because gangline resolves against an installed interpreter and never fetches
one. Gangline writes the lock and leaves it for you to read and commit. The
second heel builds the environment from the lock. After you change the
manifest, gangline re-derives the lock the same way.

The first converge on a station downloads the project's closure into the
tackroom's cache. Every later converge on that station installs from there.

Every python suite collar you wean is a course of the derby and the futurity,
like any rust suite. Before any course runs, their preflight reads each python
course's environment and refuses, naming heel, where the environment stands
nowhere or no longer answers its lock. So a fresh checkout heels each python
collar once before its first derby, and again whenever a lock it pulls has
moved.

---

## Refusals you will meet

Every kennel refusal names what it found and the act that answers it. Four are
worth knowing before you meet them:

**The repository carries uncommitted changes.** Commit, then drive the door
again. This is the house rule that no test runs without a commit, so every
record maps to a position.

**`BURS_TACKROOM` is unset.** Act 1. The kennel will not fall back to a
per-clone copy of the toolchain.

**The weaned copy differs from its brood.** Take the kit's declaration with the
`cp` the refusal prints; where the kit's declaration is what is wrong, the repair
is the kit's.

**The pinned channel is absent from the tackroom.** The refusal prints the
`rustup toolchain install` line with your own paths in it. Run it deliberately;
the kennel never downloads a toolchain on your behalf.

---

## Where the rest is written

This file is the road onto a station and the contract a collar is declared to.
Everything else about the kennel's design lives with its source, and declaring
and launching collars needs none of it.
