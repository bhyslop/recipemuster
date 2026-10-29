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
// RBTDRF — handbook-render reveille-tier fixture for theurge
//
// Exercises every handbook display tabtarget and reports per-case pass/fail.
// Each case invokes a handbook colophon with no arguments and asserts exit 0.

use std::path::Path;

use crate::rbtcee_case;
use crate::rbtcee_engine::{rbtcee_Case, rbtcee_Disposition, rbtcee_Fixture, rbtcee_Tariff, rbtcee_Verdict};
use crate::rbtcei_invocation::{rbtcei_find_tabtarget_global, rbtdri_rooted_tabtarget_command};
use crate::rbtgec_consts::{
    RBTGEC_ONBOARD_CRASH_COURSE, RBTGEC_ONBOARD_DIR_FIRST_BUILD,
    RBTGEC_ONBOARD_FIRST_CRUCIBLE,
    RBTGEC_ONBOARD_PAYOR_HB, RBTGEC_ONBOARD_START_HERE,
    RBTGEC_PAYOR_ESTABLISH,
    RBTGEC_QUOTA_BUILD,
};
use crate::rbtcem_manifest::RBTCEM_FIXTURE_HANDBOOK_RENDER;

// ── Helper ───────────────────────────────────────────────────

/// Invoke a handbook tabtarget with no arguments and return Pass iff exit 0.
/// Writes stdout/stderr traces to the per-case directory for diagnostic review.
fn rbtchh_hb_render(dir: &Path, colophon: &str, label: &str) -> rbtcee_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    let tt = match rbtcei_find_tabtarget_global(&root, colophon) {
        Ok(p) => p,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };

    let mut cmd = match rbtdri_rooted_tabtarget_command(&tt, &dir.join(label)) {
        Ok(c) => c,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    let output = match cmd.current_dir(&root).output() {
        Ok(o) => o,
        Err(e) => {
            return rbtcee_Verdict::Fail(format!(
                "{}: failed to run {}: {}",
                label,
                tt.display(),
                e
            ));
        }
    };

    let code = output.status.code().unwrap_or(-1);
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &output.stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &output.stderr);

    if code != 0 {
        return rbtcee_Verdict::Fail(format!(
            "{}: {} exited {} — {}",
            label,
            colophon,
            code,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    rbtcee_Verdict::Pass
}

// ── Onboarding cases (8) ────────────────────────────────────

fn rbtchh_hb_onboard_start_here(dir: &Path) -> rbtcee_Verdict {
    rbtchh_hb_render(dir, RBTGEC_ONBOARD_START_HERE, "onboard-start-here")
}

fn rbtchh_hb_onboard_crash_course(dir: &Path) -> rbtcee_Verdict {
    rbtchh_hb_render(dir, RBTGEC_ONBOARD_CRASH_COURSE, "onboard-crash-course")
}

fn rbtchh_hb_onboard_first_crucible(dir: &Path) -> rbtcee_Verdict {
    rbtchh_hb_render(dir, RBTGEC_ONBOARD_FIRST_CRUCIBLE, "onboard-first-crucible")
}

fn rbtchh_hb_onboard_dir_first_build(dir: &Path) -> rbtcee_Verdict {
    rbtchh_hb_render(dir, RBTGEC_ONBOARD_DIR_FIRST_BUILD, "onboard-dir-first-build")
}

fn rbtchh_hb_onboard_payor_hb(dir: &Path) -> rbtcee_Verdict {
    rbtchh_hb_render(dir, RBTGEC_ONBOARD_PAYOR_HB, "onboard-payor-hb")
}

// ── Payor cases (2) ─────────────────────────────────────────

fn rbtchh_hb_payor_establish(dir: &Path) -> rbtcee_Verdict {
    rbtchh_hb_render(dir, RBTGEC_PAYOR_ESTABLISH, "payor-establish")
}

fn rbtchh_hb_quota_build(dir: &Path) -> rbtcee_Verdict {
    rbtchh_hb_render(dir, RBTGEC_QUOTA_BUILD, "quota-build")
}

// ── Case array ──────────────────────────────────────────────

pub static RBTCHH_CASES_HANDBOOK_RENDER: &[rbtcee_Case] = &[
    rbtcee_case!(rbtchh_hb_onboard_start_here),
    rbtcee_case!(rbtchh_hb_onboard_crash_course),
    rbtcee_case!(rbtchh_hb_onboard_first_crucible),
    rbtcee_case!(rbtchh_hb_onboard_dir_first_build),
    rbtcee_case!(rbtchh_hb_onboard_payor_hb),
    rbtcee_case!(rbtchh_hb_payor_establish),
    rbtcee_case!(rbtchh_hb_quota_build),
];

// ── Fixture static ───────────────────────────────────────────

pub static RBTCHH_FIXTURE_HANDBOOK_RENDER: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_HANDBOOK_RENDER,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHH_CASES_HANDBOOK_RENDER,
    credless: true,
    tariff: rbtcee_Tariff { min_secs: None, max_secs: None, invocations: Some(7) },
};
