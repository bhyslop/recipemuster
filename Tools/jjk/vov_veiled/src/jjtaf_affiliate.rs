// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the affiliate backfill transform (jjraf_affiliate).

use super::jjraf_affiliate::jjrg_backfill_affiliation;
use super::jjrt_types::{
    jjrg_Gallops, jjrg_Heat, jjrg_HeatStatus, jjrg_Pace, jjrg_PaceState, jjrg_Tack,
    jjrg_text_to_lines,
};
use std::collections::BTreeMap;

/// A two-pace gallops in one heat, both paces sire-less — the pre-backfill shape.
fn zjjtaf_gallops() -> jjrg_Gallops {
    let mk = |state: jjrg_PaceState| jjrg_Pace {
        tacks: vec![jjrg_Tack {
            ts: "260729-1200".to_string(),
            state,
            tier: None,
            effort: None,
            text: jjrg_text_to_lines("docket"),
            silks: "test-pace".to_string(),
            basis: "0000000".to_string(),
        }],
        ..Default::default()
    };
    let mut paces = BTreeMap::new();
    paces.insert("₢CAAAA".to_string(), mk(jjrg_PaceState::Complete));
    paces.insert("₢CAAAB".to_string(), mk(jjrg_PaceState::Rough));
    let heat = jjrg_Heat {
        silks: "test-heat".to_string(),
        creation_time: "260729".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec!["₢CAAAA".to_string(), "₢CAAAB".to_string()],
        paces,
    };
    let mut heats = BTreeMap::new();
    heats.insert("₣AA".to_string(), heat);
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAC".to_string(),
        heat_order: vec!["₣AA".to_string()],
        heats,
        retention_since: None,
    }
}

#[test]
fn jjtaf_backfill_stamps_default_and_honors_overrides() {
    let mut gallops = zjjtaf_gallops();
    // The census: CAAAB's remaining work lands in the second sire (jj); everything
    // else — including the resolved CAAAA — takes the default (rb).
    let mut overrides = BTreeMap::new();
    overrides.insert("₢CAAAB".to_string(), "jj".to_string());

    let n = jjrg_backfill_affiliation(&mut gallops, "rb", &overrides);
    assert_eq!(n, 2, "every pace is stamped");

    let heat = &gallops.heats["₣AA"];
    assert_eq!(heat.paces["₢CAAAA"].sire.as_deref(), Some("rb"), "resolved pace takes the default");
    assert_eq!(heat.paces["₢CAAAB"].sire.as_deref(), Some("jj"), "override pace takes its sire");
}

#[test]
fn jjtaf_backfill_matches_overrides_by_bare_body() {
    let mut gallops = zjjtaf_gallops();
    // A display-qualified override key (sigil + heat qualifier) still matches the
    // stored coronet by its bare body.
    let mut overrides = BTreeMap::new();
    overrides.insert("₢AA·CAAAB".to_string(), "jj".to_string());

    jjrg_backfill_affiliation(&mut gallops, "rb", &overrides);
    assert_eq!(gallops.heats["₣AA"].paces["₢CAAAB"].sire.as_deref(), Some("jj"));
}
