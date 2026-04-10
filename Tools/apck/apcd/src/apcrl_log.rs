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
//! All application logging goes through `apcrl_info!`, `apcrl_error!`, and
//! `apcrl_fatal!`. No naked `println!`/`eprintln!` elsewhere in the codebase.
//!
//! Output is line-oriented to stdout. Format:
//!   `[LEVEL] [file:line] message`

pub const APCRL_LEVEL_INFO:  &str = "[INFO]";
pub const APCRL_LEVEL_ERROR: &str = "[ERROR]";
pub const APCRL_LEVEL_FATAL: &str = "[FATAL]";

/// Macro infrastructure — do not call directly. Use `apcrl_info!` instead.
#[doc(hidden)]
pub fn zapcrl_emit(level: &str, file: &str, line: u32, msg: &str) {
    println!("{} [{}:{}] {}", level, file, line, msg);
}

/// Macro infrastructure — do not call directly. Use `apcrl_fatal!` instead.
#[doc(hidden)]
pub fn zapcrl_emit_fatal(file: &str, line: u32, msg: &str) -> ! {
    println!("{} [{}:{}] {}", APCRL_LEVEL_FATAL, file, line, msg);
    std::process::exit(1);
}

/// Log at INFO level. Supports format strings.
///
/// ```ignore
/// apcrl_info!("loaded {} entries", count);
/// ```
#[macro_export]
macro_rules! apcrl_info {
    ($($arg:tt)*) => {
        $crate::apcrl_log::zapcrl_emit(
            $crate::apcrl_log::APCRL_LEVEL_INFO,
            file!(), line!(), &format!($($arg)*)
        )
    };
}

/// Log at ERROR level. Supports format strings.
///
/// ```ignore
/// apcrl_error!("failed to parse clipboard: {}", err);
/// ```
#[macro_export]
macro_rules! apcrl_error {
    ($($arg:tt)*) => {
        $crate::apcrl_log::zapcrl_emit(
            $crate::apcrl_log::APCRL_LEVEL_ERROR,
            file!(), line!(), &format!($($arg)*)
        )
    };
}

/// Log at FATAL level and exit. Supports format strings. Never returns.
///
/// ```ignore
/// apcrl_fatal!("unrecoverable: {}", err);
/// ```
#[macro_export]
macro_rules! apcrl_fatal {
    ($($arg:tt)*) => {
        $crate::apcrl_log::zapcrl_emit_fatal(
            file!(), line!(), &format!($($arg)*)
        )
    };
}
