// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC - Voce Viva Core
//!
//! Shared commit infrastructure for VOK kits.
//! Provides guard and commit functionality used by VOK and JJK.
//!
//! Two commit patterns:
//! - `vvcc_*`: Interactive/Claude-assisted commits (stages all, generates message)
//! - `vvcm_*`: Machine/programmatic commits (explicit files, explicit message)

#![allow(non_camel_case_types)]

pub mod vvcc_commit;
pub mod vvce_env;
pub mod vvcg_guard;
pub mod vvcm_machine;

#[cfg(test)]
mod vvtg_guard;

// Re-export commonly used types (RCG-compliant names)
pub use vvcc_commit::{vvcc_CommitArgs, vvcc_CommitLock, vvcc_run as commit};
pub use vvce_env::{vvce_env, VvcEnv};
pub use vvcg_guard::{vvcg_GuardArgs, vvcg_run as guard};
pub use vvcm_machine::{vvcm_CommitArgs, vvcm_commit as machine_commit};
