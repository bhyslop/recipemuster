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

//! The spawned drives (BKSLR-Lure.adoc "Two Drives").
//!
//! EVERY HURDLE THAT SPAWNS THE KENNEL STANDS HERE, and the reason is stated
//! below rather than repeated at each: cargo exports the built binary's path to
//! an integration test alone. The file opens on the door-and-library pair the
//! sheaf names, and the door hurdles that need a real artifact — a converge that
//! installs one, a walk that runs suites — follow it under the same forcing.
//!
//! THE PAIR IS ONE RULE PROVEN FROM BOTH SIDES. The DOOR refuses an uncommitted
//! repository so that every record maps to a position; the LIBRARY refuses
//! nothing, because a library consumer holds its own posture — one such
//! consumer reads cargo metadata in-process on exactly the dirty tree its own
//! census is driven against. Neither posture is observable through the other's drive, so
//! the sheaf owes both, and one lure shape carries both here.
//!
//! WHY THESE HURDLES STAND UNDER `tests/` WHERE THE OTHERS DO NOT. Cargo
//! exports `CARGO_BIN_EXE_<bin>` to an integration test and to nothing else —
//! never to a unit test inside `src/`. A spawned drive that reached instead for
//! the whistle, or for a name on the path, would judge some other artifact than
//! the one cargo just built, which is the one fact a spawned drive exists to
//! establish. The seat is therefore forced, and the forcing reaches this file
//! alone.
//!
//! THE COMPOSING SURFACE IS STILL THE ONLY HOME. It is reached below by `#[path]`
//! at the very file the unit hurdles reach as a module, so this is the same
//! source and not a second lure surface.
//!
//! IT IS REACHED THAT WAY HERE AND BY FEATURE ELSEWHERE, and the two are not a
//! contradiction. The surface is published under the crate's `lure` feature so a
//! consumer in another crate can reach it at all — `cfg(test)` is never set for a
//! dependent — but this file is a target of the very package that declares it,
//! and a package cannot turn its own feature on for its own test targets without
//! depending on itself. So the seam a consumer takes is closed to this file, and
//! `#[path]` is what remains. What the feature does NOT do is publish into the
//! binary: no binary build names it, and the consumer that does takes this crate
//! as a dev-dependency, which cargo compiles into a test target alone — which is
//! why `Tools/bkk/bk0/bkce_roots.txt` can still carve `src/bkt*` out of the
//! elected roots on the ground that nothing beneath it reaches the artifact.

#![deny(warnings)]

// The one home, reached rather than copied. `dead_code` is allowed for the
// module and not for this file: an integration test uses part of the surface,
// and the unused remainder is a fact about this hurdle rather than about the
// surface.
#[path = "../src/bktu_lure.rs"]
#[allow(dead_code, non_camel_case_types)]
mod bktu_lure;

use bkk::bkce_election::{BKCE_GEOGRAPHY_DISPATCHED, BKCE_GEOGRAPHY_SELF};
use bkk::bkcc_record;
use bkk::bkcf_guard;
use bkk::bkcf_guard::BKCF_REMEDY;
use bkk::bkcf_guard::BKCF_REPOINT;
use bkk::bkcl_leash;
use bkk::bkco_output;
use bkk::bkcq_kibble;
use bkk::bkcr_resolve::bkcr_resolve;
use bkk::bkcv_voice;
use bkk::bkcy_sweep::{bkcy_sight, bkcy_Sighting};
use bktu_lure::bktu_Lure;
use std::ffi::OsStr;
use std::path::Path;

/// The door the plants below are driven through, and the version a shim answers
/// with — anything the kennel does not expect, spelled once so the assertions
/// read against the same string the plant was composed from.
const BKTD_DOOR: &str = "mush";
const BKTD_STRANGE: &str = "0.9.999";

/// A nextest version the estate does not declare, so a kibble carrying it
/// composes a residence nothing has ever placed.
const BKTD_UNPLACED: &str = "0.9.997";

/// The programs carried forward across the narrowing: the rustup shims that
/// stand in the same directory a nextest does, and which dropping that directory
/// would otherwise take with it. Git and the toolchain's own linker stand
/// elsewhere and survive the narrowing on their own.
///
/// `rustdoc` is among them because a `cargo test` runs the doc tests too, and a
/// control drive that could not reach it would report a failure about the
/// narrowing rather than about the collar.
///
/// `cargo-clippy` IS AMONG THEM FOR THE SAME REASON AND IS THE EASIEST TO MISS.
/// A suite course is muzzled before any hurdle runs, so every drive here spawns
/// the linter — and the linter's own shim stands in exactly the directory this
/// narrowing drops. Left out, a hurdle about a MISSING NEXTEST would be quietly
/// answered by a missing linter instead, and its control drive would fail
/// reporting a crate that did not compile.
const BKTD_CARRIED: &[&str] = &["cargo", "rustc", "rustup", "rustdoc", "cargo-clippy"];

/// The file each drive plants after the lure's commit. Named once so the door's
/// console and the library's answer are asserted against the same string.
const BKTD_STRAY: &str = "stray.rs";

/// The kennel this drive judges: THE BINARY CARGO JUST BUILT, named by cargo's
/// own export — never the whistle, never a name resolved on the path.
///
/// The export is why this hurdle file stands where it does, and reading it here
/// rather than in the composing surface is why the surface can still be compiled
/// as a module of the lib, where no such export is in scope.
fn zbktd_kennel() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_bkx"))
}

/// The door word each scoop hurdle below asks for. Stated once so a typo could
/// not quietly turn these into drives of the bare kennel, which exits 0.
const ZBKTD_SCOOP: &str = "scoop";

/// Warm a sighted collar's build directory by asking cargo to build it, and hand
/// back where it stands.
///
/// THE DIRECTORY IS MADE BY A REAL BUILD rather than by `create_dir_all`, because
/// what the door removes is cargo's own answer and a hurdle that composed the
/// path would prove nothing about that answer.
fn zbktd_warm(lure: &bktu_Lure, collar: &str) -> std::path::PathBuf {
    let resolved = bkcr_resolve(lure.bktu_root(), &lure.bktu_weaned(), collar).expect("the collar should resolve");

    let sighted =
        bkcy_sight(lure.bktu_root(), &lure.bktu_weaned(), None, &resolved.collar).expect("cargo should be reachable");

    let directory = match sighted {
        bkcy_Sighting::Within(directory) => directory,
        other => panic!("the lure's own crate sights within it: {:?}", other),
    };

    std::fs::create_dir_all(directory.join("debug")).expect("the hurdle should warm the yard");

    directory
}

#[test]
fn bktd_the_door_refuses_a_dirty_lure_and_names_the_notch() {
    let lure = bktu_Lure::bktu_compose("drive-door-dirty");
    lure.bktu_write(BKTD_STRAY, "// planted by the hurdle\n");

    let out = lure.bktu_drive(zbktd_kennel());

    // NON-ZERO, and deliberately not the door's own code: the exit it takes is
    // declared private to the binary, and a hurdle spelling that number would be
    // a second home for it. What parts this refusal from any other non-zero exit
    // is the console, asserted below.
    assert!(!out.status.success(), "the door refuses a dirty lure: {}", out.status);

    let said = String::from_utf8_lossy(&out.stderr);
    assert!(
        said.contains(BKCF_REMEDY),
        "the refusal names the remedy ({}): {}",
        BKCF_REMEDY,
        said, //
    );
    assert!(
        said.contains(BKTD_STRAY),
        "the refusal names the file it found: {}",
        said
    );
}

#[test]
fn bktd_the_door_refuses_a_dangling_include_and_names_it() {
    let lure = bktu_Lure::bktu_compose("drive-door-dangling");
    lure.bktu_write("CLAUDE.md", "# context\n\n@Tools/kit/held.md\n");
    lure.bktu_commit("bank a CLAUDE.md whose include names nothing yet");

    let out = lure.bktu_drive(zbktd_kennel());

    assert!(!out.status.success(), "the door refuses a dangling include: {}", out.status);
    let said = String::from_utf8_lossy(&out.stderr);
    assert!(
        said.contains("CLAUDE.md:3: @Tools/kit/held.md"),
        "the refusal names file, line and path: {}",
        said
    );
    assert!(
        said.contains(BKCF_REPOINT),
        "the refusal names the remedy ({}): {}",
        BKCF_REPOINT,
        said, //
    );

    // THE CONTROL, over the same lure: the file the include names now stands, and
    // the door that refused above stands over it. A binary refusing any CLAUDE.md
    // at all would clear the assertions above; this is what says the refusal is
    // about the dangling.
    lure.bktu_write("Tools/kit/held.md", "held\n");
    lure.bktu_commit("stand the file the include names");

    let out = lure.bktu_drive(zbktd_kennel());

    assert!(
        out.status.success(),
        "an include naming a tracked file passes: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn bktd_the_door_stands_over_a_clean_lure() {
    let lure = bktu_Lure::bktu_compose("drive-door-clean");

    let out = lure.bktu_drive(zbktd_kennel());

    // THE CONTROL FOR THE REFUSAL ABOVE, and it is not decoration: a binary that
    // exited non-zero over anything at all would clear that hurdle. This is what
    // says the refusal is about the dirt.
    assert!(
        out.status.success(),
        "a committed lure carries nothing for the door law to refuse: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn bktd_the_library_answers_over_the_same_dirty_lure() {
    let lure = bktu_Lure::bktu_compose("drive-library-dirty");
    lure.bktu_write(BKTD_STRAY, "// planted by the hurdle\n");

    // AN ANSWER, NOT A REFUSAL — over the same ground the door refused. The
    // library reports what stands and rules nothing, which is the posture a
    // census consumer depends on; a refusal seated here would break the one
    // consumer that reads on a dirty tree by design.
    let standing = bkcf_guard::bkcf_standing(lure.bktu_root())
        .expect("the library answers over a dirty tree rather than refusing it");

    assert!(
        !standing.bkcf_clean(),
        "the library sees what the door refused: {:?}",
        standing.entries
    );
    assert!(
        standing
            .entries
            .iter()
            .any(|entry| entry.contains(BKTD_STRAY)),
        "the answer names the file the drive planted: {:?}",
        standing.entries
    );
}


/// The kennel's own name for the door under test, so a hurdle spells a door word
/// where the operator would type one.
const BKTD_DOOR_HEEL: &str = "heel";
const BKTD_DOOR_DERBY: &str = "derby";

/// The lint list every composed collar points at. A muzzle directory holding no
/// list is a finding, so a lure whose collars are meant to conform carries one.
const BKTD_CLIPPY: &str = "# the lure's lint list, empty and standing\n";

/// A lock for a crate with no dependencies at all — deterministic, and
/// hand-written because the estate has no road to regenerate one: the leash
/// passes `--locked` on every invocation, which is the flag that refuses a lock
/// needing an update.
fn zbktd_lock(name: &str) -> String {
    format!(
        "version = 4\n\n[[package]]\nname = \"{}\"\nversion = \"0.0.1\"\n",
        name
    )
}

/// A manifest for a crate with no dependencies.
fn zbktd_manifest(name: &str) -> String {
    format!(
        "[package]\nname = \"{}\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
        name
    )
}

/// Lay a dependency-free crate into the lure: manifest, lock, and the one source
/// file the caller supplies.
fn zbktd_crate(lure: &bktu_Lure, dir: &str, name: &str, source: &str, body: &str) {
    lure.bktu_write(&format!("{}/Cargo.toml", dir), &zbktd_manifest(name));
    lure.bktu_write(&format!("{}/Cargo.lock", dir), &zbktd_lock(name));
    lure.bktu_write(&format!("{}/{}", dir, source), body);
}

/// Lay a conforming app collar into the lure, the one every converge drive below
/// asks for.
///
/// A CONVERGE NEEDS A LAUNCHABLE, and the delouse hurdles want one only so that
/// heel has something to do — what they assert is about the sweep standing ahead
/// of it. The collar is composed here rather than in each hurdle on the lure
/// surface's own rule: a hurdle spelling a collar would be a second statement of
/// what the resolver demands.
fn zbktd_app(lure: &bktu_Lure) {
    // WHAT A CONVERGE LEAVES BEHIND IS IGNORED, as it is in every repository the
    // kennel actually stands in. A build directory and an installed launchable
    // are untracked artifacts, and the door law refuses an uncommitted
    // repository — so without this a lure could be converged exactly once, and
    // any hurdle asking what a SECOND drive does would be reading a refusal
    // about git rather than an answer about the door.
    lure.bktu_write(".gitignore", "target/\nTools/bin/\n");
    zbktd_crate(lure, "Tools/lure", "lure", "src/main.rs", "fn main() {}\n");
    lure.bktu_write(
        &lure.bktu_weaned_at("app-lure/bkrr.env"),
        "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"lure\"
BKRR_ROOTS=\"Tools/lure/src Tools/lure/Cargo.toml Tools/lure/Cargo.lock\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"Tools/bin\"
BKRR_BYNAME=\"lurex\"
",
    );
}

/// THE CONVERGE INSTALLS WHERE THE COLLAR SAYS, and this is the case cargo's own
/// build does not reach: the residence stands away from the output directory, so
/// an install that leaned on the build's side effect would leave nothing there at
/// all. The binary's ABSENCE beforehand is asserted as well as its presence
/// after — without it, a hurdle over a residence that happened to be populated
/// would pass having proven nothing.
#[test]
fn bktd_heel_installs_at_the_collars_residence() {
    let lure = bktu_Lure::bktu_compose("heel-installs");

    lure.bktu_write("clippy.toml", BKTD_CLIPPY);
    zbktd_crate(&lure, "Tools/lure", "lure", "src/main.rs", "fn main() {}\n");
    lure.bktu_write(
        &lure.bktu_weaned_at("app-lure/bkrr.env"),
        "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"lure\"
BKRR_ROOTS=\"Tools/lure/src Tools/lure/Cargo.toml Tools/lure/Cargo.lock\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"Tools/bin\"
BKRR_BYNAME=\"lurex\"
",
    );
    lure.bktu_commit("compose an app collar installing away from the output, under another name");

    // THE NAME IT WEARS THERE, not the one cargo struck. The two differ at this
    // collar on purpose: an install that copied the target's name through would
    // land a file no consumer of this collar looks for.
    let residence = lure.bktu_root().join("Tools/bin/lurex");
    assert!(
        !residence.exists(),
        "nothing stands at the residence before the converge: {}",
        residence.display()
    );

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "the converge lands: {} {}",
        out.status,
        said
    );
    assert!(
        residence.is_file(),
        "and the binary stands at the residence afterwards, under its declared name, where it did \
         not before: {} — {}",
        residence.display(),
        said
    );
    assert!(
        !lure.bktu_root().join("Tools/bin/lure").exists(),
        "and nothing landed under the name cargo struck: the converge renames as it installs, and \
         a copy that carried the target's name through would leave this file standing"
    );
    assert!(
        said.contains("Tools/bin"),
        "the door reports where it installed: {}",
        said
    );
}

/// A suite crate whose one hurdle fails.
const BKTD_RED: &str = "\
#[test]
fn bktd_red() {
    panic!(\"this course is red by construction\");
}
";

/// A suite crate whose one hurdle WRITES A MARK BESIDE ITSELF when it runs. The
/// mark is how a hurdle proves a runner was never spawned: an assertion on the
/// door's own console could only show that the door did not MENTION the course,
/// which is a weaker claim than the cinch makes.
const BKTD_MARKED: &str = "\
#[test]
fn bktd_marked() {
    std::fs::write(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/ran.txt\"), \"ran\")
        .expect(\"the mark is writable\");
}
";

/// Compose a suite collar over a crate standing at `dir`, its instance directory
/// beside the crate so the walk's sorted order is the directory order.
fn zbktd_suite_collar(lure: &bktu_Lure, dir: &str, name: &str) {
    lure.bktu_write(
        &lure.bktu_weaned_at(&format!("{}/bkrr.env", name)),
        &format!(
            "\
BKRR_COLLAR=\"{name}\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"{dir}/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"{dir}/src {dir}/Cargo.toml {dir}/Cargo.lock\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"bknre_cargo\"
BKRR_TONGUE=\"bknre_harness\"
"
        ),
    );
}

/// Two suite crates, the first of them red and the second marking the disk if it runs.
/// The walk sorts by instance path, so `alfa` is course one and `bravo` course
/// two whatever order they were written in.
fn zbktd_two_courses(lure: &bktu_Lure) {
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);

    zbktd_crate(lure, "Tools/alfa", "alfa", "src/lib.rs", BKTD_RED);
    zbktd_suite_collar(lure, "Tools/alfa", "suite-alfa");

    zbktd_crate(lure, "Tools/bravo", "bravo", "src/lib.rs", BKTD_MARKED);
    zbktd_suite_collar(lure, "Tools/bravo", "suite-bravo");
}

/// A suite crate whose one hurdle WRITES ITS MARK AND THEN FAILS, so a course
/// standing behind a red one can be proven both reached and red.
const BKTD_MARKED_RED: &str = "\
#[test]
fn bktd_marked_red() {
    std::fs::write(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/ran.txt\"), \"ran\")
        .expect(\"the mark is writable\");
    panic!(\"this course is red by construction, after marking\");
}
";

/// Two suite crates, BOTH red, the second marking the disk when it runs. The
/// shared two-course fixture keeps its second course green, which the record
/// and futurity hurdles lean on, so the two-red walk is composed apart.
fn zbktd_two_red_courses(lure: &bktu_Lure) {
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);

    zbktd_crate(lure, "Tools/alfa", "alfa", "src/lib.rs", BKTD_RED);
    zbktd_suite_collar(lure, "Tools/alfa", "suite-alfa");

    zbktd_crate(lure, "Tools/bravo", "bravo", "src/lib.rs", BKTD_MARKED_RED);
    zbktd_suite_collar(lure, "Tools/bravo", "suite-bravo");
}

/// The exit the derby and the futurity take when the walk ran through and a
/// course went red or was refused by the muzzle. Spelled here because the
/// constant is the binary's own and this hurdle stands outside it, as the
/// record exit is.
const ZBKTD_EXIT_FALLERS: i32 = 89;

/// The set verdict's line naming the red courses, read off a drive's console.
fn zbktd_red_line(said: &str) -> Option<&str> {
    said.lines().find(|line| line.contains("course(s) went red:"))
}

/// THE DERBY RUNS THROUGH EVERY RED COURSE (operator, 260929), and the run is
/// proven from the disk rather than from the console: the second course writes
/// its mark before it fails, so the mark says its runner was spawned behind the
/// red one ahead of it. The set verdict names both, and the exit is the walk's
/// own rather than either course's.
#[test]
fn bktd_derby_runs_through_both_red_courses() {
    let lure = bktu_Lure::bktu_compose("derby-through");
    zbktd_two_red_courses(&lure);
    lure.bktu_commit("compose two courses, both of them red");

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = zbktd_said(&out);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_FALLERS),
        "the walk exits with its own code: {}",
        said
    );
    assert!(
        mark.exists(),
        "the second course ran behind the red one ahead of it, which is what its written mark says: {}\n{}",
        mark.display(),
        said
    );
    let red = zbktd_red_line(&said).unwrap_or_else(|| panic!("no set verdict names the reds: {}", said));
    assert!(
        red.contains("suite-alfa") && red.contains("suite-bravo"),
        "the set verdict names both red courses: {}",
        red
    );
}

/// A COURSE THE MUZZLE REFUSES IS STEPPED PAST, named apart from the red ones,
/// and the course behind it still runs. The refused course lints against a list
/// of its own that bars a call its crate makes.
#[test]
fn bktd_derby_steps_past_a_muzzle_refused_course() {
    let lure = bktu_Lure::bktu_compose("derby-muzzled");
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);

    zbktd_crate(&lure, "Tools/alfa", "alfa", "src/lib.rs", BKTD_BARRED);
    lure.bktu_write("Tools/alfa/clippy.toml", ZBKTD_BARRING_LIST);
    zbktd_suite_collar(&lure, "Tools/alfa", "suite-alfa");
    lure.bktu_write(
        &lure.bktu_weaned_at("suite-alfa/bkrr.env"),
        &std::fs::read_to_string(lure.bktu_root().join(lure.bktu_weaned_at("suite-alfa/bkrr.env")))
            .expect("the collar just written is readable")
            .replace("BKRR_MUZZLE=\".\"", "BKRR_MUZZLE=\"Tools/alfa\""),
    );

    zbktd_crate(&lure, "Tools/bravo", "bravo", "src/lib.rs", BKTD_MARKED_RED);
    zbktd_suite_collar(&lure, "Tools/bravo", "suite-bravo");
    lure.bktu_commit("compose a muzzle-refused course ahead of a red one");

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = zbktd_said(&out);

    assert_eq!(out.status.code(), Some(ZBKTD_EXIT_FALLERS), "the walk exits with its own code: {}", said);
    assert!(mark.exists(), "the course behind the refused one ran: {}\n{}", mark.display(), said);
    assert!(
        said.lines()
            .any(|line| line.contains("refused by the muzzle before any hurdle ran:") && line.contains("suite-alfa")),
        "the set verdict names the refused course apart from the red ones: {}",
        said
    );
    let red = zbktd_red_line(&said).unwrap_or_else(|| panic!("no set verdict names the reds: {}", said));
    assert!(
        red.contains("suite-bravo") && !red.contains("suite-alfa"),
        "and the red line names the course that ran red alone: {}",
        red
    );
}

/// A suite crate calling the one method the barring list below refuses.
const BKTD_BARRED: &str = "\
#[test]
fn bktd_barred() {
    let held: Vec<u8> = Vec::new();
    assert!(held.is_empty());
}
";

/// A lint list barring the call the crate above makes.
const ZBKTD_BARRING_LIST: &str = "disallowed-methods = [\"std::vec::Vec::new\"]\n";

/// THE FUTURITY RUNS THROUGH EVERY LAUNCHED RED COURSE, as the derby does: a
/// pattern matching a hurdle in each course launches both, and both are named.
#[test]
fn bktd_futurity_runs_through_both_red_courses() {
    let lure = bktu_Lure::bktu_compose("narrowed-through");
    zbktd_two_red_courses(&lure);
    lure.bktu_commit("compose two courses, both of them red");

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_FUTURITY, "bktd_red", "bktd_marked_red"]);
    let said = zbktd_said(&out);

    assert_eq!(out.status.code(), Some(ZBKTD_EXIT_FALLERS), "the walk exits with its own code: {}", said);
    assert!(mark.exists(), "the second launched course ran: {}\n{}", mark.display(), said);
    let red = zbktd_red_line(&said).unwrap_or_else(|| panic!("no set verdict names the reds: {}", said));
    assert!(
        red.contains("suite-alfa") && red.contains("suite-bravo"),
        "the set verdict names both red courses: {}",
        red
    );
}

/// A RED COURSE REPORTS EVERY RED CASE (operator, 260929). A suite holding two
/// failing cases, driven through mush, names both.
///
/// WHAT THIS PROVES AND WHAT IT CANNOT. Cargo's harness runs every case of one
/// test binary whatever it is asked, so this reading would hold without the
/// kennel's run-through demand; the demand itself is proven where it is spelled
/// (`bktm_the_run_through_demand_rides_both_runs_and_neither_listing`). This is
/// the whole-door reading: both failures reach the console, each on its own
/// bounded line.
#[test]
fn bktd_a_red_course_reports_every_red_case() {
    let lure = bktu_Lure::bktu_compose("every-red-case");
    zbktd_empty_course(&lure, "Tools/twice", "suite-twice", "bknre_cargo", "bknre_harness");
    lure.bktu_write("Tools/twice/src/lib.rs", BKTD_TWICE_RED);
    lure.bktu_commit("compose a suite holding two failing cases");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, "suite-twice"]);
    let said = zbktd_said(&out);

    assert!(!out.status.success(), "the course is red: {}", said);
    assert!(
        said.contains("bktd_first_red") && said.contains("bktd_second_red"),
        "both failing cases are reported: {}",
        said
    );
}

/// A suite crate holding two failing cases.
const BKTD_TWICE_RED: &str = "\
#[test]
fn bktd_first_red() {
    panic!(\"one red case\");
}

#[test]
fn bktd_second_red() {
    panic!(\"the other red case\");
}
";

/// A COLLAR THAT DOES NOT CONFORM REFUSES THE WHOLE WALK BEFORE ANY COURSE RUNS,
/// naming it. The mark's absence carries the "before any course runs" half —
/// without it, a door that ran every sound course and refused at the end would
/// clear a hurdle that only read the console.
#[test]
fn bktd_derby_refuses_a_planted_invalid_collar_before_running() {
    let lure = bktu_Lure::bktu_compose("derby-invalid");
    zbktd_two_courses(&lure);

    // THE PLANT IS ONE MISSING FIELD, and it is the runner: a suite that declares
    // none names no command for the kennel to spawn, which is the finding class a
    // door most needs to refuse before acting rather than after.
    lure.bktu_write(
        &lure.bktu_weaned_at("suite-alfa/bkrr.env"),
        "\
BKRR_COLLAR=\"suite-alfa\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"Tools/alfa/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"Tools/alfa/src Tools/alfa/Cargo.toml Tools/alfa/Cargo.lock\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_TONGUE=\"bknre_harness\"
",
    );
    lure.bktu_commit("plant a suite collar that declares no runner");

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success(), "an unsound collar refuses the walk: {}", out.status);
    assert!(
        said.contains("suite-alfa"),
        "the refusal names the collar it found: {}",
        said
    );
    assert!(
        !mark.exists(),
        "and nothing ran: the sound course's mark stands unwritten at {}",
        mark.display()
    );
}

/// The door the narrowed walk is driven through, and the exit its pipeline takes
/// when a pattern set matches nothing — spelled here because the constant is
/// the binary's own and the contract names it.
const BKTD_DOOR_FUTURITY: &str = bkk::bkg_breviary::BKG_FUTURITY_BASE;
const BKTD_EXIT_EMPTY_MATCH: i32 = 75;

/// A PATTERN NAMING A HURDLE IN ONE COURSE RUNS THAT COURSE ALONE, and the
/// verdict names the other as not launched. The two-course fixture makes the
/// proof sharp: the course left idle is the RED one, so a futurity that launched
/// it would go red, and the green course's mark on the disk says its runner was
/// the one spawned.
#[test]
fn bktd_futurity_runs_only_the_course_holding_the_match() {
    let lure = bktu_Lure::bktu_compose("narrowed-one");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses, the first of them red");

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_FUTURITY, "bktd_marked"]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(out.status.success(), "the matched course is green: {}\n{}", out.status, said);
    assert!(mark.exists(), "the matched course's runner was spawned: {}", mark.display());
    assert!(
        said.contains("not launched") && said.contains("suite-alfa"),
        "the verdict names the course the patterns left idle: {}",
        said
    );
}

/// A PATTERN SET MATCHING NOTHING ACROSS THE UNION REFUSES UNDER THE PIPELINE'S
/// OWN EXIT, and no runner is spawned — the unwritten mark says so from the disk.
#[test]
fn bktd_futurity_matching_nothing_refuses_and_spawns_no_runner() {
    let lure = bktu_Lure::bktu_compose("narrowed-none");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses");

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_FUTURITY, "bktd_nowhere_in_the_yard"]);

    assert_eq!(
        out.status.code(),
        Some(BKTD_EXIT_EMPTY_MATCH),
        "an empty union refuses under the pipeline's exit: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!mark.exists(), "no runner was spawned: {}", mark.display());
}

/// How many passing cases the dying suite reports before its runner dies — the
/// count the 261004 close saw printed as passed over a runner that never
/// finished.
const ZBKTD_ALIVE: usize = 40;

/// A suite whose library cases all pass and whose integration case ENDS ITS
/// PROCESS inside its hold, the shape a bin test calling `process::exit` takes.
///
/// TWO TEST BINARIES ORDER THE DEATH, which one binary could not. Cargo runs the
/// library's cases first and the integration binary after, so every passing
/// line is printed before the runner dies; in one binary the cases race on
/// threads and the death could land before any of them spoke.
fn zbktd_dying_course(lure: &bktu_Lure) {
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);

    let alive: String = (0..ZBKTD_ALIVE)
        .map(|index| format!("#[test]\nfn bktd_alive_{:02}() {{}}\n\n", index))
        .collect();
    zbktd_crate(lure, "Tools/dying", "dying", "src/lib.rs", &alive);
    lure.bktu_write(
        "Tools/dying/tests/dies.rs",
        "#[test]\nfn bktd_dies() {\n    std::process::exit(3);\n}\n",
    );
    zbktd_suite_collar(lure, "Tools/dying", "suite-dying");
}

/// A RUNNER THAT DIES INSIDE ITS HOLD IS NAMED AS ONE, ITS EXIT WITH IT, AND NO
/// PASS IS COUNTED OVER IT. Forty cases reported passing and then the runner
/// ended without a word about the rest; the 261004 close saw that rendered as
/// forty ran and forty passed with a trailing guess, the exit nowhere.
///
/// THE RECORD IS READ BACK AS WELL AS THE CONSOLE, because a red whose only
/// account is a console the caller may have clipped can be read only by
/// running it again.
#[test]
fn bktd_futurity_names_a_runner_that_died_after_passing() {
    let lure = bktu_Lure::bktu_compose("dying-runner");
    zbktd_dying_course(&lure);
    lure.bktu_commit("compose a suite whose runner dies after its passing cases");

    let family = zbktd_log_family(&lure);
    let stated: Vec<(&str, &OsStr)> = family
        .iter()
        .map(|(name, path)| (*name, path.as_os_str()))
        .collect();

    let out = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR_FUTURITY, "bktd_alive", "bktd_dies"],
        &stated,
    );
    let said = zbktd_said(&out);

    assert_eq!(out.status.code(), Some(ZBKTD_EXIT_FALLERS), "the walk exits with its own code: {}", said);

    let verdict = said
        .lines()
        .find(|line| line.starts_with("RED"))
        .unwrap_or_else(|| panic!("no verdict line: {}", said));
    assert!(
        zbktd_names_an_exit(verdict),
        "the verdict names the exit the runner died with: {}",
        verdict
    );

    let course = said
        .lines()
        .find(|line| line.contains("course suite-dying is red"))
        .unwrap_or_else(|| panic!("no line names the red course: {}", said));
    assert!(zbktd_names_an_exit(course), "the course's red line names the exit: {}", course);

    assert!(
        !said.lines().any(|line| line.contains(" ran, ") && line.contains(" passed")),
        "no passing count is printed over a runner that did not finish: {}",
        said
    );

    let hist = &family[2].1;
    let record = std::fs::read_to_string(hist).unwrap_or_else(|err| panic!("the record stands at {}: {}", hist.display(), err));
    assert!(record.contains(verdict), "the record carries the verdict: {}\n{}", verdict, record);
}

/// Whether a line names an exit code: the word and a number after it.
fn zbktd_names_an_exit(line: &str) -> bool {
    line.split("exit ")
        .skip(1)
        .any(|after| after.chars().next().is_some_and(|first| first.is_ascii_digit()))
}

/// A FUTURITY WITH NOTHING TO NARROW REFUSES and names the derby, spawning
/// nothing: the set unnarrowed is the derby's.
#[test]
fn bktd_futurity_bare_refuses_naming_the_derby() {
    let lure = bktu_Lure::bktu_compose("narrowed-bare");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses");

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_FUTURITY]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success(), "the bare door refuses: {}", out.status);
    assert!(said.contains(BKTD_DOOR_DERBY), "the refusal names the derby: {}", said);
    assert!(!mark.exists(), "nothing ran: {}", mark.display());
}

/// A suite crate carrying no test at all — the crate compiles and declares
/// nothing for a runner to run.
const BKTD_EMPTY: &str = "pub fn stands() -> u32 { 1 }\n";

/// A suite collar over an empty crate, under whichever runner the caller names.
fn zbktd_empty_course(lure: &bktu_Lure, dir: &str, name: &str, runner: &str, tongue: &str) {
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);
    zbktd_crate(lure, dir, name.trim_start_matches("suite-"), "src/lib.rs", BKTD_EMPTY);
    lure.bktu_write(
        &lure.bktu_weaned_at(&format!("{}/bkrr.env", name)),
        &format!(
            "\
BKRR_COLLAR=\"{name}\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"{dir}/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"{dir}/src {dir}/Cargo.toml {dir}/Cargo.lock\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"{runner}\"
BKRR_TONGUE=\"{tongue}\"
"
        ),
    );
}

/// A SUITE THAT RAN NOTHING IS RED, UNDER EITHER RUNNER (operator, 260905). A
/// course reporting green over a suite that declared no test would be the one
/// verdict the kennel must never give: nothing was proven, and the tree reads as
/// proven.
///
/// THE TWO RUNNERS ARE ASSERTED SEPARATELY BECAUSE THEY REDDEN BY DIFFERENT
/// MEANS, and a single hurdle would hide which. Nextest refuses an empty
/// selection itself, and the composer asks it to rather than inheriting it;
/// cargo's own harness reports a clean zero and carries no flag to say otherwise,
/// so there the kennel's own case count is the whole of what stands between an
/// empty suite and a green tree. Driven and measured at this pace: the cargo arm
/// was green before that reading was seated, which is the defect it now holds
/// shut.
#[test]
fn bktd_a_suite_with_no_tests_is_red_under_nextest() {
    let lure = bktu_Lure::bktu_compose("empty-nextest");
    zbktd_empty_course(&lure, "Tools/hollow", "suite-hollow", "bknre_nextest", "bknre_nextest");
    lure.bktu_commit("compose a suite collar over a crate declaring no test");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);

    assert!(
        !out.status.success(),
        "a suite that ran nothing proved nothing, and reads red: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn bktd_a_suite_with_no_tests_is_red_under_cargo() {
    let lure = bktu_Lure::bktu_compose("empty-cargo");
    zbktd_empty_course(&lure, "Tools/hollow", "suite-hollow", "bknre_cargo", "bknre_harness");
    lure.bktu_commit("compose a suite collar over a crate declaring no test");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);

    assert!(
        !out.status.success(),
        "cargo's harness called this run clean, and the kennel's own case count is what \
         reddens it: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A NEXTEST STANDING ON THE PATH IS NEVER SPAWNED, and the launch names the
/// residence it spawned instead.
///
/// THE PLANT IS THE OLD HURDLE'S, READ THE OTHER WAY. A `cargo-nextest`
/// answering a version the kennel does not expect stands ahead of the station's
/// path — the same road a station reaches a differently-versioned nextest by —
/// and where the kennel once REFUSED over it, it now ignores it entirely. The
/// shim is what makes that legible: it echoes a version and exits zero without
/// running anything, so a launch that reached it would report a green having
/// run no case at all. That the course is green AND names the residence is what
/// says the shim was passed over.
///
/// THE KIBBLE IS THE ESTATE'S OWN, copied into the lure rather than restated, so
/// the residence this asserts against is the one *heel* actually wrote.
#[test]
fn bktd_a_nextest_on_the_path_is_never_spawned_and_the_launch_names_the_residence() {
    let lure = bktu_Lure::bktu_compose("drive-nextest-shimmed");
    let station = lure.bktu_stationed();
    lure.bktu_collar_seat();
    station.bktu_nextest_seat(&lure);

    let path = lure.bktu_shimmed("nextest-shimmed", BKTD_STRANGE);

    let out = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR, bktu_Lure::BKTU_COLLAR_NEXTEST],
        &[("PATH", path.as_os_str())],
    );

    let said = String::from_utf8_lossy(&out.stderr).into_owned()
        + &String::from_utf8_lossy(&out.stdout);

    assert!(
        out.status.success(),
        "a shimmed nextest on the path is ignored, so the course still runs: {} {}",
        out.status,
        said
    );

    // THE ABSOLUTE PATH IS THE PROOF. A launch that had reached the shim, or
    // cargo's own plugin resolution, would name neither the tackroom nor the
    // declared version.
    let residence = zbktd_nextest_residence(&station);
    assert!(
        said.contains(&residence),
        "the launch does not name the residence {}: {}",
        residence,
        said
    );

    // THE DISCRIMINATOR: the shim's own version never appears, so this is not a
    // green that merely tolerated it.
    assert!(
        !said.contains(BKTD_STRANGE),
        "the shim's version reached the launch: {}",
        said
    );
}

/// A PATH HOLDING NO NEXTEST STILL RUNS GREEN, and a cargo-runner collar over
/// the same seat runs green too.
///
/// THE PAIR IS THE PROOF, and it is the inverse of the pair this hurdle
/// replaced. While nextest was found on the path, a narrowed path had to REFUSE
/// and the cargo collar had to pass, so that the refusal was shown to be about
/// the runner. Now that nextest is reached at its own residence, the narrowed
/// path must PASS — and the cargo collar is still the control, since a kennel
/// that had quietly stopped running anything would pass both.
#[test]
fn bktd_a_narrowed_path_still_runs_the_nextest_collar_through_the_residence() {
    let lure = bktu_Lure::bktu_compose("drive-nextest-absent");
    let station = lure.bktu_stationed();
    lure.bktu_collar_seat();
    station.bktu_nextest_seat(&lure);

    let path = lure.bktu_nextestless("nextest-absent", BKTD_CARRIED);
    let stated: &[(&str, &OsStr)] = &[("PATH", path.as_os_str())];

    let ran = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR, bktu_Lure::BKTU_COLLAR_NEXTEST],
        stated,
    );

    let said = String::from_utf8_lossy(&ran.stderr).into_owned()
        + &String::from_utf8_lossy(&ran.stdout);

    assert!(
        ran.status.success(),
        "a station holding no nextest anywhere on its path still runs a nextest collar, the \
         program being reached at its residence: {} {}",
        ran.status,
        said
    );

    let residence = zbktd_nextest_residence(&station);
    assert!(
        said.contains(&residence),
        "the launch does not name the residence {}: {}",
        residence,
        said
    );

    let control = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR, bktu_Lure::BKTU_COLLAR_CARGO],
        stated,
    );

    assert!(
        control.status.success(),
        "THE CONTROL: the same seat on the same path runs green under cargo's own runner, so the \
         green above is not a kennel that stopped running anything: {} {}",
        control.status,
        String::from_utf8_lossy(&control.stderr)
    );
}

/// A NEXTEST COLLAR WHOSE KIBBLE IS NOT PLACED REFUSES, NAMING HEEL, and a
/// cargo-runner collar over the same seat runs green.
///
/// THE VERSION IS MOVED RATHER THAN THE STORE, which is the only way to pose an
/// unplaced residence that is still a proof about the residence: a drive told
/// about some other tackroom meets the FENCE first and never reaches the kibble
/// at all. Nothing is removed from the station's store, the toolchain is
/// untouched, and what changes is the one path the pin composes. The cargo
/// collar is the control that says the refusal is about the runner rather than
/// about the seat.
#[test]
fn bktd_an_unplaced_kibble_refuses_a_nextest_collar_and_names_heel() {
    let lure = bktu_Lure::bktu_compose("drive-nextest-unplaced");
    lure.bktu_collar_seat();
    lure.bktu_stationed().bktu_nextest_seat_unplaced(&lure, BKTD_UNPLACED);

    let refused = lure.bktu_drive_door(
        zbktd_kennel(),
        &[BKTD_DOOR, bktu_Lure::BKTU_COLLAR_NEXTEST],
    );

    assert!(
        !refused.status.success(),
        "an unplaced kibble refuses a nextest collar: {}",
        String::from_utf8_lossy(&refused.stderr)
    );

    let said = String::from_utf8_lossy(&refused.stderr);

    for owed in ["heel", "bki_nextest", "cargo-nextest"] {
        assert!(said.contains(owed), "the refusal names '{}': {}", owed, said);
    }

    // NOTHING WAS SPAWNED. The seam announces the invocation it is about to make
    // on its way to making it, so the absence of that line is what says the
    // check stood ahead of the runner rather than beside it.
    assert!(
        !said.contains("runs its suite"),
        "the refusal stands ahead of the runner, so no launch was announced: {}",
        said
    );

    let control = lure.bktu_drive_door(
        zbktd_kennel(),
        &[BKTD_DOOR, bktu_Lure::BKTU_COLLAR_CARGO],
    );

    assert!(
        control.status.success(),
        "THE CONTROL: the same seat with the same declaration runs green under cargo's own \
         runner, so the refusal above is about the program the other collar would have spawned: \
         {} {}",
        control.status,
        String::from_utf8_lossy(&control.stderr)
    );
}

/// Where the estate's own nextest kibble lands its binary, composed the way the
/// door composes it so a hurdle asserts against the same path.
///
/// READ FROM THE DECLARATION rather than spelled, so a bumped version moves this
/// reading with it, and BENEATH THE ROOT THE KENNEL'S OWN READER ANSWERS, so the
/// station's store is reached the way a door reaches it and never spelled here.
/// The station seat is what a hurdle holds to ask.
fn zbktd_nextest_residence(_station: &bktu_lure::bktu_Stationed) -> String {
    let feedroom = bkcq_kibble::bkcq_feedroom()
        .unwrap_or_else(|err| panic!("the suite is driven through a door that states the store: {}", err));

    // THE KIT'S OWN BROOD, read as the set: the declaration the estate weans is
    // this one byte for byte, and a hurdle reading the host's moorings instead
    // would be reading the run executing it.
    let kit = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("Tools/bkk");
    let brood = kit.join("bkd_brood");
    let standing = brood.join("bki_nextest").join("bkrk.env");

    let resolved = bkcq_kibble::bkcq_resolve(
        kit,
        &brood,
        "bki_nextest",
    )
    .unwrap_or_else(|err| panic!("could not resolve the estate's own kibble ({}): {}", standing.display(), err));

    assert!(
        resolved.findings.is_empty(),
        "the estate's own nextest kibble carries findings: {:?}",
        resolved.findings
    );

    resolved.kibble.bkcq_residence(&feedroom).display().to_string()
}

/// The gangline door refuses an uncommitted repository BEFORE it touches the
/// lock.
///
/// IT IS THE ORDERING THAT IS PROVEN HERE, and that is why this hurdle is not
/// spare beside the door-law drive above. That drive establishes that the law
/// applies; this one establishes that it applies AHEAD of the one door whose
/// whole job is to write a file — a door that refused after re-deriving would
/// leave the operator a lock they never asked for, in a tree they were told
/// nothing had happened to. The lock's bytes before and after are the assertion;
/// the refusal is the occasion for taking it.
#[test]
fn bktd_gangline_refuses_a_dirty_lure_before_it_touches_the_lock() {
    let lure = bktu_Lure::bktu_compose("drive-gangline-dirty");

    lure.bktu_write(
        "Cargo.toml",
        "[package]\nname = \"lure\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\
         [lib]\nname = \"lure\"\npath = \"src/lib.rs\"\n\
         [dependencies]\nhitch = { path = \"hitch\" }\n",
    );
    lure.bktu_write("src/lib.rs", "pub fn seated() -> u8 {\n    1\n}\n");
    lure.bktu_write(
        "hitch/Cargo.toml",
        "[package]\nname = \"hitch\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\
         [lib]\nname = \"hitch\"\npath = \"src/lib.rs\"\n",
    );
    lure.bktu_write("hitch/src/lib.rs", "pub fn hitched() -> u8 {\n    2\n}\n");

    // A LOCK THAT DOES NOT ANSWER THE MANIFEST, committed — so a gangline that
    // ran would certainly rewrite it, and an unchanged file is evidence the door
    // never reached cargo rather than evidence there was nothing to do.
    lure.bktu_write(
        "Cargo.lock",
        "# This file is automatically @generated by Cargo.\n\
         # It is not intended for manual editing.\n\
         version = 4\n\n\
         [[package]]\nname = \"lure\"\nversion = \"0.0.1\"\n",
    );
    lure.bktu_write(
        &lure.bktu_weaned_at("suite-lure/bkrr.env"),
        "BKRR_COLLAR=\"suite-lure\"\nBKRR_KIND=\"bknre_suite\"\n\
         BKRR_MANIFEST=\"Cargo.toml\"\nBKRR_TARGET=\"bknre_manifest\"\n\
         BKRR_ROOTS=\"src\"\nBKRR_FEATURES=\"\"\nBKRR_PROFILE=\"test\"\n\
         BKRR_SPEND=\"bknre_reader\"\nBKRR_TAMED_CRATES=\"\"\n\
         BKRR_FERAL_CRATES=\"bknre_tamed\"\nBKRR_MUZZLE=\".\"\nBKRR_EXERGUE=\"bknre_unstruck\"\n\
         BKRR_RUNNER=\"bknre_cargo\"\nBKRR_TONGUE=\"bknre_harness\"\n",
    );
    lure.bktu_commit("stand a lure whose lock does not answer its manifest");

    let lock = lure.bktu_root().join("Cargo.lock");
    let before = std::fs::read(&lock).expect("the lure carries a lock");

    lure.bktu_write(BKTD_STRAY, "// planted by the hurdle\n");

    let out = std::process::Command::new(zbktd_kennel())
        .current_dir(lure.bktu_root())
        .args(["gangline", "suite-lure"])
        .output()
        .expect("the kennel should spawn");

    assert!(
        !out.status.success(),
        "the door law reaches gangline like every other door"
    );

    let said = String::from_utf8_lossy(&out.stderr);
    assert!(
        said.contains(BKCF_REMEDY),
        "the refusal names the remedy ({}) rather than merely reporting the dirt: {}",
        BKCF_REMEDY,
        said
    );

    let after = std::fs::read(&lock).expect("the lock still stands");
    assert_eq!(
        before, after,
        "the lock must be untouched — the refusal stands AHEAD of the re-derivation, not after it"
    );
}

/// SCOOP TAKES EVERY COLLAR'S BUILD DIRECTORY AND REPORTS EACH, over a lure
/// carrying two crates with warm yards.
///
/// TWO CRATES AND NOT ONE, because one proves nothing about a walk: a door that
/// removed only the first collar it found would clear a single-crate hurdle
/// perfectly.
#[test]
fn bktd_scoop_takes_every_collar_and_reports_each() {
    let lure = bktu_Lure::bktu_compose("scoop-both");
    lure.bktu_crate("one", "suite-one");
    lure.bktu_crate("two", "suite-two");
    lure.bktu_commit("stand two crates up");

    let first = zbktd_warm(&lure, "suite-one");
    let second = zbktd_warm(&lure, "suite-two");

    assert!(first.is_dir() && second.is_dir(), "both yards must be warm before the sweep");
    assert_ne!(first, second, "two crates must build into two directories");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[ZBKTD_SCOOP]);

    let said = String::from_utf8_lossy(&out.stderr);

    assert!(out.status.success(), "the sweep should land: {} {}", out.status, said);
    assert!(!first.exists(), "the first yard should be gone: {}", first.display());
    assert!(!second.exists(), "the second yard should be gone: {}", second.display());
    assert!(
        said.contains("suite-one") && said.contains("suite-two"),
        "the report should name every collar it swept: {}",
        said
    );
}

/// AN INVALID COLLAR REFUSES THE WHOLE SWEEP BEFORE THE FIRST DELETION, and the
/// standing yard is what proves the ordering: a door that swept the sound collars
/// and reported the unsound one would print a message this hurdle could not tell
/// from a refusal.
#[test]
fn bktd_scoop_refuses_a_planted_invalid_collar_before_deleting() {
    let lure = bktu_Lure::bktu_compose("scoop-invalid");
    lure.bktu_crate("one", "suite-one");
    lure.bktu_commit("stand a sound crate up");

    let yard = zbktd_warm(&lure, "suite-one");

    // The planted collar names a manifest that does not stand, which is a
    // conformance finding rather than a malformed file: the walk finds it, the
    // resolver reads it, and the validation refuses it.
    lure.bktu_write(
        &lure.bktu_weaned_at("suite-planted/bkrr.env"),
        "BKRR_COLLAR=\"suite-planted\"\nBKRR_KIND=\"bknre_suite\"\n\
         BKRR_MANIFEST=\"nowhere/Cargo.toml\"\nBKRR_TARGET=\"bknre_manifest\"\n\
         BKRR_ROOTS=\"nowhere\"\nBKRR_FEATURES=\"\"\nBKRR_PROFILE=\"test\"\n\
         BKRR_SPEND=\"bknre_reader\"\nBKRR_TAMED_CRATES=\"\"\n\
         BKRR_FERAL_CRATES=\"bknre_tamed\"\nBKRR_MUZZLE=\".\"\nBKRR_EXERGUE=\"bknre_unstruck\"\n\
         BKRR_RUNNER=\"bknre_cargo\"\nBKRR_TONGUE=\"bknre_harness\"\n",
    );
    lure.bktu_commit("plant an invalid collar");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[ZBKTD_SCOOP]);

    let said = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success(), "an invalid collar refuses the sweep: {}", said);
    assert!(
        said.contains("suite-planted"),
        "the refusal should name the collar it found: {}",
        said
    );
    assert!(
        yard.is_dir(),
        "nothing may be removed before every collar is validated: {}",
        yard.display()
    );
}

/// A BUILD DIRECTORY OUTSIDE THE TREE REFUSES THE WHOLE SWEEP, naming the path,
/// and the sound collar's yard stands afterwards.
///
/// The redirect is cargo's own configuration, so what the door refuses is a real
/// answer from cargo rather than a path this hurdle composed for it.
#[test]
fn bktd_scoop_refuses_a_build_directory_outside_the_tree() {
    let lure = bktu_Lure::bktu_compose("scoop-outside");
    lure.bktu_crate("one", "suite-one");
    lure.bktu_commit("stand a sound crate up");

    let yard = zbktd_warm(&lure, "suite-one");

    let elsewhere = lure
        .bktu_root()
        .parent()
        .expect("the lure stands under a root")
        .join("bktd-outside");

    lure.bktu_write(
        ".cargo/config.toml",
        &format!("[build]\ntarget-dir = \"{}\"\n", elsewhere.display()),
    );
    lure.bktu_commit("redirect the build directory out of the tree");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[ZBKTD_SCOOP]);

    let said = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success(), "a directory outside the tree refuses: {}", said);
    assert!(
        said.contains(&*elsewhere.to_string_lossy()),
        "the refusal should name the path it will not remove: {}",
        said
    );
    assert!(
        yard.is_dir(),
        "the sound collar's yard must stand after a refusal: {}",
        yard.display()
    );
}

/// A GREEN COURSE BANKS ITS RECORD IN BOTH SEATS, and the two copies are the
/// same bytes.
///
/// THE FIELDS ARE READ BACK THROUGH THE ADMIT rather than matched as strings,
/// which is the point of the roster being one definition: a hurdle asserting on
/// substrings would pass over a record whose fields had drifted apart from the
/// reader the engine will harvest it with.
#[test]
fn bktd_a_green_course_banks_its_record_in_both_seats() {
    let lure = bktu_Lure::bktu_compose("record-green");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses, the second of them green");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, ZBKTD_GREEN]);

    assert!(
        out.status.success(),
        "the green course runs: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let leaf = format!("{}.{}", ZBKTD_GREEN, bkcc_record::BKCC_EXTENSION);
    let banked = lure.bktu_output().join(&leaf);
    let twin = lure.bktu_temp().join(&leaf);

    for seat in [&banked, &twin] {
        assert!(seat.is_file(), "the record stands at {}", seat.display());
    }

    let said = std::fs::read_to_string(&banked).expect("the record is readable");

    assert_eq!(
        said,
        std::fs::read_to_string(&twin).expect("the twin is readable"),
        "the durable twin is the same bytes as the convenience copy"
    );

    let read = bkcc_record::bkcc_admit(&said, &leaf)
        .unwrap_or_else(|err| panic!("the door's own record admits: {}", err));

    assert_eq!(read.collar, ZBKTD_GREEN, "the record names the collar it measured");
    assert_eq!(read.driven, 1, "the course drove the crate's one hurdle: {}", said);
    assert_eq!(read.passed, 1, "and it passed: {}", said);
    assert_eq!(read.held, 1, "out of the one the suite holds: {}", said);

    // A WHOLE COURSE TAKES A TIME, and the narrowing is what says it ran whole.
    // Mush was given no pattern, so both halves of that one fact must read that
    // way or the record disagrees with itself.
    assert!(read.narrowing.is_none(), "nothing narrowed the course: {}", said);
    assert!(read.wall.is_some(), "so the course carries its wall time: {}", said);

    assert!(
        !read.position.trim().is_empty(),
        "the record carries the position the course ran at: {}",
        said
    );
}

/// A RED COURSE BANKS NOTHING, which is the cinch's own half that no green run
/// can prove: the studbook keeps the last position a suite ran green at and
/// never that it was red.
///
/// THE GREEN DRIVE ABOVE IS THIS HURDLE'S CONTROL. Without one, a door that had
/// stopped writing records altogether would clear this assertion, an absence
/// being exactly what it looks for.
#[test]
fn bktd_a_red_course_banks_nothing() {
    let lure = bktu_Lure::bktu_compose("record-red");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses, the first of them red");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, ZBKTD_RED_COURSE]);

    assert!(!out.status.success(), "the red course reddens the door: {}", out.status);

    let leaf = format!("{}.{}", ZBKTD_RED_COURSE, bkcc_record::BKCC_EXTENSION);

    for seat in [lure.bktu_output(), lure.bktu_temp()] {
        assert!(
            !seat.join(&leaf).exists(),
            "a red course leaves no record at {}",
            seat.join(&leaf).display()
        );
    }
}

/// A GREEN COURSE WITH NOWHERE TO BANK ITS RECORD REFUSES, naming the variable
/// that named no directory.
///
/// THE COURSE ITSELF PASSED, which is what makes this the honest shape rather
/// than a harsh one: the measurement is the launch side's one durable output, so
/// a door that exited zero here would report a tree as measured on the strength
/// of a file nobody wrote. The exit is asserted and not merely its nonzero-ness,
/// because a caller reading the exit alone must be able to tell this refusal
/// from a course whose hurdles failed.
#[test]
fn bktd_a_green_course_with_nowhere_to_bank_refuses_naming_the_variable() {
    let lure = bktu_Lure::bktu_compose("record-nowhere");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses, the second of them green");

    // The seat states the pair on every drive, so a hurdle proving the absence
    // states it back to nothing — the last statement wins.
    let stated: &[(&str, &OsStr)] = &[(bkcc_record::BKCC_OUTPUT_DIR_VAR, OsStr::new(""))];

    let out = lure.bktu_drive_stating(zbktd_kennel(), &[BKTD_DOOR, ZBKTD_GREEN], stated);

    let said = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_RECORD),
        "the refusal takes the course record's own exit: {}",
        said
    );
    assert!(
        said.contains(bkcc_record::BKCC_OUTPUT_DIR_VAR),
        "the refusal names the variable that named no directory: {}",
        said
    );

    // AND NOTHING HALF-LANDED. The output directory refused, and the durable
    // twin's directory stands — so a writer that had banked one copy before
    // reaching the refusal would leave a record of a course the door then called
    // red.
    let leaf = format!("{}.{}", ZBKTD_GREEN, bkcc_record::BKCC_EXTENSION);
    assert!(
        !lure.bktu_temp().join(&leaf).exists(),
        "neither copy lands when one of them cannot: {}",
        lure.bktu_temp().join(&leaf).display()
    );
}

/// A RECORD THAT ALREADY STANDS REFUSES RATHER THAN BEING OVERWRITTEN, and the
/// standing bytes are what proves it: one write per collar per dispatch is the
/// whole cardinality of a fact file, so a second means two courses claimed one
/// identity.
#[test]
fn bktd_a_record_that_already_stands_refuses_rather_than_overwriting() {
    let lure = bktu_Lure::bktu_compose("record-twice");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses, the second of them green");

    let leaf = format!("{}.{}", ZBKTD_GREEN, bkcc_record::BKCC_EXTENSION);
    let standing = lure.bktu_output().join(&leaf);

    std::fs::write(&standing, ZBKTD_STANDING).expect("the hurdle seeds the seat");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, ZBKTD_GREEN]);

    let said = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_RECORD),
        "a second write of one key refuses: {}",
        said
    );
    assert_eq!(
        std::fs::read_to_string(&standing).expect("the seeded record is readable"),
        ZBKTD_STANDING,
        "and the record that stood is untouched"
    );
}

/// The two courses the fixture above composes, named where a hurdle asks for one
/// so a typo could not quietly drive the other.
const ZBKTD_GREEN: &str = "suite-bravo";
const ZBKTD_RED_COURSE: &str = "suite-alfa";

/// The exit a suite door takes when a course ran green and its record could not
/// be banked. Spelled here because the constant is the binary's own and this
/// hurdle stands outside it; the two are held together by the door's console,
/// which names the condition either way.
const ZBKTD_EXIT_RECORD: i32 = 84;

/// The kibble door word, spelled once so a typo could not turn these into drives
/// of the bare kennel, which exits 0.
const ZBKTD_HEEL: &str = "heel";

/// The exit *heel* takes when a kibble converge refuses.
const ZBKTD_EXIT_KIBBLE: i32 = 85;

#[test]
fn bktd_heel_over_a_cold_haynet_fetches_proves_builds_and_places() {
    // THE WHOLE CONVERGE, END TO END, over an archive the lure serves itself: no
    // registry is reached and no network is touched, so what this measures is
    // the door rather than crates.io's uptime.
    let lure = bktu_Lure::bktu_compose("drive-kibble-cold");
    let haynet = lure.bktu_haynet();
    haynet.bktu_kibble_seat(&lure, ZBKTD_HEEL_KIBBLE, &lure.bktu_kibble_seal());
    let residence = haynet.bktu_kibble_residence(ZBKTD_HEEL_KIBBLE);

    // THE COLD STATE IS ASSERTED RATHER THAN ASSUMED. Without this the hurdle
    // below could pass over a residence some earlier drive had left standing,
    // and would then be proving nothing about a fetch.
    assert!(!residence.is_file(), "{}", residence.display());

    let out = haynet.bktu_drive_door(&lure, zbktd_kennel(), &[ZBKTD_HEEL, ZBKTD_HEEL_KIBBLE]);

    // THE CONSOLE NAMES THE PLACEMENT BENEATH THE POSED ROOT, in the door's own
    // words: a converge that placed anywhere else read the station and never
    // the root this drive stated.
    let said = String::from_utf8_lossy(&out.stderr).into_owned()
        + &String::from_utf8_lossy(&out.stdout);
    assert!(
        said.contains(&format!("placed at {}", residence.display())),
        "the first drive does not name a placement at {}: {}",
        residence.display(),
        said
    );

    assert!(
        out.status.success(),
        "the converge refused: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        residence.is_file(),
        "nothing stands at {}",
        residence.display()
    );

    // THE PLACED FILE IS THE PROGRAM AND NOT A HALF-WRITTEN ONE, which the
    // rename is what guarantees; a hurdle that only tested the path would pass
    // over a truncated file.
    let answered = std::process::Command::new(&residence)
        .output()
        .expect("the placed binary runs");
    assert!(
        String::from_utf8_lossy(&answered.stdout).contains("the lure's own fetched program"),
        "the placed binary said {:?}",
        String::from_utf8_lossy(&answered.stdout)
    );

    // DRIVEN AGAIN IT REPORTS CURRENT AND FETCHES NOTHING. The discriminator is
    // the console rather than the residence, which stands either way, and it is
    // read in the door's own words for each arm.
    let again = haynet.bktu_drive_door(&lure, zbktd_kennel(), &[ZBKTD_HEEL, ZBKTD_HEEL_KIBBLE]);
    assert!(again.status.success());

    let said = String::from_utf8_lossy(&again.stderr).into_owned()
        + &String::from_utf8_lossy(&again.stdout);
    assert!(
        said.contains(&format!("current at {}", residence.display())),
        "the second drive does not name the current residence {}: {}",
        residence.display(),
        said
    );
    assert!(
        !said.contains("placed at"),
        "a current drive said it placed: {}",
        said
    );
}

/// The kibble name the haynet drives pose, which keys nothing: the residence is
/// keyed on the seat.
const ZBKTD_HEEL_KIBBLE: &str = "kibble-haynet";

#[test]
fn bktd_a_kibble_carrying_a_wrong_seal_refuses_naming_both_digests_and_places_nothing() {
    // THE ARCHIVE IS THE SAME BYTES the hurdle above converges. What parts the
    // two drives is the DECLARATION alone, which is what makes this a proof
    // about the seal rather than about a broken download.
    let lure = bktu_Lure::bktu_compose("drive-kibble-seal");
    let honest = lure.bktu_kibble_seal();
    let wrong = zbktd_altered(&honest);

    let haynet = lure.bktu_haynet();
    haynet.bktu_kibble_seat(&lure, ZBKTD_HEEL_KIBBLE, &wrong);
    let residence = haynet.bktu_kibble_residence(ZBKTD_HEEL_KIBBLE);

    let out = haynet.bktu_drive_door(&lure, zbktd_kennel(), &[ZBKTD_HEEL, ZBKTD_HEEL_KIBBLE]);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_KIBBLE),
        "the refusal took {:?}: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    let said = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(said.contains(&wrong), "the declared digest is unnamed: {}", said);
    assert!(said.contains(&honest), "the fetched digest is unnamed: {}", said);

    // NOTHING WAS PLACED, which is the half a reader cannot see in a message.
    assert!(
        !residence.is_file(),
        "a refused seal left {} standing",
        residence.display()
    );

    // THE ROOT STANDS, which the seat never made: the converge's lock made it,
    // so the child read the root this drive stated.
    assert!(
        haynet.bktu_root().is_dir(),
        "the haynet's root does not stand at {}, so the child never read it",
        haynet.bktu_root().display()
    );
}

/// One hex digit of a digest changed, so the seal is WRONG rather than
/// malformed.
///
/// THE SHAPE MUST SURVIVE, which is why this substitutes rather than truncates:
/// a short digest is caught at validation and would never reach the fetch, so a
/// hurdle about the CONVERGE's seal check must hand it a digest the validator
/// admits.
fn zbktd_altered(honest: &str) -> String {
    let mut said: Vec<char> = honest.chars().collect();
    said[0] = if said[0] == 'a' { 'b' } else { 'a' };
    said.into_iter().collect()
}

/// What the seeded record carries, so the assertion that it is untouched is
/// about these bytes and not about a file merely still existing.
const ZBKTD_STANDING: &str = "# seeded by the hurdle, and not a record\n";

/// The delouse's own memory, spelled here because the constant is the binary's
/// and this hurdle stands outside it. A drive states its day, so a hurdle knows
/// exactly which one a record must carry.
const ZBKTD_DAY_FILE: &str = "bkcn_delouse.day";
const ZBKTD_DAY: &str = "20260101";
const ZBKTD_NEXT_DAY: &str = "20260102-000000-0-0";

/// Where the delouse keeps its day for a given lure: the loosebox the drive was
/// handed, and no directory composed by climbing out of another.
fn zbktd_day(lure: &bktu_Lure) -> std::path::PathBuf {
    lure.bktu_loosebox().join(ZBKTD_DAY_FILE)
}

/// THE DELOUSE FIRES FROM THE CONVERGE ON A DAY WITH NO RECORD: every collar's
/// build directory is swept and named, and the day is recorded last.
///
/// TWO CRATES AND NOT ONE, on scoop's own reasoning: a door that removed only
/// the first collar it found would clear a single-crate hurdle perfectly.
#[test]
fn bktd_the_converge_delouses_a_yard_with_no_day() {
    let lure = bktu_Lure::bktu_compose("delouse-converge");
    lure.bktu_crate("one", "suite-one");
    zbktd_app(&lure);
    lure.bktu_commit("stand a suite and an app up");

    let yard = zbktd_warm(&lure, "suite-one");
    let day = zbktd_day(&lure);

    assert!(yard.is_dir(), "the yard must be warm before the sweep");
    assert!(!day.exists(), "no day may stand before the first drive");

    // THE PER-COLLAR NAMING RIDES THE VERBOSE POSITION, on the quiet flavor's
    // own freeze: the day file below is what the quiet drive itself must get
    // right, and the dial is armed here only to make the naming assertion
    // meaningful again.
    let out = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR_HEEL, "app-lure"],
        &[(bkco_output::BKCO_VERBOSE_VAR, OsStr::new("1"))],
    );
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(out.status.success(), "the converge should land: {} {}", out.status, said);
    assert!(
        said.contains("suite-one"),
        "the sweep should name every collar it took: {}",
        said
    );
    assert_eq!(
        std::fs::read_to_string(&day)
            .expect("the day should be recorded")
            .trim(),
        ZBKTD_DAY,
        "the day recorded is the day the drive stated"
    );
}

/// The cargo verb a forge build door hands the launch seam, spelled once so the
/// drive below reads as the door's own call rather than a composed one.
const ZBKTD_BUILD: &str = "build";

/// Lay an app collar whose residence stands INSIDE its own build directory, the
/// shape the matricula's app collar wears, and hand back where the binary will
/// stand once built.
///
/// THE BINARY ANSWERS ITS EXERGUE FROM A FILE THE ELECTION DOES NOT WALK. The
/// stamp stands beside the crate rather than under its elected roots, so the
/// position it carries can be the one the roots were committed at: a stamp
/// inside the roots would move the very position it states.
fn zbktd_yard_app(lure: &bktu_Lure) -> std::path::PathBuf {
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);
    zbktd_crate(
        lure,
        "Tools/yard",
        "yard",
        "src/main.rs",
        "fn main() {\n    if std::env::args().nth(1).as_deref() == Some(\"--exergue\") {\n        print!(\"{}\", include_str!(\"../stamp.txt\"));\n    }\n}\n",
    );
    lure.bktu_write(
        &lure.bktu_weaned_at("app-yard/bkrr.env"),
        "\
BKRR_COLLAR=\"app-yard\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Tools/yard/Cargo.toml\"
BKRR_TARGET=\"yard\"
BKRR_ROOTS=\"Tools/yard/src Tools/yard/Cargo.toml Tools/yard/Cargo.lock\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"Tools/yard/target/release\"
BKRR_BYNAME=\"yard\"
",
    );

    lure.bktu_root().join("Tools/yard/target/release/yard")
}

/// A BUILD DOOR THAT DELOUSES FIRST LEAVES THE DERBY WHAT IT BUILT. The forge's
/// build doors build through the launch seam, which never sweeps, into a yard
/// the day's first derby would otherwise sweep — and an app collar resident
/// inside that yard then meets the derby's preflight as the swept shape, after
/// every build the caller ran.
///
/// THE DRIVES ARE THE FORGE DOOR'S OWN ORDER: the sweep-taking arm, then the
/// launch seam's build, then the derby. The yard is warm with a stale directory
/// beforehand, so the arm has something to take, and the day stands unrecorded,
/// so the arm is the day's first kennel act.
///
/// THE CONTROL IS THE COURSE'S MARK. A derby that refused at its preflight runs
/// no course, so the mark standing is what says the walk went past the
/// preflight rather than merely exiting green over nothing.
#[test]
fn bktd_a_build_door_that_delouses_first_leaves_the_derby_its_artifact() {
    let lure = bktu_Lure::bktu_compose("delouse-build-door");
    let residence = zbktd_yard_app(&lure);

    zbktd_crate(&lure, "Tools/bravo", "bravo", "src/lib.rs", BKTD_MARKED);
    zbktd_suite_collar(&lure, "Tools/bravo", "suite-bravo");
    lure.bktu_write(".gitignore", "target/\nTools/bravo/ran.txt\n");
    lure.bktu_commit("an app collar resident inside its own yard, beside one green course");

    // THE STAMP STATES THE POSITION THE ROOTS WERE COMMITTED AT, and is
    // committed after them so the tree stays clean for the door law.
    let struck = zbktd_head(&lure);
    lure.bktu_write("Tools/yard/stamp.txt", &format!("{}\n", struck));
    lure.bktu_commit("the stamp the yard's binary answers its exergue with");

    let stale = zbktd_warm(&lure, "app-yard");
    let day = zbktd_day(&lure);
    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    assert!(stale.is_dir(), "the yard must be warm before the sweep");
    assert!(!day.exists(), "no day may stand before the first drive");

    let swept = lure.bktu_drive_door(zbktd_kennel(), &[bkco_output::BKCO_VERB_DELOUSE]);
    let built = lure.bktu_drive_door(
        zbktd_kennel(),
        &[ZBKTD_BUILD, "--release", "--manifest-path", "Tools/yard/Cargo.toml"],
    );
    let built_said = String::from_utf8_lossy(&built.stderr).into_owned();

    assert!(
        built.status.success(),
        "the launch seam's build lands: {} {}",
        built.status,
        built_said
    );
    assert!(
        residence.is_file(),
        "the build seats the binary at the residence inside its yard: {}",
        residence.display()
    );

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(
        !said.contains("nothing stands at"),
        "the derby's preflight found the artifact the build seated: {}",
        said
    );
    assert!(
        out.status.success(),
        "the derby runs green over the built residence: {} {}",
        out.status,
        said
    );
    assert!(
        mark.exists(),
        "and its course ran, leaving the mark at {}",
        mark.display()
    );
    assert!(
        swept.status.success(),
        "the sweep-taking arm lands: {} {}",
        swept.status,
        String::from_utf8_lossy(&swept.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(&day)
            .expect("the arm should record the day it swept for")
            .trim(),
        ZBKTD_DAY,
        "the arm records the day, so the derby after it sweeps nothing"
    );
}

/// A SECOND DRIVE ON THE SAME DAY SWEEPS NOTHING, and the standing yard is what
/// proves it: a door that reported the day and swept anyway would print a line
/// this hurdle could not tell from a skip.
///
/// THE YARD IS WARMED AFTER THE FIRST DRIVE, which is the whole shape of the
/// assertion. The first drive removes whatever stood; what the second must leave
/// alone is a directory that came back, exactly as a build between two drives
/// would leave one.
#[test]
fn bktd_a_second_converge_the_same_day_sweeps_nothing() {
    let lure = bktu_Lure::bktu_compose("delouse-cadence");
    lure.bktu_crate("one", "suite-one");
    zbktd_app(&lure);
    lure.bktu_commit("stand a suite and an app up");

    let first = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);
    assert!(
        first.status.success(),
        "the first converge should land: {}",
        String::from_utf8_lossy(&first.stderr)
    );

    let yard = zbktd_warm(&lure, "suite-one");
    assert!(yard.is_dir(), "the yard must be warm again before the second drive");

    // THE ALREADY-SWEPT REPORT RIDES THE VERBOSE POSITION TOO, so the dial is
    // armed here to keep this assertion meaningful under the quiet flavor's
    // freeze.
    let out = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR_HEEL, "app-lure"],
        &[(bkco_output::BKCO_VERBOSE_VAR, OsStr::new("1"))],
    );
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(out.status.success(), "the second converge should land: {} {}", out.status, said);
    assert!(
        said.contains(ZBKTD_DAY),
        "the door should report the day it read: {}",
        said
    );
    assert!(
        yard.is_dir(),
        "a second drive on the same day sweeps nothing: {}",
        yard.display()
    );
}

/// THE DELOUSE SPARES THE DIRECTORY ITS OWN BINARY STANDS IN, and takes the
/// neighbour in the same drive — which is the control the sparing is worth
/// nothing without: a door that swept nothing at all would pass the first
/// assertion exactly as a door keeping the exemption does.
///
/// THE HURDLE DRIVES A COPY OF THE KENNEL STANDING INSIDE THE LURE, and that is
/// the whole mechanism rather than a convenience. The exemption is keyed on the
/// RUNNING EXECUTABLE's own ancestry, so a drive of the built binary at its real
/// residence could never exercise it: no lure collar's build directory is an
/// ancestor of a binary standing in this repository. Copying the kennel into one
/// collar's yard puts the two in exactly the relation the delouse must
/// recognize, and `bktu_drive_door` taking the binary as a parameter is what
/// makes that composable at all.
///
/// IT IS THE DOOR'S OWN ARM AND NOT THE PREDICATE'S. `bkcy_shelters` is proven
/// over posed paths beside the sweep's other sightings
/// (`Tools/bkk/bk0/src/bkty_sweep.rs`); what only a drive can prove is that the
/// delouse LOOP consults it, so an exemption lifted out of that loop reddens
/// here and nowhere else.
#[test]
fn bktd_the_delouse_spares_the_yard_its_own_binary_stands_in() {
    let lure = bktu_Lure::bktu_compose("delouse-sheltered");
    lure.bktu_crate("one", "suite-one");
    lure.bktu_crate("two", "suite-two");
    zbktd_app(&lure);
    lure.bktu_commit("stand two suites and an app up");

    let sheltering = zbktd_warm(&lure, "suite-one");
    let neighbour = zbktd_warm(&lure, "suite-two");

    // THE COPY STANDS WHERE A BUILT LAUNCHABLE WOULD, under the sheltering
    // collar's own build directory, so the ancestry the delouse reads is the
    // ordinary one and not a shape composed to be recognized.
    let standing = sheltering.join("release");
    std::fs::create_dir_all(&standing).expect("the hurdle composes the residence it poses");
    let posed = standing.join("bkx");
    bkp::bkprs_scrivener::bkprs_program_copy(&zbktd_kennel(), &posed)
        .unwrap_or_else(|err| panic!("{}", err));

    assert!(
        sheltering.is_dir() && neighbour.is_dir(),
        "both yards must be warm before the sweep"
    );

    let out = lure.bktu_drive_door(&posed, &[BKTD_DOOR_HEEL, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(out.status.success(), "the converge should land: {} {}", out.status, said);
    assert!(
        sheltering.is_dir(),
        "the delouse took the yard holding the binary performing it: {} — {}",
        sheltering.display(),
        said
    );
    assert!(
        posed.is_file(),
        "the spared yard must still hold the binary itself: {}",
        posed.display()
    );
    assert!(
        !neighbour.exists(),
        "the sweep spared a yard it shelters nothing of: {} — {}",
        neighbour.display(),
        said
    );
}

/// A NEW DAY SWEEPS AGAIN, which is the control the hurdle above is worth
/// nothing without: a door that never swept twice would pass that assertion
/// exactly as a door keeping a cadence does.
#[test]
fn bktd_a_converge_on_a_new_day_sweeps_again() {
    let lure = bktu_Lure::bktu_compose("delouse-newday");
    lure.bktu_crate("one", "suite-one");
    zbktd_app(&lure);
    lure.bktu_commit("stand a suite and an app up");

    let first = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);
    assert!(
        first.status.success(),
        "the first converge should land: {}",
        String::from_utf8_lossy(&first.stderr)
    );

    let yard = zbktd_warm(&lure, "suite-one");
    assert!(yard.is_dir(), "the yard must be warm again before the second drive");

    let out = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR_HEEL, "app-lure"],
        &[(bktu_lure::BKTU_NOW_VAR, std::ffi::OsStr::new(ZBKTD_NEXT_DAY))],
    );
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(out.status.success(), "the converge should land: {} {}", out.status, said);
    assert!(
        !yard.exists(),
        "a new day sweeps the yard again: {}",
        yard.display()
    );
    assert_eq!(
        std::fs::read_to_string(zbktd_day(&lure))
            .expect("the day should be recorded")
            .trim(),
        &ZBKTD_NEXT_DAY[..8],
        "the record moves to the day the drive stated"
    );
}

/// AN INVALID COLLAR REFUSES THE SWEEP BEFORE THE FIRST DELETION AND RECORDS NO
/// DAY. The standing yard proves the ordering; the absent record proves the door
/// did not claim a sweep it never performed, which is the failure a day written
/// first would cause silently and forever.
#[test]
fn bktd_the_delouse_refuses_a_planted_invalid_collar_before_deleting() {
    let lure = bktu_Lure::bktu_compose("delouse-invalid");
    lure.bktu_crate("one", "suite-one");
    zbktd_app(&lure);
    lure.bktu_commit("stand a suite and an app up");

    let yard = zbktd_warm(&lure, "suite-one");

    lure.bktu_write(
        &lure.bktu_weaned_at("suite-planted/bkrr.env"),
        "BKRR_COLLAR=\"suite-planted\"\nBKRR_KIND=\"bknre_suite\"\n\
         BKRR_MANIFEST=\"nowhere/Cargo.toml\"\nBKRR_TARGET=\"bknre_manifest\"\n\
         BKRR_ROOTS=\"nowhere\"\nBKRR_FEATURES=\"\"\nBKRR_PROFILE=\"test\"\n\
         BKRR_SPEND=\"bknre_reader\"\nBKRR_TAMED_CRATES=\"\"\n\
         BKRR_FERAL_CRATES=\"bknre_tamed\"\nBKRR_MUZZLE=\".\"\nBKRR_EXERGUE=\"bknre_unstruck\"\n\
         BKRR_RUNNER=\"bknre_cargo\"\nBKRR_TONGUE=\"bknre_harness\"\n",
    );
    lure.bktu_commit("plant an invalid collar");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success(), "an invalid collar refuses the drive: {}", said);
    assert!(
        said.contains("suite-planted"),
        "the refusal should name the collar it found: {}",
        said
    );
    assert!(
        yard.is_dir(),
        "nothing may be removed before every collar is validated: {}",
        yard.display()
    );
    assert!(
        !zbktd_day(&lure).exists(),
        "a sweep that never happened records no day"
    );
}

/// THE LAUNCH SEAM NEVER DELOUSES. A course is a run, and a build that happened
/// because someone asked to run something is a build nobody decided to do — so
/// mush leaves both the yard and the record exactly as it found them.
#[test]
fn bktd_the_launch_seam_does_not_delouse() {
    let lure = bktu_Lure::bktu_compose("delouse-launch");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses");

    let day = zbktd_day(&lure);
    assert!(!day.exists(), "no day may stand before the drive");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, ZBKTD_GREEN]);

    assert!(
        out.status.success(),
        "the green course should land: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !day.exists(),
        "a run records no day, the delouse never having fired from it: {}",
        day.display()
    );
}

/// THE DELOUSE FIRES FROM THE WALK TOO, which is the test verb's half of the
/// cadence and is not inherited from the converge: derby composes its own course
/// loop, so a step hoisted into one door alone would leave the other unswept
/// while reading as covered.
#[test]
fn bktd_the_walk_delouses_before_its_courses() {
    let lure = bktu_Lure::bktu_compose("delouse-walk");
    zbktd_two_courses(&lure);
    lure.bktu_commit("compose two courses");

    let day = zbktd_day(&lure);
    assert!(!day.exists(), "no day may stand before the drive");

    // THE PRE-SWEEP ANNOUNCEMENT RIDES THE VERBOSE POSITION, so the dial is
    // armed here to keep this assertion meaningful; the day FILE below is what
    // proves the ordering under the quiet flavor's own freeze.
    let out = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR_DERBY],
        &[(bkco_output::BKCO_VERBOSE_VAR, OsStr::new("1"))],
    );
    let said = String::from_utf8_lossy(&out.stderr);

    // The walk carries a red course, which this fixture composes deliberately;
    // what the hurdle asks is whether the sweep happened AHEAD of the courses,
    // and the recorded day is what says so.
    assert!(
        said.contains(ZBKTD_DAY),
        "the walk should report the day it swept for: {}",
        said
    );
    assert_eq!(
        std::fs::read_to_string(&day)
            .expect("the day should be recorded")
            .trim(),
        ZBKTD_DAY,
        "the walk records the day before it runs a course"
    );
}

/// The door the dispatched arm is driven through: read-only, and the one that
/// prints both the election and the geography on its proclamation.
///
/// A READ-ONLY DOOR IS WHAT THIS ARM WANTS. The reading being proven is the
/// geography and the election it feeds, and a door that also built something
/// would make a failing drive ambiguous between the two.
const ZBKTD_DOOR_TATTOO: &str = "tattoo";

/// The exit a door takes when the repository's geography does not derive,
/// spelled here so an assertion reads against a number rather than a count of
/// doors.
const ZBKTD_EXIT_GEOGRAPHY: i32 = 82;

/// A lure's root as git and the kennel name it: resolved, since git answers a
/// directory by its real path and a temp root may be reached through a symlink.
fn zbktd_real(path: &std::path::Path) -> std::path::PathBuf {
    std::fs::canonicalize(path)
        .unwrap_or_else(|err| panic!("could not resolve {}: {}", path.display(), err))
}

/// Lay the app collar the dispatched arm is driven over, and hand back the
/// committed position it stands at.
///
/// THE COLLAR IS THE INSTALLING SHAPE — a residence away from cargo's output,
/// under a name cargo did not strike — because that is the shape whose residence
/// join actually differs between the two roots. A collar whose residence WAS the
/// output directory would put the delivered candidate under a `target/` the
/// second lure has never built, and the arm would be proven over an absence.
fn zbktd_geography_lure(lure: &bktu_Lure) -> String {
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);
    zbktd_crate(lure, "Tools/lure", "lure", "src/main.rs", "fn main() {}\n");
    lure.bktu_write(
        &lure.bktu_weaned_at("app-lure/bkrr.env"),
        "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"lure\"
BKRR_ROOTS=\"Tools/lure/src Tools/lure/Cargo.toml Tools/lure/Cargo.lock\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"Tools/bin\"
BKRR_BYNAME=\"lurex\"
",
    );
    lure.bktu_commit("compose an app collar for the dispatched arm");

    zbktd_head(lure)
}

/// The lure's own `HEAD`, which a hurdle needs to compose an answerer stating a
/// position the repository actually holds.
fn zbktd_head(lure: &bktu_Lure) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(lure.bktu_root())
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap_or_else(|err| panic!("could not read the lure's HEAD: {}", err));

    assert!(out.status.success(), "git would not read the lure's HEAD");

    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Plant a binary at a lure's residence join that answers the exergue with one
/// composed position, and SAYS WHICH ONE IT IS when run any other way.
///
/// A PROGRAM AND NOT A CLAIM, on the composing surface's own rule for a shimmed
/// nextest: what a residence holds is decided by what answers when it is
/// spawned, so a hurdle proving the election reads a delivered binary composes
/// one that answers.
///
/// THE MARKER IS WHAT TELLS TWO RESIDENCES APART FROM OUTSIDE. Under dispatch
/// there are two of these standing, and the only way a hurdle can say WHICH ONE
/// the door spawned is to have them say so themselves — an exit code alone
/// cannot, both being the same program.
fn zbktd_answerer(lure: &bktu_Lure, marker: &str, answer: &str) -> std::path::PathBuf {
    let planted = lure.bktu_root().join("Tools/bin/lurex");

    bkp::bkprs_scrivener::bkprs_program_write(
        &planted,
        format!(
            "#!/bin/sh\ntest \"$1\" = \"--exergue\" || {{ echo \"{}\"; exit 0; }}\ncat              <<'ANSWER'\n{}\nANSWER\n",
            marker, answer
        ),
    )
    .unwrap_or_else(|err| panic!("{}", err));

    planted
}

/// The two markers the planted answerers say themselves by.
const ZBKTD_SAYS_DELIVERED: &str = "the delivered binary answered";
const ZBKTD_SAYS_OWN: &str = "the seat's own binary answered";

/// A LINKED WORKTREE WITH A CURRENT OWN BUILD RUNS ITS OWN, AND THE DOOR SPAWNS
/// THAT FILE. This is the arm that could never run at all before the second
/// residence: an own-build verdict was a refusal, so a linked worktree that had
/// already converged its own work was sent to converge it again on every launch.
///
/// WHICH FILE RAN IS A DOOR FACT AND NOT A LOGIC ONE. The library hurdles prove
/// which way the election went; only a spawned drive can show that the door then
/// executed the binary the election named, and both residences are the same
/// program under two paths, so the two say which they are themselves.
///
/// THE SEAT IS A REAL LINKED WORKTREE of the lure that plays the delivered root,
/// so what the door derives is what a work billet derives.
#[test]
fn bktd_a_dispatched_seat_spawns_its_own_current_build() {
    let delivered = bktu_Lure::bktu_compose("geography-own-runs");

    let behind = zbktd_geography_lure(&delivered);

    zbktd_answerer(&delivered, ZBKTD_SAYS_DELIVERED, &behind);

    let lure = delivered.bktu_billet("geography-own-runs");

    lure.bktu_write("Tools/lure/src/main.rs", "fn main() { /* the seat moves on */ }\n");
    lure.bktu_commit("the seat moves an elected root past the delivered binary");

    // THE SEAT'S OWN BUILD IS CURRENT: it states the position the seat now stands
    // at, which is what a converge here would have left behind. It is planted
    // AFTER the commit deliberately, so the tree stays clean for the door law and
    // the position it states is the one HEAD holds.
    let struck = zbktd_head(&lure);
    zbktd_answerer(&lure, ZBKTD_SAYS_OWN, &struck);
    lure.bktu_commit("the seat's own converge");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stdout);
    let aside = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a current own build runs rather than being sent to converge: {} {} {}",
        out.status,
        said,
        aside
    );
    assert!(
        said.contains(ZBKTD_SAYS_OWN),
        "the door spawned the seat's OWN residence:\n{}\n{}",
        said,
        aside
    );
    assert!(
        !said.contains(ZBKTD_SAYS_DELIVERED),
        "and never the delivered one, which is standing and outrun:\n{}",
        said
    );
    assert!(
        aside.contains("bknre_own"),
        "and said so on its election line: {}",
        aside
    );
}

/// The exit the derby takes when a precondition its declarations state does not
/// hold, spelled here so an assertion reads against a number rather than a
/// door's word.
const ZBKTD_EXIT_PREFLIGHT: i32 = 87;

/// THE DERBY REFUSES AN OUTRUN APP RESIDENCE AHEAD OF ITS WALK, and the refusal
/// says the residence STANDS. The library hurdles prove the preflight's reading;
/// only a spawned drive can show that the door holds the election behind the
/// sweep and ahead of every course, and exits on its own code rather than a
/// course's.
///
/// THE CONTROL IS THE SAME LURE ONE COMMIT EARLIER: the answerer states the
/// seat's position, the derby is green and the course leaves its mark. Then the
/// seat moves an elected root, and the mark's absence on the second drive is
/// what says no course ran — an assertion on the console alone could only show
/// the door did not MENTION one.
#[test]
fn bktd_the_derby_refuses_an_outrun_app_residence_ahead_of_its_walk() {
    let lure = bktu_Lure::bktu_compose("derby-outrun");
    let struck = zbktd_geography_lure(&lure);
    zbktd_answerer(&lure, ZBKTD_SAYS_OWN, &struck);

    zbktd_crate(&lure, "Tools/bravo", "bravo", "src/lib.rs", BKTD_MARKED);
    zbktd_suite_collar(&lure, "Tools/bravo", "suite-bravo");
    // THE MARK IS IGNORED, as a build directory is, so that the control's green
    // run leaves a tree the door law still admits for the drive that follows.
    lure.bktu_write(".gitignore", "target/\nTools/bravo/ran.txt\n");
    lure.bktu_commit("a current answerer beside one green course");

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "the control: a derby over a current residence is green: {} {}",
        out.status,
        said
    );
    assert!(
        mark.exists(),
        "and its course ran, leaving the mark at {}",
        mark.display()
    );
    std::fs::remove_file(&mark).expect("the mark clears between drives");

    lure.bktu_write("Tools/lure/src/main.rs", "fn main() { /* the seat moves on */ }\n");
    lure.bktu_commit("the seat moves an elected root past the standing binary");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_PREFLIGHT),
        "the derby refuses on the preflight's own exit: {} {}",
        out.status,
        said
    );
    assert!(
        said.contains("stands at") && said.contains("Tools/bin/lurex"),
        "the refusal says the residence stands, and where: {}",
        said
    );
    assert!(
        !said.contains("nothing stands at"),
        "and never reads it as the swept shape: {}",
        said
    );
    assert!(
        said.contains(bkco_output::BKCO_VERB_HEEL),
        "a collar declaring no strike door is handed the converge: {}",
        said
    );
    assert!(
        !mark.exists(),
        "no course ran: the mark stands unwritten at {}",
        mark.display()
    );
}

/// THE DISPATCHED ARM, DRIVEN. The door enters a linked worktree, derives its
/// main worktree as the delivered root, and the election reads the binary
/// standing there rather than anything in the tree the door entered.
///
/// THIS IS THE ONE READING THE LIBRARY HURDLES CANNOT TAKE. They derive a
/// geography and hand it in as a value; only a spawned drive can say that the
/// door face takes the reading at all — from inside the tree its child stands
/// in — and threads what it read as far as the proclamation.
#[test]
fn bktd_a_linked_worktree_derives_and_names_its_delivered_root() {
    let delivered = bktu_Lure::bktu_compose("geography-dispatched");

    let struck = zbktd_geography_lure(&delivered);

    // THE DELIVERED CANDIDATE IS CURRENT AND THE SEAT'S OWN IS ABSENT, which is
    // the shape a freshly added linked worktree stands in: nothing has been converged
    // in the work tree yet and the delivered root is what answers.
    let candidate = zbktd_answerer(&delivered, ZBKTD_SAYS_DELIVERED, &struck);
    let lure = delivered.bktu_billet("geography-dispatched");

    assert!(
        !lure.bktu_root().join("Tools/bin/lurex").exists(),
        "nothing stands at the seat's own residence, so a borrow cannot be the seat reading itself"
    );

    let out = lure.bktu_drive_door(zbktd_kennel(), &[ZBKTD_DOOR_TATTOO, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stdout);
    let aside = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "the door derives a linked worktree's geography and proclaims: {} {} {}",
        out.status,
        said,
        aside
    );
    assert!(
        said.contains(&format!(
            "BKRC_GEOGRAPHY=\"{} \\\n  {}\"",
            BKCE_GEOGRAPHY_DISPATCHED,
            zbktd_real(delivered.bktu_root()).display()
        )),
        "the proclamation carries the geography as a catena naming the main worktree:\n{}",
        said
    );
    assert!(
        said.contains("BKRC_ELECTION=\"bknre_borrow\""),
        "and the election borrowed the delivered binary at {}:\n{}",
        candidate.display(),
        said
    );
}

/// THE CONTROL: THE SAME REPOSITORY DRIVEN FROM ITS MAIN WORKTREE. A main
/// checkout is its own delivered root, so the simple case stays
/// configuration-free — and without this control the hurdle above could be
/// cleared by a door that proclaimed a dispatched geography whatever it read.
///
/// THE WORKTREE STANDS AND IS SIMPLY NOT ENTERED, which is what makes this a
/// control rather than a different lure: everything the arm above had stands
/// here, and what the two drives differ in is which tree the door entered.
#[test]
fn bktd_a_main_checkout_runs_self_geography() {
    let lure = bktu_Lure::bktu_compose("geography-self");

    let struck = zbktd_geography_lure(&lure);

    // THE ANSWERER STANDS IN THE WORKTREE AND NOT IN THE MAIN WORKTREE, since it
    // is untracked ground and the main worktree is the tree this door law reads.
    let billet = lure.bktu_billet("geography-self");
    zbktd_answerer(&billet, ZBKTD_SAYS_DELIVERED, &struck);

    let out = lure.bktu_drive_door(zbktd_kennel(), &[ZBKTD_DOOR_TATTOO, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stdout);

    assert!(
        out.status.success(),
        "a main worktree is no fault: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        said.contains(&format!("BKRC_GEOGRAPHY=\"{}\"", BKCE_GEOGRAPHY_SELF)),
        "the proclamation says the seat works the tree it entered:\n{}",
        said
    );
    assert!(
        !said.contains(&billet.bktu_root().display().to_string()),
        "and names no other tree, a main worktree borrowing from none:\n{}",
        said
    );
    assert!(
        said.contains("BKRC_ELECTION=\"bknre_own\""),
        "the election reads the seat's own residence, where nothing stands, and elects its own \
         build:\n{}",
        said
    );
}

/// A GEOGRAPHY THAT DOES NOT DERIVE REFUSES AT ITS OWN EXIT, and never falls
/// back to self-geography.
///
/// THE FALLBACK IS WHAT THIS FORBIDS. A door that read past a worktree naming no
/// main worktree would turn it into a quiet own build: the seat would look
/// ordinary, every election would refuse or rebuild, and no delivered root would
/// ever be consulted by anybody. A worktree added from a bare repository is the
/// shape posed, being the one such answer git gives while the tree itself still
/// reads clean to the door law standing ahead of the geography.
#[test]
fn bktd_a_geography_that_does_not_derive_refuses_at_its_own_exit() {
    let delivered = bktu_Lure::bktu_compose("geography-underived");

    zbktd_geography_lure(&delivered);

    let lure = delivered.bktu_billet_of_bare("geography-underived");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[ZBKTD_DOOR_TATTOO, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_GEOGRAPHY),
        "a geography that does not derive takes its own exit: {}",
        said
    );
    assert!(
        said.contains("lure.git"),
        "the refusal names the common directory git answered: {}",
        said
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).is_empty(),
        "and nothing was proclaimed: a door that refused its geography answered no question"
    );

    // THE CONTROL IS THE SAME CONTENT IN A WORKTREE OF THE MAIN WORKTREE, so the
    // exit above is the bare repository and not the worktree.
    let control = delivered.bktu_billet("geography-underived-control");

    let out = control.bktu_drive_door(zbktd_kennel(), &[ZBKTD_DOOR_TATTOO, "app-lure"]);

    assert!(
        out.status.success(),
        "the control: a worktree added from a main worktree derives: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// THE BARE SELF-REPORT STANDS AHEAD OF THE GEOGRAPHY AND IS UNMOVED BY IT. The
/// whistle and the driver gates read a bare drive as a POSITION and treat any
/// non-zero exit as no position at all (`Tools/bkk/bk0/bkcp_position.sh`), so a
/// geography refusal reaching that report would counterfeit staleness — a
/// geography fault presenting as a stale binary, which is a different repair
/// aimed at a different tree.
#[test]
fn bktd_the_bare_self_report_never_reads_a_geography() {
    let delivered = bktu_Lure::bktu_compose("geography-bare-report");

    zbktd_geography_lure(&delivered);

    // THE WORST GEOGRAPHY THERE IS: it refuses at every door that derives one. If
    // the bare report derived it, this drive would exit 82.
    let lure = delivered.bktu_billet_of_bare("geography-bare-report");

    let out = lure.bktu_drive(zbktd_kennel());

    assert!(
        out.status.success(),
        "the bare self-report answers whatever the geography says: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    // THE CONTROL IS A DOOR THAT DOES DERIVE ONE, over the identical tree: without
    // it, a bare report that had simply stopped failing for some other reason
    // would clear the assertion above.
    let refused = lure.bktu_drive_door(zbktd_kennel(), &[ZBKTD_DOOR_TATTOO, "app-lure"]);

    assert_eq!(
        refused.status.code(),
        Some(ZBKTD_EXIT_GEOGRAPHY),
        "the control: a door that consumes a delivered root refuses on the same tree the bare \
         report walked past: {}",
        String::from_utf8_lossy(&refused.stderr)
    );
}

/// The exit a door takes when the election rules that the blessed binary may not
/// answer, spelled here so an assertion reads against a number.
const ZBKTD_EXIT_OUTRUN: i32 = 73;

/// THE DOOR-SIDE REFUSAL OF AN OUTRUN LINKED WORKTREE: it names BOTH residences
/// and the converge, and takes the outrun exit.
///
/// THE LIBRARY HURDLES PROVE WHICH WAY THE ELECTION WENT; THIS PROVES WHAT A
/// PERSON READS. A reader refused by two binaries and told about one of them
/// goes and converges the wrong tree, and under dispatch the wrong tree is the
/// delivered root — which is the one place heel is forbidden to write. No
/// in-process call can show that the sentence printed or that the exit was the
/// one the contract names.
#[test]
fn bktd_a_dispatched_seat_refused_by_both_names_both_and_the_converge() {
    let delivered = bktu_Lure::bktu_compose("geography-refused");

    let behind = zbktd_geography_lure(&delivered);

    // THE DELIVERED CANDIDATE IS REAL AND BEHIND, and the seat's own residence
    // holds nothing at all: the shape a linked worktree stands in after its own
    // work has moved and before it has converged.
    zbktd_answerer(&delivered, ZBKTD_SAYS_DELIVERED, &behind);

    let lure = delivered.bktu_billet("geography-refused");
    lure.bktu_write("Tools/lure/src/main.rs", "fn main() { /* the seat moves on */ }\n");
    lure.bktu_commit("the seat moves an elected root past the delivered binary");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_OUTRUN),
        "the door takes the outrun exit rather than converging on the caller's behalf: {}",
        said
    );
    assert!(
        said.contains(&zbktd_real(delivered.bktu_root()).join("Tools/bin/lurex").display().to_string()),
        "the refusal names the delivered binary it could not borrow: {}",
        said
    );
    assert!(
        said.contains(&zbktd_real(lure.bktu_root()).join("Tools/bin/lurex").display().to_string()),
        "and the seat's own residence, which is the tree a converge writes: {}",
        said
    );
    assert!(
        said.contains(BKTD_DOOR_HEEL),
        "and the converge, run deliberately rather than by the door: {}",
        said
    );
}

/// HEEL REFUSES A TARGET INSIDE THE DELIVERED ROOT, AT THE DOOR. A billet's build
/// never lands on a delivered tree: the delivered root is the borrow candidate
/// EVERY linked worktree reads, so a binary written there is written into all of
/// their elections at once.
///
/// THE MISPLACEMENT IS A REAL ONE. Git permits a worktree added inside its own
/// main worktree, and that is the one geography in which a seat's own residence
/// stands inside the delivered root — so the worktree is stood exactly there.
#[test]
fn bktd_heel_refuses_a_residence_inside_the_delivered_root() {
    let delivered = bktu_Lure::bktu_compose("geography-heel-refuses");

    zbktd_geography_lure(&delivered);

    let lure = delivered.bktu_billet_at(
        "geography-heel-refuses",
        &delivered.bktu_root().join("misplaced-billet"),
    );

    let residence = zbktd_real(lure.bktu_root()).join("Tools/bin/lurex");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);
    let said = String::from_utf8_lossy(&out.stderr);

    assert!(
        !out.status.success(),
        "a converge onto a delivered tree is refused: {}",
        said
    );
    assert!(
        said.contains(&residence.display().to_string()),
        "the refusal names the target it would have written: {}",
        said
    );
    assert!(
        said.contains(&zbktd_real(delivered.bktu_root()).display().to_string()),
        "and the delivered root that contains it: {}",
        said
    );
    assert!(
        !residence.exists(),
        "and NOTHING WAS WRITTEN — the refusal stands ahead of the build, so a door that refused \
         after converging would leave this file standing: {}",
        residence.display()
    );

    // THE CONTROL IS A WORKTREE OF THE SAME CHECKOUT STANDING BESIDE IT, which
    // must converge and install: without it, a heel that had simply stopped
    // working would clear every assertion above.
    let beside = delivered.bktu_billet("geography-heel-beside");

    let out = beside.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);

    assert!(
        out.status.success(),
        "the control: a worktree standing apart from its main worktree converges: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        beside.bktu_root().join("Tools/bin/lurex").is_file(),
        "and the binary stands at the worktree's own residence, where the refusal above left \
         nothing"
    );
}

// eof

/// A crate whose lib declares a module no file backs — the shape both real
/// exergue-bearing crates carry, reduced to the one line that matters. Cargo
/// never writes this file; a bash door does, and a seat that has not run it
/// holds nothing here.
const BKTD_ABSENT_MODULE: &str = "pub mod lureg_exergue;\n";

/// The door the composed collars below name as the one that strikes their
/// exergue. It does not have to exist for these hurdles: what is under test is
/// that the refusal NAMES it, which is the whole defect — the compiler's own
/// answer names the file and never the door.
const BKTD_STRIKE_DOOR: &str = "tt/lure-b.Strike.sh";

/// The exergue every composed collar below declares, and which no hurdle writes
/// unless it means to.
const BKTD_EXERGUE: &str = "Tools/lure/src/lureg_exergue.rs";

/// Lay a crate declaring an exergue module, and a collar of the given kind
/// declaring that exergue and the door that strikes it.
///
/// THE FILE IS DELIBERATELY NOT WRITTEN. That is the cold seat, reproduced: the
/// module line stands, the file does not, and every road below meets it.
fn zbktd_exergue_crate(lure: &bktu_Lure, collar: &str, kind_lines: &str) {
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);
    lure.bktu_write(".gitignore", "target/\nTools/bin/\nTools/lure/src/lureg_exergue.rs\n");
    lure.bktu_write("Tools/lure/Cargo.toml", &zbktd_manifest("lure"));
    lure.bktu_write("Tools/lure/Cargo.lock", &zbktd_lock("lure"));
    lure.bktu_write("Tools/lure/src/lib.rs", BKTD_ABSENT_MODULE);
    lure.bktu_write("Tools/lure/src/main.rs", "fn main() {}\n");
    lure.bktu_write(
        &lure.bktu_weaned_at(&format!("{}/bkrr.env", collar)),
        &format!(
            "\
BKRR_COLLAR=\"{}\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_ROOTS=\"Tools/lure/src Tools/lure/Cargo.toml Tools/lure/Cargo.lock\"
BKRR_FEATURES=\"\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"{}\"
BKRR_STRIKE=\"{}\"
{}",
            collar, BKTD_EXERGUE, BKTD_STRIKE_DOOR, kind_lines
        ),
    );
}

/// The app half of the roster above.
const BKTD_KIND_APP: &str = "\
BKRR_KIND=\"bknre_app\"
BKRR_TARGET=\"lure\"
BKRR_PROFILE=\"release\"
BKRR_RESIDENCE=\"Tools/bin\"
BKRR_BYNAME=\"lurex\"
";

/// The suite half.
const BKTD_KIND_SUITE: &str = "\
BKRR_KIND=\"bknre_suite\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_PROFILE=\"test\"
BKRR_RUNNER=\"bknre_cargo\"
BKRR_TONGUE=\"bknre_harness\"
";

/// What a refusal about an absent exergue must carry, whichever road met it.
///
/// THE NEGATIVE ASSERTION IS THE POINT OF THE PACE. Naming the door is only half
/// the repair; the other half is that rustc's own answer never reaches the
/// operator, because a transcript carrying BOTH would mean the road refused
/// AFTER spending a compile, which is the behavior this work replaces.
fn zbktd_names_the_door(said: &str, road: &str) {
    assert!(
        said.contains(BKTD_STRIKE_DOOR),
        "the {} refusal names the door that strikes the exergue: {}",
        road,
        said
    );
    assert!(
        said.contains(BKTD_EXERGUE),
        "the {} refusal names the file that does not stand: {}",
        road,
        said
    );
    assert!(
        !said.contains("E0583"),
        "the {} road refused AHEAD of cargo, so the compiler's own answer never arrives: {}",
        road,
        said
    );
}

/// HEEL REFUSES AHEAD OF CARGO over a collar whose exergue does not stand, and
/// names the door that writes it.
#[test]
fn bktd_heel_refuses_an_absent_exergue_and_names_the_door() {
    let lure = bktu_Lure::bktu_compose("exergue-heel");
    zbktd_exergue_crate(&lure, "app-lure", BKTD_KIND_APP);
    lure.bktu_commit("a crate whose exergue no door has struck");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(!out.status.success(), "heel refuses: {}", out.status);
    zbktd_names_the_door(&said, "heel");
}

/// THE MUZZLE REFUSES ON THE SAME READING, which is the road the defect was
/// actually found on: a sweep reached a crate whose exergue was unstruck and the
/// lint died on E0583, reported as a build failure naming no door.
#[test]
fn bktd_the_muzzle_refuses_an_absent_exergue_and_names_the_door() {
    let lure = bktu_Lure::bktu_compose("exergue-muzzle");
    zbktd_exergue_crate(&lure, "app-lure", BKTD_KIND_APP);
    lure.bktu_commit("a crate whose exergue no door has struck");

    let out = lure.bktu_drive_door(zbktd_kennel(), &["muzzle"]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(!out.status.success(), "the muzzle refuses: {}", out.status);
    zbktd_names_the_door(&said, "muzzle");
}

/// THE LAUNCH SEAM REFUSES TOO, so all three compile roads answer alike. A suite
/// whose crate cannot be compiled has no launch to compose, and the operator is
/// owed the same door here as at the other two.
#[test]
fn bktd_the_launch_seam_refuses_an_absent_exergue_and_names_the_door() {
    let lure = bktu_Lure::bktu_compose("exergue-mush");
    zbktd_exergue_crate(&lure, "suite-lure", BKTD_KIND_SUITE);
    lure.bktu_commit("a suite whose exergue no door has struck");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, "suite-lure"]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(!out.status.success(), "the launch seam refuses: {}", out.status);
    zbktd_names_the_door(&said, "launch seam");
}

/// THE SAME THREE ROADS PROCEED WHERE THE COLLAR DECLARES NONE. Without this the
/// three hurdles above would be satisfied by a reading that refused every
/// collar in the estate, which is the failure mode a presence check invites.
#[test]
fn bktd_a_collar_declaring_no_exergue_reaches_cargo() {
    // THE LURE'S NAME CARRIES NO WORD THIS HURDLE ASSERTS ON. Its directory
    // stands in every path the door prints, so a name holding the word under
    // test would satisfy a bare-substring assertion out of the transcript's
    // scenery rather than out of anything the door said.
    let lure = bktu_Lure::bktu_compose("plain-crate");
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);
    zbktd_app(&lure);
    lure.bktu_commit("a collar declaring no generated source at all");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(
        out.status.success(),
        "a collar owing no exergue converges: {} {}",
        out.status,
        said
    );

    // THE TWO SENTENCES THE READING CAN PRODUCE, asserted by their own words
    // rather than by the bare noun: the refusal, and the verdict line naming a
    // striking door. Neither belongs over a collar that declares none.
    assert!(
        !said.contains("that do not stand"),
        "the road never refuses a collar that declares none: {}",
        said
    );
    assert!(
        !said.contains("is struck by") && !said.contains("are struck by"),
        "and its verdict names no striking door: {}",
        said
    );
}

/// THE STALENESS ARM, AS A VERDICT-LINE ASSERTION. The kennel judges staleness
/// nowhere — that fence stands in the declaring crate and already refuses
/// correctly. What the kennel owes is legibility: heel names the striking door
/// beside its geography, so a tenant refusal arriving afterward reads as
/// expected rather than as a fresh mystery. This is the assertion that keeps
/// that line from being dropped as decoration.
#[test]
fn bktd_heel_names_the_striking_door_on_a_standing_exergue() {
    let lure = bktu_Lure::bktu_compose("exergue-verdict");
    zbktd_exergue_crate(&lure, "app-lure", BKTD_KIND_APP);
    // THE EXERGUE STANDS THIS TIME, so the presence reading is silent and the
    // converge runs — which is the only state in which the verdict line is
    // reached at all.
    lure.bktu_write("Tools/lure/src/lureg_exergue.rs", "// struck by the hurdle\n");
    lure.bktu_commit("a crate whose exergue stands");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_HEEL, "app-lure"]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(
        said.contains(BKTD_STRIKE_DOOR),
        "heel's verdict names the door that strikes the exergue it built over: {}",
        said
    );
}

/// The mark the quiet flavor writes when the CONSOLE has been silent for a
/// cadence. The voice publishes no constant for it, so this is a second home and
/// is named as one: what a hurdle below asks is whether a line that is not the
/// verdict is a HEARTBEAT rather than something about a case, and that question
/// cannot be put without some spelling of the mark.
const ZBKTD_HEARTBEAT: &str = "·";

/// The verdict's own lead for a green run, spelled here for the same reason.
const ZBKTD_GREEN_VERDICT: &str = "green: ";

/// Compose a seat carrying a green course under each admitted runner, with the
/// nextest residence placed so the nextest collar reaches one.
fn zbktd_voice_seat(name: &str) -> bktu_Lure {
    let lure = bktu_Lure::bktu_compose(name);
    lure.bktu_collar_seat();
    lure.bktu_stationed().bktu_nextest_seat(&lure);
    lure
}

/// A GREEN COURSE PRINTS A HEARTBEAT AND A VERDICT AND NOTHING ELSE, under each
/// admitted runner — the bounded-output cinch's green half.
///
/// TWO LURES, ONE PER RUNNER, because the recognizers are two membranes over two
/// foreign shapes and a hurdle driving one proves nothing about the other. The
/// two collars differ in their runner and in nothing else, which is what makes
/// the pair a reading about runners rather than about seats.
///
/// STDOUT ALONE IS THE CONSOLE. The voice writes there and every diagnostic goes
/// to stderr, so a hurdle folding the two together would be asserting against
/// the door's diagnostics as though they were what the operator was promised.
///
/// THE LOAD-BEARING ASSERTION IS THE ABSENCE, and it needs the dial hurdle below
/// as its control: a kennel that had stopped rendering cases ALTOGETHER would
/// clear this one, an absence being exactly what it looks for.
#[test]
fn bktd_a_green_course_speaks_a_verdict_and_no_case() {
    for collar in [
        bktu_Lure::BKTU_COLLAR_NEXTEST,
        bktu_Lure::BKTU_COLLAR_CARGO,
    ] {
        let lure = zbktd_voice_seat(&format!("voice-quiet-{}", collar));

        let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR, collar]);

        let console = String::from_utf8_lossy(&out.stdout).into_owned();

        assert!(
            out.status.success(),
            "the green course runs under {}: {} {}",
            collar,
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );

        let spoken: Vec<&str> = console
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();

        let verdicts: Vec<&&str> = spoken
            .iter()
            .filter(|line| line.starts_with(ZBKTD_GREEN_VERDICT))
            .collect();

        assert_eq!(
            verdicts.len(),
            1,
            "one verdict and one only, under {}: {:?}",
            collar,
            spoken
        );

        // THE COUNTS ARE THE KENNEL'S OWN, so the verdict says what this door
        // saw rather than what the runner summarized — and the seat carries
        // exactly one case for it to have seen.
        assert!(
            verdicts[0].contains("1 ran") && verdicts[0].contains("1 passed"),
            "the verdict counts the seat's one case, under {}: {}",
            collar,
            verdicts[0]
        );

        // Everything else on the console is a heartbeat. A fast station will
        // print none at all; a slow one prints them on the cadence, and either
        // way nothing about a case may stand here.
        for line in &spoken {
            if line.starts_with(ZBKTD_GREEN_VERDICT) {
                continue;
            }
            assert!(
                line.contains(ZBKTD_HEARTBEAT),
                "the quiet console carries the verdict and heartbeats alone, under {}: {:?}",
                collar,
                spoken
            );
        }

        assert!(
            !console.contains(bktu_Lure::BKTU_CASE),
            "the quiet flavor names no passing case, under {}: {}",
            collar,
            console
        );
    }
}

/// THE DIAL RENDERS A LINE PER CASE, AND IT IS THE ABSENCE ABOVE MADE FALSIFIABLE.
///
/// Driven in the criterion's own order — the dial set, then unset over the same
/// seat — so that the second drive is a control struck in the same call rather
/// than a reading taken somewhere else. A kennel rendering nothing at all would
/// pass the quiet hurdle and fails here.
///
/// THE DIAL IS THE SUBSTRATE'S AND THE KENNEL DECLARES NO FLAG. The name is read
/// from the kennel's own constant, so a door that grew a verbosity option would
/// not quietly satisfy this by another road.
///
/// TWO IDENTICALLY COMPOSED SEATS RATHER THAN TWO DRIVES OVER ONE, and the
/// reason is the record door's cardinality rather than anything about the dial:
/// one write per collar per dispatch is the whole cardinality of a fact file, so
/// a second drive of the same collar over one seat refuses as two courses
/// claiming it. The composition is the same call, so the dial remains the only
/// difference between the two readings.
#[test]
fn bktd_the_dial_renders_a_line_per_case_and_unset_renders_none() {
    let lure = zbktd_voice_seat("voice-dial-loud");

    let loud = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR, bktu_Lure::BKTU_COLLAR_CARGO],
        &[(bkco_output::BKCO_VERBOSE_VAR, OsStr::new("1"))],
    );

    let said = String::from_utf8_lossy(&loud.stdout).into_owned();

    assert!(
        loud.status.success(),
        "the dialed run is still green: {} {}",
        loud.status,
        String::from_utf8_lossy(&loud.stderr)
    );
    assert!(
        said.contains(bktu_Lure::BKTU_CASE),
        "the dial renders the seat's case: {}",
        said
    );

    let hush = zbktd_voice_seat("voice-dial-quiet");

    let quiet = hush.bktu_drive_door(
        zbktd_kennel(),
        &[BKTD_DOOR, bktu_Lure::BKTU_COLLAR_CARGO],
    );

    let hushed = String::from_utf8_lossy(&quiet.stdout).into_owned();

    assert!(
        quiet.status.success(),
        "the undialed run is green too: {} {}",
        quiet.status,
        String::from_utf8_lossy(&quiet.stderr)
    );
    assert!(
        !hushed.contains(bktu_Lure::BKTU_CASE),
        "the dial unset renders no case: {}",
        hushed
    );
}

/// A case the seat's crate can be made to fail, replacing the green one the
/// composing surface lays down. It asserts a falsehood on purpose, exactly as
/// the surface's own case asserts a tautology on purpose.
const ZBKTD_FAILING_CASE: &str = "lure_the_planted_case_fails";

/// The three members a logging dispatch composes, stood up under the lure's own
/// temp root so a hurdle can read back what the kennel wrote.
///
/// THE NAMES ARE THE KENNEL'S OWN CONSTANTS rather than the substrate's spelled
/// again: what is under test is the reader that answers to them, and a hurdle
/// spelling the variables a second time would keep passing after the kennel had
/// stopped reading the ones the dispatch exports.
fn zbktd_log_family(lure: &bktu_Lure) -> Vec<(&'static str, std::path::PathBuf)> {
    let seat = lure.bktu_temp().join("logs");
    std::fs::create_dir_all(&seat).expect("the hurdle composes the log seat");

    vec![
        (bkcv_voice::BKCV_LOG_LAST_VAR, seat.join("last.txt")),
        (bkcv_voice::BKCV_LOG_SAME_VAR, seat.join("same.txt")),
        (bkcv_voice::BKCV_LOG_HIST_VAR, seat.join("hist.txt")),
    ]
}

/// A FAILING CASE SPEAKS ONE BOUNDED LINE WITH THE DEPTH AT A PATH, under each
/// admitted runner — the bounded-output cinch's red half.
///
/// ONE LINE PER FAILING CASE IS THE WHOLE CLAIM, and it is why the count is
/// asserted rather than mere presence: an agent reading test output pays for
/// every line out of its working context, so a red console that had begun
/// echoing the child's own stream would still name the case and would still
/// carry the path, and only the count tells the two apart.
///
/// THE PATH IS THE HISTORICAL MEMBER'S, so the reader is sent to the depth
/// rather than handed the bounded line and left there. A drive outside a logging
/// dispatch has no path to send anyone to, which is why this hurdle composes the
/// family rather than driving bare.
///
/// THE GREEN HURDLE ABOVE IS THIS ONE'S CONTROL, in both directions: it proves
/// the console says nothing about a passing case, and this proves the silence is
/// not the console having stopped speaking.
#[test]
fn bktd_a_failing_case_speaks_one_bounded_line_with_the_depth_at_a_path() {
    for collar in [
        bktu_Lure::BKTU_COLLAR_NEXTEST,
        bktu_Lure::BKTU_COLLAR_CARGO,
    ] {
        let lure = zbktd_voice_seat(&format!("voice-red-{}", collar));

        // The seat's own case is replaced rather than joined, so the run carries
        // exactly one failing case and the count below is a reading about the
        // console rather than about how many cases were planted.
        lure.bktu_write(
            "Tools/lure/src/lib.rs",
            &format!(
                "// the lure's own crate, planted red\n\
                 #[test]\n\
                 fn {}() {{\n\
                 \x20   assert!(false, \"the plant fails on purpose\");\n\
                 }}\n",
                ZBKTD_FAILING_CASE
            ),
        );
        lure.bktu_commit("plant one failing case");

        let family = zbktd_log_family(&lure);
        let stated: Vec<(&str, &OsStr)> = family
            .iter()
            .map(|(name, path)| (*name, path.as_os_str()))
            .collect();

        let out = lure.bktu_drive_stating(zbktd_kennel(), &[BKTD_DOOR, collar], &stated);

        let console = String::from_utf8_lossy(&out.stdout).into_owned();

        assert!(
            !out.status.success(),
            "a course carrying a failing case is red under {}: {}",
            collar,
            console
        );

        let failures: Vec<&str> = console
            .lines()
            .filter(|line| line.contains(ZBKTD_FAILING_CASE))
            .collect();

        assert_eq!(
            failures.len(),
            1,
            "one bounded line for the one failing case, under {}: {}",
            collar,
            console
        );

        let hist = family
            .iter()
            .find(|(name, _)| *name == bkcv_voice::BKCV_LOG_HIST_VAR)
            .map(|(_, path)| path.to_string_lossy().into_owned())
            .expect("the family carries a historical member");

        assert!(
            failures[0].contains(&hist),
            "the failure line carries the depth's path, under {}: {}",
            collar,
            failures[0]
        );

        // THE DEPTH IS REALLY THERE. A path named in a bounded line that led
        // nowhere would satisfy every assertion above and would strand the
        // reader it was written for.
        let depth = std::fs::read_to_string(&hist).expect("the historical member stands");
        assert!(
            depth.contains(ZBKTD_FAILING_CASE),
            "the depth carries the child's own account of the case, under {}: {}",
            collar,
            hist
        );
    }
}

/// The door word that publishes a help text beside `mush`. Both are driven below
/// because both are doors a person types, and the rule under test is stated of
/// the shape rather than of one door.
const ZBKTD_DOOR_LINEUP: &str = "lineup";

/// The lead every option in every shell dialect the estate uses would wear. The
/// assertion below is that NO token wearing it stands in a help text, which is
/// the mechanical form of two separate cinches at once.
const ZBKTD_OPTION_LEAD: &str = "--";

/// NO DOOR'S HELP OFFERS A FLAG, which is two cinches proven by one reading.
///
/// The bounded-output cinch forbids a verbosity option — the flavor is the
/// substrate's dial and a door-level flag would be the interface contamination
/// the command-surface freeze bars — and the binary election's cinch forbids an
/// option naming which binary answers, the election being derived from facts
/// alone. Both are the same observable: a help text carrying no option at all.
///
/// THE READING IS MECHANICAL RATHER THAN A SEARCH FOR TWO WORDS. A hurdle
/// grepping the help for "verbose" would pass over a flag spelled any other way,
/// and the cinch is not about a spelling — it is that the shape admits the verb,
/// the collar, and what the collar's kind owns, and nothing else. So what is
/// asserted is the absence of an option LEAD, which no future spelling escapes.
///
/// THE HELP IS REACHED BY DRIVING THE DOOR BARE, which is how a person meets it:
/// each of these doors answers an argumentless invocation with its own help and
/// a fatal exit. Reaching instead for the constants would prove nothing about
/// what a caller is shown, and those constants are private to the binary.
///
/// THE CONTROL IS THE HELP ITSELF. Each door must actually have spoken — a door
/// that printed nothing at all would satisfy an absence assertion trivially — so
/// the door word is required to stand in what it said.
#[test]
fn bktd_no_doors_help_offers_an_option() {
    let lure = zbktd_voice_seat("voice-help");

    for door in [BKTD_DOOR, ZBKTD_DOOR_LINEUP] {
        let out = lure.bktu_drive_door(zbktd_kennel(), &[door]);

        let said = String::from_utf8_lossy(&out.stderr).into_owned();

        assert!(
            !out.status.success(),
            "a door driven with nothing to act on refuses: {} {}",
            door,
            said
        );

        // THE POSITIVE HALF, struck in the same call: the door spoke its own
        // help, so the absence below is a reading of a help text rather than of
        // an empty stream.
        assert!(
            said.contains(door),
            "the door names itself in its help: {} {}",
            door,
            said
        );

        assert!(
            !said.contains(ZBKTD_OPTION_LEAD),
            "no option rides {}'s shape — neither a verbosity flag nor one naming \
             which binary answers: {}",
            door,
            said
        );
    }
}

/// The substrate's own wire format for the digest line, spelled here rather than
/// cited from the kennel's constant.
///
/// THAT IS DELIBERATE AND IT INVERTS THIS FILE'S USUAL RULE. Everywhere else a
/// hurdle reads the kennel's own constant so the two cannot drift; here the lead
/// is the SUBSTRATE'S format (`BUr_yht`) that the kennel must match byte for
/// byte, so a hurdle citing the kennel's spelling would FOLLOW a drift instead of
/// catching it. What is under test is agreement with a foreign authority.
const ZBKTD_CHECKSUM_LEAD: &str = "Same log checksum: ";

/// The git context this hurdle hands the dispatch, so the second-line assertion
/// reads back what was handed over rather than the door's own fallback for a
/// station where git could not answer.
const ZBKTD_GIT_CONTEXT: &str = "bktd-planted-context";

/// The sha256 of one file, taken the way the heel's hurdles take it — through
/// the substrate's own curator rather than through the kennel.
///
/// A HASH CHECKED AGAINST ITSELF IS CHECKED AGAINST NOTHING. The crate's digest
/// is proven correct against the standard's published vectors in the library
/// hurdles; what is proven HERE is that the close digested the right bytes at
/// the right moment, and an independent implementation is what makes that a
/// reading rather than a tautology. `openssl` is what the substrate's own
/// curator spawns and what this suite already spawns for the kibble seal, so
/// nothing is asked of the station that it was not asked for already.
fn zbktd_digest(path: &std::path::Path) -> String {
    let opened = std::fs::File::open(path)
        .unwrap_or_else(|err| panic!("could not read {}: {}", path.display(), err));

    let out = std::process::Command::new("openssl")
        .arg("dgst")
        .arg("-sha256")
        .stdin(std::process::Stdio::from(opened))
        .output()
        .unwrap_or_else(|err| panic!("could not run openssl: {}", err));

    String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .next_back()
        .unwrap_or_else(|| panic!("openssl said nothing a digest can be read out of"))
        .to_string()
}

/// THE RECORD STANDS IN THREE MEMBERS AND THE CLOSE DIGESTS THE RIGHT BYTES.
///
/// THE DIGEST IS OVER THE NORMALIZED MEMBER AND THE LINE LANDS IN THE HISTORICAL
/// ONE, which is the fact this hurdle exists to pin. The two are different files
/// and the close reads one to write into the other; a hurdle that digested the
/// member the line STANDS IN could never pass, and the English of the rule reads
/// that way at first glance. The law is the substrate's (`BUr_yht`): the
/// digest is of the normalized member as its bytes finally stand.
///
/// WHAT A STRUCTURE-ONLY READING WOULD MISS, and why the equality is asserted
/// rather than the shape: hashing the wrong member of the three, reading before
/// the final flush, losing a trailing frame, or stamping the digest line the way
/// the stream above it is stamped. Every one of those yields a well-formed
/// sixty-four-character last line, and only an independent digest of the
/// normalized member as it finally stands parts them from a sound close.
///
/// THE ALGORITHM IS NOT REPROVEN HERE. That the crate's sha256 is sha256 is the
/// library hurdles' reading against the standard's own vectors; this asks only
/// what no in-process call can see — that the close ran over the right bytes in
/// the right order, through a dispatch that really composed the family.
#[test]
fn bktd_the_record_stands_in_three_members_and_digests_the_normalized_one() {
    let lure = zbktd_voice_seat("voice-record");

    let family = zbktd_log_family(&lure);
    let mut stated: Vec<(&str, &OsStr)> = family
        .iter()
        .map(|(name, path)| (*name, path.as_os_str()))
        .collect();
    stated.push((
        bkcv_voice::BKCV_GIT_CONTEXT_VAR,
        OsStr::new(ZBKTD_GIT_CONTEXT),
    ));

    let out = lure.bktu_drive_stating(
        zbktd_kennel(),
        &[BKTD_DOOR, bktu_Lure::BKTU_COLLAR_CARGO],
        &stated,
    );

    assert!(
        out.status.success(),
        "the logged course runs green: {} {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );

    let seat = |var: &str| {
        family
            .iter()
            .find(|(name, _)| *name == var)
            .map(|(_, path)| path.clone())
            .unwrap_or_else(|| panic!("the family carries {}", var))
    };

    let same = seat(bkcv_voice::BKCV_LOG_SAME_VAR);
    let hist = seat(bkcv_voice::BKCV_LOG_HIST_VAR);

    for (var, path) in &family {
        assert!(
            path.is_file(),
            "the dispatch's {} stands as a file: {}",
            var,
            path.display()
        );
    }

    let written = std::fs::read_to_string(&hist).expect("the historical member is readable");
    let lines: Vec<&str> = written.lines().collect();

    assert!(
        lines.len() >= 3,
        "the historical member carries an invocation, a context and a digest: {:?}",
        lines
    );

    // THE CHILD'S STREAM STANDS IN THE RECORD whatever the console was shown —
    // the dial moves the console and never the record, and this drive was quiet.
    assert!(
        written.contains(bktu_Lure::BKTU_CASE),
        "the child's own account stands in the record the quiet console withheld: {}",
        written
    );

    assert!(
        lines[1].contains(ZBKTD_GIT_CONTEXT),
        "the second line is the context the dispatch handed over: {:?}",
        lines
    );

    let last = lines
        .last()
        .unwrap_or_else(|| panic!("the historical member ends on a line"));

    let carried = last
        .strip_prefix(ZBKTD_CHECKSUM_LEAD)
        .unwrap_or_else(|| panic!("the last line wears the substrate's lead: {}", last));

    // EXACTLY, AND WITHOUT FOLDING CASE. Both sides render lowercase hex, so a
    // comparison that folded would be quietly admitting a rendering this format
    // does not have.
    assert_eq!(
        carried,
        zbktd_digest(&same),
        "the close digests the normalized member as its bytes finally stand"
    );
}

/// The lock-regeneration door, as a caller types it.
const BKTD_DOOR_GANGLINE: &str = "gangline";

/// The upstream collar of the arrears lure — the crate a dependency is admitted
/// to, and the one the door is driven over.
const BKTD_COLLAR_HITCH: &str = "suite-hitch";

/// The downstream collar — the crate whose lock the admission leaves behind, and
/// which the door must NAME without writing.
const BKTD_COLLAR_LURE: &str = "suite-lure";

/// The downstream manifest, which the red verdict names.
const BKTD_LURE_MANIFEST: &str = "Tools/lure/Cargo.toml";

/// Lay a collar over a crate, electing every local package its closure reaches.
///
/// THE ELECTIONS ARE WRITTEN FOR THE TREE AS IT WILL STAND, before the manifest
/// names the dependency — the same posture the gangline's own unit lure takes
/// and for the same reason. The collar declares what the crate MAY link and what
/// its position is measured over, and both answers are the same before and after
/// the admission; what the admission changes is the manifest, which is the whole
/// point of the hurdle. A collar edited in the same breath would make the hurdle
/// prove two things at once and be able to say which one failed for neither.
fn zbktd_arrears_collar(lure: &bktu_Lure, name: &str, dir: &str, roots: &str, crates: &str) {
    lure.bktu_write(
        &lure.bktu_weaned_at(&format!("{}/bkrr.env", name)),
        &format!(
            "\
BKRR_COLLAR=\"{name}\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"{dir}/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"{roots}\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"{crates}\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"bknre_cargo\"
BKRR_TONGUE=\"bknre_harness\"
"
        ),
    );
}

/// Drive the gangline over one collar of the lure.
fn zbktd_gangline(lure: &bktu_Lure, collar: &str) -> std::process::Output {
    lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_GANGLINE, collar])
}

/// Everything a drive said, both streams, as the operator meets it.
fn zbktd_said(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// Stand up three crates — `lure` linking `hitch` by path, and `spur` standing
/// apart — with a collar over each of the first two and both locks answering.
///
/// THE LOCKS ARE WRITTEN BY THE DOOR AND NEVER BY HAND, which is forced rather
/// than chosen: a lock for a crate with a path dependency names that dependency's
/// whole closure, and a hand-written one would be this hurdle's own guess at what
/// cargo derives — a guess that, if wrong, reddens the hurdle in exactly the way
/// a real defect would. The dependency-free locks elsewhere in this file are
/// hand-written safely because they have no closure to guess at.
///
/// EVERY PATH IS POSED AS A VALUE AND THE SEAT IS THIS LURE'S OWN, because the
/// harness runs hurdles in parallel and a second hurdle composing its own lure
/// must not meet this one's tree.
fn zbktd_arrears_lure(name: &str) -> bktu_Lure {
    let lure = bktu_Lure::bktu_compose(name);

    lure.bktu_write("clippy.toml", BKTD_CLIPPY);

    lure.bktu_write(
        "Tools/lure/Cargo.toml",
        "[package]\nname = \"lure\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\
         [dependencies]\nhitch = { path = \"../hitch\" }\n",
    );
    lure.bktu_write("Tools/lure/src/lib.rs", "pub fn seated() -> u8 {\n    1\n}\n");

    lure.bktu_write("Tools/hitch/Cargo.toml", &zbktd_manifest("hitch"));
    lure.bktu_write("Tools/hitch/src/lib.rs", "pub fn hitched() -> u8 {\n    2\n}\n");

    // THE ADMITTED CRATE STANDS FROM THE START so the admission below is one line
    // of a manifest rather than a tree, and so it needs no network: a path
    // dependency has no registry closure, which is what keeps these drives honest
    // behind the tackroom fence.
    lure.bktu_write("Tools/spur/Cargo.toml", &zbktd_manifest("spur"));
    lure.bktu_write("Tools/spur/src/lib.rs", "pub fn spurred() -> u8 {\n    3\n}\n");

    zbktd_arrears_collar(
        &lure,
        BKTD_COLLAR_LURE,
        "Tools/lure",
        "Tools/lure Tools/hitch Tools/spur",
        "hitch spur",
    );
    zbktd_arrears_collar(
        &lure,
        BKTD_COLLAR_HITCH,
        "Tools/hitch",
        "Tools/hitch Tools/spur",
        "spur",
    );
    lure.bktu_commit("stand the arrears lure up");

    // Upstream first, then downstream: the downstream lock covers the upstream
    // crate's closure, so deriving it second is what makes it answer.
    zbktd_gangline(&lure, BKTD_COLLAR_HITCH);
    lure.bktu_commit("hitch's lock");
    zbktd_gangline(&lure, BKTD_COLLAR_LURE);
    lure.bktu_commit("lure's lock");

    lure
}

/// Admit a path dependency to the UPSTREAM crate, and commit — so the drives that
/// follow meet a clean tree in which two locks are behind: the upstream crate's
/// own, and the downstream crate's that covers it.
fn zbktd_admit(lure: &bktu_Lure) {
    lure.bktu_write(
        "Tools/hitch/Cargo.toml",
        "[package]\nname = \"hitch\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\
         [dependencies]\nspur = { path = \"../spur\" }\n",
    );
    lure.bktu_commit("admit a dependency upstream");
}

/// A dependency admitted upstream: the door writes the one lock it was asked for
/// and NAMES the downstream lock the write left behind, with the drive it owes.
///
/// THIS IS THE WHOLE POINT OF THE READING, and the trunk is what it is for: a
/// lock re-derived for one collar left the engine's consumer behind, nothing said
/// so, and the debt surfaced days later as a build refusing at a locked tree
/// read. The door now says it at the moment the debt is created.
///
/// THE VERDICT IS RED AND THE LOCK IS STILL WRITTEN, which is why the exit is
/// asserted as non-zero rather than as the door's own refusal code: a caller must
/// still notch what stands dirty, and a reading that conflated "wrote nothing"
/// with "wrote, and left a debt" would make the notch-gangline-notch sequence
/// unreadable from the exit.
#[test]
fn bktd_the_gangline_names_the_lock_its_write_left_behind() {
    let lure = zbktd_arrears_lure("arrears-named");
    zbktd_admit(&lure);

    let out = zbktd_gangline(&lure, BKTD_COLLAR_HITCH);
    let said = zbktd_said(&out);

    assert!(
        !out.status.success(),
        "a lock left behind by the write is a red verdict: {} {}",
        out.status,
        said
    );
    assert!(
        said.contains(BKTD_LURE_MANIFEST),
        "the verdict names the MANIFEST whose lock is behind: {}",
        said
    );
    assert!(
        said.contains(&format!("{} {}", BKTD_DOOR_GANGLINE, BKTD_COLLAR_LURE)),
        "the verdict spells the drive the debt owes: {}",
        said
    );

    // THE COVERAGE RIDES THE RED TOO, so a reading that had probed nothing could
    // not satisfy this hurdle by naming the manifest through some other road.
    assert!(
        said.contains("of 1 downstream"),
        "the verdict says how many locks it probed: {}",
        said
    );
}

/// The same drive once the downstream lock is re-derived: nothing is named, and
/// the door stands green.
///
/// THE CONTROL FOR THE HURDLE ABOVE, and it is load-bearing in both directions.
/// It proves the red was about the DEBT rather than about the admission — a door
/// that reddened whenever a dependency had been admitted would clear the first
/// hurdle and fail here — and it proves the green is a reading rather than a
/// silence, because the coverage line must still count the collar it probed.
/// A green saying only that nothing was found would be satisfied by a walk that
/// reached no collar at all.
#[test]
fn bktd_the_gangline_stands_green_once_the_debt_is_settled() {
    let lure = zbktd_arrears_lure("arrears-settled");
    zbktd_admit(&lure);

    // Settle the debt the way the verdict said to, through the door itself.
    zbktd_gangline(&lure, BKTD_COLLAR_LURE);
    lure.bktu_commit("settle the downstream lock");

    let out = zbktd_gangline(&lure, BKTD_COLLAR_HITCH);
    let said = zbktd_said(&out);

    assert!(
        out.status.success(),
        "with the downstream lock re-derived the door stands green: {} {}",
        out.status,
        said
    );
    assert!(
        said.contains("1 downstream lock(s) probed"),
        "the green counts the lock it probed, or it is a silence rather than a reading: {}",
        said
    );
    assert!(
        !said.contains(BKTD_LURE_MANIFEST),
        "a settled lock is named by nothing: {}",
        said
    );
}

/// The door still refuses two collar names.
///
/// THE ARREARS READING WIDENED WHAT THE DOOR REPORTS AND NOT WHAT IT WRITES, and
/// this is the hurdle that says so from the outside. The one-collar refusal is
/// the operator's election standing in the door's own grammar; a door that had
/// grown a sweep would most plausibly grow it here first.
#[test]
fn bktd_the_gangline_still_refuses_two_collars() {
    let lure = zbktd_arrears_lure("arrears-two-collars");

    let out = lure.bktu_drive_door(
        zbktd_kennel(),
        &[BKTD_DOOR_GANGLINE, BKTD_COLLAR_HITCH, BKTD_COLLAR_LURE],
    );
    let said = zbktd_said(&out);

    assert!(
        !out.status.success(),
        "two collars are refused: {} {}",
        out.status,
        said
    );
    assert!(
        said.contains("does not sweep"),
        "the refusal says the door does not sweep: {}",
        said
    );
}

////////////////////////////////////////////////////////////////////////////////
// The python launch — real tier
//
// Mush over a converged python collar, on the converge hurdle's own precedent:
// these reach the station's real tackroom, its shared interpreter store and its
// warm wheel cache, because what they prove cannot be observed without a venv
// that actually stands.
//
// WHY THE SEATING IS NOT THE CONVERGE HURDLES'. Theirs composes a
// DEPENDENCY-FREE project and converges nothing — the converge is the act under
// test there. This one composes a project carrying a dependency and cases to
// run, and converges it as a PRECONDITION, so that what the drive judges is the
// launch. Two seatings for two acts; neither is the other's control.

/// The pin these lures declare, and the same one the converge hurdles use — the
/// station's store is shared and content-keyed, so a second pin would buy a
/// second interpreter download for nothing.
const ZBKTD_PYTHON_PIN: &str = "3.12.7";

/// The collar every python drive below composes.
const ZBKTD_PY_COLLAR: &str = "suite-lure";

/// Compose a python lure, converge it, and hand back what a drive needs.
///
/// THE MIRRORED KIBBLE IS THE ESTATE'S OWN BYTES, laid through a station seat on
/// the converge hurdle's ground: a drive SPAWNS uv, so the declaration must
/// compose the residence the station actually holds, and a fabricated version
/// could not. Copying keeps one home for the pin, which a bump reaches without
/// anyone remembering these hurdles exist.
///
/// EVERY SETUP STEP IS ASSERTED LANDED. A step that failed quietly would leave a
/// drive proving something about an environment that was never built, while
/// reading as a statement about the launch.
fn zbktd_pythoned(named: &str, manifest: &str, baked: &str, files: &[(&str, &str)]) -> bktu_Lure {
    let lure = zbktd_python_authored(named, manifest, baked, files);

    let (collar, seat) = zbktd_python_seat(&lure);

    bkk::bkch_heel::bkch_python_at(lure.bktu_root(), &lure.bktu_weaned(), &collar, &seat)
        .unwrap_or_else(|err| panic!("the lure would not converge: {}", err));

    // THE LURE IS COMMITTED AFTER THE CONVERGE because the door refuses an
    // uncommitted repository — and the converge writes outside the source tree by
    // ruling, so there is nothing of it to commit.
    lure.bktu_commit("the converged python lure");

    lure
}

/// Compose a python lure and author its lock, converging NOTHING: the hurdle
/// that holds it decides what stands where the environment would.
fn zbktd_python_authored(named: &str, manifest: &str, baked: &str, files: &[(&str, &str)]) -> bktu_Lure {
    let lure = bktu_Lure::bktu_compose(named);

    lure.bktu_project(
        "project",
        ZBKTD_PY_COLLAR,
        &bktu_lure::bktu_Bent {
            pin: ZBKTD_PYTHON_PIN.to_string(),
            requires: ">=3.12".to_string(),
            lockless: true,
            baked: baked.to_string(),
            ..Default::default()
        },
    );

    lure.bktu_write("project/pyproject.toml", manifest);

    for (at, said) in files {
        lure.bktu_write(at, said);
    }

    lure.bktu_stationed().bktu_uv_seat(&lure);

    zbktd_python_lock(&lure);

    lure
}

/// The lure's python collar, and the seat a drive over the lure reads its
/// environment at.
fn zbktd_python_seat(lure: &bktu_Lure) -> (bkk::bkcx_python::bkcx_Collar, bkk::bkcx_python::bkcx_Seat) {
    let collar = bkk::bkcx_python::bkcx_resolve(lure.bktu_root(), &lure.bktu_weaned(), ZBKTD_PY_COLLAR)
        .expect("the seated python collar resolves")
        .collar;

    let tackroom = bkcl_leash::bkcl_fenced()
        .expect("the suite runs through a door that states the tackroom");

    let seat = bkk::bkcx_python::bkcx_seat_at(lure.bktu_loosebox(), &tackroom, &collar);

    (collar, seat)
}

/// Install the pinned interpreter and author the lock over the lure's project as
/// its manifest stands now.
fn zbktd_python_lock(lure: &bktu_Lure) {
    let (collar, seat) = zbktd_python_seat(lure);
    let project = lure.bktu_root().join(collar.bkcx_project());

    // THE INTERPRETER BEFORE THE LOCK, which is the routine-download ruling's own
    // ordering in service rather than a wrinkle of the test: the lock-authoring
    // door stands in the reaching posture and may not fetch an interpreter, so on
    // a cold store there is nothing to resolve against until the install has run.
    let fetching = bkk::bkcx_python::bkcx_stated(
        &seat,
        bkk::bkcx_python::bkcx_Posture::Fetching,
    );
    let installed = bkcl_leash::bkcl_uv(
        lure.bktu_root(),
        &lure.bktu_weaned(),
        lure.bktu_root(),
        [
            OsStr::new("python"),
            OsStr::new("install"),
            OsStr::new("--no-bin"),
            OsStr::new(ZBKTD_PYTHON_PIN),
        ],
        &bkk::bkcx_python::bkcx_borne(&fetching),
    )
    .expect("the install invocation reaches uv");
    assert!(
        installed.bkcl_landed(),
        "the pinned interpreter was not installed, so nothing below proves what it claims: {}",
        installed.spelling
    );

    let reaching = bkk::bkcx_python::bkcx_stated(
        &seat,
        bkk::bkcx_python::bkcx_Posture::Reaching,
    );
    let authored = bkcl_leash::bkcl_uv(
        lure.bktu_root(),
        &lure.bktu_weaned(),
        &project,
        [
            OsStr::new("lock"),
            OsStr::new("--project"),
            project.as_os_str(),
        ],
        &bkk::bkcx_python::bkcx_borne(&reaching),
    )
    .expect("the lock-authoring invocation reaches uv");
    assert!(
        authored.bkcl_landed(),
        "the lock was not authored, so there is nothing to converge: {}",
        authored.spelling
    );
}

/// A project declaring pytest alone, and the cases the drives narrow over.
const ZBKTD_PY_SUITE: &str = "\
[project]
name = \"lure\"
version = \"0.0.1\"
requires-python = \">=3.12\"
dependencies = [\"pytest\"]
";

/// What the suite manifest's collar names baked: pytest, as the lock spells it.
const ZBKTD_PY_SUITE_BAKED: &str = "pytest";

const ZBKTD_PY_CASES: &[(&str, &str)] = &[
    (
        "project/tests/test_alpha.py",
        "def test_one():\n    assert True\n\n\ndef test_two():\n    assert True\n",
    ),
    (
        "project/tests/test_beta.py",
        "def test_three():\n    assert True\n",
    ),
];

#[test]
fn bktd_mush_over_a_python_suite_runs_the_matching_node_ids_and_names_the_narrowing() {
    let lure = zbktd_pythoned("drive-python-suite", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);

    let out = lure.bktu_drive_door(
        zbktd_kennel(),
        &["mush", ZBKTD_PY_COLLAR, "test_alpha"],
    );

    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(out.status.success(), "the green python course did not land: {}", said);

    // THE NARROWING IS THE KENNEL'S OWN COUNT, taken from the listing rather than
    // from the run: two of the three, said before anything was spawned.
    assert!(
        said.contains("ran 2 of 3"),
        "the verdict does not name the narrowing: {}",
        said
    );

    // AND THE VERDICT NAMES WHAT THE VOICE COUNTED, which is the tongue's whole
    // reason for reading per-case lines: a course that ran two cases and passed
    // both, said in the kennel's own words rather than pytest's.
    assert!(
        said.contains("green: 2 ran, 2 passed, 0 failed"),
        "the verdict does not name ran, passed and failed: {}",
        said
    );

    // THE UNMATCHED CASE DID NOT RUN. The count above could be satisfied by a
    // runner that ran everything and reported two, so the name is asserted absent
    // from the course as well.
    assert!(
        !said.contains("test_three"),
        "a case the pattern did not match was run anyway: {}",
        said
    );

    // A SECOND DRIVE REACHES NO INDEX AND FINDS THE TREE CLEAN. Both halves are
    // asserted by one act, and the second is the one that was nearly missed: the
    // first drive of this hurdle left a `__pycache__` beside the lure's own
    // sources, so THIS refusal — the door's own, on a repository it had just run
    // over — is what a launch writing into a source tree looks like from the
    // outside.
    // THE SECOND DRIVE IS A SECOND DISPATCH AND IS STATED AS ONE. A course record
    // is one write per collar per dispatch, so two drives sharing an output
    // directory is two courses claiming one collar — which the door refuses,
    // correctly. Handing the second its own directory is what makes this a
    // repeat of the launch rather than a test of the record's cardinality.
    // OUTSIDE THE CHECKOUT, because a directory composed inside it would dirty
    // the very repository the drive is about to be refused for.
    // BOTH ROOTS, because a course record is banked in each and one of them left
    // stated would collide exactly as sharing both did.
    let second = lure.bktu_temp().join("second-dispatch");
    let banked = second.join("out");
    let scratch = second.join("tmp");
    std::fs::create_dir_all(&banked).expect("the hurdle composes the second dispatch's output");
    std::fs::create_dir_all(&scratch).expect("the hurdle composes the second dispatch's temp");

    let again = lure.bktu_drive_stating(
        zbktd_kennel(),
        &["mush", ZBKTD_PY_COLLAR, "test_alpha"],
        &[
            ("BURD_OUTPUT_DIR", banked.as_os_str()),
            ("BURD_TEMP_DIR", scratch.as_os_str()),
        ],
    );
    assert!(
        again.status.success(),
        "a second drive over a standing environment did not land: {}{}",
        String::from_utf8_lossy(&again.stdout),
        String::from_utf8_lossy(&again.stderr)
    );
}

#[test]
fn bktd_mush_over_a_python_suite_refuses_a_pattern_matching_nothing_before_it_spawns() {
    let lure = zbktd_pythoned("drive-python-empty", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);

    let out = lure.bktu_drive_door(
        zbktd_kennel(),
        &["mush", ZBKTD_PY_COLLAR, "test_absent"],
    );

    let said = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success(), "a pattern matching nothing refuses: {}", said);
    assert_eq!(
        out.status.code(),
        Some(75),
        "the refusal did not take the pipeline's own empty-match code: {}",
        said
    );

    assert!(
        said.contains("NOTHING WAS SPAWNED"),
        "the refusal does not say the runner was never reached: {}",
        said
    );
    assert!(
        said.contains("of the 3 hurdle(s)"),
        "the refusal does not say of how many: {}",
        said
    );
}

#[test]
fn bktd_lineup_over_a_python_collar_lists_its_node_ids_and_runs_nothing() {
    let lure = zbktd_pythoned("drive-python-lineup", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);

    let out = lure.bktu_drive_door(zbktd_kennel(), &["lineup", ZBKTD_PY_COLLAR]);

    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(out.status.success(), "the python listing door did not land: {}", said);
    assert!(
        said.contains("test_alpha.py::test_one"),
        "it does not name a node id the recite reading holds: {}",
        said
    );
    assert!(said.contains("holds 3 hurdle(s)"), "it does not count them: {}", said);

    // THE NO-RUN, on the rust arm's own ground: a door that listed and then ran
    // would say so on pytest's own two-sentence verdict, and neither is here.
    assert!(!said.contains("green:"), "nothing ran: {}", said);
    assert!(!said.contains(" ran, "), "nothing ran: {}", said);
}

#[test]
fn bktd_mush_over_a_python_suite_refuses_a_pattern_matching_nothing_naming_lineup() {
    let lure = zbktd_pythoned("drive-python-empty-names-lineup", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);

    let out = lure.bktu_drive_door(
        zbktd_kennel(),
        &["mush", ZBKTD_PY_COLLAR, "test_absent"],
    );

    let said = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success(), "a pattern matching nothing refuses: {}", said);
    assert_eq!(
        out.status.code(),
        Some(75),
        "the refusal did not take the pipeline's own empty-match code: {}",
        said
    );

    // THE REFUSAL NAMES THE DOOR that can show the names the patterns are
    // matched against, on the rust arm's own ground — a caller handed nothing
    // else to try is a dangling pointer, and this asserts the pointer resolves.
    assert!(
        said.contains(&format!("lineup {}", ZBKTD_PY_COLLAR)),
        "the refusal does not name lineup as the door to see the node ids: {}",
        said
    );
}

#[test]
fn bktd_mush_over_a_python_suite_collecting_no_case_refuses_at_the_hollow_gate() {
    // TWO LURES AND NOT ONE, WHICH THE BARE DRIVE FORCES. The gate is reached by
    // a drive carrying no pattern — a narrowed drive refuses at the empty match
    // long before a runner is spawned — and a bare drive collects the WHOLE
    // project, so the hollow half and its control cannot be two selections over
    // one project. They share every other fact: the same pin, the same mirrored
    // kibble, the same station, the same converge.

    // THE CONTROL: one passing case, collected and counted, and the gate silent.
    let standing = zbktd_pythoned(
        "drive-python-hollow-control",
        ZBKTD_PY_SUITE,
        ZBKTD_PY_SUITE_BAKED,
        &[("project/tests/test_solo.py", "def test_only():\n    assert True\n")],
    );

    let control = standing.bktu_drive_door(zbktd_kennel(), &["mush", ZBKTD_PY_COLLAR]);

    let control_said = format!(
        "{}{}",
        String::from_utf8_lossy(&control.stdout),
        String::from_utf8_lossy(&control.stderr)
    );

    assert!(
        control.status.success(),
        "the control drive over one passing case did not land: {}",
        control_said
    );
    assert!(
        control_said.contains("green: 1 ran, 1 passed, 0 failed"),
        "the control did not count its one case: {}",
        control_said
    );

    // THE GATE: a project holding a file and no case at all. pytest collects
    // nothing and exits its own empty-collection code; the kennel reads that one
    // value and answers in its own vocabulary instead.
    let barren = zbktd_pythoned(
        "drive-python-hollow",
        ZBKTD_PY_SUITE,
        ZBKTD_PY_SUITE_BAKED,
        &[("project/tests/test_barren.py", "# no case stands in this file\n")],
    );

    let hollow = barren.bktu_drive_door(zbktd_kennel(), &["mush", ZBKTD_PY_COLLAR]);

    let hollow_said = String::from_utf8_lossy(&hollow.stderr);

    assert_eq!(
        hollow.status.code(),
        Some(83),
        "a course that collected no case did not take the hollow gate's own code: {}",
        hollow_said
    );

    assert!(
        hollow_said.contains("counted no case at all"),
        "the refusal does not say what was hollow about the course: {}",
        hollow_said
    );
}

/// The app hurdles' project: one entry point under a build backend.
///
/// THE ENTRY POINT IS THE PROJECT'S OWN SCRIPT, which means the project must be
/// BUILT and installed rather than merely resolved — a virtual project declaring
/// no build system installs nothing and would leave the venv with no script to
/// run. The backend is declared for that reason and for no other, and it is
/// uv's own: the project's build is the one build the converge performs under
/// the per-package bar, its requirements standing in no lock, so the backend the
/// pinned uv carries built in is the one the validation admits.
const ZBKTD_PY_APP_MANIFEST: &str = "\
[project]
name = \"lure\"
version = \"0.0.1\"
requires-python = \">=3.12\"

[project.scripts]
lure-app = \"lure_app:main\"

[build-system]
requires = [\"uv_build>=0.11.0,<0.12.0\"]
build-backend = \"uv_build\"

[tool.uv.build-backend]
module-name = \"lure_app\"
module-root = \"src\"
";

/// The app's one module: it prints what it was handed and exits with a value
/// nobody could mistake for the kennel's own.
const ZBKTD_PY_APP_FILES: &[(&str, &str)] = &[(
    "project/src/lure_app/__init__.py",
    "import sys\n\n\ndef main():\n    print(\"the lure app ran\", *sys.argv[1:])\n    return 17\n",
)];

/// The app collar over the composed project, answering to the byname given.
fn zbktd_py_app_collar(byname: &str) -> String {
    format!(
        "\
BKRP_COLLAR=\"suite-lure\"
BKRP_KIND=\"bknre_app\"
BKRP_MANIFEST=\"project/pyproject.toml\"
BKRP_ROOTS=\"project/pyproject.toml\"
BKRP_SPEND=\"bknre_reader\"
BKRP_BAKED_PACKAGES=\"\"
BKRP_CRUDE_PACKAGES=\"bknre_baked\"
BKRP_BYNAME=\"{}\"
",
        byname, //
    )
}

#[test]
fn bktd_mush_over_a_python_app_runs_its_entry_point_and_passes_its_exit_through() {
    // A PLANTED NONZERO EXIT, which is the whole of what this hurdle proves. An
    // app's exit is the TENANT'S and the door translates none of it, so a value
    // nobody could mistake for the kennel's own is what shows the exit crossed
    // the door untouched.
    let lure = zbktd_pythoned("drive-python-app", ZBKTD_PY_APP_MANIFEST, "", ZBKTD_PY_APP_FILES);

    // The collar is the suite shape the composer lays down; the app kind and its
    // byname are what this hurdle needs, so the declaration is restated whole.
    lure.bktu_write(&lure.bktu_weaned_at("suite-lure/bkrp.env"), &zbktd_py_app_collar("lure-app"));
    lure.bktu_commit("the app collar over the converged lure");

    let out = lure.bktu_drive_door(
        zbktd_kennel(),
        &["mush", ZBKTD_PY_COLLAR, "carried", "verbatim"],
    );

    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    assert_eq!(
        out.status.code(),
        Some(17),
        "the tenant's own exit did not cross the door untouched: {}",
        said
    );

    // AND ITS ARGUMENTS REACHED IT VERBATIM. An app holds the terminal, so what
    // it printed is proof both that the entry point ran and that nothing of the
    // door's stood between it and its caller.
    assert!(
        said.contains("the lure app ran carried verbatim"),
        "the entry point did not run with its arguments passed through: {}",
        said
    );

    // AND IT WROTE NO BYTECODE BESIDE ITS SOURCES. The converge installs the
    // project editable, so the app imports its modules from the source tree; the
    // bytecode it compiles lands under the loosebox or the tree is written by a
    // launch, which is the breach the bytecode root exists to close.
    let beside = lure.bktu_root().join("project/src/lure_app/__pycache__");
    assert!(
        !beside.exists(),
        "the app wrote bytecode beside its sources at {}",
        beside.display()
    );
}

/// A byname the project never declared: the name an app collar's entry point
/// answers to, standing in no `[project.scripts]` table.
const ZBKTD_PY_APP_UNDECLARED: &str = "lure-app-undeclared";

/// An app collar naming an entry point its project never declared refuses at the
/// launch, naming the entry point it looked for.
///
/// THE PROJECT IS THE APP HURDLE'S OWN, CONVERGED AND WHOLE, so the one thing
/// wrong is the name the collar says its entry point answers to.
#[test]
fn bktd_mush_over_a_python_app_naming_an_undeclared_entry_point_refuses() {
    let lure = zbktd_pythoned("drive-python-app-undeclared", ZBKTD_PY_APP_MANIFEST, "", ZBKTD_PY_APP_FILES);

    lure.bktu_write(
        &lure.bktu_weaned_at("suite-lure/bkrp.env"),
        &zbktd_py_app_collar(ZBKTD_PY_APP_UNDECLARED),
    );
    lure.bktu_commit("an app collar naming a script its project never declared");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[bkco_output::BKCO_VERB_MUSH, ZBKTD_PY_COLLAR]);

    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );

    assert!(
        !out.status.success(),
        "an entry point the project never declared launched: {}",
        said
    );

    assert!(
        said.contains(ZBKTD_PY_APP_UNDECLARED),
        "the refusal does not name the entry point it looked for: {}",
        said
    );
}

// The first python lock — through the doors
//
// The library hurdles prove each act. These prove the door faces: tattoo reaches
// its python arm, and the two doors whose acts repair an absent lock read past
// that finding while every other finding still bars.

/// The exit a door takes when a collar carries a finding, and the exit heel takes
/// when a converge refuses — spelled here because the constants are the binary's
/// own and this hurdle stands outside it.
const ZBKTD_EXIT_FINDINGS: i32 = 72;
const ZBKTD_EXIT_HEEL: i32 = 77;

/// Compose a lure whose python project awaits its first lock, and commit it.
///
/// NOTHING IS INSTALLED OR AUTHORED HERE, so the whole first-lock order is the
/// hurdle's own to drive through the doors.
fn zbktd_lockless(named: &str) -> bktu_Lure {
    let lure = bktu_Lure::bktu_compose(named);

    lure.bktu_project(
        "project",
        ZBKTD_PY_COLLAR,
        &bktu_lure::bktu_Bent {
            pin: ZBKTD_PYTHON_PIN.to_string(),
            requires: ">=3.12".to_string(),
            lockless: true,
            ..Default::default()
        },
    );

    lure.bktu_stationed().bktu_uv_seat(&lure);

    lure.bktu_commit("a python project awaiting its first lock");

    lure
}

/// Tattoo over a python collar proclaims it in the common vocabulary, the
/// language its family's and the election the seat's own.
#[test]
fn bktd_tattoo_over_a_python_collar_proclaims_it() {
    let lure = bktu_Lure::bktu_compose("drive-python-tattoo");

    lure.bktu_project("project", ZBKTD_PY_COLLAR, &bktu_lure::bktu_Bent::default());
    lure.bktu_commit("a python collar to proclaim");

    let out = lure.bktu_drive_door(
        zbktd_kennel(),
        &[bkco_output::BKCO_VERB_TATTOO, ZBKTD_PY_COLLAR],
    );

    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );

    assert!(out.status.success(), "the python proclamation did not land: {}", said);

    assert!(
        said.contains(&format!(
            "BKRC_LANGUAGE=\"{}\"",
            bkk::bkct_tattoo::BKCT_LANGUAGE_PYTHON, //
        )),
        "the proclamation named no python language value: {}",
        said
    );

    assert!(
        said.contains(&format!(
            "BKRC_ELECTION=\"{}\"",
            bkk::bkce_election::BKCE_ELECTED_OWN, //
        )),
        "the proclamation did not elect the seat's own environment: {}",
        said
    );
}

/// A project's first lock taken through the doors, in the order a cold store
/// imposes and the notch the door law demands between them.
///
/// TATTOO OPENS IT AS THE CONTROL: a door whose act is not the repair refuses the
/// absent lock and names the door that is.
#[test]
fn bktd_a_first_python_lock_runs_heel_then_gangline_then_heel() {
    let lure = zbktd_lockless("drive-python-first-lock");
    let lock = lure.bktu_root().join("project").join(bkk::bkcx_python::BKCX_LOCK_FILE);

    let drive = |door: &str| {
        let out = lure.bktu_drive_door(zbktd_kennel(), &[door, ZBKTD_PY_COLLAR]);
        let said = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr), //
        );
        (out.status.code(), said)
    };

    let (code, said) = drive(bkco_output::BKCO_VERB_TATTOO);
    assert_eq!(
        code,
        Some(ZBKTD_EXIT_FINDINGS),
        "the control: a lockless project's proclamation refuses on the lock: {}",
        said
    );
    assert!(
        said.contains(bkco_output::BKCO_VERB_GANGLINE),
        "the refusal did not name the door that writes a first lock: {}",
        said
    );

    let (code, said) = drive(bkco_output::BKCO_VERB_HEEL);
    assert_eq!(
        code,
        Some(ZBKTD_EXIT_HEEL),
        "the converge over a lockless project did not stop after its install: {}",
        said
    );
    assert!(
        said.contains(bkco_output::BKCO_VERB_GANGLINE) && !lock.exists(),
        "the converge did not name the door that writes a first lock, or wrote one itself: {}",
        said
    );

    let (code, said) = drive(bkco_output::BKCO_VERB_GANGLINE);
    assert_eq!(code, Some(0), "the first lock was not written: {}", said);
    assert!(lock.is_file(), "no lock stands at {} after the door wrote one", lock.display());

    lure.bktu_commit("the first lock");

    let (code, said) = drive(bkco_output::BKCO_VERB_HEEL);
    assert_eq!(code, Some(0), "the converge refused the committed first lock: {}", said);

    let (code, said) = drive(bkco_output::BKCO_VERB_TATTOO);
    assert_eq!(code, Some(0), "the proclamation still refuses once the lock stands: {}", said);
}

/// A quiet case and a case that raises a warning, the second's word one that
/// matches no governed word.
const ZBKTD_PY_WARNS: &[(&str, &str)] = &[
    (
        "project/tests/test_quiet.py",
        "def test_quiet():\n    assert True\n",
    ),
    (
        "project/tests/test_loud.py",
        "import warnings\n\n\ndef test_loud():\n    warnings.warn(\"a planted caution\")\n",
    ),
];

/// A case that raises a warning fails its course: the kennel makes every warning
/// an error, where pytest's own default prints it in a summary and passes.
///
/// THE QUIET CASE BESIDE IT IS THE CONTROL, passing in the same run, so what went
/// red is the warning and nothing else about the course.
#[test]
fn bktd_mush_over_a_python_suite_fails_the_case_that_warns() {
    let lure = zbktd_pythoned("drive-python-warns", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_WARNS);

    let out = lure.bktu_drive_door(zbktd_kennel(), &[bkco_output::BKCO_VERB_MUSH, ZBKTD_PY_COLLAR]);

    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );

    assert!(
        !out.status.success(),
        "a course whose case warned landed green: {}",
        said
    );

    assert!(
        said.contains("2 ran, 1 passed, 1 failed") && said.contains("test_loud"),
        "the verdict does not name the warning case as the one that failed: {}",
        said
    );
}

// The python walk — real tier
//
// The derby and the futurity over a yard holding a python course beside a rust
// one, on the python launch's own seating: these converge against the station's
// real tackroom because what they prove is the walk reaching a venv that stands.

/// Lay a rust suite course beside the lure's python one and commit both: the
/// crate whose one hurdle writes its mark and passes, so the disk says whether
/// its runner was spawned.
fn zbktd_rust_beside(lure: &bktu_Lure) {
    lure.bktu_write("clippy.toml", BKTD_CLIPPY);

    zbktd_crate(lure, "Tools/bravo", "bravo", "src/lib.rs", BKTD_MARKED);
    zbktd_suite_collar(lure, "Tools/bravo", "suite-bravo");

    lure.bktu_commit("a rust course beside the python one");
}

/// Both streams of a drive, read as one console.
fn zbktd_heard(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    )
}

/// Whether a preflight report names the converge over the lure's python collar
/// as the act supplying an absence.
///
/// THE LINE IS READ BY ITS VERB AND ITS IMPRINT, and never by the tabtarget
/// between them: which file the door resolves to is the dispatch's to say, and a
/// lure's spawn is not this run's dispatch.
fn zbktd_supplied_by_heel(said: &str) -> bool {
    said.lines().any(|line| {
        line.trim_start().starts_with("supplied by:")
            && line.contains(bkco_output::BKCO_VERB_HEEL)
            && line.trim_end().ends_with(ZBKTD_PY_COLLAR)
    })
}

/// THE DERBY RACES A PYTHON COURSE BESIDE A RUST ONE: both courses run, each
/// answers in the kennel's own verdict, and the python course banks its record
/// as every derby course does — whole, and carrying its wall time.
///
/// THE RUST COURSE'S MARK IS THE PROOF IT RAN, and the python course's count is
/// the proof it ran: three cases is the whole project, which only a spawned
/// pytest could have counted.
#[test]
fn bktd_the_derby_races_a_python_course_beside_a_rust_one() {
    let lure = zbktd_pythoned("derby-python-beside-rust", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);
    zbktd_rust_beside(&lure);

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = zbktd_heard(&out);

    assert!(out.status.success(), "the derby over both families did not land: {}", said);
    assert!(
        said.contains("the derby is green across 2 course(s)"),
        "the derby did not count both courses: {}",
        said
    );
    assert!(mark.exists(), "the rust course's runner was never spawned: {}", mark.display());
    assert!(
        said.contains("green: 3 ran, 3 passed, 0 failed"),
        "the python course did not run its whole project: {}",
        said
    );

    let leaf = format!("{}.{}", ZBKTD_PY_COLLAR, bkcc_record::BKCC_EXTENSION);
    let banked = lure.bktu_output().join(&leaf);
    let record = std::fs::read_to_string(&banked)
        .unwrap_or_else(|err| panic!("no record stands for the python course at {}: {}", banked.display(), err));
    let read = bkcc_record::bkcc_admit(&record, &leaf)
        .unwrap_or_else(|err| panic!("the python course's record admits: {}", err));

    assert_eq!(read.driven, 3, "the record does not carry the three cases: {}", record);
    assert!(read.narrowing.is_none(), "nothing narrowed the course: {}", record);
    assert!(read.wall.is_some(), "a whole course carries its wall time: {}", record);
}

/// A PATTERN NAMING CASES IN THE PYTHON COURSE RUNS THAT COURSE ALONE, and the
/// verdict names the rust course as not launched. The rust course's unwritten
/// mark says from the disk that its runner was never spawned, and the python
/// course's count says the narrowing reached pytest as exact node ids.
#[test]
fn bktd_the_futurity_narrows_over_a_python_course() {
    let lure = zbktd_pythoned("futurity-python", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);
    zbktd_rust_beside(&lure);

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_FUTURITY, "test_alpha"]);
    let said = zbktd_heard(&out);

    assert!(out.status.success(), "the narrowed walk did not land: {}", said);
    assert!(
        said.contains("green: 2 ran, 2 passed, 0 failed"),
        "the python course did not run the two matched cases alone: {}",
        said
    );
    assert!(
        said.contains("not launched") && said.contains("suite-bravo"),
        "the verdict does not name the course the patterns left idle: {}",
        said
    );
    assert!(!mark.exists(), "the idle rust course's runner was spawned: {}", mark.display());
}

/// AN UNCONVERGED PYTHON COLLAR IS REFUSED AT THE PREFLIGHT, naming the converge,
/// and no course runs. The lock stands and the interpreter is installed, so the
/// one thing missing is the environment.
///
/// THE CONTROL IS THE SAME LURE AFTER THE NAMED ACT: the converge driven through
/// its own door, and the same derby then green — so what refused was the absent
/// environment, and the act the refusal named is the one that supplies it.
#[test]
fn bktd_the_derby_refuses_an_unconverged_python_collar_naming_the_converge() {
    let lure = zbktd_python_authored("derby-python-unconverged", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);
    lure.bktu_commit("a python project locked and never converged");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = zbktd_heard(&out);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_PREFLIGHT),
        "an unconverged python collar did not refuse at the preflight: {}",
        said
    );
    assert!(
        said.contains(&format!("the environment of the python collar '{}'", ZBKTD_PY_COLLAR))
            && said.contains("nothing stands at"),
        "the refusal does not name the absent environment: {}",
        said
    );
    assert!(
        zbktd_supplied_by_heel(&said),
        "the refusal does not name the converge over the python collar: {}",
        said
    );
    assert!(!said.contains("green:"), "a course ran behind a refused preflight: {}", said);

    let heeled = lure.bktu_drive_door(zbktd_kennel(), &[bkco_output::BKCO_VERB_HEEL, ZBKTD_PY_COLLAR]);
    assert!(
        heeled.status.success(),
        "the converge the refusal named did not land: {}",
        zbktd_heard(&heeled)
    );

    let again = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = zbktd_heard(&again);

    assert!(again.status.success(), "the derby after the converge did not land: {}", said);
    assert!(
        said.contains("green: 3 ran, 3 passed, 0 failed"),
        "the python course did not run once its environment stood: {}",
        said
    );
}

/// The lure's manifest with its one dependency dropped, so a lock re-derived
/// over it no longer names what the environment holds.
const ZBKTD_PY_BARE: &str = "\
[project]
name = \"lure\"
version = \"0.0.1\"
requires-python = \">=3.12\"
dependencies = []
";

/// AN ENVIRONMENT THE LOCK HAS MOVED PAST IS REFUSED AT THE PREFLIGHT, naming the
/// converge. The lock answers its manifest, so the reading that refuses is the
/// environment's against the lock — the one the launch's own `uv run` would have
/// answered by quietly syncing.
///
/// THE CONTROL OPENS IT: the same lure's derby, green over the converged
/// environment, so the preflight is proven to read the environment and pass it.
/// The lock then moves past it by a dependency dropped and the lock re-derived,
/// which needs no index.
#[test]
fn bktd_the_derby_refuses_a_python_environment_behind_its_lock() {
    let lure = zbktd_pythoned("derby-python-behind", ZBKTD_PY_SUITE, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);

    let control = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    assert!(
        control.status.success(),
        "the control: the derby over a converged environment did not land: {}",
        zbktd_heard(&control)
    );

    lure.bktu_write("project/pyproject.toml", ZBKTD_PY_BARE);
    zbktd_python_lock(&lure);

    // THE COLLAR FOLLOWS THE MANIFEST: with pytest dropped, the baked list names
    // nothing, or the partition reading would bar the preflight ahead of the
    // environment reading this hurdle is about.
    let collar = lure.bktu_root().join(lure.bktu_weaned_at("suite-lure/bkrp.env"));
    let said = std::fs::read_to_string(&collar).expect("the composed collar reads");
    std::fs::write(&collar, said.replace("BKRP_BAKED_PACKAGES=\"pytest\"", "BKRP_BAKED_PACKAGES=\"\""))
        .expect("the bent collar writes");

    lure.bktu_commit("the lock moved past the environment");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = zbktd_heard(&out);

    assert_eq!(
        out.status.code(),
        Some(ZBKTD_EXIT_PREFLIGHT),
        "an environment behind its lock did not refuse at the preflight: {}",
        said
    );
    assert!(
        said.contains("does not answer the lock"),
        "the refusal does not say the environment is behind its lock: {}",
        said
    );
    assert!(
        zbktd_supplied_by_heel(&said),
        "the refusal does not name the converge over the python collar: {}",
        said
    );
    assert!(!said.contains("green:"), "a course ran behind a refused preflight: {}", said);
}

// The unhitched course — real tier
//
// A python project declaring, in uv's own key, an environment no station
// satisfies: locked, never converged, and unhitched on every station, so each
// door's reading is proven here with no platform named.

/// The suite manifest declaring an environment no station satisfies.
const ZBKTD_PY_UNHITCHED: &str = "\
[project]
name = \"lure\"
version = \"0.0.1\"
requires-python = \">=3.12\"
dependencies = [\"pytest\"]

[tool.uv]
environments = [\"sys_platform == 'nowhere'\"]
";

/// THE DERBY NAMES AN UNHITCHED COURSE AND STAYS GREEN. The python course's
/// environment stands nowhere, and no converge could stand one, so the
/// preflight reads the declaration ahead of the environment and the walk
/// launches nothing of it, writing no record and moving no exit; the rust
/// course beside it runs, its mark the proof.
#[test]
fn bktd_the_derby_names_an_unhitched_python_course_and_stays_green() {
    let lure = zbktd_python_authored("derby-python-unhitched", ZBKTD_PY_UNHITCHED, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);
    zbktd_rust_beside(&lure);

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_DERBY]);
    let said = zbktd_heard(&out);

    assert!(out.status.success(), "the derby beside a course declared out of this station did not land: {}", said);
    assert!(
        said.contains("the derby is green across 1 course(s)"),
        "the derby did not count the one course it ran: {}",
        said
    );
    assert!(
        said.contains(bkk::bkg_breviary::BKG_UNHITCHED_BASE) && said.contains(ZBKTD_PY_COLLAR) && said.contains("nowhere"),
        "the set verdict does not name the declared-out course in its project's words: {}",
        said
    );
    assert!(mark.exists(), "the rust course's runner was never spawned: {}", mark.display());

    let leaf = format!("{}.{}", ZBKTD_PY_COLLAR, bkcc_record::BKCC_EXTENSION);
    let banked = lure.bktu_output().join(&leaf);
    assert!(!banked.exists(), "a declared-out course wrote a record at {}", banked.display());
}

/// THE FUTURITY NAMES AN UNHITCHED COURSE BESIDE THE IDLE ONES. The patterns
/// match the rust course's hurdle, and the python course is never listed — its
/// runner could not stand — and is named apart from the courses the patterns
/// left idle.
#[test]
fn bktd_the_futurity_names_an_unhitched_python_course() {
    let lure = zbktd_python_authored("futurity-python-unhitched", ZBKTD_PY_UNHITCHED, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);
    zbktd_rust_beside(&lure);

    let mark = lure.bktu_root().join("Tools/bravo/ran.txt");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[BKTD_DOOR_FUTURITY, "bktd_marked"]);
    let said = zbktd_heard(&out);

    assert!(out.status.success(), "the narrowed walk beside a declared-out course did not land: {}", said);
    assert!(
        said.contains(bkk::bkg_breviary::BKG_UNHITCHED_BASE) && said.contains(ZBKTD_PY_COLLAR) && said.contains("nowhere"),
        "the verdict does not name the declared-out course in its project's words: {}",
        said
    );
    assert!(mark.exists(), "the matched rust course's runner was never spawned: {}", mark.display());
}

/// MUSH REFUSES AN UNHITCHED COLLAR NAMING ITS DECLARATION, and never the
/// converge, which cannot supply what a project declared away.
#[test]
fn bktd_mush_refuses_an_unhitched_python_collar_naming_the_declaration() {
    let lure = zbktd_python_authored("mush-python-unhitched", ZBKTD_PY_UNHITCHED, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);
    lure.bktu_commit("a python project declared for no station");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[bkco_output::BKCO_VERB_MUSH, ZBKTD_PY_COLLAR]);
    let said = zbktd_heard(&out);

    assert!(!out.status.success(), "mush launched a collar declared out of this station: {}", said);
    assert!(
        said.contains(bkk::bkg_breviary::BKG_UNHITCHED_BASE) && said.contains("nowhere"),
        "the refusal does not name the declaration: {}",
        said
    );
    assert!(!zbktd_supplied_by_heel(&said), "the refusal names the converge: {}", said);
    assert!(!said.contains("green:"), "a case ran behind the refusal: {}", said);
}

/// TATTOO READS AN UNHITCHED COLLAR AS A VERDICT AND NO FINDING: the collar
/// conforms, the door exits clean, and the declaration is said beside it.
#[test]
fn bktd_tattoo_reads_an_unhitched_python_collar_as_a_verdict_and_no_finding() {
    let lure = zbktd_python_authored("tattoo-python-unhitched", ZBKTD_PY_UNHITCHED, ZBKTD_PY_SUITE_BAKED, ZBKTD_PY_CASES);
    lure.bktu_commit("a python project declared for no station");

    let out = lure.bktu_drive_door(zbktd_kennel(), &[bkco_output::BKCO_VERB_TATTOO, ZBKTD_PY_COLLAR]);
    let said = zbktd_heard(&out);

    assert!(out.status.success(), "tattoo refused a collar declared out of this station: {}", said);
    assert!(said.contains("conforms"), "the collar's clean verdict is not said: {}", said);
    assert!(
        said.contains(bkk::bkg_breviary::BKG_UNHITCHED_BASE) && said.contains("nowhere"),
        "the declaration is not said beside the verdict: {}",
        said
    );
}
