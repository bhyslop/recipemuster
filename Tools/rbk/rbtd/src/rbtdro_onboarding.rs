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
// RBTDRO — gauntlet canonical-onboarding-sequence fixture (§3 of release
// qualification)
//
// Each case walks the operator-facing onboarding handbook track for one
// vessel-construction mode, invoking the handbook's prescribed tabtargets
// in the prescribed order:
//
//   1. inscribe_reliquary — `rbw-dI` (depot-wide toolchain mirror)
//   2. ordain_conjure     — rbw-Odf walk: yoke + ordain rbev-sentry-deb-tether
//   3. ordain_airgap      — rbw-Oda walk: enshrine, conjure forge tethered,
//                            re-enshrine, conjure airgap from forge
//   4. ordain_bind        — rbw-Odb walk: ordain rbev-bottle-plantuml
//   5. ordain_graft       — rbw-Odg walk: docker pull, BURE_TWEAK override,
//                            ordain rbev-graft-demo
//
// Disposition: StateProgressing. Build-only — no charge, no test. Cases stop
// when each handbook-prescribed hallmark lands in GAR. Per-case precondition
// probes enable a-la-carte single-case rerun.
//
// The reliquary stamp captured by case 1 is persisted to a scratch file
// under .rbk/.gauntlet/ so cases 2 and 3 (the only cases that need it) can
// read it back without depending on case-1's BURV invocation context.

use std::path::{Path, PathBuf};

use crate::case;
use crate::rbtdrb_probe::{rbtdrb_assert, rbtdrb_Probe};
use crate::rbtdrc_crucible::{rbtdrc_with_ctx, RBTDRC_FACT_HALLMARK, RBTDRC_FACT_RELIQUARY};
use crate::rbtdre_engine::{rbtdre_Disposition, rbtdre_Fixture, rbtdre_Section, rbtdre_Verdict};
use crate::rbtdri_invocation::{
    rbtdri_invoke_global, rbtdri_read_burv_fact, rbtdri_Context, rbtdri_InvokeResult,
};
use crate::rbtdrk_canonical::rbtdrk_canonical_rbra;
use crate::rbtdrm_manifest::{
    RBTDRM_COLOPHON_ENSHRINE_VESSEL, RBTDRM_COLOPHON_INSCRIBE_RELIQUARY, RBTDRM_COLOPHON_ORDAIN,
    RBTDRM_COLOPHON_YOKE_RELIQUARY, RBTDRM_FIXTURE_CANONICAL_ONBOARDING_SEQUENCE,
};

// ── Vessel identities ────────────────────────────────────────

/// Vessel directories passed positionally to global tabtargets (rbw-fO, rbw-dY).
const RBTDRO_VESSEL_DIR_CONJURE: &str = "rbev-vessels/rbev-sentry-deb-tether";
const RBTDRO_VESSEL_DIR_AIRGAP_FORGE: &str = "rbev-vessels/rbev-bottle-ifrit-forge";
const RBTDRO_VESSEL_DIR_AIRGAP: &str = "rbev-vessels/rbev-bottle-ifrit-airgap";
const RBTDRO_VESSEL_DIR_BIND: &str = "rbev-vessels/rbev-bottle-plantuml";
const RBTDRO_VESSEL_DIR_GRAFT: &str = "rbev-vessels/rbev-graft-demo";

// ── Reliquary scratch state ──────────────────────────────────

/// Fixture-private scratch path under the regime state dir. Carries the
/// reliquary stamp captured by case 1 forward to cases 2 and 3 across
/// per-case BURV-invocation isolation.
const RBTDRO_SCRATCH_RELIQUARY_REL: &str = ".rbk/.gauntlet/reliquary-stamp.txt";

// ── Graft override ───────────────────────────────────────────

/// BURE_TWEAK signal recognized by rbfd_FoundryDirectorBuild for graft mode:
/// supplies the local image reference without mutating the vessel rbrv.env.
const RBTDRO_GRAFT_TWEAK_NAME: &str = "threemodegraft";

/// Source image for graft: any locally-resolvable docker image works; an
/// upstream public image keeps this case independent of the conjure case.
const RBTDRO_GRAFT_SOURCE_IMAGE: &str = "busybox:latest";

// ── Probes ───────────────────────────────────────────────────
//
// Probes are pure `fn() -> Result<(), String>` per the rbtdrb_Probe shape and
// have no context, so they read the project root from current_dir() — theurge
// always launches from the project root.

fn rbtdro_probe_root() -> Result<PathBuf, String> {
    std::env::current_dir().map_err(|e| format!("cannot resolve project root: {}", e))
}

fn rbtdro_scratch_reliquary_path(root: &Path) -> PathBuf {
    root.join(RBTDRO_SCRATCH_RELIQUARY_REL)
}

/// Cases 1, 4, 5 probe: governor RBRA present at the canonical secrets path.
/// Established by canonical-establish §2 (rbtdrk_governor_mantle).
fn rbtdro_probe_governor_rbra() -> Result<(), String> {
    let root = rbtdro_probe_root()?;
    let path = rbtdrk_canonical_rbra(&root, "governor")?;
    if !path.exists() {
        return Err(format!("governor RBRA absent at {}", path.display()));
    }
    Ok(())
}

/// Cases 2 and 3 probe: reliquary stamp scratch file present and non-empty.
/// Established by case 1's inscribe; the scratch file lets per-vessel ordain
/// cases discover the stamp without re-walking depot inventory.
fn rbtdro_probe_reliquary_stamp() -> Result<(), String> {
    let root = rbtdro_probe_root()?;
    let path = rbtdro_scratch_reliquary_path(&root);
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("reliquary scratch '{}' unreadable: {}", path.display(), e))?;
    if content.trim().is_empty() {
        return Err(format!("reliquary scratch '{}' is empty", path.display()));
    }
    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────

fn rbtdro_invoke_logged(
    ctx: &mut rbtdri_Context,
    colophon: &str,
    args: &[&str],
    extra_env: &[(&str, &str)],
    dir: &Path,
    label: &str,
) -> Result<rbtdri_InvokeResult, String> {
    let result = rbtdri_invoke_global(ctx, colophon, args, extra_env)?;
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &result.stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &result.stderr);
    Ok(result)
}

/// Run an ordain on `vessel_dir` and return the captured hallmark string.
/// Writes invocation logs to `dir` under the `label` prefix; returns Fail
/// verdict-bearing Err so callers can early-return cleanly.
fn rbtdro_ordain_capture(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    vessel_dir: &str,
    extra_env: &[(&str, &str)],
    label: &str,
) -> Result<String, rbtdre_Verdict> {
    let result = match rbtdro_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_ORDAIN,
        &[vessel_dir],
        extra_env,
        dir,
        label,
    ) {
        Ok(r) => r,
        Err(e) => {
            return Err(rbtdre_Verdict::Fail(format!(
                "ordain {} invocation: {}",
                vessel_dir, e
            )))
        }
    };
    if result.exit_code != 0 {
        return Err(rbtdre_Verdict::Fail(format!(
            "ordain {} exit {}\n{}",
            vessel_dir, result.exit_code, result.stderr
        )));
    }
    let hallmark = match rbtdri_read_burv_fact(&result, RBTDRC_FACT_HALLMARK) {
        Ok(s) => s,
        Err(e) => {
            return Err(rbtdre_Verdict::Fail(format!(
                "read hallmark fact after ordain {}: {}",
                vessel_dir, e
            )))
        }
    };
    let _ = std::fs::write(dir.join(format!("{}-hallmark.txt", label)), &hallmark);
    Ok(hallmark)
}

/// Yoke the reliquary stamp into a vessel's rbrv.env. The yoke tabtarget
/// validates the stamp and writes RBRV_RELIQUARY; the handbook ceremony
/// includes a commit step the operator performs by hand, which the fixture
/// skips since the yoke value is rerun-derivable from the scratch file.
fn rbtdro_yoke(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    stamp: &str,
    vessel_dir: &str,
    label: &str,
) -> Result<(), rbtdre_Verdict> {
    let result = match rbtdro_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_YOKE_RELIQUARY,
        &[stamp, vessel_dir],
        &[],
        dir,
        label,
    ) {
        Ok(r) => r,
        Err(e) => {
            return Err(rbtdre_Verdict::Fail(format!(
                "yoke {} invocation: {}",
                vessel_dir, e
            )))
        }
    };
    if result.exit_code != 0 {
        return Err(rbtdre_Verdict::Fail(format!(
            "yoke {} exit {}\n{}",
            vessel_dir, result.exit_code, result.stderr
        )));
    }
    Ok(())
}

/// Sweep enshrine across all vessel rbrv.env files. Idempotent: already-
/// enshrined upstreams are skipped. The airgap chain calls this twice —
/// first to mirror the upstream rust base, then to populate the airgap
/// vessel's base anchor from the freshly-conjured forge hallmark.
fn rbtdro_enshrine(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    label: &str,
) -> Result<(), rbtdre_Verdict> {
    let result = match rbtdro_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_ENSHRINE_VESSEL,
        &[],
        &[],
        dir,
        label,
    ) {
        Ok(r) => r,
        Err(e) => return Err(rbtdre_Verdict::Fail(format!("enshrine invocation: {}", e))),
    };
    if result.exit_code != 0 {
        return Err(rbtdre_Verdict::Fail(format!(
            "enshrine exit {}\n{}",
            result.exit_code, result.stderr
        )));
    }
    Ok(())
}

/// Pull a docker image. Used by the graft case to ensure the source image
/// is locally resolvable before the BURE_TWEAK-overridden ordain.
fn rbtdro_docker_pull(image_ref: &str) -> Result<(), String> {
    let output = std::process::Command::new("docker")
        .args(["pull", image_ref])
        .output()
        .map_err(|e| format!("docker pull exec failed: {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "docker pull {} exited {}: {}",
            image_ref,
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

// ── Case 1: inscribe-reliquary ───────────────────────────────

/// Inscribe the depot-wide reliquary toolchain. Captures the reliquary
/// stamp from BURV fact, persists it to the fixture scratch file so
/// downstream conjure/airgap cases can yoke the same stamp into their
/// vessels without re-deriving it from depot inventory.
fn rbtdro_inscribe_reliquary(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdro_probe_governor_rbra,
        remediation: "rerun canonical-establish (rbtdrk_governor_mantle) before this fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_inscribe_reliquary_impl(ctx, dir))
}

fn rbtdro_inscribe_reliquary_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let result = match rbtdro_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_INSCRIBE_RELIQUARY,
        &[],
        &[],
        dir,
        "inscribe",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("inscribe invocation: {}", e)),
    };
    if result.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "inscribe exit {}\n{}",
            result.exit_code, result.stderr
        ));
    }

    let stamp = match rbtdri_read_burv_fact(&result, RBTDRC_FACT_RELIQUARY) {
        Ok(s) => s,
        Err(e) => return rbtdre_Verdict::Fail(format!("read reliquary fact: {}", e)),
    };
    let _ = std::fs::write(dir.join("reliquary-stamp.txt"), &stamp);

    let scratch = rbtdro_scratch_reliquary_path(&root);
    if let Some(parent) = scratch.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return rbtdre_Verdict::Fail(format!(
                "create scratch dir {}: {}",
                parent.display(),
                e
            ));
        }
    }
    if let Err(e) = std::fs::write(&scratch, &stamp) {
        return rbtdre_Verdict::Fail(format!(
            "write reliquary scratch {}: {}",
            scratch.display(),
            e
        ));
    }
    rbtdre_Verdict::Pass
}

// ── Case 2: ordain-conjure (rbw-Odf walk) ────────────────────

/// Yoke the reliquary stamp into rbev-sentry-deb-tether and ordain.
/// Mirrors rbw-Odf step 1 (yoke) and step 2 (ordain).
fn rbtdro_ordain_conjure(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_inscribe_reliquary before this case",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_ordain_conjure_impl(ctx, dir))
}

fn rbtdro_ordain_conjure_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();
    let stamp = match std::fs::read_to_string(rbtdro_scratch_reliquary_path(&root)) {
        Ok(s) => s.trim().to_string(),
        Err(e) => return rbtdre_Verdict::Fail(format!("read reliquary scratch: {}", e)),
    };

    if let Err(v) = rbtdro_yoke(ctx, dir, &stamp, RBTDRO_VESSEL_DIR_CONJURE, "yoke-conjure") {
        return v;
    }

    match rbtdro_ordain_capture(ctx, dir, RBTDRO_VESSEL_DIR_CONJURE, &[], "ordain-conjure") {
        Ok(_) => rbtdre_Verdict::Pass,
        Err(v) => v,
    }
}

// ── Case 3: ordain-airgap (rbw-Oda walk) ─────────────────────

/// Walk the airgap supply chain: enshrine upstream rust base, yoke the
/// reliquary into the forge vessel, conjure the forge tethered, re-enshrine
/// to populate the airgap vessel's base anchor from the freshly-built forge,
/// yoke the reliquary into the airgap vessel, conjure the airgap bottle.
/// Mirrors rbw-Oda steps 1-3.
fn rbtdro_ordain_airgap(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_inscribe_reliquary before this case",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_ordain_airgap_impl(ctx, dir))
}

fn rbtdro_ordain_airgap_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();
    let stamp = match std::fs::read_to_string(rbtdro_scratch_reliquary_path(&root)) {
        Ok(s) => s.trim().to_string(),
        Err(e) => return rbtdre_Verdict::Fail(format!("read reliquary scratch: {}", e)),
    };

    if let Err(v) = rbtdro_enshrine(ctx, dir, "enshrine-upstream") {
        return v;
    }

    if let Err(v) = rbtdro_yoke(
        ctx,
        dir,
        &stamp,
        RBTDRO_VESSEL_DIR_AIRGAP_FORGE,
        "yoke-forge",
    ) {
        return v;
    }
    if let Err(v) =
        rbtdro_ordain_capture(ctx, dir, RBTDRO_VESSEL_DIR_AIRGAP_FORGE, &[], "ordain-forge")
    {
        return v;
    }

    if let Err(v) = rbtdro_enshrine(ctx, dir, "enshrine-forge") {
        return v;
    }

    if let Err(v) = rbtdro_yoke(ctx, dir, &stamp, RBTDRO_VESSEL_DIR_AIRGAP, "yoke-airgap") {
        return v;
    }
    match rbtdro_ordain_capture(ctx, dir, RBTDRO_VESSEL_DIR_AIRGAP, &[], "ordain-airgap") {
        Ok(_) => rbtdre_Verdict::Pass,
        Err(v) => v,
    }
}

// ── Case 4: ordain-bind (rbw-Odb walk) ───────────────────────

/// Pin upstream PlantUML by digest. Bind mode reads RBRV_BIND_IMAGE from
/// rbev-bottle-plantuml/rbrv.env and mirrors the digest into GAR — no
/// reliquary, no Dockerfile, no Cloud Build. Mirrors rbw-Odb step 2.
/// (The handbook's step 1 readying a sentry hallmark is run-side and is
/// covered by canonical-establish leaving the conjured sentry from case 2.)
fn rbtdro_ordain_bind(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdro_probe_governor_rbra,
        remediation: "rerun canonical-establish (rbtdrk_governor_mantle) before this fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_ordain_bind_impl(ctx, dir))
}

fn rbtdro_ordain_bind_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    match rbtdro_ordain_capture(ctx, dir, RBTDRO_VESSEL_DIR_BIND, &[], "ordain-bind") {
        Ok(_) => rbtdre_Verdict::Pass,
        Err(v) => v,
    }
}

// ── Case 5: ordain-graft (rbw-Odg walk) ──────────────────────

/// Pull the graft source image, then ordain rbev-graft-demo with a
/// BURE_TWEAK overriding RBRV_GRAFT_IMAGE in-process. The override avoids
/// mutating rbev-graft-demo/rbrv.env so the fixture leaves no commit-worthy
/// regime drift behind. Mirrors rbw-Odg steps 1-2 (the handbook's docker
/// pull/tag + RBRV_GRAFT_IMAGE write fold into the BURE_TWEAK override).
fn rbtdro_ordain_graft(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdro_probe_governor_rbra,
        remediation: "rerun canonical-establish (rbtdrk_governor_mantle) before this fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_ordain_graft_impl(ctx, dir))
}

fn rbtdro_ordain_graft_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    if let Err(e) = rbtdro_docker_pull(RBTDRO_GRAFT_SOURCE_IMAGE) {
        return rbtdre_Verdict::Fail(format!(
            "docker pull {}: {}",
            RBTDRO_GRAFT_SOURCE_IMAGE, e
        ));
    }
    let extra_env: &[(&str, &str)] = &[
        ("BURE_TWEAK_NAME", RBTDRO_GRAFT_TWEAK_NAME),
        ("BURE_TWEAK_VALUE", RBTDRO_GRAFT_SOURCE_IMAGE),
    ];
    match rbtdro_ordain_capture(ctx, dir, RBTDRO_VESSEL_DIR_GRAFT, extra_env, "ordain-graft") {
        Ok(_) => rbtdre_Verdict::Pass,
        Err(v) => v,
    }
}

// ── Section registry ─────────────────────────────────────────

pub static RBTDRO_SECTIONS_CANONICAL_ONBOARDING_SEQUENCE: &[rbtdre_Section] = &[rbtdre_Section {
    name: "canonical-onboarding-arc",
    cases: &[
        case!(rbtdro_inscribe_reliquary),
        case!(rbtdro_ordain_conjure),
        case!(rbtdro_ordain_airgap),
        case!(rbtdro_ordain_bind),
        case!(rbtdro_ordain_graft),
    ],
}];

pub static RBTDRO_FIXTURE_CANONICAL_ONBOARDING_SEQUENCE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_CANONICAL_ONBOARDING_SEQUENCE,
    disposition: rbtdre_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    sections: RBTDRO_SECTIONS_CANONICAL_ONBOARDING_SEQUENCE,
};
