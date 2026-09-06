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

//! The whistle's three refusals and rebuilds, proven by driving it.
//!
//! WHY THESE STAND HERE AND NOT IN THE KENNEL'S OWN SUITE. The whistle is the one
//! door that converges unasked, and it is bash: it builds the kennel binary, so a
//! kennel hurdle spawning it would rebuild the artifact its own run is being
//! judged from. The fold's rule — every refusal a door pace proved by hand
//! standing as a hurdle over a lure — cannot reach it for that reason, and hands
//! it to the substrate's suite, which is a different crate and stands outside that
//! circle.
//!
//! THE SEAT CARRIES A STAND-IN KENNEL, AND THAT IS THE POINT RATHER THAN A
//! SHORTCUT. What is under test is the whistle's three disciplines — the door law,
//! the staleness comparison and the stated pin — and not one of them reads what
//! the crate under it actually is. A seat carrying the real kennel would spend a
//! full compile per hurdle to prove the same three things, and would make a
//! whistle hurdle fail whenever the kennel failed to build, which is the other
//! door's verdict wearing this one's name. So the seat carries the smallest crate
//! that can answer the standing reading: a binary named `bkmx` that states the
//! position it was struck at, in the line shape the reading matches. It wears the
//! kennel's names because the whistle resolves them by name; nothing here mints
//! one.
//!
//! TWO VARIABLES ARE OVERRIDDEN IN THE COORDINATOR, and both are the composed
//! seat meeting a door that was written for a real tree. The tools directory is
//! made RELATIVE, because the whistle joins it onto the repository root and the
//! seat declares an absolute one so its dispatch can reach the kit standing
//! outside it — the two readings are both right and only one of them can hold in
//! one variable, so the coordinator holds the dispatch's until the dispatch is
//! done with it and the whistle's afterward. The tackroom is carried IN from the
//! suite's own dispatch, because the fence provisions a toolchain store and a
//! composed station declares none; inheriting the station's is what keeps a
//! hurdle from installing a toolchain of its own.
//!
//! OBSERVED FROM OUTSIDE THE DISPATCH, as every hurdle here is.

#![deny(warnings)]

use bkmk::bkmtu_lure::bkmtu_Lure;
use bkmk::bkmtu_lure::bkmtu_missing_toolchains;
use buk::buas_seat::buas_source;
use std::path::Path;

/// The channel the seat pins, matching the estate's own.
const BUJW_PIN: &str = "1.90.0";

/// The channel the bumped-pin hurdle moves to. A second toolchain must stand for
/// this to be provable at all, which is why the required set names two.
const BUJW_BUMPED: &str = "1.89.0";

/// The stand-in's own compiler line, which is where a channel must be found for
/// the finding to mean the compiler ANSWERED rather than that the channel was
/// merely mentioned.
///
/// THE READING IS SEPARATED FROM THE WRITING deliberately. The channel constant
/// above is written into the seat's pin file; searching the answer for that same
/// bare string would be satisfied by any line that happened to carry it, and the
/// falsification that planted an impossible channel could not tell the two halves
/// apart — it red-lined on the pin being READ, which is a different fact from the
/// compiler having RUN.
const BUJW_COMPILER_SAID: &str = "compiler rustc";

/// What the whistle says when the standing binary already holds the seat's
/// position.
const BUJW_STOOD: &str = "did not rebuild";

/// What the whistle says when it is about to build because the seat moved past
/// the binary.
const BUJW_OUTRAN: &str = "The seat has outrun the kennel";

/// What the whistle says when no binary stands at all.
const BUJW_ABSENT: &str = "No kennel binary stands at";

/// The remedy an uncommitted repository is named with. Asserted rather than the
/// exit code alone, because what makes the door law a remedy and not a report is
/// that it names what the reader is to do next.
///
/// IT IS THE IMPERATIVE AND NOT THE ACT'S OWN WORD. That word is governed, and a
/// test spelling it a second time would be a second home for it — so what stands
/// here is the rest of the same sentence, which no reading governs and which the
/// refusal cannot carry without having named the act.
const BUJW_REMEDY: &str = "and drive again";

/// Lay the smallest crate the whistle can build and read a position back out of.
///
/// The binary states its seat in the line shape the standing reading matches — a
/// bracketed context, the word, then the position as the line's last field — so
/// the whistle can ask a standing binary where it was struck and compare.
fn zbujw_stand_in_kennel(lure: &bkmtu_Lure, channel: &str) {
    lure.bkmtu_write(
        "Tools/bkmk/Cargo.toml",
        "[package]\n\
         name = \"bkmk\"\n\
         version = \"0.0.1\"\n\
         edition = \"2021\"\n\
         build = \"build.rs\"\n\
         \n\
         [[bin]]\n\
         name = \"bkmx\"\n\
         path = \"src/main.rs\"\n\
         \n\
         [dependencies]\n",
    );

    lure.bkmtu_write(
        "Tools/bkmk/Cargo.lock",
        "# This file is automatically @generated by Cargo.\n\
         # It is not intended for manual editing.\n\
         version = 4\n\
         \n\
         [[package]]\n\
         name = \"bkmk\"\n\
         version = \"0.0.1\"\n",
    );

    lure.bkmtu_write(
        "Tools/bkmk/rust-toolchain.toml",
        &format!("[toolchain]\nchannel = \"{}\"\n", channel),
    );

    // The seat's own election. One root, and the sources stand under it, so a
    // commit touching them moves the position and a commit elsewhere does not —
    // which is the property the staleness hurdle drives.
    lure.bkmtu_write("Tools/bkmk/bkmce_roots.txt", "Tools/bkmk/src\n");

    // The position rides in across the exec boundary exactly as it does for the
    // real kennel, and a build that cannot state it DIES here rather than
    // producing a binary that would answer questions about itself with nothing.
    lure.bkmtu_write(
        "Tools/bkmk/build.rs",
        "fn main() {\n\
         \x20   println!(\"cargo:rerun-if-env-changed=BKMK_SEAT_POSITION\");\n\
         \x20   let seat = std::env::var(\"BKMK_SEAT_POSITION\")\n\
         \x20       .expect(\"BKMK_SEAT_POSITION is unset: no unstamped stand-in may exist\");\n\
         \x20   println!(\"cargo:rustc-env=BKMK_STAND_IN_SEAT={}\", seat);\n\
         \x20   let rustc = std::env::var(\"RUSTC\").unwrap_or_else(|_| \"rustc\".to_string());\n\
         \x20   let out = std::process::Command::new(rustc)\n\
         \x20       .arg(\"--version\")\n\
         \x20       .output()\n\
         \x20       .expect(\"the compiler answers its own version\");\n\
         \x20   println!(\n\
         \x20       \"cargo:rustc-env=BKMK_STAND_IN_COMPILER={}\",\n\
         \x20       String::from_utf8_lossy(&out.stdout).trim()\n\
         \x20   );\n\
         }\n",
    );

    lure.bkmtu_write(
        "Tools/bkmk/src/main.rs",
        "fn main() {\n\
         \x20   println!(\"[INFO] [stand-in] seat {}\", env!(\"BKMK_STAND_IN_SEAT\"));\n\
         \x20   println!(\"[INFO] [stand-in] compiler {}\", env!(\"BKMK_STAND_IN_COMPILER\"));\n\
         }\n",
    );

    // The fence reads VOK's pin from the tools directory the whistle is standing
    // in, so the seat states one.
    lure.bkmtu_write(
        "Tools/vok/rust-toolchain.toml",
        &format!("[toolchain]\nchannel = \"{}\"\n", BUJW_PIN),
    );

    // Cargo's own scratch, kept out of the seat's commits exactly as the tree
    // under test keeps its own out.
    lure.bkmtu_write("Tools/bkmk/.gitignore", "target/\n");
}

/// The coordinator that drives the whistle, in the kindle order its own CLI
/// states.
///
/// The order is the CLI's rather than this hurdle's reading of it: validation and
/// the dispatch regime first, then the fence, then the whistle. The tools
/// directory is made relative between the two halves, because everything before
/// that line wants the dispatch's absolute answer and everything after it wants
/// the whistle's seat-relative one.
fn zbujw_coordinator() -> String {
    let tackroom = std::env::var("BURD_TACKROOM").unwrap_or_default();
    assert!(
        !tackroom.is_empty(),
        "the suite's own dispatch declares no tackroom, so a composed seat has no \
         toolchain store to stand on; drive this suite through its own door"
    );

    format!(
        "#!/bin/bash\n\
         set -euo pipefail\n\
         export BURD_TACKROOM='{tackroom}'\n\
         {sources}\
         buc_context \"bujw\"\n\
         zbuv_kindle\n\
         zburd_kindle\n\
         BURC_TOOLS_DIR=Tools\n\
         zvot_kindle\n\
         zbkmcw_kindle\n\
         bkmcw_kennel\n",
        tackroom = tackroom,
        sources = buas_source(&[
            "buym_yelp.sh",
            "buc_command.sh",
            "buv_validation.sh",
            "burd_regime.sh",
        ]) + &zbujw_kit_source(),
    )
}

/// The kennel's and VOK's own bash, sourced by absolute path out of the tree the
/// suite was built from.
///
/// REACHED BY PATH AND NEVER COPIED, on the same ground the substrate is: what is
/// under test is the whistle standing in this tree, so a copy of it in the seat
/// would be a second thing free to drift from the one the estate runs. The seat
/// carries a stand-in for the CRATE the whistle builds and never for the whistle
/// itself.
fn zbujw_kit_source() -> String {
    let tools = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the substrate stands under a tools directory")
        .to_path_buf();

    let mut said = String::new();
    for module in [
        "bkmk/bkmcp_position.sh",
        "vok/vot_toolchain.sh",
        "bkmk/bkmcw_whistle.sh",
    ] {
        let path = tools.join(module);
        assert!(path.is_file(), "the tree carries no {}", path.display());
        said.push_str(&format!("source '{}'\n", path.display()));
    }
    said
}

/// Every required toolchain the station lacks, refused loud rather than skipped.
///
/// A HURDLE NOT ASKED IS A DECLARED NARROWING, never a discovery made at run time,
/// so a station that cannot answer this question is told so and the hurdle fails
/// rather than passing quietly over the thing it exists to prove.
fn zbujw_toolchains_or_die() {
    let missing = bkmtu_missing_toolchains();
    assert!(
        missing.is_empty(),
        "the station lacks {:?}, and proving a bumped pin changes the compiler needs \
         a second toolchain to bump to",
        missing
    );
}

#[test]
fn bujw_the_whistle_refuses_an_uncommitted_seat_and_names_the_remedy() {
    let lure = bkmtu_Lure::bkmtu_compose("substrate-whistle-dirty");
    zbujw_stand_in_kennel(&lure, BUJW_PIN);
    let tabtarget = lure.bkmtu_substrate_seat(&zbujw_coordinator());

    // AFTER the seat's own commit, so the tree is dirty for this reason alone.
    lure.bkmtu_write("stray.rs", "// planted by the hurdle\n");

    let out = lure.bkmtu_dispatch(&tabtarget, &[]);
    let mut said = String::from_utf8_lossy(&out.stdout).into_owned();
    said.push_str(&String::from_utf8_lossy(&out.stderr));

    assert_ne!(
        out.status.code(),
        Some(0),
        "the whistle stood over an uncommitted seat:\n{}",
        said
    );

    // THE REMEDY, NOT MERELY THE REFUSAL. A door that refused without naming the
    // notch would have handed its reader a stop and no way past it, which is the
    // half of the door law that makes it a discipline rather than an obstacle.
    assert!(
        said.contains(BUJW_REMEDY),
        "the refusal names the remedy:\n{}",
        said
    );

    // AND IT NEVER REACHED THE BUILD. The door law stands ahead of everything, so
    // a refusal that had already compiled would be a refusal in name only.
    assert!(
        !said.contains(BUJW_ABSENT) && !said.contains(BUJW_OUTRAN),
        "the door law refused before the staleness reading was ever taken:\n{}",
        said
    );
}

#[test]
fn bujw_the_whistle_builds_once_then_stands_until_the_seat_outruns_it() {
    zbujw_toolchains_or_die();

    let lure = bkmtu_Lure::bkmtu_compose("substrate-whistle-stale");
    zbujw_stand_in_kennel(&lure, BUJW_PIN);
    let tabtarget = lure.bkmtu_substrate_seat(&zbujw_coordinator());

    // FIRST DRIVE: nothing stands, so it builds.
    let first = lure.bkmtu_dispatch(&tabtarget, &[]);
    let first_said = zbujw_said(&first);
    assert_eq!(
        first.status.code(),
        Some(0),
        "the first drive stands the kennel up:\n{}",
        first_said
    );
    assert!(
        first_said.contains(BUJW_ABSENT),
        "the first drive says no binary stood:\n{}",
        first_said
    );

    // SECOND DRIVE, NOTHING MOVED: it stands. This is the half that a door
    // rebuilding unconditionally would still pass the first assertion of.
    let second = lure.bkmtu_dispatch(&tabtarget, &[]);
    let second_said = zbujw_said(&second);
    assert_eq!(second.status.code(), Some(0), "{}", second_said);
    assert!(
        second_said.contains(BUJW_STOOD),
        "the second drive did not rebuild:\n{}",
        second_said
    );

    // A COMMIT OUTSIDE THE ELECTION MOVES NOTHING. The position is walked over the
    // elected roots and never bare HEAD, so this is the control that says the
    // rebuild below is about the ELECTION rather than about any commit at all.
    lure.bkmtu_write("unelected.txt", "outside every elected root\n");
    lure.bkmtu_commit("a commit the election does not name");

    let third = lure.bkmtu_dispatch(&tabtarget, &[]);
    let third_said = zbujw_said(&third);
    assert!(
        third_said.contains(BUJW_STOOD),
        "a commit outside the election left the kennel standing:\n{}",
        third_said
    );

    // A COMMIT INSIDE IT DOES.
    lure.bkmtu_write("Tools/bkmk/src/moved.rs", "// an elected root moved\n");
    lure.bkmtu_commit("touch an elected root");

    let fourth = lure.bkmtu_dispatch(&tabtarget, &[]);
    let fourth_said = zbujw_said(&fourth);
    assert_eq!(fourth.status.code(), Some(0), "{}", fourth_said);
    assert!(
        fourth_said.contains(BUJW_OUTRAN),
        "a commit touching an elected root outran the standing binary:\n{}",
        fourth_said
    );
}

#[test]
fn bujw_a_bumped_pin_reaches_the_compiler_that_answers() {
    zbujw_toolchains_or_die();

    let lure = bkmtu_Lure::bkmtu_compose("substrate-whistle-bump");
    zbujw_stand_in_kennel(&lure, BUJW_PIN);
    let tabtarget = lure.bkmtu_substrate_seat(&zbujw_coordinator());

    let first = lure.bkmtu_dispatch(&tabtarget, &[]);
    let first_said = zbujw_said(&first);
    assert_eq!(first.status.code(), Some(0), "{}", first_said);
    assert!(
        first_said.contains(&format!("{} {}", BUJW_COMPILER_SAID, BUJW_PIN)),
        "the binary reports the compiler the pin asked for:\n{}",
        first_said
    );

    // THE PIN MOVES, AND THE ELECTION MOVES WITH IT ONLY BECAUSE THE COMMIT DOES.
    // The pin file stands outside the seat's elected roots on purpose here: what
    // forces the rebuild is the elected source beside it, so this hurdle proves
    // the CHANNEL reached the compiler and never that a pin edit alone rebuilds.
    lure.bkmtu_write(
        "Tools/bkmk/rust-toolchain.toml",
        &format!("[toolchain]\nchannel = \"{}\"\n", BUJW_BUMPED),
    );
    lure.bkmtu_write("Tools/bkmk/src/bumped.rs", "// force the rebuild\n");
    lure.bkmtu_commit("bump the pin and move an elected root");

    let second = lure.bkmtu_dispatch(&tabtarget, &[]);
    let second_said = zbujw_said(&second);
    assert_eq!(second.status.code(), Some(0), "{}", second_said);

    // THE ANSWER, NOT THE REQUEST. A stated pin is only a request; what makes the
    // bump provable is the compiler that actually ran reporting its own version,
    // which no reading of the pin file could show.
    assert!(
        second_said.contains(&format!("{} {}", BUJW_COMPILER_SAID, BUJW_BUMPED)),
        "the bumped channel is the compiler that answered:\n{}",
        second_said
    );

    // AND THE FIRST CHANNEL IS GONE, which is what parts a bump from an addition.
    // An assertion that only sought the new channel would pass over a binary
    // reporting both, and over one the whistle never rebuilt at all if the old
    // line happened to survive in the same capture.
    assert!(
        !second_said.contains(&format!("{} {}", BUJW_COMPILER_SAID, BUJW_PIN)),
        "the compiler the seat pinned before the bump no longer answers:\n{}",
        second_said
    );
}

/// Both of a dispatch's streams as one text.
///
/// The dispatch merges the coordinator's streams into its record, which is the
/// substrate's own law, so a hurdle seeking a line the door printed seeks it
/// across both rather than on the stream it was written to.
fn zbujw_said(out: &std::process::Output) -> String {
    let mut said = String::from_utf8_lossy(&out.stdout).into_owned();
    said.push_str(&String::from_utf8_lossy(&out.stderr));
    said
}

// eof
