// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOI - Vox Obscura Infrastructure
//!
//! Common types for the VOK ecosystem. Provides:
//! - Cipher registry: typed project prefixes with validation
//!
//! All project naming flows through this crate to ensure type safety
//! and compile-time enforcement of terminal exclusivity.

#![allow(non_camel_case_types)]

pub mod voic_registry;

// Re-export the Cipher type and all cipher constants
pub use voic_registry::*;
