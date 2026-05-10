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

use crate::vvcg_guard;
use crate::vvco_output::vvco_Output;
use crate::{vvco_out, vvco_err};
use std::path::Path;

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

/// Arguments for a marker commit.
///
/// A marker commit creates an empty commit (same tree as parent) without
/// staging changes, running the size guard, or invoking claude. The index
/// is left untouched — any pre-staged content remains staged afterward.
///
/// Use cases: officium invitatory, landing markers, chalk markers — all
/// commits whose purpose is to record an event, not to author content.
#[derive(Debug, Clone)]
pub struct vvcc_MarkerArgs {
    /// Prefix string to prepend to commit message (e.g., "[jj:BRAND][F00/silks]")
    pub prefix: Option<String>,
    /// Commit message subject (full body — no claude generation for markers)
    pub message: String,
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
    pub fn vvcc_commit(&self, args: &vvcc_CommitArgs, output: &mut vvco_Output) -> Result<String, String> {
        zvvcc_run_commit_workflow(args, output)
    }

    /// Create a marker commit while holding the lock.
    ///
    /// Index-invariant: skips staging, skips size guard, and writes the
    /// commit via plumbing (`git commit-tree HEAD^{tree} -p HEAD` +
    /// `git update-ref HEAD`). Pre-staged content is preserved in the index.
    ///
    /// Returns the new commit hash on success.
    pub fn vvcc_marker(&self, args: &vvcc_MarkerArgs) -> Result<String, String> {
        zvvcc_run_marker_workflow(args, None)
    }
}

impl Drop for vvcc_CommitLock {
    fn drop(&mut self) {
        zvvcc_release_lock();
    }
}

/// Acquire the commit lock using git update-ref
fn zvvcc_acquire_lock() -> Result<(), String> {
    let head_output = crate::vvce_git_command(&["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("Failed to run git rev-parse: {}", e))?;

    let lock_value = if head_output.status.success() {
        String::from_utf8_lossy(&head_output.stdout).trim().to_string()
    } else {
        "0000000000000000000000000000000000000000".to_string()
    };

    let result = crate::vvce_git_command(&["update-ref", VVCC_LOCK_REF, &lock_value, ""])
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
    let _ = crate::vvce_git_command(&["update-ref", "-d", VVCC_LOCK_REF])
        .output();
}

/// Stage all changes including untracked files with 'git add -A'
fn zvvcc_stage_changes() -> Result<(), String> {
    let result = crate::vvce_git_command(&["add", "-A"])
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
    let result = crate::vvce_git_command(&["diff", "--cached", "--quiet"])
        .output()
        .map_err(|e| format!("Failed to run git diff: {}", e))?;

    Ok(!result.status.success())
}

/// Run size guard check on staged content
fn zvvcc_run_guard(args: &vvcc_CommitArgs, output: &mut vvco_Output) -> Result<(), String> {
    let guard_args = vvcg_guard::vvcg_GuardArgs {
        limit: args.size_limit,
        warn: args.warn_limit,
    };

    let result = vvcg_guard::vvcg_run(&guard_args, None, output);

    match result {
        0 => Ok(()),
        1 => Err("Staged content exceeds size limit".to_string()),
        2 => {
            vvco_err!(output, "commit: WARNING - staged content near size limit");
            Ok(())
        }
        _ => Err(format!("Guard returned unexpected code: {}", result)),
    }
}

/// Get the staged diff for commit message generation
fn zvvcc_get_staged_diff() -> Result<String, String> {
    let output = crate::vvce_git_command(&["diff", "--cached"])
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

    let output = crate::vvce_git_command(&args)
        .output()
        .map_err(|e| format!("Failed to run git commit: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git commit failed: {}", stderr));
    }

    let hash_output = crate::vvce_git_command(&["rev-parse", "HEAD"])
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
pub fn vvcc_run(args: &vvcc_CommitArgs, output: &mut vvco_Output) -> i32 {
    let lock = match vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "commit: error: {}", e);
            return 1;
        }
    };

    match lock.vvcc_commit(args, output) {
        Ok(hash) => {
            vvco_out!(output, "{}", hash);
            0
        }
        Err(e) => {
            vvco_err!(output, "commit: error: {}", e);
            1
        }
    }
    // lock dropped here, releasing the git ref lock
}

/// Inner workflow that can return Result for cleaner error handling
fn zvvcc_run_commit_workflow(args: &vvcc_CommitArgs, output: &mut vvco_Output) -> Result<String, String> {
    if !args.no_stage {
        zvvcc_stage_changes()?;
    }

    if !args.allow_empty && !zvvcc_has_staged_changes()? {
        return Err("Nothing to commit".to_string());
    }

    zvvcc_run_guard(args, output)?;

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

/// Run a marker commit with auto-acquired lock.
///
/// Returns 0 on success (commit hash printed to output), 1 on failure.
///
/// The simple API for callers that don't need to hold the lock across
/// additional operations. For protected sequences, use
/// `vvcc_CommitLock::vvcc_acquire()` + `lock.vvcc_marker(...)`.
pub fn vvcc_marker_run(args: &vvcc_MarkerArgs, output: &mut vvco_Output) -> i32 {
    let lock = match vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "marker: error: {}", e);
            return 1;
        }
    };

    match lock.vvcc_marker(args) {
        Ok(hash) => {
            vvco_out!(output, "{}", hash);
            0
        }
        Err(e) => {
            vvco_err!(output, "marker: error: {}", e);
            1
        }
    }
}

/// Inner marker workflow. `repo_dir` lets tests target a temp repo
/// without disturbing the process cwd.
fn zvvcc_run_marker_workflow(args: &vvcc_MarkerArgs, repo_dir: Option<&Path>) -> Result<String, String> {
    let full_message = zvvcc_format_commit_message(args.prefix.as_deref(), &args.message);
    zvvcc_execute_marker(&full_message, repo_dir)
}

/// Execute the marker commit via plumbing.
///
/// Uses `git commit-tree HEAD^{tree} -p HEAD -m <msg>` to mint a commit
/// whose tree is identical to the parent's tree, regardless of index
/// state. Then `git update-ref HEAD <new> <parent>` advances the branch
/// with an old-sha guard. The index is never read or written.
fn zvvcc_execute_marker(message: &str, repo_dir: Option<&Path>) -> Result<String, String> {
    let tree = zvvcc_git_capture(&["rev-parse", "HEAD^{tree}"], repo_dir)?;
    let parent = zvvcc_git_capture(&["rev-parse", "HEAD"], repo_dir)?;

    let new_sha = zvvcc_git_capture(
        &["commit-tree", &tree, "-p", &parent, "-m", message],
        repo_dir,
    )?;

    zvvcc_git_capture(&["update-ref", "HEAD", &new_sha, &parent], repo_dir)?;

    Ok(new_sha)
}

/// Run a git command, optionally in a target dir, returning trimmed stdout.
fn zvvcc_git_capture(args: &[&str], repo_dir: Option<&Path>) -> Result<String, String> {
    let mut cmd = crate::vvce_git_command(args);
    if let Some(d) = repo_dir {
        cmd.current_dir(d);
    }
    let out = cmd
        .output()
        .map_err(|e| format!("git {}: spawn failed: {}", args.first().copied().unwrap_or(""), e))?;
    if !out.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.first().copied().unwrap_or(""),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn get_test_base() -> PathBuf {
        if let Ok(bud_temp) = std::env::var("BURD_TEMP_DIR") {
            let path = PathBuf::from(bud_temp);
            path.canonicalize().unwrap_or(path)
        } else {
            std::env::temp_dir()
        }
    }

    fn init_git_repo(dir: &Path) {
        crate::vvce_git_command(&["init", "-q"]).current_dir(dir).output().unwrap();
        crate::vvce_git_command(&["config", "user.name", "Test User"]).current_dir(dir).output().unwrap();
        crate::vvce_git_command(&["config", "user.email", "test@example.com"]).current_dir(dir).output().unwrap();
        crate::vvce_git_command(&["config", "commit.gpgsign", "false"]).current_dir(dir).output().unwrap();
        fs::write(dir.join("README"), "init\n").unwrap();
        crate::vvce_git_command(&["add", "README"]).current_dir(dir).output().unwrap();
        crate::vvce_git_command(&["commit", "-q", "-m", "init"]).current_dir(dir).output().unwrap();
    }

    fn git_capture(dir: &Path, args: &[&str]) -> String {
        let out = crate::vvce_git_command(args).current_dir(dir).output().unwrap();
        assert!(out.status.success(), "git {:?} failed: {}", args, String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    /// A marker commit must produce an empty commit AND leave pre-staged
    /// content in the index untouched.
    #[test]
    fn vvtc_marker_index_invariance() {
        let temp_dir = get_test_base().join("vvtc_marker_index_invariance");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        init_git_repo(&temp_dir);

        fs::write(temp_dir.join("dirty.txt"), "pre-staged\n").unwrap();
        crate::vvce_git_command(&["add", "dirty.txt"]).current_dir(&temp_dir).output().unwrap();

        let pre_index_diff = git_capture(&temp_dir, &["diff", "--cached"]);
        assert!(!pre_index_diff.is_empty(), "test setup: dirty.txt should be staged");

        let parent_sha = git_capture(&temp_dir, &["rev-parse", "HEAD"]);
        let parent_tree = git_capture(&temp_dir, &["rev-parse", "HEAD^{tree}"]);

        let args = vvcc_MarkerArgs {
            prefix: None,
            message: "marker test".to_string(),
        };
        let new_sha = zvvcc_run_marker_workflow(&args, Some(&temp_dir))
            .expect("marker workflow should succeed");

        let head_after = git_capture(&temp_dir, &["rev-parse", "HEAD"]);
        assert_eq!(head_after, new_sha, "HEAD should advance to the marker sha");
        assert_ne!(head_after, parent_sha, "marker must produce a new commit");

        let new_tree = git_capture(&temp_dir, &["rev-parse", "HEAD^{tree}"]);
        assert_eq!(new_tree, parent_tree, "marker tree must equal parent tree (empty commit)");

        let parents_line = git_capture(&temp_dir, &["rev-list", "--parents", "-n", "1", "HEAD"]);
        let parts: Vec<&str> = parents_line.split_whitespace().collect();
        assert_eq!(parts.len(), 2, "marker commit must have exactly one parent");
        assert_eq!(parts[1], parent_sha, "marker's parent must be the prior HEAD");

        let post_index_diff = git_capture(&temp_dir, &["diff", "--cached"]);
        assert_eq!(
            post_index_diff, pre_index_diff,
            "pre-staged content must survive marker (index-invariance)"
        );

        fs::remove_dir_all(&temp_dir).ok();
    }

    /// A marker commit must succeed even when the size guard would block
    /// the same staged content under a normal commit (guard skipped).
    #[test]
    fn vvtc_marker_skips_size_guard() {
        let temp_dir = get_test_base().join("vvtc_marker_skips_size_guard");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        init_git_repo(&temp_dir);

        let big = vec![b'x'; 200_000];
        fs::write(temp_dir.join("big.txt"), &big).unwrap();
        crate::vvce_git_command(&["add", "big.txt"]).current_dir(&temp_dir).output().unwrap();

        let parent_sha = git_capture(&temp_dir, &["rev-parse", "HEAD"]);

        let args = vvcc_MarkerArgs {
            prefix: None,
            message: "marker over staged 200KB".to_string(),
        };
        let new_sha = zvvcc_run_marker_workflow(&args, Some(&temp_dir))
            .expect("marker should ignore size guard");

        assert_ne!(new_sha, parent_sha);

        let post_index_diff = git_capture(&temp_dir, &["diff", "--cached", "--name-only"]);
        assert!(post_index_diff.contains("big.txt"), "200KB stays staged after marker");

        fs::remove_dir_all(&temp_dir).ok();
    }

    /// A marker commit applies the prefix exactly like a regular commit.
    #[test]
    fn vvtc_marker_applies_prefix() {
        let temp_dir = get_test_base().join("vvtc_marker_applies_prefix");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        init_git_repo(&temp_dir);

        let args = vvcc_MarkerArgs {
            prefix: Some("PREFIX:".to_string()),
            message: "body".to_string(),
        };
        zvvcc_run_marker_workflow(&args, Some(&temp_dir)).unwrap();

        let subject = git_capture(&temp_dir, &["log", "-1", "--pretty=%s"]);
        assert_eq!(subject, "PREFIX:body");

        fs::remove_dir_all(&temp_dir).ok();
    }
}
