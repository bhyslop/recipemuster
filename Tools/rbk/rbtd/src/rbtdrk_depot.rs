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
// RBTDRK — keyless freehold-depot fixtures (freehold-establish, freehold-churn)
//
// freehold-establish is §2 of release qualification: it ensures the durable
// freehold the gauntlet's downstream fixtures inherit, admitting FEDERATION
// PERSONAS (the no-keys org enforces disableServiceAccountKeyCreation, so the
// keyfile enrobe 400s on a fresh levy):
//   1. freehold_ensure       — install freehold RBRR prefixes; REUSE the freehold
//                              RBRD already names when ACTIVE, else levy a fresh
//                              canest depot (the levy establishes the three mantle
//                              SAs with frozen IAM)
//   2. compear               — open/confirm a live assize against the RBRF trust
//   3. gird_governor         — the payor (OAuth) seats the freehold subject as the
//                              first governor (rbw-pE)
//   4. brevet_don_director   — the girded governor brevets the freehold subject onto
//                              the director mantle, then dons it and reaches AR
//   5. brevet_don_retriever  — same for the retriever mantle
//   6. depot_recognosce      — read-only proof the levy's founding stands whole
//
// freehold-churn is the deliberate teardown that makes room for a fresh levy —
// member of no suite, operator-invoked.
//
// Disposition: StateProgressing. Each case carries a precondition probe via
// rbtdrb_Probe so a-la-carte single-case rerun fails cleanly when an earlier
// case's state is absent.

use std::path::Path;

use crate::case;
use crate::rbtdrb_probe::{rbtdrb_assert, rbtdrb_Probe};
use crate::rbtdrc_crucible::rbtdrc_with_ctx;
use crate::rbtdre_engine::{rbtdre_Case, rbtdre_Disposition, rbtdre_Fixture, rbtdre_Verdict};
use crate::rbtdri_invocation::{
    RBTDRI_BURE_CONFIRM_KEY,
    RBTDRI_BURE_CONFIRM_SKIP,
    RBTDRI_BURV_OUTPUT_SUBDIR,
    rbtdri_Context,
};
use crate::rbtdgc_consts::{
    RBTDGC_LEVY_DEPOT,
    RBTDGC_LIST_DEPOT,
    RBTDGC_RECOGNOSCE_DEPOT,
    RBTDGC_UNMAKE_DEPOT,
    RBTDGC_GIRD_POLITY,
    RBTDGC_BREVET_POLITY,
    RBTDGC_CHECK_MANTLE,
    RBTDGC_CHECK_COMPEARANCE,
    RBTDGC_FREEHOLD_SUBJECT,
    RBTDGC_RBRD_FILE,
};
use crate::rbtdrm_manifest::{
    RBTDRM_FIXTURE_FREEHOLD_ESTABLISH,
    RBTDRM_FIXTURE_FREEHOLD_CHURN,
};
use crate::rbtdrk_freehold::{
    rbtdrk_burs_tincture,
    rbtdrk_cloud_prefix_subdir,
    rbtdrk_compose_project_id,
    rbtdrk_family_stem,
    rbtdrk_install_depot_moniker,
    rbtdrk_install_freehold_prefixes,
    rbtdrk_invoke_logged,
    rbtdrk_pick_next_moniker,
    rbtdrk_probe_freehold_moniker,
    rbtdrk_probe_rbrr_present,
    rbtdrk_read_env_value,
    RBTDRK_DEPOT_STATE_COMPLETE,
    RBTDRK_FACT_EXT_DEPOT,
    RBTDRK_FACT_EXT_DEPOT_PROJECT,
    RBTDRK_FIELD_RBRD_DEPOT_MONIKER,
};

/// Placeholder moniker installed into RBRD before unmaking the freehold, so
/// rbgp_depot_unmake's live-disqualify guard — which refuses the RBRD-selected
/// project — releases the real one. The next freehold-establish run sees this
/// placeholder as absent and takes the create path.
const RBTDRK_CHURN_PLACEHOLDER_MONIKER: &str = "churned";

// ── Cases ────────────────────────────────────────────────────

/// Case 1 — freehold ensure. Installs canc-/canr- prefixes, then REUSES the
/// freehold RBRD already names when it is ACTIVE (no depot is created), else
/// mints a fresh canest moniker and levies. Cross-checks project_id against the
/// RBDC compose derivation either way. Validity is the recognosce case's job
/// (case 6), not this one — a stale-but-ACTIVE freehold is reused here and fails
/// there.
fn rbtdrk_freehold_ensure(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "rbrr.env present",
        check: rbtdrk_probe_rbrr_present,
        remediation: "ensure regime is initialized — rerun depot-lifecycle if needed",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_freehold_ensure_impl(ctx, dir))
}

fn rbtdrk_freehold_ensure_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    if let Err(e) = rbtdrk_install_freehold_prefixes(&root) {
        return rbtdre_Verdict::Fail(format!("install freehold prefixes: {}", e));
    }

    let list_pre = match rbtdrk_invoke_logged(
        ctx,
        RBTDGC_LIST_DEPOT,
        &[],
        &[],
        dir,
        "list-pre",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("depot list (pre-levy): {}", e)),
    };
    if list_pre.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "depot list (pre-levy) exit {}\n{}",
            list_pre.exit_code, list_pre.stderr
        ));
    }

    let prefix_dir = match rbtdrk_cloud_prefix_subdir(&root) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("resolve cloud_prefix subdir: {}", e)),
    };

    // Idempotent freehold ensure: reuse the freehold RBRD already names when it
    // is ACTIVE — no depot is created on a routine run; otherwise (blank, absent,
    // or DELETE_REQUESTED — a graveyarded id is treated as gone) mint a fresh
    // moniker and levy. Validity is NOT judged here: recognosce (the unconditional
    // final case) is the freehold's validity gate, so a stale-but-ACTIVE freehold
    // is reused here and fails there, prompting a deliberate churn.
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let current =
        rbtdrk_read_env_value(&rbrd, RBTDRK_FIELD_RBRD_DEPOT_MONIKER).unwrap_or_default();
    let reuse = !current.is_empty() && {
        let state_fact = list_pre
            .burv_output
            .join(RBTDRI_BURV_OUTPUT_SUBDIR)
            .join(&prefix_dir)
            .join(format!("{}.{}", current, RBTDRK_FACT_EXT_DEPOT));
        std::fs::read_to_string(&state_fact)
            .map(|s| s.trim() == RBTDRK_DEPOT_STATE_COMPLETE)
            .unwrap_or(false)
    };

    let moniker = if reuse {
        let _ =
            std::fs::write(dir.join("freehold-decision.txt"), format!("reused {}", current));
        current
    } else {
        let tincture = match rbtdrk_burs_tincture() {
            Ok(t) => t,
            Err(e) => return rbtdre_Verdict::Fail(format!("read BURS_TINCTURE: {}", e)),
        };
        let family_stem = rbtdrk_family_stem(&tincture);
        let m = match rbtdrk_pick_next_moniker(&list_pre, &root, &family_stem) {
            Ok(m) => m,
            Err(e) => return rbtdre_Verdict::Fail(format!("pick next moniker: {}", e)),
        };
        if let Err(e) = rbtdrk_install_depot_moniker(&root, &m) {
            return rbtdre_Verdict::Fail(format!("install depot moniker: {}", e));
        }
        let levy = match rbtdrk_invoke_logged(ctx, RBTDGC_LEVY_DEPOT, &[], &[], dir, "levy") {
            Ok(r) => r,
            Err(e) => return rbtdre_Verdict::Fail(format!("depot levy: {}", e)),
        };
        if levy.exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "depot levy exit {}\n{}",
                levy.exit_code, levy.stderr
            ));
        }
        let _ = std::fs::write(dir.join("freehold-decision.txt"), format!("levied {}", m));
        m
    };

    // project-id cross-check (both paths): RBDC compose must equal the depot's
    // actual id. Reuse reads the fact from list_pre (the freehold is already
    // listed there); create re-lists so the fact reflects the just-levied project.
    let fact_list = if reuse {
        list_pre
    } else {
        match rbtdrk_invoke_logged(ctx, RBTDGC_LIST_DEPOT, &[], &[], dir, "list-present") {
            Ok(r) if r.exit_code == 0 => r,
            Ok(r) => {
                return rbtdre_Verdict::Fail(format!(
                    "depot list (after levy) exit {}\n{}",
                    r.exit_code, r.stderr
                ))
            }
            Err(e) => return rbtdre_Verdict::Fail(format!("depot list (after levy): {}", e)),
        }
    };
    let fact_path = fact_list
        .burv_output
        .join(RBTDRI_BURV_OUTPUT_SUBDIR)
        .join(&prefix_dir)
        .join(format!("{}.{}", moniker, RBTDRK_FACT_EXT_DEPOT_PROJECT));
    let fact_project_id = match std::fs::read_to_string(&fact_path) {
        Ok(s) => s.trim().to_string(),
        Err(e) => {
            return rbtdre_Verdict::Fail(format!(
                "read depot-project fact '{}': {}",
                fact_path.display(),
                e
            ))
        }
    };
    if fact_project_id.is_empty() {
        return rbtdre_Verdict::Fail(format!(
            "depot-project fact is empty: {}",
            fact_path.display()
        ));
    }
    let _ = std::fs::write(dir.join("project-id.txt"), &fact_project_id);

    let composed = match rbtdrk_compose_project_id(&root, &moniker) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("compose project_id: {}", e)),
    };
    if composed != fact_project_id {
        return rbtdre_Verdict::Fail(format!(
            "project_id mismatch: RBDC compose='{}' vs depot-list fact='{}' \
             (RBDC kindle derivation diverged from payor creation)",
            composed, fact_project_id
        ));
    }
    rbtdre_Verdict::Pass
}

/// Final case — depot recognosce. Read-only proof that the levy's federation-
/// founding gestures stand whole against live GCP: the three mantle SAs, their
/// capability-sets, and the Artifact Registry Data-Access audit config. The
/// rbw-dr verb does the entire check and dies fatally naming any absent piece;
/// this case asserts only that it exits 0 — founding whole.
fn rbtdrk_depot_recognosce(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "freehold depot moniker installed",
        check: rbtdrk_probe_freehold_moniker,
        remediation: "rerun rbtdrk_freehold_ensure or the full freehold-establish fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_depot_recognosce_impl(ctx, dir))
}

fn rbtdrk_depot_recognosce_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let recognosce = match rbtdrk_invoke_logged(
        ctx,
        RBTDGC_RECOGNOSCE_DEPOT,
        &[],
        &[],
        dir,
        "recognosce",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("depot recognosce: {}", e)),
    };
    if recognosce.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "depot recognosce exit {} — founding not whole\n{}",
            recognosce.exit_code, recognosce.stderr
        ));
    }
    rbtdre_Verdict::Pass
}

/// Freehold churn — the deliberate teardown that makes room for a fresh levy.
/// Reads the freehold RBRD names, rotates the moniker to a placeholder so
/// rbgp_depot_unmake's live-disqualify guard releases the real project, unmakes
/// it (confirm skipped via the test seam), and confirms it is no longer ACTIVE.
/// A subsequent freehold-establish run then finds no ACTIVE freehold and takes
/// the create path.
fn rbtdrk_depot_churn(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "freehold depot moniker installed",
        check: rbtdrk_probe_freehold_moniker,
        remediation: "no freehold to churn — install one via freehold-establish first",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_depot_churn_impl(ctx, dir))
}

fn rbtdrk_depot_churn_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();
    let rbrd = root.join(RBTDGC_RBRD_FILE);

    let moniker = match rbtdrk_read_env_value(&rbrd, RBTDRK_FIELD_RBRD_DEPOT_MONIKER) {
        Some(m) if !m.is_empty() => m,
        _ => {
            return rbtdre_Verdict::Fail(
                "RBRD_DEPOT_MONIKER blank — no freehold to churn".to_string(),
            )
        }
    };
    let project_id = match rbtdrk_compose_project_id(&root, &moniker) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("compose project_id: {}", e)),
    };
    let _ = std::fs::write(dir.join("churned-project-id.txt"), &project_id);

    // Rotate the moniker off the live freehold so the unmake's live-disqualify
    // guard releases it, then unmake with the confirm skipped via the test seam.
    if let Err(e) = rbtdrk_install_depot_moniker(&root, RBTDRK_CHURN_PLACEHOLDER_MONIKER) {
        return rbtdre_Verdict::Fail(format!("rotate moniker before unmake: {}", e));
    }

    let unmake = match rbtdrk_invoke_logged(
        ctx,
        RBTDGC_UNMAKE_DEPOT,
        &[&project_id],
        &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
        dir,
        "churn-unmake",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("depot unmake: {}", e)),
    };
    if unmake.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "depot unmake exit {}\n{}",
            unmake.exit_code, unmake.stderr
        ));
    }

    // Confirm the freehold is no longer ACTIVE: a fresh list's state fact for the
    // churned moniker must read anything but COMPLETE (DELETE_REQUESTED is the
    // soft-delete terminal; an absent fact means fully gone). Still ACTIVE means
    // the unmake did not take.
    let list_after = match rbtdrk_invoke_logged(ctx, RBTDGC_LIST_DEPOT, &[], &[], dir, "list-after") {
        Ok(r) if r.exit_code == 0 => r,
        Ok(r) => {
            return rbtdre_Verdict::Fail(format!(
                "depot list (after unmake) exit {}\n{}",
                r.exit_code, r.stderr
            ))
        }
        Err(e) => return rbtdre_Verdict::Fail(format!("depot list (after unmake): {}", e)),
    };
    let prefix_dir = match rbtdrk_cloud_prefix_subdir(&root) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("resolve cloud_prefix subdir: {}", e)),
    };
    let state_fact = list_after
        .burv_output
        .join(RBTDRI_BURV_OUTPUT_SUBDIR)
        .join(&prefix_dir)
        .join(format!("{}.{}", moniker, RBTDRK_FACT_EXT_DEPOT));
    if let Ok(s) = std::fs::read_to_string(&state_fact) {
        if s.trim() == RBTDRK_DEPOT_STATE_COMPLETE {
            return rbtdre_Verdict::Fail(format!("freehold {} still ACTIVE after unmake", project_id));
        }
    }
    rbtdre_Verdict::Pass
}

// ── Federation-persona cases (freehold-establish) ────────────
//
// freehold-establish admits federation personas on the no-keys org. The freehold
// subject (the operator's standing Entra oid, RBTDGC_FREEHOLD_SUBJECT) is compeared,
// girded as the first governor by the payor, then breveted onto the director and
// retriever mantles and donned — the federation replacement for the retired keyfile
// governor/retriever/director enrobe + JWT-probe cases.

/// Suite-head compearance. Opens or confirms a live assize against the RBRF trust
/// (rbw-acf): a cache-hit when the operator pre-compeared, an inline device-flow prompt
/// when a TTY is present, a loud headless failure otherwise. The admission cases below
/// ride the cached federated token, so the human clicks once here, not per case.
fn rbtdrk_compear(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "freehold depot moniker installed",
        check: rbtdrk_probe_freehold_moniker,
        remediation: "rerun rbtdrk_freehold_ensure or the full freehold-establish fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| {
        let r = match rbtdrk_invoke_logged(ctx, RBTDGC_CHECK_COMPEARANCE, &[], &[], dir, "compear") {
            Ok(r) => r,
            Err(e) => return rbtdre_Verdict::Fail(format!("compearance probe: {}", e)),
        };
        if r.exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "compearance failed (exit {}) — open an assize before the run with rbw-acf \
                 (one device-flow click), or launch from a terminal so the prompt can surface\n{}",
                r.exit_code, r.stderr
            ));
        }
        rbtdre_Verdict::Pass
    })
}

/// Gird the founding governor. The payor (OAuth) seats the freehold subject as this depot's
/// first governor (rbw-pE) — the one admission outside governor wielding, the founding door a
/// fresh levy needs before any mantle can be donned. Payor-credentialed, so it needs no
/// assize. Replaces the keyfile governor-enrobe's admin-credential step.
fn rbtdrk_gird_governor(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "freehold depot moniker installed",
        check: rbtdrk_probe_freehold_moniker,
        remediation: "rerun rbtdrk_freehold_ensure or the full freehold-establish fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| {
        let gird = match rbtdrk_invoke_logged(
            ctx,
            RBTDGC_GIRD_POLITY,
            &[RBTDGC_FREEHOLD_SUBJECT],
            &[],
            dir,
            "gird-governor",
        ) {
            Ok(r) => r,
            Err(e) => return rbtdre_Verdict::Fail(format!("gird governor: {}", e)),
        };
        if gird.exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "gird governor exit {}\n{}",
                gird.exit_code, gird.stderr
            ));
        }
        rbtdre_Verdict::Pass
    })
}

/// Shared federation-admission body for director and retriever: the girded governor brevets
/// the freehold subject onto the named mantle (rbw-pB, governor-wielded — rides the assize),
/// then dons that mantle and reaches Artifact Registry (rbw-acm: compear cache-hit → don →
/// repositories.list). The don is the federation analog of the keyfile JWT access-probe.
fn rbtdrk_brevet_don_impl(ctx: &mut rbtdri_Context, dir: &Path, mantle: &str) -> rbtdre_Verdict {
    let label_brevet = format!("brevet-{}", mantle);
    let brevet = match rbtdrk_invoke_logged(
        ctx,
        RBTDGC_BREVET_POLITY,
        &[RBTDGC_FREEHOLD_SUBJECT, mantle],
        &[],
        dir,
        &label_brevet,
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("brevet {}: {}", mantle, e)),
    };
    if brevet.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "brevet {} exit {}\n{}",
            mantle, brevet.exit_code, brevet.stderr
        ));
    }

    let label_don = format!("don-{}", mantle);
    let don = match rbtdrk_invoke_logged(ctx, RBTDGC_CHECK_MANTLE, &[mantle], &[], dir, &label_don) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("don {}: {}", mantle, e)),
    };
    if don.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "don {} exit {} — mantle not donnable or AR unreachable\n{}",
            mantle, don.exit_code, don.stderr
        ));
    }
    rbtdre_Verdict::Pass
}

/// Brevet + don the director mantle for the freehold subject.
fn rbtdrk_brevet_don_director(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "freehold depot moniker installed",
        check: rbtdrk_probe_freehold_moniker,
        remediation: "rerun the full freehold-establish fixture (ensure → compear → gird) first",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_brevet_don_impl(ctx, dir, "director"))
}

/// Brevet + don the retriever mantle for the freehold subject.
fn rbtdrk_brevet_don_retriever(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "freehold depot moniker installed",
        check: rbtdrk_probe_freehold_moniker,
        remediation: "rerun the full freehold-establish fixture (ensure → compear → gird) first",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_brevet_don_impl(ctx, dir, "retriever"))
}

// ── Section registry ─────────────────────────────────────────

pub static RBTDRK_CASES_FREEHOLD_ESTABLISH: &[rbtdre_Case] = &[
    case!(rbtdrk_freehold_ensure),
    case!(rbtdrk_compear),
    case!(rbtdrk_gird_governor),
    case!(rbtdrk_brevet_don_director),
    case!(rbtdrk_brevet_don_retriever),
    case!(rbtdrk_depot_recognosce),
];

pub static RBTDRK_FIXTURE_FREEHOLD_ESTABLISH: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_FREEHOLD_ESTABLISH,
    disposition: rbtdre_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    cases: RBTDRK_CASES_FREEHOLD_ESTABLISH,
    credless: false,
};

// freehold-churn — the deliberate teardown of the freehold. Single case: rotate
// the moniker off the live project, unmake it, confirm gone. Member of no suite —
// operator-invoked, quota-reclaiming, never a suite passenger. This is the ONLY
// fixture that destroys the standing freehold; the depot-lifecycle's tear-down
// only reaches the fresh leasehold it minted (pick_next's max + 1).
pub static RBTDRK_CASES_FREEHOLD_CHURN: &[rbtdre_Case] = &[
    case!(rbtdrk_depot_churn),
];

pub static RBTDRK_FIXTURE_FREEHOLD_CHURN: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_FREEHOLD_CHURN,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRK_CASES_FREEHOLD_CHURN,
    credless: false,
};
