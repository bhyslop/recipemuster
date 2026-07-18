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

use std::path::Path;

// RCG output discipline: diagnostics via vomrl_*! (stderr) only. The census
// itself is data, not a diagnostic, so it rides plain println! to stdout —
// the stream vomrl_log reserves for exactly this (see its module doc).
use vom::{vomrl_error_now, vomrl_info_now};

fn main() {
    vomrl_info_now!("{}", vom::vomrm_matricula::vomrm_identity());

    let repo_root = Path::new(".");
    match vom::vomrm_matricula::vomrm_raise_estrays(repo_root) {
        Ok(estrays) => {
            vomrl_info_now!("estray census: {} token(s)", estrays.len());
            for token in &estrays {
                println!("{token}");
            }
        }
        Err(e) => {
            vomrl_error_now!("census raise failed: {e}");
            std::process::exit(1);
        }
    }
}
