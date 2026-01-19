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

/// Load and validate Gallops from a file
///
/// Performs three validation steps:
/// 1. Deserialize JSON to Gallops struct
/// 2. Round-trip validation: reserialize and compare bytes
/// 3. Semantic validation via jjrg_validate
///
/// Returns ValidatedGallops wrapper that guarantees the data has passed all checks.
pub fn jjdr_load(path: &Path) -> Result<jjdr_ValidatedGallops, String> {
    // Read original bytes
    let original_bytes = fs::read(path)
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;

    // Deserialize
    let gallops: jjrg_Gallops = serde_json::from_slice(&original_bytes)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Round-trip validation: ensure canonical representation
    let reserialized = serde_json::to_string_pretty(&gallops)
        .map_err(|e| format!("Failed to reserialize JSON: {}", e))?;

    if reserialized.as_bytes() != original_bytes {
        let diff_pos = zjjdr_find_first_diff(&original_bytes, reserialized.as_bytes());
        return Err(format!("Round-trip validation failed at byte {}", diff_pos));
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
