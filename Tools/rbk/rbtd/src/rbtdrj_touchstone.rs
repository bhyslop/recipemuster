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
// RBTDRJ — touchstone: theurge certifying its own operator surface.
//
// A green, credless reveille member whose cases spawn child rbtd runs through
// the real tabtarget chain (tabtarget → launcher → workbench → rbte_engine →
// binary) against the deliberately-failing calibrant fixtures, asserting the
// child's exit code, its diagnostic shape on the operator-visible stream, and
// sentinel/trace files. The watcher passes; the watched calibrant fixtures
// stay out of every green suite. In the assay asterism: the calibrant needles
// are streaked on this touchstone and compared against known verdicts.
//
// Assertion surfaces: BUK dispatch folds the coordinator's stderr into stdout
// (2>&1 through its logging loop), so a child rbtd's diagnostics — which the
// RBTDRG discipline routes entirely to stderr — arrive on the PARENT'S
// captured stdout; captured stderr carries only pre-coordinator chain
// failures. Cases therefore assert on captured stdout, except the one
// stream-placement case, which runs its child under BURD_NO_LOG (streams
// unmerged, station/logging sublayer skipped) to prove diagnostics land on
// stderr rather than stdout.
//
// Child hygiene: every logged child carries a BURV_LOG_DIR override into the
// case dir, so a deliberately-failing child never truncates the station's
// shared last-log mid-suite nor seeds the logs-buk hist census the tariff
// dowse reads; TERM=dumb keeps the whole child chain colorless so text
// assertions are exact. Self-hosting note: a child run re-enters
// zrbte_build_binary (write-on-change codegen + cargo) while the parent
// binary executes — on the clean, freshly-built tree every run starts from,
// that is a fingerprint no-op, which is what keeps a Windows relink over the
// running binary out of reach.

use std::path::{Path, PathBuf};

use crate::rbtcee_case;
use crate::rbtgec_consts::{
    RBTGEC_THEURGE_CASE,
    RBTGEC_THEURGE_FIXTURE,
    RBTGEC_THEURGE_NIHIL,
    RBTGEC_THEURGE_SUITE,
};
use crate::rbtcea_almanac::RBTCEA_SUITE_NAME_CALIBRANT;
use crate::rbtccc_crucible::rbtccc_with_ctx;
use crate::rbtcee_engine::{
    RBTCEE_FLAG_KEEP_GOING,
    RBTCEE_TRACE_FILE,
    RBTCEE_WORD_FAILED,
    RBTCEE_WORD_PASSED,
    RBTCEE_WORD_SKIPPED,
    rbtcee_Case,
    rbtcee_Disposition,
    rbtcee_Fixture,
    rbtcee_Tariff,
    rbtcee_Verdict,
};
use crate::rbtcei_invocation::{
    rbtcei_InvokeResult,
    rbtcei_invoke_global,
    rbtcei_invoke_imprint_env,
};
use crate::rbtchl_calibrant::{
    RBTCHL_CASES_COVERAGE_UNDECLARED,
    RBTCHL_CASES_COVERAGE_UNUSED,
    RBTCHL_CASES_FAIL_FAST,
    RBTCHL_CASES_PROGRESSING,
    RBTCHL_CASES_SENTINEL,
    RBTCHL_CASES_VERDICTS,
    RBTCHL_OUTPUT_FILE,
    RBTCHL_SENTINEL_FILE,
};
use crate::rbtcem_manifest::{
    RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED,
    RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED,
    RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED,
    RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST,
    RBTCEM_FIXTURE_CALIBRANT_PROGRESSING,
    RBTCEM_FIXTURE_CALIBRANT_TARIFF_DRIFT,
    RBTCEM_FIXTURE_CALIBRANT_VERDICTS,
    RBTCEM_FIXTURE_TOUCHSTONE,
};
use crate::rbtcep_platform::rbtcep_native_to_posix;

/// Child dispatch's trace-tree subdir under its BURD_TEMP_DIR — pinned to the
/// literal in rbtd_allocate_roots (main.rs). A layout change there fails
/// these assertions loudly rather than letting them pass vacuously.
const ZRBTCCT_CHILD_TRACE_SUBDIR: &str = "rbtd";

// ── Child runner ─────────────────────────────────────────────

/// One observed child run: the captured streams and exit code, plus the two
/// roots the parent handed the child — the BURV temp root (under which the
/// child's dispatch minted its temp-<stamp>/ generation, holding the child
/// rbtd trace tree) and the BURV_LOG_DIR override (None for a BURD_NO_LOG
/// child, which writes no logs at all).
struct zrbtcct_Child {
    exit_code: i32,
    stdout: String,
    stderr: String,
    burv_temp: PathBuf,
    log_dir: Option<PathBuf>,
}

/// Spawn one child theurge run through the real tabtarget chain and capture
/// its observable surface. `imprint` Some resolves the imprinted form (the
/// rbw-ts suite tabtargets); None resolves the global form (rbw-tf / rbw-tc).
fn zrbtcct_child(
    dir: &Path,
    label: &str,
    colophon: &str,
    imprint: Option<&str>,
    args: &[&str],
    no_log: bool,
) -> Result<zrbtcct_Child, rbtcee_Verdict> {
    let mut env: Vec<(String, String)> = vec![("TERM".to_string(), "dumb".to_string())];
    let log_dir = if no_log {
        env.push(("BURD_NO_LOG".to_string(), "1".to_string()));
        None
    } else {
        let d = dir.join(format!("{}-logs", label));
        std::fs::create_dir_all(&d).map_err(|e| {
            rbtcee_Verdict::Fail(format!("{}: create child log dir: {}", label, e))
        })?;
        env.push(("BURV_LOG_DIR".to_string(), rbtcep_native_to_posix(&d)));
        Some(d)
    };
    let env_refs: Vec<(&str, &str)> =
        env.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();

    let mut captured: Option<(rbtcei_InvokeResult, PathBuf)> = None;
    let invoke_verdict = rbtccc_with_ctx(|ctx| {
        let invoked = match imprint {
            Some(imp) => rbtcei_invoke_imprint_env(ctx, colophon, imp, args, &env_refs),
            None => rbtcei_invoke_global(ctx, colophon, args, &env_refs),
        };
        match invoked {
            Ok(result) => {
                let invoke_dir = match result.burv_output.file_name() {
                    Some(n) => n.to_os_string(),
                    None => {
                        return rbtcee_Verdict::Fail(format!(
                            "{}: burv_output has no dir name: {}",
                            label,
                            result.burv_output.display()
                        ))
                    }
                };
                let burv_temp = ctx.burv_temp_root.join(invoke_dir);
                captured = Some((result, burv_temp));
                rbtcee_Verdict::Pass
            }
            Err(e) => rbtcee_Verdict::Fail(format!("{}: {}", label, e)),
        }
    });
    if !matches!(invoke_verdict, rbtcee_Verdict::Pass) {
        return Err(invoke_verdict);
    }
    let (result, burv_temp) = captured.expect("captured set on Pass verdict");

    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &result.stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &result.stderr);

    Ok(zrbtcct_Child {
        exit_code: result.exit_code,
        stdout: result.stdout,
        stderr: result.stderr,
        burv_temp,
        log_dir,
    })
}

// ── Assertion helpers ────────────────────────────────────────

fn zrbtcct_expect(cond: bool, detail: String) -> Result<(), rbtcee_Verdict> {
    if cond {
        Ok(())
    } else {
        Err(rbtcee_Verdict::Fail(detail))
    }
}

/// Tail excerpt for failure messages — the whole child output would drown the
/// verdict; the tail usually carries the summary and the miss.
fn zrbtcct_tail(s: &str) -> String {
    const KEEP: usize = 800;
    if s.len() <= KEEP {
        return s.to_string();
    }
    let mut start = s.len() - KEEP;
    while !s.is_char_boundary(start) {
        start += 1;
    }
    format!("…{}", &s[start..])
}

fn zrbtcct_expect_contains(hay: &str, needle: &str, what: &str) -> Result<(), rbtcee_Verdict> {
    zrbtcct_expect(
        hay.contains(needle),
        format!(
            "{}: expected to contain '{}'\n--- observed tail ---\n{}",
            what,
            needle,
            zrbtcct_tail(hay)
        ),
    )
}

fn zrbtcct_expect_lacks(hay: &str, needle: &str, what: &str) -> Result<(), rbtcee_Verdict> {
    zrbtcct_expect(
        !hay.contains(needle),
        format!(
            "{}: expected NOT to contain '{}'\n--- observed tail ---\n{}",
            what,
            needle,
            zrbtcct_tail(hay)
        ),
    )
}

fn zrbtcct_expect_nonzero(child: &zrbtcct_Child, what: &str) -> Result<(), rbtcee_Verdict> {
    zrbtcct_expect(
        child.exit_code != 0,
        format!(
            "{}: expected nonzero child exit, got 0\n--- stdout tail ---\n{}",
            what,
            zrbtcct_tail(&child.stdout)
        ),
    )
}

fn zrbtcct_expect_zero(child: &zrbtcct_Child, what: &str) -> Result<(), rbtcee_Verdict> {
    zrbtcct_expect(
        child.exit_code == 0,
        format!(
            "{}: child exited {}\n--- stdout tail ---\n{}\n--- stderr tail ---\n{}",
            what,
            child.exit_code,
            zrbtcct_tail(&child.stdout),
            zrbtcct_tail(&child.stderr)
        ),
    )
}

/// Resolve the single temp-<stamp>/ generation the child's dispatch minted
/// under the BURV temp root this fixture handed it. Exactly one is expected —
/// one dispatch per invoke; zero means the chain never reached dispatch.
fn zrbtcct_child_temp_gen(
    child: &zrbtcct_Child,
    label: &str,
) -> Result<PathBuf, rbtcee_Verdict> {
    let entries = std::fs::read_dir(&child.burv_temp).map_err(|e| {
        rbtcee_Verdict::Fail(format!(
            "{}: read child temp root {}: {}",
            label,
            child.burv_temp.display(),
            e
        ))
    })?;
    let gens: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("temp-"))
        })
        .collect();
    match gens.len() {
        1 => Ok(gens.into_iter().next().unwrap()),
        n => Err(rbtcee_Verdict::Fail(format!(
            "{}: expected exactly one temp-* generation under {}, found {}",
            label,
            child.burv_temp.display(),
            n
        ))),
    }
}

/// One case's directory in the child rbtd's trace tree:
/// temp-<stamp>/rbtd/<case fn name>/.
fn zrbtcct_child_case_dir(
    child: &zrbtcct_Child,
    label: &str,
    case_name: &str,
) -> Result<PathBuf, rbtcee_Verdict> {
    Ok(zrbtcct_child_temp_gen(child, label)?
        .join(ZRBTCCT_CHILD_TRACE_SUBDIR)
        .join(case_name))
}

fn zrbtcct_read(path: &Path, what: &str) -> Result<String, rbtcee_Verdict> {
    std::fs::read_to_string(path)
        .map_err(|e| rbtcee_Verdict::Fail(format!("{}: read {}: {}", what, path.display(), e)))
}

/// Fold a Result-style case body into the engine's Verdict shape.
fn zrbtcct_run(body: impl FnOnce() -> Result<(), rbtcee_Verdict>) -> rbtcee_Verdict {
    match body() {
        Ok(()) => rbtcee_Verdict::Pass,
        Err(v) => v,
    }
}

// ── verdict-propagation ─────────────────────────────────────
//
// Calibrant-verdicts case order is pinned by the RBTDRL const asserts:
// [0] pass, [1] fail, [2] skip, [3] pass_with_output. Names are taken from
// the case arrays (stringify!'d fn names), so a calibrant rename tracks here
// automatically.

/// Single-case child running the deterministic pass case: exit 0, PASSED on
/// the operator stream, trace file recording the verdict.
fn rbtcct_verdict_pass_exits_zero(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "single-pass";
        let pass_case = RBTCHL_CASES_VERDICTS[0].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_CASE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_VERDICTS, pass_case],
            false,
        )?;
        zrbtcct_expect_zero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_PASSED, "single-pass stdout")?;
        zrbtcct_expect_contains(&child.stdout, pass_case, "single-pass stdout")?;
        let trace = zrbtcct_child_case_dir(&child, label, pass_case)?.join(RBTCEE_TRACE_FILE);
        let content = zrbtcct_read(&trace, "single-pass trace")?;
        zrbtcct_expect_contains(&content, RBTCEE_WORD_PASSED, "single-pass trace content")
    })
}

/// Single-case child running the deterministic skip case: a skip is not a
/// failure, so the child exits 0 with SKIPPED on the operator stream.
fn rbtcct_verdict_skip_exits_zero(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "single-skip";
        let skip_case = RBTCHL_CASES_VERDICTS[2].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_CASE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_VERDICTS, skip_case],
            false,
        )?;
        zrbtcct_expect_zero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_SKIPPED, "single-skip stdout")?;
        let trace = zrbtcct_child_case_dir(&child, label, skip_case)?.join(RBTCEE_TRACE_FILE);
        let content = zrbtcct_read(&trace, "single-skip trace")?;
        zrbtcct_expect_contains(&content, RBTCEE_WORD_SKIPPED, "single-skip trace content")
    })
}

/// Full-fixture child (default fail-fast): the deterministic fail yields a
/// nonzero exit carrying the fail detail, and the cases after the failure
/// never run — their trace dirs are never created.
fn rbtcct_verdict_fail_exits_nonzero(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "verdicts-default";
        let pass_case = RBTCHL_CASES_VERDICTS[0].name;
        let fail_case = RBTCHL_CASES_VERDICTS[1].name;
        let skip_case = RBTCHL_CASES_VERDICTS[2].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_VERDICTS],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_PASSED, "verdicts-default stdout")?;
        zrbtcct_expect_contains(&child.stdout, pass_case, "verdicts-default stdout")?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_FAILED, "verdicts-default stdout")?;
        zrbtcct_expect_contains(&child.stdout, fail_case, "verdicts-default stdout")?;
        zrbtcct_expect_contains(
            &child.stdout,
            "calibrant deterministic fail verdict",
            "verdicts-default stdout",
        )?;
        let skip_dir = zrbtcct_child_case_dir(&child, label, skip_case)?;
        zrbtcct_expect(
            !skip_dir.exists(),
            format!(
                "verdicts-default: case after the failure ran under default fail-fast — {} exists",
                skip_dir.display()
            ),
        )
    })
}

/// Full-fixture child under --keep-going: all four verdict cases run — the
/// skip surfaces, the case-written output file lands, every case has a trace
/// file, and the summary counts all four.
fn rbtcct_verdict_keep_going_runs_all(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "verdicts-keep-going";
        let output_case = RBTCHL_CASES_VERDICTS[3].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_VERDICTS, RBTCEE_FLAG_KEEP_GOING],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_SKIPPED, "verdicts-keep-going stdout")?;
        zrbtcct_expect_contains(
            &child.stdout,
            "2 passed, 1 failed, 1 skipped (4 total)",
            "verdicts-keep-going stdout",
        )?;
        for case in RBTCHL_CASES_VERDICTS {
            let trace =
                zrbtcct_child_case_dir(&child, label, case.name)?.join(RBTCEE_TRACE_FILE);
            zrbtcct_expect(
                trace.is_file(),
                format!("verdicts-keep-going: missing trace {}", trace.display()),
            )?;
        }
        let output =
            zrbtcct_child_case_dir(&child, label, output_case)?.join(RBTCHL_OUTPUT_FILE);
        let content = zrbtcct_read(&output, "verdicts-keep-going case output")?;
        zrbtcct_expect_contains(
            &content,
            "calibrant case-written output",
            "verdicts-keep-going case output",
        )
    })
}

// ── fixture-fail-fast ───────────────────────────────────────

/// Default run of the fail-fast calibrant: the trailing case after the
/// deterministic fail never runs — its sentinel is absent.
fn rbtcct_failfast_default_halts_trailing(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "failfast-default";
        let trailing = RBTCHL_CASES_FAIL_FAST.last().expect("fail-fast has cases").name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        let sentinel =
            zrbtcct_child_case_dir(&child, label, trailing)?.join(RBTCHL_SENTINEL_FILE);
        zrbtcct_expect(
            !sentinel.exists(),
            format!(
                "failfast-default: trailing case ran under fail-fast — sentinel {} exists",
                sentinel.display()
            ),
        )
    })
}

/// Keep-going run of the fail-fast calibrant: the trailing case DOES run —
/// its sentinel is present — while the fail still yields a nonzero exit.
fn rbtcct_failfast_keep_going_reaches_trailing(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "failfast-keep-going";
        let trailing = RBTCHL_CASES_FAIL_FAST.last().expect("fail-fast has cases").name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST, RBTCEE_FLAG_KEEP_GOING],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        let sentinel =
            zrbtcct_child_case_dir(&child, label, trailing)?.join(RBTCHL_SENTINEL_FILE);
        zrbtcct_expect(
            sentinel.is_file(),
            format!(
                "failfast-keep-going: trailing case did not run — sentinel {} absent",
                sentinel.display()
            ),
        )
    })
}

// ── disposition-policy ──────────────────────────────────────

/// StateProgressing fixture under the default: fail-fast is forced, both
/// probe cases run (Ok passes, Err fails), nonzero exit.
fn rbtcct_progressing_default_runs_fail_fast(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "progressing-default";
        let ok_case = RBTCHL_CASES_PROGRESSING[0].name;
        let err_case = RBTCHL_CASES_PROGRESSING[1].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_PROGRESSING],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_PASSED, "progressing-default stdout")?;
        zrbtcct_expect_contains(&child.stdout, ok_case, "progressing-default stdout")?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_FAILED, "progressing-default stdout")?;
        zrbtcct_expect_contains(&child.stdout, err_case, "progressing-default stdout")
    })
}

/// StateProgressing + --keep-going is refused with the policy diagnostic
/// BEFORE any case work: nonzero exit, the refusal text on the operator
/// stream, and neither case's trace dir ever created.
fn rbtcct_progressing_keep_going_refused(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "progressing-refused";
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_PROGRESSING, RBTCEE_FLAG_KEEP_GOING],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(
            &child.stdout,
            "keep-going mode refused for StateProgressing",
            "progressing-refused stdout",
        )?;
        for case in RBTCHL_CASES_PROGRESSING {
            let case_dir = zrbtcct_child_case_dir(&child, label, case.name)?;
            zrbtcct_expect(
                !case_dir.exists(),
                format!(
                    "progressing-refused: case ran despite policy refusal — {} exists",
                    case_dir.display()
                ),
            )?;
        }
        Ok(())
    })
}

// ── probe-diagnostics ───────────────────────────────────────

/// The unmet-probe diagnostic carries the precondition name and the
/// remediation line, in rbtccp_assert's exact format. The probe label is
/// pinned to rbtchl_progressing_probe_err's rbtccp_Probe literal.
fn rbtcct_probe_diagnostic_shape(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "probe-shape";
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_PROGRESSING],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(
            &child.stdout,
            "precondition 'calibrant deterministic err' not met:",
            "probe-shape stdout",
        )?;
        zrbtcct_expect_contains(&child.stdout, "remediation:", "probe-shape stdout")
    })
}

// ── suite-abort ─────────────────────────────────────────────

/// The registered calibrant suite proves suite-level abort: its failing
/// fixture halts the run, so the sentinel fixture ordered after it never
/// runs — one fixture reported, the sentinel's trace dir absent.
fn rbtcct_suite_abort_halts_sentinel(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "suite-abort";
        let fail_case = RBTCHL_CASES_FAIL_FAST[1].name;
        let sentinel_case = RBTCHL_CASES_SENTINEL[0].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_SUITE,
            Some(RBTCEA_SUITE_NAME_CALIBRANT),
            &[],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_FAILED, "suite-abort stdout")?;
        zrbtcct_expect_contains(&child.stdout, fail_case, "suite-abort stdout")?;
        zrbtcct_expect_contains(
            &child.stdout,
            &format!("Suite '{}': 1 fixture(s) run", RBTCEA_SUITE_NAME_CALIBRANT),
            "suite-abort stdout",
        )?;
        let sentinel_dir = zrbtcct_child_case_dir(&child, label, sentinel_case)?;
        zrbtcct_expect(
            !sentinel_dir.exists(),
            format!(
                "suite-abort: sentinel fixture ran after the aborting failure — {} exists",
                sentinel_dir.display()
            ),
        )
    })
}

// ── cli-surface ─────────────────────────────────────────────

/// An unknown fixture name errors clearly, naming the fixture.
fn rbtcct_cli_unknown_fixture_errors(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "cli-unknown";
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &["touchstone-nonesuch"],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, "touchstone-nonesuch", "cli-unknown stdout")?;
        zrbtcct_expect_contains(&child.stdout, "has no registered Fixture", "cli-unknown stdout")
    })
}

/// A fixture run with no folio dies at the bash usage gate.
fn rbtcct_cli_missing_fixture_usage(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "cli-no-folio";
        let child = zrbtcct_child(dir, label, RBTGEC_THEURGE_FIXTURE, None, &[], false)?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, "No fixture", "cli-no-folio stdout")
    })
}

/// The single-case runner with a fixture but no case lists that fixture's
/// cases and its declared tariff, exiting 0.
fn rbtcct_cli_case_listing(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "cli-case-list";
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_CASE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_VERDICTS],
            false,
        )?;
        zrbtcct_expect_zero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, "declared tariff", "cli-case-list stdout")?;
        for case in RBTCHL_CASES_VERDICTS {
            zrbtcct_expect_contains(&child.stdout, case.name, "cli-case-list stdout")?;
        }
        Ok(())
    })
}

/// The single-case runner with no arguments yields usage plus the fixture
/// listing, exiting nonzero.
fn rbtcct_cli_single_usage_lists_fixtures(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "cli-single-usage";
        let child = zrbtcct_child(dir, label, RBTGEC_THEURGE_CASE, None, &[], false)?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, "usage: rbtd single", "cli-single-usage stdout")?;
        zrbtcct_expect_contains(&child.stdout, "available fixtures:", "cli-single-usage stdout")
    })
}

// ── coverage ─────────────────────────────────────────────────
//
// Black-box proof that the RBTDRI colophon census enforcement (the
// calibrant-coverage-* fixtures, rbtcem_manifest) surfaces both directions
// from outside a child run: exit code and diagnostic text naming the
// offending colophon. The fourth case exercises the engine's documented
// single-case exemption (rbtcee_run_single_case never calls
// rbtcee_check_census) directly through the real rbw-tc chokepoint.

/// Declared and invoked: both census directions are satisfied — zero exit,
/// and the per-colophon usage report names the colophon as used.
fn rbtcct_coverage_aligned_exits_zero(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "coverage-aligned";
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED],
            false,
        )?;
        zrbtcct_expect_zero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_PASSED, "coverage-aligned stdout")?;
        zrbtcct_expect_contains(
            &child.stdout,
            &format!("colophon '{}' used", RBTGEC_THEURGE_NIHIL),
            "coverage-aligned stdout",
        )
    })
}

/// Invoked but never declared: the positive census check refuses the invoke
/// at the chokepoint, failing the fixture and naming the colophon.
fn rbtcct_coverage_undeclared_fails_naming_colophon(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "coverage-undeclared";
        let case_name = RBTCHL_CASES_COVERAGE_UNDECLARED[0].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_FAILED, "coverage-undeclared stdout")?;
        zrbtcct_expect_contains(&child.stdout, case_name, "coverage-undeclared stdout")?;
        zrbtcct_expect_contains(
            &child.stdout,
            &format!(
                "invoked colophon '{}' which is not declared in its required-colophons census",
                RBTGEC_THEURGE_NIHIL
            ),
            "coverage-undeclared stdout",
        )
    })
}

/// Declared but never invoked: the case passes clean, but the engine's
/// negative census check fails the fixture afterward, naming the colophon.
fn rbtcct_coverage_unused_fails_naming_colophon(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "coverage-unused";
        let case_name = RBTCHL_CASES_COVERAGE_UNUSED[0].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_PASSED, "coverage-unused stdout")?;
        zrbtcct_expect_contains(&child.stdout, case_name, "coverage-unused stdout")?;
        zrbtcct_expect_contains(
            &child.stdout,
            &format!(
                "census — colophon '{}' declared but never invoked",
                RBTGEC_THEURGE_NIHIL
            ),
            "coverage-unused stdout",
        )
    })
}

/// The unused fixture's one case, invoked directly through the single-case
/// runner: exempt from the negative census check, so it exits 0 despite the
/// fixture-level declaration going unused.
fn rbtcct_coverage_unused_single_case_exempt(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "coverage-unused-single";
        let case_name = RBTCHL_CASES_COVERAGE_UNUSED[0].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_CASE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED, case_name],
            false,
        )?;
        zrbtcct_expect_zero(&child, label)?;
        zrbtcct_expect_contains(
            &child.stdout,
            RBTCEE_WORD_PASSED,
            "coverage-unused-single stdout",
        )?;
        zrbtcct_expect_contains(&child.stdout, case_name, "coverage-unused-single stdout")?;
        zrbtcct_expect_lacks(
            &child.stdout,
            "declared but never invoked",
            "coverage-unused-single stdout",
        )
    })
}

// ── tariff shortfall ────────────────────────────────────────
//
// Black-box proof that the tariff shortfall gate (rbtcee_engine.rs, Tariff
// section) reddens a fixture that ran short of its declared invocation count.
// The pair is a discriminating control, not a single needle: the drift
// calibrant fails, and calibrant-coverage-aligned — which declares a count of
// one and invokes exactly once — stays green under the same driver, asserted by
// the coverage-aligned case above.

/// Declared a count no case can meet, and its one case SKIPS. Both halves are
/// load-bearing: without the gate a skipped case exits zero, so this fixture is
/// exactly the silent shortfall the gate exists to close, and its redness proves
/// the gate stands on `failed == 0` alone rather than the census check's
/// `failed == 0 && skipped == 0`.
fn rbtcct_tariff_drift_fails_naming_counts(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "tariff-drift";
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_TARIFF_DRIFT],
            false,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        // The skip is what would have carried it to a zero exit — assert it
        // happened, so a needle that started failing its case instead (and so
        // reddened for the wrong reason) cannot pass this case.
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_SKIPPED, "tariff-drift stdout")?;
        zrbtcct_expect_contains(&child.stdout, RBTCEE_WORD_FAILED, "tariff-drift stdout")?;
        zrbtcct_expect_contains(
            &child.stdout,
            "tariff count-drift — 0 invocations vs declared 1",
            "tariff-drift stdout",
        )?;
        zrbtcct_expect_contains(
            &child.stdout,
            "declared work did not run",
            "tariff-drift stdout",
        )
    })
}

// ── stream-placement ────────────────────────────────────────

/// Under BURD_NO_LOG the dispatch leaves the coordinator's streams unmerged,
/// so this is where the placement contract itself is provable: rbtd's
/// diagnostics land on stderr, not stdout. Every other case reads the folded
/// stdout the logged chain produces.
fn rbtcct_stream_placement_diags_on_stderr(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "stream-placement";
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_FIXTURE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_VERDICTS],
            true,
        )?;
        zrbtcct_expect_nonzero(&child, label)?;
        zrbtcct_expect_contains(&child.stderr, RBTCEE_WORD_FAILED, "stream-placement stderr")?;
        zrbtcct_expect_contains(
            &child.stderr,
            "calibrant deterministic fail verdict",
            "stream-placement stderr",
        )?;
        zrbtcct_expect_lacks(&child.stdout, RBTCEE_WORD_FAILED, "stream-placement stdout")
    })
}

// ── log-isolation ───────────────────────────────────────────

/// The BURV_LOG_DIR override redirects the child's self-logs into the case
/// dir: the launcher spine's override slots are thereby regression-guarded,
/// and a deliberately-failing child can never write the station's logs-buk.
fn rbtcct_log_override_isolates(dir: &Path) -> rbtcee_Verdict {
    zrbtcct_run(|| {
        let label = "log-isolation";
        let pass_case = RBTCHL_CASES_VERDICTS[0].name;
        let child = zrbtcct_child(
            dir,
            label,
            RBTGEC_THEURGE_CASE,
            None,
            &[RBTCEM_FIXTURE_CALIBRANT_VERDICTS, pass_case],
            false,
        )?;
        zrbtcct_expect_zero(&child, label)?;
        let log_dir = child.log_dir.clone().expect("logged child carries a log dir");
        zrbtcct_expect_contains(
            &child.stdout,
            &rbtcep_native_to_posix(&log_dir),
            "log-isolation stdout (log-files preamble)",
        )?;
        let names: Vec<String> = std::fs::read_dir(&log_dir)
            .map_err(|e| {
                rbtcee_Verdict::Fail(format!(
                    "log-isolation: read override log dir {}: {}",
                    log_dir.display(),
                    e
                ))
            })?
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();
        zrbtcct_expect(
            names.iter().any(|n| n.starts_with("hist-")),
            format!("log-isolation: no hist-* self-log in override dir (found: {:?})", names),
        )?;
        zrbtcct_expect(
            names.iter().any(|n| n.starts_with("same-")),
            format!("log-isolation: no same-* self-log in override dir (found: {:?})", names),
        )?;
        zrbtcct_expect(
            names.len() >= 3,
            format!("log-isolation: expected ≥3 self-logs in override dir, found {:?}", names),
        )
    })
}

// ── Case array and fixture static ───────────────────────────

pub static RBTCCT_CASES_TOUCHSTONE: &[rbtcee_Case] = &[
    // verdict-propagation
    rbtcee_case!(rbtcct_verdict_pass_exits_zero),
    rbtcee_case!(rbtcct_verdict_skip_exits_zero),
    rbtcee_case!(rbtcct_verdict_fail_exits_nonzero),
    rbtcee_case!(rbtcct_verdict_keep_going_runs_all),
    // fixture-fail-fast
    rbtcee_case!(rbtcct_failfast_default_halts_trailing),
    rbtcee_case!(rbtcct_failfast_keep_going_reaches_trailing),
    // disposition-policy
    rbtcee_case!(rbtcct_progressing_default_runs_fail_fast),
    rbtcee_case!(rbtcct_progressing_keep_going_refused),
    // probe-diagnostics
    rbtcee_case!(rbtcct_probe_diagnostic_shape),
    // suite-abort
    rbtcee_case!(rbtcct_suite_abort_halts_sentinel),
    // cli-surface
    rbtcee_case!(rbtcct_cli_unknown_fixture_errors),
    rbtcee_case!(rbtcct_cli_missing_fixture_usage),
    rbtcee_case!(rbtcct_cli_case_listing),
    rbtcee_case!(rbtcct_cli_single_usage_lists_fixtures),
    // coverage
    rbtcee_case!(rbtcct_coverage_aligned_exits_zero),
    rbtcee_case!(rbtcct_coverage_undeclared_fails_naming_colophon),
    rbtcee_case!(rbtcct_coverage_unused_fails_naming_colophon),
    rbtcee_case!(rbtcct_coverage_unused_single_case_exempt),
    // tariff-shortfall
    rbtcee_case!(rbtcct_tariff_drift_fails_naming_counts),
    // stream-placement
    rbtcee_case!(rbtcct_stream_placement_diags_on_stderr),
    // log-isolation
    rbtcee_case!(rbtcct_log_override_isolates),
];

/// credless: touchstone is a reveille member, so the guard rides every child
/// launch (BURE_TWEAK_NAME = credless guard) — inert on the rbw-t* chain,
/// which mints no tokens, and required by the reveille slot-reservation rule.
pub static RBTCCT_FIXTURE_TOUCHSTONE: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_TOUCHSTONE,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCCT_CASES_TOUCHSTONE,
    credless: true,
    tariff: rbtcee_Tariff { min_secs: None, max_secs: None, invocations: Some(21) },
};
const _: () = assert!(RBTCCT_FIXTURE_TOUCHSTONE.cases.len() == 21);
