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
// RBTDRE — case execution engine for theurge

use std::path::Path;
use std::path::PathBuf;

// ── Verdict ────────────────────────────────────────────────────

/// Verdict from a single test case execution.
pub enum rbtdre_Verdict {
    Pass,
    Fail(String),
    Skip(String),
}

// ── Disposition ────────────────────────────────────────────────

/// Fixture disposition controls per-fixture mode policy at the engine layer.
///
/// `Independent` — cases are self-contained; suite-order is informational. Engine
/// permits keep-going mode for surveying.
///
/// `StateProgressing` — case N's success establishes preconditions for case N+1.
/// Engine refuses keep-going mode (a failed case leaves a broken precondition for
/// the next case, so continuing is incoherent). Fail-fast is forced.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum rbtdre_Disposition {
    Independent,
    StateProgressing,
}

/// Resolve the effective fail-fast bool from a disposition and a caller's
/// requested keep-going mode. Returns Err if the request is incompatible with
/// the disposition (StateProgressing + keep-going).
///
/// Centralizes the keep-going / fail-fast policy at the engine boundary so all
/// callers funnel through one place.
pub fn rbtdre_resolve_fail_fast(
    disposition: rbtdre_Disposition,
    keep_going_requested: bool,
) -> Result<bool, String> {
    match (disposition, keep_going_requested) {
        (rbtdre_Disposition::StateProgressing, true) => Err(
            "rbtd: keep-going mode refused for StateProgressing fixture — \
             a failed case leaves a broken precondition for the next case, \
             so continuing is incoherent. Run individual cases via the \
             SingleCase tabtarget if you want to survey."
                .to_string(),
        ),
        (rbtdre_Disposition::StateProgressing, false) => Ok(true),
        (rbtdre_Disposition::Independent, keep_going) => Ok(!keep_going),
    }
}

// ── Case and Section ───────────────────────────────────────────

/// A named test case with a function that receives its isolated temp directory.
/// The `name` field holds the raw stringified function name (from the `case!` macro).
pub struct rbtdre_Case {
    pub name: &'static str,
    pub func: fn(&Path) -> rbtdre_Verdict,
}

/// A named group of test cases, printed as a section header during execution.
pub struct rbtdre_Section {
    pub name: &'static str,
    pub cases: &'static [rbtdre_Case],
}

/// A complete fixture definition — name, disposition, optional setup/teardown
/// hooks, and section array. All fixture-level metadata in one place; replaces
/// the prior side-channel matchers (sections_for_fixture, fixture_disposition,
/// needs_charge, needs_readiness_delay) keyed off the fixture name string.
///
/// `setup` runs before any cases. Failure aborts the fixture; cases do not run;
/// `teardown` still runs (finally-shaped) for any partial-state cleanup.
///
/// `teardown` runs unconditionally after cases regardless of case verdicts or
/// setup outcome. Errors are surfaced as warnings — teardown is best-effort.
///
/// Both hooks access invocation context via the thread-local established by
/// `rbtdrc_set_context` before `rbtdre_run_fixture` is called.
pub struct rbtdre_Fixture {
    pub name: &'static str,
    pub disposition: rbtdre_Disposition,
    pub setup: Option<fn() -> Result<(), String>>,
    pub teardown: Option<fn()>,
    pub sections: &'static [rbtdre_Section],
}

/// Case registration macro. Derives case name from function name via `stringify!`.
/// Compiler enforces uniqueness — duplicate function names won't compile.
#[macro_export]
macro_rules! case {
    ($func:path) => {
        $crate::rbtdre_engine::rbtdre_Case {
            name: stringify!($func),
            func: $func,
        }
    };
}

// ── Colors ─────────────────────────────────────────────────────

/// Terminal color codes, empty strings when color is disabled.
pub struct rbtdre_Colors {
    pub green: &'static str,
    pub red: &'static str,
    pub yellow: &'static str,
    pub white: &'static str,
    pub reset: &'static str,
}

/// Detect terminal color support from TERM environment variable.
pub fn rbtdre_detect_colors() -> rbtdre_Colors {
    match std::env::var("TERM") {
        Ok(term) if !term.is_empty() && term != "dumb" => rbtdre_Colors {
            green: "\x1b[1;32m",
            red: "\x1b[1;31m",
            yellow: "\x1b[1;33m",
            white: "\x1b[1;37m",
            reset: "\x1b[0m",
        },
        _ => rbtdre_Colors {
            green: "",
            red: "",
            yellow: "",
            white: "",
            reset: "",
        },
    }
}

// ── Trace ──────────────────────────────────────────────────────

/// Write verdict and detail to a trace file in the case temp directory.
fn rbtdre_write_trace(case_dir: &Path, display_name: &str, verdict: &rbtdre_Verdict) {
    let content = match verdict {
        rbtdre_Verdict::Pass => format!("PASSED: {}\n", display_name),
        rbtdre_Verdict::Fail(detail) => format!("FAILED: {}\n\n{}\n", display_name, detail),
        rbtdre_Verdict::Skip(reason) => {
            format!("SKIPPED: {}\n\nReason: {}\n", display_name, reason)
        }
    };
    let _ = std::fs::write(case_dir.join("trace.txt"), content);
}

// ── Dispatch ───────────────────────────────────────────────────

/// Aggregate results from running sections.
pub struct rbtdre_RunResult {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub temp_dir: PathBuf,
}

/// Run all sections sequentially, dispatching each case with per-case temp dir isolation.
pub fn rbtdre_run_sections(
    sections: &[rbtdre_Section],
    colors: &rbtdre_Colors,
    fail_fast: bool,
    root_temp: &Path,
) -> Result<rbtdre_RunResult, String> {
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;

    'outer: for section in sections {
        eprintln!(
            "\n{}--- {} ---{}",
            colors.white, section.name, colors.reset
        );

        for case in section.cases {
            let case_dir = root_temp.join(case.name);
            std::fs::create_dir_all(&case_dir).map_err(|e| {
                format!("rbtd: failed to create case dir '{}': {}", case.name, e)
            })?;

            let verdict = (case.func)(&case_dir);
            rbtdre_write_trace(&case_dir, case.name, &verdict);

            match &verdict {
                rbtdre_Verdict::Pass => {
                    eprintln!("{}PASSED:{} {}", colors.green, colors.reset, case.name);
                    passed += 1;
                }
                rbtdre_Verdict::Fail(_) => {
                    eprintln!("{}FAILED:{} {}", colors.red, colors.reset, case.name);
                    failed += 1;
                    if fail_fast {
                        break 'outer;
                    }
                }
                rbtdre_Verdict::Skip(_) => {
                    eprintln!(
                        "{}SKIPPED:{} {}",
                        colors.yellow, colors.reset, case.name
                    );
                    skipped += 1;
                }
            }
        }
    }

    Ok(rbtdre_RunResult {
        passed,
        failed,
        skipped,
        temp_dir: root_temp.to_path_buf(),
    })
}

/// Print colored summary line and trace directory location.
pub fn rbtdre_print_summary(result: &rbtdre_RunResult, colors: &rbtdre_Colors) {
    let total = result.passed + result.failed + result.skipped;
    let (color, end_color) = if result.failed > 0 {
        (colors.red, colors.reset)
    } else {
        (colors.green, colors.reset)
    };

    eprintln!(
        "\n{}{} passed, {} failed, {} skipped ({} total){}",
        color, result.passed, result.failed, result.skipped, total, end_color,
    );
    eprintln!("Trace dir: {}", result.temp_dir.display());
}

// ── Single-case operations ───────────────────────────────────

/// Find a case by name within sections.
pub fn rbtdre_find_case<'a>(
    sections: &'a [rbtdre_Section],
    target: &str,
) -> Option<&'a rbtdre_Case> {
    for section in sections {
        for case in section.cases {
            if case.name == target {
                return Some(case);
            }
        }
    }
    None
}

/// List all cases grouped by section.
pub fn rbtdre_list_cases(sections: &[rbtdre_Section]) {
    for section in sections {
        eprintln!("\n--- {} ---", section.name);
        for case in section.cases {
            eprintln!("  {}", case.name);
        }
    }
}

/// Run a fixture: setup hook → cases → teardown hook (finally-shaped).
///
/// Expects the invocation context to already be installed in the thread-local
/// (via `rbtdrc_set_context`); setup/teardown access ctx via that channel.
///
/// Setup failure short-circuits cases but still invokes teardown. Teardown
/// errors are surfaced as warnings via stderr; the function never panics on
/// teardown failure.
pub fn rbtdre_run_fixture(
    fixture: &'static rbtdre_Fixture,
    colors: &rbtdre_Colors,
    root_temp: &Path,
) -> Result<rbtdre_RunResult, String> {
    let setup_result = match fixture.setup {
        Some(f) => f(),
        None => Ok(()),
    };

    let run_result = match setup_result {
        Ok(()) => {
            let fail_fast = rbtdre_resolve_fail_fast(fixture.disposition, false)
                .expect("disposition-default mode never fails policy resolution");
            rbtdre_run_sections(fixture.sections, colors, fail_fast, root_temp)
        }
        Err(msg) => Err(format!("rbtd: fixture '{}' setup failed: {}", fixture.name, msg)),
    };

    if let Some(f) = fixture.teardown {
        f();
    }

    run_result
}

/// Run a single case without charge/quench lifecycle.
pub fn rbtdre_run_single_case(
    case: &rbtdre_Case,
    colors: &rbtdre_Colors,
    root_temp: &Path,
) -> Result<rbtdre_RunResult, String> {
    let case_dir = root_temp.join(case.name);
    std::fs::create_dir_all(&case_dir)
        .map_err(|e| format!("rbtd: failed to create case dir '{}': {}", case.name, e))?;

    let verdict = (case.func)(&case_dir);
    rbtdre_write_trace(&case_dir, case.name, &verdict);

    let (passed, failed, skipped) = match &verdict {
        rbtdre_Verdict::Pass => {
            eprintln!("{}PASSED:{} {}", colors.green, colors.reset, case.name);
            (1, 0, 0)
        }
        rbtdre_Verdict::Fail(_) => {
            eprintln!("{}FAILED:{} {}", colors.red, colors.reset, case.name);
            (0, 1, 0)
        }
        rbtdre_Verdict::Skip(_) => {
            eprintln!("{}SKIPPED:{} {}", colors.yellow, colors.reset, case.name);
            (0, 0, 1)
        }
    };

    Ok(rbtdre_RunResult {
        passed,
        failed,
        skipped,
        temp_dir: root_temp.to_path_buf(),
    })
}
