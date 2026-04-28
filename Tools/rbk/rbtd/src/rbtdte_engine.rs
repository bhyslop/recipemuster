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

// ── Edge cases ───────────────────────────────────────────────

#[test]
fn rbtdte_zero_sections() {
    static SECTIONS: &[rbtdre_Section] = &[];
    let tmp = rbtdte_make_temp("zerosec");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 0);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdte_section_with_zero_cases() {
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "empty",
        cases: &[],
    }];
    let tmp = rbtdte_make_temp("zerocases");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();
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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "all-skip",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("allskip");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();
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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "solo",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("solopass");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();
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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "solo",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("solofail");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();
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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "run-all",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("runall");
    let result = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();
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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "isolation",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("isolation");
    let _ = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();

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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "skip-trace",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("skiptrace");
    let _ = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();

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
    static SECTIONS: &[rbtdre_Section] = &[rbtdre_Section {
        name: "output",
        cases: CASES,
    }];

    let tmp = rbtdte_make_temp("caseoutput");
    let _ = rbtdre_run_sections(SECTIONS, &RBTDTE_COLORS, false, &tmp).unwrap();

    let output = std::fs::read_to_string(tmp.join("output-case").join("output.txt")).unwrap();
    assert!(output.contains("custom output data"));

    // Trace also exists alongside the case output
    let trace = std::fs::read_to_string(tmp.join("output-case").join("trace.txt")).unwrap();
    assert!(trace.contains("PASSED"));

    let _ = std::fs::remove_dir_all(&tmp);
}
