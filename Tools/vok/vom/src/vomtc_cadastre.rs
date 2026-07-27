// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrc_cadastre - the rendered projection's shape (VOSMM "Report
//! and Cadastre" Done-when): header naming the regenerate tabtarget, one
//! section per registered cipher, one row per seated declaration home, no
//! coalescing, and content free of the census tool's name (VOr_q4f).

use super::vomrb_builder::vomrb_Builder;
use super::vomrc_cadastre::{vomrc_render, VOMRC_REGENERATE};

fn zvomtc_census(corpus: &[(String, String)]) -> super::vomrm_matricula::vomrm_Matricula {
    let mut builder = vomrb_Builder::vomrb_raise();
    builder.vomrb_seat_corpus(corpus);
    builder.vomrb_seal()
}

#[test]
fn vomtc_header_names_regenerate_tabtarget_and_bars_hand_edits() {
    let census = zvomtc_census(&[]);
    let rendered = vomrc_render(&census);
    let header = rendered.lines().next().unwrap();
    assert!(header.starts_with("<!--"), "header must lead the file");
    assert!(header.contains("do not hand-edit"));
    assert!(header.contains(VOMRC_REGENERATE));
}

#[test]
fn vomtc_every_registered_cipher_gets_a_section() {
    let census = zvomtc_census(&[]);
    let rendered = vomrc_render(&census);
    for cipher in vof::ALL_CIPHERS {
        let section = format!("## {} ({})", cipher.project(), cipher.prefix());
        assert!(
            rendered.contains(&section),
            "missing cipher section: {section}"
        );
    }
}

#[test]
fn vomtc_one_row_per_declaration_home_no_coalescing() {
    // One signet declared in two files: two rows, root-relative, uncoalesced.
    let corpus = vec![
        (
            "Tools/vok/a.rs".to_string(),
            "pub fn voftt_thing() {}\n".to_string(),
        ),
        (
            "Tools/vok/b.rs".to_string(),
            "pub fn voftt_thing() {}\n".to_string(),
        ),
    ];
    let census = zvomtc_census(&corpus);
    let rendered = vomrc_render(&census);
    assert!(rendered.contains("- `voftt_thing` → `Tools/vok/a.rs`\n"));
    assert!(rendered.contains("- `voftt_thing` → `Tools/vok/b.rs`\n"));
}

#[test]
fn vomtc_rows_land_under_their_cipher_section() {
    let corpus = vec![(
        "Tools/rbk/rbtt_probe.sh".to_string(),
        "rbtt_probe() {\n:\n}\n".to_string(),
    )];
    let census = zvomtc_census(&corpus);
    let rendered = vomrc_render(&census);
    let rb_section = rendered
        .split("\n## ")
        .find(|s| s.starts_with("Recipe Bottle (rb)"))
        .expect("rb section present");
    assert!(rb_section.contains("- `rbtt_probe` → `Tools/rbk/rbtt_probe.sh`"));
}

#[test]
fn vomtc_same_file_sites_dedupe_to_one_row() {
    // The file-stem envelope and the in-content declaration are one mint in
    // one home: exactly one row.
    let corpus = vec![(
        "Tools/vok/vof/src/voftt_thing.rs".to_string(),
        "pub fn voftt_thing() {}\n".to_string(),
    )];
    let census = zvomtc_census(&corpus);
    let rendered = vomrc_render(&census);
    let row = "- `voftt_thing` → `Tools/vok/vof/src/voftt_thing.rs`\n";
    assert_eq!(rendered.matches(row).count(), 1);
}

#[test]
fn vomtc_content_never_names_the_census_tool() {
    // VOr_q4f name-hygiene at the artifact: the generated content must
    // nowhere name the tool that renders it.
    let corpus = vec![(
        "Tools/vok/a.rs".to_string(),
        "pub fn voftt_thing() {}\n".to_string(),
    )];
    let census = zvomtc_census(&corpus);
    let lowered = vomrc_render(&census).to_ascii_lowercase();
    assert!(!lowered.contains("matricula"));
    assert!(!lowered.contains("vom "));
}

// eof
