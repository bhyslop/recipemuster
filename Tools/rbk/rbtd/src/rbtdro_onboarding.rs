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
// in the prescribed order. Case order and per-case docs live with the
// functions below; the registered order is the source of truth (see the
// `cases:` array in RBTDRO_SECTIONS_ONBOARDING_SEQUENCE).
//
// Disposition: StateProgressing. Build-only — no charge, no test. Cases stop
// when each handbook-prescribed hallmark lands in GAR. Per-case precondition
// probes enable a-la-carte single-case rerun.
//
// inscribe_reliquary yokes the reliquary stamp into vessel rbrv.env files
// and commits. Downstream cases verify it ran by reading RBRV_RELIQUARY from
// a stable yoked vessel's rbrv.env — no out-of-source-tree scratch state.

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
const RBTDRO_VESSEL_DIR_CCYOLO: &str = "rbev-vessels/rbev-bottle-ccyolo";

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
// All ordain-path vessels — conjure, bind, AND graft — require RBRV_RELIQUARY.
// The reliquary tool images feed the about/vouch pipeline in every mode
// (gcloud, docker, alpine, syft); bind also uses skopeo for image-copy. Yoke
// fan-out covers every vessel that ordain will visit.

const RBTDRO_YOKE_VESSEL_DIRS: &[&str] = &[
    RBTDRO_VESSEL_DIR_SENTRY_TETHER,
    RBTDRO_VESSEL_DIR_AIRGAP_FORGE,
    RBTDRO_VESSEL_DIR_AIRGAP_BOTTLE,
    RBTDRO_VESSEL_DIR_JUPYTER,
    RBTDRO_VESSEL_DIR_PLANTUML,
    RBTDRO_VESSEL_DIR_GRAFT,
    RBTDRO_VESSEL_DIR_CCYOLO,
];

/// Operation name for the yoke step — appears in error messages, the per-vessel
/// label prefix, and the inscribe-reliquary commit message. Single source so the
/// operation's vocabulary lives in one place.
const RBTDRO_OPERATION_YOKE: &str = "yoke";

// ── Hallmark-base locator construction ───────────────────────
//
// Module-local literals matching rbgc_Constants.sh values. Used to compose the
// airgap-bottle's RBRV_IMAGE_1_ANCHOR after the forge hallmark is captured.
// Orchestration writes a hallmark-namespace locator directly into the consumer
// vessel's rbrv.env; conjure resolves the locator at airgap-bottle build time.
// The forge hallmark's existence in GAR is established by ordain-forge's
// success — no separate enshrine validation step on the consumer.

const RBTDRO_GAR_CATEGORY_HALLMARKS: &str = "rbi_hm";
const RBTDRO_ARK_BASENAME_IMAGE: &str = "image";

/// Slot 1 of the airgap-bottle vessel — the only base-image slot the airgap
/// supply chain populates from a hallmark.
const RBTDRO_AIRGAP_BASE_ANCHOR_VAR: &str = "RBRV_IMAGE_1_ANCHOR";

// ── Reliquary stamp witness ──────────────────────────────────

/// Filename of a vessel's rbrv.env (relative to the vessel directory).
const RBTDRO_VESSEL_RBRV_FILE: &str = "rbrv.env";

/// Field name yoked by case 1 in each vessel rbrv.env. Presence (non-empty)
/// is the cross-case witness that case 1 ran.
const RBTDRO_FIELD_RBRV_RELIQUARY: &str = "RBRV_RELIQUARY";

/// Stable vessel chosen for the case-1 witness probe: sentry-tether is the
/// first entry in `RBTDRO_YOKE_VESSEL_DIRS` and is yoked on every conjure
/// path, so its rbrv.env always carries the stamp after case 1.
const RBTDRO_WITNESS_VESSEL_DIR: &str = RBTDRO_VESSEL_DIR_SENTRY_TETHER;

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

/// Read an env-file value or None if absent. Mirrors the helpers in rbtdrk
/// and rbtdrp — kept local to avoid cross-module coupling.
fn rbtdro_read_env_value(path: &Path, key: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let prefix = format!("{}=", key);
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix(&prefix) {
            return Some(rest.to_string());
        }
    }
    None
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

/// Cases 3-7 probe: reliquary stamp yoked into the witness vessel's rbrv.env.
/// Case 1's yoke fan-out writes RBRV_RELIQUARY into every ordain-path vessel
/// and commits; reading the witness vessel's committed value is the cross-case
/// evidence that case 1 ran. No out-of-source-tree scratch state.
/// Kludge is local-only (no GCP), so governor RBRA is not a load-bearing
/// precondition for kludge; the witness presence confirms case 1 completed.
fn rbtdro_probe_reliquary_stamp() -> Result<(), String> {
    let root = rbtdro_probe_root()?;
    let rbrv = root
        .join(RBTDRO_WITNESS_VESSEL_DIR)
        .join(RBTDRO_VESSEL_RBRV_FILE);
    let value = rbtdro_read_env_value(&rbrv, RBTDRO_FIELD_RBRV_RELIQUARY).ok_or_else(|| {
        format!(
            "{} missing from {}",
            RBTDRO_FIELD_RBRV_RELIQUARY,
            rbrv.display()
        )
    })?;
    if value.trim().is_empty() {
        return Err(format!(
            "{} is empty in {}",
            RBTDRO_FIELD_RBRV_RELIQUARY,
            rbrv.display()
        ));
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
    vessel_sigil: &str,
    label: &str,
) -> Result<(), rbtdre_Verdict> {
    let result = match rbtdro_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_YOKE_RELIQUARY,
        &[vessel_sigil, stamp],
        &[],
        dir,
        label,
    ) {
        Ok(r) => r,
        Err(e) => {
            return Err(rbtdre_Verdict::Fail(format!(
                "{} {} invocation: {}",
                RBTDRO_OPERATION_YOKE, vessel_sigil, e
            )))
        }
    };
    if result.exit_code != 0 {
        return Err(rbtdre_Verdict::Fail(format!(
            "{} {} exit {}\n{}",
            RBTDRO_OPERATION_YOKE, vessel_sigil, result.exit_code, result.stderr
        )));
    }
    Ok(())
}

/// Enshrine one vessel — mirrors that vessel's RBRV_IMAGE_n_ORIGIN bases
/// to GAR and writes back the resulting RBRV_IMAGE_n_ANCHOR digests.
/// `rbfd_enshrine` is per-vessel; the airgap chain calls this twice with
/// different vessels (forge first, then airgap-bottle).
fn rbtdro_enshrine(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    vessel_sigil: &str,
    label: &str,
) -> Result<(), rbtdre_Verdict> {
    let result = match rbtdro_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_ENSHRINE_VESSEL,
        &[vessel_sigil],
        &[],
        dir,
        label,
    ) {
        Ok(r) => r,
        Err(e) => return Err(rbtdre_Verdict::Fail(format!("enshrine invocation: {}", e))),
    };
    if result.exit_code != 0 {
        return Err(rbtdre_Verdict::Fail(format!(
            "enshrine {} exit {}\n{}",
            vessel_sigil, result.exit_code, result.stderr
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

/// Write a value into a vessel's rbrv.env. Same atomic-rename pattern as
/// rbtdro_drive_hallmark, but targeting the vessel's regime file rather than
/// a nameplate's. Returns Err if the variable line is not found.
fn rbtdro_write_vessel_env(
    root: &Path,
    vessel_dir: &str,
    var_name: &str,
    value: &str,
) -> Result<(), String> {
    let rbrv_path = root.join(vessel_dir).join(RBTDRO_VESSEL_RBRV_FILE);
    let file = std::fs::File::open(&rbrv_path)
        .map_err(|e| format!("open rbrv.env for {}: {}", vessel_dir, e))?;

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .map_err(|e| format!("read rbrv.env for {}: {}", vessel_dir, e))?;

    let prefix = format!("{}=", var_name);
    let mut found = false;
    let rewritten: Vec<String> = lines
        .into_iter()
        .map(|line| {
            if line.starts_with(&prefix) {
                found = true;
                format!("{}={}", var_name, value)
            } else {
                line
            }
        })
        .collect();

    if !found {
        return Err(format!(
            "variable {} not found in {}/rbrv.env",
            var_name, vessel_dir
        ));
    }

    let tmp_path = rbrv_path.with_extension("env.write_tmp");
    {
        let mut tmp = std::fs::File::create(&tmp_path)
            .map_err(|e| format!("create tmp rbrv.env for {}: {}", vessel_dir, e))?;
        for line in &rewritten {
            writeln!(tmp, "{}", line)
                .map_err(|e| format!("write tmp rbrv.env for {}: {}", vessel_dir, e))?;
        }
    }
    std::fs::rename(&tmp_path, &rbrv_path)
        .map_err(|e| format!("atomic replace rbrv.env for {}: {}", vessel_dir, e))?;
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
    rbtdro_git_commit(&format!("kludge-{}: sentry hallmark", nameplate))?;

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

    rbtdro_git_commit(&format!("kludge-{}: bottle hallmark", nameplate))?;
    Ok(())
}

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

    // Yoke stamp into all ordain-side vessels in one pass.
    for vessel_dir in RBTDRO_YOKE_VESSEL_DIRS {
        let sigil = vessel_dir.rsplit('/').next().unwrap_or(vessel_dir);
        let label = format!("{}-{}", RBTDRO_OPERATION_YOKE, sigil);
        if let Err(v) = rbtdro_yoke(ctx, dir, &stamp, sigil, &label) {
            return v;
        }
    }

    // Commit the rbrv.env changes for all yoked vessels.
    if let Err(v) = rbtdro_git_commit(
        &format!("inscribe-reliquary: {} stamp into all ordain-side vessels", RBTDRO_OPERATION_YOKE),
    ) {
        return v;
    }

    rbtdre_Verdict::Pass
}

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
        "ordain-conjure: sentry-tether hallmark + propagate to consumers",
    ) {
        return v;
    }

    rbtdre_Verdict::Pass
}

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
        rbtdro_git_commit("conjure-srjcl: jupyter-bottle hallmark + propagate to srjcl")
    {
        return v;
    }

    rbtdre_Verdict::Pass
}

/// Walk the airgap supply chain: enshrine upstream rust base into forge,
/// conjure the forge tethered, write the forge-hallmark locator into the
/// airgap vessel's base anchor (no copy — orchestration writes the locator
/// directly), conjure the airgap bottle.
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

    let forge_sigil = RBTDRO_VESSEL_DIR_AIRGAP_FORGE
        .rsplit('/')
        .next()
        .unwrap_or(RBTDRO_VESSEL_DIR_AIRGAP_FORGE);

    if let Err(v) = rbtdro_enshrine(ctx, dir, forge_sigil, "enshrine-upstream") {
        return v;
    }
    // Commit before ordain-forge: ordain has clean-tree precondition.
    if let Err(v) = rbtdro_git_commit(
        "ordain-airgap: enshrine upstream rust base into forge vessel",
    ) {
        return v;
    }

    // Capture the forge hallmark — it becomes the airgap-bottle's base via
    // a hallmark-namespace locator (mechanism (c)).
    let forge_hallmark = match rbtdro_ordain_capture(
        ctx,
        dir,
        RBTDRO_VESSEL_DIR_AIRGAP_FORGE,
        &[],
        "ordain-forge",
    ) {
        Ok(h) => h,
        Err(v) => return v,
    };

    // Write the hallmark-base locator into airgap-bottle's rbrv.env. The slot
    // points at the forge image inside its hallmark subtree; conjure resolves
    // this locator to a full GAR ref at airgap-bottle build time.
    let airgap_anchor = format!(
        "{}/{}/{}:{}",
        RBTDRO_GAR_CATEGORY_HALLMARKS,
        forge_hallmark.trim(),
        RBTDRO_ARK_BASENAME_IMAGE,
        forge_hallmark.trim(),
    );
    if let Err(e) = rbtdro_write_vessel_env(
        &root,
        RBTDRO_VESSEL_DIR_AIRGAP_BOTTLE,
        RBTDRO_AIRGAP_BASE_ANCHOR_VAR,
        &airgap_anchor,
    ) {
        return rbtdre_Verdict::Fail(format!("write airgap-bottle anchor: {}", e));
    }
    let _ = std::fs::write(dir.join("airgap-anchor.txt"), &airgap_anchor);

    // Commit the locator write before ordain-airgap: ordain has clean-tree
    // precondition. The forge hallmark's existence in GAR is established by
    // ordain-forge's success above — no separate enshrine validation step.
    if let Err(v) = rbtdro_git_commit(
        "ordain-airgap: write forge-hallmark locator into airgap-bottle base anchor",
    ) {
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
        rbtdro_git_commit("ordain-airgap: airgap-bottle hallmark + propagate to moriah")
    {
        return v;
    }

    rbtdre_Verdict::Pass
}

/// Pin upstream PlantUML by digest. Bind mode reads RBRV_BIND_IMAGE from
/// rbev-bottle-plantuml/rbrv.env and mirrors the digest into GAR via Cloud
/// Build (skopeo from reliquary + about/vouch metadata). Propagates plantuml
/// hallmark to pluml via RBRN_BOTTLE_HALLMARK.
fn rbtdro_onboarding_ordain_bind(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_onboarding_inscribe_reliquary before this case",
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
        rbtdro_git_commit("ordain-bind: plantuml-bottle hallmark + propagate to pluml")
    {
        return v;
    }

    rbtdre_Verdict::Pass
}

/// Pull the graft source image, then ordain rbev-graft-demo with a
/// BURE_TWEAK overriding RBRV_GRAFT_IMAGE in-process. No consumers —
/// graft-demo is terminal.
fn rbtdro_onboarding_ordain_graft(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "reliquary stamp captured",
        check: rbtdro_probe_reliquary_stamp,
        remediation: "rerun rbtdro_onboarding_inscribe_reliquary before this case",
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

    if let Err(v) = rbtdro_git_commit("ordain-graft: graft-demo hallmark") {
        return v;
    }

    rbtdre_Verdict::Pass
}

// ── Section registry ─────────────────────────────────────────

pub static RBTDRO_SECTIONS_ONBOARDING_SEQUENCE: &[rbtdre_Section] = &[rbtdre_Section {
    name: "onboarding-arc",
    cases: &[
        case!(rbtdro_onboarding_inscribe_reliquary),
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
