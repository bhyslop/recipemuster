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
//! Degenerate skeleton: this pace stands up the crate, the bin<->lib seam, the
//! vof path-dependency, and the output module. Real matricula behavior (raise ->
//! seat -> seal -> render, per the four-tier decomposition) arrives later.
//!
//! Module prefix tree (rbtd-style {crate}{r|t}{classifier}_ scheme):
//!   vomr{c}_  runtime source - vomrl_log (output), vomrm_matricula (census),
//!             vomra_allowlist (Tier 0 file-selection allowlist)
//!   vomt{c}_  test modules    - vomtm_matricula, vomta_allowlist
//! Grown as real API lands (mint-follows-API); see Tools/vok/README.md.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

pub mod vomra_allowlist;
pub mod vomrl_log;
pub mod vomrm_matricula;

#[cfg(test)]
mod vomta_allowlist;
#[cfg(test)]
mod vomtm_matricula;

// eof
