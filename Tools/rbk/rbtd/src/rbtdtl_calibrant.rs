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
// Tests for rbtchl_calibrant — calibrant fixture foundation. Tests pin the
// case-registration ground truth that the touchstone surface fixture depends
// on: disposition tags, sections registered, each fixture's manifest census
// declaration, per-case verdicts, sentinel write/non-write contracts.
//
// Tests look up cases through the public registry (rbtcea_lookup_fixture)
// rather than calling case fns directly — exercising the same dispatch path the
// engine uses, so a registration-without-implementation regression is caught.

use std::path::PathBuf;

use crate::rbtgec_consts::RBTGEC_THEURGE_NIHIL;
use crate::rbtcea_almanac::rbtcea_lookup_fixture;
use crate::rbtcee_engine::{rbtcee_find_case, rbtcee_Disposition, rbtcee_Verdict};
use crate::rbtchl_calibrant::{RBTCHL_OUTPUT_FILE, RBTCHL_SENTINEL_FILE};
use crate::rbtcem_manifest::{
    rbtcem_required_colophons, RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED,
    RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED, RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED,
    RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST, RBTCEM_FIXTURE_CALIBRANT_PROGRESSING,
    RBTCEM_FIXTURE_CALIBRANT_SENTINEL, RBTCEM_FIXTURE_CALIBRANT_VERDICTS,
};
use crate::rbtteh_helpers::rbtteh_make_scratch;

fn rbtthl_run_case(fixture: &'static str, case_name: &str) -> (rbtcee_Verdict, PathBuf) {
    let fix = rbtcea_lookup_fixture(fixture)
        .unwrap_or_else(|| panic!("fixture '{}' not registered", fixture));
    let case = rbtcee_find_case(fix.cases, case_name)
        .unwrap_or_else(|| panic!("case '{}' not found in fixture '{}'", case_name, fixture));
    let dir = rbtteh_make_scratch(case_name);
    let verdict = (case.func)(&dir);
    (verdict, dir)
}

fn rbtthl_assert_pass(verdict: &rbtcee_Verdict, label: &str) {
    match verdict {
        rbtcee_Verdict::Pass => (),
        rbtcee_Verdict::Fail(d) => panic!("{}: expected Pass, got Fail({})", label, d),
        rbtcee_Verdict::Skip(d) => panic!("{}: expected Pass, got Skip({})", label, d),
    }
}

fn rbtthl_assert_fail_with(verdict: &rbtcee_Verdict, needle: &str, label: &str) {
    match verdict {
        rbtcee_Verdict::Fail(d) => assert!(
            d.contains(needle),
            "{}: Fail detail did not contain '{}': {}",
            label,
            needle,
            d
        ),
        other => panic!("{}: expected Fail, got {:?}", label, zrbtthl_fmt_other(other)),
    }
}

fn rbtthl_assert_skip(verdict: &rbtcee_Verdict, label: &str) {
    if !matches!(verdict, rbtcee_Verdict::Skip(_)) {
        panic!("{}: expected Skip, got {:?}", label, zrbtthl_fmt_other(verdict));
    }
}

fn zrbtthl_fmt_other(v: &rbtcee_Verdict) -> &'static str {
    match v {
        rbtcee_Verdict::Pass => "Pass",
        rbtcee_Verdict::Fail(_) => "Fail",
        rbtcee_Verdict::Skip(_) => "Skip",
    }
}

// ── manifest entries ────────────────────────────────────────

/// The four verdict-family calibrants shell out to nothing, so their census
/// declarations are empty. Deliberately NOT a family-wide sweep: the coverage
/// calibrants below declare (and mis-declare) the nihil colophon on purpose,
/// and are pinned by their own tests.
#[test]
fn rbtthl_verdict_family_declares_no_colophons() {
    for fixture in [
        RBTCEM_FIXTURE_CALIBRANT_VERDICTS,
        RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST,
        RBTCEM_FIXTURE_CALIBRANT_PROGRESSING,
        RBTCEM_FIXTURE_CALIBRANT_SENTINEL,
    ] {
        let req = rbtcem_required_colophons(fixture)
            .unwrap_or_else(|| panic!("fixture '{}' not registered in manifest", fixture));
        assert!(
            req.is_empty(),
            "fixture '{}' must declare empty required-colophons (no shell-outs); got {:?}",
            fixture,
            req
        );
    }
}

// ── per-case verdicts ───────────────────────────────────────

#[test]
fn rbtthl_verdicts_pass_returns_pass() {
    let (verdict, dir) = rbtthl_run_case(RBTCEM_FIXTURE_CALIBRANT_VERDICTS, "rbtchl_verdicts_pass");
    rbtthl_assert_pass(&verdict, "verdicts_pass");
    assert!(
        !dir.join(RBTCHL_OUTPUT_FILE).exists(),
        "pass case must not write output.txt"
    );
    assert!(
        !dir.join(RBTCHL_SENTINEL_FILE).exists(),
        "pass case must not write sentinel"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtthl_verdicts_fail_returns_fail() {
    let (verdict, dir) = rbtthl_run_case(RBTCEM_FIXTURE_CALIBRANT_VERDICTS, "rbtchl_verdicts_fail");
    rbtthl_assert_fail_with(&verdict, "calibrant", "verdicts_fail");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtthl_verdicts_skip_returns_skip() {
    let (verdict, dir) = rbtthl_run_case(RBTCEM_FIXTURE_CALIBRANT_VERDICTS, "rbtchl_verdicts_skip");
    rbtthl_assert_skip(&verdict, "verdicts_skip");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtthl_verdicts_pass_with_output_writes_output_file() {
    let (verdict, dir) = rbtthl_run_case(
        RBTCEM_FIXTURE_CALIBRANT_VERDICTS,
        "rbtchl_verdicts_pass_with_output",
    );
    rbtthl_assert_pass(&verdict, "verdicts_pass_with_output");
    let output = dir.join(RBTCHL_OUTPUT_FILE);
    assert!(
        output.exists(),
        "pass_with_output must write {}",
        RBTCHL_OUTPUT_FILE
    );
    let body = std::fs::read_to_string(&output).expect("read output.txt");
    assert!(!body.is_empty(), "output.txt must be non-empty");
    let _ = std::fs::remove_dir_all(&dir);
}

// ── sentinel write/non-write ────────────────────────────────

#[test]
fn rbtthl_failfast_pass_writes_no_sentinel() {
    let (verdict, dir) =
        rbtthl_run_case(RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST, "rbtchl_failfast_pass");
    rbtthl_assert_pass(&verdict, "failfast_pass");
    assert!(
        !dir.join(RBTCHL_SENTINEL_FILE).exists(),
        "failfast_pass must not write sentinel"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtthl_failfast_fail_returns_fail() {
    let (verdict, dir) =
        rbtthl_run_case(RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST, "rbtchl_failfast_fail");
    rbtthl_assert_fail_with(&verdict, "fail trigger", "failfast_fail");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtthl_failfast_not_reached_writes_sentinel_when_run() {
    let (verdict, dir) = rbtthl_run_case(
        RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST,
        "rbtchl_failfast_not_reached",
    );
    rbtthl_assert_pass(&verdict, "not_reached");
    assert!(
        dir.join(RBTCHL_SENTINEL_FILE).exists(),
        "not_reached must write sentinel when run"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtthl_sentinel_marks_writes_sentinel() {
    let (verdict, dir) =
        rbtthl_run_case(RBTCEM_FIXTURE_CALIBRANT_SENTINEL, "rbtchl_sentinel_marks");
    rbtthl_assert_pass(&verdict, "sentinel_marks");
    assert!(
        dir.join(RBTCHL_SENTINEL_FILE).exists(),
        "calibrant-sentinel must write sentinel"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// ── progressing probe paths ─────────────────────────────────

#[test]
fn rbtthl_progressing_probe_ok_passes() {
    let (verdict, dir) = rbtthl_run_case(
        RBTCEM_FIXTURE_CALIBRANT_PROGRESSING,
        "rbtchl_progressing_probe_ok",
    );
    rbtthl_assert_pass(&verdict, "progressing_probe_ok");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rbtthl_progressing_probe_err_fails_with_diagnostic() {
    let (verdict, dir) = rbtthl_run_case(
        RBTCEM_FIXTURE_CALIBRANT_PROGRESSING,
        "rbtchl_progressing_probe_err",
    );
    rbtthl_assert_fail_with(&verdict, "precondition", "progressing_probe_err");
    rbtthl_assert_fail_with(&verdict, "remediation:", "progressing_probe_err");
    let _ = std::fs::remove_dir_all(&dir);
}

// ── coverage fixtures: registration shape only ───────────────
//
// The coverage cases call rbtccc_with_ctx to reach the real invocation
// chokepoint, which panics without an installed rbtdrc context (see
// rbtccc_crucible::rbtccc_with_ctx). Unlike the case bodies above — pure
// functions the engine can run standalone — these need a real fixture run
// (installed context, armed census) to execute meaningfully; that exercise is
// the touchstone surface fixture's job (child rbt runs, exit-code/stderr
// assertions). These tests pin the registration ground truth only: each
// fixture is registered, Independent, single-case, with the manifest
// declaration the coverage story depends on.

fn rbtthl_assert_registered_single_case(fixture: &'static str, case_name: &str) {
    let fix = rbtcea_lookup_fixture(fixture)
        .unwrap_or_else(|| panic!("fixture '{}' not registered", fixture));
    assert!(
        matches!(fix.disposition, rbtcee_Disposition::Independent),
        "fixture '{}' must be Independent",
        fixture
    );
    assert_eq!(
        fix.cases.len(),
        1,
        "fixture '{}' must register exactly one case",
        fixture
    );
    assert!(
        rbtcee_find_case(fix.cases, case_name).is_some(),
        "fixture '{}' must register case '{}'",
        fixture,
        case_name
    );
}

#[test]
fn rbtthl_coverage_aligned_declares_and_registers_nihil() {
    rbtthl_assert_registered_single_case(
        RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED,
        "rbtchl_coverage_aligned_invokes",
    );
    let req = rbtcem_required_colophons(RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED)
        .expect("coverage-aligned must carry a manifest entry");
    assert_eq!(
        req,
        &[RBTGEC_THEURGE_NIHIL],
        "coverage-aligned must declare exactly the nihil colophon"
    );
}

#[test]
fn rbtthl_coverage_undeclared_declares_empty_and_registers() {
    rbtthl_assert_registered_single_case(
        RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED,
        "rbtchl_coverage_undeclared_invokes",
    );
    let req = rbtcem_required_colophons(RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED)
        .expect("coverage-undeclared must carry a manifest entry");
    assert!(
        req.is_empty(),
        "coverage-undeclared must declare no colophons; got {:?}",
        req
    );
}

#[test]
fn rbtthl_coverage_unused_declares_and_registers_nihil() {
    rbtthl_assert_registered_single_case(
        RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED,
        "rbtchl_coverage_unused_no_invoke",
    );
    let req = rbtcem_required_colophons(RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED)
        .expect("coverage-unused must carry a manifest entry");
    assert_eq!(
        req,
        &[RBTGEC_THEURGE_NIHIL],
        "coverage-unused must declare exactly the nihil colophon"
    );
}

#[test]
fn rbtthl_coverage_unused_case_passes_without_invoking() {
    // The one case pure-passes with no ctx/tabtarget interaction — the
    // negative census check that fails this fixture lives in the engine,
    // exercised only by a real fixture run (touchstone's sibling pace).
    let (verdict, dir) = rbtthl_run_case(
        RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED,
        "rbtchl_coverage_unused_no_invoke",
    );
    rbtthl_assert_pass(&verdict, "coverage_unused_no_invoke");
    let _ = std::fs::remove_dir_all(&dir);
}
