// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOF - Vox Obscura Foundation
//!
//! Shared types and utilities for the VOK ecosystem. Provides:
//! - Cipher registry: typed project prefixes with validation
//! - CLAUDE.md freshening: managed section manipulation
//!
//! All project naming flows through this crate to ensure type safety
//! and compile-time enforcement of terminal exclusivity.

#![allow(non_camel_case_types)]

pub mod vofc_registry;
pub mod vofe_emplace;
pub mod voff_freshen;
pub mod vofr_release;

// Re-export the Cipher type and all cipher constants
pub use vofc_registry::*;

// Re-export freshen types and functions
pub use voff_freshen::{voff_freshen, voff_collapse, voff_parse_sections, voff_ManagedSection, voff_FreshenResult};

// Re-export release types and functions
pub use vofr_release::{vofr_collect, vofr_brand, vofr_CollectResult, vofr_BrandResult};

// Re-export emplace, vacate, and freshen types and functions
pub use vofe_emplace::{vofe_emplace, vofe_EmplaceArgs, vofe_EmplaceResult};
pub use vofe_emplace::{vofe_vacate, vofe_VacateArgs, vofe_VacateResult};
pub use vofe_emplace::{vofe_freshen_forge, vofe_FreshenResult, vofe_parse_burc, vofe_BurcEnv};
