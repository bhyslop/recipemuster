// Copyright 2026 Scale Invariant, Inc.
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

use super::rbtdre_engine::*;
use super::rbtdth_helpers::rbtdth_make_scratch;

fn rbtdte_pass(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Pass
}

fn rbtdte_fail(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Fail("boom".to_string())
}

fn rbtdte_skip(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Skip("nope".to_string())
}

const RBTDTE_COLORS: rbtdre_Colors = rbtdre_Colors {
    green: "",
    red: "",
    yellow: "",
    reset: "",
};

#[test]
fn rbtdte_counts_all_verdict_types() {
    static CASES: &[rbtdre_Case] = &[
        rbtdre_Case { name: "p1", func: rbtdte_pass },
        rbtdre_Case { name: "p2", func: rbtdte_pass },
        rbtdre_Case { name: "s1", func: rbtdte_skip },
        rbtdre_Case { name: "f1", func: rbtdte_fail },
    ];

    let tmp = rbtdth_make_scratch("counts");
    let result = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 2);
    assert_eq!(result.failed, 1);
    assert_eq!(result.skipped, 1);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_fail_fast_stops_after_first_failure() {
    static CASES: &[rbtdre_Case] = &[
        rbtdre_Case { name: "ff-f1", func: rbtdte_fail },
        rbtdre_Case { name: "ff-p1", func: rbtdte_pass },
    ];

    let tmp = rbtdth_make_scratch("failfast");
    let result = rbtdre_run_cases(CASES, &RBTDTE_COLORS, true, &tmp).unwrap();
    assert_eq!(result.failed, 1);
    assert_eq!(result.passed, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_trace_files_written() {
    static CASES: &[rbtdre_Case] = &[
        rbtdre_Case {
            name: "traced-pass",
            func: rbtdte_pass,
        },
        rbtdre_Case {
            name: "traced-fail",
            func: rbtdte_fail,
        },
    ];

    let tmp = rbtdth_make_scratch("trace");
    let _ = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();

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
fn rbtdte_cases_run_in_declaration_order() {
    static CASES: &[rbtdre_Case] = &[
        rbtdre_Case { name: "ord-a", func: rbtdte_pass },
        rbtdre_Case { name: "ord-b", func: rbtdte_pass },
        rbtdre_Case { name: "ord-c", func: rbtdte_skip },
    ];

    let tmp = rbtdth_make_scratch("order");
    let result = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 2);
    assert_eq!(result.skipped, 1);
    assert_eq!(result.failed, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Edge cases ───────────────────────────────────────────────

#[test]
fn rbtdte_zero_cases() {
    static CASES: &[rbtdre_Case] = &[];
    let tmp = rbtdth_make_scratch("zerocases");
    let result = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_all_skip() {
    static CASES: &[rbtdre_Case] = &[
        rbtdre_Case { name: "sk1", func: rbtdte_skip },
        rbtdre_Case { name: "sk2", func: rbtdte_skip },
        rbtdre_Case { name: "sk3", func: rbtdte_skip },
    ];

    let tmp = rbtdth_make_scratch("allskip");
    let result = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 3);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_single_case_pass() {
    static CASES: &[rbtdre_Case] = &[rbtdre_Case {
        name: "solo-pass",
        func: rbtdte_pass,
    }];

    let tmp = rbtdth_make_scratch("solopass");
    let result = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 1);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_single_case_fail() {
    static CASES: &[rbtdre_Case] = &[rbtdre_Case {
        name: "solo-fail",
        func: rbtdte_fail,
    }];

    let tmp = rbtdth_make_scratch("solofail");
    let result = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 1);
    assert_eq!(result.skipped, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Run-all despite failures ─────────────────────────────────

#[test]
fn rbtdte_run_all_executes_every_case_despite_failures() {
    static CASES: &[rbtdre_Case] = &[
        rbtdre_Case { name: "ra-f1", func: rbtdte_fail },
        rbtdre_Case { name: "ra-p1", func: rbtdte_pass },
        rbtdre_Case { name: "ra-f2", func: rbtdte_fail },
        rbtdre_Case { name: "ra-p2", func: rbtdte_pass },
    ];

    let tmp = rbtdth_make_scratch("runall");
    let result = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();
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

fn rbtdte_write_marker(dir: &Path) -> rbtdre_Verdict {
    let _ = std::fs::write(dir.join("marker.txt"), dir.to_string_lossy().as_bytes());
    rbtdre_Verdict::Pass
}

#[test]
fn rbtdte_temp_dirs_are_distinct_and_isolated() {
    static CASES: &[rbtdre_Case] = &[
        rbtdre_Case { name: "iso-a", func: rbtdte_write_marker },
        rbtdre_Case { name: "iso-b", func: rbtdte_write_marker },
    ];

    let tmp = rbtdth_make_scratch("isolation");
    let _ = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();

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
fn rbtdte_resolve_fail_fast_independent_default_is_fail_fast() {
    let r = rbtdre_resolve_fail_fast(rbtdre_Disposition::Independent, false).unwrap();
    assert!(r);
}

#[test]
fn rbtdte_resolve_fail_fast_independent_keep_going_permitted() {
    let r = rbtdre_resolve_fail_fast(rbtdre_Disposition::Independent, true).unwrap();
    assert!(!r);
}

#[test]
fn rbtdte_resolve_fail_fast_state_progressing_default_is_fail_fast() {
    let r = rbtdre_resolve_fail_fast(rbtdre_Disposition::StateProgressing, false).unwrap();
    assert!(r);
}

#[test]
fn rbtdte_resolve_fail_fast_state_progressing_keep_going_refused() {
    let err = rbtdre_resolve_fail_fast(rbtdre_Disposition::StateProgressing, true).unwrap_err();
    assert!(err.contains("StateProgressing"));
    assert!(err.contains("keep-going"));
}

// ── Trace file content detail ────────────────────────────────

#[test]
fn rbtdte_trace_file_skip_contains_reason() {
    static CASES: &[rbtdre_Case] = &[rbtdre_Case {
        name: "traced-skip",
        func: rbtdte_skip,
    }];

    let tmp = rbtdth_make_scratch("skiptrace");
    let _ = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();

    let trace = std::fs::read_to_string(tmp.join("traced-skip").join("trace.txt")).unwrap();
    assert!(trace.contains("SKIPPED"));
    assert!(trace.contains("traced-skip"));
    assert!(trace.contains("nope"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_case_output_files_survive_in_trace_dir() {
    fn write_output(dir: &Path) -> rbtdre_Verdict {
        let _ = std::fs::write(dir.join("output.txt"), "custom output data\n");
        rbtdre_Verdict::Pass
    }

    static CASES: &[rbtdre_Case] = &[rbtdre_Case {
        name: "output-case",
        func: write_output,
    }];

    let tmp = rbtdth_make_scratch("caseoutput");
    let _ = rbtdre_run_cases(CASES, &RBTDTE_COLORS, false, &tmp).unwrap();

    let output = std::fs::read_to_string(tmp.join("output-case").join("output.txt")).unwrap();
    assert!(output.contains("custom output data"));

    // Trace also exists alongside the case output
    let trace = std::fs::read_to_string(tmp.join("output-case").join("trace.txt")).unwrap();
    assert!(trace.contains("PASSED"));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Config-evolution console ─────────────────────────────────

fn rbtdte_git(args: &[&str], root: &Path) -> std::process::Output {
    std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("git invocation")
}

#[test]
fn rbtdte_config_set_field_replaces_value_preserving_other_lines() {
    let tmp = rbtdth_make_scratch("setfield");
    let file = tmp.join("rbrv.env");
    std::fs::write(&file, "KEEP_BEFORE=1\nRBRV_ANCHOR=old\nKEEP_AFTER=2\n").unwrap();

    rbtdre_config_set_field(&file, "RBRV_ANCHOR", "new").unwrap();

    let body = std::fs::read_to_string(&file).unwrap();
    assert!(body.contains("RBRV_ANCHOR=new"));
    assert!(body.contains("KEEP_BEFORE=1"));
    assert!(body.contains("KEEP_AFTER=2"));
    assert!(!body.contains("RBRV_ANCHOR=old"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_config_zero_blanks_named_field_only() {
    let tmp = rbtdth_make_scratch("zerofield");
    let file = tmp.join("rbrd.env");
    std::fs::write(&file, "RBRD_DEPOT_MONIKER=canest3-000007\nRBRD_CLOUD_PREFIX=canc\n").unwrap();

    rbtdre_config_zero(&file, "RBRD_DEPOT_MONIKER").unwrap();

    let body = std::fs::read_to_string(&file).unwrap();
    assert!(body.contains("RBRD_DEPOT_MONIKER=\n"));
    // The sibling field is untouched — zeroing is field-scoped.
    assert!(body.contains("RBRD_CLOUD_PREFIX=canc"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_config_zero_absent_field_fails_loud() {
    let tmp = rbtdth_make_scratch("zeroabsent");
    let file = tmp.join("rbrd.env");
    std::fs::write(&file, "RBRD_CLOUD_PREFIX=canc\n").unwrap();

    // The schema-drift catch: zeroing a field that is not present errs rather
    // than silently no-op'ing (which would mask a renamed or removed field).
    let err = rbtdre_config_zero(&file, "RBRD_RENAMED_AWAY").unwrap_err();
    assert!(err.contains("RBRD_RENAMED_AWAY"));
    assert!(err.contains("not found"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_commit_nameplates_scopes_to_named_class_only() {
    let tmp = rbtdth_make_scratch("commit-scope");
    assert!(rbtdte_git(&["init", "-q"], &tmp).status.success());
    rbtdte_git(&["config", "user.email", "theurge@test"], &tmp);
    rbtdte_git(&["config", "user.name", "theurge test"], &tmp);

    // Baseline: a nameplate rbrn.env plus an unrelated tracked file, committed clean.
    let np_dir = tmp
        .join(crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR)
        .join("testnp");
    std::fs::create_dir_all(&np_dir).unwrap();
    let rbrn = np_dir.join(crate::rbtdgc_consts::RBTDGC_RBRN_FILE);
    std::fs::write(&rbrn, "RBRN_SENTRY_HALLMARK=\n").unwrap();
    let surprise = tmp.join("surprise.txt");
    std::fs::write(&surprise, "baseline\n").unwrap();
    rbtdte_git(&["add", "-A"], &tmp);
    assert!(rbtdte_git(&["commit", "-q", "-m", "baseline"], &tmp).status.success());

    // Dirty BOTH the owned nameplate file and an unrelated file — the exact
    // wrap-sweeps-everything hazard the scoped verb exists to prevent.
    std::fs::write(&rbrn, "RBRN_SENTRY_HALLMARK=kabc123\n").unwrap();
    std::fs::write(&surprise, "SURPRISE EDIT — must not be swept\n").unwrap();

    rbtdre_commit_nameplates(&tmp, &["testnp"], "test: nameplate hallmark").unwrap();

    let status = rbtdte_git(&["status", "--porcelain"], &tmp);
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
