// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrv_vesture.

use super::vomrv_vesture::*;

fn zvomtv_rust() -> vomrv_Dress {
    vomrv_dress("Tools/vok/vom/src/vomrb_builder.rs")
}

fn zvomtv_bash(path: &str) -> vomrv_Dress {
    vomrv_dress(path)
}

fn zvomtv_adoc() -> vomrv_Dress {
    vomrv_dress("Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc")
}

#[test]
fn vomtv_claims_rust_fn_and_struct() {
    assert_eq!(
        vomrv_claim_line(
            &zvomtv_rust(),
            "pub fn vomrb_seat(&mut self, repo_root: &Path) -> Result<(), String> {"
        ),
        vec![("vomrb_seat".to_string(), "vomrb_seat".to_string())]
    );
    assert_eq!(
        vomrv_claim_line(&zvomtv_rust(), "pub struct vomrb_Builder {"),
        vec![("vomrb_Builder".to_string(), "vomrb_Builder".to_string())]
    );
}

#[test]
fn vomtv_claims_rust_internal_fn_and_pub_crate() {
    assert_eq!(
        vomrv_claim_line(
            &zvomtv_rust(),
            "pub(crate) fn zvomrb_tokenize(text: &str) -> impl Iterator<Item = &str> {"
        ),
        vec![("zvomrb_tokenize".to_string(), "zvomrb_tokenize".to_string())]
    );
}

#[test]
fn vomtv_claims_rust_static_declaration() {
    assert_eq!(
        vomrv_claim_line(
            &zvomtv_rust(),
            "pub static RBTDRL_CASES_COVERAGE_UNUSED: &[rbtdre_Case] = &[];"
        ),
        vec![(
            "RBTDRL_CASES_COVERAGE_UNUSED".to_string(),
            "RBTDRL_CASES_COVERAGE_UNUSED".to_string()
        )]
    );
}

#[test]
fn vomtv_claims_rust_mod_declaration() {
    assert_eq!(
        vomrv_claim_line(&zvomtv_rust(), "pub mod vomra_allowlist;"),
        vec![("vomra_allowlist".to_string(), "vomra_allowlist".to_string())]
    );
}

#[test]
fn vomtv_claims_bash_function_declaration() {
    assert_eq!(
        vomrv_claim_line(
            &zvomtv_bash("Tools/buk/buc_command.sh"),
            "buc_log_args() { zbuc_tag_args 3 \"buc_log_args \" \"$@\"; }"
        ),
        vec![("buc_log_args".to_string(), "buc_log_args".to_string())]
    );
}

#[test]
fn vomtv_claims_bash_constant_in_its_home_family() {
    assert_eq!(
        vomrv_claim_line(
            &zvomtv_bash("Tools/buk/bubc_bands.sh"),
            "BUBC_band_admission=109 # mantle admission rejection"
        ),
        vec![("BUBC_band_admission".to_string(), "BUBC_band_admission".to_string())]
    );
    assert_eq!(
        vomrv_claim_line(
            &zvomtv_bash("Tools/buk/buc_command.sh"),
            "export BUC_VERBOSE=\"${BUC_VERBOSE:-0}\""
        ),
        vec![("BUC_VERBOSE".to_string(), "BUC_VERBOSE".to_string())]
    );
}

#[test]
fn vomtv_bash_constant_home_rule_is_prefix_bidirectional() {
    // A 0-trick or numbered module still homes its family's constants.
    assert_eq!(
        vomrv_claim_line(&zvomtv_bash("Tools/rbk/rbfc0_cli.sh"), "RBFC_THING=1"),
        vec![("RBFC_THING".to_string(), "RBFC_THING".to_string())]
    );
}

#[test]
fn vomtv_bash_cross_family_assignment_is_not_a_declaration() {
    // The launcher setting another family's constant is an inscription
    // site, never a declaration (home-file rule).
    assert!(vomrv_claim_line(
        &zvomtv_bash("Tools/buk/bul_launcher.sh"),
        "BURC_OUTPUT_ROOT_DIR=\"${BURD_CONFIG_DIR}/output\""
    )
    .is_empty());
}

#[test]
fn vomtv_tabtarget_body_is_formulary_and_claims_nothing() {
    let dress = vomrv_dress("tt/rbw-cC.Charge.tadmor.sh");
    assert!(vomrv_claim_line(&dress, "BURD_LAUNCHER=launcher.rbw_workbench.sh").is_empty());
    assert!(vomrv_claim_line(&dress, "buw_dispatch() {").is_empty());
}

#[test]
fn vomtv_markdown_body_is_reference_only_and_claims_nothing() {
    let dress = vomrv_dress("Tools/buk/README.md");
    assert!(vomrv_claim_line(&dress, "BURC_OUTPUT_ROOT_DIR=/some/example").is_empty());
    assert!(vomrv_claim_line(&dress, "[[some_anchor]]").is_empty());
}

#[test]
fn vomtv_claims_asciidoc_attribute_declaration() {
    assert_eq!(
        vomrv_claim_line(&zvomtv_adoc(), ":voslc_cipher:          <<voslc_cipher,Cipher>>"),
        vec![("voslc_cipher".to_string(), "voslc_cipher".to_string())]
    );
}

#[test]
fn vomtv_claims_asciidoc_anchor_declaration() {
    assert_eq!(
        vomrv_claim_line(&zvomtv_adoc(), "[[voslc_cipher]]"),
        vec![("voslc_cipher".to_string(), "voslc_cipher".to_string())]
    );
}

#[test]
fn vomtv_ignores_plain_prose_line() {
    assert!(vomrv_claim_line(&zvomtv_adoc(), "This is AXLA-annotated content, plainly.").is_empty());
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
