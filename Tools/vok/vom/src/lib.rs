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
//! Census lifecycle lands here: vomrb_Builder is raised, seats claimed and
//! estray inscriptions, and vomrb_seal consumes it into the immutable
//! vomrm_Matricula - a Rust typestate mirroring VOSMM's build-then-freeze
//! shape (Memos/memo-20260620-freeze-builder-pattern/, provenance only).
//! Rust, Bash, and AsciiDoc attribute/anchor vestures claim declarations
//! (vomrv_vesture); the seating validators (collision, terminal-exclusivity)
//! are not yet implemented.
//!
//! Module prefix tree (rbtd-style {crate}{r|t}{classifier}_ scheme):
//!   vomr{c}_  runtime source - vomrl_log (output), vomrm_matricula (frozen
//!             census), vomrb_builder (mutable Builder), vomrs_signet (signet
//!             trie), vomra_allowlist (Tier 0 file-selection allowlist),
//!             vomrv_vesture (per-vesture declaration-site recognizers)
//!   vomt{c}_  test modules    - vomtm_matricula, vomtb_builder, vomts_signet,
//!             vomta_allowlist, vomtv_vesture
//! Grown as real API lands (mint-follows-API); see Tools/vok/README.md.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

pub mod vomra_allowlist;
pub mod vomrb_builder;
pub mod vomrl_log;
pub mod vomrm_matricula;
pub mod vomrs_signet;
pub mod vomrv_vesture;

#[cfg(test)]
mod vomta_allowlist;
#[cfg(test)]
mod vomtb_builder;
#[cfg(test)]
mod vomtm_matricula;
#[cfg(test)]
mod vomts_signet;
#[cfg(test)]
mod vomtv_vesture;

// eof
