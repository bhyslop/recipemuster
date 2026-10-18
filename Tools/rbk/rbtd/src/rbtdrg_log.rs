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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTDRG — output module.
//
// All theurge emissions route through these macros, written to stderr.
// Stdout is reserved for future tabtarget-bypass needs.
//
// Format: `[LEVEL] [file:line] message`
//
// Suffix families implemented: `_now` only. `_if` and comparison variants
// can be added as callers grow a need for them.

use std::io::Write;

pub const RBTCEL_LEVEL_TRACE: &str = "[TRACE]";
pub const RBTCEL_LEVEL_INFO:  &str = "[INFO]";
pub const RBTCEL_LEVEL_ERROR: &str = "[ERROR]";
pub const RBTCEL_LEVEL_FATAL: &str = "[FATAL]";

fn zrbtcel_format(level: &str, file: &str, line: u32, msg: &str) -> String {
    format!("{} [{}:{}] {}", level, file, line, msg)
}

#[doc(hidden)]
pub fn zrbtcel_emit(level: &str, file: &str, line: u32, msg: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{}", zrbtcel_format(level, file, line, msg));
}

#[doc(hidden)]
pub fn zrbtcel_emit_fatal(file: &str, line: u32, msg: &str) -> ! {
    {
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "{}", zrbtcel_format(RBTCEL_LEVEL_FATAL, file, line, msg));
        let _ = stderr.flush();
    }
    std::process::exit(1);
}

#[macro_export]
macro_rules! rbtcel_trace_now {
    ($($arg:tt)*) => {
        $crate::rbtcel_log::zrbtcel_emit(
            $crate::rbtcel_log::RBTCEL_LEVEL_TRACE,
            file!(), line!(),
            &format!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! rbtcel_info_now {
    ($($arg:tt)*) => {
        $crate::rbtcel_log::zrbtcel_emit(
            $crate::rbtcel_log::RBTCEL_LEVEL_INFO,
            file!(), line!(),
            &format!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! rbtcel_error_now {
    ($($arg:tt)*) => {
        $crate::rbtcel_log::zrbtcel_emit(
            $crate::rbtcel_log::RBTCEL_LEVEL_ERROR,
            file!(), line!(),
            &format!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! rbtcel_fatal_now {
    ($($arg:tt)*) => {
        $crate::rbtcel_log::zrbtcel_emit_fatal(
            file!(), line!(),
            &format!($($arg)*),
        )
    };
}
