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
// RBTDRF — handbook-render fast-tier fixture for theurge
//
// Exercises every handbook display tabtarget and reports per-case pass/fail.
// Each case invokes a handbook colophon with no arguments and asserts exit 0.
//
// Scope note: `rbw-HWdw` (RBZ_HW_DOCKER_WSL_NATIVE) uses the param1 channel
// and is deferred to ₢A-AAS (handbook-render-param1-coverage) in ₣A- — the
// fixture's uniform "invoke and check exit 0" pattern cannot supply the
// required WSL-target argument.

use std::path::Path;
use std::process::Command;

use crate::case;
use crate::rbtdre_engine::{rbtdre_Section, rbtdre_Verdict};
use crate::rbtdri_invocation::rbtdri_find_tabtarget_global;
use crate::rbtdrm_manifest::{
    RBTDRM_COLOPHON_HANDBOOK_TOP, RBTDRM_COLOPHON_HANDBOOK_WINDOWS,
    RBTDRM_COLOPHON_HW_DOCKER_CONTEXT, RBTDRM_COLOPHON_HW_DOCKER_DESKTOP,
    RBTDRM_COLOPHON_ONBOARD_CRASH_COURSE, RBTDRM_COLOPHON_ONBOARD_CRED_DIRECTOR,
    RBTDRM_COLOPHON_ONBOARD_CRED_RETRIEVER, RBTDRM_COLOPHON_ONBOARD_DIR_FIRST_BUILD,
    RBTDRM_COLOPHON_ONBOARD_FIRST_CRUCIBLE, RBTDRM_COLOPHON_ONBOARD_GOVERNOR_HB,
    RBTDRM_COLOPHON_ONBOARD_PAYOR_HB, RBTDRM_COLOPHON_ONBOARD_START_HERE,
    RBTDRM_COLOPHON_PAYOR_ESTABLISH, RBTDRM_COLOPHON_PAYOR_REFRESH,
    RBTDRM_COLOPHON_QUOTA_BUILD,
};

// ── Helper ───────────────────────────────────────────────────

/// Invoke a handbook tabtarget with no arguments and return Pass iff exit 0.
/// Writes stdout/stderr traces to the per-case directory for diagnostic review.
fn rbtdrf_hb_render(dir: &Path, colophon: &str, label: &str) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    let tt = match rbtdri_find_tabtarget_global(&root, colophon) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(e),
    };

    let output = match Command::new(&tt).current_dir(&root).output() {
        Ok(o) => o,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!(
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
        return rbtdre_Verdict::Fail(format!(
            "{}: {} exited {} — {}",
            label,
            colophon,
            code,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    rbtdre_Verdict::Pass
}

// ── Onboarding cases (8) ────────────────────────────────────

fn rbtdrf_hb_onboard_start_here(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_ONBOARD_START_HERE, "onboard-start-here")
}

fn rbtdrf_hb_onboard_crash_course(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_ONBOARD_CRASH_COURSE, "onboard-crash-course")
}

fn rbtdrf_hb_onboard_cred_retriever(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_ONBOARD_CRED_RETRIEVER, "onboard-cred-retriever")
}

fn rbtdrf_hb_onboard_cred_director(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_ONBOARD_CRED_DIRECTOR, "onboard-cred-director")
}

fn rbtdrf_hb_onboard_first_crucible(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_ONBOARD_FIRST_CRUCIBLE, "onboard-first-crucible")
}

fn rbtdrf_hb_onboard_dir_first_build(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_ONBOARD_DIR_FIRST_BUILD, "onboard-dir-first-build")
}

fn rbtdrf_hb_onboard_payor_hb(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_ONBOARD_PAYOR_HB, "onboard-payor-hb")
}

fn rbtdrf_hb_onboard_governor_hb(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_ONBOARD_GOVERNOR_HB, "onboard-governor-hb")
}

// ── Windows cases (4 of 5 — rbw-HWdw deferred to ₢A-AAS) ────

fn rbtdrf_hb_handbook_top(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_HANDBOOK_TOP, "handbook-top")
}

fn rbtdrf_hb_handbook_windows(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_HANDBOOK_WINDOWS, "handbook-windows")
}

fn rbtdrf_hb_hw_docker_desktop(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_HW_DOCKER_DESKTOP, "hw-docker-desktop")
}

fn rbtdrf_hb_hw_docker_context(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_HW_DOCKER_CONTEXT, "hw-docker-context")
}

// ── Payor cases (3) ─────────────────────────────────────────

fn rbtdrf_hb_payor_establish(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_PAYOR_ESTABLISH, "payor-establish")
}

fn rbtdrf_hb_payor_refresh(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_PAYOR_REFRESH, "payor-refresh")
}

fn rbtdrf_hb_quota_build(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_hb_render(dir, RBTDRM_COLOPHON_QUOTA_BUILD, "quota-build")
}

// ── Section array ───────────────────────────────────────────

pub static RBTDRF_SECTIONS_HANDBOOK_RENDER: &[rbtdre_Section] = &[
    rbtdre_Section {
        name: "hb-onboarding",
        cases: &[
            case!(rbtdrf_hb_onboard_start_here),
            case!(rbtdrf_hb_onboard_crash_course),
            case!(rbtdrf_hb_onboard_cred_retriever),
            case!(rbtdrf_hb_onboard_cred_director),
            case!(rbtdrf_hb_onboard_first_crucible),
            case!(rbtdrf_hb_onboard_dir_first_build),
            case!(rbtdrf_hb_onboard_payor_hb),
            case!(rbtdrf_hb_onboard_governor_hb),
        ],
    },
    rbtdre_Section {
        name: "hb-windows",
        cases: &[
            case!(rbtdrf_hb_handbook_top),
            case!(rbtdrf_hb_handbook_windows),
            case!(rbtdrf_hb_hw_docker_desktop),
            case!(rbtdrf_hb_hw_docker_context),
        ],
    },
    rbtdre_Section {
        name: "hb-payor",
        cases: &[
            case!(rbtdrf_hb_payor_establish),
            case!(rbtdrf_hb_payor_refresh),
            case!(rbtdrf_hb_quota_build),
        ],
    },
];
