// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Guard - Pre-commit storage cost validation
//!
//! Prevents accidental commits that significantly increase repository storage
//! costs. Guards against: release tarballs, build artifacts, node_modules,
//! accidentally added binaries.
//!
//! # Cost Model
//!
//! GitHub charges by repository size, which reflects Git packfile storage
//! with delta compression. The guard approximates incremental storage cost:
//!
//! | Change Type        | Storage Cost    | Measurement           |
//! |--------------------|-----------------|----------------------|
//! | New file           | ~Blob size      | `git cat-file -s`    |
//! | Modified text      | ~Diff size      | `git diff --cached`  |
//! | Modified binary    | ~Blob size      | `git cat-file -s`    |
//! | Deleted file       | 0               | (no measurement)     |
//!
//! # Key Behaviors
//!
//! - A 6-line edit to a 350KB JSON file costs ~200 bytes, not 350KB
//! - A new 100KB tarball costs 100KB (no delta possible)
//! - A modified PNG costs full blob size (binary delta is poor)
//!
//! # Limits
//!
//! Standard limits are defined as constants. All callers must explicitly
//! specify limits - there is no Default impl to avoid hidden assumptions.
//!
//! Exit codes:
//!   0 - Under limit (OK)
//!   1 - Over limit (BLOCKED)
//!   2 - Over warn threshold (WARNING)

use std::process::Command;

/// Standard size limit for guard check (50KB)
///
/// Blocks commits where incremental storage cost exceeds this threshold.
/// Use this constant at all call sites for consistency.
pub const VVCG_SIZE_LIMIT: u64 = 50_000;

/// Standard warning threshold for guard check (30KB)
///
/// Warns when incremental storage cost exceeds this threshold.
/// Use this constant at all call sites for consistency.
pub const VVCG_WARN_LIMIT: u64 = 30_000;

/// Arguments for guard operation
///
/// No Default impl - callers must explicitly specify limits using
/// VVCG_SIZE_LIMIT and VVCG_WARN_LIMIT constants.
#[derive(Debug, Clone)]
pub struct vvcg_GuardArgs {
    /// Size limit in bytes (use VVCG_SIZE_LIMIT)
    pub limit: u64,
    /// Warning threshold in bytes (use VVCG_WARN_LIMIT)
    pub warn: u64,
}

/// Entry for a staged file with its diff size
pub(crate) struct zvvcg_StagedFile {
    pub(crate) path: String,
    pub(crate) size: u64,
}

/// Get list of staged files with their diff sizes
fn zvvcg_get_staged_files() -> Result<Vec<zvvcg_StagedFile>, String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .map_err(|e| format!("Failed to run git diff: {}", e))?;

    if !output.status.success() {
        return Err("git diff --cached --name-only failed".to_string());
    }

    let paths_str = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();

    for path in paths_str.lines() {
        if path.is_empty() {
            continue;
        }

        let size = zvvcg_get_diff_size(path)?;
        files.push(zvvcg_StagedFile {
            path: path.to_string(),
            size,
        });
    }

    Ok(files)
}

/// Get size of staged diff for a specific file
///
/// For new or modified files, returns the actual blob size in the staging area.
/// For deleted files, returns 0.
///
/// Uses git ls-files --cached -s to get the blob SHA, then git cat-file -s
/// to get the actual size. This correctly handles binary files, which would
/// otherwise be misreported by measuring git diff output length.
pub(crate) fn zvvcg_get_diff_size(path: &str) -> Result<u64, String> {
    // Get staged blob info: mode, sha, stage, path
    let output = Command::new("git")
        .args(["ls-files", "--cached", "-s", "--", path])
        .output()
        .map_err(|e| format!("Failed to run git ls-files for {}: {}", path, e))?;

    if !output.status.success() {
        return Err(format!("git ls-files --cached -s -- {} failed", path));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.trim();

    // If file is deleted (not in index), it has no size
    if line.is_empty() {
        return Ok(0);
    }

    // Parse: "100644 <sha> 0\t<path>"
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(format!("Unexpected git ls-files output for {}: {}", path, line));
    }

    let blob_sha = parts[1];

    // Get actual blob size
    let output = Command::new("git")
        .args(["cat-file", "-s", blob_sha])
        .output()
        .map_err(|e| format!("Failed to run git cat-file for {}: {}", blob_sha, e))?;

    if !output.status.success() {
        return Err(format!("git cat-file -s {} failed", blob_sha));
    }

    let size_str = String::from_utf8_lossy(&output.stdout);
    size_str
        .trim()
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse blob size for {}: {}", path, e))
}

/// Run the guard check on staged content.
///
/// Returns:
/// - 0: Under limit (OK)
/// - 1: Over limit (BLOCKED)
/// - 2: Over warn threshold (WARNING)
pub fn vvcg_run(args: &vvcg_GuardArgs) -> i32 {
    let files = match zvvcg_get_staged_files() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("guard: error: {}", e);
            return 1;
        }
    };

    let total_size: u64 = files.iter().map(|f| f.size).sum();

    if total_size > args.limit {
        eprintln!(
            "guard: BLOCKED - staged content {} bytes exceeds limit {} bytes",
            total_size, args.limit
        );
        eprintln!();
        eprintln!("Breakdown by file:");
        let mut sorted_files = files;
        sorted_files.sort_by(|a, b| b.size.cmp(&a.size));
        for f in sorted_files.iter().take(10) {
            eprintln!("  {:>10} bytes  {}", f.size, f.path);
        }
        if sorted_files.len() > 10 {
            eprintln!("  ... and {} more files", sorted_files.len() - 10);
        }
        return 1;
    }

    if total_size > args.warn {
        eprintln!(
            "guard: WARNING - staged content {} bytes exceeds warning threshold {} bytes",
            total_size, args.warn
        );
        return 2;
    }

    eprintln!(
        "guard: OK - staged content {} bytes (limit: {})",
        total_size, args.limit
    );
    0
}
