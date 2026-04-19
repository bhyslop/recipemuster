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

//! Clipboard harvest — capture every arboard-accessible clipboard flavor
//! verbatim to `{N}.{ext}` files in the journal directory (typically
//! `$HOME/apcjd/`) on the Clinical branch, before the system-clipboard
//! zero-out. PHI-at-rest stays outside the repo; anonymization and
//! promotion to test fixtures are manual. The destination is supplied by
//! the caller; see `apcrj_journal::apcrj_journal_path`. This module does
//! not emit — errors are returned to the caller for routing via `apcrl_*`.

use std::fs;
use std::io::Write;
use std::path::Path;

pub const APCRH_HARVEST_SEED_INDEX: u32 = 10000;

/// Capture every arboard-accessible clipboard flavor into `{N}.{ext}` files
/// in `dir`. Returns the N used. N seeds at `APCRH_HARVEST_SEED_INDEX` when
/// `dir` is empty, otherwise `max_existing_numeric_stem + 1`. The directory
/// is created lazily. Text is required; HTML is opportunistic — absence of
/// HTML on the clipboard is not an error.
pub fn apcrh_capture_all_flavors(dir: &Path) -> Result<u32, String> {
    fs::create_dir_all(dir)
        .map_err(|e| format!("create harvest dir {}: {}", dir.display(), e))?;

    let index = zapcrh_scan_next_index(dir)?;

    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("clipboard open: {}", e))?;

    let text = clipboard.get_text()
        .map_err(|e| format!("clipboard get_text: {}", e))?;
    zapcrh_write_text(dir, index, &text)?;

    // HTML is opportunistic — absence on the clipboard is expected for some
    // sources, not an error. Capture when present, skip otherwise.
    if let Ok(html) = clipboard.get().html() {
        zapcrh_write_html(dir, index, &html)?;
    }

    Ok(index)
}

/// Scan `dir` for numeric file stems and return `max+1`, or
/// `APCRH_HARVEST_SEED_INDEX` when none are found. Non-numeric stems and
/// subdirectories are ignored. Gaps are not filled — the scan advances past
/// the current maximum.
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
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s,
            None    => continue,
        };
        if let Ok(n) = stem.parse::<u32>() {
            max = Some(max.map_or(n, |m| m.max(n)));
        }
    }

    Ok(match max {
        Some(n) => n.saturating_add(1),
        None    => APCRH_HARVEST_SEED_INDEX,
    })
}

/// Write the text flavor to `{index}.txt` in `dir`.
pub(crate) fn zapcrh_write_text(dir: &Path, index: u32, text: &str) -> Result<(), String> {
    let path = dir.join(format!("{}.txt", index));
    let mut file = fs::File::create(&path)
        .map_err(|e| format!("create {}: {}", path.display(), e))?;
    file.write_all(text.as_bytes())
        .map_err(|e| format!("write {}: {}", path.display(), e))?;
    Ok(())
}

/// Write the HTML flavor to `{index}.html` in `dir`.
pub(crate) fn zapcrh_write_html(dir: &Path, index: u32, html: &str) -> Result<(), String> {
    let path = dir.join(format!("{}.html", index));
    let mut file = fs::File::create(&path)
        .map_err(|e| format!("create {}: {}", path.display(), e))?;
    file.write_all(html.as_bytes())
        .map_err(|e| format!("write {}: {}", path.display(), e))?;
    Ok(())
}
