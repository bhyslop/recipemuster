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
pub mod vvcc_format;
pub mod vvce_env;
pub mod vvcg_guard;
pub mod vvcm_machine;
pub mod vvcp_probe;

#[cfg(test)]
mod vvtg_guard;

#[cfg(test)]
mod vvtf_format;

// Re-export commonly used types (RCG-compliant names)
pub use vvcc_commit::{vvcc_CommitArgs, vvcc_CommitLock, vvcc_run as commit};
pub use vvcc_format::{vvcc_format_branded, vvcc_get_hallmark};
pub use vvce_env::{vvce_env, VvcEnv};
pub use vvcg_guard::{vvcg_GuardArgs, vvcg_run as guard, VVCG_SIZE_LIMIT, VVCG_WARN_LIMIT};
pub use vvcm_machine::{vvcm_CommitArgs, vvcm_commit as machine_commit};
pub use vvcp_probe::{vvcp_invitatory, vvcp_probe, VVCP_ACTION_INVITATORY};
