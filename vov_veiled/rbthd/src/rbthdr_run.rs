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
// RBTHDR — the subprocess layer. The hierophant sequences proven workers
// (RBSHC "Worker, never authority"); this is how it reaches them.
//
// Two modes, because the callers want two things:
//   capture — run a short command and read its exit + streams (git gate reads).
//   stream  — run a worker tabtarget with live stdio the operator watches, and
//             a SCRUBBED dispatch env so it runs exactly as from a clean
//             terminal. A child tabtarget rebuilds all BURD_/BURC_/BURS_ state
//             in its own launcher; letting the hierophant's inherited dispatch
//             state leak in would hand the child a half-built regime.

use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Stdio};

/// Captured result of a short command.
pub struct rbthdr_Captured {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Captured result of a command whose stdout is a byte stream, not text. The
/// transposition byte-assert needs the template's exact committed bytes —
/// `capture`'s lossy UTF-8 conversion would quietly launder the very
/// difference the assert exists to catch.
pub struct rbthdr_CapturedBytes {
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: String,
}

/// Env-var prefixes of inherited BUK dispatch state, scrubbed before a child
/// tabtarget runs so it starts from the same blank slate a terminal invocation
/// gives it. The child's own launcher (bul_launcher → bud_dispatch) rebuilds
/// every one of these.
const ZRBTHDR_SCRUB_PREFIXES: &[&str] = &["BURD_", "BURC_", "BURS_", "BURV_", "BUZ_", "BURE_"];

fn zrbthdr_command(program: &OsStr, args: &[&str], cwd: &Path) -> Command {
    let mut cmd = Command::new(program);
    cmd.args(args).current_dir(cwd);
    cmd
}

/// Run a program, capturing its streams. A spawn failure (the program could not
/// be launched at all) is fatal — the conductor cannot proceed without its
/// tools. A non-zero exit is returned, not fatal: the caller judges the code.
pub fn capture(program: impl AsRef<OsStr>, args: &[&str], cwd: &Path) -> rbthdr_Captured {
    let program = program.as_ref();
    let out = zrbthdr_command(program, args, cwd)
        .stdin(Stdio::null())
        .output()
        .unwrap_or_else(|e| {
            crate::rbthdr_fatal!("cannot launch {}: {}", program.to_string_lossy(), e)
        });
    rbthdr_Captured {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// Run a program, capturing stdout as raw bytes. Same contract as `capture`:
/// spawn failure fatal, non-zero exit returned for the caller to judge.
pub fn capture_bytes(program: impl AsRef<OsStr>, args: &[&str], cwd: &Path) -> rbthdr_CapturedBytes {
    let program = program.as_ref();
    let out = zrbthdr_command(program, args, cwd)
        .stdin(Stdio::null())
        .output()
        .unwrap_or_else(|e| {
            crate::rbthdr_fatal!("cannot launch {}: {}", program.to_string_lossy(), e)
        });
    rbthdr_CapturedBytes {
        code: out.status.code().unwrap_or(-1),
        stdout: out.stdout,
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// Run a worker with inherited stdio (live output) and a scrubbed dispatch env.
/// `env` adds explicit overrides applied AFTER the scrub — e.g. BURE_CONFIRM=skip
/// for a prompted verb the ceremony drives headlessly. Returns the exit code;
/// a spawn failure is fatal.
pub fn stream(program: impl AsRef<OsStr>, args: &[&str], cwd: &Path, env: &[(&str, &str)]) -> i32 {
    let program = program.as_ref();
    let mut cmd = zrbthdr_command(program, args, cwd);
    for (key, _) in std::env::vars() {
        if ZRBTHDR_SCRUB_PREFIXES.iter().any(|p| key.starts_with(p)) {
            cmd.env_remove(&key);
        }
    }
    for (k, v) in env {
        cmd.env(k, v);
    }
    let status = cmd
        .stdin(Stdio::null())
        .status()
        .unwrap_or_else(|e| {
            crate::rbthdr_fatal!("cannot launch {}: {}", program.to_string_lossy(), e)
        });
    status.code().unwrap_or(-1)
}

/// A `date`-formatted stamp, shelled to match the bash workers' own stamps
/// exactly (rblm_harbinger.sh). Fatal on empty — a stamp is load-bearing in the
/// retire-aside and the memo path.
fn zrbthdr_date(format: &str, cwd: &Path) -> String {
    let got = capture("date", &[format], cwd);
    if got.code != 0 {
        crate::rbthdr_fatal!("date {} failed: {}", format, got.stderr.trim());
    }
    let stamp = got.stdout.trim().to_string();
    if stamp.is_empty() {
        crate::rbthdr_fatal!("date {} resolved empty", format);
    }
    stamp
}

/// YYYYMMDD — the findings-memo date.
pub fn datestamp(cwd: &Path) -> String {
    zrbthdr_date("+%Y%m%d", cwd)
}

/// YYYYMMDD-HHMMSS — the retire-aside stamp; second-grained so two runs in one
/// day cannot collide.
pub fn timestamp(cwd: &Path) -> String {
    zrbthdr_date("+%Y%m%d-%H%M%S", cwd)
}
