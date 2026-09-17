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

//! The whereabouts reader's hurdles.
//!
//! THE GEOGRAPHY IS COMPOSED AS A DIRECTORY, NEVER AS AN ENVIRONMENT. The reader
//! takes the config directory as a value, so a hurdle poses a dispatched seat by
//! standing one up — which is what lets these run beside every other hurdle in
//! the same process. A reader that went to the environment for itself could only
//! be exercised by mutating a variable the whole runner shares, and the arm a
//! hurdle was not in would be decided by whichever hurdle ran last.
//!
//! EVERY REFUSAL CARRIES A CONTROL that reads. A file refuses for exactly one
//! reason at a time here, and the control is the same file with that one reason
//! removed — so an assertion that a malformed file refuses cannot be cleared by
//! a reader that refuses everything.

use super::bkca_whereabouts::{
    bkca_read, bkca_Geography, BKCA_DELIVERED_DIR, BKCA_FILE, BKCA_GEOGRAPHY_DISPATCHED,
    BKCA_GEOGRAPHY_SELF,
};
use super::bktu_lure::bktu_Lure;
use std::path::Path;

/// A whereabouts naming a root that stands, which is the well-formed file every
/// refusal below is a one-line departure from.
fn zbkta_sound(delivered: &Path) -> String {
    format!("{}=\"{}\"\n", BKCA_DELIVERED_DIR, delivered.display())
}

/// The refusal a malformed whereabouts hands back, or a panic naming what it read
/// instead.
///
/// A REFUSAL IS ASSERTED BY ITS WORDS AND NOT ONLY BY ITS FAILING, because every
/// arm below fails for a different reason and a reader that collapsed them would
/// clear a bare `is_err`. The file is named in every one of them, and the field
/// in all but the shape ones, so a reader handed the refusal knows which line to
/// go and repair.
fn zbkta_refusal(lure: &bktu_Lure, text: &str) -> String {
    lure.bktu_moor(BKCA_FILE, text);

    match bkca_read(Some(lure.bktu_moorings())) {
        Err(said) => said,
        Ok(read) => panic!("the whereabouts should have refused and read {:?} instead", read),
    }
}

/// ABSENT MEANS SELF, AND THE TWO ROADS TO ABSENT ARE ONE ANSWER. A seat the
/// substrate handed no config directory at all and a seat whose moorings simply
/// carries no whereabouts mean the identical thing — nobody provisioned a
/// geography — and an ordinary repository reaches the second road every time it
/// is driven.
#[test]
fn bkta_an_unprovisioned_seat_is_self_geography() {
    let lure = bktu_Lure::bktu_compose("whereabouts-absent");

    assert_eq!(
        bkca_read(None).expect("no config directory is no geography, not a fault"),
        bkca_Geography::Undispatched,
        "a seat handed no config directory works the tree it entered"
    );

    assert_eq!(
        bkca_read(Some(lure.bktu_moorings())).expect("a moorings with no whereabouts reads"),
        bkca_Geography::Undispatched,
        "a moorings that carries no whereabouts is an ordinary repository's moorings"
    );

    // THE CONTROL IS THE SAME DIRECTORY WITH THE FILE IN IT. Without it, a reader
    // that answered Undispatched for everything would clear both assertions
    // above and this hurdle would be proving that the reader does nothing.
    lure.bktu_moor(BKCA_FILE, &zbkta_sound(lure.bktu_root()));

    assert_eq!(
        bkca_read(Some(lure.bktu_moorings())).expect("a sound whereabouts reads"),
        bkca_Geography::Dispatched(lure.bktu_root().to_path_buf()),
        "the control: the same directory carrying the file reads as dispatched, so the two \
         answers above are the absence and not a constant"
    );
}

/// A PRESENT WHEREABOUTS NAMES THE DELIVERED ROOT, and the stated form carries it.
///
/// THE STATED FORM IS WHAT THE PROCLAMATION AND THE ELECTION LINE BOTH SPELL, so
/// it is asserted here rather than at either seat: it is a catena whose first
/// element is the word and whose second is the root, which is what lets a
/// consumer source the proclamation and read the root back out of one field.
#[test]
fn bkta_a_present_whereabouts_names_its_delivered_root() {
    let lure = bktu_Lure::bktu_compose("whereabouts-present");
    lure.bktu_moor(BKCA_FILE, &zbkta_sound(lure.bktu_root()));

    let read = bkca_read(Some(lure.bktu_moorings())).expect("a sound whereabouts reads");

    assert_eq!(
        read.bkca_delivered(),
        Some(lure.bktu_root()),
        "the delivered root is what the file named"
    );

    assert_eq!(
        read.bkca_stated(),
        format!("{} {}", BKCA_GEOGRAPHY_DISPATCHED, lure.bktu_root().display()),
        "a dispatched geography states its word and its root as one catena"
    );

    assert_eq!(
        bkca_Geography::Undispatched.bkca_stated(),
        BKCA_GEOGRAPHY_SELF,
        "the control: self-geography states one element and names no root, so the two forms are \
         told apart by a consumer reading the first element"
    );
}

/// A FIELD OUTSIDE THE ROSTER REFUSES BY NAME, and it refuses AHEAD of the
/// missing-field reading.
///
/// THE ORDER IS THE FINDING RATHER THAN THE FAULT. A misspelled field carries
/// both faults at once, and a reader told its roster field is missing goes
/// looking for a line that is right there in front of them under another name.
#[test]
fn bkta_a_field_outside_the_roster_refuses_by_name() {
    let lure = bktu_Lure::bktu_compose("whereabouts-foreign");

    let said = zbkta_refusal(
        &lure,
        &format!("BKRW_DELIVERD_DIR=\"{}\"\n", lure.bktu_root().display()),
    );

    assert!(
        said.contains("BKRW_DELIVERD_DIR"),
        "the refusal names the field that stands outside the roster: {}",
        said
    );
    assert!(
        said.contains(BKCA_FILE),
        "the refusal names the file to go and repair: {}",
        said
    );

    // THE CONTROL IS THE MISSING-FIELD READING, which the same file would also
    // earn: the assertion above is about WHICH of the two faults is reported, so
    // a reader reaching the second one would clear a bare `is_err` and mislead.
    assert!(
        !said.contains("does not declare"),
        "the misspelling is reported and not the absence it also causes: {}",
        said
    );
}

/// A FILE THAT STANDS AND DECLARES NOTHING REFUSES. It is the one shape a
/// fallback would swallow most quietly: an empty file parses cleanly under the
/// catena law and would read as self-geography under any reader that took an
/// absence for an answer.
#[test]
fn bkta_a_missing_field_refuses_by_name() {
    let lure = bktu_Lure::bktu_compose("whereabouts-missing");

    let said = zbkta_refusal(&lure, "# a seat the stile began and did not finish\n");

    assert!(
        said.contains(BKCA_DELIVERED_DIR),
        "the refusal names the field the file owes: {}",
        said
    );
    assert!(
        said.contains(BKCA_FILE),
        "the refusal names the file to go and repair: {}",
        said
    );

    // THE CONTROL IS THE SAME COMMENT WITH THE FIELD BENEATH IT, so the refusal
    // above is the absence and not the comment line.
    lure.bktu_moor(
        BKCA_FILE,
        &format!(
            "# a seat the stile began and did not finish\n{}",
            zbkta_sound(lure.bktu_root())
        ),
    );

    assert!(
        bkca_read(Some(lure.bktu_moorings())).is_ok(),
        "the control: the same comment above a declared field reads"
    );
}

/// A RELATIVE ROOT REFUSES, AND SO DOES AN EMPTY ONE. The seat that reads a
/// whereabouts stands in a different tree than the one that wrote it, so a
/// relative root would name whichever directory a door happened to be standing
/// in — which is precisely the coupling to the caller's working directory the
/// whole dispatch layer exists to remove.
#[test]
fn bkta_a_relative_root_refuses() {
    let lure = bktu_Lure::bktu_compose("whereabouts-relative");

    for spelling in ["..", "residence", ""] {
        let said = zbkta_refusal(&lure, &format!("{}=\"{}\"\n", BKCA_DELIVERED_DIR, spelling));

        assert!(
            said.contains(BKCA_DELIVERED_DIR) && said.contains(BKCA_FILE),
            "the refusal names the field and the file for '{}': {}",
            spelling,
            said
        );
        assert!(
            said.contains("absolute"),
            "the refusal says what is wrong with '{}' rather than only that something is: {}",
            spelling,
            said
        );
    }

    // THE CONTROL IS THE SAME FILE SPELLED ABSOLUTELY, over a root that stands.
    lure.bktu_moor(BKCA_FILE, &zbkta_sound(lure.bktu_root()));

    assert!(
        bkca_read(Some(lure.bktu_moorings())).is_ok(),
        "the control: an absolute root over a directory that stands reads"
    );
}

/// A ROOT THAT DOES NOT STAND REFUSES. It is the fault a fallback would hide
/// most expensively: the seat looks ordinary, every election runs its own build,
/// and the delivered root the stile went to the trouble of standing up is simply
/// never consulted by anybody.
#[test]
fn bkta_a_root_that_does_not_stand_refuses() {
    let lure = bktu_Lure::bktu_compose("whereabouts-nowhere");
    let nowhere = lure.bktu_temp().join("no-delivered-root-stands-here");

    let said = zbkta_refusal(&lure, &zbkta_sound(&nowhere));

    assert!(
        said.contains(&nowhere.display().to_string()),
        "the refusal names the root nobody provisioned: {}",
        said
    );
    assert!(
        said.contains(BKCA_FILE),
        "the refusal names the file to go and repair: {}",
        said
    );

    // THE CONTROL IS A FILE, NOT A DIRECTORY, at a path that does stand — the
    // reading is that a DIRECTORY stands there, and a check that only asked
    // whether something existed would clear this.
    let file = lure.bktu_moor("a-file-is-not-a-root", "");
    let said = zbkta_refusal(&lure, &zbkta_sound(&file));

    assert!(
        said.contains(&file.display().to_string()),
        "a path that stands and is no directory refuses too: {}",
        said
    );

    // THE CONTROL IS THE ROOT THAT DOES STAND.
    lure.bktu_moor(BKCA_FILE, &zbkta_sound(lure.bktu_root()));

    assert!(
        bkca_read(Some(lure.bktu_moorings())).is_ok(),
        "the control: a root that stands reads"
    );
}

// eof
