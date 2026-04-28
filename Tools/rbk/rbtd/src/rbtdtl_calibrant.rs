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
// Tests for rbtdrl_calibrant — calibrant fixture foundation. Tests pin the
// case-registration ground truth that the bash blackbox testbench depends on:
// disposition tags, sections registered, manifest-required colophons empty,
// per-case verdicts, sentinel write/non-write contracts.
//
// Tests look up cases through the public registry (rbtdrc_sections_for_fixture)
// rather than calling case fns directly — exercising the same dispatch path the
// engine uses, so a registration-without-implementation regression is caught.

use std::path::PathBuf;

use crate::rbtdrc_crucible::{rbtdrc_fixture_disposition, rbtdrc_sections_for_fixture};
use crate::rbtdre_engine::{rbtdre_find_case, rbtdre_Disposition, rbtdre_Verdict};
use crate::rbtdrl_calibrant::{RBTDRL_OUTPUT_FILE, RBTDRL_SENTINEL_FILE};
use crate::rbtdrm_manifest::{
    rbtdrm_required_colophons, RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST,
    RBTDRM_FIXTURE_CALIBRANT_PROGRESSING, RBTDRM_FIXTURE_CALIBRANT_SENTINEL,
    RBTDRM_FIXTURE_CALIBRANT_VERDICTS,
};

/// Allocate a unique tempdir for a case under std::env::temp_dir.
/// Caller is responsible for cleanup.
fn rbtdtl_make_tempdir(label: &str) -> PathBuf {
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = std::env::temp_dir().join(format!("rbtdtl-{}-{}-{}", label, pid, nanos));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create tempdir");
    dir
}

fn rbtdtl_run_case(fixture: &'static str, case_name: &str) -> (rbtdre_Verdict, PathBuf) {
    let sections = rbtdrc_sections_for_fixture(fixture);
    let case = rbtdre_find_case(sections, case_name)
        .unwrap_or_else(|| panic!("case '{}' not found in fixture '{}'", case_name, fixture));
    let dir = rbtdtl_make_tempdir(case_name);
    let verdict = (case.func)(&dir);
    (verdict, dir)
}

fn rbtdtl_assert_pass(verdict: &rbtdre_Verdict, label: &str) {
    match verdict {
        rbtdre_Verdict::Pass => (),
        rbtdre_Verdict::Fail(d) => panic!("{}: expected Pass, got Fail({})", label, d),
        rbtdre_Verdict::Skip(d) => panic!("{}: expected Pass, got Skip({})", label, d),
    }
}

fn rbtdtl_assert_fail_with(verdict: &rbtdre_Verdict, needle: &str, label: &str) {
    match verdict {
        rbtdre_Verdict::Fail(d) => assert!(
            d.contains(needle),
            "{}: Fail detail did not contain '{}': {}",
            label,
            needle,
            d
        ),
        other => panic!("{}: expected Fail, got {:?}", label, fmt_other(other)),
    }
}

fn rbtdtl_assert_skip(verdict: &rbtdre_Verdict, label: &str) {
    if !matches!(verdict, rbtdre_Verdict::Skip(_)) {
        panic!("{}: expected Skip, got {:?}", label, fmt_other(verdict));
    }
}

fn fmt_other(v: &rbtdre_Verdict) -> &'static str {
    match v {
        rbtdre_Verdict::Pass => "Pass",
        rbtdre_Verdict::Fail(_) => "Fail",
        rbtdre_Verdict::Skip(_) => "Skip",
    }
}

// ── manifest entries ────────────────────────────────────────

#[test]
fn rbtdtl_required_colophons_all_empty() {
    for fixture in [
        RBTDRM_FIXTURE_CALIBRANT_VERDICTS,
        RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST,
        RBTDRM_FIXTURE_CALIBRANT_PROGRESSING,
        RBTDRM_FIXTURE_CALIBRANT_SENTINEL,
    ] {
        let req = rbtdrm_required_colophons(fixture)
            .unwrap_or_else(|| panic!("fixture '{}' not registered in manifest", fixture));
        assert!(
            req.is_empty(),
            "fixture '{}' must declare empty required-colophons (no shell-outs); got {:?}",
            fixture,
            req
        );
    }
}

// ── disposition tags ────────────────────────────────────────

#[test]
fn rbtdtl_dispositions() {
    for fixture in [
        RBTDRM_FIXTURE_CALIBRANT_VERDICTS,
        RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST,
        RBTDRM_FIXTURE_CALIBRANT_SENTINEL,
    ] {
        assert_eq!(
            rbtdrc_fixture_disposition(fixture),
            rbtdre_Disposition::Independent,
            "{} must be Independent",
            fixture
        );
    }
    assert_eq!(
        rbtdrc_fixture_disposition(RBTDRM_FIXTURE_CALIBRANT_PROGRESSING),
        rbtdre_Disposition::StateProgressing,
        "calibrant-progressing must be StateProgressing"
    );
}

// ── sections registration ───────────────────────────────────

#[test]
fn rbtdtl_verdicts_sections_registered() {
    let sections = rbtdrc_sections_for_fixture(RBTDRM_FIXTURE_CALIBRANT_VERDICTS);
    assert_eq!(sections.len(), 1, "expected one section");
    assert_eq!(sections[0].name, "calibrant-verdicts");
    assert_eq!(sections[0].cases.len(), 4, "expected four cases");
}

#[test]
fn rbtdtl_fail_fast_sections_registered() {
    let sections = rbtdrc_sections_for_fixture(RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST);
    assert_eq!(sections.len(), 2, "expected two sections (intra + inter)");
    assert_eq!(sections[0].name, "fail-fast-intra-section");
    assert_eq!(sections[0].cases.len(), 3, "intra section expects 3 cases");
    assert_eq!(sections[1].name, "fail-fast-inter-section");
    assert_eq!(sections[1].cases.len(), 1, "inter section expects 1 case");
}

#[test]
fn rbtdtl_progressing_sections_registered() {
    let sections = rbtdrc_sections_for_fixture(RBTDRM_FIXTURE_CALIBRANT_PROGRESSING);
    assert_eq!(sections.len(), 1, "expected one section");
    assert_eq!(sections[0].name, "calibrant-progressing");
    assert_eq!(sections[0].cases.len(), 2, "expected two cases");
}

#[test]
fn rbtdtl_sentinel_sections_registered() {
    let sections = rbtdrc_sections_for_fixture(RBTDRM_FIXTURE_CALIBRANT_SENTINEL);
    assert_eq!(sections.len(), 1, "expected one section");
    assert_eq!(sections[0].name, "calibrant-sentinel");
    assert_eq!(sections[0].cases.len(), 1, "expected one case");
}

// ── per-case verdicts ───────────────────────────────────────

#[test]
fn rbtdtl_verdicts_pass_returns_pass() {
    let (verdict, dir) = rbtdtl_run_case(RBTDRM_FIXTURE_CALIBRANT_VERDICTS, "rbtdrl_verdicts_pass");
    rbtdtl_assert_pass(&verdict, "verdicts_pass");
    assert!(
        !dir.join(RBTDRL_OUTPUT_FILE).exists(),
        "pass case must not write output.txt"
    );
    assert!(
        !dir.join(RBTDRL_SENTINEL_FILE).exists(),
        "pass case must not write sentinel"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtdtl_verdicts_fail_returns_fail() {
    let (verdict, dir) = rbtdtl_run_case(RBTDRM_FIXTURE_CALIBRANT_VERDICTS, "rbtdrl_verdicts_fail");
    rbtdtl_assert_fail_with(&verdict, "calibrant", "verdicts_fail");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtdtl_verdicts_skip_returns_skip() {
    let (verdict, dir) = rbtdtl_run_case(RBTDRM_FIXTURE_CALIBRANT_VERDICTS, "rbtdrl_verdicts_skip");
    rbtdtl_assert_skip(&verdict, "verdicts_skip");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtdtl_verdicts_pass_with_output_writes_output_file() {
    let (verdict, dir) = rbtdtl_run_case(
        RBTDRM_FIXTURE_CALIBRANT_VERDICTS,
        "rbtdrl_verdicts_pass_with_output",
    );
    rbtdtl_assert_pass(&verdict, "verdicts_pass_with_output");
    let output = dir.join(RBTDRL_OUTPUT_FILE);
    assert!(
        output.exists(),
        "pass_with_output must write {}",
        RBTDRL_OUTPUT_FILE
    );
    let body = std::fs::read_to_string(&output).expect("read output.txt");
    assert!(!body.is_empty(), "output.txt must be non-empty");
    let _ = std::fs::remove_dir_all(&dir);
}

// ── sentinel write/non-write ────────────────────────────────

#[test]
fn rbtdtl_failfast_pass_writes_no_sentinel() {
    let (verdict, dir) =
        rbtdtl_run_case(RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST, "rbtdrl_failfast_pass");
    rbtdtl_assert_pass(&verdict, "failfast_pass");
    assert!(
        !dir.join(RBTDRL_SENTINEL_FILE).exists(),
        "failfast_pass must not write sentinel"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtdtl_failfast_fail_returns_fail() {
    let (verdict, dir) =
        rbtdtl_run_case(RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST, "rbtdrl_failfast_fail");
    rbtdtl_assert_fail_with(&verdict, "intra-section", "failfast_fail");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtdtl_failfast_not_reached_intra_writes_sentinel_when_run() {
    let (verdict, dir) = rbtdtl_run_case(
        RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST,
        "rbtdrl_failfast_not_reached_intra",
    );
    rbtdtl_assert_pass(&verdict, "not_reached_intra");
    assert!(
        dir.join(RBTDRL_SENTINEL_FILE).exists(),
        "intra not_reached must write sentinel when run"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtdtl_failfast_not_reached_inter_writes_sentinel_when_run() {
    let (verdict, dir) = rbtdtl_run_case(
        RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST,
        "rbtdrl_failfast_not_reached_inter",
    );
    rbtdtl_assert_pass(&verdict, "not_reached_inter");
    assert!(
        dir.join(RBTDRL_SENTINEL_FILE).exists(),
        "inter not_reached must write sentinel when run"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtdtl_sentinel_marks_writes_sentinel() {
    let (verdict, dir) =
        rbtdtl_run_case(RBTDRM_FIXTURE_CALIBRANT_SENTINEL, "rbtdrl_sentinel_marks");
    rbtdtl_assert_pass(&verdict, "sentinel_marks");
    assert!(
        dir.join(RBTDRL_SENTINEL_FILE).exists(),
        "calibrant-sentinel must write sentinel"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// ── progressing probe paths ─────────────────────────────────

#[test]
fn rbtdtl_progressing_probe_ok_passes() {
    let (verdict, dir) = rbtdtl_run_case(
        RBTDRM_FIXTURE_CALIBRANT_PROGRESSING,
        "rbtdrl_progressing_probe_ok",
    );
    rbtdtl_assert_pass(&verdict, "progressing_probe_ok");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtdtl_progressing_probe_err_fails_with_diagnostic() {
    let (verdict, dir) = rbtdtl_run_case(
        RBTDRM_FIXTURE_CALIBRANT_PROGRESSING,
        "rbtdrl_progressing_probe_err",
    );
    rbtdtl_assert_fail_with(&verdict, "precondition", "progressing_probe_err");
    rbtdtl_assert_fail_with(&verdict, "remediation:", "progressing_probe_err");
    let _ = std::fs::remove_dir_all(&dir);
}
