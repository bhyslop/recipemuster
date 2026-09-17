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

//! Hurdles over the leash (`bkcl_leash`) — the chokepoint's three disciplines.

use crate::bkcf_guard;
use crate::bkcl_leash;
use crate::bktu_lure::{bktu_Lure, bktu_missing_toolchains, BKTU_REQUIRED_TOOLCHAINS};

#[test]
fn bktl_the_pin_is_read_from_the_repositorys_own_file() {
    let lure = bktu_Lure::bktu_compose("leash-pin");

    let channel = bkcl_leash::bkcl_pin(lure.bktu_root(), lure.bktu_root()).expect("the lure carries a pin");

    assert_eq!(channel, "1.90.0");
}

#[test]
fn bktl_a_bumped_pin_reads_as_the_bumped_channel() {
    let lure = bktu_Lure::bktu_compose("leash-bump");
    lure.bktu_write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.89.0\"\n");
    lure.bktu_commit("bump the pin");

    let channel = bkcl_leash::bkcl_pin(lure.bktu_root(), lure.bktu_root()).expect("the lure carries a pin");

    // The pin is READ and then STATED at the invocation. This is the reading
    // half; that the stated channel is the one that runs is proven by the
    // binary's own compiler report, which no in-process call can show.
    assert_eq!(channel, "1.89.0");
}

#[test]
fn bktl_a_repository_with_no_pin_refuses_and_says_why() {
    let lure = bktu_Lure::bktu_compose("leash-unpinned");
    std::fs::remove_file(lure.bktu_root().join("rust-toolchain.toml")).expect("the pin stood");

    let refusal = bkcl_leash::bkcl_pin(lure.bktu_root(), lure.bktu_root()).expect_err("an unpinned repository refuses");

    assert!(
        refusal.contains("no pin governs"),
        "the refusal names what is missing: {}",
        refusal
    );
}

#[test]
fn bktl_a_pin_naming_no_channel_refuses() {
    let lure = bktu_Lure::bktu_compose("leash-channelless");
    lure.bktu_write("rust-toolchain.toml", "[toolchain]\nprofile = \"minimal\"\n");

    let refusal = bkcl_leash::bkcl_pin(lure.bktu_root(), lure.bktu_root()).expect_err("a channelless pin refuses");

    assert!(
        refusal.contains("names no channel"),
        "the refusal says the file stood but said nothing: {}",
        refusal
    );
}

#[test]
fn bktl_the_fence_is_in_force_for_this_run() {
    // The suite is launched through the substrate, which stands on the fence
    // (Tools/bkk/bk0/bkcb_tackroom.sh) before anything reaches cargo. So this asserts
    // the inherited redirect rather than performing one: the environment is
    // process-global and a hurdle that mutated it would race every sibling.
    let root = bkcl_leash::bkcl_fenced()
        .expect("the suite runs behind the fence, so both homes stand inside the tackroom");

    assert!(root.is_absolute(), "the tackroom resolves to a real place: {}", root.display());
}

#[test]
fn bktl_the_station_holds_the_toolchains_the_hurdles_require() {
    // A hurdle's demand, never the kennel's: at runtime the kennel binds whatever
    // pin a collar names and asks nothing more of the station. Proving that a
    // bumped pin changes the compiler is what needs a second toolchain present,
    // and a station without one cannot demonstrate the property at all — so the
    // absence is announced as an absence rather than passing quietly.
    let missing = bktu_missing_toolchains();

    assert!(
        missing.is_empty(),
        "the station is missing {:?} of the required toolchains {:?} — the converge is \
         `RUSTUP_HOME=<tackroom>/rustup rustup toolchain install <channel> --profile minimal`",
        missing,
        BKTU_REQUIRED_TOOLCHAINS
    );
}

#[test]
fn bktl_a_crate_is_answered_by_the_nearest_pin_above_it() {
    let lure = bktu_Lure::bktu_compose("leash-nearest");
    lure.bktu_write("kit/Cargo.toml", "[package]\nname = \"kit\"\n");
    lure.bktu_commit("a crate under the root, pinned by nobody but the root");

    let crate_dir = lure.bktu_root().join("kit");
    let channel = bkcl_leash::bkcl_pin(lure.bktu_root(), &crate_dir)
        .expect("the root's pin answers for a crate that states none");

    assert_eq!(channel, "1.90.0");
}

#[test]
fn bktl_a_crates_own_pin_beats_the_one_above_it() {
    let lure = bktu_Lure::bktu_compose("leash-nearer");
    lure.bktu_write("kit/Cargo.toml", "[package]\nname = \"kit\"\n");
    lure.bktu_write("kit/rust-toolchain.toml", "[toolchain]\nchannel = \"1.89.0\"\n");
    lure.bktu_commit("a crate that states its own channel");

    let crate_dir = lure.bktu_root().join("kit");
    let channel = bkcl_leash::bkcl_pin(lure.bktu_root(), &crate_dir)
        .expect("the crate's own pin stands");

    // NEAREST WINS, and this is the whole reason the walk is anchored at the
    // crate: a kit that bumps its own pin is answered by the bump rather than by
    // whatever the root happens to say, which is the founding's proven property
    // held one level up.
    assert_eq!(channel, "1.89.0");
}

#[test]
fn bktl_a_crate_outside_the_repository_refuses_rather_than_walking_out() {
    let holder = bktu_Lure::bktu_compose("leash-bound-holder");
    let stranger = bktu_Lure::bktu_compose("leash-bound-stranger");

    // The stranger carries a pin of its own and stands nowhere under the holder.
    // A walk that watched only for reaching its bound would never meet it, run to
    // the filesystem root, and answer with whichever pin it passed first — a
    // build pinned by the shape of the ground around the tree.
    let refusal = bkcl_leash::bkcl_pin(holder.bktu_root(), stranger.bktu_root())
        .expect_err("a crate outside the repository is refused");

    assert!(
        refusal.contains("stands outside the repository"),
        "the refusal names the bound rather than reporting some stranger's channel: {}",
        refusal
    );
}

#[test]
fn bktl_a_channel_the_tackroom_lacks_refuses_rather_than_downloading() {
    let lure = bktu_Lure::bktu_compose("leash-absent-channel");
    lure.bktu_write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.1.1\"\n");
    lure.bktu_commit("pin a channel no station holds");

    // Driven for real: the refusal stands AHEAD of the spawn, so no cargo runs
    // and nothing is fetched. A channel this old is the point — rustup would
    // happily go and get it, over minutes, saying almost nothing.
    let refusal = bkcl_leash::bkcl_drive(
        lure.bktu_root(),
        &lure.bktu_root().join("Cargo.toml"),
        "metadata",
        Vec::<String>::new(),
        &[],
    )
    .expect_err("a channel the tackroom does not hold is refused");

    assert!(
        refusal.contains("never downloads unasked"),
        "the refusal says why it stopped rather than reporting a failed build: {}",
        refusal
    );
    assert!(
        refusal.contains("rustup toolchain install 1.1.1"),
        "the refusal carries the converge that supplies what is missing: {}",
        refusal
    );
}

#[test]
fn bktl_a_recalled_run_hands_back_the_childs_own_words() {
    let lure = bktu_Lure::bktu_compose("leash-recall");
    lure.bktu_write(
        "Cargo.toml",
        "[package]\nname = \"recalled\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\
         license = \"Apache-2.0\"\n",
    );
    // Cargo refuses a manifest naming no target at all, so the crate carries a
    // source file. That is a fact about cargo's requirements rather than about
    // this reading.
    lure.bktu_write("src/lib.rs", "pub fn planted() {}\n");
    lure.bktu_commit("a crate for the recall to ask about");

    let recall = bkcl_leash::bkcl_recall(
        lure.bktu_root(),
        &lure.bktu_root().join("Cargo.toml"),
        "metadata",
        ["--no-deps", "--format-version", "1", "--offline"],
        &[],
    )
    .expect("the lure is pinned and the tackroom holds its channel");

    assert!(
        recall.run.bkcl_landed(),
        "cargo refused the lure, saying: {}",
        recall.grievance
    );

    // THE ANSWER ARRIVES AS DATA, which is the whole difference between the two
    // faces: a driven run would have put this on a terminal, where the caller
    // that asked for it could not read it.
    let said = String::from_utf8(recall.said).expect("cargo's metadata is text");
    assert!(
        said.contains("recalled"),
        "the answer describes the crate that was asked about: {}",
        said
    );
    assert!(
        said.contains("Apache-2.0"),
        "the answer carries the field the manifest declares: {}",
        said
    );

    // THE SPELLING IS NOT IN THE ANSWER. The kennel's own account of what it ran
    // goes to its record and never into the child's stdout, which is what lets a
    // caller parse that stream whole rather than hunting for where it begins.
    assert!(
        recall.run.spelling.contains("+1.90.0"),
        "the pin is stated at the invocation: {}",
        recall.run.spelling
    );
    assert!(
        recall.run.spelling.contains("--locked"),
        "the lock rides the invocation: {}",
        recall.run.spelling
    );
    assert!(
        !said.contains("--locked"),
        "nothing of the leash's own reached the child's stdout: {}",
        said
    );
}

#[test]
fn bktl_a_recalled_run_stands_on_the_same_discipline_as_a_driven_one() {
    let lure = bktu_Lure::bktu_compose("leash-recall-discipline");
    lure.bktu_write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.1.1\"\n");
    lure.bktu_commit("pin a channel no station holds");

    // The refusal the driven face takes, taken here at the same place: ahead of
    // the spawn, so nothing is fetched. A face that skipped a discipline because
    // its caller wanted an answer rather than an exit would be exactly the
    // half-applied leash the chokepoint exists to make unspellable, and the two
    // would drift the moment one of them was edited.
    let refusal = bkcl_leash::bkcl_recall(
        lure.bktu_root(),
        &lure.bktu_root().join("Cargo.toml"),
        "metadata",
        Vec::<String>::new(),
        &[],
    )
    .expect_err("a channel the tackroom does not hold is refused");

    assert!(
        refusal.contains("never downloads unasked"),
        "the refusal says why it stopped rather than reporting a failed read: {}",
        refusal
    );
}

/// THE FLOOR IS A COMPARISON AND NOT A STRING MATCH, read from both sides: a
/// version above it clears, one below refuses, and the floor itself clears.
///
/// The below-the-floor arm is why this is hurdled at all. Every station this
/// kennel runs on holds a git far above the floor, so nothing driven through a
/// door here can ever reach that arm — and the one it cannot reach is the one
/// whose job is to refuse.
#[test]
fn bktl_a_version_is_read_against_the_floor_from_both_sides() {
    for held in [bkcl_leash::BKCL_GIT_FLOOR, "1.9.0", "2.43.0", "10.0.0"] {
        assert!(
            bkcl_leash::bkcl_at_or_above(held, bkcl_leash::BKCL_GIT_FLOOR),
            "{} stands at or above {}",
            held,
            bkcl_leash::BKCL_GIT_FLOOR
        );
    }

    for held in ["1.8.4", "1.7.99", "0.9.9"] {
        assert!(
            !bkcl_leash::bkcl_at_or_above(held, bkcl_leash::BKCL_GIT_FLOOR),
            "{} stands below {}",
            held,
            bkcl_leash::BKCL_GIT_FLOOR
        );
    }
}

/// A STATION'S OWN DECORATION IS TOLERATED, because the floor is a fact about
/// the version and never about who packaged it. Git reports itself with the
/// packager's marks attached on more than one platform this estate runs on, and
/// a comparison that refused those would be refusing the packaging.
#[test]
fn bktl_a_decorated_version_compares_on_the_numbers_it_carries() {
    for decorated in ["2.43.0.windows.1", "2.39.5", "2.43.0-rc1"] {
        assert!(
            bkcl_leash::bkcl_at_or_above(decorated, bkcl_leash::BKCL_GIT_FLOOR),
            "{} carries a version above the floor whatever else it carries",
            decorated
        );
    }

    // THE CONTROL: tolerance is not indifference. A decorated version BELOW the
    // floor still refuses, so the assertions above are about the numbers rather
    // than about a comparison that has stopped reading.
    assert!(
        !bkcl_leash::bkcl_at_or_above("1.7.0.windows.1", bkcl_leash::BKCL_GIT_FLOOR),
        "decoration does not lift a version over the floor"
    );
}

/// THE STATION THIS SUITE IS RUNNING ON HOLDS THE PROGRAMS THE KENNEL SPAWNS
/// AND DID NOT BUILD AND DOES NOT PIN, which is the same posture as the
/// required-toolchain hurdle above and is reported the same way: what fails here
/// is the station, and the refusal names what to install.
///
/// GIT IS THE WHOLE ROSTER NOW, and the narrowing is what the kibble regime did
/// rather than a gap. Nextest once stood beside it here, asked what version it
/// was and judged against a constant in the leash; it is now declared by a
/// kibble and reached at its own residence, so the question "does this station
/// hold it" has no answer to give — a station holds no kibble, a TACKROOM does,
/// and the reading is a path test the drive hurdles take
/// (`tests/bktd_drive.rs`). What remains here are the station tools proper,
/// which carry a floor rather than a pin because the station's own packaging
/// supplies them.
#[test]
fn bktl_the_station_holds_the_programs_the_kennel_spawns() {
    bkcl_leash::bkcl_git_held().expect("the station holds a git at or above the floor");
}

/// RUSTUP JOINS THE ROSTER, checked through the fence itself: `bkcl_fenced`
/// is what every composition already stands on, so a station whose rustup
/// stands below the floor is met at the fence rather than partway into a
/// build.
#[test]
fn bktl_the_station_holds_a_rustup_the_kennel_trusts() {
    bkcl_leash::bkcl_rustup_held().expect("the station holds a rustup at or above the floor");
}

/// THE FLOOR IS A COMPARISON AND NOT A STRING MATCH, read from both sides
/// over POSED REPORT STRINGS: posing a rustup on the PATH is not a thing the
/// lure affords, so the comparison is driven directly rather than through a
/// spawn.
#[test]
fn bktl_a_rustup_report_is_judged_against_the_floor_from_both_sides() {
    for said in [
        "rustup 1.29.1 (d95a37b6a 2026-08-13)",
        "rustup 1.30.0 (abcdef0123 2026-09-01)",
        "rustup 2.0.0 (0000000000 2027-01-01)",
    ] {
        bkcl_leash::bkcl_rustup_judged(said, bkcl_leash::BKCL_RUSTUP_FLOOR)
            .unwrap_or_else(|err| panic!("{} stands at or above the floor: {}", said, err));
    }

    for said in [
        "rustup 1.29.0 (d95a37b6a 2026-08-13)",
        "rustup 1.9.0 (aaaaaaaaaa 2020-01-01)",
        "rustup 0.9.9 (aaaaaaaaaa 2019-01-01)",
    ] {
        let refusal = bkcl_leash::bkcl_rustup_judged(said, bkcl_leash::BKCL_RUSTUP_FLOOR)
            .unwrap_err();
        assert!(
            refusal.contains("rustup") && refusal.contains(bkcl_leash::BKCL_RUSTUP_FLOOR),
            "the refusal names rustup and the floor: {}",
            refusal
        );
    }
}

/// A SHAPE THE READER DOES NOT RECOGNIZE REFUSES, NEVER PASSES — the same
/// posture `bkcl_host` takes reading rustc's `host: ` line.
#[test]
fn bktl_an_unrecognized_rustup_report_refuses_naming_rustup() {
    for said in ["", "not rustup at all", "rustc 1.90.0 (abc123 2026-01-01)"] {
        let refusal = bkcl_leash::bkcl_rustup_judged(said, bkcl_leash::BKCL_RUSTUP_FLOOR)
            .expect_err("a shape rustup does not give is not one this reading parses a version out of");
        assert!(
            refusal.contains("rustup"),
            "the refusal names rustup rather than guessing: {}",
            refusal
        );
    }
}

/// The composed seat reading, laid down as one lure crate the two hurdles below
/// vary in exactly one respect: whether the tree holds the kennel's election.
///
/// THE BUILD SCRIPT WRITES WHAT IT SAW rather than dying on it, because both
/// arms are readings rather than one reading and one refusal: a crate that died
/// when the variable was absent could prove the stated arm and could say nothing
/// at all about the removed one. What makes the stated arm a demand is the
/// assertion, which requires a position and never accepts the word.
fn zbktl_seat_crate(lure: &bktu_Lure) {
    lure.bktu_write(
        "Cargo.toml",
        "[package]\nname = \"seated\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\
         license = \"Apache-2.0\"\nbuild = \"build.rs\"\n",
    );
    lure.bktu_write(
        "Cargo.lock",
        "# This file is automatically @generated by Cargo.\n\
         # It is not intended for manual editing.\n\
         version = 4\n\n\
         [[package]]\nname = \"seated\"\nversion = \"0.0.1\"\n",
    );
    lure.bktu_write("src/lib.rs", "pub fn seated() {}\n");

    // A build script's working directory is its package root, so the answer
    // lands beside the manifest the drive named and the hurdle reads it back
    // from a path it composed rather than one it searched for.
    lure.bktu_write(
        "build.rs",
        "fn main() {\n\
         \x20   println!(\"cargo:rerun-if-env-changed=BKK_SEAT_POSITION\");\n\
         \x20   let said = std::env::var(\"BKK_SEAT_POSITION\")\n\
         \x20       .unwrap_or_else(|_| \"unset\".to_string());\n\
         \x20   std::fs::write(\"seat-said.txt\", said)\n\
         \x20       .expect(\"the build script writes what it was handed\");\n\
         }\n",
    );
}

/// What the child's build script was handed, read back off the lure.
fn zbktl_seat_said(lure: &bktu_Lure) -> String {
    std::fs::read_to_string(lure.bktu_root().join("seat-said.txt"))
        .expect("the build script ran and wrote what it saw")
}

/// A TREE THAT HOLDS THE KENNEL IS HANDED THE POSITION, AND NO CALLER SPELLS IT.
///
/// This is the whole of what the composition bought. Before it, a door that
/// wanted to build a crate linking the kennel had to compose the reading itself
/// — the election read, the position walked, the variable exported — and a door
/// that did not know the demand existed could not build such a crate at all.
/// The engine was that door, which is why the course record's roster stood
/// spelled twice.
///
/// THE STATED ARM IS PROVEN AGAINST THE LURE'S OWN WALK rather than against the
/// word "set": the composition must hand the child a position measured over the
/// tree the child is being built in, and an ambient one inherited from whatever
/// door launched this suite would satisfy a weaker reading perfectly.
#[test]
fn bktl_a_tree_holding_the_kennels_election_is_handed_the_position() {
    let lure = bktu_Lure::bktu_compose("leash-seat-stated");
    zbktl_seat_crate(&lure);

    // The election names a root the lure actually carries: a position is walked
    // over the election with git's own pathspec grammar, so an election naming
    // nothing in this tree would refuse rather than answer.
    lure.bktu_write(bkcl_leash::BKCL_KENNEL_ELECTION, "src\n");
    lure.bktu_commit("a tree holding the kennel's election and a crate that reads the position");

    let run = bkcl_leash::bkcl_recall(
        lure.bktu_root(),
        &lure.bktu_root().join("Cargo.toml"),
        "build",
        ["--offline"],
        &[],
    )
    .expect("the lure is pinned and the tackroom holds its channel");

    assert!(
        run.run.bkcl_landed(),
        "cargo refused the lure, saying: {}",
        run.grievance
    );

    let election = bkcf_guard::bkcf_election(&lure.bktu_root().join(bkcl_leash::BKCL_KENNEL_ELECTION))
        .expect("the election the lure just committed reads");
    let walked = bkcf_guard::bkcf_seat_position(lure.bktu_root(), &election)
        .expect("the lure's own line touches its elected root");

    assert_eq!(
        zbktl_seat_said(&lure),
        walked,
        "the child was handed the position walked over the tree it was built in"
    );
}

/// A TREE THAT HOLDS NO KENNEL IS HANDED NOTHING, WHICH IS THE ARM THAT BITES.
///
/// Every door that spawns this composition was itself launched by a door that
/// may hold the variable — the kennelman that runs this very hurdle exports it —
/// so leaving an absent election to mean "inherit" would hand a tree that has
/// never heard of the kennel a position measured over a repository it does not
/// carry, and let it stamp an artifact with one. The removal is what makes the
/// absence an answer rather than a gap.
#[test]
fn bktl_a_tree_holding_no_election_is_handed_no_position() {
    let lure = bktu_Lure::bktu_compose("leash-seat-removed");
    zbktl_seat_crate(&lure);
    lure.bktu_commit("a tree with no kennel in it at all");

    assert!(
        !lure.bktu_root().join(bkcl_leash::BKCL_KENNEL_ELECTION).exists(),
        "the control's whole variance is that this tree holds no election"
    );

    let run = bkcl_leash::bkcl_recall(
        lure.bktu_root(),
        &lure.bktu_root().join("Cargo.toml"),
        "build",
        ["--offline"],
        &[],
    )
    .expect("the lure is pinned and the tackroom holds its channel");

    assert!(
        run.run.bkcl_landed(),
        "cargo refused the lure, saying: {}",
        run.grievance
    );

    assert_eq!(
        zbktl_seat_said(&lure),
        "unset",
        "the child of a kennel-less tree carries no position, inherited or otherwise"
    );
}

// eof
