// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrg_gallops::*;
use super::jjri_io::{jjri_RetentionState, jjri_retention_state};
use super::jjrv_validate::{zjjrg_is_base64, zjjrg_is_kebab_case, zjjrg_is_yymmdd, zjjrg_is_yymmdd_hhmm};
use super::jjru_util::zjjrg_increment_seed;
use super::jjrvl_validate::{jjrvl_run_validate, jjrvl_ValidateArgs, zjjrvl_appraise, zjjrvl_Appraisal};
use super::jjtu_testdir::JjkTestDir;
use std::collections::BTreeMap;

#[test]
fn jjtg_pace_state_serialization() {
    let state = jjrg_PaceState::Rough;
    let json = serde_json::to_string(&state).unwrap();
    assert_eq!(json, "\"jjgte_rough\"");
}

#[test]
fn jjtg_tack_text_deserialize_array_shape() {
    // Docket text is the array shape, taken verbatim (the tack-text→lines reprieve
    // episode that also tolerated a legacy string shape converged and was stripped).
    let current = r#"{"jjgtn_ts":"260101-1200","jjgtn_state":"jjgte_rough","jjgtn_text":["a","b"],"jjgtn_silks":"x","jjgtn_basis":"0000000"}"#;
    let tack: jjrg_Tack = serde_json::from_str(current).unwrap();
    assert_eq!(tack.text, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn jjtg_text_lines_round_trip_is_lossless() {
    for s in ["", "single", "two\nlines", "blank\n\nbetween", "trailing\n", "\nleading"] {
        assert_eq!(jjrg_lines_to_text(&jjrg_text_to_lines(s)), s);
    }
}

// ===== validate normalize-and-report (zjjrvl_appraise) =====

/// A single-heat gallops already canonical at the current schema: heat_order populated, one tack
/// per pace, array-shaped docket text.
fn canonical_gallops() -> jjrg_Gallops {
    let (hk, heat) = make_valid_heat("AC", "my-heat");
    let mut heats = BTreeMap::new();
    heats.insert(hk.clone(), heat);
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![hk],
        heats,
        retention_since: None,
    }
}

/// Variant name for assertion-failure messages.
fn appraisal_name(a: &zjjrvl_Appraisal) -> &'static str {
    match a {
        zjjrvl_Appraisal::Canonical(_) => "Canonical",
        zjjrvl_Appraisal::Normalize(..) => "Normalize",
        zjjrvl_Appraisal::Broken(_) => "Broken",
    }
}

#[test]
fn jjtg_validate_appraise_canonical_is_clean() {
    let bytes = serde_json::to_string_pretty(&canonical_gallops()).unwrap().into_bytes();
    match zjjrvl_appraise(&bytes) {
        zjjrvl_Appraisal::Canonical(_) => {}
        other => panic!("canonical gallops must appraise Canonical, got {}", appraisal_name(&other)),
    }
}

#[test]
fn jjtg_validate_appraise_compact_whitespace_normalizes() {
    // Same data, non-canonical formatting (compact, no pretty indentation) — valid but not
    // canonical, so it normalizes and the canonical struct preserves the data.
    let compact = serde_json::to_string(&canonical_gallops()).unwrap().into_bytes();
    match zjjrvl_appraise(&compact) {
        zjjrvl_Appraisal::Normalize(canon, _census) => {
            let pace = canon.heats["₣AC"].paces.get("₢ACAAA").unwrap();
            assert_eq!(pace.tacks.len(), 1);
        }
        other => panic!("compact valid gallops must Normalize, got {}", appraisal_name(&other)),
    }
}

#[test]
fn jjtg_validate_appraise_idempotent_after_normalize() {
    // Re-running validate after a normalization yields clean (exit 2, then 0).
    let compact = serde_json::to_string(&canonical_gallops()).unwrap().into_bytes();
    let canon = match zjjrvl_appraise(&compact) {
        zjjrvl_Appraisal::Normalize(c, _) => *c,
        other => panic!("expected Normalize, got {}", appraisal_name(&other)),
    };
    let canon_bytes = serde_json::to_string_pretty(&canon).unwrap().into_bytes();
    assert!(
        matches!(zjjrvl_appraise(&canon_bytes), zjjrvl_Appraisal::Canonical(_)),
        "appraising the normalized form must be Canonical (idempotent)"
    );
}

#[test]
fn jjtg_validate_appraise_garbage_is_broken() {
    assert!(matches!(zjjrvl_appraise(b"not json at all"), zjjrvl_Appraisal::Broken(_)));
}

#[test]
fn jjtg_validate_appraise_invariant_violation_is_broken() {
    // Parses, but next_heat_seed is 3 chars (must be 2) → semantic invariant failure → Broken,
    // never a silent fix.
    let mut g = canonical_gallops();
    g.next_heat_seed = "ABC".to_string();
    let bytes = serde_json::to_string_pretty(&g).unwrap().into_bytes();
    assert!(matches!(zjjrvl_appraise(&bytes), zjjrvl_Appraisal::Broken(_)));
}

// ===== top-level heat_order/heats reconcile + twin invariant =====

/// canonical_gallops with a well-formed second heat ₣AB in both heats and heat_order —
/// the base the reconcile tests diverge one axis from.
fn two_heat_gallops() -> jjrg_Gallops {
    let mut g = canonical_gallops();
    let (hk, heat) = make_valid_heat("AB", "second-heat");
    g.heat_order.push(hk.clone());
    g.heats.insert(hk, heat);
    g
}

#[test]
fn jjtg_reconcile_dedups_heat_order_keep_first() {
    // A merge concatenating heat_order duplicates a firemark; reconcile keeps the first
    // (highest-priority) slot and drops the rest — the observed jjx_list double-render.
    let mut g = two_heat_gallops(); // heat_order == [₣AC, ₣AB]
    g.heat_order.push("₣AC".to_string()); // [₣AC, ₣AB, ₣AC]
    let report = jjrg_reconcile(&mut g);
    assert_eq!(g.heat_order, vec!["₣AC".to_string(), "₣AB".to_string()]);
    assert!(report.iter().any(|r| r.contains("deduped")), "report names the dedup: {:?}", report);
}

#[test]
fn jjtg_reconcile_drops_orphan_heat_order_entry() {
    // A heat_order slot naming no heat renders nothing (muster filter_maps it); reconcile drops it.
    let mut g = canonical_gallops(); // heat_order == [₣AC]
    g.heat_order.push("₣ZZ".to_string()); // ₣ZZ has no heat
    let report = jjrg_reconcile(&mut g);
    assert_eq!(g.heat_order, vec!["₣AC".to_string()]);
    assert!(report.iter().any(|r| r.contains("naming no heat")), "report names the orphan: {:?}", report);
}

#[test]
fn jjtg_reconcile_appends_invisible_heat() {
    // A heat in heats but absent from heat_order is invisible (never rendered); reconcile appends
    // it in heats key order so it can never silently vanish.
    let mut g = canonical_gallops(); // heat_order == [₣AC]
    let (hk, heat) = make_valid_heat("AB", "invisible-heat");
    g.heats.insert(hk, heat); // heats only — never added to heat_order
    let report = jjrg_reconcile(&mut g);
    assert_eq!(g.heat_order, vec!["₣AC".to_string(), "₣AB".to_string()]);
    assert!(report.iter().any(|r| r.contains("missing from heat_order")), "report names the append: {:?}", report);
}

#[test]
fn jjtg_reconcile_idempotent() {
    let mut g = two_heat_gallops();
    g.heat_order.push("₣AC".to_string()); // diverged
    assert!(!jjrg_reconcile(&mut g).is_empty(), "first reconcile repairs");
    assert!(jjrg_reconcile(&mut g).is_empty(), "second reconcile is a no-op on an already-clean store");
}

#[test]
fn jjtg_validate_top_level_heat_order_dup() {
    let mut g = canonical_gallops();
    g.heat_order.push("₣AC".to_string()); // [₣AC, ₣AC]
    let errors = g.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("heat_order contains duplicate entries")));
}

#[test]
fn jjtg_validate_top_level_invisible_heat() {
    let mut g = canonical_gallops();
    let (hk, heat) = make_valid_heat("AB", "invisible-heat");
    g.heats.insert(hk, heat); // heats only
    let errors = g.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("heats contains firemarks not in heat_order")));
}

#[test]
fn jjtg_validate_appraise_diverged_heat_order_normalizes() {
    // The live-store repair path: a store whose heat_order carries a merge-dup appraises Normalize
    // (exit 2, self-describing census), and re-appraising the normalized form is Canonical (exit 0)
    // — the in-session fix for the observed ₣B4 double-render, no reprieve episode.
    let mut g = canonical_gallops();
    g.heat_order.push("₣AC".to_string()); // duplicate slot
    let bytes = serde_json::to_string_pretty(&g).unwrap().into_bytes();
    let canon = match zjjrvl_appraise(&bytes) {
        zjjrvl_Appraisal::Normalize(c, census) => {
            assert!(census.contains("reconcile"), "normalize census names the reconcile: {}", census);
            *c
        }
        other => panic!("diverged heat_order must Normalize, got {}", appraisal_name(&other)),
    };
    assert_eq!(canon.heat_order, vec!["₣AC".to_string()]);
    let canon_bytes = serde_json::to_string_pretty(&canon).unwrap().into_bytes();
    assert!(
        matches!(zjjrvl_appraise(&canon_bytes), zjjrvl_Appraisal::Canonical(_)),
        "re-appraising the reconciled form is Canonical (idempotent)"
    );
}

// ===== hark (retrospective load — jjrg_hark / jjdr_hark) =====

#[test]
fn jjtg_hark_accepts_canonical_bytes() {
    // The common case: a recent revision's store is already canonical, so hark loads it
    // exactly as a disk load would.
    let bytes = serde_json::to_string_pretty(&canonical_gallops()).unwrap().into_bytes();
    let harked = jjrg_Gallops::jjrg_hark(&bytes).expect("hark loads canonical bytes");
    assert_eq!(harked.heats["₣AC"].paces.get("₢ACAAA").unwrap().tacks.len(), 1);
}

#[test]
fn jjtg_hark_skips_roundtrip_where_disk_load_rejects() {
    // Compact (non-canonical) serialization of a canonical-schema store: the disk load
    // rejects it on the round-trip canonical check, but jjrg_hark — which never re-saves
    // these historical bytes — skips that check and loads the same bytes. This is the one
    // behavior that distinguishes the retrospective path from jjrg_load.
    let compact = serde_json::to_string(&canonical_gallops()).unwrap().into_bytes();

    let dir = JjkTestDir::new("jjtg_hark_roundtrip_contrast");
    let path = dir.path().join("jjg_gallops.json");
    std::fs::write(&path, &compact).unwrap();
    assert!(
        jjrg_Gallops::jjrg_load(&path).is_err(),
        "disk load must reject non-canonical bytes on the round-trip check"
    );

    let harked = jjrg_Gallops::jjrg_hark(&compact).expect("hark loads non-canonical historical bytes");
    assert_eq!(harked.heats["₣AC"].paces.get("₢ACAAA").unwrap().tacks.len(), 1);
}

#[test]
fn jjtg_hark_rejects_garbage() {
    // A revision too old for the reprieve floor (or otherwise unparseable) fails to
    // deserialize — the "too old to hark" signal.
    assert!(jjrg_Gallops::jjrg_hark(b"not json at all").is_err());
}

#[test]
fn jjtg_hark_rejects_invariant_violation() {
    // Hark shares jjdr_load's semantic validation, so a structurally-readable but
    // invariant-violating store still fails — the unbypassable-validation invariant holds.
    let mut g = canonical_gallops();
    g.next_heat_seed = "ABC".to_string();
    let bytes = serde_json::to_string_pretty(&g).unwrap().into_bytes();
    assert!(jjrg_Gallops::jjrg_hark(&bytes).is_err());
}

#[test]
fn jjtg_validate_run_clean_returns_exit0_and_leaves_file() {
    // The clean path neither locks nor commits, so it runs without a git repo. The file must be
    // byte-identical afterward.
    let dir = JjkTestDir::new("jjtg_validate_clean");
    let path = dir.path().join("jjg_gallops.json");
    let bytes = serde_json::to_string_pretty(&canonical_gallops()).unwrap().into_bytes();
    std::fs::write(&path, &bytes).unwrap();
    let (code, _out) = jjrvl_run_validate(jjrvl_ValidateArgs { file: path.clone(), size_limit: 50_000 });
    assert_eq!(code, 0, "canonical file is clean");
    assert_eq!(std::fs::read(&path).unwrap(), bytes, "clean path must not touch the file");
}

#[test]
fn jjtg_validate_run_broken_returns_exit1_and_leaves_file() {
    let dir = JjkTestDir::new("jjtg_validate_broken");
    let path = dir.path().join("jjg_gallops.json");
    let garbage = b"{ not valid".to_vec();
    std::fs::write(&path, &garbage).unwrap();
    let (code, _out) = jjrvl_run_validate(jjrvl_ValidateArgs { file: path.clone(), size_limit: 50_000 });
    assert_eq!(code, 1, "unparseable file is broken");
    assert_eq!(std::fs::read(&path).unwrap(), garbage, "broken path must not touch the file");
}

// Helper to create a minimal valid Gallops structure
fn make_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    }
}

// Helper to create a valid Tack (uses JJRG_UNKNOWN_BASIS for consistent basis format)
fn make_valid_tack(state: jjrg_PaceState, silks: &str) -> jjrg_Tack {
    jjrg_Tack {
        ts: "260101-1200".to_string(),
        state,
        tier: None,
        effort: None,
        text: vec!["Test tack text".to_string()],
        silks: silks.to_string(),
        basis: JJRG_UNKNOWN_BASIS.to_string(),
    }
}

// Helper to create a valid Pace
fn make_valid_pace(heat_id: &str, silks: &str) -> (String, jjrg_Pace) {
    let pace_key = format!("₢{}AAA", heat_id);
    let pace = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, silks)],
        ..Default::default()
    };
    (pace_key, pace)
}

// Helper to create a valid Heat
fn make_valid_heat(heat_id: &str, silks: &str) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let (pace_key, pace) = make_valid_pace(heat_id, "test-pace");
    let mut paces = BTreeMap::new();
    paces.insert(pace_key.clone(), pace);

    let heat = jjrg_Heat {
        silks: silks.to_string(),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec![pace_key],
        paces,
    };
    (heat_key, heat)
}

// ===== Validation helper tests =====

#[test]
fn jjtg_zjjrg_is_base64_valid() {
    assert!(zjjrg_is_base64("AB"));
    assert!(zjjrg_is_base64("ABCDE"));
    assert!(zjjrg_is_base64("Az09-_"));
}

#[test]
fn jjtg_zjjrg_is_base64_invalid() {
    assert!(!zjjrg_is_base64("A!"));
    assert!(!zjjrg_is_base64("A B"));
    assert!(!zjjrg_is_base64("+/"));
}

#[test]
fn jjtg_zjjrg_is_kebab_case_valid() {
    assert!(zjjrg_is_kebab_case("test"));
    assert!(zjjrg_is_kebab_case("test-pace"));
    assert!(zjjrg_is_kebab_case("my-cool-heat123"));
    assert!(zjjrg_is_kebab_case("a1-b2-c3"));
    assert!(zjjrg_is_kebab_case("Test"));
    assert!(zjjrg_is_kebab_case("MyHeat-Name"));
    assert!(zjjrg_is_kebab_case("ALLCAPS"));
    assert!(zjjrg_is_kebab_case("mixedCase-123"));
}

#[test]
fn jjtg_zjjrg_is_kebab_case_invalid() {
    assert!(!zjjrg_is_kebab_case(""));
    assert!(!zjjrg_is_kebab_case("test_pace"));
    assert!(!zjjrg_is_kebab_case("-test"));
    assert!(!zjjrg_is_kebab_case("test-"));
    assert!(!zjjrg_is_kebab_case("test--pace"));
}

#[test]
fn jjtg_zjjrg_is_yymmdd_valid() {
    assert!(zjjrg_is_yymmdd("260101"));
    assert!(zjjrg_is_yymmdd("991231"));
}

#[test]
fn jjtg_zjjrg_is_yymmdd_invalid() {
    assert!(!zjjrg_is_yymmdd("2601"));
    assert!(!zjjrg_is_yymmdd("26010101"));
    assert!(!zjjrg_is_yymmdd("26-01-01"));
    assert!(!zjjrg_is_yymmdd("26ab01"));
}

#[test]
fn jjtg_zjjrg_is_yymmdd_hhmm_valid() {
    assert!(zjjrg_is_yymmdd_hhmm("260101-1234"));
    assert!(zjjrg_is_yymmdd_hhmm("991231-2359"));
}

#[test]
fn jjtg_zjjrg_is_yymmdd_hhmm_invalid() {
    assert!(!zjjrg_is_yymmdd_hhmm("260101"));
    assert!(!zjjrg_is_yymmdd_hhmm("260101-123"));
    assert!(!zjjrg_is_yymmdd_hhmm("260101-12345"));
    assert!(!zjjrg_is_yymmdd_hhmm("26010112:34"));
}

// ===== Gallops validation tests =====

#[test]
fn jjtg_validate_minimal_valid_gallops() {
    let gallops = make_valid_gallops();
    assert!(gallops.jjrg_validate().is_ok());
}

#[test]
fn jjtg_validate_gallops_with_heat() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    // heat_order must carry the heat, else the top-level twin invariant reports it.
    gallops.heat_order.push(heat_key.clone());
    gallops.heats.insert(heat_key, heat);
    assert!(gallops.jjrg_validate().is_ok());
}

#[test]
fn jjtg_validate_invalid_next_heat_seed_length() {
    let mut gallops = make_valid_gallops();
    gallops.next_heat_seed = "ABC".to_string();
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("next_heat_seed must be 2 characters")));
}

#[test]
fn jjtg_validate_invalid_next_heat_seed_chars() {
    let mut gallops = make_valid_gallops();
    gallops.next_heat_seed = "A!".to_string();
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("invalid base64 characters")));
}

#[test]
fn jjtg_validate_heat_key_missing_prefix() {
    let mut gallops = make_valid_gallops();
    let (_, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert("AB".to_string(), heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("key must start with '₣'")));
}

#[test]
fn jjtg_validate_heat_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    heat.silks = "Invalid_Silks".to_string();
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("silks must be non-empty alphanumeric-kebab")));
}

#[test]
fn jjtg_validate_heat_invalid_creation_time() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    heat.creation_time = "2026-01-01".to_string();
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("creation_time must be YYMMDD format")));
}

#[test]
fn jjtg_validate_gallops_invalid_next_pace_seed() {
    let mut gallops = make_valid_gallops();
    gallops.next_pace_seed = "AB".to_string(); // the global seed must be 5 chars
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("next_pace_seed must be 5 characters")));
}

#[test]
fn jjtg_validate_order_paces_mismatch() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    // Add extra entry to order that doesn't exist in paces
    heat.order.push("₢ABXXX".to_string());
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("order contains keys not in paces")));
}

#[test]
fn jjtg_validate_coronet_cross_heat_uniqueness() {
    // A Coronet is a flat global id (JJS0 jjdt_coronet), living in exactly one heat.
    // The retired "must embed parent heat identity" rule is replaced by cross-heat
    // uniqueness: the same immutable Coronet appearing in two heats is the corruption
    // to catch.
    let mut gallops = make_valid_gallops();
    let (ab_key, ab_heat) = make_valid_heat("AB", "heat-ab");
    let dup_coronet = ab_heat.order[0].clone(); // ₢ABAAA

    // A second heat that (illegally) holds the very same Coronet.
    let (cd_key, mut cd_heat) = make_valid_heat("CD", "heat-cd");
    cd_heat.paces.clear();
    cd_heat.order.clear();
    cd_heat.paces.insert(
        dup_coronet.clone(),
        jjrg_Pace { tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "dup-pace")], ..Default::default() },
    );
    cd_heat.order.push(dup_coronet);

    gallops.heats.insert(ab_key, ab_heat);
    gallops.heats.insert(cd_key, cd_heat);

    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("more than one heat")));
}

#[test]
fn jjtg_qualify_coronet_renders_live_heat() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat"); // holds ₢ABAAA
    let coronet = heat.order[0].clone(); // ₢ABAAA
    gallops.heats.insert(heat_key, heat);

    // Heat-qualified emission (JJS0 jjdt_coronet): ₢ + live heat firemark (AB) +
    // the interpunct + the 5-char body.
    assert_eq!(gallops.jjrg_qualify_coronet(&coronet), "₢AB·ABAAA");
    // Accepts bare, glyphless, or already-qualified input — all normalize first.
    assert_eq!(gallops.jjrg_qualify_coronet("ABAAA"), "₢AB·ABAAA");
    assert_eq!(gallops.jjrg_qualify_coronet("₢AB·ABAAA"), "₢AB·ABAAA");
}

#[test]
fn jjtg_qualify_coronet_fail_soft_when_unaffiliated() {
    let gallops = make_valid_gallops(); // no heats
    // A Coronet no heat harbours renders bare — a display path never fabricates an
    // affiliation it cannot prove (JJS0 jjdt_coronet "Display and ingest").
    assert_eq!(gallops.jjrg_qualify_coronet("₢CAAAB"), "₢CAAAB");
}

#[test]
fn jjtg_validate_tack_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    // Get the first pace's first tack and modify its silks
    if let Some(pace) = heat.paces.values_mut().next() {
        if let Some(tack) = pace.tacks.first_mut() {
            tack.silks = "".to_string();
        }
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("silks must be non-empty alphanumeric-kebab")));
}

#[test]
fn jjtg_validate_pace_empty_tacks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks.clear();
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("tacks array must not be empty")));
}

#[test]
fn jjtg_validate_tack_invalid_ts() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].ts = "invalid".to_string();
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("ts must be YYMMDD-HHMM format")));
}

#[test]
fn jjtg_validate_tack_empty_text() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].text = vec!["".to_string()];
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("text must not be empty")));
}

#[test]
fn jjtg_validate_complete_state_valid() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].state = jjrg_PaceState::Complete;
    }
    gallops.heat_order.push(heat_key.clone());
    gallops.heats.insert(heat_key, heat);
    assert!(gallops.jjrg_validate().is_ok());
}

#[test]
fn jjtg_validate_abandoned_state_valid() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].state = jjrg_PaceState::Abandoned;
    }
    gallops.heat_order.push(heat_key.clone());
    gallops.heats.insert(heat_key, heat);
    assert!(gallops.jjrg_validate().is_ok());
}

// ===== Load/Save round-trip tests =====

#[test]
fn jjtg_serialize_deserialize_roundtrip() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    let json = serde_json::to_string_pretty(&gallops).unwrap();
    let restored: jjrg_Gallops = serde_json::from_str(&json).unwrap();

    assert_eq!(gallops.next_heat_seed, restored.next_heat_seed);
    assert_eq!(gallops.heats.len(), restored.heats.len());
}

#[test]
fn jjtg_multiple_errors_collected() {
    let mut gallops = jjrg_Gallops {
        next_heat_seed: "!!!".to_string(), // Wrong length and chars
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    };
    let (_, mut heat) = make_valid_heat("AB", "my-heat");
    heat.silks = "Invalid_Silks".to_string(); // Has underscore
    heat.creation_time = "invalid".to_string(); // Not YYMMDD
    gallops.heats.insert("₣AB".to_string(), heat);

    let errors = gallops.jjrg_validate().unwrap_err();
    // Should have multiple errors
    assert!(errors.len() >= 3);
}

// ===== Seed increment tests =====

#[test]
fn jjtg_zjjrg_increment_seed_simple() {
    assert_eq!(zjjrg_increment_seed("AA"), "AB");
    assert_eq!(zjjrg_increment_seed("AB"), "AC");
    assert_eq!(zjjrg_increment_seed("Az"), "A0");
}

#[test]
fn jjtg_zjjrg_increment_seed_carry() {
    // '_' is position 63, should wrap to 'A' (position 0) and carry
    assert_eq!(zjjrg_increment_seed("A_"), "BA");
    assert_eq!(zjjrg_increment_seed("__"), "AA"); // Full wrap around
}

#[test]
fn jjtg_zjjrg_increment_seed_three_chars() {
    assert_eq!(zjjrg_increment_seed("AAA"), "AAB");
    assert_eq!(zjjrg_increment_seed("AA_"), "ABA");
    assert_eq!(zjjrg_increment_seed("A__"), "BAA");
    assert_eq!(zjjrg_increment_seed("___"), "AAA");
}

// ===== Write operation tests =====

#[test]
fn jjtg_nominate_creates_heat() {
    let mut gallops = make_valid_gallops();
    let td = JjkTestDir::new("jjk_test_nominate");

    let args = jjrg_NominateArgs {
        silks: "test-heat".to_string(),
        created: "260113".to_string(),
    };

    let result = gallops.jjrg_nominate(args, td.path()).unwrap();

    // Check result
    assert!(result.firemark.starts_with('₣'));

    // Check heat was created
    assert!(gallops.heats.contains_key(&result.firemark));
    let heat = gallops.heats.get(&result.firemark).unwrap();
    assert_eq!(heat.silks, "test-heat");
    assert_eq!(heat.creation_time, "260113");
    assert_eq!(heat.status, jjrg_HeatStatus::Stabled);
    assert!(heat.order.is_empty());

    // Check heat seed was incremented
    assert_eq!(gallops.next_heat_seed, "AC");
}

#[test]
fn jjtg_nominate_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir();

    let args = jjrg_NominateArgs {
        silks: "invalid_silks".to_string(),
        created: "260113".to_string(),
    };

    let result = gallops.jjrg_nominate(args, &temp_dir);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("silks must be non-empty alphanumeric-kebab"));
}

#[test]
fn jjtg_nominate_invalid_created() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir();

    let args = jjrg_NominateArgs {
        silks: "test-heat".to_string(),
        created: "2026-01-13".to_string(),
    };

    let result = gallops.jjrg_nominate(args, &temp_dir);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("created must be YYMMDD format"));
}

#[test]
fn jjtg_slate_creates_pace() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "test-pace".to_string(),
        text: "Do something useful".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args).unwrap();

    // Check result — a flat global coronet minted from the gallops seed (CAAAA),
    // no embedded heat identity (JJS0 jjdt_coronet).
    assert!(result.coronet.starts_with('₢'));
    assert_eq!(result.coronet, "₢CAAAA");

    // Check pace was created
    let heat = gallops.heats.get(&heat_key).unwrap();
    assert!(heat.paces.contains_key(&result.coronet));
    let pace = heat.paces.get(&result.coronet).unwrap();
    assert_eq!(pace.tacks.len(), 1);
    assert_eq!(pace.tacks[0].silks, "test-pace");
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Rough);
    assert_eq!(pace.tacks[0].text, vec!["Do something useful".to_string()]);

    // Check order was updated
    assert!(heat.order.contains(&result.coronet));

    // Check the global pace seed advanced (CAAAA → CAAAB).
    assert_eq!(gallops.next_pace_seed, "CAAAB");
}

// ===== original-intent capture (dictation / precis / slated / redocket_count) =====

/// Slate a pace carrying the full original-intent capture into heat AB.
fn slate_intent_pace(gallops: &mut jjrg_Gallops) -> String {
    gallops.jjrg_slate(jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "intent-pace".to_string(),
        text: "The docket".to_string(),
        dictation: Some("operator's raw words".to_string()),
        precis: Some("distilled intent".to_string()),
        slated: "260715-1030".to_string(),
        before: None,
        after: None,
        first: false,
    }).unwrap().coronet
}

#[test]
fn jjtg_slate_freezes_original_intent() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key.clone(), heat);

    let coronet = slate_intent_pace(&mut gallops);

    let pace = gallops.heats[&heat_key].paces.get(&coronet).unwrap();
    assert_eq!(pace.dictation.as_deref(), Some("operator's raw words"));
    assert_eq!(pace.precis.as_deref(), Some("distilled intent"));
    assert_eq!(pace.slated.as_deref(), Some("260715-1030"));
    assert_eq!(pace.redocket_count, 0);
}

#[test]
fn jjtg_revise_docket_bumps_counter_and_preserves_intent() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key.clone(), heat);
    let coronet = slate_intent_pace(&mut gallops);

    // Single and mass reslate both funnel through jjrg_revise_docket; two
    // revisions → count 2, and the frozen capture is untouched.
    gallops.jjrg_revise_docket(&coronet, "second docket", "0000000", "260716-0900").unwrap();
    gallops.jjrg_revise_docket(&coronet, "third docket", "0000000", "260717-0900").unwrap();

    let pace = gallops.heats[&heat_key].paces.get(&coronet).unwrap();
    assert_eq!(pace.redocket_count, 2, "each docket revision bumps the drift counter");
    assert_eq!(pace.dictation.as_deref(), Some("operator's raw words"), "dictation is frozen");
    assert_eq!(pace.precis.as_deref(), Some("distilled intent"), "precis is frozen");
    assert_eq!(pace.slated.as_deref(), Some("260715-1030"), "slated is frozen");
    assert_eq!(pace.tacks[0].text, vec!["third docket".to_string()]);
}

#[test]
fn jjtg_bridle_and_release_do_not_bump_counter() {
    // Bridle and release replace the tack but are not docket revisions —
    // the increment lives in jjrg_revise_docket, not jjrg_set_tack.
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key.clone(), heat);
    let coronet = slate_intent_pace(&mut gallops);

    gallops.jjrg_bridle(&coronet, jjrg_Tier::Sonnet, None, "0000000", "260716-0900").unwrap();
    gallops.jjrg_release(&coronet, "0000000", "260716-1000").unwrap();

    let pace = gallops.heats[&heat_key].paces.get(&coronet).unwrap();
    assert_eq!(pace.redocket_count, 0, "bridle/release never bump the counter");
    assert_eq!(pace.dictation.as_deref(), Some("operator's raw words"));
}

#[test]
fn jjtg_draft_carries_intent_unchanged() {
    // Draft (and restring, which routes through it) is a relocation: the
    // capture travels with the work — never re-frozen, counter never reset.
    let mut g = make_two_heat_gallops(jjrg_PaceState::Rough, None, None);
    {
        let pace = g.heats.get_mut("₣AC").unwrap().paces.get_mut("₢ACAAA").unwrap();
        pace.dictation = Some("raw".to_string());
        pace.precis = Some("distilled".to_string());
        pace.slated = Some("260701-0800".to_string());
        pace.redocket_count = 3;
    }
    let result = g.jjrg_draft(jjrg_DraftArgs {
        coronet: "ACAAA".to_string(),
        to: "AD".to_string(),
        before: None,
        after: None,
        first: false,
    }).unwrap();
    let pace = &g.heats["₣AD"].paces[&result.new_coronet];
    assert_eq!(pace.dictation.as_deref(), Some("raw"));
    assert_eq!(pace.precis.as_deref(), Some("distilled"));
    assert_eq!(pace.slated.as_deref(), Some("260701-0800"));
    assert_eq!(pace.redocket_count, 3);
}

#[test]
fn jjtg_pace_intent_serde_additive() {
    // The no-reprieve-episode guarantee: an old-shape pace (tacks only) loads
    // with the capture absent and re-serializes byte-identical — none of the
    // new keys appear.
    let old = r#"{"jjgpn_tacks":[{"jjgtn_ts":"260101-1200","jjgtn_state":"jjgte_rough","jjgtn_text":["a"],"jjgtn_silks":"x","jjgtn_basis":"0000000"}]}"#;
    let pace: jjrg_Pace = serde_json::from_str(old).unwrap();
    assert!(pace.dictation.is_none());
    assert!(pace.precis.is_none());
    assert!(pace.slated.is_none());
    assert_eq!(pace.redocket_count, 0);
    let out = serde_json::to_string(&pace).unwrap();
    assert_eq!(out, old, "untouched old store re-serializes byte-identical");

    // A populated capture rides the jjgpn_ wire keys and round-trips.
    let mut pace2 = pace.clone();
    pace2.dictation = Some("raw".to_string());
    pace2.precis = Some("distilled".to_string());
    pace2.slated = Some("260715-1030".to_string());
    pace2.redocket_count = 2;
    let out2 = serde_json::to_string(&pace2).unwrap();
    for key in ["jjgpn_dictation", "jjgpn_precis", "jjgpn_slated", "jjgpn_redocket_count"] {
        assert!(out2.contains(key), "missing wire key {}", key);
    }
    let back: jjrg_Pace = serde_json::from_str(&out2).unwrap();
    assert_eq!(back.dictation.as_deref(), Some("raw"));
    assert_eq!(back.precis.as_deref(), Some("distilled"));
    assert_eq!(back.slated.as_deref(), Some("260715-1030"));
    assert_eq!(back.redocket_count, 2);
}

// Build a heat whose order carries history ahead of live work:
// AAA complete, AAB bridled, AAC rough.
fn make_heat_with_history(heat_id: &str) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let mut paces = BTreeMap::new();
    let mut order = Vec::new();
    for (suffix, state, silks) in [
        ("AAA", jjrg_PaceState::Complete, "wrapped-pace"),
        ("AAB", jjrg_PaceState::Bridled, "bridled-pace"),
        ("AAC", jjrg_PaceState::Rough, "rough-pace"),
    ] {
        let coronet = format!("₢{}{}", heat_id, suffix);
        paces.insert(coronet.clone(), jjrg_Pace { tacks: vec![make_valid_tack(state, silks)], ..Default::default() });
        order.push(coronet);
    }
    let heat = jjrg_Heat {
        silks: "my-heat".to_string(),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order,
        paces,
    };
    (heat_key, heat)
}

#[test]
fn jjtg_slate_first_aims_at_first_actionable_slot() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_heat_with_history("AB");
    gallops.heats.insert(heat_key.clone(), heat);

    let result = gallops.jjrg_slate(jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "chivvied-pace".to_string(),
        text: "Do this next".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: None,
        first: true,
    }).unwrap();

    let heat = gallops.heats.get(&heat_key).unwrap();
    // Head of the remaining work: after the wrapped pace, ahead of the bridled
    // one — bridled is actionable, so the new pace precedes it rather than
    // slotting in behind a Rough-only scan.
    assert_eq!(heat.order[0], "₢ABAAA");
    assert_eq!(heat.order[1], result.coronet);
    assert_eq!(heat.order[2], "₢ABAAB");
    assert_eq!(heat.order[3], "₢ABAAC");
}

#[test]
fn jjtg_slate_first_appends_when_nothing_is_actionable() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_heat_with_history("AB");
    for coronet in heat.order.clone() {
        heat.paces.get_mut(&coronet).unwrap().tacks[0].state = jjrg_PaceState::Complete;
    }
    gallops.heats.insert(heat_key.clone(), heat);

    let result = gallops.jjrg_slate(jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "chivvied-pace".to_string(),
        text: "Do this next".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: None,
        first: true,
    }).unwrap();

    let heat = gallops.heats.get(&heat_key).unwrap();
    // No actionable slot to precede — the pace lands at the end, not above the record.
    assert_eq!(heat.order.last().unwrap(), &result.coronet);
}

#[test]
fn jjtg_draft_first_aims_at_destination_first_actionable_slot() {
    let mut gallops = make_valid_gallops();
    let (dest_key, dest) = make_heat_with_history("AB");
    let (src_key, src) = make_valid_heat("CD", "src-heat");
    gallops.heats.insert(dest_key.clone(), dest);
    gallops.heats.insert(src_key, src);

    let result = gallops.jjrg_draft(jjrg_DraftArgs {
        coronet: "₢CDAAA".to_string(),
        to: "AB".to_string(),
        before: None,
        after: None,
        first: true,
    }).unwrap();

    let dest = gallops.heats.get(&dest_key).unwrap();
    // Same rule as slate and rail: the destination's completed work keeps its
    // place, and the relocated pace lands at the head of what remains.
    assert_eq!(dest.order[0], "₢ABAAA");
    assert_eq!(dest.order[1], result.new_coronet);
    assert_eq!(dest.order[2], "₢ABAAB");
}

#[test]
fn jjtg_slate_heat_not_found() {
    let mut gallops = make_valid_gallops();

    let args = jjrg_SlateArgs {
        firemark: "CD".to_string(),
        silks: "test-pace".to_string(),
        text: "Do something".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn jjtg_slate_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "invalid_silks".to_string(),
        text: "Do something".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("silks must be non-empty alphanumeric-kebab"));
}

#[test]
fn jjtg_slate_empty_text() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "test-pace".to_string(),
        text: "".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("text must not be empty"));
}

#[test]
fn jjtg_slate_with_first_inserts_at_beginning() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let existing_pace = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "new-first-pace".to_string(),
        text: "This should be first".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: None,
        first: true,
    };

    let result = gallops.jjrg_slate(args).unwrap();

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], result.coronet); // New pace is first
    assert_eq!(heat.order[1], existing_pace); // Existing pace moved to second
}

#[test]
fn jjtg_slate_with_before_inserts_at_position() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Add a second pace
    let pace2_key = "₢ABAAB".to_string();
    let pace2 = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace")],
        ..Default::default()
    };
    heat.paces.insert(pace2_key.clone(), pace2);
    heat.order.push(pace2_key.clone());

    let first_pace = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Insert before the second pace
    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "inserted-pace".to_string(),
        text: "Insert before second".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: Some(pace2_key.clone()),
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args).unwrap();

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], first_pace); // Original first unchanged
    assert_eq!(heat.order[1], result.coronet); // New pace inserted
    assert_eq!(heat.order[2], pace2_key); // Second pace moved
}

#[test]
fn jjtg_slate_with_after_inserts_at_position() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Add a second pace
    let pace2_key = "₢ABAAB".to_string();
    let pace2 = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace")],
        ..Default::default()
    };
    heat.paces.insert(pace2_key.clone(), pace2);
    heat.order.push(pace2_key.clone());

    let first_pace = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Insert after the first pace
    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "inserted-pace".to_string(),
        text: "Insert after first".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: Some(first_pace.clone()),
        first: false,
    };

    let result = gallops.jjrg_slate(args).unwrap();

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], first_pace); // Original first unchanged
    assert_eq!(heat.order[1], result.coronet); // New pace inserted after first
    assert_eq!(heat.order[2], pace2_key); // Second pace at end
}

#[test]
fn jjtg_slate_mutual_exclusivity() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let existing_pace = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    // Try with both before and first
    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "bad-pace".to_string(),
        text: "Should fail".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: Some(existing_pace),
        after: None,
        first: true,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Only one of"));
}

#[test]
fn jjtg_slate_before_invalid_coronet() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "new-pace".to_string(),
        text: "Test".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: Some("₢ABXXX".to_string()), // Non-existent
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in heat"));
}

// Move mode tests

#[test]
fn jjtg_rail_move_first() {
    // Test that --first moves to first ACTIONABLE position, not absolute position 0
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Make the first pace (from make_valid_heat) complete
    let pace1_key = heat.order[0].clone();
    heat.paces.get_mut(&pace1_key).unwrap().tacks[0].state = jjrg_PaceState::Complete;

    // Add pace2 (complete) and pace3 (rough) and pace4 (rough)
    let pace2_key = "₢ABAAB".to_string();
    let pace3_key = "₢ABAAC".to_string();
    let pace4_key = "₢ABAAD".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Complete, "second-pace")],
        ..Default::default()
    });
    heat.paces.insert(pace3_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "third-pace")],
        ..Default::default()
    });
    heat.paces.insert(pace4_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "fourth-pace")],
        ..Default::default()
    });
    heat.order.push(pace2_key.clone());
    heat.order.push(pace3_key.clone());
    heat.order.push(pace4_key.clone());

    // Order: [complete1, complete2, rough3, rough4]
    gallops.heats.insert(heat_key.clone(), heat);

    // Move fourth pace (rough) to first actionable position
    // Should go BEFORE pace3 (first rough), not before pace1 (absolute first)
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace4_key.clone()),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    // Expected order: [complete1, complete2, rough4, rough3]
    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace1_key, "complete1 should stay at index 0");
    assert_eq!(heat.order[1], pace2_key, "complete2 should stay at index 1");
    assert_eq!(heat.order[2], pace4_key, "rough4 should move to first actionable (index 2)");
    assert_eq!(heat.order[3], pace3_key, "rough3 should shift to index 3");
}

#[test]
fn jjtg_rail_move_first_all_complete() {
    // Test that --first moves to end when all paces are complete
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Make all paces complete
    let pace1_key = heat.order[0].clone();
    heat.paces.get_mut(&pace1_key).unwrap().tacks[0].state = jjrg_PaceState::Complete;

    let pace2_key = "₢ABAAB".to_string();
    let pace3_key = "₢ABAAC".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Complete, "second-pace")],
        ..Default::default()
    });
    heat.paces.insert(pace3_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Complete, "third-pace")],
        ..Default::default()
    });
    heat.order.push(pace2_key.clone());
    heat.order.push(pace3_key.clone());

    // Order: [complete1, complete2, complete3]
    gallops.heats.insert(heat_key.clone(), heat);

    // Move first pace to "first actionable" - but all are complete, so goes to end
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace1_key.clone()),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    // Expected order: [complete2, complete3, complete1]
    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace2_key);
    assert_eq!(heat.order[1], pace3_key);
    assert_eq!(heat.order[2], pace1_key, "pace1 should move to end when no actionable paces");
}

#[test]
fn jjtg_rail_move_last() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    let pace2_key = "₢ABAAB".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace")],
        ..Default::default()
    });
    heat.order.push(pace2_key.clone());

    let original_first = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Move first pace to last
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(original_first.clone()),
        before: None,
        after: None,
        first: false,
        last: true,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace2_key);
    assert_eq!(heat.order[1], original_first);
}

#[test]
fn jjtg_rail_move_before() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    let pace2_key = "₢ABAAB".to_string();
    let pace3_key = "₢ABAAC".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace")],
        ..Default::default()
    });
    heat.paces.insert(pace3_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "third-pace")],
        ..Default::default()
    });
    heat.order.push(pace2_key.clone());
    heat.order.push(pace3_key.clone());

    let original_first = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Move third pace before second (from [1,2,3] to [1,3,2])
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace3_key.clone()),
        before: Some(pace2_key.clone()),
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], original_first);
    assert_eq!(heat.order[1], pace3_key);
    assert_eq!(heat.order[2], pace2_key);
}

#[test]
fn jjtg_rail_move_after() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    let pace2_key = "₢ABAAB".to_string();
    let pace3_key = "₢ABAAC".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace")],
        ..Default::default()
    });
    heat.paces.insert(pace3_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "third-pace")],
        ..Default::default()
    });
    heat.order.push(pace2_key.clone());
    heat.order.push(pace3_key.clone());

    let original_first = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Move first pace after second (from [1,2,3] to [2,1,3])
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(original_first.clone()),
        before: None,
        after: Some(pace2_key.clone()),
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace2_key);
    assert_eq!(heat.order[1], original_first);
    assert_eq!(heat.order[2], pace3_key);
}

#[test]
fn jjtg_rail_move_requires_positioning_flag() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace_key),
        before: None,
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires exactly one positioning flag"));
}

#[test]
fn jjtg_rail_move_cannot_position_relative_to_self() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace_key.clone()),
        before: Some(pace_key),
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("relative to itself"));
}

#[test]
fn jjtg_rail_move_cannot_combine_with_order() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![pace_key.clone()],
        move_coronet: Some(pace_key),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot combine --move with positional coronets"));
}

#[test]
fn jjtg_tally_state_transition() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Transition to complete
    let args = jjrg_TallyArgs {
        coronet: pace_key.clone(),
        state: Some(jjrg_PaceState::Complete),
        text: Some("Work completed successfully".to_string()),
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    let pace = heat.paces.get(&pace_key).unwrap();
    assert_eq!(pace.tacks.len(), 1); // Replaced — single current tack
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Complete);
    assert_eq!(pace.tacks[0].text, vec!["Work completed successfully".to_string()]);
}

#[test]
fn jjtg_tally_inherit_state() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // First tack is rough, add new tack without specifying state
    let args = jjrg_TallyArgs {
        coronet: pace_key.clone(),
        state: None, // Inherit
        text: Some("Updated plan text".to_string()),
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    let pace = heat.paces.get(&pace_key).unwrap();
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Rough); // Inherited
    assert_eq!(pace.tacks[0].text, vec!["Updated plan text".to_string()]);
}

#[test]
fn jjtg_tally_inherit_text() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    let original_text = heat.paces.get(&pace_key).unwrap().tacks[0].text.clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Just change state, inherit text
    let args = jjrg_TallyArgs {
        coronet: pace_key.clone(),
        state: Some(jjrg_PaceState::Complete),
        text: None, // Inherit
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    let pace = heat.paces.get(&pace_key).unwrap();
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Complete);
    assert_eq!(pace.tacks[0].text, original_text); // Inherited
}

// ---------------------------------------------------------------------------
// Retention field (jjgrn_retention_since) + its open-time monitum classifier.
// ---------------------------------------------------------------------------

#[test]
fn jjtg_retention_field_omitted_when_off() {
    // An off store carries no value, and skip_serializing_if omits the key entirely —
    // so an off store is byte-identical with or without the field. This is the proof
    // that adding the field needed no reprieve episode: the on-disk bytes are unchanged.
    let gallops = make_valid_gallops();
    assert!(gallops.retention_since.is_none());
    let json = serde_json::to_string(&gallops).unwrap();
    assert!(!json.contains("jjgrn_retention_since"), "off store must omit the key");
}

#[test]
fn jjtg_retention_field_absent_reads_as_none() {
    // A new binary reading an old store that predates the field: serde default → None,
    // and the round-trip is clean (the field never appears).
    let gallops = make_valid_gallops();
    let json = serde_json::to_string(&gallops).unwrap();
    let back: jjrg_Gallops = serde_json::from_str(&json).unwrap();
    assert!(back.retention_since.is_none());
}

#[test]
fn jjtg_retention_field_persists_when_set() {
    let mut gallops = make_valid_gallops();
    gallops.retention_since = Some("2026-06-15".to_string());
    let json = serde_json::to_string(&gallops).unwrap();
    assert!(json.contains("jjgrn_retention_since"));
    let back: jjrg_Gallops = serde_json::from_str(&json).unwrap();
    assert_eq!(back.retention_since, Some("2026-06-15".to_string()));
}

#[test]
fn jjtg_retention_state_off_when_absent_or_empty() {
    let mut gallops = make_valid_gallops();
    gallops.retention_since = None;
    assert!(matches!(jjri_retention_state(&gallops), jjri_RetentionState::Off));
    gallops.retention_since = Some(String::new());
    assert!(matches!(jjri_retention_state(&gallops), jjri_RetentionState::Off));
    gallops.retention_since = Some("   ".to_string());
    assert!(matches!(jjri_retention_state(&gallops), jjri_RetentionState::Off));
}

#[test]
fn jjtg_retention_state_on_when_valid_iso_date() {
    let mut gallops = make_valid_gallops();
    gallops.retention_since = Some("2026-06-15".to_string());
    match jjri_retention_state(&gallops) {
        jjri_RetentionState::On(date) => assert_eq!(date, "2026-06-15"),
        _ => panic!("expected On for a valid ISO date"),
    }
    // Surrounding whitespace is trimmed to the bare date.
    gallops.retention_since = Some("  2026-06-15  ".to_string());
    match jjri_retention_state(&gallops) {
        jjri_RetentionState::On(date) => assert_eq!(date, "2026-06-15"),
        _ => panic!("expected On after trimming whitespace"),
    }
}

#[test]
fn jjtg_retention_state_malformed_when_unparseable() {
    // Non-empty but not a valid YYYY-MM-DD — classified Malformed (never a parse error),
    // carrying the raw value so the operator sees their own typo.
    for bad in ["not-a-date", "2026-13-01", "06/15/2026", "20260615", "2026-02-30"] {
        let mut gallops = make_valid_gallops();
        gallops.retention_since = Some(bad.to_string());
        match jjri_retention_state(&gallops) {
            jjri_RetentionState::Malformed(raw) => assert_eq!(raw, bad),
            _ => panic!("expected Malformed for {:?}", bad),
        }
    }
}

// ===== Bridle designation: schema, transitions, revert triggers =====

/// Rewrite the canonical single pace's current tack to the given state and designation.
fn make_designated_gallops(
    state: jjrg_PaceState,
    tier: Option<jjrg_Tier>,
    effort: Option<jjrg_Effort>,
) -> jjrg_Gallops {
    let mut g = canonical_gallops();
    let tack = &mut g.heats.get_mut("₣AC").unwrap().paces.get_mut("₢ACAAA").unwrap().tacks[0];
    tack.state = state;
    tack.tier = tier;
    tack.effort = effort;
    g
}

#[test]
fn jjtg_bridled_state_and_designation_wire_tokens() {
    // The re-minted state token, and the jjgde_ designation family for both
    // vocabularies — wire token per variant (RCG Constant Discipline).
    assert_eq!(serde_json::to_string(&jjrg_PaceState::Bridled).unwrap(), "\"jjgte_bridled\"");
    assert_eq!(serde_json::to_string(&jjrg_Tier::Haiku).unwrap(), "\"jjgde_haiku\"");
    assert_eq!(serde_json::to_string(&jjrg_Tier::Sonnet).unwrap(), "\"jjgde_sonnet\"");
    assert_eq!(serde_json::to_string(&jjrg_Tier::Opus).unwrap(), "\"jjgde_opus\"");
    assert_eq!(serde_json::to_string(&jjrg_Tier::Fable).unwrap(), "\"jjgde_fable\"");
    assert_eq!(serde_json::to_string(&jjrg_Effort::Low).unwrap(), "\"jjgde_low\"");
    assert_eq!(serde_json::to_string(&jjrg_Effort::Medium).unwrap(), "\"jjgde_medium\"");
    assert_eq!(serde_json::to_string(&jjrg_Effort::High).unwrap(), "\"jjgde_high\"");
    assert_eq!(serde_json::to_string(&jjrg_Effort::Xhigh).unwrap(), "\"jjgde_xhigh\"");
    assert_eq!(serde_json::to_string(&jjrg_Effort::Max).unwrap(), "\"jjgde_max\"");
}

#[test]
fn jjtg_designation_word_parsing_is_recognized_word_only() {
    assert_eq!(jjrg_Tier::jjrg_from_word("sonnet").unwrap(), jjrg_Tier::Sonnet);
    assert_eq!(jjrg_Tier::jjrg_from_word("fable").unwrap(), jjrg_Tier::Fable);
    assert!(jjrg_Tier::jjrg_from_word("gpt-5.5").is_err());
    assert!(jjrg_Tier::jjrg_from_word("gemini").is_err());
    assert!(jjrg_Tier::jjrg_from_word("").is_err());
    assert_eq!(jjrg_Effort::jjrg_from_word("xhigh").unwrap(), jjrg_Effort::Xhigh);
    assert!(jjrg_Effort::jjrg_from_word("ultra").is_err());
}

#[test]
fn jjtg_untouched_store_carries_no_designation_keys() {
    // The additive carve-out: an undesignated store serializes byte-identically
    // to the pre-bridle schema — no jjgtn_tier or jjgtn_effort keys appear, so
    // the load round-trip gate never trips and no reprieve episode is needed.
    let bytes = serde_json::to_string_pretty(&canonical_gallops()).unwrap();
    assert!(!bytes.contains("jjgtn_tier"));
    assert!(!bytes.contains("jjgtn_effort"));
    assert!(matches!(zjjrvl_appraise(bytes.as_bytes()), zjjrvl_Appraisal::Canonical(_)));
}

#[test]
fn jjtg_bridled_store_round_trips_canonical() {
    let g = make_designated_gallops(
        jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), Some(jjrg_Effort::High));
    let bytes = serde_json::to_string_pretty(&g).unwrap();
    assert!(bytes.contains("jjgte_bridled"));
    assert!(bytes.contains("jjgde_sonnet"));
    assert!(bytes.contains("jjgde_high"));
    // Canonical means load + round-trip gate + semantic validation all pass.
    match zjjrvl_appraise(bytes.as_bytes()) {
        zjjrvl_Appraisal::Canonical(_) => {}
        other => panic!("bridled store must appraise Canonical, got {}", appraisal_name(&other)),
    }
    // And the designation deserializes back intact.
    let reloaded: jjrg_Gallops = serde_json::from_str(&bytes).unwrap();
    let tack = &reloaded.heats["₣AC"].paces["₢ACAAA"].tacks[0];
    assert_eq!(tack.state, jjrg_PaceState::Bridled);
    assert_eq!(tack.tier, Some(jjrg_Tier::Sonnet));
    assert_eq!(tack.effort, Some(jjrg_Effort::High));
}

#[test]
fn jjtg_effort_absent_designation_is_byte_identical_to_tier_only() {
    let with_none = make_designated_gallops(jjrg_PaceState::Bridled, Some(jjrg_Tier::Haiku), None);
    let bytes = serde_json::to_string_pretty(&with_none).unwrap();
    assert!(bytes.contains("jjgtn_tier"));
    assert!(!bytes.contains("jjgtn_effort"), "effort-absent designation must omit the effort key");
    assert!(matches!(zjjrvl_appraise(bytes.as_bytes()), zjjrvl_Appraisal::Canonical(_)));
}

#[test]
fn jjtg_bridle_designates_rough_pace() {
    let mut g = canonical_gallops();
    let ctx = g.jjrg_bridle("ACAAA", jjrg_Tier::Sonnet, Some(jjrg_Effort::Xhigh), "abc1234", "260707-1100").unwrap();
    assert_eq!(ctx.state, jjrg_PaceState::Rough, "context snapshots the pre-mutation state");
    let tack = &g.heats["₣AC"].paces["₢ACAAA"].tacks[0];
    assert_eq!(tack.state, jjrg_PaceState::Bridled);
    assert_eq!(tack.tier, Some(jjrg_Tier::Sonnet));
    assert_eq!(tack.effort, Some(jjrg_Effort::Xhigh));
    assert_eq!(tack.basis, "abc1234");
    assert_eq!(tack.ts, "260707-1100");
}

#[test]
fn jjtg_bridle_refuses_non_rough_pace() {
    // Only a rough pace may be bridled — the rough filter is the precondition.
    for state in [jjrg_PaceState::Bridled, jjrg_PaceState::Complete, jjrg_PaceState::Abandoned] {
        let tier = if state == jjrg_PaceState::Bridled { Some(jjrg_Tier::Haiku) } else { None };
        let mut g = make_designated_gallops(state.clone(), tier, None);
        let err = g.jjrg_bridle("ACAAA", jjrg_Tier::Opus, None, "abc1234", "260707-1100").unwrap_err();
        assert!(err.contains("only a rough pace may be bridled"), "state {:?}: {}", state, err);
    }
}

#[test]
fn jjtg_release_unbridles_and_wipes_designation() {
    let mut g = make_designated_gallops(
        jjrg_PaceState::Bridled, Some(jjrg_Tier::Haiku), Some(jjrg_Effort::Low));
    g.jjrg_release("ACAAA", "abc1234", "260707-1101").unwrap();
    let tack = &g.heats["₣AC"].paces["₢ACAAA"].tacks[0];
    assert_eq!(tack.state, jjrg_PaceState::Rough);
    assert_eq!(tack.tier, None);
    assert_eq!(tack.effort, None);
}

#[test]
fn jjtg_release_refuses_non_bridled_pace() {
    let mut g = canonical_gallops();
    let err = g.jjrg_release("ACAAA", "abc1234", "260707-1101").unwrap_err();
    assert!(err.contains("only a bridled pace may be released"), "got: {}", err);
}

#[test]
fn jjtg_redocket_reverts_bridled_to_rough() {
    // Revert trigger: the docket is a judgment input; editing it voids the designation.
    let mut g = make_designated_gallops(
        jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), Some(jjrg_Effort::High));
    g.jjrg_revise_docket("ACAAA", "revised docket", "abc1234", "260707-1102").unwrap();
    let tack = &g.heats["₣AC"].paces["₢ACAAA"].tacks[0];
    assert_eq!(tack.state, jjrg_PaceState::Rough);
    assert_eq!(tack.tier, None);
    assert_eq!(tack.effort, None);
}

#[test]
fn jjtg_redocket_preserves_resolved_provenance() {
    // A complete pace's close-time designation provenance survives a docket edit.
    let mut g = make_designated_gallops(
        jjrg_PaceState::Complete, Some(jjrg_Tier::Haiku), Some(jjrg_Effort::Max));
    g.jjrg_revise_docket("ACAAA", "revised docket", "abc1234", "260707-1102").unwrap();
    let tack = &g.heats["₣AC"].paces["₢ACAAA"].tacks[0];
    assert_eq!(tack.state, jjrg_PaceState::Complete);
    assert_eq!(tack.tier, Some(jjrg_Tier::Haiku));
    assert_eq!(tack.effort, Some(jjrg_Effort::Max));
}

/// Two-heat gallops for draft/relocate revert tests.
fn make_two_heat_gallops(state: jjrg_PaceState, tier: Option<jjrg_Tier>, effort: Option<jjrg_Effort>) -> jjrg_Gallops {
    let mut g = make_designated_gallops(state, tier, effort);
    let (hk, heat) = make_valid_heat("AD", "dest-heat");
    g.heat_order.push(hk.clone());
    g.heats.insert(hk, heat);
    g
}

#[test]
fn jjtg_draft_reverts_bridled_to_rough() {
    // Revert trigger: relocate/transfer change the paddock context the pace was
    // judged against, so a bridled pace lands rough with its designation wiped.
    let mut g = make_two_heat_gallops(
        jjrg_PaceState::Bridled, Some(jjrg_Tier::Opus), Some(jjrg_Effort::Medium));
    let result = g.jjrg_draft(jjrg_DraftArgs {
        coronet: "ACAAA".to_string(),
        to: "AD".to_string(),
        before: None,
        after: None,
        first: false,
    }).unwrap();
    let tack = &g.heats["₣AD"].paces[&result.new_coronet].tacks[0];
    assert_eq!(tack.state, jjrg_PaceState::Rough);
    assert_eq!(tack.tier, None);
    assert_eq!(tack.effort, None);
}

#[test]
fn jjtg_draft_carries_resolved_provenance() {
    let mut g = make_two_heat_gallops(
        jjrg_PaceState::Complete, Some(jjrg_Tier::Sonnet), None);
    let result = g.jjrg_draft(jjrg_DraftArgs {
        coronet: "ACAAA".to_string(),
        to: "AD".to_string(),
        before: None,
        after: None,
        first: false,
    }).unwrap();
    let tack = &g.heats["₣AD"].paces[&result.new_coronet].tacks[0];
    assert_eq!(tack.state, jjrg_PaceState::Complete);
    assert_eq!(tack.tier, Some(jjrg_Tier::Sonnet));
}

#[test]
fn jjtg_relabel_reverts_nothing() {
    // Relabel rides tally with a silks-only change: designation carries through.
    let mut g = make_designated_gallops(
        jjrg_PaceState::Bridled, Some(jjrg_Tier::Haiku), Some(jjrg_Effort::Low));
    g.jjrg_tally(jjrg_TallyArgs {
        coronet: "ACAAA".to_string(),
        state: None,
        text: None,
        silks: Some("renamed-pace".to_string()),
    }).unwrap();
    let tack = &g.heats["₣AC"].paces["₢ACAAA"].tacks[0];
    assert_eq!(tack.state, jjrg_PaceState::Bridled);
    assert_eq!(tack.tier, Some(jjrg_Tier::Haiku));
    assert_eq!(tack.effort, Some(jjrg_Effort::Low));
    assert_eq!(tack.silks, "renamed-pace");
}

#[test]
fn jjtg_close_and_drop_persist_designation_as_provenance() {
    // Wrap (→ complete) and drop (→ abandoned) both ride tally: tier and effort persist.
    for terminal in [jjrg_PaceState::Complete, jjrg_PaceState::Abandoned] {
        let mut g = make_designated_gallops(
            jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), Some(jjrg_Effort::High));
        g.jjrg_tally(jjrg_TallyArgs {
            coronet: "ACAAA".to_string(),
            state: Some(terminal.clone()),
            text: None,
            silks: None,
        }).unwrap();
        let tack = &g.heats["₣AC"].paces["₢ACAAA"].tacks[0];
        assert_eq!(tack.state, terminal);
        assert_eq!(tack.tier, Some(jjrg_Tier::Sonnet));
        assert_eq!(tack.effort, Some(jjrg_Effort::High));
    }
}

#[test]
fn jjtg_validate_rejects_incoherent_designation() {
    // Bridled without a tier: the designation IS the tier record.
    let g = make_designated_gallops(jjrg_PaceState::Bridled, None, None);
    assert!(jjrg_validate(&g).unwrap_err().iter().any(|e| e.contains("bridled tack must carry a tier")));

    // Rough with a designation: every wipe path must have fired.
    let g = make_designated_gallops(jjrg_PaceState::Rough, Some(jjrg_Tier::Haiku), None);
    assert!(jjrg_validate(&g).unwrap_err().iter().any(|e| e.contains("rough tack must carry no tier")));

    // Effort never rides without a tier.
    let g = make_designated_gallops(jjrg_PaceState::Complete, None, Some(jjrg_Effort::Low));
    assert!(jjrg_validate(&g).unwrap_err().iter().any(|e| e.contains("effort must not ride without a tier")));
}

