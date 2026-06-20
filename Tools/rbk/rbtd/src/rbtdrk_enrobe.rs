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
// RBTDRK — freehold-enrobe fixture (bridge-legacy keyfile SA estate)
//
// freehold-enrobe is the no-levy recycle variant shared by skirmish, dogfight,
// and blockade: it runs against a depot the operator has levied by hand (no GCP
// project created per run) and exercises the bridge-legacy KEYFILE enrobe/defrock
// estate — governor enrobe, then defrock + re-enrobe retriever + director, so
// every run stresses the IAM eventual-consistency edges (delete→recreate flap).
//
// This whole module is bridge-legacy: the keyfile enrobe estate is retired whole
// when keyfile credentials (the RBRA estate) are removed from the system. It is
// deliberately isolated in its own file so that demolition is a file delete plus
// a registry line, not surgery on the keyless freehold-establish/churn fixtures
// (rbtdrk_depot).

use std::path::Path;

use crate::case;
use crate::rbtdrb_probe::{rbtdrb_assert, rbtdrb_Probe};
use crate::rbtdrc_crucible::rbtdrc_with_ctx;
use crate::rbtdre_engine::{rbtdre_Case, rbtdre_Disposition, rbtdre_Fixture, rbtdre_Verdict};
use crate::rbtdri_invocation::{rbtdri_Context, rbtdri_invoke_global, rbtdri_read_burv_fact};
use crate::rbtdgc_consts::{
    RBTDGC_DEFROCK_DIRECTOR,
    RBTDGC_DEFROCK_RETRIEVER,
    RBTDGC_ENROBE_DIRECTOR,
    RBTDGC_ENROBE_GOVERNOR,
    RBTDGC_ENROBE_RETRIEVER,
    RBTDGC_ACCOUNT_ASSAY,
    RBTDGC_ACCOUNT_DIRECTOR,
    RBTDGC_ACCOUNT_GOVERNOR,
    RBTDGC_ACCOUNT_RETRIEVER,
};
use crate::rbtdrm_manifest::{
    rbtdrm_credential_check_colophon,
    RBTDRM_FIXTURE_FREEHOLD_ENROBE,
};
use crate::rbtdrk_freehold::{
    rbtdrk_freehold_rbra,
    rbtdrk_invoke_logged,
    rbtdrk_probe_freehold_moniker,
    rbtdrk_probe_governor_rbra,
    RBTDRK_IDENTITY_DIRECTOR,
    RBTDRK_IDENTITY_RETRIEVER,
};

/// Fact-file name for the governor SA email (mirror of RBGP_FACT_GOVERNOR_SA_EMAIL
/// from rbgc_constants.sh). Read from the enrobe invocation's BURV output.
const RBTDRK_FACT_GOVERNOR_SA_EMAIL: &str = "rbgp_fact_governor_sa_email";

/// Governor enrobe. Enrobes governor against the freehold depot, reads the SA
/// email fact, copies governor RBRA from BURV output to the freehold path under
/// RBRR_SECRETS_DIR.
fn rbtdrk_governor_enrobe(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "freehold depot moniker installed",
        check: rbtdrk_probe_freehold_moniker,
        remediation: "ensure a depot is levied (freehold-establish or an operator hand-levy)",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_governor_enrobe_impl(ctx, dir))
}

fn rbtdrk_governor_enrobe_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let assay = match rbtdrk_freehold_rbra(&root, RBTDGC_ACCOUNT_ASSAY) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("freehold assay RBRA path: {}", e)),
    };

    let enrobe = match rbtdrk_invoke_logged(
        ctx,
        RBTDGC_ENROBE_GOVERNOR,
        &[],
        &[],
        dir,
        "enrobe",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("governor enrobe: {}", e)),
    };
    if enrobe.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "governor enrobe exit {}\n{}",
            enrobe.exit_code, enrobe.stderr
        ));
    }

    let email = match rbtdri_read_burv_fact(&enrobe, RBTDRK_FACT_GOVERNOR_SA_EMAIL) {
        Ok(s) => s,
        Err(e) => return rbtdre_Verdict::Fail(format!("read governor SA email fact: {}", e)),
    };
    let _ = std::fs::write(dir.join("governor-sa-email.txt"), &email);

    if !assay.exists() {
        return rbtdre_Verdict::Fail(format!(
            "assay RBRA absent after governor enrobe: {}",
            assay.display()
        ));
    }

    let freehold = match rbtdrk_freehold_rbra(&root, RBTDGC_ACCOUNT_GOVERNOR) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("freehold governor RBRA path: {}", e)),
    };
    if let Some(parent) = freehold.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return rbtdre_Verdict::Fail(format!(
                "create governor RBRA dir {}: {}",
                parent.display(),
                e
            ));
        }
    }
    if let Err(e) = std::fs::copy(&assay, &freehold) {
        return rbtdre_Verdict::Fail(format!(
            "copy assay RBRA → governor freehold {}: {}",
            freehold.display(),
            e
        ));
    }
    rbtdre_Verdict::Pass
}

/// Retriever enrobe + access-probe. The access-probe doubles as the
/// IAM-propagation gate: the enrobe tabtarget is responsible for waiting on
/// propagation before exiting (or the probe iterates internally).
fn rbtdrk_retriever_enrobe(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_enrobe or the full freehold-enrobe fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| {
        rbtdrk_role_enrobe_impl(
            ctx,
            dir,
            RBTDGC_ENROBE_RETRIEVER,
            RBTDRK_IDENTITY_RETRIEVER,
            RBTDGC_ACCOUNT_RETRIEVER,
        )
    })
}

/// Director enrobe + access-probe.
fn rbtdrk_director_enrobe(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_enrobe or the full freehold-enrobe fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| {
        rbtdrk_role_enrobe_impl(
            ctx,
            dir,
            RBTDGC_ENROBE_DIRECTOR,
            RBTDRK_IDENTITY_DIRECTOR,
            RBTDGC_ACCOUNT_DIRECTOR,
        )
    })
}

/// Shared defrock body: invoke the defrock colophon for the identity. Exercises
/// rbgg_defrock_*'s revoke-before-delete and poll_until_gone debounce, so the
/// following enrobe hits the create branch against a durably-gone SA.
fn rbtdrk_role_defrock_impl(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    defrock_colophon: &str,
    identity: &str,
    role: &str,
) -> rbtdre_Verdict {
    let label = format!("defrock-{}", role);
    let defrock = match rbtdrk_invoke_logged(ctx, defrock_colophon, &[identity], &[], dir, &label) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("defrock {}: {}", role, e)),
    };
    if defrock.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "defrock {} exit {}\n{}",
            role, defrock.exit_code, defrock.stderr
        ));
    }
    rbtdre_Verdict::Pass
}

/// Director defrock. Clears the standing director SA (revoke bindings → delete)
/// so the following director enrobe exercises the create branch.
fn rbtdrk_director_defrock(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_enrobe or the full freehold-enrobe fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| {
        rbtdrk_role_defrock_impl(
            ctx,
            dir,
            RBTDGC_DEFROCK_DIRECTOR,
            RBTDRK_IDENTITY_DIRECTOR,
            RBTDGC_ACCOUNT_DIRECTOR,
        )
    })
}

/// Retriever defrock. Clears the standing retriever SA (revoke bindings →
/// delete) so the following retriever enrobe exercises the create branch.
fn rbtdrk_retriever_defrock(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_enrobe or the full freehold-enrobe fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| {
        rbtdrk_role_defrock_impl(
            ctx,
            dir,
            RBTDGC_DEFROCK_RETRIEVER,
            RBTDRK_IDENTITY_RETRIEVER,
            RBTDGC_ACCOUNT_RETRIEVER,
        )
    })
}

/// Shared enrobe body for retriever and director: enrobe → assert assay
/// dropped by the enrobe tabtarget → copy to freehold role path → access-probe.
fn rbtdrk_role_enrobe_impl(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    enrobe_colophon: &str,
    identity: &str,
    role: &str,
) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let assay = match rbtdrk_freehold_rbra(&root, RBTDGC_ACCOUNT_ASSAY) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("freehold assay RBRA path: {}", e)),
    };

    // Enrobe is idempotent (RBSRK/RBSDK): a standing-depot rerun rotates the key
    // on the existing SA; a freshly-levied depot creates it. Either way the enrobe
    // tabtarget drops the assay RBRA.

    let label_enrobe = format!("enrobe-{}", role);
    let enrobe = match rbtdrk_invoke_logged(
        ctx,
        enrobe_colophon,
        &[identity],
        &[],
        dir,
        &label_enrobe,
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("enrobe {}: {}", role, e)),
    };
    if enrobe.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "enrobe {} exit {}\n{}",
            role, enrobe.exit_code, enrobe.stderr
        ));
    }

    if !assay.exists() {
        return rbtdre_Verdict::Fail(format!(
            "assay RBRA absent after enrobe-{}: {}",
            role,
            assay.display()
        ));
    }

    let freehold = match rbtdrk_freehold_rbra(&root, role) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("freehold {} RBRA path: {}", role, e)),
    };
    if let Some(parent) = freehold.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return rbtdre_Verdict::Fail(format!(
                "create {} RBRA dir {}: {}",
                role,
                parent.display(),
                e
            ));
        }
    }
    if let Err(e) = std::fs::copy(&assay, &freehold) {
        return rbtdre_Verdict::Fail(format!(
            "copy assay RBRA → {} freehold {}: {}",
            role,
            freehold.display(),
            e
        ));
    }

    let probe_colophon = match rbtdrm_credential_check_colophon(role) {
        Some(c) => c,
        None => return rbtdre_Verdict::Fail(format!("unknown credential role: {}", role)),
    };
    let probe_result = match rbtdri_invoke_global(ctx, probe_colophon, &[], &[]) {
        Ok(r) => r,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!("access-probe {} invocation: {}", role, e))
        }
    };
    let _ = std::fs::write(
        dir.join(format!("probe-{}-stdout.txt", role)),
        &probe_result.stdout,
    );
    let _ = std::fs::write(
        dir.join(format!("probe-{}-stderr.txt", role)),
        &probe_result.stderr,
    );
    if probe_result.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "access-probe {} exit {}\n{}",
            role, probe_result.exit_code, probe_result.stderr
        ));
    }

    rbtdre_Verdict::Pass
}

// ── Section registry ─────────────────────────────────────────

// freehold-enrobe — the no-levy recycle variant shared by skirmish, dogfight,
// and blockade. Runs against a depot the operator has levied by hand (no GCP
// project created per run). After re-mantling the governor it defrocks then
// re-enrobes retriever + director, so every run exercises the full teardown →
// re-enrobe cycle: rbgg_defrock_*'s revoke-before-delete and poll_until_gone
// debounce, then the enrobe create branch against a durably-gone SA. Defrock
// runs in reverse role order, enrobe in forward order.
pub static RBTDRK_CASES_FREEHOLD_ENROBE: &[rbtdre_Case] = &[
    case!(rbtdrk_governor_enrobe),
    case!(rbtdrk_director_defrock),
    case!(rbtdrk_retriever_defrock),
    case!(rbtdrk_retriever_enrobe),
    case!(rbtdrk_director_enrobe),
];

pub static RBTDRK_FIXTURE_FREEHOLD_ENROBE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_FREEHOLD_ENROBE,
    disposition: rbtdre_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    cases: RBTDRK_CASES_FREEHOLD_ENROBE,
    credless: false,
};
