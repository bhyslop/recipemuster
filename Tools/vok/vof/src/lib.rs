// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOF - Vox Obscura Foundation
//!
//! Shared types and utilities for the VOK ecosystem. Provides:
//! - Cipher registry: typed project prefixes with validation
//!
//! All project naming flows through this crate to ensure type safety
//! and compile-time enforcement of terminal exclusivity.

#![deny(warnings)]
#![allow(non_camel_case_types)]

pub mod vofc_registry;
pub mod vofe_emplace;
pub mod vofr_release;

// Re-export the Cipher type and all cipher constants
pub use vofc_registry::*;

// Re-export release types and functions
pub use vofr_release::{vofr_collect, vofr_brand, vofr_CollectResult, vofr_BrandResult};
pub use vofr_release::{vofr_git_tracked_files, vofr_is_veiled_path};

// Re-export emplace and vacate types and functions
pub use vofe_emplace::{vofe_emplace, vofe_EmplaceArgs, vofe_EmplaceResult};
pub use vofe_emplace::{vofe_vacate, vofe_VacateArgs, vofe_VacateResult};
pub use vofe_emplace::{vofe_parse_burc, vofe_BurcEnv};
