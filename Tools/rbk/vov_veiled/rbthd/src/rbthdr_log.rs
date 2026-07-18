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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — the hierophant's emission module (RCG Output Discipline).
//
// All output routes through here — no direct println!/eprintln! elsewhere in
// the crate. Two registers, because the hierophant serves two readers:
//
//   Narration — clean operator-facing ceremony text (section, step, line,
//   warn, success, raw). This IS the ceremony's execution-time home (ACG): the
//   maintainer reads it as the lap runs. No file:line noise.
//
//   Fatal — a diagnostic hard stop carrying [FATAL] [file:line], because a
//   fatal is a failure the maintainer debugs at the code.
//
// Everything is written to stderr; stdout is left clear for any future
// machine-readable channel.

use std::io::Write;

const ZRBTHDR_FATAL_TAG: &str = "[FATAL]";

/// Emit one narration line verbatim to stderr.
fn zrbthdr_emit(msg: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{}", msg);
}

/// A section header — a blank line, the title, and a rule beneath it.
pub fn section(title: &str) {
    zrbthdr_emit("");
    zrbthdr_emit(&format!("=== {} ===", title));
}

/// A step marker — the active act, as it begins.
pub fn step(msg: &str) {
    zrbthdr_emit(&format!(">> {}", msg));
}

/// One plain indented narration line.
pub fn line(msg: &str) {
    zrbthdr_emit(&format!("   {}", msg));
}

/// A blank narration line.
pub fn blank() {
    zrbthdr_emit("");
}

/// A loud advisory the operator must see — a known gap, a caution.
pub fn warn(msg: &str) {
    zrbthdr_emit(&format!("!! {}", msg));
}

/// A success line closing a phase.
pub fn success(msg: &str) {
    zrbthdr_emit(&format!("OK {}", msg));
}

/// A verbatim block — no per-line decoration. For the stranger prompt and other
/// content whose exact bytes are handed onward.
pub fn raw(block: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{}", block);
}

/// The diagnostic fatal sink — prints the tagged, located message and exits 1.
/// Never returns.
#[doc(hidden)]
pub fn zrbthdr_emit_fatal(file: &str, line: u32, msg: &str) -> ! {
    {
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "{} [{}:{}] {}", ZRBTHDR_FATAL_TAG, file, line, msg);
        let _ = stderr.flush();
    }
    std::process::exit(1);
}

/// Hard stop. Emits a located [FATAL] and exits 1 — the Rust form of the bash
/// workers' buc_die, used for every RBSHE `{rbbc_fatal}`.
#[macro_export]
macro_rules! rbthdr_fatal {
    ($($arg:tt)*) => {
        $crate::rbthdr_log::zrbthdr_emit_fatal(file!(), line!(), &format!($($arg)*))
    };
}
