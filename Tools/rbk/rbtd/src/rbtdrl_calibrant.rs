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
// RBTDRL — calibrant fixtures: synthetic test inputs with deterministic
// verdicts that exercise the operator-facing surface of the theurge engine.
// Internal framework-test plumbing; not end-user-facing.
//
// Four fixtures registered through rbtdrm_manifest.rs and dispatched via
// rbtdrc_crucible.rs:
//
//   calibrant-verdicts      Independent       4 cases   verdict-path coverage
//   calibrant-fail-fast     Independent       4 cases   intra/inter-section fail-fast
//   calibrant-progressing   StateProgressing  2 cases   probe Ok/Err dispatch
//   calibrant-sentinel      Independent       1 case    suite-fail-fast pivot
//
// All four declare empty rbtdrm_required_colophons — calibrant cases never
// shell out to bash tabtargets, so the manifest-coupling check is vacuous.
//
// A bash blackbox testbench consumes these fixtures and asserts engine-output
// contracts (exit codes, stderr format, fail-fast semantics, disposition ×
// keep-going policy gate).

use std::path::Path;

use crate::case;
use crate::rbtdrb_probe::{rbtdrb_assert, rbtdrb_Probe};
use crate::rbtdre_engine::{rbtdre_Section, rbtdre_Verdict};

/// Sentinel filename written into a case's temp dir to mark execution. The
/// blackbox driver asserts presence/absence by globbing under the
/// BURV_TEMP_ROOT_DIR it set for the rbtd invocation. I/O failure during
/// sentinel write fails the case visibly — silent failure would hide bugs in
/// the engine's case-dir contract.
pub(crate) const RBTDRL_SENTINEL_FILE: &str = "ran.sentinel";

/// Case-written output filename. Distinct from the engine's auto-written
/// trace.txt so the blackbox driver can verify both the engine's per-case
/// trace contract and the case's own write contract.
pub(crate) const RBTDRL_OUTPUT_FILE: &str = "output.txt";

fn rbtdrl_write_sentinel(dir: &Path) -> Result<(), rbtdre_Verdict> {
    std::fs::write(dir.join(RBTDRL_SENTINEL_FILE), "")
        .map_err(|e| rbtdre_Verdict::Fail(format!("sentinel write failed: {}", e)))
}

// ── calibrant-verdicts ──────────────────────────────────────

fn rbtdrl_verdicts_pass(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Pass
}

fn rbtdrl_verdicts_fail(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Fail("calibrant deterministic fail verdict".to_string())
}

fn rbtdrl_verdicts_skip(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Skip("calibrant deterministic skip verdict".to_string())
}

fn rbtdrl_verdicts_pass_with_output(dir: &Path) -> rbtdre_Verdict {
    if let Err(e) = std::fs::write(
        dir.join(RBTDRL_OUTPUT_FILE),
        "calibrant case-written output\n",
    ) {
        return rbtdre_Verdict::Fail(format!("output write failed: {}", e));
    }
    rbtdre_Verdict::Pass
}

pub static RBTDRL_SECTIONS_VERDICTS: &[rbtdre_Section] = &[rbtdre_Section {
    name: "calibrant-verdicts",
    cases: &[
        case!(rbtdrl_verdicts_pass),
        case!(rbtdrl_verdicts_fail),
        case!(rbtdrl_verdicts_skip),
        case!(rbtdrl_verdicts_pass_with_output),
    ],
}];

// ── calibrant-fail-fast ─────────────────────────────────────

fn rbtdrl_failfast_pass(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Pass
}

fn rbtdrl_failfast_fail(_dir: &Path) -> rbtdre_Verdict {
    rbtdre_Verdict::Fail("calibrant intra-section fail trigger".to_string())
}

/// Section-A trailing case. Under default fail-fast it never runs and its
/// sentinel must be absent; under keep-going it runs and writes the sentinel.
fn rbtdrl_failfast_not_reached_intra(dir: &Path) -> rbtdre_Verdict {
    if let Err(v) = rbtdrl_write_sentinel(dir) {
        return v;
    }
    rbtdre_Verdict::Pass
}

/// Section-B sole case. Under default fail-fast no §B case runs; under
/// keep-going it runs and writes the sentinel. Distinguishes inter-section
/// fail-fast from the intra-section variant.
fn rbtdrl_failfast_not_reached_inter(dir: &Path) -> rbtdre_Verdict {
    if let Err(v) = rbtdrl_write_sentinel(dir) {
        return v;
    }
    rbtdre_Verdict::Pass
}

pub static RBTDRL_SECTIONS_FAIL_FAST: &[rbtdre_Section] = &[
    rbtdre_Section {
        name: "fail-fast-intra-section",
        cases: &[
            case!(rbtdrl_failfast_pass),
            case!(rbtdrl_failfast_fail),
            case!(rbtdrl_failfast_not_reached_intra),
        ],
    },
    rbtdre_Section {
        name: "fail-fast-inter-section",
        cases: &[case!(rbtdrl_failfast_not_reached_inter)],
    },
];

// ── calibrant-progressing ───────────────────────────────────

/// Probe-Ok mechanism: deterministic Ok return. No env-var or file coupling
/// — a single fixture run exercises the case-body-runs-after-Probe-Ok path.
fn rbtdrl_probe_ok() -> Result<(), String> {
    Ok(())
}

/// Probe-Err mechanism: deterministic Err return. Case body never runs, so a
/// single fixture run exercises the Fail-via-Probe-Err path. Required by the
/// blackbox driver to verify rbtdrb_assert's "precondition '%s' not met:" +
/// "remediation:" stderr format.
fn rbtdrl_probe_err() -> Result<(), String> {
    Err("calibrant deterministic probe failure".to_string())
}

fn rbtdrl_progressing_probe_ok(_dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "calibrant deterministic ok",
        check: rbtdrl_probe_ok,
        remediation: "n/a — deterministic probe always Ok",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdre_Verdict::Pass
}

fn rbtdrl_progressing_probe_err(_dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "calibrant deterministic err",
        check: rbtdrl_probe_err,
        remediation: "n/a — deterministic probe always Err for engine surface verification",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdre_Verdict::Fail("calibrant probe-err case body executed unexpectedly".to_string())
}

pub static RBTDRL_SECTIONS_PROGRESSING: &[rbtdre_Section] = &[rbtdre_Section {
    name: "calibrant-progressing",
    cases: &[
        case!(rbtdrl_progressing_probe_ok),
        case!(rbtdrl_progressing_probe_err),
    ],
}];

// ── calibrant-sentinel ──────────────────────────────────────

/// Single-case Independent fixture used as a suite-level fail-fast pivot:
/// place this fixture after a failing fixture in the calibrant suite and
/// assert the sentinel is absent.
fn rbtdrl_sentinel_marks(dir: &Path) -> rbtdre_Verdict {
    if let Err(v) = rbtdrl_write_sentinel(dir) {
        return v;
    }
    rbtdre_Verdict::Pass
}

pub static RBTDRL_SECTIONS_SENTINEL: &[rbtdre_Section] = &[rbtdre_Section {
    name: "calibrant-sentinel",
    cases: &[case!(rbtdrl_sentinel_marks)],
}];
