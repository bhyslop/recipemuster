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
// RBTD Theurge — crucible test orchestrator entry point
//
// Lifecycle: verify manifest → charge crucible → run cases → quench crucible

use std::process::ExitCode;

use rbtd::rbtdrc_crucible::{
    rbtdrc_needs_readiness_delay, rbtdrc_sections_for_nameplate, rbtdrc_set_context,
    rbtdrc_take_context, RBTDRC_SERVICE_READINESS_DELAY_SECS,
};
use rbtd::rbtdre_engine::{rbtdre_detect_colors, rbtdre_print_summary, rbtdre_run_sections};
use rbtd::rbtdri_invocation::{rbtdri_Context, rbtdri_invoke};
use rbtd::rbtdrm_manifest::{rbtdrm_verify, RBTDRM_COLOPHON_CHARGE, RBTDRM_COLOPHON_QUENCH};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    let manifest = match args.get(1) {
        Some(m) => m,
        None => {
            eprintln!(
                "rbtd: usage: rbtd <manifest> <nameplate>\n\
                 theurge must be launched via tabtarget (e.g. tt/rbtd-r.Run.tadmor.sh)"
            );
            return ExitCode::FAILURE;
        }
    };

    let nameplate = match args.get(2) {
        Some(n) => n,
        None => {
            eprintln!("rbtd: no nameplate argument — which crucible to test?");
            return ExitCode::FAILURE;
        }
    };

    if let Err(msg) = rbtdrm_verify(manifest) {
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
    let mut ctx = rbtdri_Context::new(&project_root, nameplate, &burv_root);

    // ── Charge crucible ──────────────────────────────────────
    eprintln!("\nCharging crucible for nameplate '{}'...", nameplate);
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

    // ── Service readiness delay (srjcl/pluml need services to start) ──
    if rbtdrc_needs_readiness_delay(nameplate) {
        eprintln!(
            "Waiting {}s for service readiness...",
            RBTDRC_SERVICE_READINESS_DELAY_SECS
        );
        std::thread::sleep(std::time::Duration::from_secs(
            RBTDRC_SERVICE_READINESS_DELAY_SECS,
        ));
    }

    // ── Run crucible cases ───────────────────────────────────
    let sections = rbtdrc_sections_for_nameplate(nameplate);
    rbtdrc_set_context(ctx);

    let colors = rbtdre_detect_colors();
    let run_result = rbtdre_run_sections(sections, &colors, false, &root_temp);

    // ── Quench crucible (unconditional) ──────────────────────
    let mut ctx = rbtdrc_take_context();
    eprintln!("\nQuenching crucible...");
    match rbtdri_invoke(&mut ctx, RBTDRM_COLOPHON_QUENCH, &[]) {
        Ok(r) if r.exit_code == 0 => eprintln!("Crucible quenched"),
        Ok(r) => eprintln!("Warning: quench exited {}", r.exit_code),
        Err(e) => eprintln!("Warning: quench invocation failed: {}", e),
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
