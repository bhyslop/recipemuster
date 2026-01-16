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
//! let lock = vvc::CommitLock::acquire()?;
//! // ... do work while holding lock ...
//! let hash = lock.commit(&args)?;
//! // lock released when dropped
//! ```
//!
//! Exit codes:
//!   0 - Success (commit hash printed to stdout)
//!   1 - Failure (lock held, guard failed, claude failed, commit failed)

use std::process::Command;

use crate::vvcg_guard;

/// Lock reference path for commit operations
const LOCK_REF: &str = "refs/vvg/locks/vvx";

/// Default size limits
const SIZE_LIMIT: u64 = 50000;
const WARN_LIMIT: u64 = 30000;

/// Arguments for commit operation
#[derive(Debug, Clone, Default)]
pub struct CommitArgs {
    /// Prefix string to prepend to commit message (e.g., "[jj:BRAND][F00/silks]")
    pub prefix: Option<String>,
    /// Commit message; if absent, invoke claude to generate from diff
    pub message: Option<String>,
    /// Allow empty commits (for chalk markers)
    pub allow_empty: bool,
    /// Skip 'git add -A' (respect pre-staged files)
    pub no_stage: bool,
}

/// RAII guard that holds the commit lock.
///
/// The lock is automatically released when the guard is dropped, ensuring
/// no code path can leak the lock. Use this when you need to hold the lock
/// across multiple operations (e.g., modify gallops, then commit).
///
/// ## Example
/// ```ignore
/// let lock = CommitLock::acquire()?;
/// // Do protected work here...
/// let hash = lock.commit(&args)?;
/// // Lock released when `lock` goes out of scope
/// ```
pub struct CommitLock {
    /// Private field prevents external construction
    _private: (),
}

impl CommitLock {
    /// Acquire the commit lock.
    ///
    /// Returns `Err` if the lock is already held by another operation.
    /// The lock is held until this `CommitLock` is dropped.
    pub fn acquire() -> Result<Self, String> {
        acquire_lock()?;
        Ok(CommitLock { _private: () })
    }

    /// Perform a commit while holding the lock.
    ///
    /// This does NOT release the lock - the caller retains the guard
    /// and can perform additional operations if needed.
    ///
    /// Returns the commit hash on success.
    pub fn commit(&self, args: &CommitArgs) -> Result<String, String> {
        run_commit_workflow(args)
    }
}

impl Drop for CommitLock {
    fn drop(&mut self) {
        release_lock();
    }
}

/// Acquire the commit lock using git update-ref
fn acquire_lock() -> Result<(), String> {
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
        .args(["update-ref", LOCK_REF, &lock_value, ""])
        .output()
        .map_err(|e| format!("Failed to run git update-ref: {}", e))?;

    if result.status.success() {
        eprintln!("commit: lock acquired");
        Ok(())
    } else {
        Err("Another commit in progress - lock held".to_string())
    }
}

/// Release the commit lock
fn release_lock() {
    let _ = Command::new("git")
        .args(["update-ref", "-d", LOCK_REF])
        .output();
    eprintln!("commit: lock released");
}

/// Stage all changes including untracked files with 'git add -A'
fn stage_changes() -> Result<(), String> {
    let result = Command::new("git")
        .args(["add", "-A"])
        .output()
        .map_err(|e| format!("Failed to run git add: {}", e))?;

    if result.status.success() {
        eprintln!("commit: staged all changes");
        Ok(())
    } else {
        Err(format!(
            "git add -A failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ))
    }
}

/// Check if there are staged changes
fn has_staged_changes() -> Result<bool, String> {
    let result = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .output()
        .map_err(|e| format!("Failed to run git diff: {}", e))?;

    Ok(!result.status.success())
}

/// Run size guard check on staged content
fn run_guard() -> Result<(), String> {
    let args = vvcg_guard::GuardArgs {
        limit: SIZE_LIMIT,
        warn: WARN_LIMIT,
    };

    let result = vvcg_guard::run(&args);

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
fn get_staged_diff() -> Result<String, String> {
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
fn generate_message_with_claude(diff: &str) -> Result<String, String> {
    let prompt = format!(
        "Generate a concise commit message for this diff:\n\n<diff>\n{}\n</diff>\n\nRespond with only the commit message, no explanation.",
        diff
    );

    eprintln!("commit: invoking claude for commit message...");

    let output = Command::new("claude")
        .args(["--print", &prompt])
        .output()
        .map_err(|e| format!("Failed to invoke claude: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("claude --print failed: {}", stderr));
    }

    let message = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if message.is_empty() {
        return Err("claude returned empty message".to_string());
    }

    eprintln!("commit: claude generated message");
    Ok(message)
}

/// Format the full commit message with prefix and co-author
fn format_commit_message(prefix: Option<&str>, message: &str) -> String {
    let mut full_message = String::new();

    if let Some(p) = prefix {
        full_message.push_str(p);
    }

    full_message.push_str(message);
    full_message.push_str("\n\nCo-Authored-By: Claude <noreply@anthropic.com>");

    full_message
}

/// Execute the git commit
fn execute_commit(message: &str, allow_empty: bool) -> Result<String, String> {
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
/// `CommitLock::acquire()` instead.
pub fn run(args: &CommitArgs) -> i32 {
    let lock = match CommitLock::acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("commit: error: {}", e);
            return 1;
        }
    };

    match lock.commit(args) {
        Ok(hash) => {
            println!("{}", hash);
            eprintln!("commit: success");
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
fn run_commit_workflow(args: &CommitArgs) -> Result<String, String> {
    if !args.no_stage {
        stage_changes()?;
    }

    if !args.allow_empty && !has_staged_changes()? {
        return Err("Nothing to commit".to_string());
    }

    run_guard()?;

    let message = match &args.message {
        Some(m) => m.clone(),
        None => {
            let diff = get_staged_diff()?;
            if diff.is_empty() && !args.allow_empty {
                return Err("No diff to generate message from".to_string());
            }
            generate_message_with_claude(&diff)?
        }
    };

    let full_message = format_commit_message(args.prefix.as_deref(), &message);

    execute_commit(&full_message, args.allow_empty)
}
