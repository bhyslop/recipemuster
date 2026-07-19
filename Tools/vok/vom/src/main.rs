// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM binary entry - the degenerate matricula.
//!
//! Thin dispatch into the vom library (rbtd-style bin<->lib split). Operator-only;
//! never ships (VOr_q4f). Drives the full raise -> seat -> seal -> render
//! lifecycle (VOSMM-entity.adoc "Census Lifecycle"); classify-by-subtraction
//! lands an ours-cipher token as an estray only when no vesture in
//! vomrv_vesture claims it.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

use std::path::Path;

// RCG output discipline: diagnostics via vomrl_*! (stderr) only. The census
// itself is data, not a diagnostic, so it rides plain print! to stdout —
// the stream vomrl_log reserves for exactly this (see its module doc).
use vom::vomrb_builder::vomrb_Builder;
use vom::{vomrl_error_now, vomrl_info_now};

fn main() {
    vomrl_info_now!("{}", vom::vomrm_matricula::vomrm_identity());

    let mut builder = vomrb_Builder::vomrb_raise();
    match builder.vomrb_seat(Path::new(".")) {
        Ok(()) => {
            let census = builder.vomrb_seal();
            vomrl_info_now!(
                "signet trie: {} claimed",
                census.vomrm_signet_trie().vomrs_len()
            );
            vomrl_info_now!("estray census: {} token(s)", census.vomrm_estrays().len());
            print!("{}", census.vomrm_render());
        }
        Err(e) => {
            vomrl_error_now!("census seat failed: {e}");
            std::process::exit(1);
        }
    }
}
