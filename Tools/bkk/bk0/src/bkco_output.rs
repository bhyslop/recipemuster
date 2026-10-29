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

//! The kennel's declared output module. Every emission in this crate comes
//! through one of these macros and no other file carries a naked print, which is
//! the estate's standing rule for a crate's output.
//!
//! Two properties this centralization buys the kennel in particular, beyond the
//! discipline's own grounds:
//!
//! The door consumes its tenants' streams and renders the voice itself, so every
//! test door in the estate sounds the same whatever ran underneath. That is
//! only possible while the door's own voice has exactly one throat.
//!
//! And the verbosity dial is the substrate's, `BURE_VERBOSE`, read once here
//! rather than at each call site. A dial read in many places is a dial that
//! disagrees with itself.
//!
//! `warn` and `error` are parted by the run's exit and never by how grave the
//! condition reads: an error line is a promise that the exit is nonzero. A
//! condition worth announcing on a run that still succeeds wears `warn`.

use std::io::Write;
use std::path::Path;

/// The substrate's verbosity dial, read from the environment the dispatch
/// exported. Absent reads as quiet, which is the flavor a door takes by default.
pub const BKCO_VERBOSE_VAR: &str = "BURE_VERBOSE";

/// Where the substrate's dial stands for this run.
///
/// THE DIAL HAS THREE POSITIONS AND NOT TWO: at 0 the quiet flavor, at 1 the
/// verbose flavor, and ABOVE THAT the kennel's own diagnostics
/// — the invocation it composed, the election it computed. Read as a boolean the
/// middle position collapses into the top one, and an operator asking for
/// per-case reporting is handed the kennel's interior as well.
///
/// A value that is not a number reads as 1 rather than as 0: it is somebody
/// asking for more, and the generous reading is the safe one for a dial whose
/// only effect is how much is said.
pub fn bkco_level() -> u8 {
    match std::env::var(BKCO_VERBOSE_VAR) {
        Err(_) => 0,
        Ok(dial) => match dial.trim() {
            "" => 0,
            said => said.parse().unwrap_or(1),
        },
    }
}

/// Whether the verbose flavor is armed: the dial at 1 or above.
pub fn bkco_verbose() -> bool {
    bkco_level() >= 1
}

/// Whether the kennel's own diagnostics are armed: the dial at 2 or above.
pub fn bkco_diagnostic() -> bool {
    bkco_level() >= 2
}

/// The one throat. Every macro below lands here, and nothing else writes.
///
/// Both flavors go to stderr rather than stdout, deliberately: the kennel's
/// stdout belongs to whatever a door is asked to hand back, so a diagnostic on
/// it would corrupt a caller reading the answer. The estate has paid for that
/// mistake once already, on a transport where stdout was the wire.
pub fn bkco_emit(level: &str, site: &str, message: &str) {
    let mut err = std::io::stderr();
    let _ = writeln!(
        err,
        "[{}] [{}] {}",
        level,
        site,
        message, //
    );
    let _ = err.flush();
}

/// The other throat: what a door was ASKED FOR, rather than what it has to say
/// about the asking.
///
/// It goes to stdout, and the split from the diagnostics above is the whole
/// reason it exists. A door's answer is meant to be read by a program — the
/// proclamation is env-shaped text a person reads and bash sources — while every
/// diagnostic on the same run goes to stderr, so the answer arrives clean
/// whatever else the run had to say. A door with one throat would force its
/// caller to sift the two apart by their shape, which is the parse this split
/// makes unnecessary.
///
/// Carrying no level and no site, for the same reason: a decoration on the
/// answer is a decoration the caller has to strip.
pub fn bkco_answer(line: &str) {
    let mut out = std::io::stdout();
    let _ = writeln!(
        out,
        "{}",
        line, //
    );
    let _ = out.flush();
}

/// The door's own VOICE: what the kennel renders of a tenant's run.
///
/// A third throat rather than a reuse of either above, because it is neither of
/// the two things they part. It is not a diagnostic — the run it describes may
/// be perfectly well — and it is not the answer a program reads, being prose
/// meant for the person watching. What it shares with the answer is the stream
/// and the flush, and what it must not share is the answer's promise that
/// stdout carries only machine-read text.
///
/// EVERY MARK FLUSHES, which is the liveness cinch's whole mechanism. A
/// heartbeat sitting in a block buffer is a heartbeat nobody feels: stdout to a
/// pipe is block-buffered by default, which is exactly the case the mark exists
/// for, so the flush is not belt-and-braces but the thing itself.
pub fn bkco_voice(line: &str) {
    let mut out = std::io::stdout();
    let _ = writeln!(
        out,
        "{}",
        line, //
    );
    let _ = out.flush();
}

/// Render one line of the door's voice.
#[macro_export]
macro_rules! bkco_voice_now {
    ($($arg:tt)*) => {{
        $crate::bkco_output::bkco_voice(&format!($($arg)*));
    }};
}

/// Hand back what a door was asked for. Never a diagnostic.
#[macro_export]
macro_rules! bkco_answer_now {
    ($($arg:tt)*) => {{
        $crate::bkco_output::bkco_answer(&format!($($arg)*));
    }};
}

/// The kennel's own diagnostics. Silent below 2 on the substrate's dial, where
/// the frozen shape puts them.
#[macro_export]
macro_rules! bkco_trace_now {
    ($($arg:tt)*) => {{
        if $crate::bkco_output::bkco_diagnostic() {
            $crate::bkco_output::bkco_emit(
                "TRACE",
                &format!("{}:{}", file!(), line!(), //
                ),
                &format!($($arg)*),
            );
        }
    }};
}

/// An operational milestone. Unconditional by nature, so it carries no
/// conditional variants.
#[macro_export]
macro_rules! bkco_info_now {
    ($($arg:tt)*) => {{
        $crate::bkco_output::bkco_emit(
            "INFO",
            &format!("{}:{}", file!(), line!(), //
            ),
            &format!($($arg)*),
        );
    }};
}

/// Announced on a run that still succeeds.
#[macro_export]
macro_rules! bkco_warn_now {
    ($($arg:tt)*) => {{
        $crate::bkco_output::bkco_emit(
            "WARN",
            &format!("{}:{}", file!(), line!(), //
            ),
            &format!($($arg)*),
        );
    }};
}

/// A recoverable failure. The run that emits one exits nonzero.
#[macro_export]
macro_rules! bkco_error_now {
    ($($arg:tt)*) => {{
        $crate::bkco_output::bkco_emit(
            "ERROR",
            &format!("{}:{}", file!(), line!(), //
            ),
            &format!($($arg)*),
        );
    }};
}

/// Fuse a condition to its diagnostic, so a caller cannot forget to say why it
/// took the branch. Returns the condition.
#[macro_export]
macro_rules! bkco_error_if {
    ($cond:expr, $($arg:tt)*) => {{
        let held = $cond;
        if held {
            $crate::bkco_output::bkco_emit(
                "ERROR",
                &format!("{}:{}", file!(), line!(), //
                ),
                &format!($($arg)*),
            );
        }
        held
    }};
}

/// Unrecoverable. Emits and ends the door broke; never returns.
#[macro_export]
macro_rules! bkco_fatal_now {
    ($($arg:tt)*) => {{
        $crate::bkco_output::bkco_emit(
            "FATAL",
            &format!("{}:{}", file!(), line!(), //
            ),
            &format!($($arg)*),
        );
        $crate::bkco_output::bkco_broke();
    }};
}

/// The broke verdict in flight, from wherever a door met it to the binary's
/// `main`, which ends the process on the standard library's failure code.
///
/// BROKE IS THE ONE VERDICT THE KENNEL SPELLS NO INTEGER FOR. The tool failed
/// before ruling, and the code that says so is the standard library's own
/// `ExitCode::FAILURE` — the imprecise death every caller already reads — so the
/// kennel declares nothing here, beside the barred and fenced codes it cites from
/// its generated constants.
///
/// IT TRAVELS AS AN UNWIND BECAUSE THE PINNED CHANNEL OFFERS NO OTHER ROAD. Only
/// `main`'s return hands the process an `ExitCode`; the method that would exit
/// with one from any depth is unstable there. So a door that meets broke unwinds
/// this payload, and `main` answers it with that code. The unwind runs the
/// destructors standing between, which an exit would have skipped.
///
/// THE PANIC HOOK NEVER HEARS IT: `resume_unwind` raises a payload without the
/// hook, so the fatal's own line is the whole of what is said. A genuine panic
/// carries no such payload, and `main` lets it continue as the panic it is.
pub struct bkco_Broke;

/// End the door broke, by the road `bkco_Broke` describes. Never returns.
pub fn bkco_broke() -> ! {
    std::panic::resume_unwind(Box::new(bkco_Broke))
}

// ============================================================================
// The door roster, and a colophon resolved to the tabtarget that ACTUALLY
// STANDS.
//
// Every refusal that sends a reader to a door names that door, and the naming
// is where such texts rot: a colophon is the durable half of a tabtarget's name
// and the frontispiece is prose that may be reworded, so a filename remembered
// from the day the refusal was written outlives its truth silently.
//
// THE ROSTER HOMES HERE AND NOT IN THE BINARY, which is the whole reason this
// section stands in the output module rather than beside the door arms. The
// arms live in a binary no library module can see, so a refusal composed in a
// library module had to spell its door's word again — and three such spellings
// of one word stood before this. What every one of those sites was doing was
// composing an operator-facing LINE, which is what this module is for.
// ============================================================================

/// The word each door answers to: the tabtarget's frontispiece, the binary's
/// subcommand, and the word spoken in the conduct text, which are one word at
/// three seats.
pub const BKCO_VERB_MUSH: &str = "mush";
pub const BKCO_VERB_DERBY: &str = "derby";
pub const BKCO_VERB_FUTURITY: &str = crate::bkg_breviary::BKG_FUTURITY_BASE;
pub const BKCO_VERB_LINEUP: &str = "lineup";
pub const BKCO_VERB_TATTOO: &str = "tattoo";
pub const BKCO_VERB_HEEL: &str = "heel";
pub const BKCO_VERB_MUZZLE: &str = "muzzle";
pub const BKCO_VERB_GANGLINE: &str = "gangline";
pub const BKCO_VERB_SCOOP: &str = "scoop";
pub const BKCO_VERB_DELOUSE: &str = "delouse";
pub const BKCO_VERB_WHISTLE: &str = "whistle";
pub const BKCO_VERB_KENNELMAN: &str = "kennelman";
pub const BKCO_VERB_REBED: &str = "rebed";
pub const BKCO_VERB_BLEACH: &str = crate::bkg_breviary::BKG_BLEACH_BASE;

/// The colophon each door's tabtarget wears — the durable half of its filename,
/// which a rename of the frontispiece leaves standing.
///
/// THE WHISTLE, THE KENNELMAN AND REBED ARE HERE AND ARE NO DOORS OF THE
/// REGISTER. They stand in `tt/` and a refusal may have to send a reader to any
/// of them — the whistle is what a cold seat runs first, and rebed is the one
/// converge a station or a tree that is not ready is told to run — so the roster
/// carries what the lookup must be able to resolve rather than what the binary
/// dispatches. Rebed's word is also a subcommand of the binary, its bash half
/// execing the binary's half once the kennel can be built.
pub const BKCO_COLOPHON_MUSH: &str = "bkw-m";
pub const BKCO_COLOPHON_DERBY: &str = "bkw-d";
pub const BKCO_COLOPHON_FUTURITY: &str = "bkw-f";
pub const BKCO_COLOPHON_LINEUP: &str = "bkw-l";
pub const BKCO_COLOPHON_TATTOO: &str = "bkw-t";
pub const BKCO_COLOPHON_HEEL: &str = "bkw-h";
pub const BKCO_COLOPHON_MUZZLE: &str = "bkw-z";
pub const BKCO_COLOPHON_GANGLINE: &str = "bkw-g";
pub const BKCO_COLOPHON_SCOOP: &str = "bkw-s";
pub const BKCO_COLOPHON_BLEACH: &str = "bkw-B";
pub const BKCO_COLOPHON_DELOUSE: &str = "bkw-n";
pub const BKCO_COLOPHON_WHISTLE: &str = "bkw-w";
pub const BKCO_COLOPHON_KENNELMAN: &str = "bkw-k";
pub const BKCO_COLOPHON_REBED: &str = "bkw-r";

/// Verb beside colophon, which is the one pairing this kit does read
/// positionally — and reads because a refusal naming a verb must hand its
/// reader the file that performs it. The collar's strike and exergue lists are
/// the opposite case and are documented as such at `bkcr_strike`.
const ZBKCO_DOORS: &[(&str, &str)] = &[
    (BKCO_VERB_MUSH, BKCO_COLOPHON_MUSH),
    (BKCO_VERB_DERBY, BKCO_COLOPHON_DERBY),
    (BKCO_VERB_FUTURITY, BKCO_COLOPHON_FUTURITY),
    (BKCO_VERB_LINEUP, BKCO_COLOPHON_LINEUP),
    (BKCO_VERB_TATTOO, BKCO_COLOPHON_TATTOO),
    (BKCO_VERB_HEEL, BKCO_COLOPHON_HEEL),
    (BKCO_VERB_MUZZLE, BKCO_COLOPHON_MUZZLE),
    (BKCO_VERB_GANGLINE, BKCO_COLOPHON_GANGLINE),
    (BKCO_VERB_SCOOP, BKCO_COLOPHON_SCOOP),
    (BKCO_VERB_BLEACH, BKCO_COLOPHON_BLEACH),
    (BKCO_VERB_DELOUSE, BKCO_COLOPHON_DELOUSE),
    (BKCO_VERB_WHISTLE, BKCO_COLOPHON_WHISTLE),
    (BKCO_VERB_KENNELMAN, BKCO_COLOPHON_KENNELMAN),
    (BKCO_VERB_REBED, BKCO_COLOPHON_REBED), //
];

/// The tabtarget directory the substrate's dispatch names in its file.
pub const BKCO_TABTARGET_DIR_VAR: &str = "BURD_TABTARGET_DIR";

/// How a tabtarget is reached from a repository root, which is what an operator
/// types and never the absolute path the dispatch exported.
pub const BKCO_TABTARGET_REACH: &str = "tt";

/// The live tabtarget filename for a colophon under a NAMED directory, or
/// `None` where none resolves.
///
/// THE PURE HALF, AND IT IS PURE SO IT CAN BE HURDLED. The composition below
/// takes the directory the dispatch named, which a hurdle could once pose only
/// by WRITING the process environment — and a written environment is shared
/// by every hurdle the harness is running in parallel, so a hurdle posing a
/// directory would be deciding its neighbours' answers. The kibble root
/// is split at exactly this seam and for exactly this reason
/// (`bkcq_feedroom_at`).
///
/// `None` ON ANY DOUBT: an unreadable directory, no matching entry, or TWO
/// matching entries, which is an ambiguity no caller may resolve on the
/// reader's behalf. A guess would send a reader to a path that does not exist,
/// which is worse than saying less.
///
/// The prefix comparison is Rust's and so is case-SENSITIVE, which is what lets
/// two colophons parted only by case resolve to different doors on a filesystem
/// that would fold them: the distinction lives in the listing's bytes, never in
/// a path this function asks the filesystem to open.
pub fn bkco_tabtarget_in(directory: &Path, colophon: &str) -> Option<String> {
    let prefix = format!(
        "{}.",
        colophon, //
    );
    let mut found: Option<String> = None;

    for entry in std::fs::read_dir(directory).ok()? {
        let name = entry.ok()?.file_name().to_str()?.to_string();
        if name.starts_with(&prefix) && name.ends_with(".sh") {
            if found.is_some() {
                return None;
            }
            found = Some(name);
        }
    }

    found
}

/// A door's tabtarget as a reader would type it: the one that stands, or the
/// colophon's own glob where none resolves.
///
/// The degraded form is `<colophon>.*.sh` rather than a sentence, and it is
/// chosen to stay ACTIONABLE at the same seat: it is what the reader types
/// after `ls`, it names the durable half exactly, and it admits the
/// frontispiece is unknown instead of inventing one.
///
/// THE DIRECTORY IS THE DISPATCH'S, handed as a value: a door holding a dispatch
/// hands the directory its file names, and a reading holding none hands `None`
/// and renders the glob.
pub fn bkco_door(tabtargets: Option<&Path>, colophon: &str) -> String {
    let resolved = tabtargets.and_then(|dir| bkco_tabtarget_in(dir, colophon));

    format!(
        "{}/{}",
        BKCO_TABTARGET_REACH,
        resolved.unwrap_or_else(|| format!(
            "{}.*.sh",
            colophon, //
        )), //
    )
}

/// The colophon a door's verb wears, or `None` where the verb is no door of
/// this kennel.
pub fn bkco_colophon(verb: &str) -> Option<&'static str> {
    ZBKCO_DOORS
        .iter()
        .find(|(word, _)| *word == verb)
        .map(|(_, colophon)| *colophon)
}

/// What a refusal hands its reader: the door's own word, and beside it the
/// tabtarget that performs it, with the imprint the act takes.
///
/// THE VERB STAYS IN THE SENTENCE beside the resolved file rather than being
/// replaced by it. The word is what the conduct text and the sheaves call the
/// act, so a reader who knows the kennel reads the verb and a reader who does
/// not reads the file — and neither is handed a name the other cannot use.
///
/// A VERB NO ROSTER CARRIES RENDERS AS ITSELF, never as a panic: this composes
/// operator-facing text, and a refusal that died while explaining a refusal
/// would lose the condition it was called to report.
pub fn bkco_call(tabtargets: Option<&Path>, verb: &str, imprint: Option<&str>) -> String {
    let door = match bkco_colophon(verb) {
        Some(colophon) => bkco_door(tabtargets, colophon),
        None => return verb.to_string(),
    };

    match imprint {
        Some(imprint) => format!(
            "{} — {} {}",
            verb,
            door,
            imprint, //
        ),
        None => format!(
            "{} — {}",
            verb,
            door, //
        ),
    }
}

// eof
