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

//! Hurdles over the door law's reading (`bkcf_guard`).
//!
//! Every one is a library drive: it calls the kennel in-process and judges the
//! answer, which proves the logic. The door's own refusal — that it prints and
//! exits as the contract says — is a spawned drive and cannot be shown from
//! in-process, so it is owed separately and arrives with the fold.

use crate::bkcf_guard;
use crate::bkcf_guard::BKCF_REMEDY;
use crate::bktu_lure::bktu_Lure;

#[test]
fn bktf_a_composed_lure_reads_clean() {
    let lure = bktu_Lure::bktu_compose("guard-clean");

    let standing = bkcf_guard::bkcf_standing(lure.bktu_root()).expect("the lure is a repository");

    assert!(
        standing.bkcf_clean(),
        "a freshly composed and committed lure carries nothing: {:?}",
        standing.entries
    );
}

#[test]
fn bktf_an_untracked_file_is_uncommitted() {
    let lure = bktu_Lure::bktu_compose("guard-untracked");
    lure.bktu_write("stray.txt", "planted by the hurdle\n");

    let standing = bkcf_guard::bkcf_standing(lure.bktu_root()).expect("the lure is a repository");

    // The untracked arm is the one that matters: a door law admitting untracked
    // files would let a whole unwritten module ride along under a clean verdict.
    assert!(!standing.bkcf_clean(), "an untracked file is uncommitted");
    assert!(
        standing.entries.iter().any(|entry| entry.contains("stray.txt")),
        "the standing names the file it found: {:?}",
        standing.entries
    );
}

#[test]
fn bktf_a_modified_file_is_uncommitted() {
    let lure = bktu_Lure::bktu_compose("guard-modified");
    lure.bktu_write("held.txt", "first\n");
    lure.bktu_commit("bank a file to modify");
    lure.bktu_write("held.txt", "second\n");

    let standing = bkcf_guard::bkcf_standing(lure.bktu_root()).expect("the lure is a repository");

    assert!(!standing.bkcf_clean(), "a modification is uncommitted");
}

#[test]
fn bktf_the_grievance_names_the_remedy_and_every_entry() {
    let lure = bktu_Lure::bktu_compose("guard-grievance");
    lure.bktu_write("one.txt", "a\n");
    lure.bktu_write("two.txt", "b\n");

    let standing = bkcf_guard::bkcf_standing(lure.bktu_root()).expect("the lure is a repository");
    let said = standing.bkcf_grievance(lure.bktu_root());

    // A reader told the tree is dirty and not what to do about it has been handed
    // a search rather than a remedy, so the remedy is asserted, not just the
    // complaint.
    assert!(
        said.contains(BKCF_REMEDY),
        "the grievance names the remedy ({}): {}",
        BKCF_REMEDY,
        said, //
    );
    assert!(said.contains("one.txt"), "the grievance lists what it found: {}", said);
    assert!(said.contains("two.txt"), "the grievance lists what it found: {}", said);
}

#[test]
fn bktf_the_position_moves_when_the_election_is_touched() {
    let lure = bktu_Lure::bktu_compose("guard-position-moves");
    lure.bktu_write("src/held.rs", "// first\n");
    lure.bktu_commit("bank an elected file");

    let election = vec!["src".to_string()];
    let first = bkcf_guard::bkcf_seat_position(lure.bktu_root(), &election)
        .expect("a commit touches the election");
    assert_eq!(first.len(), 40, "a resolved sha, not a symbolic name: {}", first);

    lure.bktu_write("src/held.rs", "// second\n");
    lure.bktu_commit("touch the election again");
    let second = bkcf_guard::bkcf_seat_position(lure.bktu_root(), &election)
        .expect("a commit touches the election");

    assert_ne!(first, second, "a commit touching the election moves the position");
}

#[test]
fn bktf_the_position_stands_still_for_a_commit_outside_the_election() {
    let lure = bktu_Lure::bktu_compose("guard-position-stands");
    lure.bktu_write("src/held.rs", "// held\n");
    lure.bktu_commit("bank an elected file");

    let election = vec!["src".to_string()];
    let before = bkcf_guard::bkcf_seat_position(lure.bktu_root(), &election)
        .expect("a commit touches the election");

    // THE HURDLE THIS KIT WAS WRITTEN WRONG WITHOUT. A bare HEAD reading reports a
    // new position here, so a binary that did not change reads as outrun and is
    // relinked on every unrelated commit in the repository.
    lure.bktu_write("elsewhere.txt", "not source\n");
    lure.bktu_commit("commit outside the election");
    let after = bkcf_guard::bkcf_seat_position(lure.bktu_root(), &election)
        .expect("the earlier elected commit still stands");

    assert_eq!(before, after, "a commit outside the election leaves the position alone");
}

#[test]
fn bktf_an_exclusion_carves_back_out_of_an_elected_root() {
    let lure = bktu_Lure::bktu_compose("guard-position-exclusion");
    lure.bktu_write("src/held.rs", "// held\n");
    lure.bktu_commit("bank an elected file");

    // The grammar is git's own, which is why the election states pathspecs and
    // this module interprets none of them.
    let election = vec!["src".to_string(), ":(exclude)src/skipped.rs".to_string()];
    let before = bkcf_guard::bkcf_seat_position(lure.bktu_root(), &election)
        .expect("a commit touches the election");

    lure.bktu_write("src/skipped.rs", "// carved out\n");
    lure.bktu_commit("touch only the excluded file");
    let after = bkcf_guard::bkcf_seat_position(lure.bktu_root(), &election)
        .expect("the earlier elected commit still stands");

    assert_eq!(before, after, "an excluded path does not move the position");
}

#[test]
fn bktf_an_election_reads_its_roots_and_drops_its_commentary() {
    let lure = bktu_Lure::bktu_compose("guard-election-parse");
    lure.bktu_write(
        "roots.txt",
        "# a comment line\n\nTools/bkk/bk0/src   # trailing commentary\n:(exclude)Tools/bkk/bk0/src/bkt*\n",
    );

    let roots = bkcf_guard::bkcf_election(&lure.bktu_root().join("roots.txt"))
        .expect("the election stands");

    assert_eq!(
        roots,
        vec![
            "Tools/bkk/bk0/src".to_string(),
            ":(exclude)Tools/bkk/bk0/src/bkt*".to_string()
        ]
    );
}

#[test]
fn bktf_an_election_naming_nothing_refuses() {
    let lure = bktu_Lure::bktu_compose("guard-election-empty");
    lure.bktu_write("roots.txt", "# every line a comment\n\n");

    let refusal = bkcf_guard::bkcf_election(&lure.bktu_root().join("roots.txt"))
        .expect_err("an election naming nothing refuses");

    assert!(
        refusal.contains("names no root"),
        "the refusal says the file stood but elected nothing: {}",
        refusal
    );
}

// eof
