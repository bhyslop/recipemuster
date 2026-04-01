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

use std::process::ExitCode;

use rbtd::rbtdrd_dummy;
use rbtd::rbtdre_engine::{rbtdre_detect_colors, rbtdre_print_summary, rbtdre_run_sections};
use rbtd::rbtdrm_manifest::rbtdrm_verify;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    let manifest = match args.get(1) {
        Some(m) => m,
        None => {
            eprintln!(
                "rbtd: no colophon manifest argument — theurge must be launched via tabtarget"
            );
            return ExitCode::FAILURE;
        }
    };

    if let Err(msg) = rbtdrm_verify(manifest) {
        eprintln!("{}", msg);
        return ExitCode::FAILURE;
    }

    let root_temp = std::env::temp_dir().join(format!("rbtd-{}", std::process::id()));
    if let Err(e) = std::fs::create_dir_all(&root_temp) {
        eprintln!("rbtd: failed to create temp dir: {}", e);
        return ExitCode::FAILURE;
    }

    let colors = rbtdre_detect_colors();
    let result = match rbtdre_run_sections(
        rbtdrd_dummy::RBTDRD_SECTIONS,
        &colors,
        false,
        &root_temp,
    ) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("{}", msg);
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
