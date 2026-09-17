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

//! The course record's hurdles — the roster read both ways.
//!
//! NOTHING HERE TOUCHES THE ENVIRONMENT OR THE DISK. The render and the admit
//! are pure, so they are proven here; the dual write reads two variables of the
//! process it stands in, and a hurdle that set one would be setting it for every
//! other hurdle running beside it. Those proofs are the spawned drive's, where a
//! child's environment is composed rather than mutated.
//!
//! THE ROUND TRIP IS THE LOAD-BEARING HURDLE. The roster is the one definition
//! the writer and the harvester share, and what makes that more than a claim is
//! that the reader below is the one the engine will use — so a field the render
//! spells and the admit does not is a failure here rather than a surprise at the
//! harvest.

use crate::bkcc_record::*;
use crate::bkg_breviary::bkg_burc_loosebox_root_dir_base;

/// A whole green course, as the doors compose one.
fn zbktc_whole() -> bkcc_Course {
    bkcc_Course {
        collar: "suite-alfa".to_string(),
        narrowing: None,
        driven: 137,
        passed: 137,
        held: 137,
        wall: Some(std::time::Duration::from_millis(42_500)),
        position: "835f68bf91d6e83a67b8378ac7fc2a8f0c4bdf3f".to_string(),
    }
}

/// The same course narrowed by two patterns.
fn zbktc_narrowed() -> bkcc_Course {
    bkcc_Course {
        collar: "suite-alfa".to_string(),
        narrowing: Some(vec!["bktc_render".to_string(), "bktc_admit".to_string()]),
        driven: 2,
        passed: 2,
        held: 137,
        wall: None,
        position: "835f68bf91d6e83a67b8378ac7fc2a8f0c4bdf3f".to_string(),
    }
}

#[test]
fn bktc_a_whole_course_renders_every_field_of_the_roster() {
    let said = zbktc_whole().bkcc_body();

    for owed in [
        "BKRD_COLLAR=\"suite-alfa\"",
        "BKRD_NARROWING=\"bknre_whole\"",
        "BKRD_DRIVEN=\"137\"",
        "BKRD_PASSED=\"137\"",
        "BKRD_HELD=\"137\"",
        "BKRD_WALL=\"42500\"",
        "BKRD_POSITION=\"835f68bf91d6e83a67b8378ac7fc2a8f0c4bdf3f\"",
    ] {
        assert!(said.contains(owed), "the record carries '{}': {}", owed, said);
    }

    // SEVEN LINES AND NO EIGHTH. A record that grew a field the roster does not
    // name would still carry every string above, so the count is what says the
    // render and the roster are the same set rather than one containing the
    // other.
    assert_eq!(said.lines().count(), 7, "the roster is seven fields: {}", said);
}

#[test]
fn bktc_a_narrowed_course_carries_its_patterns_and_no_time() {
    let said = zbktc_narrowed().bkcc_body();

    assert!(
        said.contains("BKRD_NARROWING=\"bktc_render bktc_admit\""),
        "the patterns ride the narrowing as a catena: {}",
        said
    );

    // WALL TIME IS RECORDED ONLY WHERE THE COURSE RAN WHOLE. A narrowed course's
    // time is not the course's, and one such number poisons the series it would
    // be read against — so the field carries the token that says why there is no
    // number, never a number of its own and never nothing at all.
    assert!(
        said.contains("BKRD_WALL=\"bknre_narrowed\""),
        "a narrowed course takes no time: {}",
        said
    );
    assert!(
        !said.contains("BKRD_WALL=\"0\""),
        "and does not record a zero, which a reader would take for a fast course: {}",
        said
    );
}

#[test]
fn bktc_a_record_survives_the_round_trip_whichever_shape_it_took() {
    for owed in [zbktc_whole(), zbktc_narrowed()] {
        let said = owed.bkcc_body();

        let read = bkcc_admit(&said, "the hurdle's own record")
            .unwrap_or_else(|err| panic!("the writer's output is readable: {}", err));

        assert_eq!(read, owed, "what was written is what is read back: {}", said);
    }
}

#[test]
fn bktc_the_leaf_names_the_collar_and_the_regime() {
    assert_eq!(
        zbktc_whole().bkcc_leaf(),
        "suite-alfa.bkrd",
        "the row is the collar and the type is the extension"
    );
}

#[test]
fn bktc_a_record_that_disagrees_with_itself_refuses() {
    // The two halves of one fact, each spelled against the other. Neither can be
    // trusted to be the honest one, so the reader refuses rather than preferring
    // a side.
    for (said, what) in [
        (
            "BKRD_COLLAR=\"suite-alfa\"\nBKRD_NARROWING=\"bknre_whole\"\nBKRD_DRIVEN=\"1\"\n\
             BKRD_PASSED=\"1\"\nBKRD_HELD=\"1\"\nBKRD_WALL=\"bknre_narrowed\"\n\
             BKRD_POSITION=\"abc\"\n",
            "a whole course claiming it was narrowed",
        ),
        (
            "BKRD_COLLAR=\"suite-alfa\"\nBKRD_NARROWING=\"one\"\nBKRD_DRIVEN=\"1\"\n\
             BKRD_PASSED=\"1\"\nBKRD_HELD=\"9\"\nBKRD_WALL=\"400\"\n\
             BKRD_POSITION=\"abc\"\n",
            "a narrowed course carrying a time",
        ),
    ] {
        let err = bkcc_admit(said, "the hurdle's own record")
            .expect_err(&format!("{} refuses", what));

        assert!(
            err.contains("disagrees with itself"),
            "the refusal says which rule was broken ({}): {}",
            what,
            err
        );
    }
}

#[test]
fn bktc_a_field_outside_the_roster_refuses_by_name() {
    let said = format!("{}{}", zbktc_whole().bkcc_body(), "BKRD_STATION=\"beast\"\n");

    let err = bkcc_admit(&said, "the hurdle's own record")
        .expect_err("a record carrying a field the roster does not name refuses");

    // THE RECORD CARRIES NO STATION, and this hurdle is where that cinch is
    // enforced rather than merely stated: the station is the dispatcher's fact
    // and the engine stamps it at the harvest, so a writer that put one here has
    // written a record this reader cannot vouch for.
    assert!(
        err.contains("BKRD_STATION"),
        "the refusal names the field it did not expect: {}",
        err
    );
}

#[test]
fn bktc_a_missing_field_refuses_by_name() {
    let whole = zbktc_whole().bkcc_body();

    for owed in [
        "BKRD_COLLAR",
        "BKRD_NARROWING",
        "BKRD_DRIVEN",
        "BKRD_PASSED",
        "BKRD_HELD",
        "BKRD_WALL",
        "BKRD_POSITION",
    ] {
        let without: String = whole
            .lines()
            .filter(|line| !line.starts_with(owed))
            .map(|line| format!("{}\n", line))
            .collect();

        let err = match bkcc_admit(&without, "the hurdle's own record") {
            Err(err) => err,
            Ok(admitted) => panic!(
                "a record missing {} was admitted rather than refused: {:?}",
                owed, admitted
            ),
        };

        assert!(
            err.contains(owed),
            "a record missing {} refuses naming it: {}",
            owed,
            err
        );
    }
}

#[test]
fn bktc_a_count_that_is_no_count_refuses() {
    let said = zbktc_whole().bkcc_body().replace("BKRD_DRIVEN=\"137\"", "BKRD_DRIVEN=\"many\"");

    let err = bkcc_admit(&said, "the hurdle's own record")
        .expect_err("a count that is not a count refuses");

    assert!(
        err.contains("BKRD_DRIVEN") && err.contains("many"),
        "the refusal names the field and what it carried: {}",
        err
    );
}

/// AN ABSENT LOOSEBOX REFUSES AND NAMES THE VARIABLE, which is the whole of the
/// no-fallback rule made observable. A door handed nothing has two wrong
/// answers available to it — compose one, or treat the absence as "no loosebox
/// today" — and both write, or decline to sweep, somewhere nobody is looking.
/// The refusal names the variable because that is the only thing an operator
/// meeting it can act on.
#[test]
fn bktc_an_absent_loosebox_refuses_naming_the_variable() {
    for said in ["", "   "] {
        let err = bkcc_looseboxed(said).expect_err(concat!(
            "an absent ",
            bkg_burc_loosebox_root_dir_base!(),
            " refuses", //
        ));

        assert!(
            err.contains(BKCC_LOOSEBOX_DIR_VAR),
            "the refusal names {}: {}",
            BKCC_LOOSEBOX_DIR_VAR,
            err
        );
    }
}

// eof
