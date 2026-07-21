// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use crate::jjrg_gallops::{jjrg_Heat, jjrg_Pace, jjrg_Tack, jjrg_Gallops, jjrg_HeatStatus, jjrg_PaceState, JJRG_UNKNOWN_BASIS, JJRG_STATE_ROUGH, JJRG_STATE_COMPLETE, JJRG_STATE_ABANDONED};
use crate::jjtu_testdir::{JjkTestDir, jjtu_seam_on_ground, jjtu_poison_config};
use crate::jjrf_favor::{jjrf_Firemark, jjrf_Coronet};
use crate::jjrgc_get_coronets::jjrgc_coronets_over;
use crate::jjrgs_get_spec::jjrgs_get_spec_over;
use crate::jjrm_mcp::zjjrm_load_gallops_over;
use std::collections::BTreeMap;
use std::path::Path;

fn create_test_gallops() -> jjrg_Gallops {
    let mut paces = BTreeMap::new();
    paces.insert(
        "₢ABAAA".to_string(),
        jjrg_Pace {
            tacks: vec![jjrg_Tack {
                ts: "260101-1200".to_string(),
                state: jjrg_PaceState::Rough,
                tier: None,
                effort: None,
                text: vec!["First pace rough plan".to_string()],
                silks: "test-pace-one".to_string(),
                basis: JJRG_UNKNOWN_BASIS.to_string(),
            }],
            ..Default::default()
        },
    );
    paces.insert(
        "₢ABAAB".to_string(),
        jjrg_Pace {
            tacks: vec![jjrg_Tack {
                ts: "260101-1300".to_string(),
                state: jjrg_PaceState::Complete,
                tier: None,
                effort: None,
                text: vec!["Completed pace".to_string()],
                silks: "test-pace-two".to_string(),
                basis: JJRG_UNKNOWN_BASIS.to_string(),
            }],
            ..Default::default()
        },
    );

    let mut heats = BTreeMap::new();
    heats.insert(
        "₣AB".to_string(),
        jjrg_Heat {
            silks: "test-heat".to_string(),
            creation_time: "260101".to_string(),
            status: jjrg_HeatStatus::Racing,
            order: vec!["₢ABAAA".to_string(), "₢ABAAB".to_string()],
            paces,
        },
    );

    jjrg_Gallops {
        next_heat_seed: "AC".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec!["₣AB".to_string()],
        heats,
        retention_since: None,
    }
}

#[test]
fn jjtq_muster_output_format() {
    // This test validates the TSV format conceptually
    // Full integration test would need file I/O
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();
    let expected_format = format!(
        "₣AB\t{}\tracing\t{}",
        heat.silks,
        heat.paces.len()
    );
    assert!(expected_format.contains("test-heat"));
    assert!(expected_format.contains("2")); // pace count
}

// Note: mount output is now plain text format instead of JSON.
// Output format is tested through integration tests.

#[test]
fn jjtq_heat_status_filter_current() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();
    assert_eq!(heat.status, jjrg_HeatStatus::Racing);

    // Simulate filter
    let filter = Some(jjrg_HeatStatus::Racing);
    let matches = filter.as_ref().map_or(true, |f| &heat.status == f);
    assert!(matches);
}

#[test]
fn jjtq_heat_status_filter_retired() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();

    // Simulate filter for retired (should not match)
    let filter = Some(jjrg_HeatStatus::Retired);
    let matches = filter.as_ref().map_or(true, |f| &heat.status == f);
    assert!(!matches);
}

#[test]
fn jjtq_pace_state_as_str() {
    assert_eq!(jjrg_PaceState::Rough.jjrg_as_str(), JJRG_STATE_ROUGH);
    assert_eq!(jjrg_PaceState::Complete.jjrg_as_str(), JJRG_STATE_COMPLETE);
    assert_eq!(jjrg_PaceState::Abandoned.jjrg_as_str(), JJRG_STATE_ABANDONED);
}

#[test]
fn jjtq_find_first_actionable_pace() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();

    // Find first actionable pace in order
    let mut found_coronet: Option<String> = None;
    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                match tack.state {
                    jjrg_PaceState::Rough => {
                        found_coronet = Some(coronet_key.clone());
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }

    assert_eq!(found_coronet, Some("₢ABAAA".to_string()));
}

// ============================================================================
// GetSpec tests
// ============================================================================

#[test]
fn jjtq_get_spec_extracts_tack_text() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();
    let pace = heat.paces.get("₢ABAAA").unwrap();
    let spec = pace.tacks.first().map(|t| t.text.clone());
    assert_eq!(spec, Some(vec!["First pace rough plan".to_string()]));
}

#[test]
fn jjtq_get_spec_second_pace() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();
    let pace = heat.paces.get("₢ABAAB").unwrap();
    let spec = pace.tacks.first().map(|t| t.text.clone());
    assert_eq!(spec, Some(vec!["Completed pace".to_string()]));
}

// ============================================================================
// GetCoronets tests
// ============================================================================

fn create_test_gallops_with_mixed_states() -> jjrg_Gallops {
    let mut paces = BTreeMap::new();
    paces.insert(
        "₢ACAAA".to_string(),
        jjrg_Pace {
            tacks: vec![jjrg_Tack {
                ts: "260101-1200".to_string(),
                state: jjrg_PaceState::Complete,
                tier: None,
                effort: None,
                text: vec!["Done".to_string()],
                silks: "pace-complete".to_string(),
                basis: JJRG_UNKNOWN_BASIS.to_string(),
            }],
            ..Default::default()
        },
    );
    paces.insert(
        "₢ACAAB".to_string(),
        jjrg_Pace {
            tacks: vec![jjrg_Tack {
                ts: "260101-1300".to_string(),
                state: jjrg_PaceState::Rough,
                tier: None,
                effort: None,
                text: vec!["Needs work".to_string()],
                silks: "pace-rough".to_string(),
                basis: JJRG_UNKNOWN_BASIS.to_string(),
            }],
            ..Default::default()
        },
    );
    paces.insert(
        "₢ACAAC".to_string(),
        jjrg_Pace {
            tacks: vec![jjrg_Tack {
                ts: "260101-1400".to_string(),
                state: jjrg_PaceState::Rough,
                tier: None,
                effort: None,
                text: vec!["Ready to fly".to_string()],
                silks: "pace-rough-two".to_string(),
                basis: JJRG_UNKNOWN_BASIS.to_string(),
            }],
            ..Default::default()
        },
    );
    paces.insert(
        "₢ACAAD".to_string(),
        jjrg_Pace {
            tacks: vec![jjrg_Tack {
                ts: "260101-1500".to_string(),
                state: jjrg_PaceState::Abandoned,
                tier: None,
                effort: None,
                text: vec!["Gave up".to_string()],
                silks: "pace-abandoned".to_string(),
                basis: JJRG_UNKNOWN_BASIS.to_string(),
            }],
            ..Default::default()
        },
    );

    let mut heats = BTreeMap::new();
    heats.insert(
        "₣AC".to_string(),
        jjrg_Heat {
            silks: "mixed-state-heat".to_string(),
            creation_time: "260101".to_string(),
            status: jjrg_HeatStatus::Racing,
            order: vec![
                "₢ACAAA".to_string(),
                "₢ACAAB".to_string(),
                "₢ACAAC".to_string(),
                "₢ACAAD".to_string(),
            ],
            paces,
        },
    );

    jjrg_Gallops {
        next_heat_seed: "AD".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec!["₣AC".to_string()],
        heats,
        retention_since: None,
    }
}

#[test]
fn jjtq_get_coronets_all() {
    let gallops = create_test_gallops_with_mixed_states();
    let heat = gallops.heats.get("₣AC").unwrap();

    // No filter - all coronets
    let coronets: Vec<&String> = heat.order.iter().collect();
    assert_eq!(coronets.len(), 4);
}

#[test]
fn jjtq_get_coronets_remaining_filter() {
    let gallops = create_test_gallops_with_mixed_states();
    let heat = gallops.heats.get("₣AC").unwrap();

    // --remaining filter: exclude complete and abandoned
    let remaining: Vec<&String> = heat.order.iter().filter(|coronet_key| {
        if let Some(pace) = heat.paces.get(*coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                return tack.state != jjrg_PaceState::Complete && tack.state != jjrg_PaceState::Abandoned;
            }
        }
        true
    }).collect();

    assert_eq!(remaining.len(), 2);
    assert!(remaining.contains(&&"₢ACAAB".to_string())); // rough
    assert!(remaining.contains(&&"₢ACAAC".to_string())); // rough
}

#[test]
fn jjtq_get_coronets_rough_filter() {
    let gallops = create_test_gallops_with_mixed_states();
    let heat = gallops.heats.get("₣AC").unwrap();

    // --rough filter: only rough paces
    let rough: Vec<&String> = heat.order.iter().filter(|coronet_key| {
        if let Some(pace) = heat.paces.get(*coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                return tack.state == jjrg_PaceState::Rough;
            }
        }
        false
    }).collect();

    assert_eq!(rough.len(), 2);
    assert!(rough.contains(&&"₢ACAAB".to_string()));
    assert!(rough.contains(&&"₢ACAAC".to_string()));
}

// ============================================================================
// Abandoned-marker tests — exercise the real render boundary over a resolved
// load, locking the output contract (not a reimplementation of the logic).
// Driven through the seam-resolved `_over` doors (`zjjrm_load_gallops_over`)
// rather than the const-gated public entry, so they are const-INDEPENDENT: a
// seam-OFF form here survives the studbook cutover flip, and a seam-ON proof
// per handler witnesses the studbook read the flip turns on.
// ============================================================================

fn save_mixed_states(dir: &JjkTestDir) -> std::path::PathBuf {
    let file = dir.path().join("jjg_gallops.json");
    create_test_gallops_with_mixed_states().jjrg_save(&file).unwrap();
    file
}

#[test]
fn jjtq_coronets_default_tags_abandoned() {
    let td = JjkTestDir::new("jjk_test_coronets_abandoned");
    let file = save_mixed_states(&td);
    let firemark = jjrf_Firemark::jjrf_parse("₣AC").unwrap();

    // Seam-OFF: the render funnels a path-loaded gallops; the poison config
    // proves off never reaches for the studbook.
    let (code, out) = jjrgc_coronets_over(
        zjjrm_load_gallops_over(false, &file, &jjtu_poison_config()),
        &firemark,
        false,
        false,
    );

    assert_eq!(code, 0);
    // Coronets render heat-qualified (JJS0 jjdt_coronet) — heat ₣AC, so ₢AC·<body>;
    // the abandoned pace still carries the marker with the coronet the first token.
    let tagged = format!("₢AC·ACAAD  [{}]", JJRG_STATE_ABANDONED);
    assert!(out.lines().any(|l| l == tagged.as_str()), "expected tagged abandoned line, got:\n{}", out);
    // Live paces render qualified with no tag — exact-line match proves no tag leaked.
    assert!(out.lines().any(|l| l == "₢AC·ACAAA")); // complete
    assert!(out.lines().any(|l| l == "₢AC·ACAAB")); // rough
    assert!(out.lines().any(|l| l == "₢AC·ACAAC")); // rough
}

#[test]
fn jjtq_coronets_remaining_still_excludes_abandoned() {
    let td = JjkTestDir::new("jjk_test_coronets_remaining");
    let file = save_mixed_states(&td);
    let firemark = jjrf_Firemark::jjrf_parse("₣AC").unwrap();

    let (code, out) = jjrgc_coronets_over(
        zjjrm_load_gallops_over(false, &file, &jjtu_poison_config()),
        &firemark,
        true,
        false,
    );

    assert_eq!(code, 0);
    // --remaining excludes abandoned entirely: no line, no marker. Check the bare
    // body (present in both bare and heat-qualified renderings) so the exclusion
    // holds regardless of display form.
    assert!(!out.contains("ACAAD"));
    assert!(!out.contains(JJRG_STATE_ABANDONED));
}

#[test]
fn jjtq_coronets_seam_on_reads_studbook() {
    // Seam-ON: over=true ignores the (nonexistent) path and ref-reads the seeded
    // studbook. code==0 plus the studbook's own rows prove the read came from the
    // studbook — a path read would have errored on the nonexistent file.
    let (_ground, config) = jjtu_seam_on_ground(
        "jjtq_coronets_seam_on",
        create_test_gallops_with_mixed_states(),
    );
    let firemark = jjrf_Firemark::jjrf_parse("₣AC").unwrap();

    let (code, out) = jjrgc_coronets_over(
        zjjrm_load_gallops_over(true, Path::new("/nonexistent/jjtq-never-read.json"), &config),
        &firemark,
        false,
        false,
    );

    assert_eq!(code, 0, "seam-on read must succeed from the studbook, got:\n{}", out);
    let tagged = format!("₢AC·ACAAD  [{}]", JJRG_STATE_ABANDONED);
    assert!(out.lines().any(|l| l == tagged.as_str()), "studbook rows missing, got:\n{}", out);
    assert!(out.lines().any(|l| l == "₢AC·ACAAA"));
}

#[test]
fn jjtq_brief_leads_with_abandoned_marker() {
    let td = JjkTestDir::new("jjk_test_brief_abandoned");
    let file = save_mixed_states(&td);
    let coronet = jjrf_Coronet::jjrf_parse("₢ACAAD").unwrap();

    let (code, out) = jjrgs_get_spec_over(
        zjjrm_load_gallops_over(false, &file, &jjtu_poison_config()),
        &coronet,
    );

    assert_eq!(code, 0);
    assert!(out.starts_with(&format!("[{}]", JJRG_STATE_ABANDONED)), "brief should lead with marker, got:\n{}", out);
    assert!(out.contains("Gave up")); // docket text preserved below the marker
}

#[test]
fn jjtq_brief_live_pace_is_verbatim() {
    let td = JjkTestDir::new("jjk_test_brief_live");
    let file = save_mixed_states(&td);
    let coronet = jjrf_Coronet::jjrf_parse("₢ACAAB").unwrap();

    let (code, out) = jjrgs_get_spec_over(
        zjjrm_load_gallops_over(false, &file, &jjtu_poison_config()),
        &coronet,
    );

    assert_eq!(code, 0);
    // Live pace: no marker, docket text verbatim.
    assert!(!out.contains(&format!("[{}]", JJRG_STATE_ABANDONED)));
    assert_eq!(out.trim_end(), "Needs work");
}

#[test]
fn jjtq_brief_seam_on_reads_studbook() {
    // Seam-ON companion to the get_coronets proof: the get_spec render consumes a
    // studbook-loaded gallops when over=true, path untouched (nonexistent).
    let (_ground, config) = jjtu_seam_on_ground(
        "jjtq_brief_seam_on",
        create_test_gallops_with_mixed_states(),
    );
    let coronet = jjrf_Coronet::jjrf_parse("₢ACAAD").unwrap();

    let (code, out) = jjrgs_get_spec_over(
        zjjrm_load_gallops_over(true, Path::new("/nonexistent/jjtq-never-read.json"), &config),
        &coronet,
    );

    assert_eq!(code, 0, "seam-on read must succeed from the studbook, got:\n{}", out);
    assert!(out.starts_with(&format!("[{}]", JJRG_STATE_ABANDONED)), "studbook docket missing, got:\n{}", out);
    assert!(out.contains("Gave up"));
}
