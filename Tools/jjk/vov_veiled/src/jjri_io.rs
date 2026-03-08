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
/// 3. Recompute paddock_file from firemark for every heat (JJK owns this field)
/// 4. Paddock existence check: fatal with mv instructions if files are at legacy paths
/// 5. Semantic validation via jjrg_validate
///
/// Step 3 means the stored paddock_file value is ignored — JJK always derives it from
/// the firemark. After migration, the next save writes the encoded value back to disk.
///
/// Returns ValidatedGallops wrapper that guarantees the data has passed all checks.
pub fn jjdr_load(path: &Path) -> Result<jjdr_ValidatedGallops, String> {
    // Read original bytes
    let original_bytes = fs::read(path)
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;

    // Deserialize (mut: paddock_file will be recomputed below)
    let mut gallops: jjrg_Gallops = serde_json::from_slice(&original_bytes)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Detect old-format files: heat_order absent in JSON → serde default → empty vec.
    // Old-format files cannot pass round-trip check because BTreeMap serializes heats
    // in sorted key order, which differs from original furlough-shuffled order.
    let is_migration_mode = gallops.heat_order.is_empty();

    // Round-trip validation: run before recomputation to validate the stored format.
    // Skipped for old-format files (heat_order empty) — BTreeMap key order differs from
    // original IndexMap insertion order, so round-trip will always fail for these files.
    if !is_migration_mode {
        let reserialized = serde_json::to_string_pretty(&gallops)
            .map_err(|e| format!("Failed to reserialize JSON: {}", e))?;

        if reserialized.as_bytes() != original_bytes {
            let diff_pos = zjjdr_find_first_diff(&original_bytes, reserialized.as_bytes());
            return Err(format!("Round-trip validation failed at byte {}", diff_pos));
        }
    }

    // Recompute paddock_file from firemark for every heat.
    // JJK owns this field exclusively; whatever is stored in JSON is overwritten.
    // This allows old gallops files (with raw-firemark paths) to load transparently —
    // the correct encoded path is used in memory, and written back on the next save.
    for (firemark_key, heat) in &mut gallops.heats {
        let bare = firemark_key.trim_start_matches('₣');
        heat.paddock_file = jjri_paddock_path(bare);
    }

    // Populate heat_order from sorted heats keys on first load of old-format file.
    // BTreeMap guarantees sorted key order here.
    if is_migration_mode {
        gallops.heat_order = gallops.heats.keys().cloned().collect();
    }

    // Paddock existence check: verify each encoded paddock file exists on disk.
    // If missing, compute the legacy raw-firemark path and emit mv instructions.
    // Reports ALL missing files at once so a single repair pass suffices.
    {
        let mut repairs: Vec<String> = Vec::new();
        for (firemark_key, heat) in &gallops.heats {
            if !Path::new(&heat.paddock_file).exists() {
                let bare = firemark_key.trim_start_matches('₣');
                let legacy_path = format!(".claude/jjm/jjp_{}.md", bare);
                if Path::new(&legacy_path).exists() {
                    repairs.push(format!("  mv {} {}", legacy_path, heat.paddock_file));
                } else {
                    repairs.push(format!(
                        "  # WARNING: paddock missing for heat \"{}\", expected: {}",
                        bare, heat.paddock_file
                    ));
                }
            }
        }
        if !repairs.is_empty() {
            return Err(format!(
                "Paddock files need renaming before proceeding:\n\n{}",
                repairs.join("\n")
            ));
        }
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
