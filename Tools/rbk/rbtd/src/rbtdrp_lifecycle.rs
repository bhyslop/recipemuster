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
// RBTDRP — depot-lifecycle fixture for theurge release qualification
//
// The ephemeral depot: a full birth-to-death proof that stands up a fresh
// depot, drives the spec-named tripwire recovery (levy refused at its
// inscription tail by the clean-tree gate — the depot live but un-inscribed —
// then recovered standalone via rbw-rdi and confirmed via rbw-rdc), proves
// the live-unmake guard, and tears it down — the gauntlet's create→destroy
// lifecycle test, run alongside the durable freehold-establish (which carries
// the clean full-levy proof).
//
// It shares the freehold scheme (rbtcgf_freehold) — one prefix family, one moniker
// picker — with the durable fixtures; it does NOT carry its own. The picker's
// `max + 1` always mints a moniker ABOVE the standing freehold's, so this fixture's
// tear-down only reaches the fresh leasehold it minted, never the freehold (only
// the deliberate, suiteless freehold-churn destroys that).
//
// Case 1 — the marshal-zero attestation gate — lives in the sibling rbtcga_attest
// module; the registry below composes it ahead of this arc's five cases.

use std::path::Path;

use crate::rbtcee_case;
use crate::rbtccp_probe::{rbtccp_assert, rbtccp_Probe};
use crate::rbtccc_crucible::rbtccc_with_ctx;
use crate::rbtcee_engine::{
    rbtcee_commit_regime, rbtcee_Case, rbtcee_Disposition, rbtcee_Fixture, rbtcee_RegimeFile,
    rbtcee_Tariff, rbtcee_Verdict,
};
use crate::rbtcei_invocation::{
    rbtcei_Context,
    RBTCEI_BURE_CONFIRM_KEY,
    RBTCEI_BURE_CONFIRM_SKIP,
};
use crate::rbtgec_consts::{
    RBTGEC_COLLATE_DEPOT,
    RBTGEC_INCUSE_DEPOT,
    RBTGEC_LEVY_DEPOT,
    RBTGEC_LIST_DEPOT,
    RBTGEC_RBRD_FILE,
    RBTGEC_UNMAKE_DEPOT,
};
use crate::rbtgeb_breviary::RBTGEB_BUSX_CLEAN_DENTIL;
use crate::rbtcem_manifest::RBTCEM_FIXTURE_DEPOT_LIFECYCLE;
use crate::rbtcga_attest::rbtcga_marshal_zero_attestation;
use crate::rbtcgf_freehold::{
    rbtcgf_burs_tincture,
    rbtcgf_cloud_prefix_subdir,
    rbtcgf_compose_project_id,
    rbtcgf_crosscheck_project_id,
    rbtcgf_family_stem,
    rbtcgf_install_depot_moniker,
    rbtcgf_install_freehold_prefixes,
    rbtcgf_invoke_logged,
    rbtcgf_pick_next_moniker,
    rbtcgf_read_env_value,
    rbtcgf_unmake_preamble,
    rbtcgf_UnmakeSpec,
    RBTCGF_FIELD_RBRD_CLOUD_PREFIX,
    RBTCGF_FIELD_RBRD_DEPOT_MONIKER,
};

/// Placeholder moniker installed by tear_down before invoking rbw-dU. With the
/// live moniker still in rbrd.env, RBDC composes to the depot's own project_id
/// and rbgp_depot_unmake's live-disqualify guard refuses the call. Rotating to a
/// value outside the family stem makes RBDC compose to a different value so the
/// guard lets the unmake through. Marshal-zero is the recovery between runs and
/// blanks the field anyway.
const RBTCGL_TEAR_DOWN_PLACEHOLDER_MONIKER: &str = "torndown";

/// `DELETE_REQUESTED` lifecycle state — appears in `rbgp_depot_list` output
/// after a soft-delete, used by tear-down to relax the post-unmake assertion.
const RBTCGL_DELETE_REQUESTED: &str = "DELETE_REQUESTED";

/// Probe line appended to rbrd.env (uncommitted) between moniker install and
/// levy — the deliberate dirty-tracked-tree state that makes levy's tripwire
/// inscription refuse with the clean-tree band while the depot stands.
/// Committed by the tripwire-recover case; marshal-zero rewrites rbrd.env
/// before the next run.
const RBTCGL_TRIPWIRE_PROBE_LINE: &str = "# rbtdrp tripwire-recovery probe";

// ── Probe ────────────────────────────────────────────────────
//
// rbtccp_Probe.check is `fn() -> Result<(), String>` with no parameters,
// so the probe reads the project root from current_dir() — theurge always
// launches from the project root.

/// Live-disqualify case probe: depot levied (RBRD_CLOUD_PREFIX +
/// RBRD_DEPOT_MONIKER both non-blank, RBDC kindle composes a non-empty
/// project_id). Established by the stand-up case.
fn rbtcgl_probe_depot_levied() -> Result<(), String> {
    let root = std::env::current_dir()
        .map_err(|e| format!("cannot resolve project root: {}", e))?;
    let rbrd = root.join(RBTGEC_RBRD_FILE);

    let cloud = rbtcgf_read_env_value(&rbrd, RBTCGF_FIELD_RBRD_CLOUD_PREFIX).unwrap_or_default();
    if cloud.is_empty() {
        return Err(format!(
            "{} blank in {} — freehold prefixes not installed",
            RBTCGF_FIELD_RBRD_CLOUD_PREFIX,
            rbrd.display()
        ));
    }

    let moniker = rbtcgf_read_env_value(&rbrd, RBTCGF_FIELD_RBRD_DEPOT_MONIKER).unwrap_or_default();
    if moniker.is_empty() {
        return Err(format!(
            "{} blank in {} — depot stand-up did not run",
            RBTCGF_FIELD_RBRD_DEPOT_MONIKER,
            rbrd.display()
        ));
    }

    Ok(())
}

// ── Case 2: depot stand-up ───────────────────────────────────

/// Append the tripwire probe line to rbrd.env, uncommitted — the deliberate
/// dirty-tracked-tree state that drives levy's inscription refusal.
fn zrbtcgl_append_tripwire_probe(rbrd: &Path) -> Result<(), String> {
    let mut content = std::fs::read_to_string(rbrd)
        .map_err(|e| format!("read {}: {}", rbrd.display(), e))?;
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(RBTCGL_TRIPWIRE_PROBE_LINE);
    content.push('\n');
    std::fs::write(rbrd, content).map_err(|e| format!("write {}: {}", rbrd.display(), e))
}

/// Case 2 — depot stand-up, refused at the tripwire tail. Installs freehold
/// prefixes, picks the next free moniker in the freehold family, appends the
/// uncommitted probe line to rbrd.env, and levies: every provisioning step
/// lands, then the tripwire inscription — the levy tail behind the
/// clean-tree gate — refuses with the clean-tree band, leaving the depot in
/// the spec-named "live but un-inscribed" state the recovery arc finishes.
/// Re-lists to refresh facts, reads project_id from the
/// `<moniker>.depot-project` fact file, and cross-checks it against the RBDC
/// compose derivation. The moniker survives in rbrd.env for the later cases.
fn rbtcgl_depot_stand_up(dir: &Path) -> rbtcee_Verdict {
    rbtccc_with_ctx(|ctx| zrbtcgl_depot_stand_up_impl(ctx, dir))
}

fn zrbtcgl_depot_stand_up_impl(ctx: &mut rbtcei_Context, dir: &Path) -> rbtcee_Verdict {
    let root = ctx.rbtcei_project_root().to_path_buf();

    if let Err(e) = rbtcgf_install_freehold_prefixes(&root) {
        return rbtcee_Verdict::Fail(format!("install freehold prefixes: {}", e));
    }

    let list_pre = match rbtcgf_invoke_logged(
        ctx,
        RBTGEC_LIST_DEPOT,
        &[],
        &[],
        dir,
        "list-pre",
    ) {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("depot list (pre-levy): {}", e)),
    };
    if list_pre.exit_code != 0 {
        return rbtcee_Verdict::Fail(format!(
            "depot list (pre-levy) exit {}\n{}",
            list_pre.exit_code, list_pre.stderr
        ));
    }

    let tincture = match rbtcgf_burs_tincture() {
        Ok(t) => t,
        Err(e) => return rbtcee_Verdict::Fail(format!("read BURS_TINCTURE: {}", e)),
    };
    let family_stem = rbtcgf_family_stem(&tincture);
    let moniker = match rbtcgf_pick_next_moniker(&list_pre, &root, &family_stem) {
        Ok(m) => m,
        Err(e) => return rbtcee_Verdict::Fail(format!("pick next moniker: {}", e)),
    };
    if let Err(e) = rbtcgf_install_depot_moniker(&root, &moniker) {
        return rbtcee_Verdict::Fail(format!("install depot moniker: {}", e));
    }

    if let Err(e) = zrbtcgl_append_tripwire_probe(&root.join(RBTGEC_RBRD_FILE)) {
        return rbtcee_Verdict::Fail(format!("append tripwire probe: {}", e));
    }

    let levy = match rbtcgf_invoke_logged(
        ctx,
        RBTGEC_LEVY_DEPOT,
        &[],
        &[],
        dir,
        "levy",
    ) {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("depot levy: {}", e)),
    };
    if levy.exit_code != RBTGEB_BUSX_CLEAN_DENTIL {
        return rbtcee_Verdict::Fail(format!(
            "depot levy exit {} — expected the clean-tree band {} from the \
             tripwire inscription refusal at the levy tail\n{}",
            levy.exit_code, RBTGEB_BUSX_CLEAN_DENTIL, levy.stderr
        ));
    }

    let list_present = match rbtcgf_invoke_logged(
        ctx,
        RBTGEC_LIST_DEPOT,
        &[],
        &[],
        dir,
        "list-present",
    ) {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("depot list (after levy): {}", e)),
    };
    if list_present.exit_code != 0 {
        return rbtcee_Verdict::Fail(format!(
            "depot list (after levy) exit {}\n{}",
            list_present.exit_code, list_present.stderr
        ));
    }

    let prefix_dir = match rbtcgf_cloud_prefix_subdir(&root) {
        Ok(p) => p,
        Err(e) => return rbtcee_Verdict::Fail(format!("resolve cloud_prefix subdir: {}", e)),
    };
    if let Some(verdict) =
        rbtcgf_crosscheck_project_id(&root, &list_present, &prefix_dir, &moniker, dir)
    {
        return verdict;
    }

    rbtcee_Verdict::Pass
}

// ── Cases 3-4: tripwire recovery arc ─────────────────────────

/// Case 3 — tripwire recover. Pre-condition: stand-up left the depot live but
/// un-inscribed (the levy-tail refusal) with the probe line uncommitted.
/// Commits the probe line — the "after committing" step the depot-levy spec
/// prescribes — then invokes the standalone recovery entry (rbw-rdi,
/// payor-credentialed, no positional token) and expects a clean inscription.
fn rbtcgl_tripwire_recover(dir: &Path) -> rbtcee_Verdict {
    let probe = rbtccp_Probe {
        name: "depot levied (RBRD_CLOUD_PREFIX + RBRD_DEPOT_MONIKER set)",
        check: rbtcgl_probe_depot_levied,
        remediation: "rerun the stand-up case (rbtcgl_depot_stand_up) before this case",
    };
    if let Err(v) = rbtccp_assert(&probe) {
        return v;
    }
    rbtccc_with_ctx(|ctx| zrbtcgl_tripwire_recover_impl(ctx, dir))
}

fn zrbtcgl_tripwire_recover_impl(ctx: &mut rbtcei_Context, dir: &Path) -> rbtcee_Verdict {
    let root = ctx.rbtcei_project_root().to_path_buf();

    if let Err(e) = rbtcee_commit_regime(
        &root,
        &[rbtcee_RegimeFile::Rbrd],
        "depot-lifecycle: commit tripwire probe line before standalone inscription",
    ) {
        return rbtcee_Verdict::Fail(format!("commit tripwire probe: {}", e));
    }

    let inscribe = match rbtcgf_invoke_logged(
        ctx,
        RBTGEC_INCUSE_DEPOT,
        &[],
        &[],
        dir,
        "incuse",
    ) {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("tripwire inscribe: {}", e)),
    };
    if inscribe.exit_code != 0 {
        return rbtcee_Verdict::Fail(format!(
            "tripwire inscribe exit {}\n{}",
            inscribe.exit_code, inscribe.stderr
        ));
    }

    rbtcee_Verdict::Pass
}

/// Case 4 — tripwire confirm. The standalone check (rbw-rdc,
/// payor-credentialed, no positional token) pulls the just-inscribed tripwire
/// and byte-compares it against the local rbrd.env — the recover→confirm
/// ceremony's proof leg.
fn rbtcgl_tripwire_confirm(dir: &Path) -> rbtcee_Verdict {
    let probe = rbtccp_Probe {
        name: "depot levied (RBRD_CLOUD_PREFIX + RBRD_DEPOT_MONIKER set)",
        check: rbtcgl_probe_depot_levied,
        remediation: "rerun the stand-up case (rbtcgl_depot_stand_up) before this case",
    };
    if let Err(v) = rbtccp_assert(&probe) {
        return v;
    }
    rbtccc_with_ctx(|ctx| zrbtcgl_tripwire_confirm_impl(ctx, dir))
}

fn zrbtcgl_tripwire_confirm_impl(ctx: &mut rbtcei_Context, dir: &Path) -> rbtcee_Verdict {
    let check = match rbtcgf_invoke_logged(
        ctx,
        RBTGEC_COLLATE_DEPOT,
        &[],
        &[],
        dir,
        "collate",
    ) {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("tripwire check: {}", e)),
    };
    if check.exit_code != 0 {
        return rbtcee_Verdict::Fail(format!(
            "tripwire check exit {}\n{}",
            check.exit_code, check.stderr
        ));
    }

    rbtcee_Verdict::Pass
}

// ── Case 5: live-disqualify refusal ──────────────────────────

/// Recovery-diagnostic substring emitted by `rbgp_depot_unmake`'s
/// live-disqualify branch. The branch names `RBRD_DEPOT_MONIKER` rename or
/// `rbw-MZ` as recovery paths; the assertion matches on the field-name token,
/// which is invariant across cosmetic message edits.
const RBTCGL_LIVE_DISQUALIFY_RECOVERY: &str = "RBRD_DEPOT_MONIKER";

/// Case 5 — live-disqualify refusal. Pre-condition: depot levied by stand-up
/// (probe asserts both RBRD_CLOUD_PREFIX and RBRD_DEPOT_MONIKER are non-blank).
/// Composes the live RBDC_DEPOT_PROJECT_ID and invokes `rbw-dU` with it as $1;
/// expects non-zero exit + recovery diagnostic naming `RBRD_DEPOT_MONIKER`
/// (BBAA9 contract). The refusal lands before authenticate, so no GCP traffic
/// occurs — assertion is on exit-code + diagnostic shape only.
fn rbtcgl_depot_live_disqualify(dir: &Path) -> rbtcee_Verdict {
    let probe = rbtccp_Probe {
        name: "depot levied (RBRD_CLOUD_PREFIX + RBRD_DEPOT_MONIKER set)",
        check: rbtcgl_probe_depot_levied,
        remediation: "rerun the stand-up case (rbtcgl_depot_stand_up) before this case",
    };
    if let Err(v) = rbtccp_assert(&probe) {
        return v;
    }
    rbtccc_with_ctx(|ctx| zrbtcgl_depot_live_disqualify_impl(ctx, dir))
}

fn zrbtcgl_depot_live_disqualify_impl(
    ctx: &mut rbtcei_Context,
    dir: &Path,
) -> rbtcee_Verdict {
    let root = ctx.rbtcei_project_root().to_path_buf();

    let rbrd = root.join(RBTGEC_RBRD_FILE);
    let moniker = match rbtcgf_read_env_value(&rbrd, RBTCGF_FIELD_RBRD_DEPOT_MONIKER) {
        Some(m) if !m.is_empty() => m,
        _ => {
            return rbtcee_Verdict::Fail(
                "RBRD_DEPOT_MONIKER blank — probe should have caught this".to_string(),
            )
        }
    };

    let project_id = match rbtcgf_compose_project_id(&root, &moniker) {
        Ok(p) => p,
        Err(e) => return rbtcee_Verdict::Fail(format!("compose project_id: {}", e)),
    };
    let _ = std::fs::write(dir.join("live-project-id.txt"), &project_id);

    let result = match rbtcgf_invoke_logged(
        ctx,
        RBTGEC_UNMAKE_DEPOT,
        &[&project_id],
        &[(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)],
        dir,
        "live-disqualify",
    ) {
        Ok(r) => r,
        Err(e) => {
            return rbtcee_Verdict::Fail(format!("{} invocation: {}", RBTGEC_UNMAKE_DEPOT, e))
        }
    };

    if result.exit_code == 0 {
        return rbtcee_Verdict::Fail(format!(
            "{} '{}' exited 0 — BBAA9 live-disqualify contract violated \
             (refusal must die when target == RBDC_DEPOT_PROJECT_ID)",
            RBTGEC_UNMAKE_DEPOT, project_id
        ));
    }

    // BUW dispatch merges stderr→stdout; assertion checks combined output.
    let combined = format!("{}{}", result.stdout, result.stderr);
    if !combined.contains(RBTCGL_LIVE_DISQUALIFY_RECOVERY) {
        return rbtcee_Verdict::Fail(format!(
            "{} live-disqualify diagnostic did not name '{}' as recovery path\n\
             stdout:\n{}\n\nstderr:\n{}",
            RBTGEC_UNMAKE_DEPOT, RBTCGL_LIVE_DISQUALIFY_RECOVERY, result.stdout, result.stderr
        ));
    }

    rbtcee_Verdict::Pass
}

// ── Case 6: depot tear-down ──────────────────────────────────

/// Case 6 — depot tear-down. Pre-condition: depot exists from stand-up. Reads
/// moniker from rbrd.env, composes the project_id from it, rotates
/// RBRD_DEPOT_MONIKER to a placeholder so rbgp_depot_unmake's live-disqualify
/// guard lets the unmake through, invokes rbw-dU with the captured project_id
/// (BURE_CONFIRM=skip), re-lists, and verifies the depot is absent or in
/// DELETE_REQUESTED state via fact-file content read (no stdout-grep).
fn rbtcgl_depot_tear_down(dir: &Path) -> rbtcee_Verdict {
    rbtccc_with_ctx(|ctx| zrbtcgl_depot_tear_down_impl(ctx, dir))
}

fn zrbtcgl_depot_tear_down_impl(ctx: &mut rbtcei_Context, dir: &Path) -> rbtcee_Verdict {
    let outcome = match rbtcgf_unmake_preamble(
        ctx,
        dir,
        &rbtcgf_UnmakeSpec {
            blank_moniker_msg: "stand-up did not run or rbrd.env is missing the moniker \
                                (RBRD_DEPOT_MONIKER is blank)",
            project_id_filename: "project-id.txt",
            placeholder_moniker: RBTCGL_TEAR_DOWN_PLACEHOLDER_MONIKER,
            unmake_label: "unmake",
        },
    ) {
        Ok(o) => o,
        Err(v) => return v,
    };

    // Post-unmake assertion — tear-down is a fail-CLOSED allowlist: ONLY an
    // absent fact or DELETE_REQUESTED passes; every other state — including an
    // unreadable fact — fails. This strict allowlist IS the gauntlet's
    // create→destroy proof, and its polarity is STRUCTURALLY INVERTED from
    // churn's fail-open denylist; the two are deliberately not merged.
    if !outcome.state_fact.exists() {
        return rbtcee_Verdict::Pass;
    }

    let depot_state = match std::fs::read_to_string(&outcome.state_fact) {
        Ok(s) => s.trim().to_string(),
        Err(e) => {
            return rbtcee_Verdict::Fail(format!(
                "read depot fact '{}': {}",
                outcome.state_fact.display(),
                e
            ))
        }
    };

    if depot_state == RBTCGL_DELETE_REQUESTED {
        return rbtcee_Verdict::Pass;
    }

    rbtcee_Verdict::Fail(format!(
        "depot '{}' (project '{}') still present with unexpected state '{}' after unmake \
         (expected absent or '{}')",
        outcome.moniker, outcome.project_id, depot_state, RBTCGL_DELETE_REQUESTED
    ))
}

// ── Case registry ────────────────────────────────────────────

pub static RBTCGL_CASES_DEPOT_LIFECYCLE: &[rbtcee_Case] = &[
    rbtcee_case!(rbtcga_marshal_zero_attestation),
    rbtcee_case!(rbtcgl_depot_stand_up),
    rbtcee_case!(rbtcgl_tripwire_recover),
    rbtcee_case!(rbtcgl_tripwire_confirm),
    rbtcee_case!(rbtcgl_depot_live_disqualify),
    rbtcee_case!(rbtcgl_depot_tear_down),
];

pub static RBTCGL_FIXTURE_DEPOT_LIFECYCLE: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_DEPOT_LIFECYCLE,
    disposition: rbtcee_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    cases: RBTCGL_CASES_DEPOT_LIFECYCLE,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
const _: () = assert!(RBTCGL_FIXTURE_DEPOT_LIFECYCLE.cases.len() == 6);
