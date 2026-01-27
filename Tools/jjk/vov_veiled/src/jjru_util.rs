// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops utility functions
//!
//! Helper functions for seed management, commit capture, and I/O.

use std::io::Read as IoRead;
use crate::jjrc_core::jjrc_timestamp_full as timestamp_full;
use crate::jjrf_favor::JJRF_CHARSET;
use crate::jjrt_types::*;

/// Increment a base64 seed string (with carry)
/// Works for both 2-char (heat) and 3-char (pace) seeds
#[allow(dead_code)]
pub(crate) fn zjjrg_increment_seed(seed: &str) -> String {
    let mut chars: Vec<u8> = seed.bytes().collect();
    let mut carry = true;

    // Process from right to left
    for i in (0..chars.len()).rev() {
        if !carry {
            break;
        }

        // Find current position in charset
        let pos = JJRF_CHARSET.iter().position(|&c| c == chars[i]).unwrap_or(0);

        if pos == 63 {
            // Wrap around
            chars[i] = JJRF_CHARSET[0];
            // carry remains true
        } else {
            chars[i] = JJRF_CHARSET[pos + 1];
            carry = false;
        }
    }

    String::from_utf8(chars).unwrap_or_else(|_| seed.to_string())
}

/// Capture current commit SHA for basis field
///
/// Runs `git rev-parse --short=N HEAD` where N is derived from JJRG_UNKNOWN_BASIS length.
/// Returns JJRG_UNKNOWN_BASIS on error (e.g., not in a git repo).
pub fn jjrg_capture_commit_sha() -> String {
    use std::process::Command;

    let short_arg = format!("--short={}", JJRG_UNKNOWN_BASIS.len());
    match Command::new("git")
        .args(["rev-parse", &short_arg, "HEAD"])
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => JJRG_UNKNOWN_BASIS.to_string(),
    }
}

/// Create a new Tack with current timestamp and basis SHA
///
/// Centralizes Tack creation to ensure consistent timestamp/basis capture.
/// All code creating new Tacks should use this helper.
pub fn jjrg_make_tack(
    state: jjrg_PaceState,
    text: String,
    silks: String,
    direction: Option<String>,
) -> jjrg_Tack {
    jjrg_Tack {
        ts: timestamp_full(),
        state,
        text,
        silks,
        basis: jjrg_capture_commit_sha(),
        direction,
    }
}

/// Read text from stdin (for CLI commands)
pub fn jjrg_read_stdin() -> Result<String, String> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)
        .map_err(|e| format!("Failed to read from stdin: {}", e))?;
    Ok(buffer.trim_end().to_string())
}

/// Read text from stdin, returning None if stdin is empty or a tty
pub fn jjrg_read_stdin_optional() -> Result<Option<String>, String> {
    use std::io::IsTerminal;

    // If stdin is a terminal, no input was piped
    if std::io::stdin().is_terminal() {
        return Ok(None);
    }

    let text = jjrg_read_stdin()?;
    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}
