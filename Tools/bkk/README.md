# Bash Kennel (BKK)

The launch discipline every door that builds, launches or tests a managed subprogram satisfies: uniform conditions, an honest answer to whether the artifact is current, and bounded reporting that never lies, whatever language produced the artifact and whatever language the door is written in.

The kit stands on the bash substrate: every door is a tabtarget, every declaration a bash-readable regime file, and the bootstrap is bash on the station's tackroom, whatever the kennel's interior is written in.
Language backends are tenants inside the discipline rather than the discipline itself; rust is the first, and python stands beside it.

The road from an installed parcel to a whistle that answers is the kit's handbook, `Tools/bkk/HANDBOOK.md`; this file is what you do with the kennel once that road is walked.

## What the kennel holds to

<a id="bksk_platform_blind"></a>**Platform-Blind Premise**

The kennel carries no platform-specific aspect: no platform name enters its source, its collars, its regime files, or its hurdles, and no door branches on what the host is.
What differs between hosts is met by a tool's presence or by a declared fact: a step only one platform's tooling can perform is guarded on the tool standing or declared on the collar, and any key a residence ever carries is opaque text the kennel joins and never reads.

**Every door refuses an uncommitted repository.** A kennel door reads the repository a collar stands in and refuses while anything is uncommitted, a source file, a test file and an untracked file alike, so every verdict maps to a position.

**Currency is git position.** An artifact carries the source position it was struck from, and a door compares positions rather than timestamps.

**A door is bounded by the repository it entered.** It proves what that one repository can prove about itself, and reads no other.

## The doors

Every door is a tabtarget, one per door and never one per collar, and a call is three things in order: the verb, which is the tabtarget's colophon and frontispiece; the collar, which is the door's first argument; and after the collar, what the collar's kind owns, the selection patterns for a suite and the program's own arguments for an app.
No flag rides the call.
The verbosity dial is the substrate's own `BURE_VERBOSE`.

| Door | Argument | What it does |
| --- | --- | --- |
| `bkw-m.Mush` | collar | Launch what one collar names: a suite under its declared runner, an app under the binary election |
| `bkw-d.Derby` | none | Run every suite collar the weaned set holds, sequentially, running through every red course and naming each in the set verdict |
| `bkw-f.Futurity` | patterns | Run the hurdles the patterns name across every suite collar the weaned set holds, running through every red course and naming each red course and each course not launched |
| `bkw-l.Lineup` | collar | List a suite's hurdles as its runner names them, running nothing |
| `bkw-t.Tattoo` | collar | Proclaim one collar and report every conformance finding, changing nothing on disk |
| `bkw-h.Heel` | collar | Build a collar's launchable current through the leash and install it at its residence |
| `bkw-z.Muzzle` | none | Lint every collar the weaned set holds at the muzzle directory it declares |
| `bkw-g.Gangline` | collar | Re-derive one collar's lock, the one act in which the leash lifts the lock flag |
| `bkw-s.Scoop` | none | Remove every collar's build output, on your word alone |
| `bkw-n.Delouse` | none | Take the day's sweep of every collar's build directory where none has landed today; derby and heel take it themselves, and a build door calls it ahead of its build |
| `bkw-w.Whistle` | none | Report the kennel's own making, building it first where the seat has outrun it |
| `bkw-r.Rebed` | none | Lay each required channel whose compiler does not answer, leaving every answering one as it stands, then converge the kennel and perform the heels and strikes the derby's preflight names; safe to drive at any time |
| `bkw-k.Kennelman` | filter | Run this kit's own hurdles outside the kennel, so a broken kennel can still be told it is broken |

The first ten are the kennel's own doors.
The last three are tabtargets of the kit standing outside that register: the whistle is the bootstrap and is bash, rebed re-lays the toolchain in bash for the same reason, and the kennelman runs the kennel's suite when the kennel cannot.
No door is reached through another.

A collar is a declaration, one per launchable, named by the directory it sits in; "Declaring a collar" below is how to write one.
The kennel runs only the declarations your moorings' weaned set holds, `«moorings»/bkd_weaned/`, one instance directory per declaration, and a kit ships the declarations it offers in its brood, `Tools/«kit»/bkd_brood/`, which no door reads.
You activate one by copying it, never by writing it:

```bash
mkdir -p «moorings»/bkd_weaned
cp -r Tools/bkk/bkd_brood/butun_suite «moorings»/bkd_weaned/
```

Commit the copy.
It must stay byte for byte what the brood carries: a door that meets a copy differing from its brood refuses before it reads anything else, naming both files and the copy that repairs it.
A collar of your own, declaring a crate of yours, has no brood and is written straight into the weaned set.

This kit's brood carries its own two suites, the substrate's suite (`butun_suite`, over the substrate's self-test crate at `Tools/bkk/but0`, which stands in this kit because the substrate holds no Rust), the collars of the exemplar crates and projects it ships, each introduced in its section below, and the two kibbles its doors spawn: `bki_nextest`, which a collar whose runner is nextest needs, and `bki_uv`, which a python collar needs.
Wean the kibbles your collars reach the same way:

```bash
cp -r Tools/bkk/bkd_brood/bki_nextest «moorings»/bkd_weaned/
```

With the substrate's suite weaned, the shortest proof that the kennel works on your station is:

```bash
tt/bkw-t.Tattoo.sh butun_suite
tt/bkw-m.Mush.sh butun_suite
```

The first reads the collar and reports; the second runs it.

## Declaring a collar

This is the whole contract a collar is declared to.
It is read off the kennel's own collar reader, which is the authority: where this text and a finding disagree, the finding is right and this text is the defect.

<a id="bkscl_collar"></a>**Collar**

The kennel's declaration regime at launchable grain: one instance per launchable, proclaiming to the system what may be launched and on what terms.
The genus across every language tenant — a rust crate's binary, a python venv's entry point, a typescript package's runner all wear one — so a door reads one shape whatever produced the thing it names.

### Where it stands

A collar is one family file in a directory of its own: `bkrr.env` for a rust collar, `bkrp.env` for a python one.
That directory's name is the collar's name, and the file's collar field must repeat it.
The directory stands in your moorings' weaned set, `«moorings»/bkd_weaned/`, and the kennel reads that set and nothing else: a family file anywhere else in the repository declares nothing to any door.
One instance directory holds one family file, so a name stands once across the set whatever its family; one crate may carry an app collar and a suite collar under two names.
A collar a kit ships stands in the kit's `bkd_brood/` and is weaned by copying, as above; a name two kits' broods both carry refuses.

Every path a collar carries is relative to the repository root, whatever directory the collar itself stands in.

### The file's syntax

The file is bash-readable, and the kennel reads it without a shell, so it takes a strict subset:

- A line is blank, a comment whose first non-whitespace character is `#`, or an assignment `KEY=value`. Nothing else, and no `export`.
- A value with no whitespace may stand bare. Anything else is double-quoted. Single quotes are refused.
- A value of several elements is double-quoted and continued across lines with a space and a backslash ending every line but the last. The space matters: it is what keeps two elements apart.
- No `$`, no backticks, and no backslash inside a value except the one ending a continued line. Values are stated literally.
- Nothing follows a closing quote on its line.

```bash
BKRR_ROOTS="Tools/mine/src \
  Tools/mine/Cargo.toml \
  Tools/mine/Cargo.lock"
```

### A rust collar

The kit ships a rust hello crate, `Tools/bkk/bke0/bker`, with an app and a suite collar in the kit's brood (`bkern_app`, `bkern_suite`): std alone, no dependencies, every field in its plainest lawful shape, and the one obligation a reader app owes shown in a build script and a few lines of its output module.
Read it beside this section, and wean it to prove the rust tenant on your station.

#### The fields

Seventeen fields exist.
Every collar declares the first twelve.
An app declares two more, a suite declares a different two, and the seventeenth stands only beside a generated file.
Each field the collar owes must be declared even where its value is empty, since a door cannot tell an empty declaration from an omission; a field that belongs to the other kind is a finding.

| Field | Owed by | Admits |
| --- | --- | --- |
| `BKRR_COLLAR` | every collar | the collar's name, equal to its directory's name |
| `BKRR_KIND` | every collar | `bknre_app` or `bknre_suite`, closed |
| `BKRR_MANIFEST` | every collar | the crate's `Cargo.toml`, which must stand |
| `BKRR_TARGET` | every collar | an app: the cargo binary target's name. A suite: `bknre_manifest` for every test of the manifest, or one test target's name |
| `BKRR_ROOTS` | every collar | the elected roots: git pathspecs, one or more (see below) |
| `BKRR_FEATURES` | every collar | cargo feature names separated by whitespace; empty for none |
| `BKRR_PROFILE` | every collar | a cargo profile name, `dev`, `test`, `release`, `bench` or a custom one. A suite may leave it empty for cargo's default; an app names one, since the built binary is found by it |
| `BKRR_SPEND` | every collar | `bknre_reader` or `bknre_writer`, closed (see below) |
| `BKRR_TAMED_CRATES` | every collar | the manifest's direct dependencies by name (see below) |
| `BKRR_FERAL_CRATES` | every collar | `bknre_tamed`, and nothing else |
| `BKRR_MUZZLE` | every collar | a directory holding a `clippy.toml`; never empty, `.` being the usual value |
| `BKRR_EXERGUE` | every collar | `bknre_unstruck` where the compile reads no generated file; otherwise every generated file it reads |
| `BKRR_RESIDENCE` | an app | the directory the built binary is installed into and launched from |
| `BKRR_BYNAME` | an app | the binary's file name inside the residence, usually the target's name |
| `BKRR_RUNNER` | a suite | `bknre_cargo` (`cargo test`) or `bknre_nextest` (`cargo nextest run`), closed |
| `BKRR_TONGUE` | a suite | the output shape the kennel reads: `bknre_harness` for cargo's harness, `bknre_nextest` for nextest, closed. Match it to the runner |
| `BKRR_STRIKE` | a collar whose `BKRR_EXERGUE` names a file | the tabtargets that write those files |

**The roots** say what the launchable is made from, and the kennel measures whether a binary is current by the newest commit touching them.
An element may be a directory, a file, or a `:(exclude)` pathspec carving part of an earlier element back out.
The set may not be empty, some commit must touch it, and it must reach every crate of yours that is compiled into the launchable: the crate itself and every local path dependency, followed through theirs.
A suite is compiled with its own crate's dev-dependencies and an app is not, so the two kinds reach different sets.
Reaching a crate means naming its directory, a directory above it, or paths inside it.

**The crate lists** part the manifest's direct dependencies, local and registry alike.
Every one is named in `BKRR_TAMED_CRATES`, except that an app does not name its dev-dependencies, since they are never compiled into it.
Only direct edges are named, since the lock pins everything beyond them.
A crate whose build script fetches during compilation is refused outright, so `BKRR_FERAL_CRATES` carries `bknre_tamed` alone: naming a crate there is a finding, and the remedy is to take the crate out of the closure.

**The spend** says whether the launchable writes shared durable state.
A `bknre_reader` borrows an installed binary wherever that binary is current with the roots.
A `bknre_writer` is always launched from the installed binary at its residence, whatever any position says, so a stale build never writes to a live store.

**A reader app owes one answer.** Run with `--exergue` as its first argument, it prints the commit it was built at, forty hex characters on a line, and nothing else.
The kennel asks before every launch and runs the binary only where the tree under the roots is the same at that commit as at your worktree; a binary that gives no answer is never launched, and rebuilding the same crate cannot make it answer.
The kennel's doors state the commit into every build as `BKCH_COLLAR_POSITION`, so the crate needs a build script that refuses where the variable is unset, hands it to the compile with `cargo:rustc-env=BKCH_COLLAR_POSITION=…`, and asks cargo to rebuild when it changes with `cargo:rerun-if-env-changed=BKCH_COLLAR_POSITION`; the app prints it with `env!`.
A crate that is also a library other crates link carries no build script for it: that script would run inside every linking crate's build, and the engine's and the census's build passthrough states no position, so each of those builds would fail.
Such a crate reads the position with `env!("BKCH_COLLAR_POSITION")` in the code that answers `--exergue`, and a build missing the variable then fails to compile, which is its refusal.
No generated file stands in that compile, so `BKRR_EXERGUE` stays `bknre_unstruck`.
A writer is never asked, and a suite is compiled from the source your worktree holds, so neither owes the answer.

**The exergue** is a generated source file a compile reads and nobody commits.
Where a collar's compile reads one, including one read by a path dependency, `BKRR_EXERGUE` names each file and `BKRR_STRIKE` names the doors that write them.
The two are declared together or not at all.
A seat on which an exergue has not been struck is refused before cargo runs, naming the doors to run.

#### Two collars

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

Commit the collar into the weaned set, then drive `tt/bkw-t.Tattoo.sh` over it.
Tattoo reports every finding at once and changes nothing, so a collar is done when tattoo reports none.

### A python collar

A python collar points at a uv project as a rust collar points at a crate.
The kit ships four projects under `Tools/bkk/bke0/`, each with its collars in the kit's brood: a hello project with an app and a suite (`bkepn_app`, `bkepn_suite`), an OpenMM suite (`bkeon_suite`), a torch suite (`bketn_suite`), and a declared project with an app and a suite (`bkeun_app`, `bkeun_suite`) whose manifest names the one platform it runs on.
Read them beside this section, and wean one to prove the python tenant on your station.

#### The project

A uv project is three files standing together in one directory:

- `pyproject.toml`, which the collar names.
- `.python-version`, the exact interpreter as three numbers, such as `3.12.7`. A bare `3.12` is refused.
- `uv.lock`, written by gangline and never by hand.

The kennel states uv's settings itself on every call, so the project's `[tool.uv]` table carries only what is the project's own: its sources and its indexes.
A non-default index is `explicit = true` and is drawn from by a `[tool.uv.sources]` entry, and `index-strategy` is never set.
The build and binary keys (`no-build`, `no-build-package`, `no-binary`, `no-binary-package`) are never set either: the kennel states the bar itself.
A flat index, in either spelling (`find-links`, `format = "flat"`), is refused, because over one uv proves no coverage at the lock.
A path, workspace or git source is refused: one project stands per environment, and a sibling project is reached as a wheel or not at all.
Metadata is static, never `dynamic`.

**The kennel never builds a dependency.** Every dependency arrives as a wheel the lock hashes.
Gangline resolves fresh under uv's build bar at every drive, so a package with no wheel for a platform the project declares in `required-environments`, or no wheel anywhere, refuses the lock, naming the package and the environment in uv's words.
Heel names every package the lock carries, save the project's own, as one uv may not build, so a station whose platform no wheel covers refuses rather than compiling.
A project with an app declares uv's own backend, `uv_build`, in a range the kennel's uv satisfies, because the project's own build is the one build heel performs and its requirements stand in no lock.

Keep `requires-python` to the pin's minor, such as `>=3.12,<3.13`.
The lock records every wheel the range admits on every platform, so an open range locks wheels no station of yours will take.

The kennel runs every suite with warnings as errors.
A warning a case raises fails that case, and one raised while the suite is collected fails the course; the repair is in the project, never a filter.

**A project may declare the platforms its lock covers**, in uv's own `environments` key of its `[tool.uv]` table, such as `environments = ["sys_platform == 'linux'"]`.
uv resolves for those platforms alone and records them in the lock.
On a station outside them, every door reports the project's collars as `unhitched`: the derby and the `bkw-f` door name each such course in their verdict, launch nothing of it, write it no record and stay green; heel and mush refuse, naming the declaration; tattoo says so beside its clean verdict.
Nothing on that station is missing, so there is nothing to converge.
A station that lacks a tool is never so reported: its course stays red, naming the tool.
Name the platform in the project's manifest and nowhere else, never in a collar.
The declared exemplar (`bkeun_app`, `bkeun_suite`) shows the shape.

A project that only runs a suite declares no build system and is never built.
A project with an app declares its entry point under `[project.scripts]` and a build system; uv's own backend, `uv_build` in the range the kennel's uv carries, builds it with nothing fetched.

#### The python fields

| Field | Owed by | Admits |
| --- | --- | --- |
| `BKRP_COLLAR` | every collar | the collar's name, equal to its directory's name |
| `BKRP_KIND` | every collar | `bknre_app` or `bknre_suite`, closed |
| `BKRP_MANIFEST` | every collar | the project's `pyproject.toml`, which must stand |
| `BKRP_ROOTS` | every collar | the project's directory |
| `BKRP_SPEND` | every collar | `bknre_reader` or `bknre_writer`, closed |
| `BKRP_BAKED_PACKAGES` | every collar | the manifest's direct dependencies by name, as the lock spells them (see below) |
| `BKRP_CRUDE_PACKAGES` | every collar | `bknre_baked`, and nothing else |
| `BKRP_BYNAME` | an app | the entry point's key in `[project.scripts]` |
| `BKRP_TARGET` | a suite | `bknre_manifest` for the project's whole suite, as the runner's own configuration finds it |
| `BKRP_RUNNER` | a suite | `bknre_pytest`, closed |
| `BKRP_TONGUE` | a suite | `bknre_pytest`, closed |

No residence is declared: an app's entry point stands only in the environment the kennel builds for the project, under the [Loosebox](#bkspy_loosebox), outside your tree.

**The package lists** part the project's direct dependencies by how they arrive.
Every direct dependency the lock's own account of the manifest carries is named in `BKRP_BAKED_PACKAGES`, the dev group's included for an app and a suite alike, since one environment per project carries both; a name is spelled as the lock spells it, so `Jinja2` is `jinja2`.
Only direct edges are named, since the lock pins everything beyond them, and tattoo proves each named package stands in the lock with a wheel.
A package that would be compiled from its source on a station the lock covers would be named in `BKRP_CRUDE_PACKAGES`, which carries `bknre_baked` alone: naming a package there is a finding, and the remedy is to take the package out of the closure or serve a wheel for it.

#### The first lock

A new project has no lock.
Take the doors in this order:

```bash
tt/bkw-h.Heel.sh «collar»
tt/bkw-g.Gangline.sh «collar»
```

Commit the lock gangline wrote, then:

```bash
tt/bkw-h.Heel.sh «collar»
tt/bkw-m.Mush.sh «collar»
```

The first heel installs the pinned interpreter and stops, naming gangline, because gangline resolves against an installed interpreter and never fetches one.
Gangline writes the lock and leaves it for you to read and commit.
The second heel builds the environment from the lock.
After you change the manifest, gangline re-derives the lock the same way.

The first converge on a station downloads the project's closure into the tackroom's cache.
Every later converge on that station installs from there.

Every python suite collar you wean is a course of the derby and of `bkw-f`, like any rust suite.
Before any course runs, their preflight reads each python course's environment and refuses, naming heel, where the environment stands nowhere or no longer answers its lock.
So a fresh worktree heels each python collar once before its first derby, and again whenever a lock it pulls has moved.

## Refusals you will meet

Every kennel refusal names what it found and the act that answers it.
Four are worth knowing before you meet them:

**The repository carries uncommitted changes.** Commit, then drive the door again.
This is the house rule that no test runs without a commit, so every record maps to a position.

**`BURS_TACKROOM` is not set.** The station regime refuses a station file that omits the tackroom, naming it, before any kennel door runs.
Declare the tackroom in your station file, as the handbook's first act says.
The kennel will not fall back to a per-clone copy of the toolchain.

**The weaned copy differs from its brood.** Take the kit's declaration with the `cp` the refusal prints; where the kit's declaration is what is wrong, the repair is the kit's.

**The pinned toolchain does not answer in the tackroom.** Absent, gutted on disk, or answering another release: the refusal names `tt/bkw-r.Rebed.sh`, which re-lays it.
Run it deliberately; the kennel never downloads a toolchain on your behalf.

## Where the rest is written

This file is the kennel's doors and the contract a collar is declared to, and the handbook beside it is the road onto a station.
Everything else about the kennel's design lives with its authors, and declaring and launching collars needs none of it.

## Glossary

<a id="bksnc_leash"></a>**Leash**

The chokepoint every toolchain call the kennel makes passes through: the pin stated, the lock flag on every call, the tackroom fence.

<a id="bksnc_hurdle"></a>**Hurdle**

One test case, a launch carrying an expectation.

<a id="bksnc_course"></a>**Course**

A suite: the set of hurdles one collar launches as one unit.

<a id="bkslr_lure"></a>**Lure**

A seat composed for one of the kennel's own hurdles, which the kennel is driven over as over a real one.

<a id="bkscl_tamed"></a>**Tamed**

A crate whose build reaches nothing past the repository, the tackroom and cargo's registry cache, so nothing is fetched on the way.

<a id="bkscl_feral"></a>**Feral**

A crate whose build fetches during compilation; the kennel refuses it and never provisions for it.

<a id="bksnc_unhitched"></a>**Unhitched**

A course whose project declares the platforms its lock covers, on a station outside them: neither red nor skipped, and named in the verdict.

<a id="bkspy_loosebox"></a>**Loosebox**

The place a worktree's environments and other rebuildable build products stand, beside the worktree and outside every source tree, each path beneath it opening on the worktree's own directory name.
