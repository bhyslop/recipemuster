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
// RBTD Theurge — test orchestrator entry point
//
// Subcommands:
//   rbtd <fixture> [--keep-going]
//     Single-fixture runner — charge, run all cases, quench. --keep-going
//     requests keep-going mode, resolved against the fixture's disposition
//     (refused for StateProgressing) by rbtcee_resolve_fail_fast.
//   rbtd suite <suite> [--keep-going]
//     Suite runner — resolve the suite's fixtures (composition owned here, not
//     in bash) and run each in sequence, fail-fast, with one aggregate summary.
//     --keep-going applies per fixture; the cross-fixture break-on-failure is
//     unchanged.
//   rbtd single <fixture> [case]
//     Single-case runner — no charge/quench. List cases or run one.
//   rbtd dowse <log-dir>
//     Observed-tariff census — read-only report over the station's logs-buk
//     history; no tree guard, no roots, no context.

// all emission via rbtdrg_*! — no direct println!/eprintln!

#![allow(non_camel_case_types)]
#![allow(private_interfaces)]
#![deny(warnings)]

use std::path::PathBuf;
use std::process::ExitCode;

use rbt::rbtcea_almanac::{
    rbtcea_lookup_fixture, rbtcea_lookup_suite, RBTCEA_FIXTURES, RBTCEA_SUITES,
};
use rbt::rbtccc_crucible::{rbtccc_set_context, rbtccc_take_context};
use rbt::rbtcee_engine::{
    RBTCEE_FLAG_KEEP_GOING,
    rbtcee_TariffRow,
    rbtcee_detect_colors,
    rbtcee_find_case,
    rbtcee_list_cases,
    rbtcee_parse_keep_going,
    rbtcee_print_summary,
    rbtcee_print_tariff_table,
    rbtcee_run_fixture,
    rbtcee_run_single_case,
    rbtcee_tariff_declared,
    rbtcee_tree_clean,
};
use rbt::rbtcei_invocation::{
    rbtcei_Context, rbtcei_invoke_global,
    RBTCEI_BURD_TEMP_DIR_KEY,
};
use rbt::rbtccd_dowse::rbtccd_dowse;
use rbt::rbtgec_consts::RBTGEC_CRUCIBLE_ACTIVE;
use rbt::rbtcep_platform::rbtcep_path_from_env;

/// The flag the kennel's election asks a standing binary before it launches or
/// borrows one. Its home is `BKCE_EXERGUE_FLAG` in the kennel's election
/// (`Tools/bkk/bk0/src/bkce_election.rs`); spelled here because this crate links
/// no kennel.
const RBTM_FLAG_EXERGUE: &str = "--exergue";

/// Answer the kennel's `--exergue` with the position this binary was struck
/// from: what the kennel's converge stated to the build script, which handed it
/// to this compile.
///
/// THE ONE STDOUT LINE THE THEURGE WRITES, and it stands here rather than in
/// RBTDRG for the reason RBTDRG gives for leaving stdout clear: this is the
/// machine-read channel, the kennel reading the answer's position-shaped lines
/// from stdout, so the answer is the bare position and nothing else.
fn rbtm_answer_exergue() -> ExitCode {
    use std::io::Write;
    let mut stdout = std::io::stdout().lock();
    match writeln!(stdout, "{}", env!("BKCH_COLLAR_POSITION")) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some(RBTM_FLAG_EXERGUE) => rbtm_answer_exergue(),
        Some("single") => rbtm_run_single(&args[2..]),
        Some("suite") => rbtm_run_suite(&args[2..]),
        Some("dowse") => rbtm_run_dowse(&args[2..]),
        _ => rbtm_run_fixture(&args[1..]),
    }
}

// ── Dowse (observed-tariff census) ───────────────────────────

fn rbtm_run_dowse(args: &[String]) -> ExitCode {
    let log_dir = match args.first() {
        Some(d) => PathBuf::from(d),
        None => rbt::rbtcel_fatal_now!(
            "rbtd dowse: usage: rbtd dowse <log-dir>\n\
             launch via tabtarget: tt/rbw-td.TariffDowse.sh"
        ),
    };
    match rbtccd_dowse(&log_dir) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => rbt::rbtcel_fatal_now!("rbtd: {}", msg),
    }
}

struct rbtm_Roots {
    trace_root: PathBuf,
    burv_temp_root: PathBuf,
    burv_output_root: PathBuf,
}

fn rbtm_allocate_roots() -> Result<rbtm_Roots, String> {
    let burd_temp = rbtcep_path_from_env(RBTCEI_BURD_TEMP_DIR_KEY)?;

    // Both BURV roots are anchored under the per-run trace dir
    // (BURD_TEMP_DIR/rbtd), which is stable for the life of the run. The output
    // root deliberately does NOT live under BURD_OUTPUT_DIR (output-buk/current):
    // that dir is cleared at the start of every BUK dispatch, so a fact captured
    // there can be deleted out from under a still-running invocation by any
    // default-root dispatch — leaving the durable temp copy as the only survivor
    // (the failure that made rbgp_fact_governor_sa_email unreadable mid-suite).
    let trace_root = burd_temp.join("rbtd");
    let burv_temp_root = trace_root.join("burv-temp");
    let burv_output_root = trace_root.join("burv-output");

    std::fs::create_dir_all(&trace_root)
        .map_err(|e| format!("rbtd: failed to create trace root '{}': {}", trace_root.display(), e))?;
    std::fs::create_dir_all(&burv_temp_root)
        .map_err(|e| format!("rbtd: failed to create burv temp root '{}': {}", burv_temp_root.display(), e))?;
    std::fs::create_dir_all(&burv_output_root)
        .map_err(|e| format!("rbtd: failed to create burv output root '{}': {}", burv_output_root.display(), e))?;

    Ok(rbtm_Roots { trace_root, burv_temp_root, burv_output_root })
}

// ── Single-fixture runner ────────────────────────────────────

fn rbtm_run_fixture(args: &[String]) -> ExitCode {
    let (positionals, keep_going) = match rbtcee_parse_keep_going(args) {
        Ok(v) => v,
        Err(msg) => rbt::rbtcel_fatal_now!("rbtd: {}", msg),
    };
    let fixture = match positionals.first() {
        Some(n) => n,
        None => rbt::rbtcel_fatal_now!(
            "rbtd: usage: rbtd <fixture> [{}]\n\
             theurge must be launched via tabtarget (e.g. tt/rbw-tf.FixtureRun.sh tadmor)",
            RBTCEE_FLAG_KEEP_GOING
        ),
    };
    if positionals.len() > 1 {
        rbt::rbtcel_fatal_now!(
            "rbtd: unexpected argument '{}' — usage: rbtd <fixture> [{}]",
            positionals[1], RBTCEE_FLAG_KEEP_GOING
        );
    }

    let project_root = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => rbt::rbtcel_fatal_now!("rbtd: cannot determine working directory: {}", e),
    };

    // Run-start hygiene guard — reaches every drive, not only the suite (see
    // rbtcee_tree_clean's doc comment). A single-fixture run can commit config
    // via the console (rbtcee_engine's commit verbs) or drive against an
    // uncommitted engine change, so a dirty tree here refuses exactly as a
    // suite start does.
    if let Err(msg) = rbtcee_tree_clean(&project_root) {
        rbt::rbtcel_fatal_now!(
            "rbtd: refusing to run on a dirty working tree — \
             notch (commit) first, then drive.\n{}",
            msg
        );
    }

    let roots = match rbtm_allocate_roots() {
        Ok(r) => r,
        Err(msg) => rbt::rbtcel_fatal_now!("{}", msg),
    };

    let ctx = rbtcei_Context::new(
        &project_root,
        fixture,
        &roots.burv_temp_root,
        &roots.burv_output_root,
    );

    let fixture_def = match rbtcea_lookup_fixture(fixture) {
        Some(f) => f,
        None => rbt::rbtcel_fatal_now!(
            "rbtd: fixture '{}' has no registered Fixture — \
             no Fixture static is bound. \
             Update rbtcea_lookup_fixture in rbtdrc_crucible.rs.",
            fixture
        ),
    };

    rbtccc_set_context(ctx);

    let colors = rbtcee_detect_colors();
    let run_result = rbtcee_run_fixture(fixture_def, &colors, &roots.trace_root, keep_going);

    let _ctx = rbtccc_take_context();

    let result = match run_result {
        Ok(r) => r,
        Err(msg) => rbt::rbtcel_fatal_now!("rbtd: {}", msg),
    };

    rbtcee_print_summary(&result, &colors);

    if result.failed > 0 {
        rbt::rbtcel_error_now!("rbtd: {} case(s) failed", result.failed);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

// ── Suite runner ─────────────────────────────────────────────

fn rbtm_run_suite(args: &[String]) -> ExitCode {
    let (positionals, keep_going) = match rbtcee_parse_keep_going(args) {
        Ok(v) => v,
        Err(msg) => rbt::rbtcel_fatal_now!("rbtd suite: {}", msg),
    };
    let suite = match positionals.first() {
        Some(name) => match rbtcea_lookup_suite(name) {
            Some(s) => s,
            None => {
                rbt::rbtcel_error_now!("rbtd suite: unknown suite '{}'", name);
                rbtm_list_suites();
                return ExitCode::FAILURE;
            }
        },
        None => {
            rbt::rbtcel_error_now!("rbtd suite: no suite argument");
            rbtm_list_suites();
            return ExitCode::FAILURE;
        }
    };
    if positionals.len() > 1 {
        rbt::rbtcel_fatal_now!(
            "rbtd suite: unexpected argument '{}' — usage: rbtd suite <suite> [{}]",
            positionals[1], RBTCEE_FLAG_KEEP_GOING
        );
    }

    let project_root = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => rbt::rbtcel_fatal_now!("rbtd: cannot determine working directory: {}", e),
    };

    // Run-start hygiene guard, once per suite (under the bash loop it ran once
    // per fixture). A suite commits a sequence of hallmark/yoke changes; a dirty
    // tree at the start would interleave the operator's uncommitted edits. Reaches
    // every drive, not only this one — see rbtcee_tree_clean's doc comment.
    if let Err(msg) = rbtcee_tree_clean(&project_root) {
        rbt::rbtcel_fatal_now!(
            "rbtd: refusing to start a suite run on a dirty working tree — \
             notch (commit) first, then drive.\n{}",
            msg
        );
    }

    // Roots allocated once per suite; all fixtures share the trace/burv roots.
    // This matches the bash loop, where every per-fixture process inherited the
    // same BURD_TEMP_DIR and thus the same trace root.
    let roots = match rbtm_allocate_roots() {
        Ok(r) => r,
        Err(msg) => rbt::rbtcel_fatal_now!("{}", msg),
    };

    let colors = rbtcee_detect_colors();

    let mut total_passed = 0usize;
    let mut total_failed = 0usize;
    let mut total_skipped = 0usize;
    let mut ran = 0usize;
    // Collected per-fixture tariff footprints for the suite-end drift table.
    let mut tariff_rows: Vec<rbtcee_TariffRow> = Vec::new();

    // Sequential, fail-fast across fixtures — matches the bash for-loop under
    // `set -e`. A fixture whose cases fail (or whose setup errors) stops the
    // suite; fixtures already run have charged-and-quenched cleanly.
    //
    // Note: a case that *panics* (rather than returning a Fail verdict) unwinds
    // past its fixture's quench and aborts the whole suite process with that
    // fixture's crucible left charged — the same leak the per-process bash loop
    // had. Routine failure is always a Fail verdict, which quenches normally
    // (teardown is finally-shaped in rbtcee_run_fixture).
    let mut next_invoke_count = 0u32;
    for fixture in suite.fixtures {
        let mut ctx = rbtcei_Context::new(
            &project_root,
            fixture.name,
            &roots.burv_temp_root,
            &roots.burv_output_root,
        );
        // Carry the BURV invoke counter across fixtures so per-invoke dir names
        // never collide between fixtures. A fresh Context starts at 0, so without
        // this every fixture's first invoke reuses invoke-00000 — and bud's
        // start-of-dispatch current/->previous/ promotion then leaks the prior
        // fixture's chaining facts into a non-chained invoke's previous/.
        // Suite-monotonic numbering gives each invoke its own dir, closing that.
        ctx.rbtcei_set_invoke_count(next_invoke_count);
        rbtccc_set_context(ctx);

        let run_result = rbtcee_run_fixture(fixture, &colors, &roots.trace_root, keep_going);

        next_invoke_count = rbtccc_take_context().rbtcei_invoke_count();

        match run_result {
            Ok(result) => {
                rbtcee_print_summary(&result, &colors);
                tariff_rows.push(rbtcee_TariffRow {
                    name: fixture.name.to_string(),
                    tariff: fixture.tariff,
                    elapsed_secs: result.elapsed_secs,
                    invocations: result.invocations,
                });
                total_passed += result.passed;
                total_failed += result.failed;
                total_skipped += result.skipped;
                ran += 1;
                if result.failed > 0 {
                    break;
                }
            }
            Err(msg) => {
                rbt::rbtcel_error_now!("rbtd: fixture '{}': {}", fixture.name, msg);
                total_failed += 1;
                ran += 1;
                break;
            }
        }
    }

    rbtcee_print_tariff_table(&tariff_rows);

    rbt::rbtcel_info_now!(
        "Suite '{}': {} fixture(s) run, {} passed, {} failed, {} skipped",
        suite.name, ran, total_passed, total_failed, total_skipped
    );
    let registered_fixtures = suite.fixtures.len();
    if ran < registered_fixtures {
        rbt::rbtcel_info_now!(
            "{} reached of {} registered", ran, registered_fixtures
        );
        for fixture in &suite.fixtures[ran..] {
            rbt::rbtcel_info_now!("UNREACHED: {}", fixture.name);
        }
    }

    if total_failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn rbtm_list_suites() {
    rbt::rbtcel_info_now!("available suites:");
    for s in RBTCEA_SUITES {
        rbt::rbtcel_info_now!("  {}", s.name);
    }
}

// ── Single-case runner ───────────────────────────────────────

fn rbtm_run_single(args: &[String]) -> ExitCode {
    let fixture = match args.first() {
        Some(f) => f,
        None => {
            rbt::rbtcel_error_now!(
                "rbtd single: usage: rbtd single <fixture> [case]\n\
                 omit case to list all cases for the fixture"
            );
            rbtm_list_fixtures();
            return ExitCode::FAILURE;
        }
    };

    if !RBTCEA_FIXTURES.iter().any(|f| f.name == *fixture) {
        rbt::rbtcel_error_now!("rbtd single: unknown fixture '{}'", fixture);
        rbtm_list_fixtures();
        return ExitCode::FAILURE;
    }

    // Set up execution context early — needed for charge check and case execution
    let project_root = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => rbt::rbtcel_fatal_now!("rbtd: cannot determine working directory: {}", e),
    };

    let roots = match rbtm_allocate_roots() {
        Ok(r) => r,
        Err(msg) => rbt::rbtcel_fatal_now!("{}", msg),
    };

    // Context is required for every case execution path (rbtccc_with_ctx).
    // Fixtures with a setup hook (crucible charge) additionally verify their
    // crucible is charged externally — single-case mode never charges. The
    // charge probe below runs BEFORE rbtccc_set_context, so it is deliberately
    // outside the fixture's census: it is harness machinery, not fixture
    // behavior.
    let mut ctx = rbtcei_Context::new(
        &project_root,
        fixture,
        &roots.burv_temp_root,
        &roots.burv_output_root,
    );

    let fixture_def = match rbtcea_lookup_fixture(fixture) {
        Some(f) => f,
        None => rbt::rbtcel_fatal_now!(
            "rbtd single: fixture '{}' has no registered Fixture",
            fixture
        ),
    };

    if fixture_def.setup.is_some() {
        match rbtcei_invoke_global(
            &mut ctx,
            RBTGEC_CRUCIBLE_ACTIVE,
            &[fixture],
            &[],
        ) {
            Ok(r) if r.exit_code == 0 => {}
            _ => rbt::rbtcel_fatal_now!(
                "rbtd single: crucible not charged for '{}'\n\
                 charge first: tt/rbw-cC.Charge.{}.sh",
                fixture, fixture
            ),
        }
    }

    rbtccc_set_context(ctx);

    let cases = fixture_def.cases;

    // No case argument — list all cases
    let case_name = match args.get(1) {
        None => {
            rbt::rbtcel_info_now!(
                "fixture '{}' declared tariff: [{}]",
                fixture, rbtcee_tariff_declared(&fixture_def.tariff)
            );
            rbtcee_list_cases(cases);
            return ExitCode::SUCCESS;
        }
        Some(n) => n,
    };

    // Find the case
    let case = match rbtcee_find_case(cases, case_name) {
        Some(c) => c,
        None => {
            rbt::rbtcel_error_now!(
                "rbtd single: case '{}' not found in fixture '{}'",
                case_name, fixture
            );
            rbtcee_list_cases(cases);
            return ExitCode::FAILURE;
        }
    };

    let colors = rbtcee_detect_colors();
    let result = match rbtcee_run_single_case(case, &colors, &roots.trace_root) {
        Ok(r) => r,
        Err(msg) => rbt::rbtcel_fatal_now!("rbtd: case execution error: {}", msg),
    };

    rbtcee_print_summary(&result, &colors);

    if result.failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn rbtm_list_fixtures() {
    rbt::rbtcel_info_now!("available fixtures:");
    let mut eta_min = 0u64;
    let mut eta_max = 0u64;
    for f in RBTCEA_FIXTURES {
        rbt::rbtcel_info_now!("  {:<28} [{}]", f.name, rbtcee_tariff_declared(&f.tariff));
        eta_min += f.tariff.min_secs.unwrap_or(0);
        eta_max += f.tariff.max_secs.unwrap_or(0);
    }
    // Declared-cost ETA envelope: the sum of declared min/max wall-clock across
    // the roster. Undeclared bounds contribute zero, so this is a lower bound on
    // the envelope, not a promise — it grows as declarations are seeded.
    rbt::rbtcel_info_now!(
        "declared-cost ETA envelope (sum of declared bounds): min {}s, max {}s",
        eta_min, eta_max
    );
}
