// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops JSON operations
//!
//! Implements read/write operations on the Gallops JSON store.
//! All operations are atomic (write to temp, then rename).
//!
//! This module re-exports the public API from the new modular structure.

use std::path::Path;

// Re-export types
pub use crate::jjrt_types::*;

// Re-export validation
pub use crate::jjrv_validate::jjrg_validate;

// Re-export utilities
pub use crate::jjru_util::{jjrg_capture_commit_sha, jjrg_make_tack, jjrg_read_stdin, jjrg_read_stdin_optional};

// Re-export operations
pub use crate::jjro_ops::{jjrg_nominate, jjrg_slate, jjrg_rail, jjrg_tally, jjrg_draft, jjrg_retire, jjrg_build_trophy_preview, jjrg_furlough, jjrg_curry, jjrg_garland, jjrg_restring};

// Backwards compatibility: impl methods on jjrg_Gallops
impl jjrg_Gallops {
    /// Load Gallops from a file path (legacy API - prefer jjdr_load)
    pub fn jjrg_load(path: &Path) -> Result<Self, String> {
        use crate::jjri_io::jjdr_load;

        // Use validated load, then unwrap to plain Gallops
        jjdr_load(path).map(|vg| vg.into_inner())
    }

    /// Save Gallops to a file path (legacy API - prefer jjdr_save)
    pub fn jjrg_save(&self, path: &Path) -> Result<(), String> {
        use crate::jjri_io::jjdr_save;

        jjdr_save(self, path)
    }

    /// Validate the Gallops structure
    pub fn jjrg_validate(&self) -> Result<(), Vec<String>> {
        jjrg_validate(self)
    }

    /// Nominate a new Heat
    pub fn jjrg_nominate(&mut self, args: jjrg_NominateArgs, base_path: &Path) -> Result<jjrg_NominateResult, String> {
        jjrg_nominate(self, args, base_path)
    }

    /// Slate a new Pace
    pub fn jjrg_slate(&mut self, args: jjrg_SlateArgs) -> Result<jjrg_SlateResult, String> {
        jjrg_slate(self, args)
    }

    /// Rail - reorder Paces within a Heat
    pub fn jjrg_rail(&mut self, args: jjrg_RailArgs) -> Result<Vec<String>, String> {
        jjrg_rail(self, args)
    }

    /// Tally - add a new Tack to a Pace
    pub fn jjrg_tally(&mut self, args: jjrg_TallyArgs) -> Result<(), String> {
        jjrg_tally(self, args)
    }

    /// Draft - move a Pace from one Heat to another
    pub fn jjrg_draft(&mut self, args: jjrg_DraftArgs) -> Result<jjrg_DraftResult, String> {
        jjrg_draft(self, args)
    }

    /// Retire a Heat
    pub fn jjrg_retire(
        &mut self,
        args: jjrg_RetireArgs,
        base_path: &Path,
        steeplechase: &[crate::jjrs_steeplechase::jjrs_SteeplechaseEntry],
    ) -> Result<jjrg_RetireResult, String> {
        jjrg_retire(self, args, base_path, steeplechase)
    }

    /// Build trophy markdown preview (dry-run)
    pub fn jjrg_build_trophy_preview(
        &self,
        firemark: &str,
        paddock_content: &str,
        today: &str,
        steeplechase: &[crate::jjrs_steeplechase::jjrs_SteeplechaseEntry],
    ) -> Result<String, String> {
        jjrg_build_trophy_preview(self, firemark, paddock_content, today, steeplechase)
    }

    /// Furlough a Heat - change status or rename
    pub fn jjrg_furlough(&mut self, args: jjrg_FurloughArgs) -> Result<(), String> {
        jjrg_furlough(self, args)
    }

    /// Garland a Heat - celebrate completion and create continuation
    pub fn jjrg_garland(&mut self, args: jjrg_GarlandArgs, base_path: &Path) -> Result<jjrg_GarlandResult, String> {
        jjrg_garland(self, args, base_path)
    }

    /// Restring - bulk draft multiple paces atomically
    pub fn jjrg_restring(&mut self, args: jjrg_RestringArgs) -> Result<jjrg_RestringResult, String> {
        jjrg_restring(self, args)
    }
}
