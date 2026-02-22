// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! RAII test directory guard for JJK tests.
//!
//! Provides automatic cleanup of temporary directories, even on panic.

use std::path::{Path, PathBuf};

pub struct JjkTestDir(PathBuf);

impl JjkTestDir {
    pub fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for JjkTestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
