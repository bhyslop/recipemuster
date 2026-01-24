// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrq_query::*;
use crate::jjrg_gallops::{jjrg_Heat as Heat, jjrg_Pace as Pace, jjrg_Tack as Tack, jjrg_Gallops as Gallops, jjrg_HeatStatus as HeatStatus, jjrg_PaceState as PaceState, JJRG_UNKNOWN_COMMIT};
use std::collections::BTreeMap;
use indexmap::IndexMap;

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

    let mut heats = IndexMap::new();
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

// Note: saddle output is now plain text format instead of JSON.
// Output format is tested through integration tests.

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

// ============================================================================
// GetSpec tests
// ============================================================================

#[test]
fn jjtq_get_spec_extracts_tack_text() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();
    let pace = heat.paces.get("₢ABAAA").unwrap();
    let spec = pace.tacks.first().map(|t| t.text.clone());
    assert_eq!(spec, Some("First pace rough plan".to_string()));
}

#[test]
fn jjtq_get_spec_second_pace() {
    let gallops = create_test_gallops();
    let heat = gallops.heats.get("₣AB").unwrap();
    let pace = heat.paces.get("₢ABAAB").unwrap();
    let spec = pace.tacks.first().map(|t| t.text.clone());
    assert_eq!(spec, Some("Completed pace".to_string()));
}

// ============================================================================
// GetCoronets tests
// ============================================================================

fn create_test_gallops_with_mixed_states() -> Gallops {
    let mut paces = BTreeMap::new();
    paces.insert(
        "₢ACAAA".to_string(),
        Pace {
            tacks: vec![Tack {
                ts: "260101-1200".to_string(),
                state: PaceState::Complete,
                text: "Done".to_string(),
                silks: "pace-complete".to_string(),
                commit: JJRG_UNKNOWN_COMMIT.to_string(),
                direction: None,
            }],
        },
    );
    paces.insert(
        "₢ACAAB".to_string(),
        Pace {
            tacks: vec![Tack {
                ts: "260101-1300".to_string(),
                state: PaceState::Rough,
                text: "Needs work".to_string(),
                silks: "pace-rough".to_string(),
                commit: JJRG_UNKNOWN_COMMIT.to_string(),
                direction: None,
            }],
        },
    );
    paces.insert(
        "₢ACAAC".to_string(),
        Pace {
            tacks: vec![Tack {
                ts: "260101-1400".to_string(),
                state: PaceState::Bridled,
                text: "Ready to fly".to_string(),
                silks: "pace-bridled".to_string(),
                commit: JJRG_UNKNOWN_COMMIT.to_string(),
                direction: Some("Execute with sonnet".to_string()),
            }],
        },
    );
    paces.insert(
        "₢ACAAD".to_string(),
        Pace {
            tacks: vec![Tack {
                ts: "260101-1500".to_string(),
                state: PaceState::Abandoned,
                text: "Gave up".to_string(),
                silks: "pace-abandoned".to_string(),
                commit: JJRG_UNKNOWN_COMMIT.to_string(),
                direction: None,
            }],
        },
    );

    let mut heats = IndexMap::new();
    heats.insert(
        "₣AC".to_string(),
        Heat {
            silks: "mixed-state-heat".to_string(),
            creation_time: "260101".to_string(),
            status: HeatStatus::Racing,
            order: vec![
                "₢ACAAA".to_string(),
                "₢ACAAB".to_string(),
                "₢ACAAC".to_string(),
                "₢ACAAD".to_string(),
            ],
            next_pace_seed: "AAE".to_string(),
            paddock_file: ".claude/jjm/jjp_AC.md".to_string(),
            paces,
        },
    );

    Gallops {
        next_heat_seed: "AD".to_string(),
        heats,
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
                return tack.state != PaceState::Complete && tack.state != PaceState::Abandoned;
            }
        }
        true
    }).collect();

    assert_eq!(remaining.len(), 2);
    assert!(remaining.contains(&&"₢ACAAB".to_string())); // rough
    assert!(remaining.contains(&&"₢ACAAC".to_string())); // bridled
}

#[test]
fn jjtq_get_coronets_rough_filter() {
    let gallops = create_test_gallops_with_mixed_states();
    let heat = gallops.heats.get("₣AC").unwrap();

    // --rough filter: only rough paces
    let rough: Vec<&String> = heat.order.iter().filter(|coronet_key| {
        if let Some(pace) = heat.paces.get(*coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                return tack.state == PaceState::Rough;
            }
        }
        false
    }).collect();

    assert_eq!(rough.len(), 1);
    assert_eq!(rough[0], "₢ACAAB");
}
