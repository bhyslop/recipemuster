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
// RBTDRX — cross-platform path transmutation at theurge's bash/Rust boundary.
//
// Cygwin runs Windows-native binaries from a POSIX-style shell. Paths cross
// the boundary in both directions:
//
//   - Intake (POSIX → native): env vars and arguments arrive as
//     `/cygdrive/c/...` or bare `/home/...` from Cygwin bash; Rust's PathBuf
//     joins them with backslashes on a Windows target, producing mixed
//     `/cygdrive/c/...\rbtd` paths Windows resolves unpredictably.
//   - Outflow (native → POSIX): PathBuf::display() emits native form
//     (`C:\foo\bar`); embedded in a bash script or env var consumed by
//     Cygwin bash, the consumer sees an unparseable path.
//
// This module provides the two conversions plus a cached Cygwin-detection
// probe. On Linux and macOS both conversions are identity, so call sites
// can be unconditional.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

static RBTDRX_IS_CYGWIN: OnceLock<bool> = OnceLock::new();

/// Detect whether we're running under a Cygwin-spawned shell environment.
///
/// On Cygwin, the launching bash exports `OSTYPE=cygwin` to spawned
/// processes — including Windows-native binaries like a
/// `x86_64-pc-windows-gnu` theurge. Result is cached for binary lifetime.
pub fn rbtdrx_is_cygwin() -> bool {
    *RBTDRX_IS_CYGWIN.get_or_init(|| {
        matches!(std::env::var("OSTYPE").as_deref(), Ok("cygwin"))
    })
}

/// Convert a path string from POSIX form (as Cygwin bash uses) to native
/// PathBuf form (as the Windows OS uses). On non-Cygwin platforms, identity.
///
/// Returns `Err` only when running on Cygwin AND the path is not a recognised
/// fast-path shape AND the `cygpath` fallback fails — i.e., the platform
/// claims Cygwin via OSTYPE but cygpath is unreachable.
pub fn rbtdrx_posix_to_native(s: &str) -> Result<PathBuf, String> {
    rbtdrx_posix_to_native_for(s, rbtdrx_is_cygwin())
}

/// Convert a Path from native form to POSIX form suitable for embedding in
/// a Cygwin bash script or for passing as an env var to a Cygwin-bash
/// tabtarget. On non-Cygwin platforms, returns the path's display form.
///
/// Best-effort: on Cygwin, falls back to `cygpath -u` for unrecognised
/// native shapes; if cygpath fails the original display form is returned
/// rather than failing the caller. Outflow sites (env vars, format!
/// interpolations) cannot meaningfully propagate errors mid-script.
pub fn rbtdrx_native_to_posix(p: &Path) -> String {
    rbtdrx_native_to_posix_for(&p.to_string_lossy(), rbtdrx_is_cygwin())
}

// ── Pure implementation — explicit is_cygwin parameter for unit tests ──

pub(crate) fn rbtdrx_posix_to_native_for(s: &str, is_cygwin: bool) -> Result<PathBuf, String> {
    if !is_cygwin {
        return Ok(PathBuf::from(s));
    }

    // Fast path: /cygdrive/X/foo → X:\foo
    if let Some(native) = rbtdrx_cygdrive_to_native(s) {
        return Ok(native);
    }

    // Already native Windows form (drive-letter prefix)
    if rbtdrx_looks_native_windows(s) {
        return Ok(PathBuf::from(s));
    }

    // Slow path: bare POSIX (/home/..., /tmp/...) needs cygpath
    rbtdrx_cygpath(s, "-w")
        .map(PathBuf::from)
        .ok_or_else(|| format!("rbtdrx: cygpath -w failed for '{}'", s))
}

pub(crate) fn rbtdrx_native_to_posix_for(s: &str, is_cygwin: bool) -> String {
    if !is_cygwin {
        return s.to_string();
    }

    // Fast path: X:\foo or X:/foo → /cygdrive/x/foo
    if let Some(posix) = rbtdrx_drive_to_cygdrive(s) {
        return posix;
    }

    // Already POSIX-looking: normalise any backslashes, return
    if s.starts_with('/') {
        return s.replace('\\', "/");
    }

    // Slow path: cygpath -u; fall back to the original form
    rbtdrx_cygpath(s, "-u").unwrap_or_else(|| s.to_string())
}

// ── Fast-path conversions (pure, unit-testable) ────────────────────────

/// Match `/cygdrive/X[/tail]` and produce `X:\tail`. Returns None if the
/// input does not match this shape.
pub(crate) fn rbtdrx_cygdrive_to_native(s: &str) -> Option<PathBuf> {
    let rest = s.strip_prefix("/cygdrive/")?;
    let mut chars = rest.chars();
    let drive = chars.next()?;
    if !drive.is_ascii_alphabetic() {
        return None;
    }
    let tail = chars.as_str();
    // Drive must be followed by '/' or end-of-string — otherwise it's not
    // /cygdrive/X form but something like /cygdrive/foo.
    if !tail.is_empty() && !tail.starts_with('/') {
        return None;
    }
    let drive_upper = drive.to_ascii_uppercase();
    let win_tail = tail.replace('/', "\\");
    Some(PathBuf::from(format!("{}:{}", drive_upper, win_tail)))
}

/// Match a drive-letter native path (`X:\foo`, `X:/foo`, or bare `X:`) and
/// produce the corresponding `/cygdrive/x/foo` form.
pub(crate) fn rbtdrx_drive_to_cygdrive(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    if bytes.len() < 2 {
        return None;
    }
    if !bytes[0].is_ascii_alphabetic() || bytes[1] != b':' {
        return None;
    }
    let drive = (bytes[0] as char).to_ascii_lowercase();
    let tail = &s[2..];
    if tail.is_empty() {
        return Some(format!("/cygdrive/{}", drive));
    }
    let posix_tail = tail.replace('\\', "/");
    let posix_tail = if posix_tail.starts_with('/') {
        posix_tail
    } else {
        format!("/{}", posix_tail)
    };
    Some(format!("/cygdrive/{}{}", drive, posix_tail))
}

pub(crate) fn rbtdrx_looks_native_windows(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

// ── cygpath fallback ───────────────────────────────────────────────────

fn rbtdrx_cygpath(input: &str, flag: &str) -> Option<String> {
    let output = Command::new("cygpath").arg(flag).arg(input).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8(output.stdout).ok()?;
    Some(s.trim_end_matches(['\n', '\r']).to_string())
}
