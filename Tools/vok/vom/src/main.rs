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
//!
//! Two verbs (VOSMM "Report and Cadastre"): the no-arg report run is
//! read-only over the tree - presentments, estray section, the digest audit,
//! and the cadastre freshness gate; the `cadastre` verb writes the one
//! generated complete-census file and nothing else.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

use std::path::Path;

// RCG output discipline: diagnostics via vomrl_*! (stderr) only. The census
// itself is data, not a diagnostic, so it rides plain print! to stdout —
// the stream vomrl_log reserves for exactly this (see its module doc).
use vom::vomrb_builder::vomrb_Builder;
use vom::vomrp_presentment::vomrp_render_all;
use vom::{vomrl_error_now, vomrl_info_now};

fn main() {
    vomrl_info_now!("{}", vom::vomrm_matricula::vomrm_identity());

    let args: Vec<String> = std::env::args().skip(1).collect();
    let verb = args.first().map(String::as_str);
    if let Some(other) = verb {
        if other != "cadastre" {
            vomrl_error_now!("unknown verb `{other}` (expected no verb, or `cadastre`)");
            std::process::exit(2);
        }
    }

    let repo_root = Path::new(".");
    let mut builder = vomrb_Builder::vomrb_raise();
    if let Err(e) = builder.vomrb_seat(repo_root) {
        vomrl_error_now!("census seat failed: {e}");
        std::process::exit(1);
    }
    let census = builder.vomrb_seal();
    vomrl_info_now!(
        "signet trie: {} claimed",
        census.vomrm_signet_trie().vomrs_len()
    );

    if verb == Some("cadastre") {
        let rendered = vom::vomrc_cadastre::vomrc_render(&census);
        let target = repo_root.join(vom::vomrc_cadastre::VOMRC_CADASTRE_PATH);
        if let Err(e) = std::fs::write(&target, &rendered) {
            vomrl_error_now!("cadastre write failed: {e}");
            std::process::exit(1);
        }
        vomrl_info_now!(
            "cadastre rendered: {} ({} bytes)",
            vom::vomrc_cadastre::VOMRC_CADASTRE_PATH,
            rendered.len()
        );
        return;
    }

    vomrl_info_now!("estray census: {} token(s)", census.vomrm_estrays().len());

    let collisions = census.vomrm_exact_collisions();
    let terminal_breaches = census.vomrm_terminal_exclusivity();
    vomrl_info_now!(
        "seating validators: {} collision(s), {} terminal-exclusivity breach(es)",
        collisions.len(),
        terminal_breaches.len()
    );

    let digest_findings = match vom::vomrd_digest::vomrd_gather(repo_root) {
        Ok(corpus) => vom::vomrd_digest::vomrd_audit(&census, &corpus),
        Err(e) => {
            vomrl_error_now!("digest gather failed: {e}");
            std::process::exit(1);
        }
    };
    vomrl_info_now!("digest audit: {} dead row(s)", digest_findings.len());

    let freshness = vom::vomrc_cadastre::vomrc_freshness(&census, repo_root);

    print!("{}", vomrp_render_all(&collisions));
    print!("{}", vomrp_render_all(&terminal_breaches));
    print!("{}", vomrp_render_all(&digest_findings));
    if let Some(stale) = &freshness {
        print!("{}", stale.vomrp_render());
        println!();
    }
    print!("{}", census.vomrm_render());
}

// eof
