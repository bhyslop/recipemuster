// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM binary entry - the degenerate matricula.
//!
//! Thin dispatch into the vom library (rbtd-style bin<->lib split). Operator-only;
//! never ships (VOr_q4f). Real subcommand dispatch (raise/seat/seal/render)
//! arrives in later paces.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

// RCG output discipline: all emission via vomrl_*! - no direct println!/eprintln!
use vom::vomrl_info_now;

fn main() {
    // Degenerate: emit the crate identity as an operational milestone (proving
    // the bin<->lib seam, the vof path-dependency, and the output path end to
    // end), then exit clean. Real census output arrives in later paces.
    vomrl_info_now!("{}", vom::vomrm_matricula::vomrm_identity());
}
