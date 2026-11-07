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

//! The command line, and the crate's declared output module: every line the
//! tool prints is printed here.
//!
//! THE WORKING DIRECTORY IS THE PROJECT ROOT, as every tabtarget's trampoline
//! leaves it, and the clock is read here and nowhere else, so the library below
//! takes both as parameters.
//!
//! SOUND ON SUCCESS, THE STANDARD FAILURE CODE ON EVERY REFUSAL: a refusal
//! means the act did not happen, or happened only as far as the line says.

// Output discipline: all emission via bkhlrc_main — no direct println!/eprintln! elsewhere

use crate::bkhlrb_bale::{
    bkhlrb_Outcome,
    bkhlrb_bale,
    bkhlrb_latest,
    bkhlrb_read_back,
    BKHLRB_DAYS_BACK, //
};
use crate::bkhlrd_day::bkhlrd_Day;
use crate::bkhlrg_ground::{
    bkhlrg_read,
    BKHLRG_HAYLOFT_KEY,
    BKHLRG_LOG_KEY,
    BKHLRG_TEMP_KEY, //
};
use crate::bkhlru_unbale::{
    bkhlru_listing,
    bkhlru_unbale, //
};
use std::path::Path;
use std::process::ExitCode;
use std::time::SystemTime;

/// The verb that packs.
pub const BKHLRC_BALE: &str = "bale";

/// The verb that unpacks or lists.
pub const BKHLRC_UNBALE: &str = "unbale";

/// What every line the tool prints to its error stream opens with.
const ZBKHLRC_VOICE: &str = "bkhl";

/// Run the command line: print what the act answers, or why it refused.
pub fn bkhlrc_main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let today = bkhlrd_Day::bkhlrd_from_time(SystemTime::now());

    match bkhlrc_run(&args, Path::new("."), today) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!(
                "{}: {}",
                ZBKHLRC_VOICE,
                why, //
            );
            ExitCode::FAILURE
        }
    }
}

/// Run one command over the project at `root` on `today`, answering the lines
/// to print or why it refused.
pub fn bkhlrc_run(args: &[String], root: &Path, today: bkhlrd_Day) -> Result<Vec<String>, String> {
    let words: Vec<&str> = args.iter().map(String::as_str).collect();
    match words.as_slice() {
        [verb] if *verb == BKHLRC_BALE => Err(bkhlrc_latest_said(
            &format!(
                "{} takes the last day it packs, and none was given",
                BKHLRC_BALE, //
            ),
            today,
        )),
        [verb, day] if *verb == BKHLRC_BALE => {
            let last =
                bkhlrd_Day::bkhlrd_parse(day).map_err(|why| bkhlrc_latest_said(&why, today))?;
            if last > bkhlrb_latest(today) {
                return Err(bkhlrc_latest_said(
                    &format!(
                        "{} is later than {} days before today in UTC",
                        last.bkhlrd_digits(),
                        BKHLRB_DAYS_BACK, //
                    ),
                    today,
                ));
            }
            zbkhlrc_bale(root, last, today)
        }
        [verb] if *verb == BKHLRC_UNBALE => bkhlru_listing(&bkhlrg_read(root)?),
        [verb, archive, target] if *verb == BKHLRC_UNBALE => {
            let ground = bkhlrg_read(root)?;
            let unpacked = bkhlru_unbale(&ground, Path::new(archive), Path::new(target))?;
            Ok(vec![format!(
                "unpacked {} into {}",
                archive,
                unpacked.display(), //
            )])
        }
        _ => Err(bkhlrc_usage()),
    }
}

/// A refusal over the day bale was handed, closed by the latest day it takes
/// on `today`, ready to paste.
pub fn bkhlrc_latest_said(why: &str, today: bkhlrd_Day) -> String {
    format!(
        "{}\nthe latest day {} takes now: {}",
        why,
        BKHLRC_BALE,
        bkhlrb_latest(today).bkhlrd_digits(), //
    )
}

/// The usage text, naming both verbs and what each takes.
pub fn bkhlrc_usage() -> String {
    format!(
        "usage: {} DAY | {} [ARCHIVE TARGET]\n  \
         {}: pack every entry of the folders {} and {} name whose newest file is dated on or \
         before DAY into one tar.gz in the folder {} names, read it back against the originals, \
         then remove them. DAY is eight digits, year then month then day, and is always given; \
         none later than {} days ago is taken, and a missing, malformed or later day is refused \
         naming the latest day taken.\n  \
         {}: with no argument, list the archives in the folder {} names, one per line, each ready \
         to paste as ARCHIVE; with ARCHIVE and a TARGET folder that is empty or absent, unpack \
         the archive there.\n  \
         Days are calendar days in UTC, never the local zone's.",
        BKHLRC_BALE,
        BKHLRC_UNBALE,
        BKHLRC_BALE,
        BKHLRG_LOG_KEY,
        BKHLRG_TEMP_KEY,
        BKHLRG_HAYLOFT_KEY,
        BKHLRB_DAYS_BACK,
        BKHLRC_UNBALE,
        BKHLRG_HAYLOFT_KEY, //
    )
}

/// Bale with the read-back check, answering what was done.
fn zbkhlrc_bale(root: &Path, last: bkhlrd_Day, today: bkhlrd_Day) -> Result<Vec<String>, String> {
    let ground = bkhlrg_read(root)?;
    match bkhlrb_bale(&ground, last, today, &bkhlrb_read_back)? {
        bkhlrb_Outcome::bkhlrb_Nothing(last) => Ok(vec![format!(
            "nothing is dated on or before {}: no archive was written and nothing was removed",
            last.bkhlrd_digits(), //
        )]),
        bkhlrb_Outcome::bkhlrb_Wrote(baled) => {
            let summary = format!(
                "baled {} entries ({} members, {} through {}) into {}",
                baled.units,
                baled.members,
                baled.first.bkhlrd_digits(),
                baled.last.bkhlrd_digits(),
                baled.archive.display(), //
            );
            if baled.standing.is_empty() {
                return Ok(vec![summary]);
            }
            let mut why = format!(
                "{}, read back whole, but {} original(s) could not be removed:",
                summary,
                baled.standing.len(), //
            );
            for (origin, err) in &baled.standing {
                why.push_str(&format!(
                    "\n  {}: {}",
                    origin.display(),
                    err, //
                ));
            }
            Err(why)
        }
    }
}

// eof
