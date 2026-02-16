// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Commit - Core commit infrastructure
//!
//! Provides atomic commit workflow: lock, stage, guard, commit.
//! Used by vvx commit and JJK commands for consistent commit handling.
//!
//! ## Usage Patterns
//!
//! ### Simple commit (lock acquired and released automatically):
//! ```ignore
//! let exit_code = vvc::commit(&args);
//! ```
//!
//! ### Protected operation (hold lock across multiple steps):
//! ```ignore
//! let lock = vvc::vvcc_CommitLock::vvcc_acquire()?;
//! // ... do work while holding lock ...
//! let hash = lock.vvcc_commit(&args)?;
//! // lock released when dropped
//! ```
//!
//! Exit codes:
//!   0 - Success (commit hash printed to stdout)
//!   1 - Failure (lock held, guard failed, claude failed, commit failed)

use std::process::Command;

use crate::vvcg_guard;

/// Lock reference path for commit operations
const VVCC_LOCK_REF: &str = "refs/vvg/locks/vvx";

/// Arguments for commit operation
///
/// Note: No Default impl - callers must explicitly specify all fields,
/// including size limits. Use VVCG_SIZE_LIMIT/VVCG_WARN_LIMIT constants
/// for standard behavior.
#[derive(Debug, Clone)]
pub struct vvcc_CommitArgs {
    /// Prefix string to prepend to commit message (e.g., "[jj:BRAND][F00/silks]")
    pub prefix: Option<String>,
    /// Commit message; if absent, invoke claude to generate from diff
    pub message: Option<String>,
    /// Allow empty commits (for chalk markers)
    pub allow_empty: bool,
    /// Skip 'git add -A' (respect pre-staged files)
    pub no_stage: bool,
    /// Size limit in bytes (use VVCG_SIZE_LIMIT for standard behavior)
    pub size_limit: u64,
    /// Warning threshold in bytes (use VVCG_WARN_LIMIT for standard behavior)
    pub warn_limit: u64,
}

/// RAII guard that holds the commit lock.
///
/// The lock is automatically released when the guard is dropped, ensuring
/// no code path can leak the lock. Use this when you need to hold the lock
/// across multiple operations (e.g., modify gallops, then commit).
///
/// ## Example
/// ```ignore
/// let lock = vvcc_CommitLock::vvcc_acquire()?;
/// // Do protected work here...
/// let hash = lock.vvcc_commit(&args)?;
/// // Lock released when `lock` goes out of scope
/// ```
pub struct vvcc_CommitLock {
    /// Private field prevents external construction
    _private: (),
}

impl vvcc_CommitLock {
    /// Acquire the commit lock.
    ///
    /// Returns `Err` if the lock is already held by another operation.
    /// The lock is held until this `vvcc_CommitLock` is dropped.
    pub fn vvcc_acquire() -> Result<Self, String> {
        zvvcc_acquire_lock()?;
        Ok(vvcc_CommitLock { _private: () })
    }

    /// Perform a commit while holding the lock.
    ///
    /// This does NOT release the lock - the caller retains the guard
    /// and can perform additional operations if needed.
    ///
    /// Returns the commit hash on success.
    pub fn vvcc_commit(&self, args: &vvcc_CommitArgs) -> Result<String, String> {
        zvvcc_run_commit_workflow(args)
    }
}

impl Drop for vvcc_CommitLock {
    fn drop(&mut self) {
        zvvcc_release_lock();
    }
}

/// Acquire the commit lock using git update-ref
fn zvvcc_acquire_lock() -> Result<(), String> {
    let head_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("Failed to run git rev-parse: {}", e))?;

    let lock_value = if head_output.status.success() {
        String::from_utf8_lossy(&head_output.stdout).trim().to_string()
    } else {
        "0000000000000000000000000000000000000000".to_string()
    };

    let result = Command::new("git")
        .args(["update-ref", VVCC_LOCK_REF, &lock_value, ""])
        .output()
        .map_err(|e| format!("Failed to run git update-ref: {}", e))?;

    if result.status.success() {
        Ok(())
    } else {
        Err("Another commit in progress - lock held".to_string())
    }
}

/// Release the commit lock
fn zvvcc_release_lock() {
    let _ = Command::new("git")
        .args(["update-ref", "-d", VVCC_LOCK_REF])
        .output();
}

/// Stage all changes including untracked files with 'git add -A'
fn zvvcc_stage_changes() -> Result<(), String> {
    let result = Command::new("git")
        .args(["add", "-A"])
        .output()
        .map_err(|e| format!("Failed to run git add: {}", e))?;

    if result.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git add -A failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ))
    }
}

/// Check if there are staged changes
fn zvvcc_has_staged_changes() -> Result<bool, String> {
    let result = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .output()
        .map_err(|e| format!("Failed to run git diff: {}", e))?;

    Ok(!result.status.success())
}

/// Run size guard check on staged content
fn zvvcc_run_guard(args: &vvcc_CommitArgs) -> Result<(), String> {
    let guard_args = vvcg_guard::vvcg_GuardArgs {
        limit: args.size_limit,
        warn: args.warn_limit,
    };

    let result = vvcg_guard::vvcg_run(&guard_args, None);

    match result {
        0 => Ok(()),
        1 => Err("Staged content exceeds size limit".to_string()),
        2 => {
            eprintln!("commit: WARNING - staged content near size limit");
            Ok(())
        }
        _ => Err(format!("Guard returned unexpected code: {}", result)),
    }
}

/// Get the staged diff for commit message generation
fn zvvcc_get_staged_diff() -> Result<String, String> {
    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .map_err(|e| format!("Failed to run git diff: {}", e))?;

    if !output.status.success() {
        return Err("git diff --cached failed".to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Generate commit message using claude CLI
fn zvvcc_generate_message_with_claude(diff: &str) -> Result<String, String> {
    let prompt = format!(
        "Generate a concise commit message for this diff:\n\n<diff>\n{}\n</diff>\n\nRespond with only the commit message, no explanation.",
        diff
    );

    let output = crate::vvce_claude_command()
        .args([
            "--print",
            "--system-prompt",
            "Output only a conventional git commit message. No explanation or commentary. Do not wrap in markdown code blocks.",
            "--model",
            "haiku",
            "--no-session-persistence",
            &prompt,
        ])
        .output()
        .map_err(|e| format!("Failed to invoke claude: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("claude --print failed: {}", stderr));
    }

    let message = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Strip markdown code blocks if Claude wrapped the response
    let message = if message.starts_with("```") {
        let after_opening = message.find('\n').map(|i| i + 1).unwrap_or(0);
        let before_closing = message.rfind("```").unwrap_or(message.len());
        if before_closing > after_opening {
            message[after_opening..before_closing].trim().to_string()
        } else {
            message
        }
    } else {
        message
    };

    // Strip Co-Authored-By trailer if Claude CLI added it from global settings
    let message: String = message
        .lines()
        .filter(|line| !line.starts_with("Co-Authored-By:"))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    if message.is_empty() {
        return Err("claude returned empty message".to_string());
    }

    Ok(message)
}

/// Format the full commit message with prefix and co-author
fn zvvcc_format_commit_message(prefix: Option<&str>, message: &str) -> String {
    let mut full_message = String::new();

    if let Some(p) = prefix {
        full_message.push_str(p);
    }

    full_message.push_str(message);

    full_message
}

/// Execute the git commit
fn zvvcc_execute_commit(message: &str, allow_empty: bool) -> Result<String, String> {
    let mut args = vec!["commit", "-m", message];

    if allow_empty {
        args.push("--allow-empty");
    }

    let output = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run git commit: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git commit failed: {}", stderr));
    }

    let hash_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("Failed to get commit hash: {}", e))?;

    if !hash_output.status.success() {
        return Err("Failed to retrieve commit hash".to_string());
    }

    Ok(String::from_utf8_lossy(&hash_output.stdout)
        .trim()
        .to_string())
}

/// Run the commit workflow.
///
/// Returns exit code: 0 for success, 1 for failure.
/// On success, prints commit hash to stdout.
///
/// This is the simple API that acquires and releases the lock automatically.
/// For operations that need to hold the lock across multiple steps, use
/// `vvcc_CommitLock::vvcc_acquire()` instead.
pub fn vvcc_run(args: &vvcc_CommitArgs) -> i32 {
    let lock = match vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("commit: error: {}", e);
            return 1;
        }
    };

    match lock.vvcc_commit(args) {
        Ok(hash) => {
            println!("{}", hash);
            0
        }
        Err(e) => {
            eprintln!("commit: error: {}", e);
            1
        }
    }
    // lock dropped here, releasing the git ref lock
}

/// Inner workflow that can return Result for cleaner error handling
fn zvvcc_run_commit_workflow(args: &vvcc_CommitArgs) -> Result<String, String> {
    if !args.no_stage {
        zvvcc_stage_changes()?;
    }

    if !args.allow_empty && !zvvcc_has_staged_changes()? {
        return Err("Nothing to commit".to_string());
    }

    zvvcc_run_guard(args)?;

    let message = match &args.message {
        Some(m) => m.clone(),
        None => {
            let diff = zvvcc_get_staged_diff()?;
            if diff.is_empty() && !args.allow_empty {
                return Err("No diff to generate message from".to_string());
            }
            zvvcc_generate_message_with_claude(&diff)?
        }
    };

    let full_message = zvvcc_format_commit_message(args.prefix.as_deref(), &message);

    zvvcc_execute_commit(&full_message, args.allow_empty)
}
