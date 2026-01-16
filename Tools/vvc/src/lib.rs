//! VVC - Voce Viva Core
//!
//! Shared commit infrastructure for VOK kits.
//! Provides guard and commit functionality used by VOK and JJK.

pub mod vvcc_commit;
pub mod vvcg_guard;

// Re-export commonly used types
pub use vvcc_commit::{CommitArgs, run as commit};
pub use vvcg_guard::{GuardArgs, run as guard};
