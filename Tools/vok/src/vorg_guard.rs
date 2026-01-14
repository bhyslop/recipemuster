// vorg_guard.rs - Pre-commit size validation
//
// Core VOK functionality that measures staged diff sizes before commit.
// Prevents catastrophic auto-adds (node_modules, build artifacts, binaries)
// while allowing small edits to large files.
//
// Usage: vvx guard [--limit <bytes>] [--warn <bytes>]
//
// Exit codes:
//   0 - Under limit
//   1 - Over limit (with breakdown by file)
//   2 - Over warn threshold (proceed with caution)

use clap::Args;
use std::process::Command;

#[derive(Args, Debug)]
pub struct GuardArgs {
    /// Size limit in bytes (default: 500000)
    #[arg(long, default_value = "500000")]
    pub limit: u64,

    /// Warning threshold in bytes (default: 250000)
    #[arg(long, default_value = "250000")]
    pub warn: u64,
}

/// Entry for a staged file with its diff size
struct StagedFile {
    path: String,
    size: u64,
}

/// Get list of staged files with their diff sizes
fn get_staged_files() -> Result<Vec<StagedFile>, String> {
    // Get list of staged file paths
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

        let size = get_diff_size(path)?;
        files.push(StagedFile {
            path: path.to_string(),
            size,
        });
    }

    Ok(files)
}

/// Get size of staged diff for a specific file
fn get_diff_size(path: &str) -> Result<u64, String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--", path])
        .output()
        .map_err(|e| format!("Failed to run git diff for {}: {}", path, e))?;

    if !output.status.success() {
        return Err(format!("git diff --cached -- {} failed", path));
    }

    Ok(output.stdout.len() as u64)
}

pub fn run(args: GuardArgs) -> i32 {
    let files = match get_staged_files() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("guard: error: {}", e);
            return 1;
        }
    };

    let total_size: u64 = files.iter().map(|f| f.size).sum();

    if total_size > args.limit {
        eprintln!("guard: BLOCKED - staged content {} bytes exceeds limit {} bytes", total_size, args.limit);
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
        eprintln!("guard: WARNING - staged content {} bytes exceeds warning threshold {} bytes", total_size, args.warn);
        return 2;
    }

    eprintln!("guard: OK - staged content {} bytes (limit: {})", total_size, args.limit);
    0
}
