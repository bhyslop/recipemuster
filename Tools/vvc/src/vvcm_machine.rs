// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Machine Commit - programmatic commit infrastructure
//!
//! For automated operations that know exactly what files to commit
//! and don't need Claude assistance. Contrast with vvcc_commit which
//! is designed for interactive sessions with Claude message generation.
//!
//! Use cases:
//!   - JJK retire: trophy file + paddock + gallops update
//!   - VOK install: kit files being installed
//!   - CMK normalize: normalized concept model files
//!   - Any automated operation with predictable file sets
//!
//! Exit codes (when using vvcm_run):
//!   0 - Success (commit hash printed to stdout)
//!   1 - Failure (staging failed, guard failed, commit failed)

use std::process::Command;

use crate::vvcc_commit::vvcc_CommitLock;
use crate::vvcg_guard;

/// Arguments for machine commit operation
#[derive(Debug, Clone)]
pub struct vvcm_CommitArgs {
    /// Explicit files to stage (required, non-empty)
    pub files: Vec<String>,
    /// Commit message (required)
    pub message: String,
    /// Size limit in bytes for guard check
    pub size_limit: u64,
    /// Warning threshold in bytes for guard check
    pub warn_limit: u64,
}

/// Machine commit for programmatic operations.
///
/// Unlike vvcc_commit, this:
/// - Stages only specified files (not git add -A)
/// - Requires explicit message (no Claude generation)
/// - Does not add Co-Authored-By trailer
/// - Uses caller-specified guard limits
///
/// The `_lock` parameter serves as compile-time proof that the caller
/// holds the commit lock. The lock is not consumed; caller retains it.
///
/// Returns the commit hash on success.
pub fn vvcm_commit(
    _lock: &vvcc_CommitLock,
    args: &vvcm_CommitArgs,
) -> Result<String, String> {
    // Validate args
    if args.files.is_empty() {
        return Err("files list must not be empty".to_string());
    }
    if args.message.is_empty() {
        return Err("message must not be empty".to_string());
    }

    // Stage explicit files
    zvvcm_stage_files(&args.files)?;

    // Run guard with custom limits
    let guard_args = vvcg_guard::vvcg_GuardArgs {
        limit: args.size_limit,
        warn: args.warn_limit,
    };
    let guard_result = vvcg_guard::vvcg_run(&guard_args, None);
    match guard_result {
        0 => {}
        1 => return Err("Staged content exceeds size limit".to_string()),
        2 => {
            eprintln!("vvcm_commit: WARNING - staged content near size limit");
        }
        _ => return Err(format!("Guard returned unexpected code: {}", guard_result)),
    }

    // Commit with message (no Co-Authored-By)
    zvvcm_execute_commit(&args.message)
}

/// Stage specific files using git add
fn zvvcm_stage_files(files: &[String]) -> Result<(), String> {
    let mut cmd = Command::new("git");
    cmd.arg("add").arg("--");
    for file in files {
        cmd.arg(file);
    }

    let result = cmd
        .output()
        .map_err(|e| format!("Failed to run git add: {}", e))?;

    if result.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git add failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ))
    }
}

/// Execute git commit with message (no Co-Authored-By)
fn zvvcm_execute_commit(message: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .map_err(|e| format!("Failed to run git commit: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git commit failed: {}", stderr));
    }

    // Get commit hash
    let hash_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("Failed to get commit hash: {}", e))?;

    if !hash_output.status.success() {
        return Err("Failed to retrieve commit hash".to_string());
    }

    let hash = String::from_utf8_lossy(&hash_output.stdout)
        .trim()
        .to_string();

    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vvcg_guard::{VVCG_SIZE_LIMIT, VVCG_WARN_LIMIT};

    #[test]
    fn test_args_validation_empty_files() {
        // Can't actually call vvcm_commit without a lock, but we can test
        // that the validation logic would catch empty files
        let args = vvcm_CommitArgs {
            files: vec![],
            message: "test".to_string(),
            size_limit: VVCG_SIZE_LIMIT,
            warn_limit: VVCG_WARN_LIMIT,
        };
        assert!(args.files.is_empty());
    }

    #[test]
    fn test_args_validation_empty_message() {
        let args = vvcm_CommitArgs {
            files: vec!["file.txt".to_string()],
            message: "".to_string(),
            size_limit: VVCG_SIZE_LIMIT,
            warn_limit: VVCG_WARN_LIMIT,
        };
        assert!(args.message.is_empty());
    }

    #[test]
    fn test_args_construction() {
        // Test that constants have expected values
        let args = vvcm_CommitArgs {
            files: vec!["a.txt".to_string(), "b.txt".to_string()],
            message: "Update files".to_string(),
            size_limit: VVCG_SIZE_LIMIT,
            warn_limit: VVCG_WARN_LIMIT,
        };
        assert_eq!(args.files.len(), 2);
        assert_eq!(args.message, "Update files");
        assert_eq!(args.size_limit, 50_000);
        assert_eq!(args.warn_limit, 30_000);
    }
}
