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
// RBTDRL — calibrant fixtures: synthetic test inputs with deterministic
// verdicts that exercise the operator-facing surface of the theurge engine.
// Internal framework-test plumbing; not end-user-facing.
//
// Eight fixtures registered through rbtcem_manifest.rs and dispatched via
// rbtccc_crucible.rs:
//
//   calibrant-verdicts             Independent  4 cases  verdict-path coverage
//   calibrant-fail-fast            Independent  3 cases  fail-fast halts subsequent cases
//   calibrant-progressing  StateProgressing     2 cases  probe Ok/Err dispatch
//   calibrant-sentinel             Independent  1 case   suite-fail-fast pivot
//   calibrant-coverage-aligned     Independent  1 case   declared+invoked census colophon
//   calibrant-coverage-undeclared  Independent  1 case   invoked-but-undeclared -> positive-check FAIL
//   calibrant-coverage-unused      Independent  1 case   declared-but-unused -> negative-check FAIL
//   calibrant-tariff-drift         Independent  1 case   skip + unmeetable count -> shortfall FAIL
//
// The first four and calibrant-tariff-drift declare empty
// rbtcem_required_colophons — their cases never shell out to bash tabtargets,
// so the manifest-coupling check is vacuous. The three coverage fixtures invoke
// (or deliberately don't invoke) the synthetic RBTGEC_THEURGE_NIHIL colophon to
// exercise the census enforcement itself; their manifest declarations are what
// differ between them. calibrant-tariff-drift's needle is instead its TARIFF —
// a declared invocation count no case can meet.
//
// The touchstone surface fixture (rbtcct_touchstone.rs) consumes these
// fixtures as child rbtd runs through the real tabtarget chain and asserts
// engine-output contracts (exit codes, diagnostic format, fail-fast
// semantics, disposition × keep-going policy gate, census diagnostics).
// calibrant-fail-fast and calibrant-sentinel additionally compose the
// registered `calibrant` suite — touchstone's suite-abort subject; the rest
// stay roster-only.

use std::path::Path;

use crate::rbtcee_case;
use crate::rbtgec_consts::RBTGEC_THEURGE_NIHIL;
use crate::rbtccp_probe::{rbtccp_assert, rbtccp_Probe};
use crate::rbtccc_crucible::rbtccc_with_ctx;
use crate::rbtcee_engine::{rbtcee_Case, rbtcee_Disposition, rbtcee_Fixture, rbtcee_Tariff, rbtcee_Verdict};
use crate::rbtcei_invocation::rbtcei_invoke_global;
use crate::rbtcem_manifest::{
    RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED, RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED,
    RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED, RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST,
    RBTCEM_FIXTURE_CALIBRANT_PROGRESSING, RBTCEM_FIXTURE_CALIBRANT_SENTINEL,
    RBTCEM_FIXTURE_CALIBRANT_TARIFF_DRIFT, RBTCEM_FIXTURE_CALIBRANT_VERDICTS,
};

/// Sentinel filename written into a case's temp dir to mark execution. The
/// touchstone fixture asserts presence/absence under the BURV_TEMP_ROOT_DIR
/// it set for the child rbtd invocation. I/O failure during sentinel write
/// fails the case visibly — silent failure would hide bugs in the engine's
/// case-dir contract.
pub(crate) const RBTCHL_SENTINEL_FILE: &str = "ran.sentinel";

/// Case-written output filename. Distinct from the engine's auto-written
/// trace.txt so the touchstone fixture can verify both the engine's per-case
/// trace contract and the case's own write contract.
pub(crate) const RBTCHL_OUTPUT_FILE: &str = "output.txt";

fn rbtchl_write_sentinel(dir: &Path) -> Result<(), rbtcee_Verdict> {
    std::fs::write(dir.join(RBTCHL_SENTINEL_FILE), "")
        .map_err(|e| rbtcee_Verdict::Fail(format!("sentinel write failed: {}", e)))
}

// ── calibrant-verdicts ──────────────────────────────────────

fn rbtchl_verdicts_pass(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Pass
}

fn rbtchl_verdicts_fail(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Fail("calibrant deterministic fail verdict".to_string())
}

fn rbtchl_verdicts_skip(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Skip("calibrant deterministic skip verdict".to_string())
}

fn rbtchl_verdicts_pass_with_output(dir: &Path) -> rbtcee_Verdict {
    if let Err(e) = std::fs::write(
        dir.join(RBTCHL_OUTPUT_FILE),
        "calibrant case-written output\n",
    ) {
        return rbtcee_Verdict::Fail(format!("output write failed: {}", e));
    }
    rbtcee_Verdict::Pass
}

pub static RBTCHL_CASES_VERDICTS: &[rbtcee_Case] = &[
    rbtcee_case!(rbtchl_verdicts_pass),
    rbtcee_case!(rbtchl_verdicts_fail),
    rbtcee_case!(rbtchl_verdicts_skip),
    rbtcee_case!(rbtchl_verdicts_pass_with_output),
];

// ── calibrant-fail-fast ─────────────────────────────────────

fn rbtchl_failfast_pass(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Pass
}

fn rbtchl_failfast_fail(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Fail("calibrant fail-fast fail trigger".to_string())
}

/// Trailing case after the deterministic fail. Under default fail-fast it
/// never runs and its sentinel must be absent; under keep-going it runs and
/// writes the sentinel.
fn rbtchl_failfast_not_reached(dir: &Path) -> rbtcee_Verdict {
    if let Err(v) = rbtchl_write_sentinel(dir) {
        return v;
    }
    rbtcee_Verdict::Pass
}

pub static RBTCHL_CASES_FAIL_FAST: &[rbtcee_Case] = &[
    rbtcee_case!(rbtchl_failfast_pass),
    rbtcee_case!(rbtchl_failfast_fail),
    rbtcee_case!(rbtchl_failfast_not_reached),
];

// ── calibrant-progressing ───────────────────────────────────

/// Probe-Ok mechanism: deterministic Ok return. No env-var or file coupling
/// — a single fixture run exercises the case-body-runs-after-Probe-Ok path.
fn rbtchl_probe_ok() -> Result<(), String> {
    Ok(())
}

/// Probe-Err mechanism: deterministic Err return. Case body never runs, so a
/// single fixture run exercises the Fail-via-Probe-Err path. Required by the
/// touchstone fixture to verify rbtccp_assert's "precondition '%s' not met:" +
/// "remediation:" diagnostic format.
fn rbtchl_probe_err() -> Result<(), String> {
    Err("calibrant deterministic probe failure".to_string())
}

fn rbtchl_progressing_probe_ok(_dir: &Path) -> rbtcee_Verdict {
    let probe = rbtccp_Probe {
        name: "calibrant deterministic ok",
        check: rbtchl_probe_ok,
        remediation: "n/a — deterministic probe always Ok",
    };
    if let Err(v) = rbtccp_assert(&probe) {
        return v;
    }
    rbtcee_Verdict::Pass
}

fn rbtchl_progressing_probe_err(_dir: &Path) -> rbtcee_Verdict {
    let probe = rbtccp_Probe {
        name: "calibrant deterministic err",
        check: rbtchl_probe_err,
        remediation: "n/a — deterministic probe always Err for engine surface verification",
    };
    if let Err(v) = rbtccp_assert(&probe) {
        return v;
    }
    rbtcee_Verdict::Fail("calibrant probe-err case body executed unexpectedly".to_string())
}

pub static RBTCHL_CASES_PROGRESSING: &[rbtcee_Case] = &[
    rbtcee_case!(rbtchl_progressing_probe_ok),
    rbtcee_case!(rbtchl_progressing_probe_err),
];

// ── calibrant-sentinel ──────────────────────────────────────

/// Single-case Independent fixture used as a suite-level fail-fast pivot:
/// place this fixture after a failing fixture in the calibrant suite and
/// assert the sentinel is absent.
fn rbtchl_sentinel_marks(dir: &Path) -> rbtcee_Verdict {
    if let Err(v) = rbtchl_write_sentinel(dir) {
        return v;
    }
    rbtcee_Verdict::Pass
}

pub static RBTCHL_CASES_SENTINEL: &[rbtcee_Case] = &[rbtcee_case!(rbtchl_sentinel_marks)];

// ── calibrant-coverage-aligned / -undeclared / -unused ───────
//
// Three single-case Independent fixtures proving the RBTDRI colophon census
// enforcement lands its diagnostics against the synthetic RBTGEC_THEURGE_NIHIL
// colophon — zero cloud/filesystem side effects. Manifest declaration
// (rbtcem_required_colophons) is the only difference between the three; the
// touchstone surface fixture spawns each as a child rbtd run and asserts the
// resulting exit code and diagnostic text (coverage section of its case
// catalog). Roster-only: a member of no suite.

/// Invoke the nihil tabtarget through the real invocation chokepoint
/// (rbtcei_invoke_global). Shared body for the aligned and undeclared cases —
/// their fixtures' manifest declarations are what differ.
fn zrbtchl_invoke_nihil() -> rbtcee_Verdict {
    rbtccc_with_ctx(|ctx| match rbtcei_invoke_global(ctx, RBTGEC_THEURGE_NIHIL, &[], &[]) {
        Ok(result) if result.exit_code == 0 => rbtcee_Verdict::Pass,
        Ok(result) => rbtcee_Verdict::Fail(format!("nihil tabtarget exited {}", result.exit_code)),
        Err(e) => rbtcee_Verdict::Fail(e),
    })
}

/// Declared and invoked: the census positive check has nothing to refuse, and
/// the negative check finds the declared colophon in the used-set — Pass.
fn rbtchl_coverage_aligned_invokes(_dir: &Path) -> rbtcee_Verdict {
    zrbtchl_invoke_nihil()
}

/// Invoked but never declared (this fixture's manifest entry is `Some(&[])`):
/// the census positive check in rbtcei_invoke_impl refuses the invoke before
/// the tabtarget ever launches, failing the fixture.
fn rbtchl_coverage_undeclared_invokes(_dir: &Path) -> rbtcee_Verdict {
    zrbtchl_invoke_nihil()
}

/// Declared but never invoked: this case passes clean, and the engine's
/// negative census check fails the fixture afterward for the unused
/// declaration.
fn rbtchl_coverage_unused_no_invoke(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Pass
}

pub static RBTCHL_CASES_COVERAGE_ALIGNED: &[rbtcee_Case] = &[rbtcee_case!(rbtchl_coverage_aligned_invokes)];
pub static RBTCHL_CASES_COVERAGE_UNDECLARED: &[rbtcee_Case] =
    &[rbtcee_case!(rbtchl_coverage_undeclared_invokes)];
pub static RBTCHL_CASES_COVERAGE_UNUSED: &[rbtcee_Case] = &[rbtcee_case!(rbtchl_coverage_unused_no_invoke)];

// ── calibrant-tariff-drift ──────────────────────────────────
//
// The needle for the tariff shortfall gate (rbtcee_engine.rs, Tariff section):
// a fixture whose declared invocation count its cases cannot meet. Its single
// case SKIPS, which is the whole point — a skip exits zero, so before the gate
// existed this fixture was green while running none of its declared work, the
// exact lying-gate shape the gate closes.
//
// The skip is also what pins the gate's ONE contestable clause. The engine's
// census negative check stands down on a skipped case; this gate deliberately
// does not, and a future tidy-up that made the two match by adding
// `skipped == 0` would turn this fixture green — reddening the touchstone case
// that watches it rather than silently deleting the ruling.
//
// Roster-only, like the coverage calibrants: a member of no suite, driven only
// as a child of the touchstone surface fixture.

/// Skip without invoking anything, against a declared count of one. Observed 0
/// vs declared 1 is the drift; the Skip verdict is what would otherwise carry
/// the fixture to a zero exit.
fn rbtchl_tariff_drift_skips_short(_dir: &Path) -> rbtcee_Verdict {
    rbtcee_Verdict::Skip("calibrant declared-count shortfall, by construction".to_string())
}

pub static RBTCHL_CASES_TARIFF_DRIFT: &[rbtcee_Case] = &[rbtcee_case!(rbtchl_tariff_drift_skips_short)];

// ── Fixture statics ──────────────────────────────────────────

pub static RBTCHL_FIXTURE_VERDICTS: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CALIBRANT_VERDICTS,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHL_CASES_VERDICTS,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
const _: () = assert!(RBTCHL_FIXTURE_VERDICTS.cases.len() == 4);

pub static RBTCHL_FIXTURE_FAIL_FAST: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHL_CASES_FAIL_FAST,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
const _: () = assert!(RBTCHL_FIXTURE_FAIL_FAST.cases.len() == 3);

pub static RBTCHL_FIXTURE_PROGRESSING: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CALIBRANT_PROGRESSING,
    disposition: rbtcee_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    cases: RBTCHL_CASES_PROGRESSING,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
const _: () = assert!(RBTCHL_FIXTURE_PROGRESSING.cases.len() == 2);

pub static RBTCHL_FIXTURE_SENTINEL: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CALIBRANT_SENTINEL,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHL_CASES_SENTINEL,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
const _: () = assert!(RBTCHL_FIXTURE_SENTINEL.cases.len() == 1);

pub static RBTCHL_FIXTURE_COVERAGE_ALIGNED: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHL_CASES_COVERAGE_ALIGNED,
    credless: false,
    // The one calibrant declaring a count it MEETS — its single case invokes
    // nihil exactly once. Standing beside calibrant-tariff-drift it is the
    // shortfall gate's positive control: same driver, same tabtarget chain, a
    // met declaration staying green. Without a declaration here the pair would
    // prove only that a fixture can fail, never that the gate discriminates.
    tariff: rbtcee_Tariff { min_secs: None, max_secs: None, invocations: Some(1) },
};
const _: () = assert!(RBTCHL_FIXTURE_COVERAGE_ALIGNED.cases.len() == 1);

pub static RBTCHL_FIXTURE_COVERAGE_UNDECLARED: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHL_CASES_COVERAGE_UNDECLARED,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
const _: () = assert!(RBTCHL_FIXTURE_COVERAGE_UNDECLARED.cases.len() == 1);

pub static RBTCHL_FIXTURE_COVERAGE_UNUSED: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHL_CASES_COVERAGE_UNUSED,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
const _: () = assert!(RBTCHL_FIXTURE_COVERAGE_UNUSED.cases.len() == 1);

pub static RBTCHL_FIXTURE_TARIFF_DRIFT: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CALIBRANT_TARIFF_DRIFT,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHL_CASES_TARIFF_DRIFT,
    credless: false,
    // DELIBERATELY UNMEETABLE — do not "repair" by lowering this to 0. The one
    // case invokes nothing, so the declaration is short by construction, and
    // that mismatch is the whole fixture: its expected verdict is FAIL. Aligning
    // the count would turn it green and silently delete the test.
    tariff: rbtcee_Tariff { min_secs: None, max_secs: None, invocations: Some(1) },
};
const _: () = assert!(RBTCHL_FIXTURE_TARIFF_DRIFT.cases.len() == 1);
