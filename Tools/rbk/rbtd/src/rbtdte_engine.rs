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

use std::path::{Path, PathBuf};

use super::rbtdre_engine::*;

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
    white: "",
    reset: "",
};

fn rbtdte_make_temp(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rbtd-test-{}-{}", std::process::id(), label));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn rbtdte_counts_all_verdict_types() {
    static CASES: &[rbtdre_Case] = &[
        rbtdre_Case { name: "p1", func: rbtdte_pass },
        rbtdre_Case { name: "p2", func: rbtdte_pass },
        rbtdre_Case { name: "s1", func: rbtdte_skip },
        rbtdre_Case { name: "f1", func: rbtdte_fail },
    ];
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "counting",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("counts");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();
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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "fast",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("failfast");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, true, &tmp).unwrap();
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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "tracing",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("trace");
    let _ = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();

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
fn rbtdte_multiple_sections_run_sequentially() {
    static CASES_A: &[rbtdre_Case] = &[rbtdre_Case {
        name: "ms-a1",
        func: rbtdte_pass,
    }];
    static CASES_B: &[rbtdre_Case] = &[
        rbtdre_Case {
            name: "ms-b1",
            func: rbtdte_pass,
        },
        rbtdre_Case {
            name: "ms-b2",
            func: rbtdte_skip,
        },
    ];
    static SECTIONS: &[rbtdre_Section] = &[
        rbtdre_Section {
            name: "section-a",
            cases: CASES_A,
        },
        rbtdre_Section {
            name: "section-b",
            cases: CASES_B,
        },
    ];

    let tmp = rbtdte_make_temp("multi");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 2);
    assert_eq!(result.skipped, 1);
    assert_eq!(result.failed, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_fail_fast_stops_across_sections() {
    static CASES_A: &[rbtdre_Case] = &[rbtdre_Case {
        name: "ffs-f1",
        func: rbtdte_fail,
    }];
    static CASES_B: &[rbtdre_Case] = &[rbtdre_Case {
        name: "ffs-p1",
        func: rbtdte_pass,
    }];
    static SECTIONS: &[rbtdre_Section] = &[
        rbtdre_Section {
            name: "fails",
            cases: CASES_A,
        },
        rbtdre_Section {
            name: "never-reached",
            cases: CASES_B,
        },
    ];

    let tmp = rbtdte_make_temp("failacross");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, true, &tmp).unwrap();
    assert_eq!(result.failed, 1);
    assert_eq!(result.passed, 0);
    assert!(!tmp.join("ffs-p1").exists());
    let _ = std::fs::remove_dir_all(&tmp);
}
