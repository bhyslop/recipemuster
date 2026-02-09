// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vvcg_guard module
//!
//! Unit tests for guard cost model and integration test for the full guard workflow.
//! These tests avoid std::env::set_current_dir() to prevent race conditions in parallel test runs.

#[cfg(test)]
mod tests {
    use crate::vvcg_guard::{zvvcg_get_diff_size, vvcg_run, vvcg_GuardArgs, VVCG_SIZE_LIMIT, VVCG_WARN_LIMIT};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    /// Get test temp directory - uses BUD if available, falls back to system temp
    fn get_test_base() -> PathBuf {
        // Try to use BUD temp dir if available (when run via tabtarget)
        if let Ok(bud_temp) = std::env::var("BURD_TEMP_DIR") {
            let path = PathBuf::from(bud_temp);
            // Canonicalize to absolute path (BURD_TEMP_DIR may be relative)
            path.canonicalize().unwrap_or(path)
        } else {
            // Fall back to system temp for direct cargo test
            std::env::temp_dir()
        }
    }

    /// Helper to initialize a git repo in a directory
    fn init_git_repo(dir: &Path) {
        Command::new("git")
            .args(["init"])
            .current_dir(dir)
            .output()
            .expect("git init failed");

        // Set user config for commits
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(dir)
            .output()
            .expect("git config user.name failed");

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(dir)
            .output()
            .expect("git config user.email failed");
    }

    /// Helper to stage a file in a git repo
    fn stage_file(dir: &Path, path: &str) {
        Command::new("git")
            .args(["add", path])
            .current_dir(dir)
            .output()
            .expect("git add failed");
    }

    #[test]
    fn vvtg_text_file_size() {
        // Create temp dir and git repo
        let temp_dir = get_test_base().join("vvtg_text_file_size");
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous run
        fs::create_dir_all(&temp_dir).unwrap();
        init_git_repo(&temp_dir);

        // Create and stage a text file
        let file_path = temp_dir.join("test.txt");
        let content = "Hello, World!\n";
        fs::write(&file_path, content).unwrap();
        stage_file(&temp_dir, "test.txt");

        // Get diff size
        let size = zvvcg_get_diff_size("test.txt", Some(&temp_dir)).unwrap();

        // For a new text file, size should equal blob size (content length)
        assert_eq!(size, content.len() as u64);

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn vvtg_binary_file_size() {
        // Create temp dir and git repo
        let temp_dir = get_test_base().join("vvtg_binary_file_size");
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous run
        fs::create_dir_all(&temp_dir).unwrap();
        init_git_repo(&temp_dir);

        // Create and stage a binary file (null bytes make it binary)
        let file_path = temp_dir.join("test.bin");
        let content = vec![0u8; 100]; // 100 null bytes
        fs::write(&file_path, &content).unwrap();
        stage_file(&temp_dir, "test.bin");

        // Get diff size
        let size = zvvcg_get_diff_size("test.bin", Some(&temp_dir)).unwrap();

        // For a new binary file, size should equal blob size (100 bytes)
        // This is the key test: binary files report actual blob size, not diff output
        assert_eq!(size, 100);

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn vvtg_deleted_file_size() {
        // Create temp dir and git repo
        let temp_dir = get_test_base().join("vvtg_deleted_file_size");
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous run
        fs::create_dir_all(&temp_dir).unwrap();
        init_git_repo(&temp_dir);

        // Create, commit, then delete a file
        let file_path = temp_dir.join("test.txt");
        fs::write(&file_path, "content\n").unwrap();
        stage_file(&temp_dir, "test.txt");

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(&temp_dir)
            .output()
            .expect("git commit failed");

        fs::remove_file(&file_path).unwrap();
        stage_file(&temp_dir, "test.txt");

        // Get diff size for deleted file
        let size = zvvcg_get_diff_size("test.txt", Some(&temp_dir)).unwrap();

        // Deleted files should have 0 incremental cost
        assert_eq!(size, 0);

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn vvtg_large_binary_blocked() {
        // Create temp dir and git repo (integration test)
        let temp_dir = get_test_base().join("vvtg_large_binary_blocked");
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous run
        fs::create_dir_all(&temp_dir).unwrap();
        init_git_repo(&temp_dir);

        // Create and stage a 100KB binary file
        let file_path = temp_dir.join("large.bin");
        let content = vec![0u8; 100_000]; // 100KB
        fs::write(&file_path, &content).unwrap();
        stage_file(&temp_dir, "large.bin");

        // Run guard check with standard limits
        let args = vvcg_GuardArgs {
            limit: VVCG_SIZE_LIMIT,  // 50KB
            warn: VVCG_WARN_LIMIT,   // 30KB
        };
        let result = vvcg_run(&args, Some(&temp_dir));

        // Should be blocked (exit code 1) because 100KB > 50KB limit
        assert_eq!(result, 1);

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }
}
