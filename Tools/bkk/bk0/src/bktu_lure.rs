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

//! The composing surface for a lure.
//!
//! A lure is a seat composed for one hurdle: a git repository under the
//! dispatch's temp root, carrying a commit, a pin, and whatever else the hurdle
//! asks for. The kennel is driven over it exactly as over a real seat.
//!
//! A LURE IS COMPOSED, NEVER COMMITTED to the tree its hurdles run in. A lure is
//! a repository of its own — its own door law, its own geography, its own
//! moorings and weaned set — and a fixture standing in the host tree would be
//! host ground every one of those readings takes.
//!
//! THIS IS THE ONLY HOME. A hurdle that reaches the temp root, composes a
//! repository, or spawns the kennel reaches it here — never through a bare
//! `std::env` or a `git` of its own. The founding surface is deliberately small:
//! it composes and it cleans up, and it grows as hurdles ask it to.
//!
//! A PROGRAM IS WRITTEN BY A CHILD, NEVER BY THIS PROCESS. Hurdles run as
//! threads of one process, and every spawn below forks it, so every file a
//! hurdle means to run is written or copied by Pastern's program writers
//! (`bkp::bkprs_scrivener`), whose module states the race and why a child closes
//! it. The kennel's muzzle bars the calls that arm or copy a file in-process,
//! and `bktu_write` refuses a file already armed.
//!
//! TWO SHAPES ARE COMPOSED HERE. The bare lure above is one. The other is a
//! substrate seat laid down inside it: the trampoline, moorings, station file,
//! launcher stub, coordinator and tabtarget a dispatch needs, so the substrate
//! can be driven through its own door rather than sourced bare. A hurdle asking
//! for one gets it from here for the same reason it gets a lure from here.

use bkp::bkprs_scrivener::bkprs_program_write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The dispatch's temp root. ABSENT IS A REFUSAL, never a fallback: a surface
/// that cannot find this variable refuses the hurdle naming it and reaches for
/// no ambient temp directory, so the kennel's hurdles are always launched
/// through the substrate.
pub const BKTU_TEMP_ROOT_VAR: &str = "BURD_TEMP_DIR";

/// The substrate's declaration that a run keeps NO record, stated on every
/// spawn below.
///
/// A HURDLE-SPAWNED KENNEL KEEPS NO RECORD, and saying so is honest rather
/// than a softening. The spawn hands the child this lure's temp root, which is
/// the station fact it genuinely stands on, and composes no log family, so the
/// child keeps no record whatever else it is told. The declaration stands
/// because the harness stands in for a tabtarget, and a tabtarget is what
/// declares a log mode: this is the substrate's own word that the run asked
/// for none.
pub const BKTU_NO_LOG_VAR: &str = "BURD_NO_LOG";

/// The dispatch's output directory, which a drive is handed a lure-local one of
/// rather than inheriting the session's.
pub const BKTU_OUTPUT_DIR_VAR: &str = "BURD_OUTPUT_DIR";

/// The checkout's own loosebox, where a door keeps memory that must outlive the
/// per-invocation directory — the delouse's day among it. A drive is handed a
/// lure-local one for the same reason it is handed the pair above: a child
/// inheriting the session's would keep its memory where the run executing it
/// keeps its own.
pub const BKTU_LOOSEBOX_DIR_VAR: &str = "BURD_LOOSEBOX_DIR";

/// The dispatch's config directory, which a bare kennel spawn does not inherit
/// and is handed this lure's own in its place.
///
/// THE KENNEL READS ITS WEANED SET THERE, so a child that inherited the
/// driver's config directory would run the driver's declarations over the
/// lure's tree without any hurdle noticing. The strip keeps the driver's out and
/// the spawn states the lure's moorings, committed inside the lure's own
/// repository the way every consumer keeps its moorings.
pub const BKTU_CONFIG_DIR_VAR: &str = "BURD_CONFIG_DIR";

/// What a BARE KENNEL SPAWN does not inherit: the two channels that would make a
/// hurdle's verdict a function of which tabtarget drove the suite.
///
/// NARROW ON PURPOSE. Everything else a dispatch exports is a fact about the
/// STATION the child genuinely stands on — the tackroom above all, which is the
/// toolchain fence every cargo invocation is required to inherit — so a
/// prefix-wide sweep here would sever it. A bare spawn establishes nothing of
/// its own and is handed what it needs instead.
/// THE LOG FAMILY IS STRIPPED TOO, and it is the sharpest member of the roster
/// rather than a tidying. Those three names are ABSOLUTE PATHS INTO THE
/// DRIVER'S OWN RECORD, so a child kennel inheriting them opens the very files
/// the run executing this hurdle is writing — truncating them at its open, then
/// filling them with its own lure's stream. The driver's record then holds
/// whichever child spoke last, which is exactly what was found in the field:
/// the whole suite's record standing at seventy-eight lines of one hurdle's
/// python output. Unlike the config directory, this is not a verdict the
/// stripping protects but the EVIDENCE, and nothing in the outer run notices it
/// go.
///
/// THE POSED KIBBLE ROOT IS STRIPPED AS WELL, and that is what makes a drive over
/// the lure itself a STATION drive: a root the driver happened to carry would
/// otherwise reach a child that was never handed one. A posed root reaches a
/// child by one road only, the haynet's own drive, which states it.
const ZBKTU_SPAWN_STRIPPED: &[&str] = &[
    BKTU_CONFIG_DIR_VAR,
    ZBKTU_FOLIO_FAMILY,
    ZBKTU_LOG_FAMILY,
    BKTU_FEEDROOM_VAR,
];

/// The kennel's own variable carrying a posed kibble root — its constant read
/// from the other side, asserted equal by a hurdle rather than merely intended
/// to agree.
pub const BKTU_FEEDROOM_VAR: &str = "BKCQ_FEEDROOM";

/// The three members the amanuensis mode composes, reached by their shared
/// stem so the roster cannot come to carry two of three.
const ZBKTU_LOG_FAMILY: &str = "BURD_LOG_";

/// What a COMPOSED SEAT does not inherit, which is every channel the substrate
/// speaks across an exec.
///
/// WIDE ON PURPOSE, and the mirror of the roster above: a composed seat is a
/// repository of its own and re-establishes every one of these for itself, so a
/// value the outer dispatch set that survived into it would make the hurdle's
/// verdict a function of the driver. `BURE_` passes untouched, being the
/// substrate's operator-ambient channel by design.
const ZBKTU_SEAT_STRIPPED: &[&str] = &[
    "BURC_",
    "BURD_",
    "BURS_",
    "BURV_",
    ZBKTU_FOLIO_FAMILY,
    BKTU_FEEDROOM_VAR,
];

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
/// other.
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
///
/// EVERY LURE CARRIES ONE, not only a substrate seat: the kennel reads the
/// declarations a repository runs from its moorings' weaned set, so a lure
/// posing a declaration poses it there, committed.
const ZBKTU_MOORINGS: &str = ".buk";

/// The weaned set's name under the moorings — the kennel's own constant read
/// from the other side, asserted equal by a hurdle rather than merely intended
/// to agree.
const ZBKTU_WEANED: &str = "bkd_weaned";

/// A kit's brood name under its tools directory, read from the other side on
/// the same terms.
const ZBKTU_BROOD: &str = "bkd_brood";

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
    /// repository, the output pair, the temp root and the served registry are
    /// keyed on it: a registry keyed on a CONSTANT is one two hurdles composing a
    /// kibble share, and the harness runs them in parallel, so one removes and
    /// re-tars the archive the other is midway through converging — which
    /// presents as a tar that cannot open a file the seal has just read. Where a
    /// kibble lands is not keyed here at all: that is the haynet's root.
    name: String,
    root: PathBuf,
    output: PathBuf,
    temp: PathBuf,
    loosebox: PathBuf,
}

impl bktu_Lure {
    /// Compose a lure: a git repository under the temp root, carrying a pin and
    /// one commit.
    ///
    /// The name is the hurdle's, so a directory left behind by a panicking test
    /// says which hurdle made it.
    pub fn bktu_compose(name: &str) -> bktu_Lure {
        let declared = zbktu_declared_temp();

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
        let (output, temp, loosebox) = zbktu_seat(&declared, name);

        let lure = bktu_Lure {
            name: name.to_string(),
            root,
            output,
            temp,
            loosebox,
        };

        lure.bktu_write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.90.0\"\n");

        lure.bktu_git(&["init", "--quiet"]);
        lure.bktu_git(&["config", "user.email", "kennel@example.invalid"]);
        lure.bktu_git(&["config", "user.name", "kennel hurdle"]);
        lure.bktu_commit("compose the lure");

        lure
    }

    /// Stand a LINKED WORKTREE of this lure and hand it back as a lure of its own
    /// — the shape a linked worktree takes, this lure being its main worktree and
    /// therefore its delivered root.
    ///
    /// A REAL WORKTREE AND NEVER A POSED ONE. The kennel derives its geography by
    /// asking git from inside the tree a door entered, so the only honest way to
    /// put a door in a linked worktree is to stand the tree git will answer about:
    /// a hurdle that composed some other stand-in would be proving the stand-in.
    ///
    /// IT STANDS OUTSIDE THIS LURE'S TREE, beside it under the temp root, the way
    /// a work billet stands beside its sire's standing clone: a worktree inside
    /// the main worktree would be untracked ground in the delivered root, and a
    /// hurdle wanting that misplacement stands it with `bktu_billet_at`.
    ///
    /// DETACHED, because a branch per hurdle would be a name one hurdle could
    /// hold against another's, and nothing a hurdle proves turns on a branch.
    #[allow(dead_code)]
    pub fn bktu_billet(&self, name: &str) -> bktu_Lure {
        let declared = zbktu_declared_temp();
        self.bktu_billet_at(name, &Path::new(&declared).join(format!("bktu-billet-{}", name)))
    }

    /// Stand a linked worktree of this lure at a path the hurdle names.
    ///
    /// THE MISPLACED SHAPE IS THIS METHOD'S REASON: a worktree added inside its
    /// own main worktree is a real thing git permits, and the one geography a
    /// converge must refuse to write through, so a hurdle proving that refusal
    /// needs to stand one exactly there.
    #[allow(dead_code)]
    pub fn bktu_billet_at(&self, name: &str, root: &Path) -> bktu_Lure {
        let declared = zbktu_declared_temp();
        let _ = std::fs::remove_dir_all(root);
        let spelled = root.display().to_string();
        self.bktu_git(&["worktree", "prune"]);
        self.bktu_git(&["worktree", "add", "--quiet", "--detach", &spelled]);
        let (output, temp, loosebox) = zbktu_seat(&declared, &format!("billet-{}", name));

        bktu_Lure {
            name: name.to_string(),
            root: root.to_path_buf(),
            output,
            temp,
            loosebox,
        }
    }

    /// Stand a linked worktree added from a BARE clone of this lure — the one
    /// geography git answers without naming a main worktree, which the kennel
    /// refuses rather than reading as self.
    ///
    /// THE BARE CLONE STANDS INSIDE THE BILLET'S OWN SEAT, so the seat's removal
    /// takes it and no second path needs remembering; it is spelled with the
    /// `.git` suffix a bare repository conventionally wears, which is exactly the
    /// spelling that must not be mistaken for a main worktree's own directory.
    #[allow(dead_code)]
    pub fn bktu_billet_of_bare(&self, name: &str) -> bktu_Lure {
        let declared = zbktu_declared_temp();
        let root = Path::new(&declared).join(format!("bktu-billet-{}", name));
        let _ = std::fs::remove_dir_all(&root);
        let (output, temp, loosebox) = zbktu_seat(&declared, &format!("billet-{}", name));
        let bare = loosebox.join("lure.git");

        zbktu_git_in(
            Path::new(&declared),
            &["clone", "--quiet", "--bare", &self.root.display().to_string(), &bare.display().to_string()],
        );
        zbktu_git_in(&bare, &["worktree", "add", "--quiet", "--detach", &root.display().to_string()]);

        bktu_Lure {
            name: name.to_string(),
            root,
            output,
            temp,
            loosebox,
        }
    }

    /// Where the lure stands.
    pub fn bktu_root(&self) -> &Path {
        &self.root
    }

    /// This lure's moorings — the config directory every drive over it is
    /// handed.
    #[allow(dead_code)]
    pub fn bktu_moorings(&self) -> PathBuf {
        self.root.join(ZBKTU_MOORINGS)
    }

    /// This lure's weaned set — the declarations a drive over it runs, and the
    /// set an in-process reading is handed.
    #[allow(dead_code)]
    pub fn bktu_weaned(&self) -> PathBuf {
        self.bktu_moorings().join(ZBKTU_WEANED)
    }

    /// A lure-relative path inside the weaned set, for a hurdle posing a
    /// declaration of its own: `instance/family` in, the path to write out.
    #[allow(dead_code)]
    pub fn bktu_weaned_at(&self, held: &str) -> String {
        format!("{}/{}/{}", ZBKTU_MOORINGS, ZBKTU_WEANED, held)
    }

    /// A lure-relative path inside one kit's brood, for a hurdle posing the
    /// declaration a weaned copy is proven against.
    #[allow(dead_code)]
    pub fn bktu_brood_at(&self, kit: &str, held: &str) -> String {
        format!("Tools/{}/{}/{}", kit, ZBKTU_BROOD, held)
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
    ///
    /// A POSED KIBBLE ROOT IS NEVER STATED HERE. It reaches a child through the
    /// haynet's own drive and no other, so a hurdle stating one by hand would be
    /// opening the road the two seat types exist to close.
    #[allow(dead_code)]
    pub fn bktu_drive_stating(
        &self,
        kennel: &Path,
        args: &[&str],
        stated: &[(&str, &std::ffi::OsStr)],
    ) -> Output {
        assert!(
            stated.iter().all(|(name, _)| *name != BKTU_FEEDROOM_VAR),
            "{} is stated by a haynet's drive alone, never by hand",
            BKTU_FEEDROOM_VAR
        );

        self.zbktu_spawn(kennel, args, stated)
    }

    /// The one spawn, beneath both seats' roads.
    fn zbktu_spawn(
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
        // which is the driver's seat and never the lure's, and the FOLIO, which
        // is a caller's own argument. Everything else the dispatch exports is a
        // fact about the STATION the child genuinely stands on — the tackroom
        // above all, which is the toolchain fence every cargo invocation is
        // required to inherit — and a prefix-wide sweep here would sever it. The
        // dispatch surface below sweeps wide precisely because the seat it
        // enters re-establishes its own; a bare kennel spawn establishes
        // nothing, so it is handed what it needs instead.
        zbktu_strip(&mut child, ZBKTU_SPAWN_STRIPPED);

        child.env(BKTU_CONFIG_DIR_VAR, self.bktu_moorings());
        child.env(BKTU_OUTPUT_DIR_VAR, &self.output);
        child.env(BKTU_TEMP_ROOT_VAR, &self.temp);
        // STATED AHEAD OF THE CALLER'S OWN, so a hurdle proving what the voice
        // does with a composed log family may still state one over it.
        child.env(BKTU_NO_LOG_VAR, "1");
        child.env(BKTU_LOOSEBOX_DIR_VAR, &self.loosebox);
        child.env(BKTU_NOW_VAR, BKTU_NOW);

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

        zbktu_armed(
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

            zbktu_armed(
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
    pub const BKTU_CASE: &'static str = "lure_the_crate_carries_one_case";

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
                &format!("{}/{}/{}/bkrr.env", ZBKTU_MOORINGS, ZBKTU_WEANED, collar),
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
            &format!("{}/{}/{}/bkrr.env", ZBKTU_MOORINGS, ZBKTU_WEANED, collar),
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

        // THE PLACEHOLDER LOCK CARRIES THE PROJECT'S OWN ENTRY, as every lock uv
        // writes does, because the partition reading is read against it: a lock
        // with no entry for the project is a finding, and the conforming project
        // this composer stands for carries none.
        if !bent.lockless {
            self.bktu_write(
                &seat("uv.lock"),
                "version = 1\nrevision = 3\nrequires-python = \">=3.12\"\n\n\
                 [[package]]\nname = \"lure\"\nversion = \"0.0.1\"\nsource = { virtual = \".\" }\n\n\
                 [package.metadata]\n",
            );
        }

        self.bktu_write(&seat(".python-version"), &format!("{}\n", bent.pin));

        let roots = if at.is_empty() {
            "pyproject.toml".to_string()
        } else {
            format!("{}/pyproject.toml", at)
        };

        self.bktu_write(
            &format!("{}/{}/{}/bkrp.env", ZBKTU_MOORINGS, ZBKTU_WEANED, collar),
            &format!(
                "\
BKRP_COLLAR=\"{}\"
BKRP_KIND=\"bknre_suite\"
BKRP_MANIFEST=\"{}\"
BKRP_ROOTS=\"{}\"
BKRP_SPEND=\"bknre_reader\"
BKRP_BAKED_PACKAGES=\"{}\"
BKRP_CRUDE_PACKAGES=\"bknre_baked\"
BKRP_TARGET=\"bknre_manifest\"
BKRP_RUNNER=\"bknre_pytest\"
BKRP_TONGUE=\"bknre_pytest\"
",
                collar,
                seat("pyproject.toml"),
                roots,
                bent.baked
            ),
        );
    }

    /// Lay a local simple index under this lure's seat, serving the files given
    /// for one package, and hand back the URL a manifest declares it by.
    ///
    /// UNDER THE SEAT AND NEVER IN THE TREE: the index is a hurdle's fixture, and
    /// a fixture inside the repository would be untracked ground the door law
    /// refuses. A SIMPLE INDEX AND NEVER A FLAT ONE, because over a flat index uv
    /// enforces no declared environment for a package carrying an sdist, so a
    /// hurdle served from one proves nothing about coverage. No hash fragment
    /// rides a link: uv locks and installs from an index without them, benched.
    #[allow(dead_code)]
    pub fn bktu_simple_index(&self, package: &str, files: &[(String, Vec<u8>)]) -> String {
        let index = self.temp.join("index");
        let served = index.join("files");
        let listing = index.join("simple").join(package);

        for dir in [&served, &listing] {
            std::fs::create_dir_all(dir)
                .unwrap_or_else(|err| panic!("could not lay an index at {}: {}", dir.display(), err));
        }

        let mut rows = String::new();
        for (name, bytes) in files {
            std::fs::write(served.join(name), bytes)
                .unwrap_or_else(|err| panic!("could not serve {} from the index: {}", name, err));
            rows.push_str(&format!("<a href=\"../../files/{0}\">{0}</a><br/>\n", name));
        }

        std::fs::write(
            listing.join("index.html"),
            format!("<!DOCTYPE html><html><body>\n{}</body></html>\n", rows),
        )
        .unwrap_or_else(|err| panic!("could not write the index's listing for {}: {}", package, err));

        format!("file://{}", index.join("simple").display())
    }
}

/// Compose a wheel: a real zip archive of stored entries carrying one module and
/// its metadata, tagged as the hurdle says.
///
/// A REAL ARCHIVE BECAUSE UV OPENS IT AT THE LOCK. A placeholder served as a
/// wheel refuses as an invalid package format before any environment is read
/// (benched), so a hurdle proving a coverage refusal must serve a wheel uv can
/// read metadata out of; the sdist beside it may be a placeholder, the bar
/// keeping uv from opening it. The record carries no digests, which uv tolerates
/// at the lock; no hurdle installs what this composes.
#[allow(dead_code)]
pub fn bktu_wheel(name: &str, version: &str, tag: &str) -> (String, Vec<u8>) {
    let dist = format!("{}-{}.dist-info", name, version);

    let mut entries: Vec<(String, Vec<u8>)> = vec![
        (
            format!("{}/__init__.py", name),
            format!("BKTU_SERVED = \"{} {}\"\n", name, version).into_bytes(),
        ),
        (
            format!("{}/METADATA", dist),
            format!("Metadata-Version: 2.1\nName: {}\nVersion: {}\n", name, version).into_bytes(),
        ),
        (
            format!("{}/WHEEL", dist),
            format!(
                "Wheel-Version: 1.0\nGenerator: bktu\nRoot-Is-Purelib: true\nTag: {}\n",
                tag
            )
            .into_bytes(),
        ),
    ];

    let mut record = String::new();
    for (path, _) in &entries {
        record.push_str(&format!("{},,\n", path));
    }
    record.push_str(&format!("{}/RECORD,,\n", dist));
    entries.push((format!("{}/RECORD", dist), record.into_bytes()));

    (format!("{}-{}-{}.whl", name, version, tag), zbktu_zip(&entries))
}

/// A zip archive of stored entries: local headers, the central directory, and
/// its end record, as the format's specification lays them.
fn zbktu_zip(entries: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut central: Vec<u8> = Vec::new();

    for (path, data) in entries {
        let offset = out.len() as u32;
        let crc = zbktu_crc32(data);
        let name = path.as_bytes();
        let size = data.len() as u32;

        out.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0x21u16.to_le_bytes());
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(name);
        out.extend_from_slice(data);

        central.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0x21u16.to_le_bytes());
        central.extend_from_slice(&crc.to_le_bytes());
        central.extend_from_slice(&size.to_le_bytes());
        central.extend_from_slice(&size.to_le_bytes());
        central.extend_from_slice(&(name.len() as u16).to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u32.to_le_bytes());
        central.extend_from_slice(&offset.to_le_bytes());
        central.extend_from_slice(name);
    }

    let start = out.len() as u32;
    let size = central.len() as u32;
    out.extend_from_slice(&central);

    out.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    out.extend_from_slice(&size.to_le_bytes());
    out.extend_from_slice(&start.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());

    out
}

/// The zip format's own checksum over one entry.
fn zbktu_crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;

    for byte in data {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }

    !crc
}

impl bktu_Lure {

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
        self.zbktu_program(
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
        self.zbktu_program(
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
        self.zbktu_program(
            &format!("{}/lure_workbench.sh", ZBKTU_MOORINGS),
            coordinator,
        );

        // The door.
        self.zbktu_program(
            ZBKTU_TABTARGET,
            "#!/bin/bash\n\
             export BURD_LAUNCHER=launcher.lure_workbench.sh\n\
             exec \"${BASH_SOURCE[0]%/*}/z-launcher.sh\" \"${0##*/}\" \"${@}\"\n",
        );

        // What the dispatch composes as it runs, which a seat keeps out of its own
        // commit exactly as a consumer tree keeps them out of its own.
        self.bktu_write(".gitignore", "logs/\noutput/\ntemp/\n");

        self.bktu_commit("lay down the substrate seat");

        self.root.join(ZBKTU_TABTARGET)
    }

    /// Dispatch a tabtarget the seat composed and hand back what the child said.
    ///
    /// OBSERVED FROM OUTSIDE THE SHELL. The child's exit code and both its streams
    /// are read here, in a process the dispatch cannot reach, which is the posture
    /// a harness asserting on a bash process is required to hold.
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

    /// Lay a kibble down over an archive this lure SERVES ITSELF, named as the
    /// caller names it, and commit it.
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
    ///
    /// REACHED THROUGH A HAYNET AND NO OTHER WAY (`bktu_Haynet::bktu_kibble_seat`),
    /// so a kibble this lure invents converges beneath a root its hurdle owns and
    /// never in the station's store.
    fn zbktu_kibble_lay(&self, named: &str, seal: &str) {
        let served = self.zbktu_served();

        self.bktu_write(
            &format!("{}/{}/{}/bkrk.env", ZBKTU_MOORINGS, ZBKTU_WEANED, named),
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
    /// THE DRIVE MUST THEREFORE REACH THE STATION'S OWN STORE, which is where
    /// that residence stands — a drive over a station seat, the posed root
    /// stripped. A hurdle reaching some other root instead would find it cold
    /// and would be measuring a converge rather than a launch.
    #[allow(dead_code)]
    fn zbktu_nextest_seat(&self) {
        let declared = self.zbktu_nextest_declaration();

        self.bktu_write(
            &format!("{}/{}/{}/bkrk.env", ZBKTU_MOORINGS, ZBKTU_WEANED, ZBKTU_NEXTEST_KIBBLE),
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
    fn zbktu_nextest_seat_unplaced(&self, version: &str) {
        let declared = self.zbktu_nextest_declaration();
        let standing = zbktu_declared(&declared, "BKRK_VERSION");

        assert_ne!(
            standing, version,
            "the unplaced seat must declare a version the estate does NOT, or the residence it \
             composes is the one heel already wrote"
        );

        self.bktu_write(
            &format!("{}/{}/{}/bkrk.env", ZBKTU_MOORINGS, ZBKTU_WEANED, ZBKTU_NEXTEST_KIBBLE),
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
    fn zbktu_uv_seat(&self) {
        self.bktu_write(
            &format!("{}/{}/{}/bkrk.env", ZBKTU_MOORINGS, ZBKTU_WEANED, ZBKTU_UV_KIBBLE),
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
    fn zbktu_uv_seat_unplaced(&self, version: &str) {
        let declared = self.zbktu_declaration(ZBKTU_UV_KIBBLE);
        let standing = zbktu_declared(&declared, "BKRK_VERSION");

        assert_ne!(
            standing, version,
            "the unplaced seat must declare a version the estate does NOT, or the residence it \
             composes is the one heel already placed"
        );

        self.bktu_write(
            &format!("{}/{}/{}/bkrk.env", ZBKTU_MOORINGS, ZBKTU_WEANED, ZBKTU_UV_KIBBLE),
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
            .join("..")
            .join(ZBKTU_BROOD)
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

    /// Write a file into the lure, creating the directories above it.
    ///
    /// AN ORDINARY FILE ONLY. This process holds the file open while it writes,
    /// so a program written here could meet the race Pastern's program writers
    /// exist to close; a path already standing armed refuses rather than being
    /// rewritten in place, its mode surviving the write.
    pub fn bktu_write(&self, relative: &str, content: &str) {
        let path = self.root.join(relative);
        zbktu_unarmed(&path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .unwrap_or_else(|err| panic!("could not make {}: {}", parent.display(), err));
        }
        std::fs::write(&path, content)
            .unwrap_or_else(|err| panic!("could not write {}: {}", path.display(), err));
    }

    /// Write a program into the lure and arm it, the write taken by a child.
    fn zbktu_program(&self, relative: &str, body: &str) {
        zbktu_armed(&self.root.join(relative), body);
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

    /// This lure's posed seat: a kibble root the hurdle owns, beneath the
    /// lure's own temp directory.
    #[allow(dead_code)]
    pub fn bktu_haynet(&self) -> bktu_Haynet {
        bktu_Haynet {
            root: self.temp.join(ZBKTU_HAYNET),
        }
    }

    /// This lure's station seat, which carries no root.
    #[allow(dead_code)]
    pub fn bktu_stationed(&self) -> bktu_Stationed {
        bktu_Stationed { _unposed: () }
    }
}

/// Where a lure's haynet stands beneath its temp directory.
const ZBKTU_HAYNET: &str = "haynet";

/// A POSED SEAT: a kibble root one hurdle owns, beneath its own dispatch temp
/// directory, and the only seat a fake kibble is composed on.
///
/// THE ROOT IS CARRIED AND NEVER SHARED. Every converge over this seat is handed
/// it — in process as the argument, on a bare kennel spawn as the kennel's own
/// variable stated on the child — so two hurdles holding one kibble name own two
/// residences, and no hurdle's converge can land where another's asserts.
///
/// THE ROOT IS NEVER PRE-CREATED. The converge's lock makes it, so a refusal
/// hurdle finding the root standing after its drive has proven the root was
/// read — which is the one thing a seat a door ignored cannot fake.
///
/// NO CONVERSION LEADS FROM IT TO A STATION SEAT, nor from one to it: a hurdle
/// holds the seat its store is, and the type is that decision.
pub struct bktu_Haynet {
    root: PathBuf,
}

impl bktu_Haynet {
    /// The root this seat carries.
    #[allow(dead_code)]
    pub fn bktu_root(&self) -> &Path {
        &self.root
    }

    /// Lay a kibble down in the lure over the archive the lure serves, named as
    /// the caller names it and declaring the seal the caller hands it.
    ///
    /// ON THIS SEAT ALONE, because a fake kibble converged anywhere but a root
    /// its hurdle owns would land in the station's shared store. The name is the
    /// caller's and keys nothing: the residence is keyed on this seat.
    #[allow(dead_code)]
    pub fn bktu_kibble_seat(&self, lure: &bktu_Lure, kibble: &str, seal: &str) {
        lure.zbktu_kibble_lay(kibble, seal);
    }

    /// Where a kibble of the lure's served program, converged over this seat,
    /// lands its binary — composed the way the door composes it, so a hurdle
    /// asserts against the same path.
    #[allow(dead_code)]
    pub fn bktu_kibble_residence(&self, kibble: &str) -> PathBuf {
        self.root
            .join(kibble)
            .join(bktu_Lure::BKTU_KIBBLE_VERSION)
            .join(bktu_Lure::BKTU_KIBBLE_PROGRAM)
    }

    /// Run the kennel as a child over a lure with this seat's root stated, and
    /// hand back what it said.
    #[allow(dead_code)]
    pub fn bktu_drive_door(&self, lure: &bktu_Lure, kennel: &Path, door: &[&str]) -> Output {
        lure.zbktu_spawn(kennel, door, &[(BKTU_FEEDROOM_VAR, self.root.as_os_str())])
    }
}

/// A STATION SEAT: it carries no root, and a hurdle holding one reaches the
/// station's real residences through the kennel's own reader and no other way.
///
/// THE ESTATE'S OWN KIBBLES ARE LAID THROUGH IT, because what a lure copies of
/// them is the declaration the station's residences answer to: a hurdle reaching
/// the residence heel already wrote is reading the station, and saying so in the
/// type is what keeps a reader of the hurdle from mistaking it for one that owns
/// its store. A drive over the lure itself is a drive over this seat, the posed
/// root being stripped from every such spawn.
///
/// IT NEVER WRITES THE STATION'S STORE, and nothing on it composes a kibble the
/// lure invents: that is a haynet's alone.
pub struct bktu_Stationed {
    _unposed: (),
}

impl bktu_Stationed {
    /// Lay the estate's own nextest kibble down in a lure.
    #[allow(dead_code)]
    pub fn bktu_nextest_seat(&self, lure: &bktu_Lure) {
        lure.zbktu_nextest_seat();
    }

    /// Lay the estate's own nextest kibble down in a lure WITH ITS VERSION
    /// MOVED, so its residence is one nothing has ever placed.
    #[allow(dead_code)]
    pub fn bktu_nextest_seat_unplaced(&self, lure: &bktu_Lure, version: &str) {
        lure.zbktu_nextest_seat_unplaced(version);
    }

    /// Lay the estate's own uv kibble down in a lure.
    #[allow(dead_code)]
    pub fn bktu_uv_seat(&self, lure: &bktu_Lure) {
        lure.zbktu_uv_seat();
    }

    /// Lay the estate's own uv kibble down in a lure WITH ITS VERSION MOVED, so
    /// its residence is one nothing has ever placed.
    #[allow(dead_code)]
    pub fn bktu_uv_seat_unplaced(&self, lure: &bktu_Lure, version: &str) {
        lure.zbktu_uv_seat_unplaced(version);
    }
}

/// The per-seat directories a lure is handed: the output pair and the loosebox,
/// beneath one lure-local seat under the dispatch's temp root.
///
/// ONE COMPOSITION FOR BOTH LURE SHAPES, a composed repository and a linked
/// worktree of one, so a billet is handed exactly the directories a lure is and
/// the two cannot come to differ in what a drive over them inherits.
fn zbktu_seat(declared: &str, name: &str) -> (PathBuf, PathBuf, PathBuf) {
    let seat = Path::new(declared).join(format!("bktu-seat-{}", name));
    let output = seat.join("out");
    let temp = seat.join("temp");
    // SPELLED, NOT CITED, and the position is why: this file is compiled a
    // second time as a module of the integration-test crate by `#[path]`,
    // where the kennel's breviary is another crate's and unreachable by
    // `crate::`, and the lure imports nothing of the kennel by design. A
    // literal standing outside the declaring crate is the printed-string
    // rule's third position, owed nothing until the twins are published.
    let loosebox = seat.join("loosebox");

    for dir in [&output, &temp, &loosebox] {
        let _ = std::fs::remove_dir_all(dir);
        std::fs::create_dir_all(dir).unwrap_or_else(|err| {
            panic!("could not compose a lure's seat at {}: {}", dir.display(), err)
        });
    }

    (output, temp, loosebox)
}

/// The dispatch's temp root, refusing where it is unset.
fn zbktu_declared_temp() -> String {
    std::env::var(BKTU_TEMP_ROOT_VAR).unwrap_or_else(|_| {
        panic!(
            "{} is unset. The kennel's hurdles stand under the dispatch's temp root and \
             reach for no ambient temp directory; drive the suite through its own door",
            BKTU_TEMP_ROOT_VAR
        )
    })
}

/// Run git in a directory that is not a lure's own root, dying on a refusal.
fn zbktu_git_in(at: &Path, args: &[&str]) {
    let out = Command::new("git")
    .arg("-C")
    .arg(at)
    .args(args)
    .output()
    .unwrap_or_else(|err| panic!("could not run git {:?}: {}", args, err));

    if !out.status.success() {
    panic!(
        "git {:?} refused at {}: {}",
        args,
        at.display(),
        String::from_utf8_lossy(&out.stderr).trim()
    );
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
    zbktu_unarmed(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|err| panic!("could not make {}: {}", parent.display(), err));
    }
    std::fs::write(path, body)
        .unwrap_or_else(|err| panic!("could not compose {}: {}", path.display(), err));
}

/// Write a hurdle's program at `path` and arm it, refusing the hurdle if the
/// writer fails.
fn zbktu_armed(path: &Path, body: &str) {
    bkprs_program_write(path, body).unwrap_or_else(|err| panic!("{}", err));
}

/// Refuse an in-process write over a file already standing armed.
///
/// The muzzle bars the calls that arm a file, but rewriting one that is already
/// armed keeps its mode and calls nothing barred; this closes that road at the
/// lure's own writers.
fn zbktu_unarmed(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    if let Ok(standing) = std::fs::metadata(path) {
        assert!(
            standing.permissions().mode() & 0o111 == 0,
            "{} stands armed, so it is a program and not an ordinary file: rewrite it with \
             bkprs_program_write, which writes in a child",
            path.display()
        );
    }
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
    /// What the collar's baked list carries — the direct dependencies the
    /// bent manifest declares, as the lock spells them, whitespace-separated.
    pub baked: String,
}

impl Default for bktu_Bent {
    fn default() -> Self {
        bktu_Bent {
            pin: "3.12.7".to_string(),
            requires: ">=3.12".to_string(),
            uv: String::new(),
            lockless: false,
            baked: String::new(),
        }
    }
}


// eof
