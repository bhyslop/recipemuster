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
//! test door in the estate sounds the same whatever ran underneath
//! (BKSNC-Kennelcraft.adoc "Bounded output"). That is only possible while
//! the door's own voice has exactly one throat.
//!
//! And the verbosity dial is the substrate's, `BURE_VERBOSE`, read once here
//! rather than at each call site. A dial read in many places is a dial that
//! disagrees with itself.
//!
//! `warn` and `error` are parted by the run's exit and never by how grave the
//! condition reads: an error line is a promise that the exit is nonzero. A
//! condition worth announcing on a run that still succeeds wears `warn`.

use std::io::Write;

/// The substrate's verbosity dial, read from the environment the dispatch
/// exported. Absent reads as quiet, which is the flavor a door takes by default.
pub const BKCO_VERBOSE_VAR: &str = "BURE_VERBOSE";

/// Where the substrate's dial stands for this run.
///
/// THE DIAL HAS THREE POSITIONS AND NOT TWO, which is the frozen shape's own
/// reading of it (BKSNC-Kennelcraft.adoc "Invocation shape"): at 0 the quiet
/// flavor, at 1 the verbose flavor, and ABOVE THAT the kennel's own diagnostics
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
    let _ = writeln!(err, "[{}] [{}] {}", level, site, message);
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
    let _ = writeln!(out, "{}", line);
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
/// EVERY MARK FLUSHES, which is the liveness cinch's whole mechanism
/// (BKSNC-Kennelcraft.adoc "Bounded output"). A heartbeat sitting in a block
/// buffer is a heartbeat nobody feels: stdout to a pipe is block-buffered by
/// default, which is exactly the case the mark exists for, so the flush is not
/// belt-and-braces but the thing itself.
pub fn bkco_voice(line: &str) {
    let mut out = std::io::stdout();
    let _ = writeln!(out, "{}", line);
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
                &format!("{}:{}", file!(), line!()),
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
            &format!("{}:{}", file!(), line!()),
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
            &format!("{}:{}", file!(), line!()),
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
            &format!("{}:{}", file!(), line!()),
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
                &format!("{}:{}", file!(), line!()),
                &format!($($arg)*),
            );
        }
        held
    }};
}

/// Unrecoverable. Emits and exits; never returns.
#[macro_export]
macro_rules! bkco_fatal_now {
    ($($arg:tt)*) => {{
        $crate::bkco_output::bkco_emit(
            "FATAL",
            &format!("{}:{}", file!(), line!()),
            &format!($($arg)*),
        );
        std::process::exit($crate::bkco_output::BKCO_EXIT_FATAL);
    }};
}

/// The exit a fatal takes. Distinct from cargo's own 101 so a caller reading the
/// exit alone can tell the kennel's refusal from a compile that died under it.
pub const BKCO_EXIT_FATAL: i32 = 70;

// eof
