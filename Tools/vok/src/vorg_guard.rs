// vorg_guard.rs - Pre-commit size validation
//
// Core VOK functionality that measures staged blob sizes before commit.
// Prevents catastrophic auto-adds (node_modules, build artifacts, binaries).
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

/// Entry for a staged file with its blob size
struct StagedFile {
    path: String,
    size: u64,
}

/// Get list of staged files with their blob sizes
fn get_staged_files() -> Result<Vec<StagedFile>, String> {
    // Get staged file info using git diff-index
    // Format: :old_mode new_mode old_oid new_oid status\tpath
    let output = Command::new("git")
        .args(["diff-index", "--cached", "-z", "HEAD"])
        .output()
        .map_err(|e| format!("Failed to run git diff-index: {}", e))?;

    if !output.status.success() {
        // If HEAD doesn't exist (initial commit), try against empty tree
        let empty_tree = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
        let output = Command::new("git")
            .args(["diff-index", "--cached", "-z", empty_tree])
            .output()
            .map_err(|e| format!("Failed to run git diff-index: {}", e))?;

        if !output.status.success() {
            return Err("git diff-index failed".to_string());
        }
        return parse_diff_index_output(&output.stdout);
    }

    parse_diff_index_output(&output.stdout)
}

/// Parse null-separated git diff-index output
fn parse_diff_index_output(data: &[u8]) -> Result<Vec<StagedFile>, String> {
    let mut files = Vec::new();

    // Output format: ":old_mode new_mode old_oid new_oid status\0path\0"
    // Split on null bytes
    let parts: Vec<&[u8]> = data.split(|&b| b == 0).collect();

    let mut i = 0;
    while i < parts.len() {
        let part = parts[i];
        if part.is_empty() {
            i += 1;
            continue;
        }

        // Parse the info line: ":old_mode new_mode old_oid new_oid status"
        let info = String::from_utf8_lossy(part);
        if !info.starts_with(':') {
            // This is a path from previous entry, skip
            i += 1;
            continue;
        }

        // Extract new_oid (4th field, after splitting on space)
        let fields: Vec<&str> = info[1..].split_whitespace().collect();
        if fields.len() < 4 {
            i += 1;
            continue;
        }

        let new_oid = fields[3];
        let status = fields.get(4).map(|s| s.chars().next()).flatten();

        // Skip deleted files (status 'D') - they don't add to commit size
        if status == Some('D') {
            i += 2; // Skip info line and path
            continue;
        }

        // Get path from next part
        i += 1;
        if i >= parts.len() {
            break;
        }
        let path = String::from_utf8_lossy(parts[i]).to_string();

        // Get blob size using git cat-file -s
        if new_oid != "0000000000000000000000000000000000000000" {
            let size = get_blob_size(new_oid)?;
            files.push(StagedFile { path, size });
        }

        i += 1;
    }

    Ok(files)
}

/// Get size of a blob by OID
fn get_blob_size(oid: &str) -> Result<u64, String> {
    let output = Command::new("git")
        .args(["cat-file", "-s", oid])
        .output()
        .map_err(|e| format!("Failed to run git cat-file: {}", e))?;

    if !output.status.success() {
        return Err(format!("git cat-file -s {} failed", oid));
    }

    let size_str = String::from_utf8_lossy(&output.stdout);
    size_str
        .trim()
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse size '{}': {}", size_str.trim(), e))
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
