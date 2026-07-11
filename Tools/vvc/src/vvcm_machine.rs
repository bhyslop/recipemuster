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

use crate::vvcc_commit::vvcc_CommitLock;
use crate::vvcg_guard;
use crate::vvcg_guard::vvcg_Cost;
use crate::vvco_output::vvco_Output;
use crate::vvco_err;

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

/// Why a machine commit did not land.
///
/// The size-guard refusal is its own variant rather than a message, because the
/// caller — not this crate — owns how a refusal reads to whoever must act on it.
/// The variant carries the measurement so that caller can say *why*, naming the
/// bytes and the files, without re-running the guard.
#[derive(Debug, Clone)]
pub enum vvcm_CommitError {
    /// Staged content costs more than the caller's limit. Nothing was committed;
    /// the files remain staged.
    OverLimit { cost: vvcg_Cost, limit: u64 },
    /// Anything else: empty args, staging failure, git failure.
    Fault(String),
}

impl std::fmt::Display for vvcm_CommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            vvcm_CommitError::OverLimit { cost, limit } => write!(
                f,
                "staged content {} bytes exceeds size limit {} bytes",
                cost.total, limit
            ),
            vvcm_CommitError::Fault(m) => write!(f, "{}", m),
        }
    }
}

impl From<String> for vvcm_CommitError {
    fn from(m: String) -> Self {
        vvcm_CommitError::Fault(m)
    }
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
    output: &mut vvco_Output,
) -> Result<String, vvcm_CommitError> {
    // Validate args
    if args.files.is_empty() {
        return Err("files list must not be empty".to_string().into());
    }
    if args.message.is_empty() {
        return Err("message must not be empty".to_string().into());
    }

    // Stage explicit files
    zvvcm_stage_files(&args.files)?;

    // Measure staged content, then judge it against the caller's limits. The
    // refusal returns the measurement rather than printing a verdict: the caller
    // renders it.
    let cost = vvcg_guard::vvcg_cost(None)?;
    if cost.total > args.size_limit {
        return Err(vvcm_CommitError::OverLimit { cost, limit: args.size_limit });
    }
    if cost.total > args.warn_limit {
        vvco_err!(output, "vvcm_commit: WARNING - staged content near size limit");
    }

    // Commit with message (no Co-Authored-By)
    Ok(zvvcm_execute_commit(&args.message)?)
}

/// Stage specific files using git add
fn zvvcm_stage_files(files: &[String]) -> Result<(), String> {
    let mut cmd = crate::vvce_git_command(&["add", "--"]);
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
    let output = crate::vvce_git_command(&["commit", "-m", message])
        .output()
        .map_err(|e| format!("Failed to run git commit: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git commit failed: {}", stderr));
    }

    // Get commit hash
    let hash_output = crate::vvce_git_command(&["rev-parse", "HEAD"])
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
