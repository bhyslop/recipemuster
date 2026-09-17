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

//! The muzzle — the enumerated calls a door refuses to compile.
//!
//! The design is the muzzle sheaf's and is cited rather than restated. What this
//! module holds is the drive: the linter reached through the leash, and the
//! muzzle's own findings told apart from every other.
//!
//! NOTHING IS BANKED HERE, AND THE ABSENCE IS THE DESIGN (BKSMZ-Muzzle.adoc
//! "Currency"). A muzzle's verdict is a statement about the moment it ran, and
//! what carries that moment forward is the course that ran behind it — never a
//! file this module writes. A record in the tracked tree would also make this
//! door the one door whose own output trips the door law it stands behind.
//!
//! THE LINT LIST IS STATED, NEVER SEARCHED FOR. The linter's own upward search
//! resolves from each crate root, and the crates under this door share no root —
//! so the search would find a different list per crate, or none. The list's
//! directory is named by the collar that wears it and handed to the child as
//! environment, which is what lets one list govern crates sharing no root and
//! what keeps it governing them unchanged if they ever join one.
//!
//! CAPPING IS THE WHOLE TRICK, and without it this door would breach the
//! charter it enforces. The governed crates carry `#![deny(warnings)]` in their
//! own source, and a source attribute outranks a command-line lint flag — so
//! asking for one lint scopes NOTHING: driving the linter at all would promote
//! its entire corpus to hard errors, which is the wholesale adoption the sheaf
//! bars, arriving by a back door. Capping is the one flag that outranks the
//! attribute. So no lint can fail the compile, and this door decides instead: it
//! refuses on the muzzle's own findings and on nothing else, while every other
//! finding still prints as a warning rather than being silenced.
//!
//! A NON-ZERO EXIT WITH NO MUZZLE FINDING IS A COMPILE FAILURE and is reported
//! as one. Reading it as a refusal would be the exit code lying about which
//! thing went wrong — the crate did not build, which is not a statement about
//! any call it makes.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::bkce_election;
use crate::bkch_heel;
use crate::bkcl_leash;
use crate::bkcr_resolve::bkcr_Collar;

/// The linter's verb.
const ZBKCZ_VERB: &str = "clippy";

/// The whole of what the child is asked, in order: every target compiled so the
/// bar reaches test modules as well as shipped code, colour off so the report
/// reads the same on a terminal and in a log, then past the separator the cap
/// and the one lint this door refuses on.
///
/// The leash spells the manifest and the lock itself, ahead of the separator.
const ZBKCZ_ASKED: [&str; 8] = [
    "--all-targets",
    "--color",
    "never",
    "--",
    "--cap-lints",
    "warn",
    "-W",
    "clippy::disallowed_methods",
];

/// The mark that tells a muzzle finding from every other the linter emits: the
/// lint's own help URL, which no other lint's output carries.
///
/// The output is read for this rather than the exit code, because the cap has
/// made the exit code silent about lints by design. Reading the linter's own
/// name for its own lint is the narrowest thing that could serve.
const ZBKCZ_MARK: &str = "index.html#disallowed_methods";

/// The environment variable naming the directory the linter reads its list from.
/// Foreign schema, foreign name.
const ZBKCZ_CONF_VAR: &str = "CLIPPY_CONF_DIR";

/// The collar's OWN position — the converge's contract with the crate it builds
/// (`bkch_heel::BKCH_COLLAR_POSITION_VAR`, where the doc comment says why the
/// two cannot be one name).
///
/// LINTING COMPILES, WHICH IS THE WHOLE REASON THIS IS OWED. A crate whose build
/// script demands the reading dies here exactly as it would at a build door, and
/// the failure arrives as a compile error inside a lint sweep — a refusal that
/// says nothing about the crate's calls and sends its reader to the wrong
/// question entirely. Every road that BUILDS a crate must state what that
/// crate's build script demands, and this door is one.
///
/// NO PER-CRATE DECISION IS MADE: this is the collar's own declared roots read
/// off the collar in hand, stated uniformly for whatever collar is being linted.
/// A
/// collar declaring no roots — a lure's, composed for one hurdle — is stated
/// nothing, its absence an answer rather than a failure, on the precedent the
/// kennel election's absence already sets.
const ZBKCZ_COLLAR_ROOTS_FIELD: &str = "BKRR_ROOTS";

/// What one collar's drive came to.
///
/// EVERY ARM CARRIES THE CHILD'S WHOLE STREAM, the clean one included. A caller
/// that refuses needs the report to print; a caller that passes needs it to
/// record, because the cap leaves every lint this door does not name standing as
/// a warning and those warnings are worth keeping even on the drive that found
/// nothing barred. An arm that carried nothing would make the clean case the one
/// case whose evidence is gone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkcz_Verdict {
    /// The linter found no barred call.
    Clean(String),
    /// The muzzle refused: the report carries each finding and its sentence.
    Refused(String),
    /// The crate did not compile, which is not a statement about its calls.
    Unbuilt(String),
}

/// Lint one collar at the muzzle directory it declares.
///
/// The manifest and the muzzle both come from the collar, which is what retires
/// the hand roster this door replaces: a crate enters the muzzle by wearing a
/// collar that names a lint list, and leaves it by naming a different one. No
/// roster stands anywhere for a crate to be silently absent from.
pub fn bkcz_lint(repository: &Path, collar: &bkcr_Collar) -> Result<bkcz_Verdict, String> {
    // AHEAD OF EVERYTHING, because this is the road the defect was found on: the
    // sweep reached a crate whose exergue no door had struck yet, and the lint
    // run died on E0583 — which this door then reported as a build failure and
    // not a muzzle finding, correctly and uselessly, the operator being told the
    // crate did not compile and never which door would make it.
    crate::bkcr_resolve::bkcr_exergue_stands(repository, collar)?;

    let manifest = repository.join(collar.bkcr_field("BKRR_MANIFEST"));
    let conf = bkcz_conf_dir(repository, collar)?;

    let asked: Vec<&OsStr> = ZBKCZ_ASKED.iter().map(OsStr::new).collect();

    let declared = collar.bkcr_field(ZBKCZ_COLLAR_ROOTS_FIELD);
    let collar_seat = if declared.trim().is_empty() {
        None
    } else {
        Some(bkce_election::bkce_seat(repository, declared)?)
    };

    let mut stated: Vec<(&str, &OsStr)> = vec![(ZBKCZ_CONF_VAR, conf.as_os_str())];
    if let Some(position) = collar_seat.as_deref() {
        stated.push((
            bkch_heel::BKCH_COLLAR_POSITION_VAR,
            OsStr::new(position),
        ));
    }

    let recalled =
        bkcl_leash::bkcl_recall(repository, &manifest, ZBKCZ_VERB, asked, &stated)?;

    // THE LINTER SPEAKS ON STDERR, and its stdout is machine-readable. Both are
    // read here because a compile failure and a muzzle finding can arrive on
    // either, and the report handed to an operator is the whole of what the
    // child said rather than one channel of it.
    let report = format!(
        "{}{}",
        String::from_utf8_lossy(&recalled.said),
        recalled.grievance
    );

    if report.contains(ZBKCZ_MARK) {
        return Ok(bkcz_Verdict::Refused(report));
    }

    if !recalled.run.bkcl_landed() {
        return Ok(bkcz_Verdict::Unbuilt(report));
    }

    Ok(bkcz_Verdict::Clean(report))
}

/// The absolute directory a collar's muzzle field names.
///
/// ABSOLUTE, because the linter runs from the manifest's own directory: a
/// relative directory would resolve against the wrong place, and the linter
/// would report the list missing while every other reader found it perfectly.
pub fn bkcz_conf_dir(repository: &Path, collar: &bkcr_Collar) -> Result<PathBuf, String> {
    let declared = collar.bkcr_field("BKRR_MUZZLE");

    let directory = repository.join(declared);

    std::fs::canonicalize(&directory).map_err(|err| {
        format!(
            "the collar '{}' names the muzzle directory '{}', which does not resolve: {}",
            collar.name, declared, err
        )
    })
}

// eof
