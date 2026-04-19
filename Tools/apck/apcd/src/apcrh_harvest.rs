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

//! Clipboard harvest — capture every **declared** clipboard flavor
//! verbatim to `{N}-in.{tag}.{ext}` files in the journal directory
//! (typically `$HOME/apcjd/`) on every focus with changed content. The
//! `tag` is the classifier's verdict (`clinical` or `nonclinical`); the
//! full filename is e.g. `10000-in.clinical.rtf`,
//! `10000-in.clinical.utf8.txt`, `10000-in.nonclinical.utf8.txt`.
//!
//! Enumeration is delegated to `apcrb_pasteboard`
//! (`apcrb_capture_declared_flavors`), which reads NSPasteboard directly
//! on macOS. There is no alternate capture path — a failure in `apcrb`
//! (FFI error, non-macOS build, empty pasteboard) surfaces as an `Err`
//! that the caller logs via `apcrl_error_now!` and leaves the focus
//! cycle without an artifact on disk. Making failure visible beats
//! silently duplicating arboard work the classifier already did.
//!
//! On the Clinical branch, the focus handler pairs each capture with a
//! `{N}-out.txt` anonymized copy written by `apcap_main`. PHI-at-rest
//! stays outside the repo; anonymization and promotion to test fixtures
//! are manual. The destination is supplied by the caller; see
//! `apcrj_journal::apcrj_journal_path`. This module does not emit —
//! errors are returned to the caller for routing via `apcrl_*`;
//! `apcrb_pasteboard` logs its own FFI diagnostics at the boundary.

use std::fs;
use std::path::Path;

pub const APCRH_HARVEST_SEED_INDEX: u32 = 10000;

/// Classifier verdict used to tag harvest filenames. The tag is decided by
/// the caller *before* harvest runs, not inferred from filename later.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum apcrh_Classification {
    Clinical,
    NonClinical,
}

impl apcrh_Classification {
    /// Filename infix written between `-in.` and the extension.
    pub fn apcrh_tag(self) -> &'static str {
        match self {
            apcrh_Classification::Clinical    => "clinical",
            apcrh_Classification::NonClinical => "nonclinical",
        }
    }
}

/// Capture every **declared** clipboard flavor into `{N}-in.{tag}.{ext}`
/// files in `dir`, where `tag` is `classification`'s filename infix.
/// Returns the N used. N seeds at `APCRH_HARVEST_SEED_INDEX` when `dir` is
/// empty, otherwise `max_leading_digit_run + 1` across any filenames that
/// begin with digits (legacy bare `{N}.{ext}`, prior `{N}-in.{ext}`,
/// `{N}-out.{ext}`, new `{N}-in.{tag}.{ext}`, and multi-flavor
/// `{N}-in.{tag}.rtf` / `{N}-in.{tag}.utf8.txt` all count). The directory
/// is created lazily.
///
/// Enumeration is delegated to `apcrb_pasteboard`. An error from `apcrb`
/// is propagated — harvest does not synthesize a fallback capture. The
/// caller (`apcap_main`) logs the failure and continues triage; absence
/// of a `{N}-in.{tag}.*` file on disk is the visible signal that the
/// NSPasteboard path failed.
pub fn apcrh_capture_all_flavors(
    dir: &Path,
    classification: apcrh_Classification,
) -> Result<u32, String> {
    fs::create_dir_all(dir)
        .map_err(|e| format!("create harvest dir {}: {}", dir.display(), e))?;

    let index = zapcrh_scan_next_index(dir)?;

    crate::apcrb_pasteboard::apcrb_capture_declared_flavors(
        dir, classification, index,
    )?;

    Ok(index)
}

/// Scan `dir` for files whose name begins with a digit run and return
/// `max+1`, or `APCRH_HARVEST_SEED_INDEX` when none are found. Files whose
/// name does not start with a digit (e.g. `apcap.log`, `README`) are
/// ignored, as are subdirectories. Gaps are not filled — the scan advances
/// past the current maximum. The leading-digit parse makes mixed naming
/// styles — legacy bare `{N}.{ext}`, prior `{N}-in.{ext}`, `{N}-out.{ext}`,
/// and new `{N}-in.{tag}.{ext}` — all count toward the same index space.
pub(crate) fn zapcrh_scan_next_index(dir: &Path) -> Result<u32, String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("read harvest dir {}: {}", dir.display(), e))?;

    let mut max: Option<u32> = None;
    for entry in entries {
        let entry = entry
            .map_err(|e| format!("read harvest entry: {}", e))?;
        if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
            continue;
        }
        let path = entry.path();
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(s) => s,
            None    => continue,
        };
        let digits: String = name.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            continue;
        }
        if let Ok(n) = digits.parse::<u32>() {
            max = Some(max.map_or(n, |m| m.max(n)));
        }
    }

    Ok(match max {
        Some(n) => n.saturating_add(1),
        None    => APCRH_HARVEST_SEED_INDEX,
    })
}

