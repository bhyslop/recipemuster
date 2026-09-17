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

//! The delouse — the periodic sweep on a day, standing where the build path's
//! forced full clean stood.
//!
//! THE MEMORY AND NOT THE SWEEP. What a directory is and whether it may be
//! removed is `bkcy_sweep`'s, and this module reaches it rather than repeating
//! it; what is here is the one thing the on-demand door has no use for — a
//! record of the day the yard was last taken, so the act happens once and the
//! builds after it pay nothing.
//!
//! THE DAY IS THE DISPATCH'S OWN FACT, read from the timestamp the substrate
//! composed before this process began, and never from a clock this crate reads
//! or a crate it would take to read one. The course record already declines to
//! probe its host for the same reason: a door asking a question its caller has
//! already answered gets a second answer that can differ from the first.
//!
//! THE DAY IS READ FROM THE FILE'S CONTENT AND NEVER FROM ITS MTIME. A
//! modification time is metadata every copy, restore and checkout is free to
//! move, and the whole worth of this record is that it says which day the sweep
//! actually finished on.
//!
//! IT IS WRITTEN LAST, after every removal has landed, which is what makes a
//! crash mid-sweep cost a second sweep rather than a skipped one. A record
//! written first would claim a day whose work never finished, and the next
//! door would believe it.
//!
//! NOTHING IT WRITES STANDS IN THE TRACKED TREE (BKSNC-Kennelcraft.adoc
//! "Currency"). This is a door's own working memory rather than a measurement
//! of a tree, so it stands in the seat's own scratch and is read by this door
//! alone.

use std::path::Path;


/// The dispatch's timestamp, whose leading field is the day.
pub const BKCN_NOW_VAR: &str = "BURD_NOW_STAMP";

/// The record's basename in the seat's own scratch.
pub const BKCN_DAY_FILE: &str = "bkcn_delouse.day";

/// How many leading characters of the dispatch timestamp spell a day.
const ZBKCN_DAY_WIDTH: usize = 8;

/// The day this dispatch stands in, as the substrate spelled it.
pub fn bkcn_day() -> Result<String, String> {
    let said = std::env::var(BKCN_NOW_VAR)
        .ok()
        .filter(|stamp| !stamp.trim().is_empty())
        .ok_or_else(|| {
            format!(
                "the delouse has no day to keep: {} names no timestamp. A door reached outside a \
                 dispatch has no seat and no clock of its own — reach it through its tabtarget",
                BKCN_NOW_VAR
            )
        })?;

    let day: String = said.trim().chars().take(ZBKCN_DAY_WIDTH).collect();

    if day.len() != ZBKCN_DAY_WIDTH || !day.chars().all(|glyph| glyph.is_ascii_digit()) {
        return Err(format!(
            "{} carries '{}', whose leading {} character(s) spell no day — the delouse keeps the \
             dispatch's own reckoning and composes none of its own",
            BKCN_NOW_VAR,
            said.trim(),
            ZBKCN_DAY_WIDTH
        ));
    }

    Ok(day)
}

/// The day the yard was last swept, or `None` where no sweep has landed here.
///
/// AN UNREADABLE RECORD IS NO RECORD, and answering `None` rather than an error
/// is deliberate: every way this read can fail — an absent file, a truncated
/// one, bytes that spell no day — leaves the same question open, which is
/// whether a sweep has landed today, and the safe answer to an open question is
/// to sweep again. A sweep repeated costs a rebuild; a sweep skipped on the
/// strength of a record nobody could read is the failure this door exists to
/// prevent.
pub fn bkcn_read(seat: &Path) -> Option<String> {
    let held = std::fs::read_to_string(seat.join(BKCN_DAY_FILE)).ok()?;
    let day = held.trim().to_string();

    if day.len() != ZBKCN_DAY_WIDTH || !day.chars().all(|glyph| glyph.is_ascii_digit()) {
        return None;
    }

    Some(day)
}

/// Record the day, the last act of a sweep that landed whole.
pub fn bkcn_write(seat: &Path, day: &str) -> Result<(), String> {
    let record = seat.join(BKCN_DAY_FILE);

    std::fs::write(&record, format!("{}\n", day)).map_err(|err| {
        format!(
            "the yard was swept and the day could not be recorded at {}: {} — the next door will \
             sweep it again",
            record.display(),
            err
        )
    })
}

// eof
