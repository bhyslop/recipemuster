// Copyright 2026 Scale Invariant, Inc.
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

//! Logging infrastructure — severity-named macros with auto file/line capture.
//!
//! All application logging goes through `apcrl_*_now!`, `apcrl_*_if!`, and
//! comparison variants. No naked `println!`/`eprintln!` elsewhere in the codebase.
//!
//! Output is line-oriented to stdout. Format:
//!   `[LEVEL] [YYYY-MM-DD HH:MM:SS.mmm] [file:line] message`
//!
//! The timestamp is local wall-clock with millisecond precision, captured at
//! emit time by `chrono::Local::now()`. Level, timestamp, and file:line flow
//! through a single formatter so stdout and the tee sink never drift.
//!
//! An optional file-tee sink can be installed via `apcrl_tee_init` once at
//! application startup. When present, every emission is appended to the tee
//! file in the exact format that goes to stdout — one format, two sinks. Tee
//! write failures are swallowed to avoid recursive-logging hazards; stdout
//! remains authoritative.
//!
//! Suffix families:
//!   `_now`  — unconditional emission
//!   `_if`   — boolean conditional, returns the bool
//!   `_eq`, `_ne`, `_lt`, `_gt`, `_le`, `_ge` — comparison conditional,
//!            stringifies arguments, returns the bool

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

pub const APCRL_LEVEL_TRACE: &str = "[TRACE]";
pub const APCRL_LEVEL_INFO:  &str = "[INFO]";
pub const APCRL_LEVEL_ERROR: &str = "[ERROR]";
pub const APCRL_LEVEL_FATAL: &str = "[FATAL]";

static ZAPCRL_TEE: OnceLock<Mutex<File>> = OnceLock::new();

/// Install a file-tee sink. Every subsequent `apcrl_*` emission is appended
/// to `path` in the exact format that goes to stdout. The file is opened in
/// append-create mode. Call once from application startup; subsequent calls
/// return `Err` without disturbing the active sink.
pub fn apcrl_tee_init(path: &Path) -> Result<(), String> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("open tee file {}: {}", path.display(), e))?;
    ZAPCRL_TEE.set(Mutex::new(file))
        .map_err(|_| "tee already initialized".to_string())?;
    Ok(())
}

fn zapcrl_tee_write(line: &str) {
    if let Some(mtx) = ZAPCRL_TEE.get() {
        if let Ok(mut f) = mtx.lock() {
            let _ = writeln!(f, "{}", line);
            let _ = f.flush();
        }
    }
}

fn zapcrl_format(level: &str, file: &str, line: u32, msg: &str) -> String {
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    format!("{} [{}] [{}:{}] {}", level, ts, file, line, msg)
}

#[doc(hidden)]
pub fn zapcrl_emit(level: &str, file: &str, line: u32, msg: &str) {
    let formatted = zapcrl_format(level, file, line, msg);
    println!("{}", formatted);
    zapcrl_tee_write(&formatted);
}

#[doc(hidden)]
pub fn zapcrl_emit_fatal(file: &str, line: u32, msg: &str) -> ! {
    let formatted = zapcrl_format(APCRL_LEVEL_FATAL, file, line, msg);
    println!("{}", formatted);
    zapcrl_tee_write(&formatted);
    std::process::exit(1);
}

// ============================================================
// Unconditional (_now)
// ============================================================

#[macro_export] macro_rules! apcrl_trace_now {
    ($($arg:tt)*) => { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_TRACE, file!(), line!(), &format!($($arg)*)) };
}
#[macro_export] macro_rules! apcrl_info_now {
    ($($arg:tt)*) => { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_INFO, file!(), line!(), &format!($($arg)*)) };
}
#[macro_export] macro_rules! apcrl_error_now {
    ($($arg:tt)*) => { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_ERROR, file!(), line!(), &format!($($arg)*)) };
}
#[macro_export] macro_rules! apcrl_fatal_now {
    ($($arg:tt)*) => { $crate::apcrl_log::zapcrl_emit_fatal(file!(), line!(), &format!($($arg)*)) };
}

// ============================================================
// Boolean conditional (_if)
// ============================================================

#[macro_export] macro_rules! apcrl_trace_if {
    ($cond:expr, $($arg:tt)*) => {{ let z = $cond; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_TRACE, file!(), line!(), &format!($($arg)*)); } z }};
}
#[macro_export] macro_rules! apcrl_error_if {
    ($cond:expr, $($arg:tt)*) => {{ let z = $cond; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_ERROR, file!(), line!(), &format!($($arg)*)); } z }};
}
#[macro_export] macro_rules! apcrl_fatal_if {
    ($cond:expr, $($arg:tt)*) => { if $cond { $crate::apcrl_log::zapcrl_emit_fatal(file!(), line!(), &format!($($arg)*)); } };
}

// ============================================================
// Trace comparison variants
// ============================================================

#[macro_export] macro_rules! apcrl_trace_eq {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl == *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_TRACE, file!(), line!(), &format!("{}: {} ({:?}) == {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_trace_ne {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl != *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_TRACE, file!(), line!(), &format!("{}: {} ({:?}) != {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_trace_lt {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl < *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_TRACE, file!(), line!(), &format!("{}: {} ({:?}) < {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_trace_gt {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl > *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_TRACE, file!(), line!(), &format!("{}: {} ({:?}) > {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_trace_le {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl <= *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_TRACE, file!(), line!(), &format!("{}: {} ({:?}) <= {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_trace_ge {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl >= *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_TRACE, file!(), line!(), &format!("{}: {} ({:?}) >= {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}

// ============================================================
// Error comparison variants
// ============================================================

#[macro_export] macro_rules! apcrl_error_eq {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl == *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_ERROR, file!(), line!(), &format!("{}: {} ({:?}) == {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_error_ne {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl != *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_ERROR, file!(), line!(), &format!("{}: {} ({:?}) != {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_error_lt {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl < *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_ERROR, file!(), line!(), &format!("{}: {} ({:?}) < {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_error_gt {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl > *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_ERROR, file!(), line!(), &format!("{}: {} ({:?}) > {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_error_le {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl <= *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_ERROR, file!(), line!(), &format!("{}: {} ({:?}) <= {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}
#[macro_export] macro_rules! apcrl_error_ge {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); let z = *zl >= *zr; if z { $crate::apcrl_log::zapcrl_emit($crate::apcrl_log::APCRL_LEVEL_ERROR, file!(), line!(), &format!("{}: {} ({:?}) >= {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } z }};
}

// ============================================================
// Fatal comparison variants
// ============================================================

#[macro_export] macro_rules! apcrl_fatal_eq {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); if *zl == *zr { $crate::apcrl_log::zapcrl_emit_fatal(file!(), line!(), &format!("{}: {} ({:?}) == {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } }};
}
#[macro_export] macro_rules! apcrl_fatal_ne {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); if *zl != *zr { $crate::apcrl_log::zapcrl_emit_fatal(file!(), line!(), &format!("{}: {} ({:?}) != {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } }};
}
#[macro_export] macro_rules! apcrl_fatal_lt {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); if *zl < *zr { $crate::apcrl_log::zapcrl_emit_fatal(file!(), line!(), &format!("{}: {} ({:?}) < {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } }};
}
#[macro_export] macro_rules! apcrl_fatal_gt {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); if *zl > *zr { $crate::apcrl_log::zapcrl_emit_fatal(file!(), line!(), &format!("{}: {} ({:?}) > {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } }};
}
#[macro_export] macro_rules! apcrl_fatal_le {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); if *zl <= *zr { $crate::apcrl_log::zapcrl_emit_fatal(file!(), line!(), &format!("{}: {} ({:?}) <= {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } }};
}
#[macro_export] macro_rules! apcrl_fatal_ge {
    ($left:expr, $right:expr, $($arg:tt)*) => {{ let (zl, zr) = (&$left, &$right); if *zl >= *zr { $crate::apcrl_log::zapcrl_emit_fatal(file!(), line!(), &format!("{}: {} ({:?}) >= {} ({:?})", format!($($arg)*), stringify!($left), zl, stringify!($right), zr)); } }};
}
