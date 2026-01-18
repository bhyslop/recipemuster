// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vvcg_guard - VVC guard size validation

#[cfg(test)]
mod tests {
    use crate::vvcg_guard::{vvcg_run, vvcg_GuardArgs, VVCG_SIZE_LIMIT, VVCG_WARN_LIMIT};
    use std::fs;
    use std::process::Command;

    /// Get test temp directory - uses BUD if available, falls back to system temp
    fn vvtg_get_test_base() -> std::path::PathBuf {
        // Try to use BUD temp dir if available (when run via tabtarget)
        if let Ok(bud_temp) = std::env::var("BUD_TEMP_DIR") {
            let path = std::path::PathBuf::from(bud_temp);
            // Canonicalize to absolute path (BUD_TEMP_DIR may be relative)
            path.canonicalize().unwrap_or(path)
        } else {
            // Fall back to system temp for direct cargo test
            std::env::temp_dir()
        }
    }

    /// Helper to create a test git repo
    fn vvtg_setup_test_repo(name: &str) -> std::path::PathBuf {
        let test_dir = vvtg_get_test_base().join("vvc-guard-tests").join(name);

        // Clean up any existing test repo
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).expect("Failed to remove existing test dir");
        }

        fs::create_dir_all(&test_dir).expect("Failed to create test dir");

        // Initialize git repo
        let output = Command::new("git")
            .args(["init"])
            .current_dir(&test_dir)
            .output()
            .expect("Failed to init git repo");
        assert!(output.status.success(), "git init failed");

        // Configure git user for commits
        let output = Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(&test_dir)
            .output()
            .expect("Failed to set git user name");
        assert!(output.status.success(), "git config user.name failed");

        let output = Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&test_dir)
            .output()
            .expect("Failed to set git user email");
        assert!(output.status.success(), "git config user.email failed");

        // Create initial commit to establish HEAD
        // (some git operations need at least one commit)
        fs::write(test_dir.join(".gitignore"), "").expect("Failed to create .gitignore");
        let output = Command::new("git")
            .args(["add", ".gitignore"])
            .current_dir(&test_dir)
            .output()
            .expect("Failed to add .gitignore");
        assert!(output.status.success(), "git add .gitignore failed");

        let output = Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(&test_dir)
            .output()
            .expect("Failed to create initial commit");
        assert!(output.status.success(), "git commit failed");

        test_dir
    }

    /// Helper to stage a file in a test repo
    fn vvtg_stage_file(repo: &std::path::Path, name: &str, content: &[u8]) {
        let file_path = repo.join(name);
        fs::write(&file_path, content).expect("Failed to write test file");

        let output = Command::new("git")
            .args(["add", name])
            .current_dir(repo)
            .output()
            .expect("Failed to stage file");

        if !output.status.success() {
            panic!(
                "Failed to stage file {}: {}",
                name,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    /// Helper to get diff size in a specific repo directory
    fn vvtg_get_size_in_repo(repo: &std::path::Path, file: &str) -> Result<u64, String> {
        // Get staged blob info
        let output = Command::new("git")
            .args(["ls-files", "--cached", "-s", "--", file])
            .current_dir(repo)
            .output()
            .map_err(|e| format!("Failed to run git ls-files: {}", e))?;

        if !output.status.success() {
            return Err(format!("git ls-files failed for {}", file));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let line = stdout.trim();

        if line.is_empty() {
            return Ok(0); // File deleted or not in index
        }

        // Parse: "100644 <sha> 0\t<path>"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(format!("Unexpected git ls-files output: {}", line));
        }

        let blob_sha = parts[1];

        // Get blob size
        let output = Command::new("git")
            .args(["cat-file", "-s", blob_sha])
            .current_dir(repo)
            .output()
            .map_err(|e| format!("Failed to run git cat-file: {}", e))?;

        if !output.status.success() {
            return Err(format!("git cat-file -s {} failed", blob_sha));
        }

        let size_str = String::from_utf8_lossy(&output.stdout);
        size_str
            .trim()
            .parse::<u64>()
            .map_err(|e| format!("Failed to parse size: {}", e))
    }

    #[test]
    fn vvtg_text_file_size() {
        let repo = vvtg_setup_test_repo("vvtg_text_file_size");

        // Create and stage a text file with known size
        let content = b"Hello, world!\nThis is a test file.\n";
        vvtg_stage_file(&repo, "test.txt", content);

        let size = vvtg_get_size_in_repo(&repo, "test.txt").expect("Failed to get diff size");

        assert_eq!(
            size,
            content.len() as u64,
            "Text file size should match content length"
        );
    }

    #[test]
    fn vvtg_binary_file_size() {
        let repo = vvtg_setup_test_repo("vvtg_binary_file_size");

        // Create a binary file with known size (1KB of random-ish bytes)
        let content: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
        vvtg_stage_file(&repo, "binary.bin", &content);

        let size = vvtg_get_size_in_repo(&repo, "binary.bin").expect("Failed to get diff size");

        assert_eq!(
            size,
            content.len() as u64,
            "Binary file size should match actual content length, not git diff output"
        );
    }

    #[test]
    fn vvtg_deleted_file_size() {
        let repo = vvtg_setup_test_repo("vvtg_deleted_file_size");

        // Create and commit a file
        let content = b"This file will be deleted\n";
        vvtg_stage_file(&repo, "to_delete.txt", content);

        Command::new("git")
            .args(["commit", "-m", "Second commit"])
            .current_dir(&repo)
            .output()
            .expect("Failed to commit");

        // Delete and stage the deletion
        fs::remove_file(repo.join("to_delete.txt")).expect("Failed to delete file");

        Command::new("git")
            .args(["add", "to_delete.txt"])
            .current_dir(&repo)
            .output()
            .expect("Failed to stage deletion");

        let size = vvtg_get_size_in_repo(&repo, "to_delete.txt").expect("Failed to get diff size");

        assert_eq!(size, 0, "Deleted file should have size 0");
    }

    #[test]
    fn vvtg_large_binary_blocked() {
        let repo = vvtg_setup_test_repo("vvtg_large_binary_blocked");

        // Create a binary file larger than the default limit (100KB > 50KB default)
        let content: Vec<u8> = vec![0xFF; 100_000];
        vvtg_stage_file(&repo, "large.bin", &content);

        // Verify file was staged correctly
        let size = vvtg_get_size_in_repo(&repo, "large.bin").expect("Failed to get size");
        assert_eq!(size, 100_000, "File should be staged with correct size");

        // Run guard with standard limits (50KB) - 100KB file should be blocked
        let args = vvcg_GuardArgs {
            limit: VVCG_SIZE_LIMIT,
            warn: VVCG_WARN_LIMIT,
        };
        let exit_code = vvcg_run(&args, Some(&repo));

        assert_eq!(
            exit_code, 1,
            "100KB file should be blocked by 50KB standard limit (exit code 1)"
        );
    }

    #[test]
    fn vvtg_regression_tarball() {
        let repo = vvtg_setup_test_repo("vvtg_regression_tarball");

        // Simulate a 2MB tarball (the original failure case)
        let content: Vec<u8> = vec![0x1F; 2_000_000]; // ~2MB
        vvtg_stage_file(&repo, "vvk-parcel-1000.tar.gz", &content);

        // Verify the file size is correctly detected
        let size = vvtg_get_size_in_repo(&repo, "vvk-parcel-1000.tar.gz")
            .expect("Failed to get tarball size");

        assert_eq!(
            size,
            2_000_000,
            "Tarball size should be 2MB, not ~60 bytes from diff output"
        );

        // Run guard with standard limits (should block)
        let args = vvcg_GuardArgs {
            limit: VVCG_SIZE_LIMIT,
            warn: VVCG_WARN_LIMIT,
        };
        let exit_code = vvcg_run(&args, Some(&repo));

        assert_eq!(
            exit_code, 1,
            "2MB tarball must be blocked by standard limit (exit code 1)"
        );
    }
}
