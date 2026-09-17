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

//! The course record — the fact file a door writes after a course that ran
//! green, and the launch side's one durable measurement.
//!
//! GREEN ONLY, AND THE FILE'S EXISTENCE IS THE VERDICT. A red course writes
//! nothing and no refusal writes anything, so nothing here carries a disposition
//! field and nothing carries failure detail: the studbook keeps the last
//! position a suite ran green at and never that it was red. A failure is
//! repaired rather than remembered, and the per-hurdle detail a repair wants
//! stands in the historical log member where the runner's own stream already
//! holds it.
//!
//! THE KENNEL WRITES THE RECORD AND NEVER READS IT. Its life past the output
//! directory belongs to the engine that harvests it, which is why the roster is
//! stated once and both sides read it: the render below writes the fields and
//! the admit below reads them back, so a harvester validating a file is running
//! the writer's own contract rather than a second description of it.
//!
//! THE RECORD CARRIES NO STATION. The station is the dispatcher's fact, already
//! on every dispatch record, and the engine stamps it at the harvest — so a door
//! that probed its host would be answering a question its caller had already
//! answered, and would want a crate to do it with.
//!
//! IT IS A REGIME AND NOT A FORMAT OF ITS OWN. The file is the substrate's
//! assignment form, the regime plane's own prefix is its extension, and the
//! reader beside the kennel is what parses it back — so nothing here hand-rolls
//! a serializer, and the writer proves its own output readable by admitting it
//! through that reader before either copy lands.

use std::path::{Path, PathBuf};

use bkl::bklrc_catena;

/// The dispatch's output directory — the latest-command copy, cleared on the
/// next dispatch, and the one the engine reads the moment a door returns.
pub const BKCC_OUTPUT_DIR_VAR: &str = "BURD_OUTPUT_DIR";

/// The dispatch's temp directory — the durable twin, addressable by anyone who
/// captured the path.
pub const BKCC_TEMP_DIR_VAR: &str = "BURD_TEMP_DIR";

/// The checkout's own loosebox — where this checkout's derived and rebuildable
/// build products stand, one directory outside every source tree, composed by
/// the substrate and keyed on the checkout's dirname.
pub const BKCC_LOOSEBOX_DIR_VAR: &str = "BURD_LOOSEBOX_DIR";

/// The checkout's own loosebox, settled from what a caller was handed.
///
/// IT IS THE CHECKOUT'S LOOSEBOX AND NOT THE PER-DISPATCH DIRECTORY, which is
/// the whole reason the loosebox is read: the scratch directory the substrate
/// hands a door is composed fresh for each invocation, so a record written there
/// would be a record no later door could ever find, and the sweep would fire at
/// every build exactly as the forced clean it replaces did.
///
/// IT READS A VARIABLE AND COMPOSES NOTHING. The loosebox is the substrate's own
/// composition — the root a regime field names, keyed by the checkout's dirname
/// — and this door takes it whole. It stood on the temp root's parent before
/// there was a root to name, and that reading is gone: a door deriving a
/// directory by walking up out of one it was handed is guessing at a layout it
/// does not own, and the guess reads as correct until the layout moves.
pub fn bkcc_looseboxed(said: &str) -> Result<PathBuf, String> {
    let said = said.trim();

    if said.is_empty() {
        return Err(format!(
            "this door has no loosebox to work in: {} names no directory. A door reached \
             outside a dispatch has no seat, and this one writes into no ambient directory",
            BKCC_LOOSEBOX_DIR_VAR
        ));
    }

    let seat = PathBuf::from(said);

    if !seat.is_dir() {
        return Err(format!(
            "{} names {}, which is no directory that stands — the dispatch composes its own \
             seats, so a name pointing at nothing is a misprovisioned seat rather than a \
             directory to create",
            BKCC_LOOSEBOX_DIR_VAR,
            seat.display()
        ));
    }

    Ok(seat)
}

/// The one line of this module that consults the environment, so every reading
/// above it can be posed a value and driven without touching a process-wide
/// setting its neighbours also read.
pub fn bkcc_loosebox() -> Result<PathBuf, String> {
    bkcc_looseboxed(&std::env::var(BKCC_LOOSEBOX_DIR_VAR).unwrap_or_default())
}

/// The extension that classifies a fact file as a course record.
///
/// IT IS THE REGIME'S OWN PREFIX, which is what makes the multi-form's promise
/// hold here — a fact file recognizable on disk by extension alone. The plane
/// already guarantees the token is spent nowhere else, so a registry entry
/// spelling a second token would be a second name for one thing.
pub const BKCC_EXTENSION: &str = "bkrd";

/// The narrowing's value where nothing narrowed the course.
pub const BKCC_WHOLE: &str = "bknre_whole";

/// The wall time's value where the course did not run whole.
pub const BKCC_NARROWED: &str = "bknre_narrowed";

/// The roster, in the order a record renders it.
pub const BKCC_COLLAR: &str = "BKRD_COLLAR";
pub const BKCC_NARROWING: &str = "BKRD_NARROWING";
pub const BKCC_DRIVEN: &str = "BKRD_DRIVEN";
pub const BKCC_PASSED: &str = "BKRD_PASSED";
pub const BKCC_HELD: &str = "BKRD_HELD";
pub const BKCC_WALL: &str = "BKRD_WALL";
pub const BKCC_POSITION: &str = "BKRD_POSITION";

/// What one green course measured.
///
/// THE NARROWING AND THE WALL TIME ARE ONE FACT SEEN TWICE, and both are carried
/// because a regime file is read a line at a time: a reader holding the wall
/// line must not have to find the narrowing line to know whether the number
/// means anything. `None` on the narrowing is a whole course, which is the only
/// shape that carries a time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcc_Course {
    /// The collar the course launched, which is the suite's identity.
    pub collar: String,
    /// The selection patterns the course ran under; `None` where nothing
    /// narrowed it.
    pub narrowing: Option<Vec<String>>,
    /// How many hurdles the course drove.
    pub driven: usize,
    /// How many of them passed.
    pub passed: usize,
    /// How many hurdles the suite holds — what the driven count is out of.
    pub held: usize,
    /// The course's wall time; `Some` only where the course ran whole.
    pub wall: Option<std::time::Duration>,
    /// The position the course ran at.
    pub position: String,
}

impl bkcc_Course {
    /// The record as it lands: the substrate's assignment form, one field per
    /// line, in the roster's order.
    ///
    /// EVERY FIELD IS SPELLED AND NONE STANDS VACANT. A reader deriving *whole
    /// course* or *no time taken* from an ABSENCE could not tell a declaration
    /// from an omission, which is the one distinction a validating reader exists
    /// to draw — so the two facts a number cannot carry are carried by a sprue
    /// token instead.
    pub fn bkcc_body(&self) -> String {
        let narrowing = match &self.narrowing {
            Some(patterns) => patterns.join(" "),
            None => BKCC_WHOLE.to_string(),
        };

        let wall = match self.wall {
            Some(taken) => taken.as_millis().to_string(),
            None => BKCC_NARROWED.to_string(),
        };

        let mut said = String::new();
        said.push_str(&zbkcc_line(BKCC_COLLAR, &self.collar));
        said.push_str(&zbkcc_line(BKCC_NARROWING, &narrowing));
        said.push_str(&zbkcc_line(BKCC_DRIVEN, &self.driven.to_string()));
        said.push_str(&zbkcc_line(BKCC_PASSED, &self.passed.to_string()));
        said.push_str(&zbkcc_line(BKCC_HELD, &self.held.to_string()));
        said.push_str(&zbkcc_line(BKCC_WALL, &wall));
        said.push_str(&zbkcc_line(BKCC_POSITION, &self.position));
        said
    }

    /// The file this course's record lands as, under the multi-form's
    /// type-as-extension naming: the collar is the row, the regime is the type.
    pub fn bkcc_leaf(&self) -> String {
        format!("{}.{}", self.collar, BKCC_EXTENSION)
    }
}

/// Read a record back under the roster — the harvester's face, and the only one.
///
/// IT IS THE WRITER'S OWN CONTRACT READ BACKWARD rather than a second
/// description of the file. A harvester validating a record and a hurdle
/// asserting one both come through here, so a field that moved moves for both in
/// the same edit and neither can drift from the render above.
///
/// A FIELD OUTSIDE THE ROSTER REFUSES rather than being carried along. The
/// roster is the one definition of what the studbook keeps about a test
/// disposition, and a reader that tolerated an extra field would be admitting a
/// record whose writer meant something this one does not know.
pub fn bkcc_admit(source: &str, whence: &str) -> Result<bkcc_Course, String> {
    let regime = bklrc_catena::bklrc_admit(source, whence)?;

    for key in regime.bklrc_keys() {
        if !bkcc_roster().contains(&key) {
            return Err(format!(
                "{} declares '{}', which no field of the course record's roster names — the \
                 roster is the one definition of what the studbook keeps about a test \
                 disposition, so a field outside it is a record this reader cannot vouch for",
                whence, key
            ));
        }
    }

    let collar = zbkcc_demand(&regime, BKCC_COLLAR, whence)?.to_string();

    let narrowing = match zbkcc_demand(&regime, BKCC_NARROWING, whence)? {
        BKCC_WHOLE => None,
        patterns => Some(patterns.split_whitespace().map(str::to_string).collect()),
    };

    let driven = zbkcc_counted(&regime, BKCC_DRIVEN, whence)?;
    let passed = zbkcc_counted(&regime, BKCC_PASSED, whence)?;
    let held = zbkcc_counted(&regime, BKCC_HELD, whence)?;

    let wall = match zbkcc_demand(&regime, BKCC_WALL, whence)? {
        BKCC_NARROWED => None,
        taken => Some(std::time::Duration::from_millis(taken.parse().map_err(|_| {
            format!(
                "{} carries '{}' as {}, which is neither a count of milliseconds nor the token \
                 '{}' that stands where a narrowed course took no time",
                whence, taken, BKCC_WALL, BKCC_NARROWED
            )
        })?)),
    };

    let position = zbkcc_demand(&regime, BKCC_POSITION, whence)?.to_string();

    // THE TWO SPELLINGS OF ONE FACT MUST AGREE, and disagreement is a refusal
    // rather than a preference for one of them. A record saying it ran whole and
    // took no time — or that it was narrowed and took 40 seconds — was written by
    // something that did not hold the rule, and neither line can be trusted to be
    // the honest half.
    if narrowing.is_none() != wall.is_some() {
        return Err(format!(
            "{} disagrees with itself: {} and {} carry one fact between them — a whole course \
             takes a time and a narrowed one takes the token '{}' — and this record spells the \
             two halves differently",
            whence, BKCC_NARROWING, BKCC_WALL, BKCC_NARROWED
        ));
    }

    Ok(bkcc_Course {
        collar,
        narrowing,
        driven,
        passed,
        held,
        wall,
        position,
    })
}

/// Write one course's record into the dispatch's output directory and its
/// durable twin.
///
/// THE DUAL WRITE IS THE SUBSTRATE'S, RESTATED IN RUST AND NOT SHELLED OUT.
/// Both copies land or neither does, and a copy that already stands is a
/// double-write bug rather than a file to overwrite: one write per collar per
/// dispatch is the multi-form's own cardinality, and a second write of one key
/// means two courses claimed one identity.
///
/// AN ABSENT DIRECTORY REFUSES AND REACHES FOR NO FALLBACK. A door that wrote
/// its measurement into an ambient temp directory would be recording where
/// nothing looks, which is indistinguishable from not recording at all — and the
/// kennel's doors report and refuse rather than converging on their own. A
/// hurdle driving a green course therefore states its own two directories, which
/// is the same discipline the composing surface already holds for the temp root.
pub fn bkcc_write(course: &bkcc_Course) -> Result<Vec<PathBuf>, String> {
    let body = course.bkcc_body();
    let leaf = course.bkcc_leaf();

    // THE WRITER PROVES ITS OWN OUTPUT READABLE BEFORE EITHER COPY LANDS. A
    // collar name or a selection pattern is text from outside this module, and a
    // value carrying whitespace, a quote or something bash would expand composes
    // a file the reader refuses — so the record would be unreadable at exactly
    // the moment nobody is left to notice. Admitting it here turns that into a
    // refusal naming the door, at the cost of one parse.
    bkcc_admit(&body, &leaf).map_err(|err| {
        format!(
            "the course record composed for '{}' is not readable under the catena law, so it was \
             not written: {}",
            course.collar, err
        )
    })?;

    let mut landed = Vec::new();

    for variable in [BKCC_OUTPUT_DIR_VAR, BKCC_TEMP_DIR_VAR] {
        let directory = zbkcc_directory(variable)?;
        let seat = directory.join(&leaf);

        if seat.exists() {
            return Err(format!(
                "a course record already stands at {} — one write per collar per dispatch is the \
                 whole cardinality of a fact file, so a second means two courses claimed the \
                 collar '{}'",
                seat.display(),
                course.collar
            ));
        }

        std::fs::write(&seat, &body).map_err(|err| {
            format!("could not write the course record at {}: {}", seat.display(), err)
        })?;

        landed.push(seat);
    }

    Ok(landed)
}

/// One assignment line in the substrate's form.
fn zbkcc_line(key: &str, value: &str) -> String {
    format!("{}=\"{}\"\n", key, value)
}

/// The roster, in the order a record renders it.
///
/// PUBLISHED BECAUSE A SECOND SIDE READS RECORDS. The kennel writes a course
/// record and never reads one back; its life past the output directory is the
/// engine's, which validates a harvested record against this same roster. That
/// reader spelled the seven names a second time while it could not link this
/// crate, and one definition serving both sides is what this module has always
/// stated it wanted.
pub fn bkcc_roster() -> [&'static str; 7] {
    [
        BKCC_COLLAR,
        BKCC_NARROWING,
        BKCC_DRIVEN,
        BKCC_PASSED,
        BKCC_HELD,
        BKCC_WALL,
        BKCC_POSITION,
    ]
}

/// A field the roster demands, refused by name where it does not stand.
fn zbkcc_demand<'a>(
    regime: &'a bklrc_catena::bklrc_Regime,
    key: &str,
    whence: &str,
) -> Result<&'a str, String> {
    regime.bklrc_scalar(key).ok_or_else(|| {
        format!(
            "{} carries no {} — every field of the course record's roster is required and never \
             vacant, a reader deriving a value from an absence being unable to tell a declaration \
             from an omission",
            whence, key
        )
    })
}

/// A field the roster types as a count.
fn zbkcc_counted(
    regime: &bklrc_catena::bklrc_Regime,
    key: &str,
    whence: &str,
) -> Result<usize, String> {
    let said = zbkcc_demand(regime, key, whence)?;

    said.parse().map_err(|_| {
        format!("{} carries '{}' as {}, which is no count", whence, said, key)
    })
}

/// The directory one copy lands in, refused where the dispatch named none.
fn zbkcc_directory(variable: &str) -> Result<PathBuf, String> {
    let said = std::env::var(variable)
        .ok()
        .filter(|path| !path.trim().is_empty())
        .ok_or_else(|| {
            format!(
                "the course ran green and its record has nowhere to land: {} names no directory. \
                 A door reached outside a dispatch has no output seat, and this one writes into no \
                 ambient directory — a record written where nothing looks is indistinguishable \
                 from a record never written. Reach the door through its tabtarget, or state the \
                 directory the record is to land in",
                variable
            )
        })?;

    let directory = PathBuf::from(said);

    if !Path::new(&directory).is_dir() {
        return Err(format!(
            "{} names {}, which is no directory that stands — the dispatch composes its own \
             output seats, so a name pointing at nothing is a misprovisioned seat rather than a \
             directory to create",
            variable,
            directory.display()
        ));
    }

    Ok(directory)
}

// eof
