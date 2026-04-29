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
// RBTDRO — gauntlet onboarding-sequence fixture (§3 of release qualification)
//
// Each case walks the operator-facing onboarding handbook track for one
// vessel-construction mode, invoking the handbook's prescribed tabtargets
// in the prescribed order:
//
//   1. inscribe_reliquary  — `rbw-dI` (depot-wide toolchain mirror) +
//                             yoke stamp into all ordain-side vessels + auto-commit
//   2. enshrine_bases      — `rbw-dE` (global enshrine sweep) + auto-commit
//   3. kludge_tadmor       — local docker build for tadmor sentry + bottle
//   4. kludge_ccyolo       — local docker build for ccyolo sentry + bottle
//   5. ordain_conjure      — ordain rbev-sentry-deb-tether + propagate to consumers
//   6. conjure_srjcl       — ordain rbev-bottle-anthropic-jupyter + propagate to srjcl
//   7. ordain_airgap       — airgap supply chain: enshrine, forge, enshrine, airgap
//   8. ordain_bind         — bind rbev-bottle-plantuml + propagate to pluml
//   9. ordain_graft        — docker pull + BURE_TWEAK ordain rbev-graft-demo
//
// Disposition: StateProgressing. Build-only — no charge, no test. Cases stop
// when each handbook-prescribed hallmark lands in GAR. Per-case precondition
// probes enable a-la-carte single-case rerun.
//
// The reliquary stamp captured by case 1 is persisted to a scratch file
// under .rbk/.gauntlet/ so downstream cases can verify the stamp landed
// without re-walking depot inventory.

use std::io::{BufRead, BufReader, Write};
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
    RBTDRM_COLOPHON_ENSHRINE_VESSEL, RBTDRM_COLOPHON_INSCRIBE_RELIQUARY,
    RBTDRM_COLOPHON_KLUDGE_BOTTLE, RBTDRM_COLOPHON_KLUDGE_SENTRY, RBTDRM_COLOPHON_ORDAIN,
    RBTDRM_COLOPHON_YOKE_RELIQUARY, RBTDRM_FIXTURE_ONBOARDING_SEQUENCE,
};

// ── Vessel directories ────────────────────────────────────────

const RBTDRO_VESSEL_DIR_SENTRY_TETHER: &str = "rbev-vessels/rbev-sentry-deb-tether";
const RBTDRO_VESSEL_DIR_AIRGAP_FORGE: &str = "rbev-vessels/rbev-bottle-ifrit-forge";
const RBTDRO_VESSEL_DIR_AIRGAP_BOTTLE: &str = "rbev-vessels/rbev-bottle-ifrit-airgap";
const RBTDRO_VESSEL_DIR_PLANTUML: &str = "rbev-vessels/rbev-bottle-plantuml";
const RBTDRO_VESSEL_DIR_JUPYTER: &str = "rbev-vessels/rbev-bottle-anthropic-jupyter";
const RBTDRO_VESSEL_DIR_GRAFT: &str = "rbev-vessels/rbev-graft-demo";

// ── Nameplate monikers ────────────────────────────────────────

const RBTDRO_NAMEPLATE_TADMOR: &str = "tadmor";
const RBTDRO_NAMEPLATE_CCYOLO: &str = "ccyolo";
const RBTDRO_NAMEPLATE_MORIAH: &str = "moriah";
const RBTDRO_NAMEPLATE_SRJCL: &str = "srjcl";
const RBTDRO_NAMEPLATE_PLUML: &str = "pluml";

// ── Nameplate hallmark variable names (lines in .rbk/<nameplate>/rbrn.env) ──

const RBTDRO_HALLMARK_VAR_BOTTLE: &str = "RBRN_BOTTLE_HALLMARK";
const RBTDRO_HALLMARK_VAR_SENTRY: &str = "RBRN_SENTRY_HALLMARK";

// ── Consumer arrays ───────────────────────────────────────────

/// Nameplates that receive the sentry-tether hallmark from ordain-conjure.
const RBTDRO_CONSUMERS_SENTRY_TETHER: &[&str] = &[
    RBTDRO_NAMEPLATE_MORIAH,
    RBTDRO_NAMEPLATE_SRJCL,
    RBTDRO_NAMEPLATE_PLUML,
];

/// Nameplates that receive the airgap-bottle hallmark from ordain-airgap.
const RBTDRO_CONSUMERS_AIRGAP_BOTTLE: &[&str] = &[RBTDRO_NAMEPLATE_MORIAH];

/// Nameplates that receive the plantuml-bottle hallmark from ordain-bind.
const RBTDRO_CONSUMERS_PLANTUML_BOTTLE: &[&str] = &[RBTDRO_NAMEPLATE_PLUML];

/// Nameplates that receive the jupyter-bottle hallmark from conjure-srjcl.
const RBTDRO_CONSUMERS_JUPYTER_BOTTLE: &[&str] = &[RBTDRO_NAMEPLATE_SRJCL];

// ── Yoke fan-out — inscribe-reliquary yoking all ordain-side vessels ─────────
//
// ccyolo has RBRV_RELIQUARY= in its rbrv.env (present, empty) — it is a
// conjure-mode vessel whose Dockerfile pulls from the reliquary tool images,
// so it needs the stamp at ordain time. Included.
//
// rbev-graft-demo also has RBRV_RELIQUARY= but graft-mode vessels do not use
// the reliquary toolchain (they push a pre-built image; no Dockerfile build).
// Excluded from yoke fan-out despite the presence of the field.

const RBTDRO_YOKE_VESSEL_DIRS: &[&str] = &[
    RBTDRO_VESSEL_DIR_SENTRY_TETHER,
    RBTDRO_VESSEL_DIR_AIRGAP_FORGE,
    RBTDRO_VESSEL_DIR_AIRGAP_BOTTLE,
    RBTDRO_VESSEL_DIR_PLANTUML,
    RBTDRO_VESSEL_DIR_JUPYTER,
    "rbev-vessels/rbev-bottle-ccyolo",
    // rbev-graft-demo excluded: graft mode does not use reliquary tool images
];

// ── Reliquary scratch state ──────────────────────────────────

/// Fixture-private scratch path under the regime state dir. Carries the
/// reliquary stamp captured by case 1 forward to downstream cases across
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

/// Cases 1, 2, 8, 9 probe: governor RBRA present at the canonical secrets path.
/// Established by canonical-establish §2 (rbtdrk_governor_mantle).
fn rbtdro_probe_governor_rbra() -> Result<(), String> {
    let root = rbtdro_probe_root()?;
    let path = rbtdrk_canonical_rbra(&root, "governor")?;
    if !path.exists() {
        return Err(format!("governor RBRA absent at {}", path.display()));
    }
    Ok(())
}

/// Cases 3-7 probe: reliquary stamp scratch file present and non-empty.
/// Established by case 1's inscribe; the scratch file lets per-vessel cases
/// verify case 1 ran without re-walking depot inventory.
/// Kludge is local-only (no GCP), so governor RBRA is not a load-bearing
/// precondition for kludge; the scratch presence confirms case 1 completed.
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

/// Drive a hallmark value into a nameplate's rbrn.env. Mirrors the
/// zrbob_drive_hallmark substitution logic from rbob_bottle.sh: load file
/// lines, replace the matching `VAR=*` line with the new value, atomic rename.
/// Returns Err if the variable line is not found (hard fail — configuration error).
fn rbtdro_drive_hallmark(
    root: &Path,
    nameplate: &str,
    var_name: &str,
    hallmark: &str,
) -> Result<(), String> {
    let rbrn_path = root.join(".rbk").join(nameplate).join("rbrn.env");
    let file = std::fs::File::open(&rbrn_path)
        .map_err(|e| format!("open rbrn.env for {}: {}", nameplate, e))?;

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .map_err(|e| format!("read rbrn.env for {}: {}", nameplate, e))?;

    let prefix = format!("{}=", var_name);
    let mut found = false;
    let rewritten: Vec<String> = lines
        .into_iter()
        .map(|line| {
            if line.starts_with(&prefix) {
                found = true;
                format!("{}={}", var_name, hallmark)
            } else {
                line
            }
        })
        .collect();

    if !found {
        return Err(format!(
            "variable {} not found in .rbk/{}/rbrn.env",
            var_name, nameplate
        ));
    }

    let tmp_path = root
        .join(".rbk")
        .join(nameplate)
        .join("rbrn.env.drive_tmp");
    {
        let mut tmp = std::fs::File::create(&tmp_path)
            .map_err(|e| format!("create tmp rbrn.env for {}: {}", nameplate, e))?;
        for line in &rewritten {
            writeln!(tmp, "{}", line)
                .map_err(|e| format!("write tmp rbrn.env for {}: {}", nameplate, e))?;
        }
    }
    std::fs::rename(&tmp_path, &rbrn_path)
        .map_err(|e| format!("atomic replace rbrn.env for {}: {}", nameplate, e))?;
    Ok(())
}

/// Auto-commit helper. Checks git status --porcelain; if the working tree is
/// clean, returns Ok without committing. Otherwise runs `git add -A` then
/// `git commit -m <message>`. Theurge launches from project root so no
/// explicit current_dir is needed.
fn rbtdro_git_commit(message: &str) -> Result<(), rbtdre_Verdict> {
    let status = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| {
            rbtdre_Verdict::Fail(format!("git status --porcelain exec failed: {}", e))
        })?;
    if !status.status.success() {
        return Err(rbtdre_Verdict::Fail(format!(
            "git status --porcelain exited {}",
            status.status.code().unwrap_or(-1)
        )));
    }
    let porcelain = String::from_utf8_lossy(&status.stdout);
    if porcelain.trim().is_empty() {
        // Nothing to commit — clean tree is not an error.
        return Ok(());
    }

    let add = std::process::Command::new("git")
        .args(["add", "-A"])
        .output()
        .map_err(|e| rbtdre_Verdict::Fail(format!("git add -A exec failed: {}", e)))?;
    if !add.status.success() {
        return Err(rbtdre_Verdict::Fail(format!(
            "git add -A exited {}: {}",
            add.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&add.stderr).trim()
        )));
    }

    let commit = std::process::Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .map_err(|e| rbtdre_Verdict::Fail(format!("git commit exec failed: {}", e)))?;
    if !commit.status.success() {
        return Err(rbtdre_Verdict::Fail(format!(
            "git commit exited {}: {}",
            commit.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&commit.stderr).trim()
        )));
    }
    Ok(())
}

/// Kludge helper: build sentry and bottle locally for a nameplate. Both steps
/// are local docker builds with no GCP dependency. The kludge tabtargets drive
/// hallmarks directly into the nameplate's rbrn.env via zrbob_drive_hallmark.
/// rbw-cKS and rbw-cKB are global (param1-channel) tabtargets — no imprint suffix,
/// nameplate passed as a positional argument.
fn rbtdro_kludge_nameplate(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    nameplate: &str,
) -> Result<(), rbtdre_Verdict> {
    let sentry_result = match rbtdro_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_KLUDGE_SENTRY,
        &[nameplate],
        &[],
        dir,
        &format!("kludge-sentry-{}", nameplate),
    ) {
        Ok(r) => r,
        Err(e) => {
            return Err(rbtdre_Verdict::Fail(format!(
                "kludge sentry {} invocation: {}",
                nameplate, e
            )))
        }
    };
    if sentry_result.exit_code != 0 {
        return Err(rbtdre_Verdict::Fail(format!(
            "kludge sentry {} exit {}\n{}",
            nameplate, sentry_result.exit_code, sentry_result.stderr
        )));
    }

    // Commit sentry hallmark before bottle kludge — kludge asserts clean tree.
    rbtdro_git_commit(&format!("BBAAu kludge-{}: sentry hallmark", nameplate))?;

    let bottle_result = match rbtdro_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_KLUDGE_BOTTLE,
        &[nameplate],
        &[],
        dir,
        &format!("kludge-bottle-{}", nameplate),
    ) {
        Ok(r) => r,
        Err(e) => {
            return Err(rbtdre_Verdict::Fail(format!(
                "kludge bottle {} invocation: {}",
                nameplate, e
            )))
        }
    };
    if bottle_result.exit_code != 0 {
        return Err(rbtdre_Verdict::Fail(format!(
            "kludge bottle {} exit {}\n{}",
            nameplate, bottle_result.exit_code, bottle_result.stderr
        )));
    }

    rbtdro_git_commit(&format!("BBAAu kludge-{}: bottle hallmark", nameplate))?;
    Ok(())
}

// ── Case 1: inscribe-reliquary ───────────────────────────────

/// Inscribe the depot-wide reliquary toolchain. Captures the reliquary
/// stamp from BURV fact, persists it to the fixture scratch file, then
/// yokes the stamp into all ordain-side vessels in one pass and auto-commits.
fn rbtdro_onboarding_inscribe_reliquary(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdro_probe_governor_rbra,
        remediation: "rerun canonical-establish (rbtdrk_governor_mantle) before this fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_inscribe_reliquary_impl(ctx, dir))
}

fn rbtdro_onboarding_inscribe_reliquary_impl(
    ctx: &mut rbtdri_Context,
    dir: &Path,
) -> rbtdre_Verdict {
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

    // Yoke stamp into all ordain-side vessels in one pass.
    for vessel_dir in RBTDRO_YOKE_VESSEL_DIRS {
        let short = vessel_dir.rsplit('/').next().unwrap_or(vessel_dir);
        let label = format!("yoke-{}", short);
        if let Err(v) = rbtdro_yoke(ctx, dir, &stamp, vessel_dir, &label) {
            return v;
        }
    }

    // Commit the rbrv.env changes for all yoked vessels.
    if let Err(v) = rbtdro_git_commit(
        "BBAAu inscribe-reliquary: yoke stamp into all ordain-side vessels",
    ) {
        return v;
    }

    rbtdre_Verdict::Pass
}

// ── Case 2: enshrine-bases ────────────────────────────────────

/// Sweep enshrine across all vessel rbrv.env files to populate base-image
/// anchors from upstream registries. Idempotent. Auto-commits resulting
/// rbrv.env anchor fields.
fn rbtdro_onboarding_enshrine_bases(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdro_probe_governor_rbra,
        remediation: "rerun canonical-establish (rbtdrk_governor_mantle) before this fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_enshrine_bases_impl(ctx, dir))
}

fn rbtdro_onboarding_enshrine_bases_impl(
    ctx: &mut rbtdri_Context,
    dir: &Path,
) -> rbtdre_Verdict {
    if let Err(v) = rbtdro_enshrine(ctx, dir, "enshrine-bases") {
        return v;
    }
    if let Err(v) = rbtdro_git_commit("BBAAu enshrine-bases: populate enshrined base anchors") {
        return v;
    }
    rbtdre_Verdict::Pass
}

// ── Case 3: kludge-tadmor ────────────────────────────────────

/// Build tadmor sentry and bottle locally. Kludge is local docker — no GCP.
/// Probe: reliquary scratch present (confirms case 1 completed).
fn rbtdro_onboarding_kludge_tadmor(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_onboarding_inscribe_reliquary before this case",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_kludge_tadmor_impl(ctx, dir))
}

fn rbtdro_onboarding_kludge_tadmor_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    match rbtdro_kludge_nameplate(ctx, dir, RBTDRO_NAMEPLATE_TADMOR) {
        Ok(()) => rbtdre_Verdict::Pass,
        Err(v) => v,
    }
}

// ── Case 4: kludge-ccyolo ────────────────────────────────────

/// Build ccyolo sentry and bottle locally. Kludge is local docker — no GCP.
/// Probe: reliquary scratch present (confirms case 1 completed).
fn rbtdro_onboarding_kludge_ccyolo(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_onboarding_inscribe_reliquary before this case",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_kludge_ccyolo_impl(ctx, dir))
}

fn rbtdro_onboarding_kludge_ccyolo_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    match rbtdro_kludge_nameplate(ctx, dir, RBTDRO_NAMEPLATE_CCYOLO) {
        Ok(()) => rbtdre_Verdict::Pass,
        Err(v) => v,
    }
}

// ── Case 5: ordain-conjure (rbw-Odf walk) ────────────────────

/// Ordain rbev-sentry-deb-tether (conjure mode). Case 1 yoked the reliquary
/// stamp into the vessel. Propagates the resulting hallmark to all sentry-tether
/// consumers (moriah, srjcl, pluml) via RBRN_SENTRY_HALLMARK.
fn rbtdro_onboarding_ordain_conjure(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_onboarding_inscribe_reliquary before this case",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_ordain_conjure_impl(ctx, dir))
}

fn rbtdro_onboarding_ordain_conjure_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let hallmark = match rbtdro_ordain_capture(
        ctx,
        dir,
        RBTDRO_VESSEL_DIR_SENTRY_TETHER,
        &[],
        "ordain-conjure",
    ) {
        Ok(h) => h,
        Err(v) => return v,
    };

    for nameplate in RBTDRO_CONSUMERS_SENTRY_TETHER {
        if let Err(e) =
            rbtdro_drive_hallmark(&root, nameplate, RBTDRO_HALLMARK_VAR_SENTRY, &hallmark)
        {
            return rbtdre_Verdict::Fail(format!(
                "drive sentry hallmark into {}: {}",
                nameplate, e
            ));
        }
    }

    if let Err(v) = rbtdro_git_commit(
        "BBAAu ordain-conjure: sentry-tether hallmark + propagate to consumers",
    ) {
        return v;
    }

    rbtdre_Verdict::Pass
}

// ── Case 6: conjure-srjcl (jupyter bottle) ───────────────────

/// Ordain rbev-bottle-anthropic-jupyter (conjure mode). Propagates the
/// resulting hallmark to srjcl via RBRN_BOTTLE_HALLMARK.
fn rbtdro_onboarding_conjure_srjcl(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_onboarding_inscribe_reliquary before this case",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_conjure_srjcl_impl(ctx, dir))
}

fn rbtdro_onboarding_conjure_srjcl_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let hallmark = match rbtdro_ordain_capture(
        ctx,
        dir,
        RBTDRO_VESSEL_DIR_JUPYTER,
        &[],
        "ordain-jupyter",
    ) {
        Ok(h) => h,
        Err(v) => return v,
    };

    for nameplate in RBTDRO_CONSUMERS_JUPYTER_BOTTLE {
        if let Err(e) =
            rbtdro_drive_hallmark(&root, nameplate, RBTDRO_HALLMARK_VAR_BOTTLE, &hallmark)
        {
            return rbtdre_Verdict::Fail(format!(
                "drive jupyter hallmark into {}: {}",
                nameplate, e
            ));
        }
    }

    if let Err(v) =
        rbtdro_git_commit("BBAAu conjure-srjcl: jupyter-bottle hallmark + propagate to srjcl")
    {
        return v;
    }

    rbtdre_Verdict::Pass
}

// ── Case 7: ordain-airgap (rbw-Oda walk) ─────────────────────

/// Walk the airgap supply chain: enshrine upstream rust base, conjure the
/// forge tethered, re-enshrine to populate the airgap vessel's base anchor
/// from the freshly-built forge, conjure the airgap bottle.
/// Case 1 yoked the reliquary stamp into both forge and airgap vessels.
/// Propagates airgap-bottle hallmark to moriah via RBRN_BOTTLE_HALLMARK.
fn rbtdro_onboarding_ordain_airgap(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_onboarding_inscribe_reliquary before this case",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_ordain_airgap_impl(ctx, dir))
}

fn rbtdro_onboarding_ordain_airgap_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    if let Err(v) = rbtdro_enshrine(ctx, dir, "enshrine-upstream") {
        return v;
    }

    // Forge is intermediate — hallmark captured but has no consumer.
    if let Err(v) =
        rbtdro_ordain_capture(ctx, dir, RBTDRO_VESSEL_DIR_AIRGAP_FORGE, &[], "ordain-forge")
    {
        return v;
    }

    if let Err(v) = rbtdro_enshrine(ctx, dir, "enshrine-forge") {
        return v;
    }

    let airgap_hallmark = match rbtdro_ordain_capture(
        ctx,
        dir,
        RBTDRO_VESSEL_DIR_AIRGAP_BOTTLE,
        &[],
        "ordain-airgap",
    ) {
        Ok(h) => h,
        Err(v) => return v,
    };

    for nameplate in RBTDRO_CONSUMERS_AIRGAP_BOTTLE {
        if let Err(e) =
            rbtdro_drive_hallmark(&root, nameplate, RBTDRO_HALLMARK_VAR_BOTTLE, &airgap_hallmark)
        {
            return rbtdre_Verdict::Fail(format!(
                "drive airgap hallmark into {}: {}",
                nameplate, e
            ));
        }
    }

    if let Err(v) =
        rbtdro_git_commit("BBAAu ordain-airgap: airgap-bottle hallmark + propagate to moriah")
    {
        return v;
    }

    rbtdre_Verdict::Pass
}

// ── Case 8: ordain-bind (rbw-Odb walk) ───────────────────────

/// Pin upstream PlantUML by digest. Bind mode reads RBRV_BIND_IMAGE from
/// rbev-bottle-plantuml/rbrv.env and mirrors the digest into GAR — no
/// reliquary, no Dockerfile, no Cloud Build. Propagates plantuml hallmark
/// to pluml via RBRN_BOTTLE_HALLMARK.
fn rbtdro_onboarding_ordain_bind(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdro_probe_governor_rbra,
        remediation: "rerun canonical-establish (rbtdrk_governor_mantle) before this fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_ordain_bind_impl(ctx, dir))
}

fn rbtdro_onboarding_ordain_bind_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let hallmark = match rbtdro_ordain_capture(
        ctx,
        dir,
        RBTDRO_VESSEL_DIR_PLANTUML,
        &[],
        "ordain-bind",
    ) {
        Ok(h) => h,
        Err(v) => return v,
    };

    for nameplate in RBTDRO_CONSUMERS_PLANTUML_BOTTLE {
        if let Err(e) =
            rbtdro_drive_hallmark(&root, nameplate, RBTDRO_HALLMARK_VAR_BOTTLE, &hallmark)
        {
            return rbtdre_Verdict::Fail(format!(
                "drive plantuml hallmark into {}: {}",
                nameplate, e
            ));
        }
    }

    if let Err(v) =
        rbtdro_git_commit("BBAAu ordain-bind: plantuml-bottle hallmark + propagate to pluml")
    {
        return v;
    }

    rbtdre_Verdict::Pass
}

// ── Case 9: ordain-graft (rbw-Odg walk) ──────────────────────

/// Pull the graft source image, then ordain rbev-graft-demo with a
/// BURE_TWEAK overriding RBRV_GRAFT_IMAGE in-process. No consumers —
/// graft-demo is terminal.
fn rbtdro_onboarding_ordain_graft(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdro_probe_governor_rbra,
        remediation: "rerun canonical-establish (rbtdrk_governor_mantle) before this fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdro_onboarding_ordain_graft_impl(ctx, dir))
}

fn rbtdro_onboarding_ordain_graft_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
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
    let hallmark = match rbtdro_ordain_capture(
        ctx,
        dir,
        RBTDRO_VESSEL_DIR_GRAFT,
        extra_env,
        "ordain-graft",
    ) {
        Ok(h) => h,
        Err(v) => return v,
    };
    let _ = hallmark; // graft-demo is terminal — no consumer propagation

    if let Err(v) = rbtdro_git_commit("BBAAu ordain-graft: graft-demo hallmark") {
        return v;
    }

    rbtdre_Verdict::Pass
}

// ── Section registry ─────────────────────────────────────────

pub static RBTDRO_SECTIONS_ONBOARDING_SEQUENCE: &[rbtdre_Section] = &[rbtdre_Section {
    name: "onboarding-arc",
    cases: &[
        case!(rbtdro_onboarding_inscribe_reliquary),
        case!(rbtdro_onboarding_enshrine_bases),
        case!(rbtdro_onboarding_kludge_tadmor),
        case!(rbtdro_onboarding_kludge_ccyolo),
        case!(rbtdro_onboarding_ordain_conjure),
        case!(rbtdro_onboarding_conjure_srjcl),
        case!(rbtdro_onboarding_ordain_airgap),
        case!(rbtdro_onboarding_ordain_bind),
        case!(rbtdro_onboarding_ordain_graft),
    ],
}];

pub static RBTDRO_FIXTURE_ONBOARDING_SEQUENCE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_ONBOARDING_SEQUENCE,
    disposition: rbtdre_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    sections: RBTDRO_SECTIONS_ONBOARDING_SEQUENCE,
};
