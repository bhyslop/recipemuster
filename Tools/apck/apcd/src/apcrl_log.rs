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
//!   `[LEVEL] [file:line] message`
//!
//! Suffix families:
//!   `_now`  — unconditional emission
//!   `_if`   — boolean conditional, returns the bool
//!   `_eq`, `_ne`, `_lt`, `_gt`, `_le`, `_ge` — comparison conditional,
//!            stringifies arguments, returns the bool

pub const APCRL_LEVEL_TRACE: &str = "[TRACE]";
pub const APCRL_LEVEL_INFO:  &str = "[INFO]";
pub const APCRL_LEVEL_ERROR: &str = "[ERROR]";
pub const APCRL_LEVEL_FATAL: &str = "[FATAL]";

#[doc(hidden)]
pub fn zapcrl_emit(level: &str, file: &str, line: u32, msg: &str) {
    println!("{} [{}:{}] {}", level, file, line, msg);
}

#[doc(hidden)]
pub fn zapcrl_emit_fatal(file: &str, line: u32, msg: &str) -> ! {
    println!("{} [{}:{}] {}", APCRL_LEVEL_FATAL, file, line, msg);
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
