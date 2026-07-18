// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM runtime - the matricula machinery.
//!
//! Degenerate: this module holds only the identity/link proof for the walking
//! skeleton. The census lifecycle (raise -> seat -> seal -> render) and the
//! four-tier decomposition (VOSMM-entity.adoc) populate it in later paces.

/// The crate's identity line, incorporating the foundation cipher's project
/// resolved through the vof path-dependency. Returned for callers and tests;
/// this function emits nothing (emission is the caller's concern, via vomrl_*!).
pub fn vomrm_identity() -> String {
    format!(
        "vom {} - Vox Matricula (foundation cipher: {})",
        env!("CARGO_PKG_VERSION"),
        vof::VO.project(),
    )
}

// eof
