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

// ── Case and Section ───────────────────────────────────────────

/// A named test case with a function that receives its isolated temp directory.
/// The `name` field holds the raw stringified function name (from the `case!` macro).
/// Use `rbtdre_display_name` to derive the human-readable display name.
pub struct rbtdre_Case {
    pub name: &'static str,
    pub func: fn(&Path) -> rbtdre_Verdict,
}

/// A named group of test cases, printed as a section header during execution.
pub struct rbtdre_Section {
    pub name: &'static str,
    pub cases: &'static [rbtdre_Case],
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

// ── Display name ─────────────────────────────────────────────

/// Transform raw case name (stringified function name) to display name.
/// Strip module prefix (everything up to and including first `_`),
/// then convert remaining `_` to `-`.
pub fn rbtdre_display_name(raw: &str) -> String {
    let stripped = match raw.find('_') {
        Some(i) => &raw[i + 1..],
        None => raw,
    };
    stripped.replace('_', "-")
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
            let display = rbtdre_display_name(case.name);
            let case_dir = root_temp.join(&display);
            std::fs::create_dir_all(&case_dir).map_err(|e| {
                format!("rbtd: failed to create case dir '{}': {}", display, e)
            })?;

            let verdict = (case.func)(&case_dir);
            rbtdre_write_trace(&case_dir, &display, &verdict);

            match &verdict {
                rbtdre_Verdict::Pass => {
                    eprintln!("{}PASSED:{} {}", colors.green, colors.reset, display);
                    passed += 1;
                }
                rbtdre_Verdict::Fail(_) => {
                    eprintln!("{}FAILED:{} {}", colors.red, colors.reset, display);
                    failed += 1;
                    if fail_fast {
                        break 'outer;
                    }
                }
                rbtdre_Verdict::Skip(_) => {
                    eprintln!(
                        "{}SKIPPED:{} {}",
                        colors.yellow, colors.reset, display
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

/// Find a case by display name within sections.
pub fn rbtdre_find_case<'a>(
    sections: &'a [rbtdre_Section],
    target: &str,
) -> Option<&'a rbtdre_Case> {
    for section in sections {
        for case in section.cases {
            if rbtdre_display_name(case.name) == target {
                return Some(case);
            }
        }
    }
    None
}

/// List all cases with display names, grouped by section.
pub fn rbtdre_list_cases(sections: &[rbtdre_Section]) {
    for section in sections {
        eprintln!("\n--- {} ---", section.name);
        for case in section.cases {
            eprintln!("  {}", rbtdre_display_name(case.name));
        }
    }
}

/// Run a single case without charge/quench lifecycle.
pub fn rbtdre_run_single_case(
    case: &rbtdre_Case,
    colors: &rbtdre_Colors,
    root_temp: &Path,
) -> Result<rbtdre_RunResult, String> {
    let display = rbtdre_display_name(case.name);
    let case_dir = root_temp.join(&display);
    std::fs::create_dir_all(&case_dir)
        .map_err(|e| format!("rbtd: failed to create case dir '{}': {}", display, e))?;

    let verdict = (case.func)(&case_dir);
    rbtdre_write_trace(&case_dir, &display, &verdict);

    let (passed, failed, skipped) = match &verdict {
        rbtdre_Verdict::Pass => {
            eprintln!("{}PASSED:{} {}", colors.green, colors.reset, display);
            (1, 0, 0)
        }
        rbtdre_Verdict::Fail(_) => {
            eprintln!("{}FAILED:{} {}", colors.red, colors.reset, display);
            (0, 1, 0)
        }
        rbtdre_Verdict::Skip(_) => {
            eprintln!("{}SKIPPED:{} {}", colors.yellow, colors.reset, display);
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
