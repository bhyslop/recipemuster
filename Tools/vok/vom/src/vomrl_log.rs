// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM output module - line-oriented diagnostics (RCG Output Discipline).
//!
//! The crate's single emission path. Every diagnostic is one complete line,
//! `[LEVEL] [file:line] message`, written to stderr - the module appends the
//! newline, callers never include it, and the call site's file and line are
//! captured automatically by the macros. stdout is deliberately reserved for
//! future census data output, so a consumer (e.g. Job Jockey shelling out to
//! this binary) can parse that stream uncorrupted by diagnostics.
//!
//! Four levels per RCG: trace (debug builds only), info (milestones), error
//! (recoverable), fatal (emit then exit). Only the `_now` variants exist today;
//! the `_if` and comparison variants are added per RCG when a call site needs
//! them.

/// Emit one diagnostic line to stderr. Called only by the `vomrl_*_now!` macros,
/// which capture the call site's file and line; not invoked directly.
#[doc(hidden)]
pub fn vomrl_emit(level: &str, file: &str, line: u32, args: std::fmt::Arguments) {
    // RCG output discipline: the crate's one sanctioned stderr write.
    eprintln!("[{}] [{}:{}] {}", level, file, line, args);
}

/// Trace: development debugging. Emits only in debug builds.
#[macro_export]
macro_rules! vomrl_trace_now {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::vomrl_log::vomrl_emit("TRACE", file!(), line!(), format_args!($($arg)*));
        }
    };
}

/// Info: operational milestone. Always emitted.
#[macro_export]
macro_rules! vomrl_info_now {
    ($($arg:tt)*) => {
        $crate::vomrl_log::vomrl_emit("INFO", file!(), line!(), format_args!($($arg)*))
    };
}

/// Error: recoverable failure. Emits and continues.
#[macro_export]
macro_rules! vomrl_error_now {
    ($($arg:tt)*) => {
        $crate::vomrl_log::vomrl_emit("ERROR", file!(), line!(), format_args!($($arg)*))
    };
}

/// Fatal: unrecoverable failure. Emits, then exits the process (never returns).
#[macro_export]
macro_rules! vomrl_fatal_now {
    ($($arg:tt)*) => {{
        $crate::vomrl_log::vomrl_emit("FATAL", file!(), line!(), format_args!($($arg)*));
        ::std::process::exit(1);
    }};
}

// eof
