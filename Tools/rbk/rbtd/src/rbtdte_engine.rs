// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTDTE — tests for case execution engine

use std::path::Path;

use super::rbtcee_engine::*;
use super::rbtteh_helpers::rbtteh_make_scratch;

fn rbttee_pass(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Pass
}

fn rbttee_fail(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Fail("boom".to_string())
}

fn rbttee_skip(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Skip("nope".to_string())
}

const RBTTEE_COLORS: rbtcee_Colors = rbtcee_Colors {
    green: "",
    red: "",
    yellow: "",
    reset: "",
};

#[test]
fn rbttee_counts_all_verdict_types() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case { name: "p1", func: rbttee_pass },
        rbtcee_Case { name: "p2", func: rbttee_pass },
        rbtcee_Case { name: "s1", func: rbttee_skip },
        rbtcee_Case { name: "f1", func: rbttee_fail },
    ];

    let tmp = rbtteh_make_scratch("counts");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 2);
    assert_eq!(result.failed, 1);
    assert_eq!(result.skipped, 1);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_fail_fast_stops_after_first_failure() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case { name: "ff-f1", func: rbttee_fail },
        rbtcee_Case { name: "ff-p1", func: rbttee_pass },
    ];

    let tmp = rbtteh_make_scratch("failfast");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, true, &tmp).unwrap();
    assert_eq!(result.failed, 1);
    assert_eq!(result.passed, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_fail_fast_reports_unreached_cases() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case { name: "un-f1", func: rbttee_fail },
        rbtcee_Case { name: "un-p1", func: rbttee_pass },
        rbtcee_Case { name: "un-p2", func: rbttee_pass },
    ];

    let tmp = rbtteh_make_scratch("unreached");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, true, &tmp).unwrap();
    assert_eq!(result.registered, 3);
    assert_eq!(result.unreached, vec!["un-p1", "un-p2"]);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_full_reach_leaves_unreached_empty() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case { name: "full-p1", func: rbttee_pass },
        rbtcee_Case { name: "full-p2", func: rbttee_pass },
    ];

    let tmp = rbtteh_make_scratch("fullreach");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, true, &tmp).unwrap();
    assert_eq!(result.registered, 2);
    assert!(result.unreached.is_empty());
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_trace_files_written() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case {
            name: "traced-pass",
            func: rbttee_pass,
        },
        rbtcee_Case {
            name: "traced-fail",
            func: rbttee_fail,
        },
    ];

    let tmp = rbtteh_make_scratch("trace");
    let _ = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();

    let pass_trace =
        std::fs::read_to_string(tmp.join("traced-pass").join("trace.txt")).unwrap();
    assert!(pass_trace.contains("PASSED"));

    let fail_trace =
        std::fs::read_to_string(tmp.join("traced-fail").join("trace.txt")).unwrap();
    assert!(fail_trace.contains("FAILED"));
    assert!(fail_trace.contains("boom"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_cases_run_in_declaration_order() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case { name: "ord-a", func: rbttee_pass },
        rbtcee_Case { name: "ord-b", func: rbttee_pass },
        rbtcee_Case { name: "ord-c", func: rbttee_skip },
    ];

    let tmp = rbtteh_make_scratch("order");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 2);
    assert_eq!(result.skipped, 1);
    assert_eq!(result.failed, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Edge cases ───────────────────────────────────────────────

#[test]
fn rbttee_zero_cases() {
    static CASES: &[rbtcee_Case] = &[];
    let tmp = rbtteh_make_scratch("zerocases");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_all_skip() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case { name: "sk1", func: rbttee_skip },
        rbtcee_Case { name: "sk2", func: rbttee_skip },
        rbtcee_Case { name: "sk3", func: rbttee_skip },
    ];

    let tmp = rbtteh_make_scratch("allskip");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 3);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_single_case_pass() {
    static CASES: &[rbtcee_Case] = &[rbtcee_Case {
        name: "solo-pass",
        func: rbttee_pass,
    }];

    let tmp = rbtteh_make_scratch("solopass");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 1);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_single_case_fail() {
    static CASES: &[rbtcee_Case] = &[rbtcee_Case {
        name: "solo-fail",
        func: rbttee_fail,
    }];

    let tmp = rbtteh_make_scratch("solofail");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 1);
    assert_eq!(result.skipped, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Run-all despite failures ─────────────────────────────────

#[test]
fn rbttee_run_all_executes_every_case_despite_failures() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case { name: "ra-f1", func: rbttee_fail },
        rbtcee_Case { name: "ra-p1", func: rbttee_pass },
        rbtcee_Case { name: "ra-f2", func: rbttee_fail },
        rbtcee_Case { name: "ra-p2", func: rbttee_pass },
    ];

    let tmp = rbtteh_make_scratch("runall");
    let result = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 2);
    assert_eq!(result.failed, 2);
    // All four case dirs were created — every case ran
    assert!(tmp.join("ra-f1").exists());
    assert!(tmp.join("ra-p1").exists());
    assert!(tmp.join("ra-f2").exists());
    assert!(tmp.join("ra-p2").exists());
    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Temp dir isolation ───────────────────────────────────────

fn rbttee_write_marker(dir: &Path) -> rbtcee_Verdict {
    let _ = std::fs::write(dir.join("marker.txt"), dir.to_string_lossy().as_bytes());
    rbtcee_Verdict::Pass
}

#[test]
fn rbttee_temp_dirs_are_distinct_and_isolated() {
    static CASES: &[rbtcee_Case] = &[
        rbtcee_Case { name: "iso-a", func: rbttee_write_marker },
        rbtcee_Case { name: "iso-b", func: rbttee_write_marker },
    ];

    let tmp = rbtteh_make_scratch("isolation");
    let _ = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();

    let dir_a = tmp.join("iso-a");
    let dir_b = tmp.join("iso-b");

    // Dirs are distinct paths
    assert_ne!(dir_a, dir_b);

    // Each has its own marker
    let marker_a = std::fs::read_to_string(dir_a.join("marker.txt")).unwrap();
    let marker_b = std::fs::read_to_string(dir_b.join("marker.txt")).unwrap();
    assert!(marker_a.contains("iso-a"));
    assert!(marker_b.contains("iso-b"));

    // No cross-contamination: each dir has exactly 2 files (marker.txt + trace.txt)
    let entries_a: Vec<_> = std::fs::read_dir(&dir_a).unwrap().collect();
    let entries_b: Vec<_> = std::fs::read_dir(&dir_b).unwrap().collect();
    assert_eq!(entries_a.len(), 2);
    assert_eq!(entries_b.len(), 2);

    // Trace content references only its own case
    let trace_a = std::fs::read_to_string(dir_a.join("trace.txt")).unwrap();
    let trace_b = std::fs::read_to_string(dir_b.join("trace.txt")).unwrap();
    assert!(trace_a.contains("iso-a"));
    assert!(trace_b.contains("iso-b"));
    assert!(!trace_a.contains("iso-b"));
    assert!(!trace_b.contains("iso-a"));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Disposition policy gate ──────────────────────────────────

#[test]
fn rbttee_resolve_fail_fast_independent_default_is_fail_fast() {
    let r = rbtcee_resolve_fail_fast(rbtcee_Disposition::Independent, false).unwrap();
    assert!(r);
}

#[test]
fn rbttee_resolve_fail_fast_independent_keep_going_permitted() {
    let r = rbtcee_resolve_fail_fast(rbtcee_Disposition::Independent, true).unwrap();
    assert!(!r);
}

#[test]
fn rbttee_resolve_fail_fast_state_progressing_default_is_fail_fast() {
    let r = rbtcee_resolve_fail_fast(rbtcee_Disposition::StateProgressing, false).unwrap();
    assert!(r);
}

#[test]
fn rbttee_resolve_fail_fast_state_progressing_keep_going_refused() {
    let err = rbtcee_resolve_fail_fast(rbtcee_Disposition::StateProgressing, true).unwrap_err();
    assert!(err.contains("StateProgressing"));
    assert!(err.contains("keep-going"));
}

// ── Runner CLI arg parsing ───────────────────────────────────

fn rbttee_args(raw: &[&str]) -> Vec<String> {
    raw.iter().map(|s| s.to_string()).collect()
}

#[test]
fn rbttee_parse_keep_going_absent() {
    let (pos, kg) = rbtcee_parse_keep_going(&rbttee_args(&["tadmor"])).unwrap();
    assert_eq!(pos, vec!["tadmor".to_string()]);
    assert!(!kg);
}

#[test]
fn rbttee_parse_keep_going_trailing() {
    let (pos, kg) =
        rbtcee_parse_keep_going(&rbttee_args(&["tadmor", RBTCEE_FLAG_KEEP_GOING])).unwrap();
    assert_eq!(pos, vec!["tadmor".to_string()]);
    assert!(kg);
}

#[test]
fn rbttee_parse_keep_going_position_independent() {
    let (pos, kg) =
        rbtcee_parse_keep_going(&rbttee_args(&[RBTCEE_FLAG_KEEP_GOING, "tadmor"])).unwrap();
    assert_eq!(pos, vec!["tadmor".to_string()]);
    assert!(kg);
}

#[test]
fn rbttee_parse_keep_going_unknown_flag_rejected() {
    let err = rbtcee_parse_keep_going(&rbttee_args(&["tadmor", "--keep-goign"])).unwrap_err();
    assert!(err.contains("--keep-goign"));
    assert!(err.contains(RBTCEE_FLAG_KEEP_GOING));
}

// ── Trace file content detail ────────────────────────────────

#[test]
fn rbttee_trace_file_skip_contains_reason() {
    static CASES: &[rbtcee_Case] = &[rbtcee_Case {
        name: "traced-skip",
        func: rbttee_skip,
    }];

    let tmp = rbtteh_make_scratch("skiptrace");
    let _ = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();

    let trace = std::fs::read_to_string(tmp.join("traced-skip").join("trace.txt")).unwrap();
    assert!(trace.contains("SKIPPED"));
    assert!(trace.contains("traced-skip"));
    assert!(trace.contains("nope"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_case_output_files_survive_in_trace_dir() {
    fn zrbttee_write_output(dir: &Path) -> rbtcee_Verdict {
        let _ = std::fs::write(dir.join("output.txt"), "custom output data\n");
        rbtcee_Verdict::Pass
    }

    static CASES: &[rbtcee_Case] = &[rbtcee_Case {
        name: "output-case",
        func: zrbttee_write_output,
    }];

    let tmp = rbtteh_make_scratch("caseoutput");
    let _ = rbtcee_run_cases(CASES, &RBTTEE_COLORS, false, &tmp).unwrap();

    let output = std::fs::read_to_string(tmp.join("output-case").join("output.txt")).unwrap();
    assert!(output.contains("custom output data"));

    // Trace also exists alongside the case output
    let trace = std::fs::read_to_string(tmp.join("output-case").join("trace.txt")).unwrap();
    assert!(trace.contains("PASSED"));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Config-evolution console ─────────────────────────────────

fn rbttee_git(args: &[&str], root: &Path) -> std::process::Output {
    std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("git invocation")
}

#[test]
fn rbttee_config_set_field_replaces_value_preserving_other_lines() {
    let tmp = rbtteh_make_scratch("setfield");
    let file = tmp.join(crate::rbtdgc_consts::RBTDGC_RBRV_FILE);
    std::fs::write(&file, "KEEP_BEFORE=1\nRBRV_ANCHOR=old\nKEEP_AFTER=2\n").unwrap();

    rbtcee_config_set_field(&file, "RBRV_ANCHOR", "new").unwrap();

    let body = std::fs::read_to_string(&file).unwrap();
    assert!(body.contains("RBRV_ANCHOR=new"));
    assert!(body.contains("KEEP_BEFORE=1"));
    assert!(body.contains("KEEP_AFTER=2"));
    assert!(!body.contains("RBRV_ANCHOR=old"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_config_zero_blanks_named_field_only() {
    let tmp = rbtteh_make_scratch("zerofield");
    let file = tmp.join("rbrd.env");
    std::fs::write(&file, "RBRD_DEPOT_MONIKER=canest3-000007\nRBRD_CLOUD_PREFIX=canc\n").unwrap();

    rbtcee_config_zero(&file, "RBRD_DEPOT_MONIKER").unwrap();

    let body = std::fs::read_to_string(&file).unwrap();
    assert!(body.contains("RBRD_DEPOT_MONIKER=\n"));
    // The sibling field is untouched — zeroing is field-scoped.
    assert!(body.contains("RBRD_CLOUD_PREFIX=canc"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_config_zero_absent_field_fails_loud() {
    let tmp = rbtteh_make_scratch("zeroabsent");
    let file = tmp.join("rbrd.env");
    std::fs::write(&file, "RBRD_CLOUD_PREFIX=canc\n").unwrap();

    // The schema-drift catch: zeroing a field that is not present errs rather
    // than silently no-op'ing (which would mask a renamed or removed field).
    let err = rbtcee_config_zero(&file, "RBRD_RENAMED_AWAY").unwrap_err();
    assert!(err.contains("RBRD_RENAMED_AWAY"));
    assert!(err.contains("not found"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_commit_nameplates_scopes_to_named_class_only() {
    let tmp = rbtteh_make_scratch("commit-scope");
    assert!(rbttee_git(&["init", "-q"], &tmp).status.success());
    rbttee_git(&["config", "user.email", "theurge@test"], &tmp);
    rbttee_git(&["config", "user.name", "theurge test"], &tmp);

    // Baseline: a nameplate rbrn.env plus an unrelated tracked file, committed clean.
    // The directory is the INSCRIPTION (sprue + epithet) while the moniker
    // handed to the verb below stays bare — so this asserts the composition,
    // not merely that some path was found.
    let np_dir = tmp
        .join(crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR)
        .join(format!("{}testnp", crate::rbtdgc_consts::RBTDGC_NAMEPLATE_SPRUE));
    std::fs::create_dir_all(&np_dir).unwrap();
    let rbrn = np_dir.join(crate::rbtdgc_consts::RBTDGC_RBRN_FILE);
    std::fs::write(&rbrn, "RBRN_SENTRY_HALLMARK=\n").unwrap();
    let surprise = tmp.join("surprise.txt");
    std::fs::write(&surprise, "baseline\n").unwrap();
    rbttee_git(&["add", "-A"], &tmp);
    assert!(rbttee_git(&["commit", "-q", "-m", "baseline"], &tmp).status.success());

    // Dirty BOTH the owned nameplate file and an unrelated file — the exact
    // wrap-sweeps-everything hazard the scoped verb exists to prevent.
    std::fs::write(&rbrn, "RBRN_SENTRY_HALLMARK=kabc123\n").unwrap();
    std::fs::write(&surprise, "SURPRISE EDIT — must not be swept\n").unwrap();

    rbtcee_commit_nameplates(&tmp, &["testnp"], "test: nameplate hallmark").unwrap();

    let status = rbttee_git(&["status", "--porcelain"], &tmp);
    let out = String::from_utf8_lossy(&status.stdout);
    // The nameplate file is committed (no longer dirty)...
    assert!(
        !out.contains("rbrn.env"),
        "nameplate file must be committed; status: {:?}",
        out
    );
    // ...and the surprise edit survives uncommitted — never swept into the commit.
    assert!(
        out.contains("surprise.txt"),
        "surprise edit must survive uncommitted; status: {:?}",
        out
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Tariff evaluation seam ─────────────────────────────────────
//
// The pure seam is the tariff feature's testable heart: given a declared tariff
// and an observed (elapsed, invocations), it must flag exactly the violated
// bounds and nothing else. No spawning, no timing — deliberate violation of each
// kind asserts its flag, and every unchecked bound stays silent.

#[test]
fn rbttee_tariff_unchecked_never_flags() {
    // RBTCEE_UNCHECKED (all bounds None) must report all-clear no matter how extreme
    // the observation — this is what "undeclared fixtures run exactly as before"
    // rests on.
    let r = rbtcee_evaluate_tariff(&rbtcee_Tariff::RBTCEE_UNCHECKED, 0, 0);
    assert!(!r.too_fast && !r.too_slow && !r.count_drift);
    let r = rbtcee_evaluate_tariff(&rbtcee_Tariff::RBTCEE_UNCHECKED, 99_999, 9_999);
    assert!(!r.too_fast && !r.too_slow && !r.count_drift);
}

#[test]
fn rbttee_tariff_min_is_a_strict_floor() {
    let t = rbtcee_Tariff { min_secs: Some(10), max_secs: None, invocations: None };
    // Strictly below → too_fast (the vacuity catch).
    assert!(rbtcee_evaluate_tariff(&t, 9, 0).too_fast);
    // Exactly at, and above → clear (a boundary green is not vacuous).
    assert!(!rbtcee_evaluate_tariff(&t, 10, 0).too_fast);
    assert!(!rbtcee_evaluate_tariff(&t, 11, 0).too_fast);
    // The floor never trips the drift warnings.
    let r = rbtcee_evaluate_tariff(&t, 9, 0);
    assert!(!r.too_slow && !r.count_drift);
}

#[test]
fn rbttee_tariff_max_is_a_strict_ceiling_warning() {
    let t = rbtcee_Tariff { min_secs: None, max_secs: Some(60), invocations: None };
    assert!(rbtcee_evaluate_tariff(&t, 61, 0).too_slow);
    assert!(!rbtcee_evaluate_tariff(&t, 60, 0).too_slow);
    assert!(!rbtcee_evaluate_tariff(&t, 59, 0).too_slow);
    // A too-slow observation is never also a failure.
    assert!(!rbtcee_evaluate_tariff(&t, 61, 0).too_fast);
}

#[test]
fn rbttee_tariff_count_drift_is_exact_mismatch() {
    let t = rbtcee_Tariff { min_secs: None, max_secs: None, invocations: Some(23) };
    assert!(rbtcee_evaluate_tariff(&t, 0, 22).count_drift);
    assert!(rbtcee_evaluate_tariff(&t, 0, 24).count_drift);
    assert!(!rbtcee_evaluate_tariff(&t, 0, 23).count_drift);
}

#[test]
fn rbttee_tariff_bounds_are_independent() {
    // All three declared: a clean observation flags nothing; a mixed violation
    // (too-fast AND count-drift) flags exactly those two, leaving too-slow off.
    let t = rbtcee_Tariff { min_secs: Some(5), max_secs: Some(50), invocations: Some(3) };
    let clean = rbtcee_evaluate_tariff(&t, 20, 3);
    assert!(!clean.too_fast && !clean.too_slow && !clean.count_drift);
    let mixed = rbtcee_evaluate_tariff(&t, 2, 4);
    assert!(mixed.too_fast && mixed.count_drift && !mixed.too_slow);
    // The report echoes the observation for the declared-vs-observed print.
    assert_eq!(mixed.elapsed_secs, 2);
    assert_eq!(mixed.invocations, 4);
}

// ── Colophon census wired into rbtcee_run_fixture ────────────────
//
// Mirrors the real Context::new -> rbtccc_set_context -> rbtcee_run_fixture
// -> rbtccc_take_context sequence main.rs drives, so the negative check
// (declared-but-unused fails a fully-green fixture), its skip suppression,
// and the single-case exemption (rbtcee_run_single_case never consults it)
// are proven against the real wiring, not a reimplementation of it. The case
// reaches ctx through rbtccc_with_ctx — the same channel a real fixture's
// case uses.
//
// Arm order: rbtccc_set_context derives the census from the MANIFEST, and
// these synthetic fixture names have no manifest entry — so set_context arms
// None, and each test overrides with its synthetic declared set AFTER
// set_context. rbtccc_take_context disarms, leaving the worker thread clean.

const ZRBTTEE_CENSUS_COL_USED: &str = "zrbtdte-col-used";
const ZRBTTEE_CENSUS_COL_UNUSED: &str = "zrbtdte-col-unused";

fn zrbttee_census_invoke_used(_dir: &Path) -> rbtcee_Verdict {
    crate::rbtccc_crucible::rbtccc_with_ctx(|ctx| {
        match crate::rbtcei_invocation::rbtcei_invoke_global(ctx, ZRBTTEE_CENSUS_COL_USED, &[], &[]) {
            Ok(r) if r.exit_code == 0 => rbtcee_Verdict::Pass,
            Ok(r) => rbtcee_Verdict::Fail(format!("exit {}", r.exit_code)),
            Err(e) => rbtcee_Verdict::Fail(e),
        }
    })
}

static ZRBTTEE_CENSUS_CASES: &[rbtcee_Case] = &[crate::rbtcee_case!(zrbttee_census_invoke_used)];

static ZRBTTEE_CENSUS_FIXTURE: rbtcee_Fixture = rbtcee_Fixture {
    name: "zrbtdte-census-fixture",
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: ZRBTTEE_CENSUS_CASES,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};

/// Scratch project root with a tt/ script satisfying ZRBTTEE_CENSUS_COL_USED
/// via the global-tabtarget shape (`{colophon}.<frontispiece>.sh`).
fn zrbttee_census_scratch(label: &str) -> std::path::PathBuf {
    let tmp = rbtteh_make_scratch(label);
    let tt = tmp.join("tt");
    std::fs::create_dir_all(&tt).unwrap();
    let script = tt.join(format!("{}.Foo.sh", ZRBTTEE_CENSUS_COL_USED));
    std::fs::write(&script, "#!/bin/bash\nexit 0\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    tmp
}

#[test]
fn rbttee_run_fixture_passes_when_declared_colophons_all_used() {
    let tmp = zrbttee_census_scratch("census-fixture-pass");
    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let ctx = crate::rbtcei_invocation::rbtcei_Context::new(
        &tmp, ZRBTTEE_CENSUS_FIXTURE.name, &burv_temp_root, &burv_output_root,
    );
    crate::rbtccc_crucible::rbtccc_set_context(ctx);
    crate::rbtcei_invocation::rbtcei_census_arm(Some(&[ZRBTTEE_CENSUS_COL_USED]), &[]);

    let result = rbtcee_run_fixture(&ZRBTTEE_CENSUS_FIXTURE, &RBTTEE_COLORS, &tmp, false).unwrap();

    let _ = crate::rbtccc_crucible::rbtccc_take_context();

    assert_eq!(result.failed, 0, "every declared colophon was invoked — must not fail");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_run_fixture_fails_on_declared_but_unused_colophon() {
    let tmp = zrbttee_census_scratch("census-fixture-fail");
    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let ctx = crate::rbtcei_invocation::rbtcei_Context::new(
        &tmp, ZRBTTEE_CENSUS_FIXTURE.name, &burv_temp_root, &burv_output_root,
    );
    crate::rbtccc_crucible::rbtccc_set_context(ctx);
    // Declares a SECOND colophon the case never invokes.
    crate::rbtcei_invocation::rbtcei_census_arm(Some(&[
        ZRBTTEE_CENSUS_COL_USED,
        ZRBTTEE_CENSUS_COL_UNUSED,
    ]), &[]);

    let result = rbtcee_run_fixture(&ZRBTTEE_CENSUS_FIXTURE, &RBTTEE_COLORS, &tmp, false).unwrap();

    let _ = crate::rbtccc_crucible::rbtccc_take_context();

    assert!(result.failed > 0, "a declared-but-never-invoked colophon must fail the fixture");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_run_single_case_ignores_unused_declared_colophons() {
    // Single-case runs enforce positive only: rbtcee_run_single_case never
    // calls rbtcee_check_census, so the same armed declared-but-unused
    // colophon that fails a full fixture run above must NOT fail a
    // single-case run of the identical case.
    let tmp = zrbttee_census_scratch("census-single-case-exempt");
    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let ctx = crate::rbtcei_invocation::rbtcei_Context::new(
        &tmp, ZRBTTEE_CENSUS_FIXTURE.name, &burv_temp_root, &burv_output_root,
    );
    crate::rbtccc_crucible::rbtccc_set_context(ctx);
    crate::rbtcei_invocation::rbtcei_census_arm(Some(&[
        ZRBTTEE_CENSUS_COL_USED,
        ZRBTTEE_CENSUS_COL_UNUSED,
    ]), &[]);

    let result = rbtcee_run_single_case(&ZRBTTEE_CENSUS_CASES[0], &RBTTEE_COLORS, &tmp).unwrap();

    let _ = crate::rbtccc_crucible::rbtccc_take_context();

    assert_eq!(result.failed, 0, "single-case run must not enforce the negative census direction");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttee_run_fixture_permitted_colophon_invocation_not_refused() {
    // Positive-only tier, proven through the real set_context/run_fixture
    // wiring: the required set is deliberately empty (so an unmediated
    // invoke would refuse, exactly as it does when a fixture declares zero
    // required colophons), but the invoked colophon IS permitted — so the
    // invoke must succeed and the fixture must pass.
    let tmp = zrbttee_census_scratch("census-fixture-permitted-invoked");
    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let ctx = crate::rbtcei_invocation::rbtcei_Context::new(
        &tmp, ZRBTTEE_CENSUS_FIXTURE.name, &burv_temp_root, &burv_output_root,
    );
    crate::rbtccc_crucible::rbtccc_set_context(ctx);
    crate::rbtcei_invocation::rbtcei_census_arm(Some(&[]), &[ZRBTTEE_CENSUS_COL_USED]);

    let result = rbtcee_run_fixture(&ZRBTTEE_CENSUS_FIXTURE, &RBTTEE_COLORS, &tmp, false).unwrap();

    let _ = crate::rbtccc_crucible::rbtccc_take_context();

    assert_eq!(result.passed, 1, "a permitted colophon's invoke must not be refused");
    assert_eq!(result.failed, 0, "a permitted colophon's invoke must not be refused");

    let _ = std::fs::remove_dir_all(&tmp);
}

static ZRBTTEE_CENSUS_NOINVOKE_CASES: &[rbtcee_Case] = &[crate::rbtcee_case!(rbttee_pass)];

static ZRBTTEE_CENSUS_NOINVOKE_FIXTURE: rbtcee_Fixture = rbtcee_Fixture {
    name: "zrbtdte-census-noinvoke-fixture",
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: ZRBTTEE_CENSUS_NOINVOKE_CASES,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};

#[test]
fn rbttee_run_fixture_permitted_colophon_unused_does_not_fail() {
    // Negative check ignores the permitted tier entirely: a permitted
    // colophon declared but never invoked must not fail an otherwise-green
    // fixture, unlike the required-tier sibling above
    // (rbttee_run_fixture_fails_on_declared_but_unused_colophon) where the
    // identical non-invocation DOES fail the fixture.
    let tmp = rbtteh_make_scratch("census-fixture-permitted-unused");
    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let ctx = crate::rbtcei_invocation::rbtcei_Context::new(
        &tmp, ZRBTTEE_CENSUS_NOINVOKE_FIXTURE.name, &burv_temp_root, &burv_output_root,
    );
    crate::rbtccc_crucible::rbtccc_set_context(ctx);
    crate::rbtcei_invocation::rbtcei_census_arm(Some(&[]), &[ZRBTTEE_CENSUS_COL_UNUSED]);

    let result =
        rbtcee_run_fixture(&ZRBTTEE_CENSUS_NOINVOKE_FIXTURE, &RBTTEE_COLORS, &tmp, false).unwrap();

    let _ = crate::rbtccc_crucible::rbtccc_take_context();

    assert_eq!(result.passed, 1);
    assert_eq!(result.failed, 0, "an unused permitted colophon must not fail the fixture");

    let _ = std::fs::remove_dir_all(&tmp);
}

fn zrbttee_census_skip(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Skip("credential unavailable".to_string())
}

static ZRBTTEE_CENSUS_SKIP_CASES: &[rbtcee_Case] = &[crate::rbtcee_case!(zrbttee_census_skip)];

static ZRBTTEE_CENSUS_SKIP_FIXTURE: rbtcee_Fixture = rbtcee_Fixture {
    name: "zrbtdte-census-skip-fixture",
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: ZRBTTEE_CENSUS_SKIP_CASES,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};

#[test]
fn rbttee_run_fixture_skip_suppresses_negative_census() {
    // Suite-passenger protection: a self-skipping fixture's run is not
    // exhaustive, so a declared colophon its skipped cases never invoked must
    // not fail it — the same declared-but-unused set that fails a fully-run
    // fixture above is suppressed here by the skip.
    let tmp = rbtteh_make_scratch("census-skip-suppress");
    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let ctx = crate::rbtcei_invocation::rbtcei_Context::new(
        &tmp, ZRBTTEE_CENSUS_SKIP_FIXTURE.name, &burv_temp_root, &burv_output_root,
    );
    crate::rbtccc_crucible::rbtccc_set_context(ctx);
    crate::rbtcei_invocation::rbtcei_census_arm(Some(&[ZRBTTEE_CENSUS_COL_UNUSED]), &[]);

    let result =
        rbtcee_run_fixture(&ZRBTTEE_CENSUS_SKIP_FIXTURE, &RBTTEE_COLORS, &tmp, false).unwrap();

    let _ = crate::rbtccc_crucible::rbtccc_take_context();

    assert_eq!(result.skipped, 1);
    assert_eq!(result.failed, 0, "a skipped run must suppress the negative census check");

    let _ = std::fs::remove_dir_all(&tmp);
}
