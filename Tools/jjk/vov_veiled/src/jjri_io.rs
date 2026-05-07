// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops I/O operations with validation
//!
//! Provides validated load/save operations for Gallops JSON with round-trip guarantees.

use std::fs;
use std::path::Path;
use crate::jjrt_types::jjrg_Gallops;
use crate::jjrv_validate::jjrg_validate;

/// Validated Gallops wrapper
///
/// This newtype ensures that Gallops instances are validated on load.
/// Only jjdr_load can create instances, guaranteeing all ValidatedGallops
/// have passed both deserialization and semantic validation.
pub struct jjdr_ValidatedGallops(jjrg_Gallops);

impl jjdr_ValidatedGallops {
    /// Access the inner Gallops (immutable)
    pub fn inner(&self) -> &jjrg_Gallops {
        &self.0
    }

    /// Access the inner Gallops (mutable)
    pub fn inner_mut(&mut self) -> &mut jjrg_Gallops {
        &mut self.0
    }

    /// Consume the wrapper and return the inner Gallops
    pub fn into_inner(self) -> jjrg_Gallops {
        self.0
    }

    /// Test-only constructor (bypasses validation)
    #[cfg(test)]
    pub fn test_wrap(g: jjrg_Gallops) -> Self {
        Self(g)
    }
}

/// Find first byte position where two byte slices differ
fn zjjdr_find_first_diff(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .position(|(x, y)| x != y)
        .unwrap_or_else(|| a.len().min(b.len()))
}

/// Encode a firemark string into a case-safe paddock filename path.
///
/// Each character is prefixed with 'u' (uppercase) or 'l' (lowercase),
/// ensuring the resulting filename is unambiguous on case-insensitive
/// filesystems (macOS HFS+/APFS).
///
/// Examples:
///   "AG" → ".claude/jjm/jjp_uAuG.md"
///   "Ag" → ".claude/jjm/jjp_uAlg.md"
///   "ag" → ".claude/jjm/jjp_lalg.md"
pub fn jjri_paddock_path(firemark: &str) -> String {
    let encoded: String = firemark.chars().map(|c| {
        if c.is_uppercase() {
            format!("u{}", c)
        } else {
            format!("l{}", c)
        }
    }).collect();
    format!(".claude/jjm/jjp_{}.md", encoded)
}

/// Load and validate Gallops from a file
///
/// Performs these steps in order:
/// 1. Deserialize JSON to Gallops struct
/// 2. Round-trip validation: reserialize and compare bytes (validates stored format)
/// 3. Semantic validation via jjrg_validate
///
/// Returns ValidatedGallops wrapper that guarantees the data has passed all checks.
pub fn jjdr_load(path: &Path) -> Result<jjdr_ValidatedGallops, String> {
    // Read original bytes
    let original_bytes = fs::read(path)
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;

    let mut gallops: jjrg_Gallops = serde_json::from_slice(&original_bytes)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Detect old-format files: heat_order absent or schema_version absent.
    // Old-format files cannot pass round-trip check because BTreeMap serializes heats
    // in sorted key order, which differs from original furlough-shuffled order.
    //
    // Also detect stale next_pensum_seed field (removed in v3.7 — pensum seeds moved
    // to officium-local storage). Serde silently ignores the unknown field on read,
    // but re-serialization omits it, causing round-trip byte mismatch until the next
    // save writes the clean format back to disk.
    //
    // Also detect stale paddock_file field (removed in v4 — paddock path now derived
    // from firemark via jjri_paddock_path on demand, not stored). Same auto-migration
    // pattern as next_pensum_seed.
    const PENSUM_SEED_KEY: &[u8] = b"\"next_pensum_seed\"";
    const PADDOCK_FILE_KEY: &[u8] = b"\"paddock_file\"";
    let needs_pensum_seed_removal = !gallops.heats.is_empty()
        && original_bytes.windows(PENSUM_SEED_KEY.len()).any(|w| w == PENSUM_SEED_KEY);
    let needs_paddock_file_removal = !gallops.heats.is_empty()
        && original_bytes.windows(PADDOCK_FILE_KEY.len()).any(|w| w == PADDOCK_FILE_KEY);
    let is_migration_mode = gallops.heat_order.is_empty()
        || gallops.schema_version.is_none()
        || needs_pensum_seed_removal
        || needs_paddock_file_removal;

    // Round-trip validation: skipped for migration-mode files because reserialized
    // output will differ from original (dropped fields, sorted key order).
    if !is_migration_mode {
        let reserialized = serde_json::to_string_pretty(&gallops)
            .map_err(|e| format!("Failed to reserialize JSON: {}", e))?;

        if reserialized.as_bytes() != original_bytes {
            let diff_pos = zjjdr_find_first_diff(&original_bytes, reserialized.as_bytes());
            return Err(format!("Round-trip validation failed at byte {}", diff_pos));
        }
    }

    // Populate missing fields on first load of old-format file.
    // BTreeMap guarantees sorted key order for heat_order.
    if is_migration_mode {
        if gallops.heat_order.is_empty() {
            gallops.heat_order = gallops.heats.keys().cloned().collect();
        }
        gallops.schema_version = Some(4);
    }

    // Semantic validation
    jjrg_validate(&gallops)
        .map_err(|errors| format!("Validation failed: {}", errors.join("; ")))?;

    Ok(jjdr_ValidatedGallops(gallops))
}

/// Save Gallops to a file with validation
///
/// Performs atomic write with validation:
/// 1. Serialize to JSON
/// 2. Write to temp file
/// 3. Validate temp file via jjdr_load
/// 4. Rename temp to target (atomic)
///
/// If validation fails, temp file is deleted and error returned.
pub fn jjdr_save(gallops: &jjrg_Gallops, path: &Path) -> Result<(), String> {
    // Serialize to pretty JSON
    let json = serde_json::to_string_pretty(gallops)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    // Create temp file in same directory for atomic rename
    let parent = path.parent().ok_or_else(|| "Invalid path: no parent directory".to_string())?;
    let temp_path = parent.join(format!(".tmp.{}.json", std::process::id()));

    // Write to temp file
    fs::write(&temp_path, &json)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;

    // Validate what we wrote by loading it back
    match jjdr_load(&temp_path) {
        Ok(_) => {
            // Validation passed, rename to target
            fs::rename(&temp_path, path)
                .map_err(|e| format!("Failed to rename temp file to target: {}", e))?;
            Ok(())
        }
        Err(e) => {
            // Validation failed, clean up temp file
            let _ = fs::remove_file(&temp_path);
            Err(format!("Save validation failed: {}", e))
        }
    }
}

/// Save Gallops and commit with paddock in a single operation
///
/// This is the standard routine for JJK operations that modify gallops.
/// It saves the gallops file, then commits both the gallops and the paddock file
/// for the given heat using machine_commit.
///
/// Returns the commit hash on success.
pub fn jjri_persist(
    lock: &vvc::vvcc_CommitLock,
    gallops: &jjrg_Gallops,
    file: &Path,
    firemark: &crate::jjrf_favor::jjrf_Firemark,
    message: String,
    size_limit: u64,
    output: &mut vvc::vvco_Output,
) -> Result<String, String> {
    // Save gallops first
    jjdr_save(gallops, file)?;

    // Construct paths for commit
    let gallops_path = file.to_string_lossy().to_string();
    let paddock_path = jjri_paddock_path(firemark.jjrf_as_str());

    // Commit using machine_commit with explicit file list
    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![
            gallops_path,
            paddock_path,
        ],
        message,
        size_limit,
        warn_limit: 30000,
    };

    vvc::machine_commit(lock, &commit_args, output)
}
