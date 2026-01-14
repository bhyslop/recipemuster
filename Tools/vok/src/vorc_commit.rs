// vorc_commit.rs - Core commit infrastructure for VOK
//
// Provides atomic commit workflow: lock, stage, guard, commit.
// Used by vvc-commit and JJ commands for consistent commit handling.
//
// Usage: vvx commit [--prefix <str>] [--message <str>] [--allow-empty] [--no-stage]
//
// Exit codes:
//   0 - Success (commit hash printed to stdout)
//   1 - Failure (lock held, guard failed, claude failed, commit failed)

use clap::Args;
use std::process::Command;

/// Lock reference path for commit operations
const LOCK_REF: &str = "refs/vvg/locks/vvx";

/// Default size limits (match vvg_git.sh constants)
const SIZE_LIMIT: u64 = 50000;
const WARN_LIMIT: u64 = 30000;

#[derive(Args, Debug)]
pub struct CommitArgs {
    /// Prefix string to prepend to commit message (e.g., "[jj:BRAND][F00/silks]")
    #[arg(long)]
    pub prefix: Option<String>,

    /// Commit message; if absent, invoke claude to generate from diff
    #[arg(short, long)]
    pub message: Option<String>,

    /// Allow empty commits (for chalk markers)
    #[arg(long)]
    pub allow_empty: bool,

    /// Skip 'git add -u' (respect pre-staged files)
    #[arg(long)]
    pub no_stage: bool,
}

/// Acquire the commit lock using git update-ref
/// Returns Ok(()) on success, Err(message) if lock already held
fn acquire_lock() -> Result<(), String> {
    // Get current HEAD as lock value
    let head_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("Failed to run git rev-parse: {}", e))?;

    let lock_value = if head_output.status.success() {
        String::from_utf8_lossy(&head_output.stdout).trim().to_string()
    } else {
        // No HEAD yet (empty repo) - use null SHA
        "0000000000000000000000000000000000000000".to_string()
    };

    // Try to create the ref - empty third arg means "only create if ref doesn't exist"
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

    // Exit code 0 = no differences (nothing staged)
    // Exit code 1 = differences exist (something staged)
    Ok(!result.status.success())
}

/// Run size guard check on staged content
/// Returns Ok(()) if under limit, Err if over limit
fn run_guard() -> Result<(), String> {
    // Reuse the guard logic from vorg_guard
    // We import it as a module in vorm_main, so call it directly
    let args = crate::vorg_guard::GuardArgs {
        limit: SIZE_LIMIT,
        warn: WARN_LIMIT,
    };

    let result = crate::vorg_guard::run(args);

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

    // Get the commit hash
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

pub fn run(args: CommitArgs) -> i32 {
    // Step 1: Acquire lock
    if let Err(e) = acquire_lock() {
        eprintln!("commit: error: {}", e);
        return 1;
    }

    // From here on, we must release the lock on any exit path
    let result = run_commit_workflow(&args);

    // Step 7: Release lock (always, regardless of success/failure)
    release_lock();

    match result {
        Ok(hash) => {
            // Output commit hash to stdout
            println!("{}", hash);
            eprintln!("commit: success");
            0
        }
        Err(e) => {
            eprintln!("commit: error: {}", e);
            1
        }
    }
}

/// Inner workflow that can return Result for cleaner error handling
fn run_commit_workflow(args: &CommitArgs) -> Result<String, String> {
    // Step 2: Stage changes (unless --no-stage)
    if !args.no_stage {
        stage_changes()?;
    }

    // Check if we have anything to commit (unless --allow-empty)
    if !args.allow_empty && !has_staged_changes()? {
        return Err("Nothing to commit".to_string());
    }

    // Step 3: Run size guard
    run_guard()?;

    // Step 4: Get or generate commit message
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

    // Step 5: Format full message with prefix and co-author
    let full_message = format_commit_message(args.prefix.as_deref(), &message);

    // Step 6: Execute commit
    execute_commit(&full_message, args.allow_empty)
}
