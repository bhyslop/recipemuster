// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Klaxon — the crate's declared diagnostic output module (RCG "Output
//! Discipline"). Every naked `println!`/`eprintln!`/`print!`/`eprint!` in
//! this crate outside this file is a violation; route through the
//! `jjrk_*_now!` macros instead.
//!
//! Target: stderr only. The client-facing verdict text (`CallToolResult`,
//! monitum/interdictum grammar — JJS0 "MCP Transport") and the sectional's
//! phase-grain command narration (`jjrsj_sectional`) are both distinct
//! surfaces this module does not touch — RCG Output Discipline governs the
//! diagnostic sink alone.

use std::fmt;

/// Severity level, per RCG's four-level matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrk_Level {
    Trace,
    Info,
    Error,
    Fatal,
}

impl jjrk_Level {
    fn jjrk_tag(&self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Info => "INFO",
            Self::Error => "ERROR",
            Self::Fatal => "FATAL",
        }
    }
}

impl fmt::Display for jjrk_Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.jjrk_tag())
    }
}

/// Render one emission line: `[LEVEL] [file:line] message`. Exposed so tests
/// can assert the exact wire format without capturing stderr.
pub fn jjrk_render(level: jjrk_Level, file: &str, line: u32, message: &str) -> String {
    format!("[{}] [{}:{}] {}", level.jjrk_tag(), file, line, message)
}

/// Emit one rendered line to stderr. The macros below are the only intended
/// callers — they capture `file!()`/`line!()` at the call site.
pub fn jjrk_emit(level: jjrk_Level, file: &str, line: u32, message: &str) {
    eprintln!("{}", jjrk_render(level, file, line, message));
}

/// Development-debugging emission: unconditional, continues.
#[macro_export]
macro_rules! jjrk_trace_now {
    ($($arg:tt)*) => {
        $crate::jjrk_klaxon::jjrk_emit($crate::jjrk_klaxon::jjrk_Level::Trace, file!(), line!(), &format!($($arg)*))
    };
}

/// Operational-milestone emission: unconditional, continues.
#[macro_export]
macro_rules! jjrk_info_now {
    ($($arg:tt)*) => {
        $crate::jjrk_klaxon::jjrk_emit($crate::jjrk_klaxon::jjrk_Level::Info, file!(), line!(), &format!($($arg)*))
    };
}

/// Recoverable-failure emission: unconditional, continues.
#[macro_export]
macro_rules! jjrk_error_now {
    ($($arg:tt)*) => {
        $crate::jjrk_klaxon::jjrk_emit($crate::jjrk_klaxon::jjrk_Level::Error, file!(), line!(), &format!($($arg)*))
    };
}

/// Unrecoverable-failure emission: emits, then exits the process. Never
/// returns — per RCG, distinct from this crate's established panic-and-catch
/// fail-loud conduct (JJS0 "MCP Transport"), reserved for a condition the
/// answer-always membrane must not be given a chance to catch and answer.
#[macro_export]
macro_rules! jjrk_fatal_now {
    ($($arg:tt)*) => {{
        $crate::jjrk_klaxon::jjrk_emit($crate::jjrk_klaxon::jjrk_Level::Fatal, file!(), line!(), &format!($($arg)*));
        std::process::exit(1)
    }};
}
