// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the affiliate fill-blanks transforms (jjraf_affiliate): the
//! coronet-grain single fill, the firemark-grain heat drain, and the read-only
//! blank audit.

use super::jjraf_affiliate::{jjaf_PaceFill, jjrg_audit_blanks, jjrg_fill_heat, jjrg_fill_pace};
use super::jjrt_types::{
    jjrg_Gallops, jjrg_Heat, jjrg_HeatStatus, jjrg_Pace, jjrg_PaceState, jjrg_Tack,
    jjrg_text_to_lines,
};
use std::collections::BTreeMap;

/// A one-tack pace in the given state carrying the given sire (`None` = blank).
fn zjjtaf_pace(state: jjrg_PaceState, sire: Option<&str>) -> jjrg_Pace {
    jjrg_Pace {
        tacks: vec![jjrg_Tack {
            ts: "260729-1200".to_string(),
            state,
            tier: None,
            effort: None,
            text: jjrg_text_to_lines("docket"),
            silks: "test-pace".to_string(),
            basis: "0000000".to_string(),
        }],
        sire: sire.map(str::to_string),
        ..Default::default()
    }
}

/// Two heats. ₣AA harbours two blanks (CAAAA, CAAAB), one foreign-sire pace
/// (CAAAC → rb), and one already-jj pace (CAAAD). ₣AB harbours one blank
/// (CAAAE). The mixed-sire shape the drain and audit must respect.
fn zjjtaf_gallops() -> jjrg_Gallops {
    let mut aa_paces = BTreeMap::new();
    aa_paces.insert("₢CAAAA".to_string(), zjjtaf_pace(jjrg_PaceState::Rough, None));
    aa_paces.insert("₢CAAAB".to_string(), zjjtaf_pace(jjrg_PaceState::Rough, None));
    aa_paces.insert("₢CAAAC".to_string(), zjjtaf_pace(jjrg_PaceState::Rough, Some("rb")));
    aa_paces.insert("₢CAAAD".to_string(), zjjtaf_pace(jjrg_PaceState::Complete, Some("jj")));
    let aa = jjrg_Heat {
        silks: "test-heat".to_string(),
        creation_time: "260729".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec![
            "₢CAAAA".to_string(),
            "₢CAAAB".to_string(),
            "₢CAAAC".to_string(),
            "₢CAAAD".to_string(),
        ],
        paces: aa_paces,
    };

    let mut ab_paces = BTreeMap::new();
    ab_paces.insert("₢CAAAE".to_string(), zjjtaf_pace(jjrg_PaceState::Rough, None));
    let ab = jjrg_Heat {
        silks: "other-heat".to_string(),
        creation_time: "260729".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec!["₢CAAAE".to_string()],
        paces: ab_paces,
    };

    let mut heats = BTreeMap::new();
    heats.insert("₣AA".to_string(), aa);
    heats.insert("₣AB".to_string(), ab);
    jjrg_Gallops {
        next_heat_seed: "AC".to_string(),
        next_pace_seed: "CAAAF".to_string(),
        heat_order: vec!["₣AA".to_string(), "₣AB".to_string()],
        heats,
        retention_since: None,
    }
}

fn zjjtaf_sire(g: &jjrg_Gallops, heat: &str, coronet: &str) -> Option<String> {
    g.heats[heat].paces[coronet].sire.clone()
}

// ---- Coronet grain ----------------------------------------------------------

#[test]
fn jjtaf_fill_pace_stamps_a_blank() {
    let mut g = zjjtaf_gallops();
    assert!(matches!(jjrg_fill_pace(&mut g, "₢CAAAA", "jj"), jjaf_PaceFill::Filled));
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAA").as_deref(), Some("jj"), "the blank took the sire");
    // Siblings are untouched.
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAB"), None);
}

#[test]
fn jjtaf_fill_pace_on_same_sire_is_noop() {
    let mut g = zjjtaf_gallops();
    assert!(matches!(jjrg_fill_pace(&mut g, "₢CAAAD", "jj"), jjaf_PaceFill::AlreadySame));
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAD").as_deref(), Some("jj"), "unchanged");
}

#[test]
fn jjtaf_fill_pace_on_different_sire_conflicts_without_overwriting() {
    let mut g = zjjtaf_gallops();
    match jjrg_fill_pace(&mut g, "₢CAAAC", "jj") {
        jjaf_PaceFill::Conflict(incumbent) => assert_eq!(incumbent, "rb", "the conflict names the incumbent"),
        _ => panic!("a foreign-sire pace must conflict, never flatten"),
    }
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAC").as_deref(), Some("rb"), "the foreign sire survives");
}

#[test]
fn jjtaf_fill_pace_absent_coronet() {
    let mut g = zjjtaf_gallops();
    assert!(matches!(jjrg_fill_pace(&mut g, "₢CZZZZ", "jj"), jjaf_PaceFill::Absent));
}

#[test]
fn jjtaf_fill_pace_accepts_qualified_input() {
    // A display-qualified coronet (sigil + heat qualifier) resolves to the same
    // stored pace as its bare body.
    let mut g = zjjtaf_gallops();
    assert!(matches!(jjrg_fill_pace(&mut g, "₢AA·CAAAB", "jj"), jjaf_PaceFill::Filled));
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAB").as_deref(), Some("jj"));
}

// ---- Firemark grain ---------------------------------------------------------

#[test]
fn jjtaf_fill_heat_drains_blanks_and_spares_the_affiliated() {
    let mut g = zjjtaf_gallops();
    let report = jjrg_fill_heat(&mut g, "₣AA", "jj").expect("the heat resolves");

    assert_eq!(report.filled, vec!["₢AA·CAAAA".to_string(), "₢AA·CAAAB".to_string()], "both blanks fill, in heat order");
    assert_eq!(report.already, vec!["₢AA·CAAAD".to_string()], "the jj pace is reported, not re-stamped");
    assert_eq!(report.skipped, vec![("₢AA·CAAAC".to_string(), "rb".to_string())], "the foreign pace is skipped, naming its sire");

    // The two blanks now carry jj; the foreign rb pace is untouched.
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAA").as_deref(), Some("jj"));
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAB").as_deref(), Some("jj"));
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAC").as_deref(), Some("rb"), "a heat drain never flattens a foreign sire");
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAD").as_deref(), Some("jj"));
}

#[test]
fn jjtaf_fill_heat_absent_firemark() {
    let mut g = zjjtaf_gallops();
    assert!(jjrg_fill_heat(&mut g, "₣ZZ", "jj").is_none());
}

// ---- Audit ------------------------------------------------------------------

#[test]
fn jjtaf_audit_groups_blanks_by_heat_in_order() {
    let g = zjjtaf_gallops();
    let groups = jjrg_audit_blanks(&g);

    assert_eq!(groups.len(), 2, "both heats have blanks");
    assert_eq!(groups[0].firemark, "₣AA");
    assert_eq!(groups[0].silks, "test-heat");
    assert_eq!(groups[0].coronets, vec!["₢AA·CAAAA".to_string(), "₢AA·CAAAB".to_string()], "only the blanks, in heat order");
    assert_eq!(groups[1].firemark, "₣AB");
    assert_eq!(groups[1].coronets, vec!["₢AB·CAAAE".to_string()]);
}

#[test]
fn jjtaf_audit_empty_after_draining_every_heat() {
    // The done-when proof: drain each heat's blanks, and the audit reports none
    // remaining. The foreign rb pace was never a blank, so it neither drains nor
    // lingers as an unaffiliated straggler.
    let mut g = zjjtaf_gallops();
    jjrg_fill_heat(&mut g, "₣AA", "jj").expect("heat AA resolves");
    jjrg_fill_heat(&mut g, "₣AB", "jj").expect("heat AB resolves");

    assert!(jjrg_audit_blanks(&g).is_empty(), "no unaffiliated paces remain after the drain");
    assert_eq!(zjjtaf_sire(&g, "₣AA", "₢CAAAC").as_deref(), Some("rb"), "the drain left the foreign sire intact");
}
