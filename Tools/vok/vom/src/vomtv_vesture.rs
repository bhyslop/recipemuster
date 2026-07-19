// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrv_vesture.

use super::vomrv_vesture::*;

#[test]
fn vomtv_claims_rust_fn_and_struct() {
    assert_eq!(
        vomrv_claim_line("pub fn vomrb_seat(&mut self, repo_root: &Path) -> Result<(), String> {"),
        vec![("vomrb_seat".to_string(), "vomrb_seat".to_string())]
    );
    assert_eq!(
        vomrv_claim_line("pub struct vomrb_Builder {"),
        vec![("vomrb_Builder".to_string(), "vomrb_Builder".to_string())]
    );
}

#[test]
fn vomtv_claims_rust_internal_fn_and_pub_crate() {
    assert_eq!(
        vomrv_claim_line("pub(crate) fn zvomrb_tokenize(text: &str) -> impl Iterator<Item = &str> {"),
        vec![("zvomrb_tokenize".to_string(), "zvomrb_tokenize".to_string())]
    );
}

#[test]
fn vomtv_claims_rust_static_declaration() {
    assert_eq!(
        vomrv_claim_line("pub static RBTDRL_CASES_COVERAGE_UNUSED: &[rbtdre_Case] = &[];"),
        vec![(
            "RBTDRL_CASES_COVERAGE_UNUSED".to_string(),
            "RBTDRL_CASES_COVERAGE_UNUSED".to_string()
        )]
    );
}

#[test]
fn vomtv_claims_rust_mod_declaration() {
    assert_eq!(
        vomrv_claim_line("pub mod vomra_allowlist;"),
        vec![("vomra_allowlist".to_string(), "vomra_allowlist".to_string())]
    );
}

#[test]
fn vomtv_claims_bash_function_declaration() {
    assert_eq!(
        vomrv_claim_line("buc_log_args() { zbuc_tag_args 3 \"buc_log_args \" \"$@\"; }"),
        vec![("buc_log_args".to_string(), "buc_log_args".to_string())]
    );
}

#[test]
fn vomtv_claims_bash_top_level_constant() {
    assert_eq!(
        vomrv_claim_line("BUBC_band_admission=109 # mantle admission rejection"),
        vec![("BUBC_band_admission".to_string(), "BUBC_band_admission".to_string())]
    );
    assert_eq!(
        vomrv_claim_line("export BUC_VERBOSE=\"${BUC_VERBOSE:-0}\""),
        vec![("BUC_VERBOSE".to_string(), "BUC_VERBOSE".to_string())]
    );
}

#[test]
fn vomtv_claims_asciidoc_attribute_declaration() {
    assert_eq!(
        vomrv_claim_line(":voslc_cipher:          <<voslc_cipher,Cipher>>"),
        vec![("voslc_cipher".to_string(), "voslc_cipher".to_string())]
    );
}

#[test]
fn vomtv_claims_asciidoc_anchor_declaration() {
    assert_eq!(
        vomrv_claim_line("[[voslc_cipher]]"),
        vec![("voslc_cipher".to_string(), "voslc_cipher".to_string())]
    );
}

#[test]
fn vomtv_ignores_plain_prose_line() {
    assert!(vomrv_claim_line("This is AXLA-annotated content, plainly.").is_empty());
}

#[test]
fn vomtv_claims_rivet_shape_regardless_of_case_site() {
    assert!(vomrv_claim_rivet("VOr_q4f"));
    assert!(vomrv_claim_rivet("RBr_a3f"));
    assert!(vomrv_claim_rivet("JJr_a7c"));
    assert!(!vomrv_claim_rivet("voslc_cipher"));
    assert!(!vomrv_claim_rivet("BUBC_band_admission"));
}

// eof
