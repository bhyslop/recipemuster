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

//! The delouse's hurdles — its memory, read and written.
//!
//! THE MEMORY AND NOT THE CADENCE. What a door does with a day it has read is a
//! fact about the binary, so the sweep-once-and-not-twice ordering is proven in
//! the integration seat over a spawned drive (`Tools/bkk/bk0/tests/bktd_drive.rs`)
//! and never here. What stands here is the reading and the writing themselves,
//! which is where the failure that matters lives: a record this module cannot
//! read must never be mistaken for a record saying today.
//!
//! THE ENVIRONMENT IS NOT PLANTED HERE. A unit hurdle setting a process-wide
//! variable is visible to every other hurdle the runner has in flight, so the
//! two readings that take one — the day and the seat — are proven through the
//! spawned drives instead, where the child's environment is the hurdle's to
//! compose.

use std::path::Path;

use super::bkcc_record::bkcc_looseboxed;
use super::bkcn_delouse::{bkcn_read, bkcn_write, BKCN_DAY_FILE};
use super::bktu_lure::bktu_Lure;
use super::bkg_breviary::{bkg_burc_loosebox_root_dir_base, BKG_BURC_LOOSEBOX_ROOT_DIR_BASE};

/// A seat of this hurdle's own to keep a record in.
///
/// THE LURE IS THE ONLY HOME FOR A DIRECTORY, so a hurdle wanting one composes a
/// lure and takes its seat rather than reaching for an ambient temp directory —
/// which is the same rule the drives keep, held here where nothing is spawned.
/// The lure is returned along with the seat because it clears the seat when it
/// falls, and a hurdle that dropped it would be asserting over a directory that
/// had just been removed.
fn zbktn_seat(name: &str) -> (bktu_Lure, std::path::PathBuf) {
    let lure = bktu_Lure::bktu_compose(name);
    let seat = lure.bktu_temp().to_path_buf();
    (lure, seat)
}

/// NO RECORD IS NO DAY, which is the state every seat starts in and the one that
/// must sweep.
#[test]
fn bktn_a_seat_with_no_record_reads_no_day() {
    let (_lure, seat) = zbktn_seat("delouse-absent");

    assert_eq!(bkcn_read(&seat), None);
}

/// WHAT WAS WRITTEN IS WHAT IS READ, which is the whole contract between the two
/// halves of this memory.
#[test]
fn bktn_a_recorded_day_reads_back() {
    let (_lure, seat) = zbktn_seat("delouse-round-trip");

    bkcn_write(&seat, "20260906").expect("the seat should take a record");

    assert_eq!(bkcn_read(&seat).as_deref(), Some("20260906"));
}

/// A SECOND WRITE REPLACES THE FIRST. The record says which day the yard was
/// last taken and never which days it has ever been taken, so an appending
/// writer would leave a file whose reading is the first day forever.
#[test]
fn bktn_a_second_record_replaces_the_first() {
    let (_lure, seat) = zbktn_seat("delouse-replace");

    bkcn_write(&seat, "20260906").expect("the seat should take a record");
    bkcn_write(&seat, "20260907").expect("the seat should take a second record");

    assert_eq!(bkcn_read(&seat).as_deref(), Some("20260907"));
}

/// AN UNREADABLE RECORD IS NO RECORD, and the hurdle drives every shape of
/// unreadable this module can meet — because each of them, answered as a day,
/// would skip a sweep on the strength of bytes nobody can vouch for.
#[test]
fn bktn_a_record_spelling_no_day_reads_none() {
    let (_lure, seat) = zbktn_seat("delouse-illegible");
    let record = seat.join(BKCN_DAY_FILE);

    for held in ["", "   ", "not-a-day", "2026090", "202609067", "2026-09-06"] {
        std::fs::write(&record, held).expect("the seat should take bytes");

        assert_eq!(
            bkcn_read(&seat),
            None,
            "'{}' spells no day and must read as no record",
            held
        );
    }
}

/// SURROUNDING WHITESPACE IS NOT A DEFECT. The writer ends its line, so a reader
/// that took the newline as part of the day would never match its own writing.
#[test]
fn bktn_a_record_reads_past_its_own_line_ending() {
    let (_lure, seat) = zbktn_seat("delouse-trimmed");

    std::fs::write(seat.join(BKCN_DAY_FILE), "  20260906  \n\n").expect("the seat takes bytes");

    assert_eq!(bkcn_read(&seat).as_deref(), Some("20260906"));
}

/// THE RECORD STANDS OUTSIDE THE TREE (BKSNC-Kennelcraft.adoc "Currency").
/// A door's own memory inside a repository would dirty it, and the door law
/// refuses an uncommitted repository — so a delouse keeping its day in the tree
/// would refuse every drive after its first.
#[test]
fn bktn_the_record_stands_outside_the_repository() {
    let lure = bktu_Lure::bktu_compose("delouse-outside");
    let seat = lure.bktu_temp();

    bkcn_write(seat, "20260906").expect("the seat should take a record");

    assert!(
        !seat.join(BKCN_DAY_FILE).starts_with(lure.bktu_root()),
        "the day stands at {}, which is inside the lure at {}",
        seat.join(BKCN_DAY_FILE).display(),
        lure.bktu_root().display()
    );
}

/// A HURDLE'S OWN SEAT IS ITS OWN, which is what the lure's per-seat root buys
/// and what a delouse racing its neighbours would cost. Two lures compose two
/// seats, and a record in one is invisible from the other.
#[test]
fn bktn_two_seats_keep_two_records() {
    let (_mine_lure, mine) = zbktn_seat("delouse-mine");
    let (_yours_lure, yours) = zbktn_seat("delouse-yours");

    assert_ne!(mine, yours, "two lures should compose two seats");

    bkcn_write(&mine, "20260906").expect("the seat should take a record");

    assert_eq!(bkcn_read(&mine).as_deref(), Some("20260906"));
    assert_eq!(bkcn_read(&yours), None);
}

/// A SEAT THAT DOES NOT STAND TAKES NO RECORD, and the writer says so rather
/// than answering as though it had written one — a delouse that reported a day
/// it never recorded would sweep once and skip forever after.
#[test]
fn bktn_a_seat_that_does_not_stand_refuses_the_record() {
    let (_lure, seat) = zbktn_seat("delouse-nowhere");
    let seat = seat.join("nothing-stands-here");

    assert!(
        bkcn_write(Path::new(&seat), "20260906").is_err(),
        "a record written where nothing stands should refuse"
    );
}

/// THE DAY STANDS IN THE LOOSEBOX THAT WAS POSED, and in no directory composed
/// by walking up out of another. The reading this replaces took the parent of
/// the dispatch's own scratch directory — correct only while the substrate's
/// layout held, and silently wrong the moment it moved. Posing the loosebox and
/// asserting the parent stays empty is what tells the two apart: a reading that
/// still climbed would answer the seat, and the record would land there.
#[test]
fn bktn_the_day_stands_in_the_posed_loosebox_and_never_in_a_parent() {
    let (_lure, seat) = zbktn_seat("delouse-loosebox");

    let loosebox = seat.join("loosebox-posed");
    std::fs::create_dir_all(&loosebox).expect(concat!(
        "the hurdle composes the ",
        bkg_burc_loosebox_root_dir_base!(),
        " it poses", //
    ));

    let answered = bkcc_looseboxed(&loosebox.display().to_string())
        .expect(concat!(
            "a ",
            bkg_burc_loosebox_root_dir_base!(),
            " that stands is answered", //
        ));

    assert_eq!(
        answered,
        loosebox,
        "the reading answers the posed {} whole",
        BKG_BURC_LOOSEBOX_ROOT_DIR_BASE, //
    );

    bkcn_write(&answered, "20260906").expect(concat!(
        "the ",
        bkg_burc_loosebox_root_dir_base!(),
        " should take a record", //
    ));

    assert_eq!(bkcn_read(&answered).as_deref(), Some("20260906"));
    assert_eq!(
        bkcn_read(&seat),
        None,
        "nothing was written to {}, the directory the reading this replaces would have climbed to",
        seat.display()
    );
}

// eof
