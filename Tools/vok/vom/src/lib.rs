// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM - Vox Matricula
//!
//! A transient, frozen census of the repository's Inscriptions below the Cipher
//! tier, built from source on demand and discarded after the call that summoned
//! it (VOSMM-entity.adoc; entity quoin vosmm_matricula).
//!
//! Operator-only infrastructure: this crate must never enter the distributed
//! substrate and must never be linked by the shipped vvr binary (VOr_q4f). It is
//! a standalone bin reached only through its own vow-m* tabtargets.
//!
//! Degenerate skeleton: this pace stands up the crate, the bin<->lib seam, and
//! the vof path-dependency. Real matricula behavior (raise -> seat -> seal ->
//! render, per the four-tier decomposition) arrives in later paces.
//!
//! Module prefix tree (crate-prefix r/t scheme, per RCG source/test convention):
//!   vomr_  runtime source modules (the census machinery)
//!   vomt_  test modules
//! Grown as real API lands (mint-follows-API); see Tools/vok/README.md.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

pub mod vomr_matricula;

#[cfg(test)]
mod vomt_matricula;

// eof
