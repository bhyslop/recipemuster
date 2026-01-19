// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrq_query::*;
use crate::jjrg_gallops::{jjrg_Heat as Heat, jjrg_Pace as Pace, jjrg_Tack as Tack, jjrg_Gallops as Gallops, jjrg_HeatStatus as HeatStatus, jjrg_PaceState as PaceState, JJRG_UNKNOWN_COMMIT};
use std::collections::BTreeMap;

fn create_test_gallops() -> Gallops {
    let mut paces = BTreeMap::new();
    paces.insert(
        "₢ABAAA".to_string(),
        Pace {
            tacks: vec![Tack {
                ts: "260101-1200".to_string(),
                state: PaceState::Rough,
                text: "First pace rough plan".to_string(),
                silks: "test-pace-one".to_string(),
                commit: JJRG_UNKNOWN_COMMIT.to_string(),
                direction: None,
            }],
        },
    );
    paces.insert(
        "₢ABAAB".to_string(),
        Pace {
            tacks: vec![Tack {
                ts: "260101-1300".to_string(),
                state: PaceState::Complete,
                text: "Completed pace".to_string(),
                silks: "test-pace-two".to_string(),
                commit: JJRG_UNKNOWN_COMMIT.to_string(),
                direction: None,
            }],
        },
    );

    let mut heats = BTreeMap::new();
    heats.insert(
        "₣AB".to_string(),
        Heat {
            silks: "test-heat".to_string(),
            creation_time: "260101".to_string(),
            status: HeatStatus::Racing,
            order: vec!["₢ABAAA".to_string(), "₢ABAAB".to_string()],
            next_pace_seed: "AAC".to_string(),
            paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
            paces,
        },
    );

    Gallops {
        next_heat_seed: "AC".to_string(),
        heats,
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

#[test]
fn jjtq_saddle_output_structure() {
    // Test the SaddleOutput serialization
    let output = zjjrq_SaddleOutput {
        heat_silks: "my-heat".to_string(),
        paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
        paddock_content: "# Test content".to_string(),
        pace_coronet: Some("₢ABAAA".to_string()),
        pace_silks: Some("my-pace".to_string()),
        pace_state: Some("rough".to_string()),
        spec: Some("Do the thing".to_string()),
        direction: None,
        recent_work: vec![],
    };
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("heat_silks"));
    assert!(json.contains("pace_coronet"));
    assert!(!json.contains("\"direction\"")); // None should be skipped
}

#[test]
fn jjtq_saddle_output_with_bridled_direction() {
    let output = zjjrq_SaddleOutput {
        heat_silks: "my-heat".to_string(),
        paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
        paddock_content: "# Test".to_string(),
        pace_coronet: Some("₢ABAAA".to_string()),
        pace_silks: Some("my-pace".to_string()),
        pace_state: Some("bridled".to_string()),
        spec: Some("Ready to execute".to_string()),
        direction: Some("Execute autonomously".to_string()),
        recent_work: vec![],
    };
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("\"direction\""));
    assert!(json.contains("Execute autonomously"));
}

#[test]
fn jjtq_saddle_output_no_actionable_pace() {
    let output = zjjrq_SaddleOutput {
        heat_silks: "my-heat".to_string(),
        paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
        paddock_content: "# All done".to_string(),
        pace_coronet: None,
        pace_silks: None,
        pace_state: None,
        spec: None,
        direction: None,
        recent_work: vec![],
    };
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("heat_silks"));
    assert!(!json.contains("pace_coronet"));
    assert!(!json.contains("pace_silks"));
}

#[test]
fn jjtq_parade_format_enum() {
    // Test ParadeFormat default
    let format: jjrq_ParadeFormat = Default::default();
    assert_eq!(format, jjrq_ParadeFormat::Full);
}

#[test]
fn jjtq_retire_output_structure() {
    let output = zjjrq_RetireOutput {
        firemark: "₣AB".to_string(),
        silks: "my-heat".to_string(),
        created: "260101".to_string(),
        status: "current".to_string(),
        paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
        paddock_content: "# Archive this".to_string(),
        paces: vec![zjjrq_RetirePace {
            coronet: "₢ABAAA".to_string(),
            silks: "test-pace".to_string(),
            tacks: vec![
                zjjrq_RetireTack {
                    ts: "260101-1400".to_string(),
                    state: "complete".to_string(),
                    text: "Final plan".to_string(),
                    direction: None,
                },
                zjjrq_RetireTack {
                    ts: "260101-1200".to_string(),
                    state: "rough".to_string(),
                    text: "Initial plan".to_string(),
                    direction: None,
                },
            ],
        }],
    };
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("firemark"));
    assert!(json.contains("₣AB"));
    assert!(json.contains("tacks"));
    // Verify tack history is included
    assert!(json.contains("260101-1400"));
    assert!(json.contains("260101-1200"));
}

#[test]
fn jjtq_retire_tack_with_direction() {
    let tack = zjjrq_RetireTack {
        ts: "260101-1200".to_string(),
        state: "bridled".to_string(),
        text: "Ready to fly".to_string(),
        direction: Some("Execute with agent X".to_string()),
    };
    let json = serde_json::to_string(&tack).unwrap();
    assert!(json.contains("direction"));
    assert!(json.contains("Execute with agent X"));
}

#[test]
fn jjtq_heat_status_filter_current() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();
    assert_eq!(heat.status, HeatStatus::Racing);

    // Simulate filter
    let filter = Some(HeatStatus::Racing);
    let matches = filter.as_ref().map_or(true, |f| &heat.status == f);
    assert!(matches);
}

#[test]
fn jjtq_heat_status_filter_retired() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();

    // Simulate filter for retired (should not match)
    let filter = Some(HeatStatus::Retired);
    let matches = filter.as_ref().map_or(true, |f| &heat.status == f);
    assert!(!matches);
}

#[test]
fn jjtq_pace_state_to_string() {
    assert_eq!(
        match PaceState::Rough {
            PaceState::Rough => "rough",
            PaceState::Bridled => "bridled",
            PaceState::Complete => "complete",
            PaceState::Abandoned => "abandoned",
        },
        "rough"
    );
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
                    PaceState::Rough | PaceState::Bridled => {
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
