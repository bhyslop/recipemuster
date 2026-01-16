//! VVC - Voce Viva Core
//!
//! Shared commit infrastructure for VOK kits.
//! Provides guard and commit functionality used by VOK and JJK.

#![allow(non_camel_case_types)]

pub mod vvcc_commit;
pub mod vvcg_guard;

// Re-export commonly used types (RCG-compliant names)
pub use vvcc_commit::{vvcc_CommitArgs, vvcc_CommitLock, vvcc_run as commit};
pub use vvcg_guard::{vvcg_GuardArgs, vvcg_run as guard};
