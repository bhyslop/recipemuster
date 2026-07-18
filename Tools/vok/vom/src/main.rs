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

fn main() {
    // Degenerate: exercise the bin<->lib seam and the vof path-dependency, then
    // exit clean. Emits nothing yet - the RCG output module is deferred to the
    // first pace with real census output to route.
    let _ = vom::vomr_identity();
}
