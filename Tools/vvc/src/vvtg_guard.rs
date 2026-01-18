// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vvcg_guard - VVC guard size validation

#[cfg(test)]
mod tests {
    use crate::vvcg_guard::{vvcg_run, vvcg_GuardArgs, zvvcg_get_diff_size};
    use std::fs;
    use std::process::Command;

    /// Get test temp directory - uses BUD if available, falls back to system temp
    fn vvtg_get_test_base() -> std::path::PathBuf {
        // Try to use BUD temp dir if available (when run via tabtarget)
        if let Ok(bud_temp) = std::env::var("BUD_TEMP_DIR") {
            std::path::PathBuf::from(bud_temp)
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
        Command::new("git")
            .args(["init"])
            .current_dir(&test_dir)
            .output()
            .expect("Failed to init git repo");

        // Configure git user for commits
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(&test_dir)
            .output()
            .expect("Failed to set git user name");

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&test_dir)
            .output()
            .expect("Failed to set git user email");

        test_dir
    }

    /// Helper to stage a file in a test repo
    fn vvtg_stage_file(repo: &std::path::Path, name: &str, content: &[u8]) {
        let file_path = repo.join(name);
        fs::write(&file_path, content).expect("Failed to write test file");

        Command::new("git")
            .args(["add", name])
            .current_dir(repo)
            .output()
            .expect("Failed to stage file");
    }

    #[test]
    fn vvtg_text_file_size() {
        let repo = vvtg_setup_test_repo("vvtg_text_file_size");

        // Create and stage a text file with known size
        let content = b"Hello, world!\nThis is a test file.\n";
        vvtg_stage_file(&repo, "test.txt", content);

        // Change directory to the test repo to run zvvcg_get_diff_size
        let original_dir = std::env::current_dir().expect("Failed to get current dir");
        std::env::set_current_dir(&repo).expect("Failed to change to test repo");

        let size = zvvcg_get_diff_size("test.txt").expect("Failed to get diff size");

        // Restore original directory
        std::env::set_current_dir(original_dir).expect("Failed to restore directory");

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

        let original_dir = std::env::current_dir().expect("Failed to get current dir");
        std::env::set_current_dir(&repo).expect("Failed to change to test repo");

        let size = zvvcg_get_diff_size("binary.bin").expect("Failed to get diff size");

        std::env::set_current_dir(original_dir).expect("Failed to restore directory");

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
            .args(["commit", "-m", "Initial commit"])
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

        let original_dir = std::env::current_dir().expect("Failed to get current dir");
        std::env::set_current_dir(&repo).expect("Failed to change to test repo");

        let size = zvvcg_get_diff_size("to_delete.txt").expect("Failed to get diff size");

        std::env::set_current_dir(original_dir).expect("Failed to restore directory");

        assert_eq!(size, 0, "Deleted file should have size 0");
    }

    #[test]
    fn vvtg_large_binary_blocked() {
        let repo = vvtg_setup_test_repo("vvtg_large_binary_blocked");

        // Create a binary file larger than the default limit (100KB > 50KB default)
        let content: Vec<u8> = vec![0xFF; 100_000];
        vvtg_stage_file(&repo, "large.bin", &content);

        let original_dir = std::env::current_dir().expect("Failed to get current dir");
        std::env::set_current_dir(&repo).expect("Failed to change to test repo");

        // Run guard with default limit (500KB) - should pass
        let args = vvcg_GuardArgs::default();
        let exit_code = vvcg_run(&args);

        std::env::set_current_dir(&original_dir).expect("Failed to restore directory");

        assert_eq!(exit_code, 0, "100KB file should pass 500KB default limit");

        // Now test with a smaller limit that should block it
        std::env::set_current_dir(&repo).expect("Failed to change to test repo");

        let small_args = vvcg_GuardArgs {
            limit: 50_000,
            warn: 25_000,
        };
        let exit_code = vvcg_run(&small_args);

        std::env::set_current_dir(&original_dir).expect("Failed to restore directory");

        assert_eq!(
            exit_code, 1,
            "100KB file should be blocked by 50KB limit (exit code 1)"
        );
    }

    #[test]
    fn vvtg_regression_tarball() {
        let repo = vvtg_setup_test_repo("vvtg_regression_tarball");

        // Simulate a 2MB tarball (the original failure case)
        let content: Vec<u8> = vec![0x1F; 2_000_000]; // ~2MB
        vvtg_stage_file(&repo, "vvk-parcel-1000.tar.gz", &content);

        let original_dir = std::env::current_dir().expect("Failed to get current dir");
        std::env::set_current_dir(&repo).expect("Failed to change to test repo");

        // Verify the file size is correctly detected
        let size = zvvcg_get_diff_size("vvk-parcel-1000.tar.gz")
            .expect("Failed to get tarball size");

        assert_eq!(
            size,
            2_000_000,
            "Tarball size should be 2MB, not ~60 bytes from diff output"
        );

        // Run guard with 50KB limit (should block)
        let args = vvcg_GuardArgs {
            limit: 50_000,
            warn: 25_000,
        };
        let exit_code = vvcg_run(&args);

        std::env::set_current_dir(&original_dir).expect("Failed to restore directory");

        assert_eq!(
            exit_code, 1,
            "2MB tarball must be blocked by 50KB limit (exit code 1)"
        );
    }
}
