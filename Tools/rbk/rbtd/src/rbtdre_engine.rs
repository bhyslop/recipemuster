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

// RCG output discipline: all emission via rbtdrg_*! — no direct println!/eprintln!

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};

// ── Heartbeat ──────────────────────────────────────────────────

/// Interval between progress heartbeats for a blocking case.
///
/// A case's function is an opaque blocking call — for ordain-bearing cases it
/// waits out a multi-minute cloud build whose per-poll status is captured (and
/// thus silenced) by the tabtarget invocation in rbtdri. Without a heartbeat
/// the console shows nothing between a case's start and its terminal verdict.
const RBTDRE_HEARTBEAT_INTERVAL_SECS: u64 = 30;

/// Run a case's function, emitting a periodic `running <case> (Ns elapsed)`
/// heartbeat to the console while it blocks.
///
/// The case function runs on the **calling** thread so the `rbtdrc` thread-local
/// invocation context (established before the fixture runs) stays visible to it.
/// A background thread emits the heartbeat and exits the instant the case
/// returns — the dropped sender disconnects the channel, so short cases incur no
/// added latency and only genuinely long waits ever tick.
fn rbtdre_run_with_heartbeat(case: &rbtdre_Case, case_dir: &Path) -> rbtdre_Verdict {
    let (tx, rx) = mpsc::channel::<()>();
    let name = case.name;
    let start = Instant::now();
    let interval = Duration::from_secs(RBTDRE_HEARTBEAT_INTERVAL_SECS);

    let handle = std::thread::spawn(move || loop {
        match rx.recv_timeout(interval) {
            // Case finished (sender dropped) — stop heartbeating.
            Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => return,
            // Interval elapsed with the case still running — emit progress.
            Err(mpsc::RecvTimeoutError::Timeout) => {
                crate::rbtdrg_info_now!("running {} ({}s elapsed)", name, start.elapsed().as_secs());
            }
        }
    });

    let verdict = (case.func)(case_dir);
    drop(tx);
    let _ = handle.join();
    verdict
}

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

// ── Working-tree hygiene ───────────────────────────────────────

/// Returns Ok(()) when the working tree rooted at `root` is clean
/// (`git status --porcelain` empty); Err with a human diagnostic when the tree
/// carries uncommitted changes or git itself fails. Shared by the pristine
/// fixture's Class-A check and the suite run-start hygiene guard so both express
/// "clean tree" through one implementation.
pub fn rbtdre_tree_clean(root: &Path) -> Result<(), String> {
    match std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(root)
        .output()
    {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let trimmed = stdout.trim();
            if trimmed.is_empty() {
                Ok(())
            } else {
                Err(format!(
                    "working tree not clean — uncommitted changes:\n{}",
                    trimmed
                ))
            }
        }
        Ok(out) => Err(format!(
            "git status --porcelain failed (exit {}): {}",
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stderr).trim()
        )),
        Err(e) => Err(format!("git status invocation failed: {}", e)),
    }
}

// ── Fixture config-evolution console ───────────────────────────
//
// The home for the domain-intimate actions a fixture performs to evolve and
// commit the tracked config it tests. A fixture forward-evolves config in
// exactly three classes — vessel (rbrv.env), nameplate (rbrn.env), and regime
// (rbrd.env / rbrr.env) — and commits each through its own scoped verb here.
// `rbtdre_tree_clean` above is the read-side member of this console; these are
// the write side.
//
// The load-bearing property is scope: every commit verb DERIVES its own paths
// from a class identifier (a nameplate moniker, a vessel dir, a regime-file
// tag) and stages only those. No verb accepts a free-form file list, so a
// fixture is structurally incapable of sweeping a surprise edit — another
// officium's work, an operator's half-finished change — into its commit. The
// only entry that touches an arbitrary path list, `rbtdre_commit_paths`, is
// private to this module; external callers reach it solely through the
// class-typed verbs. Doctrine: claude-rbk-theurge-ifrit-context.md.

/// Vessel-local regime file. The vessel rbrv.env has no rbcc source-of-truth
/// home (it is composed Rust-side, not by bash — see rbtdrp_attest), so it
/// stays a bare literal here, as it does at the other vessel sites.
const RBTDRE_VESSEL_RBRV_FILE: &str = "rbrv.env";

/// Stage and commit EXACTLY the given repo-relative paths — never a wider set.
/// Module-private: the class verbs are the only callers, which is what seals
/// the can't-sweep property — no external caller can hand this an arbitrary
/// list. A scoped `git status --porcelain -- <paths>` runs first; an empty
/// result means none of the owned paths changed (an idempotent re-run, a
/// terminal step with no consumers) and is a clean no-op, not an error. An
/// empty `paths` is itself a no-op — guarding it is essential, since a bare
/// `git status --porcelain --` with no pathspec would survey the WHOLE tree
/// and defeat the scoping.
fn rbtdre_commit_paths(root: &Path, paths: &[String], message: &str) -> Result<(), String> {
    if paths.is_empty() {
        return Ok(());
    }

    let mut status_args: Vec<&str> = vec!["status", "--porcelain", "--"];
    status_args.extend(paths.iter().map(String::as_str));
    let status = std::process::Command::new("git")
        .args(&status_args)
        .current_dir(root)
        .output()
        .map_err(|e| format!("git status --porcelain exec failed: {}", e))?;
    if !status.status.success() {
        return Err(format!(
            "git status --porcelain exited {}: {}",
            status.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&status.stderr).trim()
        ));
    }
    if String::from_utf8_lossy(&status.stdout).trim().is_empty() {
        // None of the owned paths changed — clean no-op, not an error.
        return Ok(());
    }

    let mut add_args: Vec<&str> = vec!["add", "--"];
    add_args.extend(paths.iter().map(String::as_str));
    let add = std::process::Command::new("git")
        .args(&add_args)
        .current_dir(root)
        .output()
        .map_err(|e| format!("git add exec failed: {}", e))?;
    if !add.status.success() {
        return Err(format!(
            "git add exited {}: {}",
            add.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&add.stderr).trim()
        ));
    }

    let commit = std::process::Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(root)
        .output()
        .map_err(|e| format!("git commit exec failed: {}", e))?;
    if !commit.status.success() {
        return Err(format!(
            "git commit exited {}: {}",
            commit.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&commit.stderr).trim()
        ));
    }
    Ok(())
}

/// Commit the named nameplates' rbrn.env files and nothing else. Each moniker
/// derives `<moorings>/<nameplate>/rbrn.env`.
pub fn rbtdre_commit_nameplates(
    root: &Path,
    nameplates: &[&str],
    message: &str,
) -> Result<(), String> {
    let paths: Vec<String> = nameplates
        .iter()
        .map(|np| {
            format!(
                "{}/{}/{}",
                crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR,
                np,
                crate::rbtdgc_consts::RBTDGC_RBRN_FILE
            )
        })
        .collect();
    rbtdre_commit_paths(root, &paths, message)
}

/// Commit the named vessels' rbrv.env files and nothing else. Each vessel dir
/// (a moorings-relative path) derives `<vessel_dir>/rbrv.env`.
pub fn rbtdre_commit_vessels(
    root: &Path,
    vessel_dirs: &[&str],
    message: &str,
) -> Result<(), String> {
    let paths: Vec<String> = vessel_dirs
        .iter()
        .map(|dir| format!("{}/{}", dir, RBTDRE_VESSEL_RBRV_FILE))
        .collect();
    rbtdre_commit_paths(root, &paths, message)
}

/// Commit EVERY vessel's rbrv.env — the set a wildcard yoke rewrites. Still
/// vessel-scoped: the enumeration walks only the vessels dir and only collects
/// rbrv.env leaves, so nothing outside the vessel class can ride along. Paths
/// are sorted for stable commit staging.
pub fn rbtdre_commit_vessels_all(root: &Path, message: &str) -> Result<(), String> {
    let vessels_rel = crate::rbtd_vessels_dir!();
    let vessels_abs = root.join(vessels_rel);
    let mut paths: Vec<String> = Vec::new();
    let entries = std::fs::read_dir(&vessels_abs)
        .map_err(|e| format!("read vessels dir {}: {}", vessels_abs.display(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("read vessels dir entry: {}", e))?;
        if entry.path().is_dir() {
            let name = entry.file_name();
            let rbrv_rel = format!(
                "{}/{}/{}",
                vessels_rel,
                name.to_string_lossy(),
                RBTDRE_VESSEL_RBRV_FILE
            );
            if root.join(&rbrv_rel).is_file() {
                paths.push(rbrv_rel);
            }
        }
    }
    paths.sort();
    rbtdre_commit_paths(root, &paths, message)
}

/// The regime files a fixture forward-evolves in place — the marshal-zero
/// family of tracked depot/runtime config.
pub enum rbtdre_RegimeFile {
    Rbrd,
    Rbrr,
}

/// Commit the named regime files and nothing else. The closed enum is what
/// keeps the regime class scoped — there is no free-form path to widen it.
pub fn rbtdre_commit_regime(
    root: &Path,
    files: &[rbtdre_RegimeFile],
    message: &str,
) -> Result<(), String> {
    let paths: Vec<String> = files
        .iter()
        .map(|f| {
            match f {
                rbtdre_RegimeFile::Rbrd => crate::rbtdgc_consts::RBTDGC_RBRD_FILE,
                rbtdre_RegimeFile::Rbrr => crate::rbtdgc_consts::RBTDGC_RBRR_FILE,
            }
            .to_string()
        })
        .collect();
    rbtdre_commit_paths(root, &paths, message)
}

/// Set a named `FIELD=value` line in an in-place tracked config file, preserving
/// every other line, via atomic rename. **Fails loud if the field is absent** —
/// the find-or-err is the schema-drift catch: a renamed or removed field stops
/// the run instead of silently writing nothing. The generalized form of the
/// vessel-env write embryo (rbtdro_write_vessel_env), and the shared core of the
/// config-zero seam below.
pub fn rbtdre_config_set_field(file: &Path, field: &str, value: &str) -> Result<(), String> {
    let f = std::fs::File::open(file).map_err(|e| format!("open {}: {}", file.display(), e))?;
    let lines: Vec<String> = BufReader::new(f)
        .lines()
        .collect::<Result<_, _>>()
        .map_err(|e| format!("read {}: {}", file.display(), e))?;

    let prefix = format!("{}=", field);
    let mut found = false;
    let rewritten: Vec<String> = lines
        .into_iter()
        .map(|line| {
            if line.starts_with(&prefix) {
                found = true;
                format!("{}={}", field, value)
            } else {
                line
            }
        })
        .collect();

    if !found {
        return Err(format!("field {} not found in {}", field, file.display()));
    }

    let mut tmp = file.to_path_buf().into_os_string();
    tmp.push(".write_tmp");
    let tmp = PathBuf::from(tmp);
    {
        let mut out = std::fs::File::create(&tmp)
            .map_err(|e| format!("create tmp for {}: {}", file.display(), e))?;
        for line in &rewritten {
            writeln!(out, "{}", line)
                .map_err(|e| format!("write tmp for {}: {}", file.display(), e))?;
        }
    }
    std::fs::rename(&tmp, file)
        .map_err(|e| format!("atomic replace {}: {}", file.display(), e))?;
    Ok(())
}

/// Zero a named field (write `FIELD=`) in an in-place tracked config file — the
/// marshal-zero-family reset seam. Zeroing before an in-place write defeats the
/// stale-value false-green: a silently-skipped write then leaves an obviously
/// empty value, not a passing stale one. The inherited fail-on-absent doubles
/// as a schema-drift catch. One validated seam, never a per-field function farm.
pub fn rbtdre_config_zero(file: &Path, field: &str) -> Result<(), String> {
    rbtdre_config_set_field(file, field, "")
}

// ── Case and Fixture ───────────────────────────────────────────

/// A named test case with a function that receives its isolated temp directory.
/// The `name` field holds the raw stringified function name (from the `case!` macro).
pub struct rbtdre_Case {
    pub name: &'static str,
    pub func: fn(&Path) -> rbtdre_Verdict,
}

/// A complete fixture definition — name, disposition, optional setup/teardown
/// hooks, and case array.
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
    pub cases: &'static [rbtdre_Case],
    /// Reveille-tier credless guard (BUS0 tweak doctrine, slot-reservation rule).
    /// When true, every tabtarget Command built while this fixture runs carries
    /// `BURE_TWEAK_NAME=<credless guard>`, and the token-mint membranes reject
    /// with the credless band code — the fixture cannot use credentials, by
    /// construction, regardless of which suite hosts it. True for exactly the
    /// reveille-suite members; a guarded fixture's cases carry no tweaks of their
    /// own (in the reveille tier the slot belongs to the guard).
    pub credless: bool,
}

/// A named suite — an ordered set of fixtures run as one sequential batch.
///
/// Suite composition lives here, in theurge, not in bash: the `rbw-ts.TestSuite.*`
/// tabtargets pass only their imprint (the suite name) and theurge resolves the
/// membership. Members are `&'static rbtdre_Fixture` references to the registered
/// fixture statics, so a mistyped member is a compile error — not a name string
/// mirrored across the bash/Rust boundary that fails at runtime.
pub struct rbtdre_Suite {
    pub name: &'static str,
    pub fixtures: &'static [&'static rbtdre_Fixture],
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
    pub reset: &'static str,
}

/// Detect terminal color support from TERM environment variable.
pub fn rbtdre_detect_colors() -> rbtdre_Colors {
    match std::env::var("TERM") {
        Ok(term) if !term.is_empty() && term != "dumb" => rbtdre_Colors {
            green: "\x1b[1;32m",
            red: "\x1b[1;31m",
            yellow: "\x1b[1;33m",
            reset: "\x1b[0m",
        },
        _ => rbtdre_Colors {
            green: "",
            red: "",
            yellow: "",
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

/// Aggregate results from running a fixture's cases.
pub struct rbtdre_RunResult {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub temp_dir: PathBuf,
}

/// Run all cases sequentially, dispatching each with per-case temp dir isolation.
pub fn rbtdre_run_cases(
    cases: &[rbtdre_Case],
    colors: &rbtdre_Colors,
    fail_fast: bool,
    root_temp: &Path,
) -> Result<rbtdre_RunResult, String> {
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;

    for case in cases {
        let case_dir = root_temp.join(case.name);
        std::fs::create_dir_all(&case_dir).map_err(|e| {
            format!("rbtd: failed to create case dir '{}': {}", case.name, e)
        })?;

        let verdict = rbtdre_run_with_heartbeat(case, &case_dir);
        rbtdre_write_trace(&case_dir, case.name, &verdict);

        match &verdict {
            rbtdre_Verdict::Pass => {
                crate::rbtdrg_info_now!("{}PASSED:{} {}", colors.green, colors.reset, case.name);
                passed += 1;
            }
            rbtdre_Verdict::Fail(msg) => {
                crate::rbtdrg_info_now!("{}FAILED:{} {}", colors.red, colors.reset, case.name);
                crate::rbtdrg_info_now!("{}", msg);
                failed += 1;
                if fail_fast {
                    break;
                }
            }
            rbtdre_Verdict::Skip(_) => {
                crate::rbtdrg_info_now!(
                    "{}SKIPPED:{} {}",
                    colors.yellow, colors.reset, case.name
                );
                skipped += 1;
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

    crate::rbtdrg_info_now!(
        "{}{} passed, {} failed, {} skipped ({} total){}",
        color, result.passed, result.failed, result.skipped, total, end_color,
    );
    crate::rbtdrg_info_now!("Trace dir: {}", result.temp_dir.display());
}

// ── Single-case operations ───────────────────────────────────

/// Find a case by name in a flat case array.
pub fn rbtdre_find_case<'a>(
    cases: &'a [rbtdre_Case],
    target: &str,
) -> Option<&'a rbtdre_Case> {
    cases.iter().find(|c| c.name == target)
}

/// List all cases by name.
pub fn rbtdre_list_cases(cases: &[rbtdre_Case]) {
    for case in cases {
        crate::rbtdrg_info_now!("  {}", case.name);
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
            rbtdre_run_cases(fixture.cases, colors, fail_fast, root_temp)
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

    let verdict = rbtdre_run_with_heartbeat(case, &case_dir);
    rbtdre_write_trace(&case_dir, case.name, &verdict);

    let (passed, failed, skipped) = match &verdict {
        rbtdre_Verdict::Pass => {
            crate::rbtdrg_info_now!("{}PASSED:{} {}", colors.green, colors.reset, case.name);
            (1, 0, 0)
        }
        rbtdre_Verdict::Fail(_) => {
            crate::rbtdrg_info_now!("{}FAILED:{} {}", colors.red, colors.reset, case.name);
            (0, 1, 0)
        }
        rbtdre_Verdict::Skip(_) => {
            crate::rbtdrg_info_now!("{}SKIPPED:{} {}", colors.yellow, colors.reset, case.name);
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
