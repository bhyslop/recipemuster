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
// RBTD Theurge — test orchestrator entry point
//
// Subcommands:
//   rbtd <manifest> <fixture>
//     Suite runner — verify manifest, charge, run all cases, quench.
//   rbtd single <manifest> <fixture> [case]
//     Single-case runner — no charge/quench. List cases or run one.

use std::process::ExitCode;

use rbtd::rbtdrc_crucible::{
    rbtdrc_needs_charge, rbtdrc_needs_readiness_delay, rbtdrc_sections_for_fixture,
    rbtdrc_set_context, rbtdrc_take_context, RBTDRC_SERVICE_READINESS_DELAY_SECS,
};
use rbtd::rbtdre_engine::{
    rbtdre_detect_colors, rbtdre_find_case, rbtdre_list_cases, rbtdre_print_summary,
    rbtdre_run_sections, rbtdre_run_single_case,
};
use rbtd::rbtdri_invocation::{rbtdri_Context, rbtdri_invoke};
use rbtd::rbtdrm_manifest::{rbtdrm_verify, RBTDRM_COLOPHON_CHARGE, RBTDRM_COLOPHON_QUENCH};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("single") {
        return run_single(&args[2..]);
    }

    run_suite(&args[1..])
}

// ── Suite runner ─────────────────────────────────────────────

fn run_suite(args: &[String]) -> ExitCode {
    let manifest = match args.first() {
        Some(m) => m,
        None => {
            eprintln!(
                "rbtd: usage: rbtd <manifest> <fixture>\n\
                 theurge must be launched via tabtarget (e.g. tt/rbtd-r.Run.tadmor.sh)"
            );
            return ExitCode::FAILURE;
        }
    };

    let fixture = match args.get(1) {
        Some(n) => n,
        None => {
            eprintln!("rbtd: no fixture argument — which test fixture to run?");
            return ExitCode::FAILURE;
        }
    };

    if let Err(msg) = rbtdrm_verify(manifest, fixture) {
        eprintln!("{}", msg);
        return ExitCode::FAILURE;
    }

    let project_root = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("rbtd: cannot determine working directory: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let root_temp = std::env::temp_dir().join(format!("rbtd-{}", std::process::id()));
    if let Err(e) = std::fs::create_dir_all(&root_temp) {
        eprintln!("rbtd: failed to create temp dir: {}", e);
        return ExitCode::FAILURE;
    }

    let burv_root = root_temp.join("burv");
    let mut ctx = rbtdri_Context::new(&project_root, fixture, &burv_root);
    let needs_charge = rbtdrc_needs_charge(fixture);

    // ── Charge crucible (crucible fixtures only) ─────────────
    if needs_charge {
        eprintln!("\nCharging crucible for nameplate '{}'...", fixture);
        match rbtdri_invoke(&mut ctx, RBTDRM_COLOPHON_CHARGE, &[]) {
            Ok(r) if r.exit_code == 0 => {
                eprintln!("Crucible charged");
            }
            Ok(r) => {
                eprintln!(
                    "Charge failed (exit {})\n{}",
                    r.exit_code, r.stderr
                );
                return ExitCode::FAILURE;
            }
            Err(e) => {
                eprintln!("Charge invocation failed: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Service readiness delay (srjcl/pluml need services to start)
        if rbtdrc_needs_readiness_delay(fixture) {
            eprintln!(
                "Waiting {}s for service readiness...",
                RBTDRC_SERVICE_READINESS_DELAY_SECS
            );
            std::thread::sleep(std::time::Duration::from_secs(
                RBTDRC_SERVICE_READINESS_DELAY_SECS,
            ));
        }
    }

    // ── Run cases ────────────────────────────────────────────
    let sections = rbtdrc_sections_for_fixture(fixture);
    rbtdrc_set_context(ctx);

    let colors = rbtdre_detect_colors();
    let run_result = rbtdre_run_sections(sections, &colors, false, &root_temp);

    // ── Quench crucible (crucible fixtures only, unconditional) ──
    let mut ctx = rbtdrc_take_context();
    if needs_charge {
        eprintln!("\nQuenching crucible...");
        match rbtdri_invoke(&mut ctx, RBTDRM_COLOPHON_QUENCH, &[]) {
            Ok(r) if r.exit_code == 0 => eprintln!("Crucible quenched"),
            Ok(r) => eprintln!("Warning: quench exited {}", r.exit_code),
            Err(e) => eprintln!("Warning: quench invocation failed: {}", e),
        }
    }

    // ── Report ───────────────────────────────────────────────
    let result = match run_result {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("rbtd: case execution error: {}", msg);
            return ExitCode::FAILURE;
        }
    };

    rbtdre_print_summary(&result, &colors);

    if result.failed > 0 {
        eprintln!("rbtd: {} case(s) failed", result.failed);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

// ── Single-case runner ───────────────────────────────────────

fn run_single(args: &[String]) -> ExitCode {
    let manifest = match args.first() {
        Some(m) => m,
        None => {
            eprintln!(
                "rbtd single: usage: rbtd single <manifest> <fixture> [case]\n\
                 omit case to list all cases for the fixture"
            );
            return ExitCode::FAILURE;
        }
    };

    let fixture = match args.get(1) {
        Some(f) => f,
        None => {
            eprintln!("rbtd single: no fixture argument");
            return ExitCode::FAILURE;
        }
    };

    if let Err(msg) = rbtdrm_verify(manifest, fixture) {
        eprintln!("{}", msg);
        return ExitCode::FAILURE;
    }

    let sections = rbtdrc_sections_for_fixture(fixture);

    // No case argument — list all cases
    let case_name = match args.get(2) {
        None => {
            rbtdre_list_cases(sections);
            return ExitCode::SUCCESS;
        }
        Some(n) => n,
    };

    // Find the case
    let case = match rbtdre_find_case(sections, case_name) {
        Some(c) => c,
        None => {
            eprintln!(
                "rbtd single: case '{}' not found in fixture '{}'",
                case_name, fixture
            );
            rbtdre_list_cases(sections);
            return ExitCode::FAILURE;
        }
    };

    // Set up execution context
    let project_root = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("rbtd: cannot determine working directory: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let root_temp = std::env::temp_dir().join(format!("rbtd-{}", std::process::id()));
    if let Err(e) = std::fs::create_dir_all(&root_temp) {
        eprintln!("rbtd: failed to create temp dir: {}", e);
        return ExitCode::FAILURE;
    }

    // Crucible cases need invocation context (but no charge/quench)
    if rbtdrc_needs_charge(fixture) {
        let burv_root = root_temp.join("burv");
        let ctx = rbtdri_Context::new(&project_root, fixture, &burv_root);
        rbtdrc_set_context(ctx);
    }

    let colors = rbtdre_detect_colors();
    let result = match rbtdre_run_single_case(case, &colors, &root_temp) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("rbtd: case execution error: {}", msg);
            return ExitCode::FAILURE;
        }
    };

    rbtdre_print_summary(&result, &colors);

    if result.failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
