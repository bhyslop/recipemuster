// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The composing surface for a lure (BKSLR-Lure.adoc "The Lure").
//!
//! A lure is a seat composed for one hurdle: a git repository under the
//! dispatch's temp root, carrying a commit, a pin, and whatever else the hurdle
//! asks for. The kennel is driven over it exactly as over a real seat.
//!
//! A LURE IS COMPOSED, NEVER COMMITTED. The kennel's doors walk the tree, so a
//! fixture crate standing in the repository would be a real collar to every one
//! of them, and excluding it would be a second concept no sheaf holds.
//!
//! THIS IS THE ONLY HOME. A hurdle that reaches the temp root, composes a
//! repository, or spawns the kennel reaches it here — never through a bare
//! `std::env` or a `git` of its own. The founding surface is deliberately small:
//! it composes and it cleans up, and it grows as hurdles ask it to.
//!
//! TWO SHAPES ARE COMPOSED HERE. The bare lure above is one. The other is a
//! substrate seat laid down inside it: the trampoline, moorings, station file,
//! launcher stub, coordinator and tabtarget a dispatch needs, so the substrate
//! can be driven through its own door rather than sourced bare. A hurdle asking
//! for one gets it from here for the same reason it gets a lure from here.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The dispatch's temp root. ABSENT IS A REFUSAL, never a fallback: a surface
/// that cannot find this variable refuses the hurdle naming it and reaches for
/// no ambient temp directory, so the kennel's hurdles are always launched
/// through the substrate.
pub const BKTU_TEMP_ROOT_VAR: &str = "BURD_TEMP_DIR";

/// The dispatch's output directory, which a drive is handed a lure-local one of
/// rather than inheriting the session's.
pub const BKTU_OUTPUT_DIR_VAR: &str = "BURD_OUTPUT_DIR";

/// The checkout's own loosebox, where a door keeps memory that must outlive the
/// per-invocation directory — the delouse's day among it. A drive is handed a
/// lure-local one for the same reason it is handed the pair above: a child
/// inheriting the session's would keep its memory where the run executing it
/// keeps its own.
pub const BKTU_LOOSEBOX_DIR_VAR: &str = "BURD_LOOSEBOX_DIR";

/// The dispatch's config directory — where the kennel reads its geography, and
/// therefore the one variable that can make a hurdle's verdict a function of
/// which tabtarget drove the suite.
pub const BKTU_CONFIG_DIR_VAR: &str = "BURD_CONFIG_DIR";

/// What a BARE KENNEL SPAWN does not inherit: the two channels that would make a
/// hurdle's verdict a function of which tabtarget drove the suite.
///
/// NARROW ON PURPOSE. Everything else a dispatch exports is a fact about the
/// STATION the child genuinely stands on — the tackroom above all, which is the
/// toolchain fence every cargo invocation is required to inherit — so a
/// prefix-wide sweep here would sever it. A bare spawn establishes nothing of
/// its own and is handed what it needs instead.
const ZBKTU_SPAWN_STRIPPED: &[&str] = &[BKTU_CONFIG_DIR_VAR, ZBKTU_FOLIO_FAMILY];

/// What a COMPOSED SEAT does not inherit, which is every channel the substrate
/// speaks across an exec.
///
/// WIDE ON PURPOSE, and the mirror of the roster above: a composed seat is a
/// repository of its own and re-establishes every one of these for itself, so a
/// value the outer dispatch set that survived into it would make the hurdle's
/// verdict a function of the driver. `BURE_` passes untouched, being the
/// substrate's operator-ambient channel by design.
const ZBKTU_SEAT_STRIPPED: &[&str] = &["BURC_", "BURD_", "BURS_", "BURV_", ZBKTU_FOLIO_FAMILY];

/// The zipper's folio channel, stripped from every spawn beside the config
/// directory.
///
/// IT IS A CALLER'S ARGUMENT AND NOT A SETTING, which is what makes a leak
/// through it read as a real regression: driving this suite as
/// `tt/bkw-m.Mush.sh suite-buk` once put `BUZ_FOLIO=suite-buk` in the running
/// process, a composed seat's whistle read it as its own argument, and three
/// hurdles went red against a refusal meant for somebody else. The dispatch
/// surface below already strips it; a bare kennel spawn is the other road into
/// a child and owes the same strip.
const ZBKTU_FOLIO_FAMILY: &str = "BUZ_";

/// The dispatch's timestamp, which a drive is handed one of rather than
/// inheriting the session's.
///
/// A DRIVE STANDS ON A DAY, because the delouse keeps its cadence by the
/// dispatch's own reckoning and refuses where none is stated. A hurdle proving
/// what the sweep does on a second drive states this itself — the same day to
/// prove the cadence holds, another to prove it lapses — and the value here is
/// what every other drive stands on.
pub const BKTU_NOW_VAR: &str = "BURD_NOW_STAMP";

/// The day every drive stands on unless the hurdle states another.
pub const BKTU_NOW: &str = "20260101-000000-0-0";

/// The toolchains the kennel's own hurdles require: the estate's pin and one
/// other (BKSLR-Lure.adoc "The Required Toolchains").
///
/// THIS IS A HURDLE'S DEMAND, NEVER THE KENNEL'S. At runtime the kennel binds
/// whatever pin a collar's repository names and asks nothing of the station
/// beyond that pin. Its hurdles ask more, because proving that a bumped pin
/// changes the compiler needs a second toolchain present to bump to — a property
/// no single-toolchain station can demonstrate at all.
pub const BKTU_REQUIRED_TOOLCHAINS: &[&str] = &["1.90.0", "1.89.0"];

/// The seat's moorings directory — its config layer, holding the regime file,
/// the station file and the launcher stub.
///
/// `.buk` is a moorings name the estate already admits (`tt/z-launcher.sh` names
/// it beside `rbmm_moorings` as one of the shapes a consumer may keep), so a
/// composed seat spells an established name here rather than minting one.
const ZBKTU_MOORINGS: &str = ".buk";

/// The seat's own door: the tabtarget a hurdle dispatches through, and the only
/// way into the seat. The colophon is the lure's own word, so nothing in a
/// composed tree can be mistaken for an estate door.
const ZBKTU_TABTARGET: &str = "tt/lure-t.Trivial.sh";

/// The program cargo reaches for when a collar's runner is nextest — named here
/// because a narrowed path is composed by finding the directories that hold it.
const ZBKTU_NEXTEST: &str = "cargo-nextest";

/// The estate's own nextest kibble, named here so a lure laying it down
/// reaches the one directory that declaration stands in. It is the kennel's
/// own constant read from the other side, and the two are asserted equal by a
/// hurdle rather than merely intended to agree.
const ZBKTU_NEXTEST_KIBBLE: &str = "bki_nextest";

/// The estate's own uv kibble, read from the other side exactly as the nextest
/// one above is.
const ZBKTU_UV_KIBBLE: &str = "bki_uv";

/// The station's shared tool store, and the kennel's own quarter of it — the
/// two facts a hurdle needs to name a residence the door will compose.
/// Both are the kennel's own constants read from the other side.
const ZBKTU_TACKROOM_VAR: &str = "BURD_TACKROOM";
const ZBKTU_QUARTER: &str = "bkk";

/// The colophon `ZBKTU_TABTARGET` carries, which the dispatch splits out and
/// hands the coordinator as its first argument. Named here because a hurdle
/// asserting that the command reached the coordinator asserts against the same
/// string the inscription spells.
pub const BKTU_COLOPHON: &str = "lure-t";

/// Which required toolchains the station is missing, if any.
///
/// The station is read through the same `RUSTUP_HOME` the fence exported, so
/// this asks about the tackroom rather than about whatever the station user's
/// own rustup happens to hold.
pub fn bktu_missing_toolchains() -> Vec<&'static str> {
    let out = Command::new("rustup")
        .arg("toolchain")
        .arg("list")
        .output()
        .unwrap_or_else(|err| panic!("could not ask rustup what the station holds: {}", err));

    let held = String::from_utf8_lossy(&out.stdout).to_string();

    BKTU_REQUIRED_TOOLCHAINS
        .iter()
        .copied()
        .filter(|wanted| !held.contains(wanted))
        .collect()
}

/// A composed seat, removed when the hurdle drops it.
pub struct bktu_Lure {
    /// The hurdle's own name for this lure.
    ///
    /// KEPT SO THAT EVERY SEAT COMPOSED BESIDE THE LURE IS THIS HURDLE'S. The
    /// repository, the output pair and the temp root were already keyed on it;
    /// the served registry and the tackroom were keyed on a CONSTANT instead,
    /// which made two hurdles composing a kibble share both directories — and
    /// the harness runs them in parallel, so one removed and re-tarred the
    /// archive the other was midway through converging. Driven, and it presented
    /// as a tar that could not open a file the seal had just read.
    name: String,
    root: PathBuf,
    output: PathBuf,
    temp: PathBuf,
    loosebox: PathBuf,
    moorings: PathBuf,
}

impl bktu_Lure {
    /// Compose a lure: a git repository under the temp root, carrying a pin and
    /// one commit.
    ///
    /// The name is the hurdle's, so a directory left behind by a panicking test
    /// says which hurdle made it.
    pub fn bktu_compose(name: &str) -> bktu_Lure {
        let declared = std::env::var(BKTU_TEMP_ROOT_VAR).unwrap_or_else(|_| {
            panic!(
                "{} is unset. The kennel's hurdles stand under the dispatch's temp root and \
                 reach for no ambient temp directory; drive the suite through its own door",
                BKTU_TEMP_ROOT_VAR
            )
        });

        let root = Path::new(&declared).join(format!("bktu-lure-{}", name));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root)
            .unwrap_or_else(|err| panic!("could not compose a lure at {}: {}", root.display(), err));

        // A SEAT HAS AN OUTPUT DIRECTORY, so the lure composes one rather than
        // letting a drive inherit the session's. A door that banks a fact file
        // writes into the two directories a dispatch names, and a child that
        // inherited them would write its hurdle's measurements into the
        // directories of the run executing it — asserting around whatever else
        // stood there, and leaving its seedings behind. THE PAIR STANDS OUTSIDE
        // THE REPOSITORY, because everything inside it is tracked ground the
        // door law reads.
        //
        // THE PAIR STANDS BENEATH ONE LURE-LOCAL SEAT, and the extra level is
        // what makes this lure a faithful dispatch rather than an approximate
        // one. A dispatch hands a door a per-invocation directory beneath a
        // per-seat root that outlives it, and a door keeping its own memory
        // across invocations — the delouse's day among them — stands that memory
        // at the root. A lure handing the root itself would put every lure's
        // memory in the directory of the run executing them, where they would
        // read each other's and race.
        let seat = Path::new(&declared).join(format!("bktu-seat-{}", name));
        let output = seat.join("out");
        let temp = seat.join("temp");
        // SPELLED, NOT CITED, and the position is why: this file is compiled a
        // second time as a module of the integration-test crate by `#[path]`,
        // where the kennel's breviary is another crate's and unreachable by
        // `crate::`, and the lure imports nothing of the kennel by design. A
        // literal standing outside the declaring crate is the printed-string
        // rule's third position, owed nothing until the twins are published.
        let loosebox = seat.join("loosebox");

        // A SEAT HAS A CONFIG DIRECTORY TOO, and it stands OUTSIDE the
        // repository with the other two — which is the shape being proven and
        // not a convenience. A dispatched seat's moorings stands in the outspan,
        // apart from the work billet the trampoline enters, so a lure whose
        // moorings sat inside its own repository would be proving a geometry
        // nothing uses; it would also be untracked ground inside a tree every
        // door law reading walks, so the first hurdle to write a whereabouts
        // into it would refuse its own drive.
        let moorings = Path::new(&declared).join(format!("bktu-buk-{}", name));

        for seat in [&output, &temp, &loosebox, &moorings] {
            let _ = std::fs::remove_dir_all(seat);
            std::fs::create_dir_all(seat).unwrap_or_else(|err| {
                panic!("could not compose a lure's seat at {}: {}", seat.display(), err)
            });
        }

        let lure = bktu_Lure {
            name: name.to_string(),
            root,
            output,
            temp,
            loosebox,
            moorings,
        };

        lure.bktu_write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.90.0\"\n");

        lure.bktu_git(&["init", "--quiet"]);
        lure.bktu_git(&["config", "user.email", "kennel@example.invalid"]);
        lure.bktu_git(&["config", "user.name", "kennel hurdle"]);
        lure.bktu_commit("compose the lure");

        lure
    }

    /// Where the lure stands.
    pub fn bktu_root(&self) -> &Path {
        &self.root
    }

    /// Where a drive over this lure banks a fact file: the seat's output
    /// directory, and the durable twin beside it.
    ///
    /// A HURDLE READS THE RECORD HERE, and it is the same pair every drive is
    /// handed, so a hurdle asserting over a file is looking exactly where the
    /// child was told to write it.
    #[allow(dead_code)]
    pub fn bktu_output(&self) -> &Path {
        &self.output
    }

    /// The durable twin of the output directory above.
    #[allow(dead_code)]
    pub fn bktu_temp(&self) -> &Path {
        &self.temp
    }

    /// This lure's own loosebox — the seat-level directory a door keeps memory
    /// in across invocations, handed to every drive over this lure.
    ///
    /// IT IS A SIBLING OF THE PAIR ABOVE AND NOT THEIR PARENT. A door reads the
    /// loosebox from its own variable and climbs out of nothing, so a lure
    /// handing the seat itself would let a reading that still climbed pass.
    #[allow(dead_code)]
    pub fn bktu_loosebox(&self) -> &Path {
        &self.loosebox
    }

    /// This lure's own moorings — the config directory every drive over it is
    /// handed, and where a hurdle plants a whereabouts to pose a dispatched
    /// seat.
    ///
    /// IT STANDS OUTSIDE THE REPOSITORY, which is what a dispatched seat's
    /// moorings does: the seat is an ephemeral directory apart from the trees it
    /// works, and the trampoline enters the work tree from it. A hurdle planting
    /// a geography here is therefore composing the real shape rather than a
    /// stand-in for it.
    #[allow(dead_code)]
    pub fn bktu_moorings(&self) -> &Path {
        &self.moorings
    }

    /// Run the kennel as a child over this lure and hand back what it said.
    ///
    /// A hurdle that SPAWNS the kennel reaches it here, exactly as one that
    /// composes a seat does: this is the only home, and a bare `Command` in a
    /// hurdle would be the second one.
    ///
    /// THE CALLER NAMES THE BINARY, and it must be the one cargo just built —
    /// `CARGO_BIN_EXE_<bin>`, which cargo exports to an integration test and to
    /// nothing else. It is a parameter rather than a reading taken here because
    /// that export is out of scope where this file compiles as a unit-test
    /// module of the lib, and a surface reaching for a name on the path would
    /// judge some artifact other than the run's own.
    ///
    /// The lure is reached by the child's WORKING DIRECTORY, which is how a door
    /// finds the repository it is standing in.
    ///
    /// `asked` is what the child is told, a door word and whatever that door
    /// takes. It is a slice rather than an option because a bare drive — the
    /// kennel asked to report its own making — is one value of it and not a
    /// separate act, and two entry points would let the two drift.
    ///
    /// `dead_code` is allowed because this method's callers stand in the
    /// integration seat, which compiles this file as a crate of its own; the lib
    /// test build compiles it too and reaches none of them.
    #[allow(dead_code)]
    pub fn bktu_drive(&self, kennel: &Path) -> Output {
        self.bktu_drive_door(kennel, &[])
    }

    /// Run the kennel as a child over this lure, spelling a door and whatever it
    /// takes, and hand back what it said.
    ///
    /// THE SAME SPAWN AS ABOVE, with the arguments a caller would type. The bare
    /// drive asks the kennel about itself and is the narrowest case of this one
    /// rather than a second road to a child, which is why it delegates: a second
    /// `Command` in this file would be the thing the surface exists to prevent,
    /// one file down.
    #[allow(dead_code)]
    pub fn bktu_drive_door(&self, kennel: &Path, door: &[&str]) -> Output {
        self.bktu_drive_stating(kennel, door, &[])
    }

    /// Run the kennel as a child over this lure, STATING what it is asked and
    /// what environment it stands in, and hand back what it said.
    ///
    /// THE ONE SPAWN, which is why both faces above delegate to it: a hurdle
    /// planting an environment and one planting nothing must reach the door the
    /// same way, or the plant is being compared against a differently-launched
    /// control.
    ///
    /// THE SEAT'S OWN OUTPUT PAIR IS STATED ON EVERY DRIVE, ahead of whatever the
    /// caller states, so a door that banks a fact file writes into this lure's
    /// directories rather than into the ones the run executing the hurdle is
    /// using. A caller may still state either name itself — the last statement
    /// wins — which is how a hurdle proves what a door does when a directory is
    /// absent or already carries the file.
    ///
    /// WHAT A HURDLE STATES HERE IS THE STATION AS THE DOOR WILL SEE IT. A
    /// mismatch or an absence is a fact about the programs a station holds, and
    /// the only honest way to compose one is to hand the child a station that
    /// holds them differently — never by reaching into the kennel and telling it
    /// something untrue about the one it is standing on.
    #[allow(dead_code)]
    pub fn bktu_drive_stating(
        &self,
        kennel: &Path,
        args: &[&str],
        stated: &[(&str, &std::ffi::OsStr)],
    ) -> Output {
        let mut child = Command::new(kennel);
        child.current_dir(&self.root).args(args);

        // THE STRIP COMES FIRST AND IT IS NARROW ON PURPOSE. What a spawn must
        // not inherit is the two channels that would make this hurdle's verdict
        // a function of which tabtarget drove the suite: the CONFIG DIRECTORY,
        // which is where the kennel reads its geography, and the FOLIO, which is
        // a caller's own argument. Everything else the dispatch exports is a
        // fact about the STATION the child genuinely stands on — the tackroom
        // above all, which is the toolchain fence every cargo invocation is
        // required to inherit — and a prefix-wide sweep here would sever it. The
        // dispatch surface below sweeps wide precisely because the seat it
        // enters re-establishes its own; a bare kennel spawn establishes
        // nothing, so it is handed what it needs instead.
        zbktu_strip(&mut child, ZBKTU_SPAWN_STRIPPED);

        child.env(BKTU_OUTPUT_DIR_VAR, &self.output);
        child.env(BKTU_TEMP_ROOT_VAR, &self.temp);
        child.env(BKTU_LOOSEBOX_DIR_VAR, &self.loosebox);
        child.env(BKTU_NOW_VAR, BKTU_NOW);
        child.env(BKTU_CONFIG_DIR_VAR, &self.moorings);

        for (name, value) in stated {
            child.env(name, value);
        }

        child.output().unwrap_or_else(|err| {
            panic!("could not run {} over the lure: {}", kennel.display(), err)
        })
    }

    /// This station's own PATH with a `cargo-nextest` OF THE HURDLE'S CHOOSING
    /// standing ahead of it.
    ///
    /// THE PLANT IS A PROGRAM, NOT A CLAIM. What a station holds is decided by
    /// what answers when a program is spawned, so a hurdle proving the kennel
    /// reads a version composes a program that answers one — the same road a
    /// station reaches a differently-versioned nextest by.
    ///
    /// AHEAD OF THE STATION'S PATH IS ENOUGH, AND THAT IS A FACT ABOUT THE
    /// FENCE. Cargo looks for a subcommand in `CARGO_HOME/bin` before it looks
    /// on the path, and under the fence `CARGO_HOME` is the tackroom's own — in
    /// which no nextest stands, the estate's nextest collars reaching one on the
    /// path. So a shim at the front of the path is what the child finds. A
    /// station that had installed nextest into its cargo home would need the
    /// plant there instead, and this face would then be proving nothing; the
    /// hurdles below asserting the SHIM'S OWN version rather than merely a
    /// refusal are what would catch that.
    #[allow(dead_code)]
    pub fn bktu_shimmed(&self, name: &str, version: &str) -> std::ffi::OsString {
        let ahead = self.zbktu_bin(name);

        zbktu_door(
            &ahead.join(ZBKTU_NEXTEST),
            &format!("#!/bin/sh\necho \"cargo-nextest {} (planted by a hurdle)\"\n", version),
        );

        let mut elements: Vec<PathBuf> = vec![ahead];

        if let Some(standing) = std::env::var_os("PATH") {
            elements.extend(std::env::split_paths(&standing));
        }

        std::env::join_paths(elements).expect("a composed path element carries no separator")
    }

    /// A PATH carrying only the programs a door needs, AND NO NEXTEST — the
    /// station that never installed one.
    ///
    /// TWO MOVES, AND BOTH ARE NEEDED. Every directory of the station's own path
    /// that HOLDS a nextest is dropped — which is the narrowing itself — and the
    /// programs named in `carried` are re-exposed ahead of what remains, as
    /// scripts that exec the file they resolved to. The second move is forced by
    /// the first: on this station nextest stands in the very directory cargo
    /// does, so a path that merely dropped directories would drop cargo with it
    /// and prove a station with no cargo rather than one with no nextest.
    ///
    /// WHAT REMAINS OF THE STATION'S PATH IS KEPT rather than replaced by the
    /// carried list, because a suite that actually RUNS needs more than the
    /// programs the kennel spawns — a linker, an assembler, whatever the
    /// toolchain reaches for. The control drive is the one that would find that
    /// out, and it should find out by running rather than by a hurdle author
    /// guessing the list.
    #[allow(dead_code)]
    pub fn bktu_nextestless(&self, name: &str, carried: &[&str]) -> std::ffi::OsString {
        let only = self.zbktu_bin(name);

        for program in carried {
            let found = zbktu_resolved(program).unwrap_or_else(|| {
                panic!(
                    "'{}' stands nowhere on this station's path, so a hurdle cannot carry it \
                     forward into a narrowed one",
                    program
                )
            });

            zbktu_door(
                &only.join(program),
                &format!("#!/bin/sh\nexec {} \"$@\"\n", found.display()),
            );
        }

        let mut elements: Vec<PathBuf> = vec![only];

        if let Some(standing) = std::env::var_os("PATH") {
            elements.extend(
                std::env::split_paths(&standing)
                    .filter(|directory| !directory.join(ZBKTU_NEXTEST).is_file()),
            );
        }

        std::env::join_paths(elements).expect("a composed path element carries no separator")
    }

    /// A directory of composed programs, made and answered.
    ///
    /// It stands OUTSIDE the git repository the lure is — under the temp root
    /// beside it rather than within it — because everything the lure holds is
    /// committed and a door refuses an uncommitted tree. A bin directory written
    /// into the lure would dirty the very repository the hurdle then drives a
    /// door over, and the refusal met would be the door law's rather than the
    /// one under test.
    ///
    /// It outlives the lure's own removal, standing beside rather than beneath
    /// it, and that is accepted: it is a few hundred bytes under the dispatch's
    /// own temp root, which the dispatch clears.
    fn zbktu_bin(&self, name: &str) -> PathBuf {
        let bin = self
            .root
            .parent()
            .unwrap_or(&self.root)
            .join(format!("bktu-bin-{}", name));
        let _ = std::fs::remove_dir_all(&bin);
        std::fs::create_dir_all(&bin)
            .unwrap_or_else(|err| panic!("could not make {}: {}", bin.display(), err));
        bin
    }

    /// The collar names a `bktu_collar_seat` lays down: one crate, reached under
    /// each of the two admitted runners.
    ///
    /// Named here rather than spelled at each hurdle so that a hurdle asserting
    /// against a collar and the surface composing it cannot come to disagree
    /// about what it is called.
    #[allow(dead_code)]
    pub const BKTU_COLLAR_NEXTEST: &'static str = "suite-lure-nextest";
    #[allow(dead_code)]
    pub const BKTU_COLLAR_CARGO: &'static str = "suite-lure-cargo";

    /// The one case the seat's crate carries, on the same rule as the collars
    /// above: a hurdle asserting that the console named a case — or that it did
    /// not — asks about THIS name, and spelling it at the hurdle would let the
    /// two drift until the assertion passed over a console that had gone quiet
    /// about something else entirely.
    #[allow(dead_code)]
    pub const BKTU_CASE: &'static str = "bktu_the_seat_carries_one_case";

    /// Lay a launchable down in this lure: one trivial crate, and a suite collar
    /// over it under each admitted runner.
    ///
    /// WHAT THIS COMPOSES IS A RESOLVABLE COLLAR, which is more than a file of
    /// declarations: every door validates through the resolver, and the resolver
    /// asks cargo about the manifest, elects the closure against the roots, and
    /// wants the declared muzzle directory to hold a lint list. A seat missing
    /// any of those refuses at the findings and a hurdle beyond it would be
    /// reading the wrong refusal.
    ///
    /// THE CRATE CARRIES NO DEPENDENCY AND ITS LOCK IS WRITTEN BY HAND. The
    /// leash passes `--locked` at every invocation, and the estate holds no road
    /// to regenerate a lock; for a crate with an empty closure the file is three
    /// lines and deterministic, which is why the seat can carry one at all.
    ///
    /// THE TWO COLLARS DIFFER IN THEIR RUNNER AND IN NOTHING ELSE, which is what
    /// makes the cargo one a control: a refusal met at the nextest collar and not
    /// at this one is about the runner rather than about the seat around it.
    #[allow(dead_code)]
    pub fn bktu_collar_seat(&self) {
        self.bktu_write(
            "Tools/lure/Cargo.toml",
            "[package]\nname = \"lure\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\n\
             [dependencies]\n",
        );
        // ONE REAL CASE, because a green course must now have RUN something: a
        // suite that counted no case is red at every door on the operator's
        // ruling, so a seat whose crate declared nothing could no longer serve as
        // any hurdle's "this runs green" control. The case asserts a tautology on
        // purpose — what it exists to establish is that a case EXISTS, not
        // anything about the lure.
        self.bktu_write(
            "Tools/lure/src/lib.rs",
            &format!(
                "// the lure's own crate\n\
                 #[test]\n\
                 fn {}() {{\n\
                 \x20   assert!(true, \"a green course must have run something\");\n\
                 }}\n",
                Self::BKTU_CASE
            ),
        );
        self.bktu_write(
            "Tools/lure/Cargo.lock",
            "version = 4\n\n[[package]]\nname = \"lure\"\nversion = \"0.0.1\"\n",
        );

        // The linter's own file, which a declared muzzle directory must hold. It
        // is empty because what the muzzle would find in this crate is nothing;
        // what the resolver asks is that the file stand.
        self.bktu_write("clippy.toml", "");

        // BUILD OUTPUT IS IGNORED, as it is in every repository this kennel runs
        // over. The door law refuses an UNCOMMITTED repository, and a hurdle
        // that drives twice over one seat builds on the first drive and would
        // meet its own artifact as untracked work on the second — a refusal
        // about the hurdle's own leavings rather than about anything under test.
        // The estate's own trees ignore it; a lure that did not was the odd one.
        self.bktu_write(".gitignore", "target/\n");

        for (collar, runner, tongue) in [
            (Self::BKTU_COLLAR_NEXTEST, "bknre_nextest", "bknre_nextest"),
            (Self::BKTU_COLLAR_CARGO, "bknre_cargo", "bknre_harness"),
        ] {
            self.bktu_write(
                &format!("{}/bkrr.env", collar),
                &format!(
                    "BKRR_COLLAR=\"{}\"\n\
                     BKRR_KIND=\"bknre_suite\"\n\
                     BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"\n\
                     BKRR_TARGET=\"bknre_manifest\"\n\
                     BKRR_ROOTS=\"Tools/lure/src Tools/lure/Cargo.toml Tools/lure/Cargo.lock\"\n\
                     BKRR_FEATURES=\"\"\n\
                     BKRR_PROFILE=\"test\"\n\
                     BKRR_SPEND=\"bknre_reader\"\n\
                     BKRR_TAMED_CRATES=\"\"\n\
                     BKRR_FERAL_CRATES=\"bknre_tamed\"\n\
                     BKRR_MUZZLE=\".\"\nBKRR_EXERGUE=\"bknre_unstruck\"\n\
                     BKRR_RUNNER=\"{}\"\n\
                     BKRR_TONGUE=\"{}\"\n",
                    collar, runner, tongue
                ),
            );
        }

        self.bktu_commit("lay a launchable down");
    }

    /// Lay one crate down at the named directory with a conforming collar over
    /// it — `at` empty meaning the lure's own root.
    ///
    /// THE COLLAR IS COMPOSED HERE AND NOT IN A HURDLE, on this surface's own
    /// rule: a hurdle that spelled a collar would be a second statement of what
    /// the resolver demands, free to drift from the first the next time a field
    /// is added. What a hurdle supplies is the shape it needs — where the crate
    /// stands and what the collar is called — and never the conformance.
    #[allow(dead_code)]
    pub fn bktu_crate(&self, at: &str, collar: &str) {
        let seat = |leaf: &str| {
            if at.is_empty() {
                leaf.to_string()
            } else {
                format!("{}/{}", at, leaf)
            }
        };

        self.bktu_write(
            &seat("Cargo.toml"),
            "[package]\nname = \"lure\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\
             [lib]\nname = \"lure\"\npath = \"src/lib.rs\"\n",
        );
        self.bktu_write(
            &seat("Cargo.lock"),
            "# This file is automatically @generated by Cargo.\n\
             # It is not intended for manual editing.\n\
             version = 4\n\n\
             [[package]]\nname = \"lure\"\nversion = \"0.0.1\"\n",
        );
        self.bktu_write(&seat("src/lib.rs"), "pub fn seated() -> u32 {\n    7\n}\n");
        self.bktu_write("clippy.toml", "disallowed-methods = []\n");

        let roots = if at.is_empty() {
            "src \\\n  Cargo.toml".to_string()
        } else {
            format!("{}/src \\\n  {}/Cargo.toml", at, at)
        };

        self.bktu_write(
            &format!("{}/bkrr.env", collar),
            &format!(
                "\
BKRR_COLLAR=\"{}\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"{}\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"{}\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"bknre_cargo\"
BKRR_TONGUE=\"bknre_harness\"
",
                collar,
                seat("Cargo.toml"),
                roots
            ),
        );
    }

    /// Lay a uv PROJECT down in this lure, with a python collar over it.
    ///
    /// WHAT IT WRITES IS A PROJECT AND NEVER AN ENVIRONMENT. The three files a
    /// uv project carries are text this surface can author; the venv they
    /// declare is uv's to build, and building one would put a network fetch
    /// inside a hermetic hurdle. So nothing here reaches the network, and what
    /// the hurdles over it prove is the READING — which is the whole of what
    /// this pace's validation does.
    ///
    /// THE COLLAR IS COMPOSED HERE AND NOT IN A HURDLE, on this surface's own
    /// rule: a hurdle that spelled a collar would be a second statement of what
    /// the resolver demands, free to drift from the first the next time a field
    /// is added. What a hurdle supplies is the shape it needs — where the
    /// project stands, what the collar is called, and which of the project's
    /// files it wants bent — and never the conformance.
    ///
    /// THE BENDS ARE PARAMETERS RATHER THAN A SECOND COMPOSER, because every
    /// refusal the settling sitting listed is one honest project with one thing
    /// wrong with it. A composer per refusal would be seven near-copies of this
    /// body, and the six that were not being edited would drift.
    #[allow(dead_code)]
    pub fn bktu_project(&self, at: &str, collar: &str, bent: &bktu_Bent) {
        let seat = |leaf: &str| {
            if at.is_empty() {
                leaf.to_string()
            } else {
                format!("{}/{}", at, leaf)
            }
        };

        let mut manifest = format!(
            "[project]\nname = \"lure\"\nversion = \"0.0.1\"\nrequires-python = \"{}\"\n",
            bent.requires
        );
        manifest.push_str(&bent.uv);

        self.bktu_write(&seat("pyproject.toml"), &manifest);

        if !bent.lockless {
            self.bktu_write(
                &seat("uv.lock"),
                "version = 1\nrequires-python = \">=3.12\"\n",
            );
        }

        self.bktu_write(&seat(".python-version"), &format!("{}\n", bent.pin));

        let roots = if at.is_empty() {
            "pyproject.toml".to_string()
        } else {
            format!("{}/pyproject.toml", at)
        };

        self.bktu_write(
            &format!("{}/bkrp.env", collar),
            &format!(
                "\
BKRP_COLLAR=\"{}\"
BKRP_KIND=\"bknre_suite\"
BKRP_MANIFEST=\"{}\"
BKRP_ROOTS=\"{}\"
BKRP_SPEND=\"bknre_reader\"
BKRP_TARGET=\"bknre_manifest\"
BKRP_RUNNER=\"bknre_pytest\"
BKRP_TONGUE=\"bknre_pytest\"
",
                collar,
                seat("pyproject.toml"),
                roots
            ),
        );
    }

    /// Lay a substrate seat down in this lure: a tree the substrate can be
    /// dispatched in, and hand back the tabtarget that enters it.
    ///
    /// What a seat must carry is the substrate's own law, and this restates none
    /// of it. What it composes is that law's answer for a tree standing under the
    /// temp root: the trampoline, the moorings with their regime and station
    /// files, the launcher stub, the coordinator whose body the hurdle supplies,
    /// and one tabtarget.
    ///
    /// THE SUBSTRATE IS REACHED BY PATH AND NEVER COPIED. `BURC_TOOLS_DIR` names
    /// the tools directory of the tree the hurdle is running in, so the kit under
    /// test is the one standing in that tree — the same bytes the suite was built
    /// from, rather than a copy free to drift from them.
    ///
    /// ONE DEVIATION FROM THE CANONICAL STUB, and it is forced. The stub's source
    /// line is repo-root-relative because a consumer's kit stands inside the
    /// consumer's own tree; a seat's does not, by the sentence above, so the line
    /// is rooted here. Every other line the law states is spelled as it stands.
    pub fn bktu_substrate_seat(&self, coordinator: &str) -> PathBuf {
        let substrate = zbktu_substrate();

        // The seat's own trampoline. It is the SOLE FILE THAT KNOWS THIS SEAT'S
        // moorings directory name, which is why a composed seat carries one of
        // its own rather than borrowing the estate's.
        self.bktu_write(
            "tt/z-launcher.sh",
            &format!(
                "#!/bin/bash\n\
                 # The seat's own tabtarget trampoline.\n\
                 set -u\n\
                 \n\
                 z_dir=\"${{BASH_SOURCE[0]%/*}}\"\n\
                 case \"${{z_dir}}\" in\n\
                 \x20 /*) ;;\n\
                 \x20 *)  z_dir=\"${{PWD}}/${{z_dir}}\" ;;\n\
                 esac\n\
                 \n\
                 test -n \"${{BURD_LAUNCHER:-}}\" || {{ echo \"z-launcher: BURD_LAUNCHER unset\" >&2; exit 1; }}\n\
                 \n\
                 z_moorings_dir=\"{moorings}\"\n\
                 z_launcher=\"${{z_dir}}/../${{z_moorings_dir}}/launchers/${{BURD_LAUNCHER}}\"\n\
                 \n\
                 test -f \"${{z_launcher}}\" || {{\n\
                 \x20 echo \"z-launcher: no launcher '${{BURD_LAUNCHER}}' (looked for ${{z_launcher}})\" >&2\n\
                 \x20 exit 1\n\
                 }}\n\
                 \n\
                 cd -P \"${{z_dir}}/..\" || {{ echo \"z-launcher: cannot cd to repo root\" >&2; exit 1; }}\n\
                 \n\
                 export BURD_CONFIG_DIR=\"${{PWD}}/${{z_moorings_dir}}\"\n\
                 \n\
                 exec \"${{z_launcher}}\" \"${{@}}\"\n",
                moorings = ZBKTU_MOORINGS
            ),
        );

        // The regime file. The temp, output and log roots stand INSIDE the lure:
        // a seat that borrowed the suite's own would write its record into the
        // tree under test and take that tree's temp directory for its scratch.
        self.bktu_write(
            &format!("{}/burc.env", ZBKTU_MOORINGS),
            &format!(
                "BURC_STATION_FILE={moorings}/burs.env\n\
                 BURC_TABTARGET_DIR=tt\n\
                 BURC_TABTARGET_DELIMITER=.\n\
                 BURC_TOOLS_DIR={substrate}\n\
                 BURC_PROJECT_ROOT=..\n\
                 BURC_MANAGED_KITS=buk\n\
                 BURC_TEMP_ROOT_DIR=temp\n\
                 BURC_OUTPUT_ROOT_DIR=output\n\
                 BURC_LOOSEBOX_ROOT_DIR=loosebox\n\
                 BURC_LOG_LAST=last\n\
                 BURC_LOG_EXT=txt\n",
                moorings = ZBKTU_MOORINGS,
                substrate = substrate.display()
            ),
        );

        // The station file the BURS regime names. A dispatch that cannot find one
        // refuses, which is the substrate's law rather than a defect to route
        // around, so the seat provides exactly the fields the regime enrolls.
        self.bktu_write(
            &format!("{}/burs.env", ZBKTU_MOORINGS),
            "BURS_LOG_DIR=logs\nBURS_USER=kennel\nBURS_TINCTURE=k\n",
        );

        // The launcher stub, in the shape the Launcher Stub Law states, with the
        // one rooted line the doc comment above accounts for. The coordinator is
        // consumer-owned and stands in the moorings beside the regime files, so
        // the stub reaches it through the config directory the trampoline just
        // exported rather than through `BURC_TOOLS_DIR`, which names the kit.
        self.bktu_write(
            &format!("{}/launchers/launcher.lure_workbench.sh", ZBKTU_MOORINGS),
            &format!(
                "#!/bin/bash\n\
                 # Launcher stub - delegates to lure workbench\n\
                 source \"{substrate}/buk/bul_launcher.sh\"\n\
                 bul_launch \"${{BURD_CONFIG_DIR}}/lure_workbench.sh\" \"$@\"\n",
                substrate = substrate.display()
            ),
        );

        // The coordinator: the seat's own formulary, and the hurdle's to write.
        // The dispatch hands it the colophon as its first argument.
        self.bktu_write(
            &format!("{}/lure_workbench.sh", ZBKTU_MOORINGS),
            coordinator,
        );

        // The door.
        self.bktu_write(
            ZBKTU_TABTARGET,
            "#!/bin/bash\n\
             export BURD_LAUNCHER=launcher.lure_workbench.sh\n\
             exec \"${BASH_SOURCE[0]%/*}/z-launcher.sh\" \"${0##*/}\" \"${@}\"\n",
        );

        // What the dispatch composes as it runs, which a seat keeps out of its own
        // commit exactly as a consumer tree keeps them out of its own.
        self.bktu_write(".gitignore", "logs/\noutput/\ntemp/\n");

        let doors = [
            "tt/z-launcher.sh".to_string(),
            format!("{}/launchers/launcher.lure_workbench.sh", ZBKTU_MOORINGS),
            format!("{}/lure_workbench.sh", ZBKTU_MOORINGS),
            ZBKTU_TABTARGET.to_string(),
        ];
        for door in doors {
            zbktu_chmod(&self.root.join(door));
        }

        self.bktu_commit("lay down the substrate seat");

        self.root.join(ZBKTU_TABTARGET)
    }

    /// Dispatch a tabtarget the seat composed and hand back what the child said.
    ///
    /// OBSERVED FROM OUTSIDE THE SHELL. The child's exit code and both its streams
    /// are read here, in a process the dispatch cannot reach, which is the posture
    /// a harness asserting on a bash process is required to hold
    /// (BKSOB-Obedience.adoc "Observation Posture").
    ///
    /// THE SEAT ESTABLISHES ITS OWN REGIMES. This process is itself running under
    /// a dispatch, so it carries that dispatch's `BURC_` and `BURD_` values and
    /// whatever `BURV_` overrides the session set. Inherited, they would point the
    /// seat's log family and temp root back at the tree the suite is running in —
    /// which is the one thing the seat composes its own for. `BURE_` passes
    /// untouched, being the substrate's operator-ambient channel by design.
    ///
    /// THE WORKING DIRECTORY IS DELIBERATELY NOT SET. Normalizing it to the repo
    /// root is the trampoline's own responsibility, so a dispatch driven from
    /// wherever the runner happens to stand is what proves the seat's trampoline
    /// does its job.
    pub fn bktu_dispatch(&self, tabtarget: &Path, args: &[&str]) -> Output {
        let mut child = Command::new(tabtarget);
        child.args(args);

        // THE STRIP IS BY PREFIX AND HAS TO REACH EVERY CHANNEL THE SUBSTRATE
        // SPEAKS ACROSS AN EXEC, not merely the regime ones. A composed seat is
        // a repository of its own and must answer for itself; a variable the
        // outer dispatch set that survives into it makes the hurdle's verdict a
        // function of which tabtarget happened to drive the suite.
        //
        // `BUZ_` is the one this list first missed, and it is the worst one to
        // miss: it is the ZIPPER'S FOLIO CHANNEL — how a tabtarget's imprint
        // reaches its coordinator — so what leaks through it is precisely a
        // CALLER'S ARGUMENT. Driving this suite as `tt/bkw-m.Mush.sh
        // suite-buk` put `BUZ_FOLIO=suite-buk` in this process's environment,
        // the composed seat's whistle read it as its own argument, and three
        // whistle hurdles went red against a refusal meant for somebody else.
        // The same suite driven through a door taking no imprint was green, so
        // the failure moved with the DRIVER rather than with the work — which is
        // the shape that makes a leak like this read as a real regression.
        zbktu_strip(&mut child, ZBKTU_SEAT_STRIPPED);

        child.output().unwrap_or_else(|err| {
            panic!("could not dispatch {}: {}", tabtarget.display(), err)
        })
    }

    /// The program the composed kibble seat declares, and its version.
    ///
    /// Named here rather than spelled at each hurdle so that a hurdle asserting
    /// against a kibble and the surface composing it cannot come to disagree
    /// about what it is called.
    #[allow(dead_code)]
    pub const BKTU_KIBBLE_PROGRAM: &'static str = "lure-fetched";
    #[allow(dead_code)]
    pub const BKTU_KIBBLE_VERSION: &'static str = "0.1.0";

    /// What this lure's composed kibble is called.
    ///
    /// PER-HURDLE, AND THAT IS WHAT KEEPS THE SHARED STORE HONEST. A converge
    /// hurdle lands a binary in the STATION'S OWN tackroom — it can do nothing
    /// else, since a composed store holds no toolchain and the fence refuses one
    /// that is not the store `CARGO_HOME` resolves into — so the residence a
    /// hurdle writes stands beside every other hurdle's. Keyed on the lure, two
    /// hurdles that both compose a kibble own different residences; keyed on a
    /// constant, one hurdle's converge would land at exactly the path another
    /// asserts is empty.
    #[allow(dead_code)]
    pub fn bktu_kibble_name(&self) -> String {
        format!("kibble-{}", self.name)
    }

    /// Lay a kibble down over an archive this lure SERVES ITSELF, and answer the
    /// tackroom a drive over it must be told.
    ///
    /// THE ARCHIVE IS LOCAL, AND THAT IS WHAT MAKES THE FETCH ARM HURDLE-ABLE.
    /// A kibble declares the base its archive stands under and the door joins
    /// the program and the version onto it; a `file://` base is a base like any
    /// other to curl, so the whole converge — fetch, seal, unpack, build, land —
    /// runs end to end with no registry reached and no network touched. A hurdle
    /// pointed at a real registry would be measuring crates.io's uptime.
    ///
    /// THE ARCHIVE IS COMPOSED RATHER THAN COMMITTED, and it stands OUTSIDE the
    /// lure's repository beside its other seats: everything inside the lure is
    /// tracked ground the door law reads, and a tarball written into it would
    /// dirty the very repository the hurdle then drives a door over.
    ///
    /// `seal` IS THE CALLER'S, which is the whole point of this face. A hurdle
    /// proving the converge declares the archive's true digest, and one proving
    /// the refusal declares a wrong one — the archive being byte-identical in
    /// both cases, so what the two drives differ in is the declaration alone.
    /// The true digest is answered by `bktu_kibble_seal` below.
    #[allow(dead_code)]
    pub fn bktu_kibble_seat(&self, seal: &str) -> PathBuf {
        let served = self.zbktu_served();
        let named = self.bktu_kibble_name();

        self.bktu_write(
            &format!("{}/bkrk.env", named),
            &format!(
                "BKRK_KIBBLE=\"{}\"\n\
                 BKRK_KIND=\"bknre_source\"\n\
                 BKRK_PROGRAM=\"{}\"\n\
                 BKRK_VERSION=\"{}\"\n\
                 BKRK_SEAL=\"{}\"\n\
                 BKRK_BYNAME=\"{}\"\n\
                 BKRK_REGISTRY=\"file://{}\"\n\
                 BKRK_CHANNEL=\"1.90.0\"\n\
                 BKRK_TARGET=\"{}\"\n\
                 BKRK_FEATURES=\"\"\n",
                named,
                Self::BKTU_KIBBLE_PROGRAM,
                Self::BKTU_KIBBLE_VERSION,
                seal,
                Self::BKTU_KIBBLE_PROGRAM,
                served.display(),
                Self::BKTU_KIBBLE_PROGRAM
            ),
        );

        self.bktu_commit("lay a kibble down");

        // THE STATION'S OWN TACKROOM, NEVER A COMPOSED ONE. A converge builds
        // what it fetched, and every build passes the leash, which refuses a
        // store the two cargo homes do not resolve into — so a hurdle handing
        // the door some other tackroom would be proving the FENCE and never
        // reaching a seal. What the hurdle owns instead is its own residence
        // under that store, named for the lure, which it clears to pose a cold
        // one. Nothing else in the store is touched.
        let tackroom = PathBuf::from(std::env::var(ZBKTU_TACKROOM_VAR).unwrap_or_else(|_| {
            panic!(
                "{} is unset. A kibble stands in the station's shared tackroom and a hurdle over \
                 one reaches the same store the door will; drive the suite through its own door",
                ZBKTU_TACKROOM_VAR
            )
        }));

        let _ = std::fs::remove_dir_all(
            tackroom.join(ZBKTU_QUARTER).join(&named),
        );

        tackroom
    }

    /// Lay THE ESTATE'S OWN nextest kibble down inside this lure, copied rather
    /// than restated.
    ///
    /// COPIED, BECAUSE THE PIN HAS ONE HOME. A lure that spelled the version and
    /// the seal itself would be a second declaration of both, free to drift from
    /// the one the estate builds against the moment either is bumped — and the
    /// hurdle would then be proving that the kennel can reach a kibble the
    /// hurdle invented. What is copied is the file, so a drive over this lure
    /// resolves the same declaration a drive over the estate does, and the
    /// residence it composes is the one heel already wrote.
    ///
    /// THE DRIVE MUST THEREFORE BE TOLD THE SESSION'S OWN TACKROOM, which is
    /// where that residence stands. A hurdle stating a composed one instead
    /// would find it cold and would be measuring a converge rather than a
    /// launch.
    #[allow(dead_code)]
    pub fn bktu_nextest_seat(&self) {
        let declared = self.zbktu_nextest_declaration();

        self.bktu_write(
            &format!("{}/bkrk.env", ZBKTU_NEXTEST_KIBBLE),
            &declared,
        );
        self.bktu_commit("lay the estate's own nextest kibble down");
    }

    /// Lay the estate's own nextest kibble down WITH ITS VERSION MOVED, so its
    /// residence is one nothing has ever placed.
    ///
    /// THE UNPLACED STATE IS POSED BY MOVING THE DECLARATION, never by composing
    /// a cold tackroom. A drive told about a tackroom other than the fence's own
    /// meets the fence first — `CARGO_HOME` and `RUSTUP_HOME` still resolve into
    /// the real store, which is a posture violation and refuses before any
    /// kibble is read — so a hurdle that swapped the store would be proving the
    /// fence rather than the residence. Moving the version keeps the store, the
    /// fence and the toolchain exactly as they stand and changes only the path
    /// the pin composes, which IS the thing under test.
    ///
    /// The seal is left as the real one and is never reached: the residence test
    /// stands ahead of the fetch, so a drive over this seat refuses before
    /// anything is downloaded.
    #[allow(dead_code)]
    pub fn bktu_nextest_seat_unplaced(&self, version: &str) {
        let declared = self.zbktu_nextest_declaration();
        let standing = zbktu_declared(&declared, "BKRK_VERSION");

        assert_ne!(
            standing, version,
            "the unplaced seat must declare a version the estate does NOT, or the residence it \
             composes is the one heel already wrote"
        );

        self.bktu_write(
            &format!("{}/bkrk.env", ZBKTU_NEXTEST_KIBBLE),
            &declared.replace(
                &format!("BKRK_VERSION=\"{}\"", standing),
                &format!("BKRK_VERSION=\"{}\"", version),
            ),
        );
        self.bktu_commit("lay a nextest kibble down at a version nothing placed");
    }

    /// The estate's own nextest declaration, read from the tree this suite was
    /// built from.
    fn zbktu_nextest_declaration(&self) -> String {
        self.zbktu_declaration(ZBKTU_NEXTEST_KIBBLE)
    }

    /// Lay the estate's own uv kibble down, so a lure spawning uv composes the
    /// residence the STATION actually holds.
    ///
    /// COPIED RATHER THAN RESTATED, on the nextest seat's own ground and for the
    /// sharper reason: a hurdle that SPAWNS uv must reach the residence heel
    /// already wrote, which a fabricated version cannot name. Writing the real
    /// version into a hurdle instead would give the pin a second home, left
    /// behind saying something that used to be true at the next bump; copying
    /// the estate's own bytes keeps one home a bump reaches without anyone
    /// remembering this surface exists.
    ///
    /// THE VALIDATION HURDLES STILL FABRICATE, and the difference is what each
    /// proves: they prove what the READER does with a declaration and touch no
    /// station, so a real version would be a second home bought for nothing.
    #[allow(dead_code)]
    pub fn bktu_uv_seat(&self) {
        self.bktu_write(
            &format!("{}/bkrk.env", ZBKTU_UV_KIBBLE),
            &self.zbktu_declaration(ZBKTU_UV_KIBBLE),
        );
    }

    /// Lay the estate's own uv kibble down WITH ITS VERSION MOVED, so its
    /// residence is one nothing has ever placed.
    ///
    /// THE UNPLACED STATE IS POSED BY MOVING THE DECLARATION, on the nextest
    /// seat's precedent and for its exact reason: a drive told about a tackroom
    /// other than the fence's own meets the fence first, so a hurdle that
    /// swapped the store would be proving the fence rather than the residence.
    /// Moving the version keeps the store and the fence exactly as they stand
    /// and changes only the path the pin composes, which IS the thing under
    /// test.
    ///
    /// The seal is left as the real one and is never reached: the residence test
    /// stands ahead of the fetch, so a drive over this seat refuses before
    /// anything is downloaded.
    #[allow(dead_code)]
    pub fn bktu_uv_seat_unplaced(&self, version: &str) {
        let declared = self.zbktu_declaration(ZBKTU_UV_KIBBLE);
        let standing = zbktu_declared(&declared, "BKRK_VERSION");

        assert_ne!(
            standing, version,
            "the unplaced seat must declare a version the estate does NOT, or the residence it \
             composes is the one heel already placed"
        );

        self.bktu_write(
            &format!("{}/bkrk.env", ZBKTU_UV_KIBBLE),
            &declared.replace(
                &format!("BKRK_VERSION=\"{}\"", standing),
                &format!("BKRK_VERSION=\"{}\"", version),
            ),
        );
    }

    /// One of the estate's own kibble declarations, read from the tree this
    /// suite was built from.
    ///
    /// ONE READER FOR EVERY KIBBLE A LURE MIRRORS. A second copy per kibble
    /// would be a second place for the "reach the estate's bytes, never restate
    /// them" rule to be forgotten, and the copy that forgot it would be the one
    /// nobody was editing.
    fn zbktu_declaration(&self, kibble: &str) -> String {
        let standing = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(kibble)
            .join("bkrk.env");

        std::fs::read_to_string(&standing).unwrap_or_else(|err| {
            panic!(
                "could not read the estate's own {} kibble at {}: {} — a lure proving a launch \
                 reaches THAT declaration rather than one of its own",
                kibble,
                standing.display(),
                err
            )
        })
    }

    /// The true sha256 of the archive this lure serves, taken the way the door
    /// takes it.
    ///
    /// ASKED OF OPENSSL RATHER THAN REMEMBERED, because a digest written into a
    /// hurdle would have to be re-taken by hand every time the served crate's
    /// bytes changed — and a hurdle proving a CONVERGE would then fail for the
    /// reason the hurdle proving a REFUSAL exists to catch, which is the one
    /// confusion these two must never be able to make.
    #[allow(dead_code)]
    pub fn bktu_kibble_seal(&self) -> String {
        // THE ARCHIVE IS COMPOSED BEFORE IT IS HASHED, because a hurdle asks for
        // the digest in order to declare it and so reaches this face FIRST. The
        // composition is idempotent, so the seat laid down afterwards hashes the
        // same bytes.
        let _ = self.zbktu_served();
        let archive = self.zbktu_archive();

        let opened = std::fs::File::open(&archive)
            .unwrap_or_else(|err| panic!("could not read {}: {}", archive.display(), err));

        let out = Command::new("openssl")
            .arg("dgst")
            .arg("-sha256")
            .stdin(std::process::Stdio::from(opened))
            .output()
            .unwrap_or_else(|err| panic!("could not run openssl: {}", err));

        String::from_utf8_lossy(&out.stdout)
            .split_whitespace()
            .next_back()
            .unwrap_or_else(|| panic!("openssl said nothing this hurdle can read a digest out of"))
            .to_string()
    }

    /// Compose the served archive, and answer the directory that stands as the
    /// registry base.
    ///
    /// THE LAYOUT IS THE REGISTRY'S OWN, restated here so the door's composition
    /// finds what it composes: `<base>/<program>/<program>-<version>.crate`. A
    /// lure that laid the file down anywhere else would be proving that the door
    /// can fetch a path the lure happened to agree with rather than the one a
    /// registry publishes.
    ///
    /// THE CRATE IS THE SMALLEST THING THAT BUILDS A BINARY, because what the
    /// hurdle measures is the converge and not the compiler: a fetched crate
    /// with a dependency would price every drive at a resolve.
    fn zbktu_served(&self) -> PathBuf {
        let base = self
            .root
            .parent()
            .unwrap_or(&self.root)
            .join(format!("bktu-reg-{}", self.name));

        let unpacked = format!(
            "{}-{}",
            Self::BKTU_KIBBLE_PROGRAM,
            Self::BKTU_KIBBLE_VERSION
        );
        let staging = base.join("staging").join(&unpacked);

        // COMPOSED ONCE PER LURE, AND THE ONCE-NESS IS LOAD-BEARING. A hurdle
        // reads the archive's digest and then lays a kibble down declaring it,
        // so this face is reached twice; a second composition would re-tar the
        // same files at new modification times and answer a DIFFERENT digest,
        // and the declaration would then be wrong for the archive standing
        // beside it. The hurdle would fail as a seal refusal — which is exactly
        // the verdict the refusal hurdle exists to produce, and the one
        // confusion these two must never be able to make.
        if base
            .join(Self::BKTU_KIBBLE_PROGRAM)
            .join(format!("{}.crate", unpacked))
            .is_file()
        {
            return base;
        }

        std::fs::create_dir_all(staging.join("src"))
            .unwrap_or_else(|err| panic!("could not make {}: {}", staging.display(), err));

        zbktu_put(
            &staging.join("Cargo.toml"),
            &format!(
                "[package]\nname = \"{}\"\nversion = \"{}\"\nedition = \"2021\"\n\n\
                 [[bin]]\nname = \"{}\"\npath = \"src/main.rs\"\n\n[dependencies]\n",
                Self::BKTU_KIBBLE_PROGRAM,
                Self::BKTU_KIBBLE_VERSION,
                Self::BKTU_KIBBLE_PROGRAM
            ),
        );
        zbktu_put(
            &staging.join("Cargo.lock"),
            &format!(
                "version = 4\n\n[[package]]\nname = \"{}\"\nversion = \"{}\"\n",
                Self::BKTU_KIBBLE_PROGRAM,
                Self::BKTU_KIBBLE_VERSION
            ),
        );
        zbktu_put(
            &staging.join("src/main.rs"),
            "fn main() {\n    println!(\"the lure's own fetched program\");\n}\n",
        );

        let served = base.join(Self::BKTU_KIBBLE_PROGRAM);
        std::fs::create_dir_all(&served)
            .unwrap_or_else(|err| panic!("could not make {}: {}", served.display(), err));

        let out = Command::new("tar")
            .arg("-czf")
            .arg(served.join(format!("{}.crate", unpacked)))
            .arg("-C")
            .arg(base.join("staging"))
            .arg(&unpacked)
            .output()
            .unwrap_or_else(|err| panic!("could not run tar: {}", err));

        if !out.status.success() {
            panic!(
                "tar refused to compose the served archive: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }

        base
    }

    /// Where the served archive stands, for the digest reading above.
    fn zbktu_archive(&self) -> PathBuf {
        self.root
            .parent()
            .unwrap_or(&self.root)
            .join(format!("bktu-reg-{}", self.name))
            .join(Self::BKTU_KIBBLE_PROGRAM)
            .join(format!(
                "{}-{}.crate",
                Self::BKTU_KIBBLE_PROGRAM,
                Self::BKTU_KIBBLE_VERSION
            ))
    }

    /// Where a kibble converged over this lure lands its binary, composed the
    /// way the door composes it so a hurdle asserts against the same path.
    #[allow(dead_code)]
    pub fn bktu_kibble_residence(&self, tackroom: &Path) -> PathBuf {
        tackroom
            .join(ZBKTU_QUARTER)
            .join(self.bktu_kibble_name())
            .join(Self::BKTU_KIBBLE_VERSION)
            .join(Self::BKTU_KIBBLE_PROGRAM)
    }

    /// Write a file into this lure's MOORINGS, creating the directories above it,
    /// and hand back where it stands.
    ///
    /// THE MOORINGS IS NOT THE REPOSITORY, which is the whole reason this is a
    /// second method rather than a path a caller joins for itself: a whereabouts
    /// written into the tree would be untracked ground inside a repository every
    /// door law reading walks, and the hurdle that planted it would refuse its own
    /// drive. It is also the shape being proven — a dispatched seat's config
    /// directory stands apart from the tree its trampoline enters.
    #[allow(dead_code)]
    pub fn bktu_moor(&self, relative: &str, content: &str) -> PathBuf {
        let path = self.moorings.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|err| {
                panic!("could not compose {}: {}", parent.display(), err)
            });
        }
        std::fs::write(&path, content)
            .unwrap_or_else(|err| panic!("could not write {}: {}", path.display(), err));
        path
    }

    /// Write a file into the lure, creating the directories above it.
    pub fn bktu_write(&self, relative: &str, content: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .unwrap_or_else(|err| panic!("could not make {}: {}", parent.display(), err));
        }
        std::fs::write(&path, content)
            .unwrap_or_else(|err| panic!("could not write {}: {}", path.display(), err));
    }

    /// Stage everything standing and commit it.
    pub fn bktu_commit(&self, intent: &str) {
        self.bktu_git(&["add", "-A"]);
        self.bktu_git(&["commit", "--quiet", "--allow-empty", "-m", intent]);
    }

    /// Run git inside the lure, dying on a refusal — a hurdle whose ground could
    /// not be composed has proven nothing, and must not read as a pass.
    pub fn bktu_git(&self, args: &[&str]) {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .args(args)
            .output()
            .unwrap_or_else(|err| panic!("could not run git {:?}: {}", args, err));

        if !out.status.success() {
            panic!(
                "git {:?} refused in the lure at {}: {}",
                args,
                self.root.display(),
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
    }
}

/// The substrate a seat is composed against: the tools directory of the tree this
/// suite was built from, which is the tree the hurdle is running in.
///
/// Reached by path from the crate's own manifest, so the kit under test is the
/// one standing in that tree and never a copy the kennel holds. A tree missing
/// the shared launcher refuses here, naming the path, rather than failing several
/// processes deep in bash with a message about something else.
/// Take the named families off a child's environment.
///
/// THE ONE PLACE A STRIP IS SPELLED, which is why the two spawns hand it a roster
/// rather than each walking the environment for themselves: what must come off
/// differs by what the child re-establishes, but the act does not, and two walks
/// would be two places for a family to be forgotten. Prefixes, so an exact name
/// is its own family of one.
fn zbktu_strip(child: &mut Command, families: &[&str]) {
    for (name, _) in std::env::vars() {
        if families.iter().any(|family| name.starts_with(family)) {
            child.env_remove(name);
        }
    }
}

fn zbktu_substrate() -> PathBuf {
    // TWO SEGMENTS UP, not one: the kennel's manifest stands at the crate's own
    // address, the address stands beneath the kit directory, and the kit
    // directory is what a tools directory carries. The kit directory is a
    // delivery name and constrains nothing, so the walk counts segments rather
    // than reading either name.
    let substrate = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| {
            panic!(
                "the kennel's manifest stands at an address beneath a kit directory, \
                 and that kit directory under a tools directory: {}",
                env!("CARGO_MANIFEST_DIR")
            )
        })
        .to_path_buf();

    let launcher = substrate.join("buk").join("bul_launcher.sh");
    assert!(
        launcher.is_file(),
        "no substrate stands at {}: a seat is composed against the kit in the tree \
         the hurdle runs in, and this tree carries none",
        launcher.display()
    );

    substrate
}

/// Arm a composed door with the executable bit.
///
/// The trampoline, the launcher stub, the coordinator and the tabtarget are all
/// EXEC'd rather than sourced, so a seat whose doors are not armed refuses deep
/// inside the dispatch with a message about the wrong thing.
#[cfg(unix)]
fn zbktu_chmod(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut mode = std::fs::metadata(path)
        .unwrap_or_else(|err| panic!("could not read {}: {}", path.display(), err))
        .permissions();
    mode.set_mode(0o755);
    std::fs::set_permissions(path, mode)
        .unwrap_or_else(|err| panic!("could not arm {}: {}", path.display(), err));
}

/// Where the platform carries no executable bit, a door is reached by its shebang
/// and nothing is owed here.
#[cfg(not(unix))]
fn zbktu_chmod(_path: &Path) {}

/// One field's value out of a regime file's text.
///
/// A HURDLE'S OWN NARROW READING, deliberately not the reader crate's. What this
/// serves is a REWRITE — the value is wanted so the same assignment can be
/// spelled again with a different one — and the reader crate answers with the
/// joined value rather than with the line, which is not what a substitution
/// needs.
fn zbktu_declared(text: &str, field: &str) -> String {
    text.lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix(&format!("{}=", field))
                .map(|rest| rest.trim().trim_matches('"').to_string())
        })
        .unwrap_or_else(|| panic!("the declaration carries no {}", field))
}

/// Write a composed file OUTSIDE the lure's repository, creating the
/// directories above it.
///
/// Distinct from `bktu_write`, which writes INTO the lure: everything there is
/// tracked ground the door law reads, and the seats composed beside the lure —
/// its served registry, its tackroom — must never be.
fn zbktu_put(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|err| panic!("could not make {}: {}", parent.display(), err));
    }
    std::fs::write(path, body)
        .unwrap_or_else(|err| panic!("could not compose {}: {}", path.display(), err));
}

/// Write a composed program and arm it.
fn zbktu_door(path: &Path, body: &str) {
    std::fs::write(path, body)
        .unwrap_or_else(|err| panic!("could not compose {}: {}", path.display(), err));
    zbktu_chmod(path);
}

/// Where one program stands on THIS station's own path.
///
/// The walk is the shell's own: each element of `PATH` in order, the first file
/// that stands wins. Written here rather than shelled out to, because a hurdle
/// asking a shell where a program is would be asking the very lookup it is about
/// to narrow.
fn zbktu_resolved(program: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;

    std::env::split_paths(&path)
        .map(|directory| directory.join(program))
        .find(|candidate| candidate.is_file())
}

impl Drop for bktu_Lure {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);

        // THE MOORINGS STANDS APART FROM THE SEAT and is taken by name. It is
        // composed beside the pair rather than beneath it, so the seat's own
        // removal below does not reach it, and a lure that left one behind would
        // strew the dispatch's temp root with config directories nothing reads.
        let _ = std::fs::remove_dir_all(&self.moorings);

        // THE SEAT GOES WHOLE, which takes the output directory and the temp
        // directory with it and whatever a door left standing at the root
        // between them — a door's own memory being exactly such a thing.
        match self.temp.parent() {
            Some(seat) => {
                let _ = std::fs::remove_dir_all(seat);
            }
            None => {
                let _ = std::fs::remove_dir_all(&self.output);
                let _ = std::fs::remove_dir_all(&self.temp);
            }
        }
    }
}

/// What one hurdle bends about an otherwise conforming uv project.
///
/// THE DEFAULT IS THE CONFORMING PROJECT, so a hurdle states the ONE thing it
/// is about and the reader sees that one thing rather than a seven-field
/// literal with six fields the same as its neighbours'.
#[allow(dead_code)]
pub struct bktu_Bent {
    /// What `.python-version` carries.
    pub pin: String,
    /// What the manifest's `requires-python` carries.
    pub requires: String,
    /// Whatever `[tool.uv]` text the project declares, appended whole.
    pub uv: String,
    /// Whether the lock file is withheld.
    pub lockless: bool,
}

impl Default for bktu_Bent {
    fn default() -> Self {
        bktu_Bent {
            pin: "3.12.7".to_string(),
            requires: ">=3.12".to_string(),
            uv: String::new(),
            lockless: false,
        }
    }
}

// eof
