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
// RBTDRK — gauntlet canonical-establish fixture (§2 of release qualification)
//
// Establishes long-lived canonical state the gauntlet's downstream fixtures
// inherit. canonical-establish admits FEDERATION PERSONAS — the no-keys org enforces
// disableServiceAccountKeyCreation, so the keyfile enrobe 400s on a fresh levy:
//   1. depot_levy            — install canonical RBRR prefixes; levy a fresh canest depot
//                              (the levy establishes the three mantle SAs with frozen IAM)
//   2. compear               — open/confirm a live assize against the RBRF trust (rbw-acf):
//                              one device-flow click at suite head, then the cases below
//                              ride the cached federated token headless
//   3. gird_governor         — the payor (OAuth) seats the freehold subject as the first
//                              governor (rbw-pE) — the founding door a fresh levy needs
//   4. brevet_don_director   — the girded governor brevets the freehold subject onto the
//                              director mantle, then dons it and reaches AR (rbw-pB + rbw-acm)
//   5. brevet_don_retriever  — same for the retriever mantle
//   6. depot_recognosce      — read-only proof the levy's founding stands whole
//
// The keyfile enrobe cases (rbtdrk_governor_enrobe / role_enrobe / role_defrock) are
// retained below for the canonical-ENROBE fixture (skirmish/dogfight/blockade), which still
// exercises the bridge-legacy keyfile estate; only canonical-ESTABLISH moved to personas.
//
// Disposition: StateProgressing. Each case carries a precondition probe via
// rbtdrb_Probe so a-la-carte single-case rerun fails cleanly when an earlier
// case's state is absent. Tabtarget invocations are responsible for waiting
// out their own propagation (no separate iam-propagation-wait case).

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::case;
use crate::rbtdrb_probe::{rbtdrb_assert, rbtdrb_Probe};
use crate::rbtdrc_crucible::rbtdrc_with_ctx;
use crate::rbtdre_engine::{rbtdre_Case, rbtdre_Disposition, rbtdre_Fixture, rbtdre_Verdict};
use crate::rbtdrx_platform::rbtdrx_path_from_env;
use crate::rbtdri_invocation::{
    RBTDRI_BURE_CONFIRM_KEY,
    RBTDRI_BURE_CONFIRM_SKIP,
    RBTDRI_BURV_OUTPUT_SUBDIR,
    rbtdri_Context,
    rbtdri_InvokeResult,
    rbtdri_invoke_global,
    rbtdri_read_burv_fact,
};
use crate::rbtdgc_consts::{
    RBTDGC_DEFROCK_DIRECTOR,
    RBTDGC_DEFROCK_RETRIEVER,
    RBTDGC_ENROBE_DIRECTOR,
    RBTDGC_ENROBE_RETRIEVER,
    RBTDGC_LEVY_DEPOT,
    RBTDGC_LIST_DEPOT,
    RBTDGC_RECOGNOSCE_DEPOT,
    RBTDGC_UNMAKE_DEPOT,
    RBTDGC_ENROBE_GOVERNOR,
    RBTDGC_RBRA_FILE,
    RBTDGC_RBRD_FILE,
    RBTDGC_RBRR_FILE,
    RBTDGC_ACCOUNT_ASSAY,
    RBTDGC_ACCOUNT_DIRECTOR,
    RBTDGC_ACCOUNT_GOVERNOR,
    RBTDGC_ACCOUNT_RETRIEVER,
    RBTDGC_GIRD_POLITY,
    RBTDGC_BREVET_POLITY,
    RBTDGC_CHECK_MANTLE,
    RBTDGC_CHECK_COMPEARANCE,
    RBTDGC_FREEHOLD_SUBJECT,
};
use crate::rbtdrm_manifest::{
    rbtdrm_credential_check_colophon,
    RBTDRM_FIXTURE_CANONICAL_ESTABLISH,
    RBTDRM_FIXTURE_CANONICAL_ENROBE,
    RBTDRM_FIXTURE_CANONICAL_CHURN,
};

// ── Canonical-fixture identities ─────────────────────────────

/// Canonical RBRR prefix bases installed by case 1. Distinct from pristine's
/// throwaway prefix bases (prlc/prlr); per-station tincture from BURS is
/// composed in at runtime so parallel-station runs land in disjoint cloud
/// names. Case 2's probe detects canonical state by reading the moniker's
/// family stem (also tinctured) from rbrr.env.
pub(crate) const RBTDRK_CANONICAL_CLOUD_BASE: &str = "canc";
pub(crate) const RBTDRK_CANONICAL_RUNTIME_BASE: &str = "canr";

/// Family-stem base for canonical depots; six-digit auto-increment suffix per
/// run. Depots persist post-success for operator inspection; reruns pick the
/// next free suffix by walking depot_list output. Per-station tincture is
/// composed in at runtime so each station's monikers fact-file-walk against
/// a disjoint family stem. Era-bumped past `canest` and `canest2` to
/// side-step pending-delete projectId reservations from burned-bridges
/// teardown. The `canest2` bump was forced when the prior gmail-identity
/// payor's deleted `canest2bhm100000` depot held the global projectId in
/// DELETE_REQUESTED: project IDs are globally unique and reserved ~30 days
/// post-delete, and the active-only, single-identity allocator re-derives a
/// reserved ID it can neither see nor own.
pub(crate) const RBTDRK_FAMILY_STEM_BASE: &str = "canest3";

/// Static identities for the canonical SA cycle. Stable across runs because
/// each run uses a fresh canest depot project.
const RBTDRK_IDENTITY_RETRIEVER: &str = "canest-ret";
const RBTDRK_IDENTITY_DIRECTOR: &str = "canest-dir";

const RBTDRK_FAMILY_NUMERIC_FLOOR: u32 = 100000;
const RBTDRK_FAMILY_NUMERIC_WIDTH: usize = 6;

// rbgp_depot_list emits fact files at
// `<cloud_prefix>/<moniker>.depot` (state) and
// `<cloud_prefix>/<moniker>.depot-project` (project_id). The cloud_prefix
// subdir prevents collisions between same-moniker depots under different
// cloud_prefixes.
const RBTDRK_FACT_EXT_DEPOT: &str = "depot";
const RBTDRK_FACT_EXT_DEPOT_PROJECT: &str = "depot-project";
/// Depot lifecycle state string rbw-dl emits into the `.depot` fact for an ACTIVE
/// project (mirrors the bash RBGP_DEPOT_STATE_COMPLETE). The reuse gate compares
/// the current freehold's state fact against this; anything else (DELETE_REQUESTED,
/// or no fact at all) is treated as "needs creation".
const RBTDRK_DEPOT_STATE_COMPLETE: &str = "COMPLETE";
/// Placeholder moniker installed into RBRD before unmaking the canonical
/// freehold, so rbgp_depot_unmake's live-disqualify guard — which refuses the
/// RBRD-selected project — releases the real one. The next canonical-establish
/// run sees this placeholder as absent and takes the create path.
const RBTDRK_CHURN_PLACEHOLDER_MONIKER: &str = "churned";
const RBTDRK_FACT_GOVERNOR_SA_EMAIL: &str = "rbgp_fact_governor_sa_email";

const RBTDRK_FIELD_RBRD_CLOUD_PREFIX: &str = "RBRD_CLOUD_PREFIX";
const RBTDRK_FIELD_RBRR_RUNTIME_PREFIX: &str = "RBRR_RUNTIME_PREFIX";
const RBTDRK_FIELD_RBRD_DEPOT_MONIKER: &str = "RBRD_DEPOT_MONIKER";
const RBTDRK_FIELD_RBRR_SECRETS_DIR: &str = "RBRR_SECRETS_DIR";

/// BURS station-file env var (exported by bul_launcher.sh) — absolute path
/// to the developer's burs.env. Source for BURS_TINCTURE.
const RBTDRK_ENV_STATION_FILE: &str = "BURD_STATION_FILE";

/// Read BURS_TINCTURE from the station file resolved via BURD_STATION_FILE.
/// BURS validation upstream (zburs_enforce) guarantees the value is 1-3 chars
/// of lowercase alphanumeric starting with a letter.
pub(crate) fn rbtdrk_burs_tincture() -> Result<String, String> {
    let path = rbtdrx_path_from_env(RBTDRK_ENV_STATION_FILE)?;
    rbtdrk_read_env_value(&path, "BURS_TINCTURE")
        .ok_or_else(|| format!("BURS_TINCTURE not in {}", path.display()))
}

/// Compose canonical RBRD_CLOUD_PREFIX with the given tincture.
pub(crate) fn rbtdrk_canonical_cloud_prefix(tincture: &str) -> String {
    format!("{}{}-", RBTDRK_CANONICAL_CLOUD_BASE, tincture)
}

/// Compose canonical RBRR_RUNTIME_PREFIX with the given tincture.
pub(crate) fn rbtdrk_canonical_runtime_prefix(tincture: &str) -> String {
    format!("{}{}-", RBTDRK_CANONICAL_RUNTIME_BASE, tincture)
}

/// Compose canonical family stem with the given tincture.
pub(crate) fn rbtdrk_family_stem(tincture: &str) -> String {
    format!("{}{}", RBTDRK_FAMILY_STEM_BASE, tincture)
}

// ── Helpers ──────────────────────────────────────────────────

/// Read an env-file value or None if absent. Mirrors pristine's helper —
/// kept local to avoid coupling canonical-establish to pristine-lifecycle.
fn rbtdrk_read_env_value(path: &Path, key: &str) -> Option<String> {
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

fn rbtdrk_resolve(root: &Path, raw: &str) -> PathBuf {
    if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        root.join(raw)
    }
}

fn rbtdrk_replace_env_fields(content: &str, pairs: &[(&str, &str)]) -> String {
    let mut result: String = content
        .lines()
        .map(|line| {
            for (key, value) in pairs {
                let assign = format!("{}=", key);
                if line.starts_with(&assign) {
                    return format!("{}{}", assign, value);
                }
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    if content.ends_with('\n') {
        result.push('\n');
    }
    result
}

fn rbtdrk_git_add_and_commit(root: &Path, file: &str, message: &str) -> Result<(), String> {
    rbtdrk_git_add_and_commit_paths(root, &[file], message)
}

fn rbtdrk_git_add_and_commit_paths(
    root: &Path,
    files: &[&str],
    message: &str,
) -> Result<(), String> {
    let mut add_args: Vec<&str> = vec!["add"];
    add_args.extend_from_slice(files);
    let add = Command::new("git")
        .args(&add_args)
        .current_dir(root)
        .output()
        .map_err(|e| format!("rbtdrk: git add invocation failed: {}", e))?;
    if !add.status.success() {
        return Err(format!(
            "rbtdrk: git add failed (exit {}): {}",
            add.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&add.stderr).trim()
        ));
    }
    let commit = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(root)
        .output()
        .map_err(|e| format!("rbtdrk: git commit invocation failed: {}", e))?;
    if !commit.status.success() {
        return Err(format!(
            "rbtdrk: git commit failed (exit {}): {}",
            commit.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&commit.stderr).trim()
        ));
    }
    Ok(())
}

/// Resolve canonical RBRA path for a role: <RBRR_SECRETS_DIR>/<role>/rbra.env.
pub(crate) fn rbtdrk_canonical_rbra(root: &Path, role: &str) -> Result<PathBuf, String> {
    let rbrr = root.join(RBTDGC_RBRR_FILE);
    let secrets_dir = rbtdrk_read_env_value(&rbrr, RBTDRK_FIELD_RBRR_SECRETS_DIR)
        .ok_or_else(|| format!("RBRR_SECRETS_DIR missing from {}", rbrr.display()))?;
    if secrets_dir.is_empty() {
        return Err(format!("RBRR_SECRETS_DIR is blank in {}", rbrr.display()));
    }
    Ok(rbtdrk_resolve(root, &secrets_dir)
        .join(role)
        .join(RBTDGC_RBRA_FILE))
}

// ── Canonical-prefix install (case 1) ───────────────────────

/// Idempotently install canc-/canr- prefixes. CLOUD_PREFIX lands in rbrd.env,
/// RUNTIME_PREFIX lands in rbrr.env. Returns Ok without committing when both
/// already match the canonical markers; otherwise rewrites both files and
/// commits in one go.
pub(crate) fn rbtdrk_install_canonical_prefixes(root: &Path) -> Result<(), String> {
    let rbrr = root.join(RBTDGC_RBRR_FILE);
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let tincture = rbtdrk_burs_tincture()?;
    let cloud_target = rbtdrk_canonical_cloud_prefix(&tincture);
    let runtime_target = rbtdrk_canonical_runtime_prefix(&tincture);

    let cloud = rbtdrk_read_env_value(&rbrd, RBTDRK_FIELD_RBRD_CLOUD_PREFIX).unwrap_or_default();
    let runtime =
        rbtdrk_read_env_value(&rbrr, RBTDRK_FIELD_RBRR_RUNTIME_PREFIX).unwrap_or_default();
    if cloud == cloud_target && runtime == runtime_target {
        return Ok(());
    }

    if cloud != cloud_target {
        let content = std::fs::read_to_string(&rbrd)
            .map_err(|e| format!("rbtdrk: read {}: {}", rbrd.display(), e))?;
        let new_content = rbtdrk_replace_env_fields(
            &content,
            &[(RBTDRK_FIELD_RBRD_CLOUD_PREFIX, cloud_target.as_str())],
        );
        std::fs::write(&rbrd, &new_content)
            .map_err(|e| format!("rbtdrk: write {}: {}", rbrd.display(), e))?;
    }

    if runtime != runtime_target {
        let content = std::fs::read_to_string(&rbrr)
            .map_err(|e| format!("rbtdrk: read {}: {}", rbrr.display(), e))?;
        let new_content = rbtdrk_replace_env_fields(
            &content,
            &[(RBTDRK_FIELD_RBRR_RUNTIME_PREFIX, runtime_target.as_str())],
        );
        std::fs::write(&rbrr, &new_content)
            .map_err(|e| format!("rbtdrk: write {}: {}", rbrr.display(), e))?;
    }

    let commit_msg = format!(
        "canonical-establish fixture: install canonical prefixes ({}/{})",
        cloud_target, runtime_target
    );
    rbtdrk_git_add_and_commit_paths(
        root,
        &[RBTDGC_RBRR_FILE, RBTDGC_RBRD_FILE],
        &commit_msg,
    )
}

fn rbtdrk_install_depot_moniker(root: &Path, moniker: &str) -> Result<(), String> {
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let content = std::fs::read_to_string(&rbrd)
        .map_err(|e| format!("rbtdrk: read {}: {}", rbrd.display(), e))?;
    let new_content =
        rbtdrk_replace_env_fields(&content, &[(RBTDRK_FIELD_RBRD_DEPOT_MONIKER, moniker)]);
    std::fs::write(&rbrd, &new_content)
        .map_err(|e| format!("rbtdrk: write {}: {}", rbrd.display(), e))?;
    let commit_msg = format!(
        "canonical-establish fixture: set {}={}",
        RBTDRK_FIELD_RBRD_DEPOT_MONIKER, moniker
    );
    rbtdrk_git_add_and_commit(root, RBTDGC_RBRD_FILE, &commit_msg)
}

/// Compose depot project_id from kindled regime values: <CLOUD>d-<moniker>.
fn rbtdrk_compose_project_id(root: &Path, moniker: &str) -> Result<String, String> {
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let cloud_prefix = rbtdrk_read_env_value(&rbrd, RBTDRK_FIELD_RBRD_CLOUD_PREFIX)
        .ok_or_else(|| format!("RBRD_CLOUD_PREFIX missing from {}", rbrd.display()))?;
    Ok(format!("{}d-{}", cloud_prefix, moniker))
}

/// Cloud-prefix subdir name used in depot fact-file layout
/// (`<cloud_prefix>/<moniker>.depot`). Derived from RBRD_CLOUD_PREFIX with
/// the structural trailing `-` stripped so it matches the filesystem layout
/// emitted by zrbgp_depot_state_emit.
fn rbtdrk_cloud_prefix_subdir(root: &Path) -> Result<String, String> {
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let cloud_prefix = rbtdrk_read_env_value(&rbrd, RBTDRK_FIELD_RBRD_CLOUD_PREFIX)
        .ok_or_else(|| format!("RBRD_CLOUD_PREFIX missing from {}", rbrd.display()))?;
    Ok(cloud_prefix.trim_end_matches('-').to_string())
}

/// Pick the next free moniker for `family_stem` by walking the depot_list
/// invocation's BURV output dir for `<family>NNNNNN.depot` files under the
/// current cloud_prefix subdir. Returns
/// `<family>RBTDRK_FAMILY_NUMERIC_FLOOR` when no matching files exist.
/// Restricting the walk to the current cloud_prefix is what makes allocation
/// collision-safe: a same-numbered moniker under a foreign cloud_prefix is
/// correctly ignored.
///
/// Caller contract: `list_result` MUST be from a freshly-invoked depot_list
/// that ran in the current process. The fact-file scan IS the collision
/// check — stale state means picking a colliding moniker. Do not reuse a
/// `list_result` across cases or pass an operator-cached value.
fn rbtdrk_pick_next_moniker(
    list_result: &rbtdri_InvokeResult,
    root: &Path,
    family_stem: &str,
) -> Result<String, String> {
    let prefix_dir = rbtdrk_cloud_prefix_subdir(root)?;
    let dir = list_result
        .burv_output
        .join(RBTDRI_BURV_OUTPUT_SUBDIR)
        .join(&prefix_dir);
    let entries = match std::fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => {
            return Ok(format!("{}{}", family_stem, RBTDRK_FAMILY_NUMERIC_FLOOR));
        }
    };
    let suffix_ext = format!(".{}", RBTDRK_FACT_EXT_DEPOT);
    let mut max_suffix: Option<u32> = None;
    for entry in entries.flatten() {
        let name = match entry.file_name().into_string() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let stem = match name.strip_suffix(&suffix_ext) {
            Some(s) => s,
            None => continue,
        };
        let numeric = match stem.strip_prefix(family_stem) {
            Some(s) => s,
            None => continue,
        };
        if numeric.len() != RBTDRK_FAMILY_NUMERIC_WIDTH {
            continue;
        }
        let parsed: u32 = match numeric.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        max_suffix = Some(max_suffix.map_or(parsed, |m| m.max(parsed)));
    }
    let next = match max_suffix {
        Some(m) => m + 1,
        None => RBTDRK_FAMILY_NUMERIC_FLOOR,
    };
    Ok(format!("{}{}", family_stem, next))
}

fn rbtdrk_invoke_logged(
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

// ── Probes ───────────────────────────────────────────────────

/// Probes are pure `fn() -> Result<(), String>` per the rbtdrb_Probe shape and
/// have no context, so they read the project root from current_dir() — theurge
/// always launches from the project root.
fn rbtdrk_probe_root() -> Result<PathBuf, String> {
    std::env::current_dir().map_err(|e| format!("cannot resolve project root: {}", e))
}

/// Case 1 probe: rbrr.env exists. Sanity precondition — canonical-establish
/// presumes the regime has been initialized at least to the marshal-zero
/// blank-template shape.
fn rbtdrk_probe_rbrr_present() -> Result<(), String> {
    let root = rbtdrk_probe_root()?;
    let rbrr = root.join(RBTDGC_RBRR_FILE);
    if !rbrr.exists() {
        return Err(format!("rbrr.env not found at {}", rbrr.display()));
    }
    Ok(())
}

/// Case 2 probe: canonical depot moniker installed in rbrd.env. Established
/// by case 1; absence means depot-levy didn't run or rbrd.env was rewritten.
fn rbtdrk_probe_canonical_moniker() -> Result<(), String> {
    let root = rbtdrk_probe_root()?;
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let tincture = rbtdrk_burs_tincture()?;
    let family_stem = rbtdrk_family_stem(&tincture);
    let moniker =
        rbtdrk_read_env_value(&rbrd, RBTDRK_FIELD_RBRD_DEPOT_MONIKER).unwrap_or_default();
    if !moniker.starts_with(&family_stem) {
        return Err(format!(
            "{}={:?} does not begin with '{}' — canonical depot moniker not installed",
            RBTDRK_FIELD_RBRD_DEPOT_MONIKER, moniker, family_stem
        ));
    }
    Ok(())
}

/// Cases 3 and 4 probe: governor RBRA file present at the canonical path.
/// Established by case 2's enrobe + canonical-copy step.
fn rbtdrk_probe_governor_rbra() -> Result<(), String> {
    let root = rbtdrk_probe_root()?;
    let path = rbtdrk_canonical_rbra(&root, RBTDGC_ACCOUNT_GOVERNOR)?;
    if !path.exists() {
        return Err(format!("governor RBRA absent at {}", path.display()));
    }
    Ok(())
}

// ── Cases ────────────────────────────────────────────────────

/// Case 1 — canonical freehold ensure. Installs canc-/canr- prefixes, then
/// REUSES the freehold RBRD already names when it is ACTIVE (no depot is
/// created), else mints a fresh canest moniker and levies. Cross-checks
/// project_id against the RBDC compose derivation either way. Validity is the
/// recognosce case's job (case 5), not this one — a stale-but-ACTIVE freehold is
/// reused here and fails there. The fn name retains `_levy` though it now
/// reuses or levies.
fn rbtdrk_depot_levy(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "rbrr.env present",
        check: rbtdrk_probe_rbrr_present,
        remediation: "ensure regime is initialized — rerun pristine-lifecycle if needed",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_depot_levy_impl(ctx, dir))
}

fn rbtdrk_depot_levy_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    if let Err(e) = rbtdrk_install_canonical_prefixes(&root) {
        return rbtdre_Verdict::Fail(format!("install canonical prefixes: {}", e));
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
    // fifth case) is the freehold's validity gate, so a stale-but-ACTIVE freehold
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

/// Case 2 — governor enrobe. Enrobes governor against the canonical depot,
/// reads the SA email fact, copies governor RBRA from BURV output to the
/// canonical path under RBRR_SECRETS_DIR.
fn rbtdrk_governor_enrobe(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "canonical depot moniker installed",
        check: rbtdrk_probe_canonical_moniker,
        remediation: "rerun rbtdrk_depot_levy or the full canonical-establish fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_governor_enrobe_impl(ctx, dir))
}

fn rbtdrk_governor_enrobe_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let assay = match rbtdrk_canonical_rbra(&root, RBTDGC_ACCOUNT_ASSAY) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical assay RBRA path: {}", e)),
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

    let canonical = match rbtdrk_canonical_rbra(&root, RBTDGC_ACCOUNT_GOVERNOR) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical governor RBRA path: {}", e)),
    };
    if let Some(parent) = canonical.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return rbtdre_Verdict::Fail(format!(
                "create governor RBRA dir {}: {}",
                parent.display(),
                e
            ));
        }
    }
    if let Err(e) = std::fs::copy(&assay, &canonical) {
        return rbtdre_Verdict::Fail(format!(
            "copy assay RBRA → governor canonical {}: {}",
            canonical.display(),
            e
        ));
    }
    rbtdre_Verdict::Pass
}

/// Case 3 — retriever enrobe + access-probe. The access-probe doubles as the
/// IAM-propagation gate: the enrobe tabtarget is responsible for waiting on
/// propagation before exiting (or the probe iterates internally). No separate
/// propagation-wait case.
fn rbtdrk_retriever_enrobe(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_enrobe or the full canonical-establish fixture",
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

/// Case 4 — director enrobe + access-probe.
fn rbtdrk_director_enrobe(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_enrobe or the full canonical-establish fixture",
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

/// Shared defrock body: invoke the defrock colophon for the identity, 404-tolerant
/// (the standing SA may be present or already gone). Exercises rbgg_defrock_*'s
/// revoke-before-delete and poll_until_gone debounce, so the following enrobe
/// hits the create branch against a durably-gone SA.
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

/// Case — director defrock. Clears the standing director SA (revoke bindings →
/// delete) so the following director enrobe exercises the create branch.
fn rbtdrk_director_defrock(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_enrobe or the full canonical-establish fixture",
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

/// Case — retriever defrock. Clears the standing retriever SA (revoke bindings →
/// delete) so the following retriever enrobe exercises the create branch.
fn rbtdrk_retriever_defrock(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_enrobe or the full canonical-establish fixture",
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
/// dropped by the enrobe tabtarget → copy to canonical role path → access-probe.
fn rbtdrk_role_enrobe_impl(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    enrobe_colophon: &str,
    identity: &str,
    role: &str,
) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let assay = match rbtdrk_canonical_rbra(&root, RBTDGC_ACCOUNT_ASSAY) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical assay RBRA path: {}", e)),
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

    let canonical = match rbtdrk_canonical_rbra(&root, role) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical {} RBRA path: {}", role, e)),
    };
    if let Some(parent) = canonical.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return rbtdre_Verdict::Fail(format!(
                "create {} RBRA dir {}: {}",
                role,
                parent.display(),
                e
            ));
        }
    }
    if let Err(e) = std::fs::copy(&assay, &canonical) {
        return rbtdre_Verdict::Fail(format!(
            "copy assay RBRA → {} canonical {}: {}",
            role,
            canonical.display(),
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

/// Case 5 — depot recognosce. Read-only proof that the levy's federation-founding
/// gestures stand whole against live GCP: the three mantle SAs, their capability-
/// sets, and the Artifact Registry Data-Access audit config. The rbw-dr verb does
/// the entire check and dies fatally naming any absent piece; this case asserts
/// only that it exits 0 — founding whole. Runs after the establish sequence, so a
/// pass also confirms the enrobes left the mantle founding intact.
fn rbtdrk_depot_recognosce(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "canonical depot moniker installed",
        check: rbtdrk_probe_canonical_moniker,
        remediation: "rerun rbtdrk_depot_levy or the full canonical-establish fixture",
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

/// Canonical freehold churn — the deliberate teardown that makes room for a
/// fresh levy. Reads the freehold RBRD names, rotates the moniker to a
/// placeholder so rbgp_depot_unmake's live-disqualify guard releases the real
/// project, unmakes it (confirm skipped via the test seam), and confirms it is
/// no longer ACTIVE. A subsequent canonical-establish run then finds no ACTIVE
/// freehold and takes the create path.
fn rbtdrk_depot_churn(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "canonical depot moniker installed",
        check: rbtdrk_probe_canonical_moniker,
        remediation: "no canonical freehold to churn — install one via canonical-establish first",
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
                "RBRD_DEPOT_MONIKER blank — no canonical freehold to churn".to_string(),
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

// ── Federation-persona cases (canonical-establish) ───────────
//
// canonical-establish admits federation personas on the no-keys org. The freehold
// subject (the operator's standing Entra oid, RBTDGC_FREEHOLD_SUBJECT) is compeared,
// girded as the first governor by the payor, then breveted onto the director and
// retriever mantles and donned — replacing the keyfile governor/retriever/director
// enrobe + JWT-probe cases (which stay live below for canonical-ENROBE).

/// Suite-head compearance. Opens or confirms a live assize against the RBRF trust
/// (rbw-acf): a cache-hit when the operator pre-compeared, an inline device-flow prompt
/// when a TTY is present, a loud headless failure otherwise. The admission cases below
/// ride the cached federated token, so the human clicks once here, not per case.
fn rbtdrk_compear(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "canonical depot moniker installed",
        check: rbtdrk_probe_canonical_moniker,
        remediation: "rerun rbtdrk_depot_levy or the full canonical-establish fixture",
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
        name: "canonical depot moniker installed",
        check: rbtdrk_probe_canonical_moniker,
        remediation: "rerun rbtdrk_depot_levy or the full canonical-establish fixture",
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

/// Case 4 — brevet + don the director mantle for the freehold subject.
fn rbtdrk_brevet_don_director(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "canonical depot moniker installed",
        check: rbtdrk_probe_canonical_moniker,
        remediation: "rerun the full canonical-establish fixture (levy → compear → gird) first",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_brevet_don_impl(ctx, dir, "director"))
}

/// Case 5 — brevet + don the retriever mantle for the freehold subject.
fn rbtdrk_brevet_don_retriever(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "canonical depot moniker installed",
        check: rbtdrk_probe_canonical_moniker,
        remediation: "rerun the full canonical-establish fixture (levy → compear → gird) first",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_brevet_don_impl(ctx, dir, "retriever"))
}

// ── Section registry ─────────────────────────────────────────

pub static RBTDRK_CASES_CANONICAL_ESTABLISH: &[rbtdre_Case] = &[
    case!(rbtdrk_depot_levy),
    case!(rbtdrk_compear),
    case!(rbtdrk_gird_governor),
    case!(rbtdrk_brevet_don_director),
    case!(rbtdrk_brevet_don_retriever),
    case!(rbtdrk_depot_recognosce),
];

pub static RBTDRK_FIXTURE_CANONICAL_ESTABLISH: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_CANONICAL_ESTABLISH,
    disposition: rbtdre_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    cases: RBTDRK_CASES_CANONICAL_ESTABLISH,
    credless: false,
};

// canonical-enrobe — the no-levy recycle variant shared by skirmish, dogfight,
// and blockade. Runs against a depot the operator has levied by hand (no GCP
// project created per run). After re-mantling the governor it defrocks then
// re-enrobes retriever + director, so every run exercises the full teardown →
// re-enrobe cycle: rbgg_defrock_*'s revoke-before-delete and poll_until_gone
// debounce, then the enrobe create branch against a durably-gone SA. This
// deliberately stresses the IAM eventual-consistency edges (the delete→recreate
// read-flap) on every run. Each case carries its own precondition probe
// (governor enrobe probes the standing depot's moniker; defrock/enrobe probe the
// governor). Defrock runs in reverse role order, enrobe in forward order. Sharing
// the case fns is the same provenance-vs-behavior split tadmor/moriah exploit
// with RBTDRC_CASES_SECURITY.
pub static RBTDRK_CASES_CANONICAL_ENROBE: &[rbtdre_Case] = &[
    case!(rbtdrk_governor_enrobe),
    case!(rbtdrk_director_defrock),
    case!(rbtdrk_retriever_defrock),
    case!(rbtdrk_retriever_enrobe),
    case!(rbtdrk_director_enrobe),
];

pub static RBTDRK_FIXTURE_CANONICAL_ENROBE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_CANONICAL_ENROBE,
    disposition: rbtdre_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    cases: RBTDRK_CASES_CANONICAL_ENROBE,
    credless: false,
};

// canonical-churn — the deliberate teardown of the canonical freehold. Single
// case: rotate the moniker off the live project, unmake it, confirm gone. Member
// of no suite — operator-invoked, quota-reclaiming, never a suite passenger.
pub static RBTDRK_CASES_CANONICAL_CHURN: &[rbtdre_Case] = &[
    case!(rbtdrk_depot_churn),
];

pub static RBTDRK_FIXTURE_CANONICAL_CHURN: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_CANONICAL_CHURN,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRK_CASES_CANONICAL_CHURN,
    credless: false,
};
