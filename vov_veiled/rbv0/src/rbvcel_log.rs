// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCEL — the hierophant's emission module (RCG Output Discipline).
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
//
// Every section banner names the elected world (RBSHC "The election"). The
// world is held here once, set at a command's entry before its first banner,
// so the banner call sites carry no world of their own and none can forget it.

use std::io::Write;
use std::sync::OnceLock;

use crate::rbvcew_world;

const ZRBVCEL_FATAL_TAG: &str = "[FATAL]";

/// The world every section banner names, elected once per process.
static ZRBVCEL_WORLD: OnceLock<rbvcew_world::rbvcew_World> = OnceLock::new();

/// Emit one narration line verbatim to stderr.
fn zrbvcel_emit(msg: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{}", msg);
}

/// Elect the world the section banners name — once, at a command's entry.
/// A second election is a fault: one process conducts in one world.
pub fn rbvcel_elect(world: rbvcew_world::rbvcew_World) {
    if ZRBVCEL_WORLD.set(world).is_err() {
        crate::rbvcel_fatal!("the banner world was elected twice in one process");
    }
}

/// A section header — a blank line, the elected world and the title, and a
/// rule beneath it. A banner before the election is a fault, never a banner
/// naming no world.
pub fn rbvcel_section(title: &str) {
    let world = ZRBVCEL_WORLD
        .get()
        .copied()
        .unwrap_or_else(|| crate::rbvcel_fatal!("a section banner before the world was elected"));
    zrbvcel_emit("");
    zrbvcel_emit(&format!("=== [{}] {} ===", rbvcew_world::rbvcew_banner(world), title));
}

/// A step marker — the active act, as it begins.
pub fn rbvcel_step(msg: &str) {
    zrbvcel_emit(&format!(">> {}", msg));
}

/// One plain indented narration line.
pub fn rbvcel_line(msg: &str) {
    zrbvcel_emit(&format!("   {}", msg));
}

/// A blank narration line.
pub fn rbvcel_blank() {
    zrbvcel_emit("");
}

/// A loud advisory the operator must see — a known gap, a caution.
pub fn rbvcel_warn(msg: &str) {
    zrbvcel_emit(&format!("!! {}", msg));
}

/// A success line closing a phase.
pub fn rbvcel_success(msg: &str) {
    zrbvcel_emit(&format!("OK {}", msg));
}

/// A verbatim block — no per-line decoration. For the stranger prompt and other
/// content whose exact bytes are handed onward.
pub fn rbvcel_raw(block: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{}", block);
}

/// Interactive confirmation ({rbskb_prompt}, RBS0 "Human Bash Ceremony Control
/// Voicings"): shows `msg`, blocks on the hierophant's own stdin for a y/N
/// line, and fatals on anything but an affirmative — decline, empty input, or
/// closed stdin are all a decline. Only the ostend and the docimasy's gauntlet
/// stage prompt; every other worker's own confirmation gate is left to fire on
/// its own inherited stdio (rbvcer_run::rbvcer_stream leaves it unmolested here).
pub fn rbvcel_confirm(msg: &str) {
    zrbvcel_emit(&format!("?? {} [y/N] ", msg));
    let mut line = String::new();
    let read = std::io::stdin().read_line(&mut line);
    let affirmative = matches!(read, Ok(n) if n > 0)
        && matches!(line.trim().to_lowercase().as_str(), "y" | "yes");
    if !affirmative {
        crate::rbvcel_fatal!("declined: {}", msg);
    }
}

/// The diagnostic fatal sink — prints the tagged, located message and exits 1.
/// Never returns.
#[doc(hidden)]
pub fn zrbvcel_emit_fatal(file: &str, line: u32, msg: &str) -> ! {
    {
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "{} [{}:{}] {}", ZRBVCEL_FATAL_TAG, file, line, msg);
        let _ = stderr.flush();
    }
    std::process::exit(1);
}

/// Hard stop. Emits a located [FATAL] and exits 1 — the Rust form of the bash
/// workers' buc_die_now, used for every RBSHE `{rbskb_fatal}`.
#[macro_export]
macro_rules! rbvcel_fatal {
    ($($arg:tt)*) => {
        $crate::rbvcel_log::zrbvcel_emit_fatal(file!(), line!(), &format!($($arg)*))
    };
}
