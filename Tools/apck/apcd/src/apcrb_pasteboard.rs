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

//! Pasteboard — macOS NSPasteboard FFI that enumerates the **declared**
//! UTIs of the general pasteboard's first item and writes each declared
//! flavor's raw bytes to the journal directory as `{N}-in.{tag}.{ext}`.
//! "Declared" means the set the producer actually wrote; macOS may
//! synthesize additional derived types on read, and those are deliberately
//! skipped — the goal is parity with `osascript -e 'clipboard info'`,
//! which reports declared types only.
//!
//! The classifier path (arboard `get_text()` in `apcap_main`) is untouched
//! by this module — `apcrb` writes to disk, it does not feed detection.
//!
//! UTI-to-extension mapping is data-driven: known public UTIs get
//! human-readable extensions, anything else falls through to a sanitized
//! `{uti}.bin` form so the flavor space extends without code changes.
//!
//! FFI surface is confined to `apcrb_capture_declared_flavors`. Nothing in
//! this module is `unsafe` today — `objc2-app-kit`'s `generalPasteboard`,
//! `pasteboardItems`, `NSPasteboardItem::types`, and `dataForType` are all
//! safe wrappers in 0.3.2. If a future refactor needs raw pointer access
//! (e.g. to avoid a byte-vec copy), keep the `unsafe` block inside this
//! file only.

use std::fs;
use std::io::Write;
use std::path::Path;

use crate::apcrh_harvest::apcrh_Classification;

#[cfg(target_os = "macos")]
use objc2_app_kit::NSPasteboard;

/// Capture every declared pasteboard flavor of the general pasteboard's
/// first item to `{index}-in.{tag}.{ext}` files in `dir`. Returns the
/// filenames written (empty if the pasteboard has no items or we are not on
/// macOS). Errors are logged via `apcrl_error_now!` at the FFI boundary
/// before being returned so the caller can decide on fallback without
/// losing the diagnostic detail.
#[cfg(target_os = "macos")]
pub fn apcrb_capture_declared_flavors(
    dir:            &Path,
    classification: apcrh_Classification,
    index:          u32,
) -> Result<Vec<String>, String> {
    fs::create_dir_all(dir)
        .map_err(|e| {
            let msg = format!("create harvest dir {}: {}", dir.display(), e);
            crate::apcrl_error_now!("apcrb: {}", msg);
            msg
        })?;

    let tag = classification.apcrh_tag();
    let pasteboard = NSPasteboard::generalPasteboard();

    let items = match pasteboard.pasteboardItems() {
        Some(items) => items,
        None        => {
            crate::apcrl_error_now!("apcrb: pasteboardItems returned nil");
            return Err("pasteboardItems nil".to_string());
        }
    };

    // By Apple convention the first item is the producer's primary write.
    // Copy All from Epic produces exactly one item; enumerating beyond it
    // invites macOS-synthesized derivatives we've explicitly scoped out.
    // If the pasteboard has zero items — despite arboard in the classifier
    // path having successfully read text from it moments ago — that is a
    // genuine disagreement worth surfacing, not a soft success.
    let first_item = match items.iter().next() {
        Some(item) => item,
        None       => {
            crate::apcrl_error_now!("apcrb: pasteboard has 0 items");
            return Err("pasteboard has 0 items".to_string());
        }
    };

    let types = first_item.types();
    let mut filenames: Vec<String> = Vec::new();

    for uti in types.iter() {
        let uti_str = uti.to_string();

        let data = match first_item.dataForType(&uti) {
            Some(d) => d,
            None    => {
                crate::apcrl_error_now!(
                    "apcrb: dataForType({}) returned nil — declared but empty",
                    uti_str
                );
                continue;
            }
        };

        let ext = apcrb_extension_for_uti(&uti_str);
        let filename = format!("{}-in.{}.{}", index, tag, ext);
        let path = dir.join(&filename);

        let bytes = data.to_vec();
        let mut file = match fs::File::create(&path) {
            Ok(f)  => f,
            Err(e) => {
                let msg = format!("create {}: {}", path.display(), e);
                crate::apcrl_error_now!("apcrb: {}", msg);
                return Err(msg);
            }
        };
        if let Err(e) = file.write_all(&bytes) {
            let msg = format!("write {} ({} bytes): {}", path.display(), bytes.len(), e);
            crate::apcrl_error_now!("apcrb: {}", msg);
            return Err(msg);
        }

        crate::apcrl_info_now!(
            "apcrb: wrote {} ({} bytes, uti={})",
            filename, bytes.len(), uti_str
        );
        filenames.push(filename);
    }

    Ok(filenames)
}

/// Non-macOS stub — preserves cross-platform compilation but returns an
/// honest error. Harvest is Mac-first; a future pace adds Windows
/// pasteboard enumeration and replaces this arm.
#[cfg(not(target_os = "macos"))]
pub fn apcrb_capture_declared_flavors(
    _dir:           &Path,
    _classification: apcrh_Classification,
    _index:         u32,
) -> Result<Vec<String>, String> {
    Err("apcrb: NSPasteboard FFI is macOS-only".to_string())
}

/// Map an NSPasteboard UTI to a filename extension. Known public UTIs
/// receive a human-readable extension; legacy NSPasteboard names and
/// anything else fall through to `{sanitized}.bin` so the flavor space
/// extends without code changes. The returned string does **not** include a
/// leading dot — callers compose `{N}-in.{tag}.{ext}` directly.
pub fn apcrb_extension_for_uti(uti: &str) -> String {
    match uti {
        "public.rtf"              => "rtf".to_string(),
        "public.utf8-plain-text"  => "utf8.txt".to_string(),
        "public.utf16-plain-text" => "utf16.txt".to_string(),
        "public.plain-text"       => "txt".to_string(),
        "public.html"             => "html".to_string(),
        "public.tiff"             => "tiff".to_string(),
        "public.png"              => "png".to_string(),
        "public.jpeg"             => "jpeg".to_string(),
        "public.url"              => "url".to_string(),
        "public.file-url"         => "fileurl".to_string(),
        _                         => format!("{}.bin", zapcrb_sanitize_uti(uti)),
    }
}

/// Reduce a UTI (or legacy pasteboard type name) to a filename-safe token.
/// ASCII alphanumerics and hyphens pass through unchanged; everything else
/// — dots, slashes, colons, whitespace, punctuation — collapses to a
/// single hyphen. Empty input yields `"flavor"` as a last-resort placeholder
/// so the output always has at least one character before the `.bin`.
fn zapcrb_sanitize_uti(uti: &str) -> String {
    let mut out = String::with_capacity(uti.len());
    let mut last_was_hyphen = false;
    for c in uti.chars() {
        let keep = matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-');
        if keep {
            out.push(c);
            last_was_hyphen = c == '-';
        } else if !last_was_hyphen {
            out.push('-');
            last_was_hyphen = true;
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() { "flavor".to_string() } else { trimmed }
}

#[cfg(test)]
pub(crate) fn zapcrb_sanitize_uti_for_test(uti: &str) -> String {
    zapcrb_sanitize_uti(uti)
}
