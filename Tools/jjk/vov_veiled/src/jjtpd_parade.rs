// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_parade command (heat/pace display)
//!
//! Module-private helpers (`zjjrpd_*`) are deliberately NOT reached from this
//! sibling test module — `z` means private, never widened for test access (RCG).
//! Their behavior is covered through the public `jjrpd_run_parade` boundary
//! below: length dispatch, the empty-targets error, the always-on gazette, and
//! the remaining filter all exercise the private helpers as a side effect.

use super::jjrpd_parade::{jjrpd_run_parade, jjrpd_ParadeArgs};
use super::jjrg_gallops::{jjrg_Gallops, jjrg_Heat, jjrg_Pace, jjrg_Tack, jjrg_HeatStatus, jjrg_PaceState, JJRG_UNKNOWN_BASIS};
use super::jjrz_gazette::{jjrz_Gazette, jjrz_Slug, jjrz_parse_reslate_input};
use std::collections::BTreeMap;

// ===== Helper functions =====

fn make_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
    }
}

/// Build a heat with one pace carrying a specific docket and pace state.
fn make_heat_with_docket(
    heat_id: &str,
    status: jjrg_HeatStatus,
    state: jjrg_PaceState,
    docket: &str,
) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let pace_key = format!("₢{}AAA", heat_id);
    let tack = jjrg_Tack {
        ts: "260101-1200".to_string(),
        state,
        text: docket.lines().map(|l| l.to_string()).collect(),
        silks: format!("pace-{}", heat_id),
        basis: JJRG_UNKNOWN_BASIS.to_string(),
    };
    let mut paces = BTreeMap::new();
    paces.insert(pace_key.clone(), jjrg_Pace { tacks: vec![tack] });
    let heat = jjrg_Heat {
        silks: format!("heat-{}", heat_id),
        creation_time: "260101".to_string(),
        status,
        order: vec![pace_key],
        next_pace_seed: "AAB".to_string(),
        paces,
    };
    (heat_key, heat)
}

/// Persist a gallops to a per-test temp directory so `jjrpd_run_parade`
/// (which loads from disk) can be driven at its public boundary. Each test
/// gets its OWN directory: jjrg_save writes a `.tmp.<pid>.json` alongside the
/// target, and since parallel tests share one pid, a shared directory would
/// race that temp name across saves. The unique `name` isolates each test.
fn write_temp_gallops(name: &str, gallops: &jjrg_Gallops) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("jjtpd_{}_{}", name, std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("gallops.json");
    gallops.jjrg_save(&path).unwrap();
    path
}

/// Read the current (latest tack) docket lines for one pace.
fn current_docket_lines(gallops: &jjrg_Gallops, heat_key: &str, coronet_key: &str) -> Vec<String> {
    gallops.heats.get(heat_key).expect("heat present")
        .paces.get(coronet_key).expect("pace present")
        .tacks.first().expect("tack present")
        .text.clone()
}

/// Model the operator's bash bridge over a shown gazette: drop the paddock
/// notices, rewrite the output-typed `jjezs_pace` slug to the input-typed
/// `jjezs_reslate`, and apply the transformable heading renames this round-trip
/// was born to carry (`## Locked` -> `## Cinched`, `## Done` -> `## Done when`).
fn bridge_show_to_reslate(shown: &str) -> String {
    let mut out = String::new();
    let mut in_paddock = false;
    for line in shown.lines() {
        if line.starts_with("# jjezs_paddock") {
            in_paddock = true;
            continue;
        }
        if let Some(rest) = line.strip_prefix("# jjezs_pace ") {
            in_paddock = false;
            out.push_str("# jjezs_reslate ");
            out.push_str(rest);
            out.push('\n');
            continue;
        }
        if line.starts_with("# jjezs_") {
            in_paddock = false;
        }
        if in_paddock {
            continue;
        }
        let rewritten = match line.trim_end() {
            "## Locked" => "## Cinched",
            "## Done" => "## Done when",
            _ => line,
        };
        out.push_str(rewritten);
        out.push('\n');
    }
    out
}

// ===== Target length validation (pure string contract) =====
// The length classifier that drives jjrpd_run_parade dispatch: a firemark is
// 2 chars, a coronet 5, both with an optional ₢/₣ glyph stripped first.

#[test]
fn jjtpd_target_length_firemark_valid() {
    let target = "AB";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert_eq!(target_str.len(), 2);
}

#[test]
fn jjtpd_target_length_firemark_with_prefix_valid() {
    let target = "₣AB";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert_eq!(target_str.len(), 2);
}

#[test]
fn jjtpd_target_length_coronet_valid() {
    let target = "ABAAA";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert_eq!(target_str.len(), 5);
}

#[test]
fn jjtpd_target_length_coronet_with_prefix_valid() {
    let target = "₢ABAAA";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert_eq!(target_str.len(), 5);
}

#[test]
fn jjtpd_target_length_invalid_three_chars() {
    let target = "ABC";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert!(target_str.len() != 2 && target_str.len() != 5);
}

// ===== Public-boundary dispatch tests (jjrpd_run_parade) =====

#[test]
fn jjtpd_empty_targets_errors() {
    // The auto-select is gone: target selection now arrives solely via the
    // gazette halter notice the MCP arm parses, so an empty target list is a
    // caller bug (the zero-write path the move was made to close) and errors
    // rather than silently picking a racing heat. A live racing heat is present
    // precisely to prove it is NOT auto-selected.
    let mut gallops = make_valid_gallops();
    let (k, h) = make_heat_with_docket("AB", jjrg_HeatStatus::Racing, jjrg_PaceState::Rough, "## Goal\nalpha");
    gallops.heats.insert(k, h);
    let path = write_temp_gallops("empty_targets", &gallops);

    let mut gz = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
    let (code, out) = jjrpd_run_parade(
        jjrpd_ParadeArgs { file: path.clone(), targets: vec![], remaining: false },
        &mut gz,
    );
    assert_eq!(code, 1);
    assert!(out.contains("no target specified"), "got: {}", out);
    assert!(!gz.jjrz_emit().contains("₢ABAAA"), "no racing heat may be auto-selected on empty targets");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn jjtpd_heterogeneous_list_populates_gazette_for_each_target() {
    // Firemark target expands; coronet target returns its single pace. Both land
    // in one gazette while the tool-result stays terse (bodies only in gazette).
    let mut gallops = make_valid_gallops();
    let (k_ab, h_ab) = make_heat_with_docket("AB", jjrg_HeatStatus::Racing, jjrg_PaceState::Rough, "## Goal\nalpha");
    let (k_cd, h_cd) = make_heat_with_docket("CD", jjrg_HeatStatus::Racing, jjrg_PaceState::Rough, "## Goal\nbeta");
    gallops.heats.insert(k_ab, h_ab);
    gallops.heats.insert(k_cd, h_cd);
    let path = write_temp_gallops("heterogeneous", &gallops);

    let mut gz = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
    let (code, out) = jjrpd_run_parade(
        jjrpd_ParadeArgs {
            file: path.clone(),
            targets: vec!["₣AB".to_string(), "₢CDAAA".to_string()],
            remaining: false,
        },
        &mut gz,
    );
    assert_eq!(code, 0);
    let emitted = gz.jjrz_emit();
    assert!(emitted.contains("₢ABAAA"), "firemark expansion missing from gazette: {}", emitted);
    assert!(emitted.contains("alpha"));
    assert!(emitted.contains("₢CDAAA"), "coronet target missing from gazette: {}", emitted);
    assert!(emitted.contains("beta"));
    // Terse result carries the coronet header + state string (covers pace-state-str).
    assert!(out.contains("[rough]"), "terse result should render pace state: {}", out);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn jjtpd_bad_target_length_errors() {
    let mut gallops = make_valid_gallops();
    let (k, h) = make_heat_with_docket("AB", jjrg_HeatStatus::Racing, jjrg_PaceState::Rough, "## Goal\nx");
    gallops.heats.insert(k, h);
    let path = write_temp_gallops("bad_length", &gallops);

    let mut gz = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
    let (code, out) = jjrpd_run_parade(
        jjrpd_ParadeArgs { file: path.clone(), targets: vec!["ABC".to_string()], remaining: false },
        &mut gz,
    );
    assert_eq!(code, 1);
    assert!(out.contains("must be Firemark"), "got: {}", out);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn jjtpd_remaining_filters_firemark_but_coronet_returns_regardless() {
    // remaining affects firemark expansion only; a directly-named coronet returns
    // whatever its state.
    let mut gallops = make_valid_gallops();
    let (k, h) = make_heat_with_docket("AB", jjrg_HeatStatus::Racing, jjrg_PaceState::Complete, "## Goal\ndone-work");
    gallops.heats.insert(k, h);
    let path = write_temp_gallops("remaining", &gallops);

    // Firemark + remaining: the complete pace is excluded from the gazette.
    let mut gz_heat = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
    let (c1, _) = jjrpd_run_parade(
        jjrpd_ParadeArgs { file: path.clone(), targets: vec!["₣AB".to_string()], remaining: true },
        &mut gz_heat,
    );
    assert_eq!(c1, 0);
    assert!(!gz_heat.jjrz_emit().contains("₢ABAAA"), "remaining should exclude the complete pace from firemark expansion");

    // Coronet target: returned regardless of remaining.
    let mut gz_pace = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
    let (c2, _) = jjrpd_run_parade(
        jjrpd_ParadeArgs { file: path.clone(), targets: vec!["₢ABAAA".to_string()], remaining: true },
        &mut gz_pace,
    );
    assert_eq!(c2, 0);
    assert!(gz_pace.jjrz_emit().contains("₢ABAAA"), "a directly-named coronet must return regardless of state");
    let _ = std::fs::remove_file(&path);
}

// ===== End-to-end round-trip scenario (the pace's own origin) =====

#[test]
fn jjtpd_round_trip_show_to_reslate_migrates_headings_per_docket() {
    // Cross-heat, transformable heading renames driven end-to-end: show ->
    // bash bridge -> mass reslate, asserting faithful per-docket change. The
    // fixture constructs its own dockets — no dependence on any live docket.
    let docket_ab = "## Goal\nbuild the thing\n\n## Locked\nno new verb\n\n## Done\ncriterion A";
    let docket_cd = "## Locked\nsingle home\n\n## Done\ncriterion C";

    let mut gallops = make_valid_gallops();
    let (k_ab, h_ab) = make_heat_with_docket("AB", jjrg_HeatStatus::Racing, jjrg_PaceState::Rough, docket_ab);
    let (k_cd, h_cd) = make_heat_with_docket("CD", jjrg_HeatStatus::Racing, jjrg_PaceState::Rough, docket_cd);
    gallops.heats.insert(k_ab, h_ab);
    gallops.heats.insert(k_cd, h_cd);

    // 1. SHOW emits paddock(s) + pace dockets into the gazette (modeled via the
    //    same gazette API jjrpd_run_parade uses to populate it).
    let mut shown = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
    shown.jjrz_add(jjrz_Slug::Paddock, "AB", "paddock-ab body").unwrap();
    shown.jjrz_add(jjrz_Slug::Paddock, "CD", "paddock-cd body").unwrap();
    shown.jjrz_add(jjrz_Slug::Pace, "₢ABAAA", docket_ab).unwrap();
    shown.jjrz_add(jjrz_Slug::Pace, "₢CDAAA", docket_cd).unwrap();
    let shown_md = shown.jjrz_emit();

    // 2. The deliberate bash bridge: drop paddocks, rewrite slug, rename headings.
    let bridged = bridge_show_to_reslate(&shown_md);
    assert!(bridged.contains("# jjezs_reslate ₢ABAAA"));
    assert!(bridged.contains("# jjezs_reslate ₢CDAAA"));
    assert!(!bridged.contains("jjezs_paddock"), "paddocks must be dropped by the bridge");
    assert!(!bridged.contains("jjezs_pace"), "output-typed slug must be rewritten");

    // 3. RESLATE parses the bridged gazette into per-coronet dockets.
    let pairs = jjrz_parse_reslate_input(&bridged).expect("bridged gazette parses as reslate input");
    assert_eq!(pairs.len(), 2);

    // 4. APPLY each docket back to the gallops (pure revise path).
    for (coronet, docket) in &pairs {
        gallops
            .jjrg_revise_docket(coronet, docket, "0000000", "260101-1200")
            .unwrap_or_else(|e| panic!("revise {} failed: {}", coronet, e));
    }

    // 5. Each docket migrated faithfully and independently.
    let ab = current_docket_lines(&gallops, "₣AB", "₢ABAAA");
    assert!(ab.iter().any(|l| l == "## Cinched"), "AB: Locked -> Cinched");
    assert!(ab.iter().any(|l| l == "## Done when"), "AB: Done -> Done when");
    assert!(!ab.iter().any(|l| l == "## Locked"));
    assert!(!ab.iter().any(|l| l == "## Done"));
    assert!(ab.iter().any(|l| l == "build the thing"), "AB body preserved");

    let cd = current_docket_lines(&gallops, "₣CD", "₢CDAAA");
    assert!(cd.iter().any(|l| l == "## Cinched"), "CD: Locked -> Cinched");
    assert!(cd.iter().any(|l| l == "## Done when"), "CD: Done -> Done when");
    assert!(cd.iter().any(|l| l == "single home"), "CD body preserved");
}
