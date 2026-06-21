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
// RBTDRK — shared freehold-scheme machinery for the depot test fixtures
//
// One depot test-prefix scheme — the freehold — serves every depot fixture:
// the durable freehold operations (rbtdrk_depot) and the
// ephemeral depot-lifecycle (rbtdrp_lifecycle). This module is the single home
// for the scheme: prefix bases, family stem, static SA identities, the env-file
// install/rewrite helpers, the auto-increment moniker picker, and the case
// precondition probes. The fixtures compose on top of it; none carries its own
// copy. (The collapse of the former canonical/pristine two-scheme world: the
// surviving cloud/runtime prefix VALUES are `canc`/`canr` and the family stem is
// `canest3`, kept as opaque deployed strings so the live freehold keeps working
// — the names here are freehold vocabulary, the values are vestigial.)
//
// The auto-increment picker's `max + 1` rule is what keeps the lifecycle fixture
// from ever colliding with the standing freehold: a routine lifecycle run always
// mints a FRESH moniker and tears that down — only the deliberate, suiteless
// freehold-churn ever destroys the freehold itself.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::rbtdrx_platform::rbtdrx_path_from_env;
use crate::rbtdri_invocation::{
    RBTDRI_BURV_OUTPUT_SUBDIR,
    rbtdri_Context,
    rbtdri_InvokeResult,
    rbtdri_invoke_global,
};
use crate::rbtdgc_consts::{
    RBTDGC_RBRA_FILE,
    RBTDGC_RBRD_FILE,
    RBTDGC_RBRR_FILE,
};

// ── Freehold-scheme identities ───────────────────────────────

/// Freehold RBRR prefix bases installed by the establish/stand-up cases.
/// Per-station tincture from BURS is composed in at runtime so parallel-station
/// runs land in disjoint cloud names. The probe detects freehold state by
/// reading the moniker's family stem (also tinctured) from rbrr.env. (Deployed
/// VALUES retained from the former canonical scheme — opaque strings now.)
pub(crate) const RBTDRK_FREEHOLD_CLOUD_BASE: &str = "canc";
pub(crate) const RBTDRK_FREEHOLD_RUNTIME_BASE: &str = "canr";

/// Family-stem base for freehold depots; six-digit auto-increment suffix per
/// run. Depots persist post-success for operator inspection; reruns pick the
/// next free suffix by walking depot_list output. Per-station tincture is
/// composed in at runtime so each station's monikers fact-file-walk against
/// a disjoint family stem. The `3` is an era bump (past `canest`/`canest2`)
/// that side-stepped pending-delete projectId reservations from burned-bridges
/// teardown: project IDs are globally unique and reserved ~30 days post-delete,
/// and the active-only, single-identity allocator re-derives a reserved ID it
/// can neither see nor own. (Deployed VALUE retained from the former canonical
/// scheme.)
pub(crate) const RBTDRK_FREEHOLD_STEM_BASE: &str = "canest3";

const RBTDRK_FAMILY_NUMERIC_FLOOR: u32 = 100000;
const RBTDRK_FAMILY_NUMERIC_WIDTH: usize = 6;

// rbgp_depot_list emits fact files at
// `<cloud_prefix>/<moniker>.depot` (state) and
// `<cloud_prefix>/<moniker>.depot-project` (project_id). The cloud_prefix
// subdir prevents collisions between same-moniker depots under different
// cloud_prefixes.
pub(crate) const RBTDRK_FACT_EXT_DEPOT: &str = "depot";
pub(crate) const RBTDRK_FACT_EXT_DEPOT_PROJECT: &str = "depot-project";
/// Depot lifecycle state string rbw-dl emits into the `.depot` fact for an ACTIVE
/// project (mirrors the bash RBGP_DEPOT_STATE_COMPLETE). The reuse gate compares
/// the current freehold's state fact against this; anything else (DELETE_REQUESTED,
/// or no fact at all) is treated as "needs creation".
pub(crate) const RBTDRK_DEPOT_STATE_COMPLETE: &str = "COMPLETE";

pub(crate) const RBTDRK_FIELD_RBRD_CLOUD_PREFIX: &str = "RBRD_CLOUD_PREFIX";
pub(crate) const RBTDRK_FIELD_RBRR_RUNTIME_PREFIX: &str = "RBRR_RUNTIME_PREFIX";
pub(crate) const RBTDRK_FIELD_RBRD_DEPOT_MONIKER: &str = "RBRD_DEPOT_MONIKER";
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

/// Compose freehold RBRD_CLOUD_PREFIX with the given tincture.
pub(crate) fn rbtdrk_freehold_cloud_prefix(tincture: &str) -> String {
    format!("{}{}-", RBTDRK_FREEHOLD_CLOUD_BASE, tincture)
}

/// Compose freehold RBRR_RUNTIME_PREFIX with the given tincture.
pub(crate) fn rbtdrk_freehold_runtime_prefix(tincture: &str) -> String {
    format!("{}{}-", RBTDRK_FREEHOLD_RUNTIME_BASE, tincture)
}

/// Compose freehold family stem with the given tincture.
pub(crate) fn rbtdrk_family_stem(tincture: &str) -> String {
    format!("{}{}", RBTDRK_FREEHOLD_STEM_BASE, tincture)
}

// ── Helpers ──────────────────────────────────────────────────

/// Read an env-file value or None if absent. The bash regime files use
/// `KEY=value` lines (unquoted); comment and blank lines are skipped.
pub(crate) fn rbtdrk_read_env_value(path: &Path, key: &str) -> Option<String> {
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

/// Resolve a path that may be absolute or relative-to-project-root.
pub(crate) fn rbtdrk_resolve(root: &Path, raw: &str) -> PathBuf {
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

/// Resolve the freehold RBRA path for a role: <RBRR_SECRETS_DIR>/<role>/rbra.env.
pub(crate) fn rbtdrk_freehold_rbra(root: &Path, role: &str) -> Result<PathBuf, String> {
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

// ── Freehold-prefix install ──────────────────────────────────

/// Idempotently install the freehold canc-/canr- prefixes. CLOUD_PREFIX lands in
/// rbrd.env, RUNTIME_PREFIX lands in rbrr.env. Returns Ok without committing when
/// both already match the freehold markers; otherwise rewrites both files and
/// commits in one go.
pub(crate) fn rbtdrk_install_freehold_prefixes(root: &Path) -> Result<(), String> {
    let rbrr = root.join(RBTDGC_RBRR_FILE);
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let tincture = rbtdrk_burs_tincture()?;
    let cloud_target = rbtdrk_freehold_cloud_prefix(&tincture);
    let runtime_target = rbtdrk_freehold_runtime_prefix(&tincture);

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
        "freehold fixture: install freehold prefixes ({}/{})",
        cloud_target, runtime_target
    );
    rbtdrk_git_add_and_commit_paths(
        root,
        &[RBTDGC_RBRR_FILE, RBTDGC_RBRD_FILE],
        &commit_msg,
    )
}

pub(crate) fn rbtdrk_install_depot_moniker(root: &Path, moniker: &str) -> Result<(), String> {
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let content = std::fs::read_to_string(&rbrd)
        .map_err(|e| format!("rbtdrk: read {}: {}", rbrd.display(), e))?;
    let new_content =
        rbtdrk_replace_env_fields(&content, &[(RBTDRK_FIELD_RBRD_DEPOT_MONIKER, moniker)]);
    std::fs::write(&rbrd, &new_content)
        .map_err(|e| format!("rbtdrk: write {}: {}", rbrd.display(), e))?;
    let commit_msg = format!(
        "freehold fixture: set {}={}",
        RBTDRK_FIELD_RBRD_DEPOT_MONIKER, moniker
    );
    rbtdrk_git_add_and_commit(root, RBTDGC_RBRD_FILE, &commit_msg)
}

/// Compose depot project_id from kindled regime values: <CLOUD>d-<moniker>.
pub(crate) fn rbtdrk_compose_project_id(root: &Path, moniker: &str) -> Result<String, String> {
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let cloud_prefix = rbtdrk_read_env_value(&rbrd, RBTDRK_FIELD_RBRD_CLOUD_PREFIX)
        .ok_or_else(|| format!("RBRD_CLOUD_PREFIX missing from {}", rbrd.display()))?;
    Ok(format!("{}d-{}", cloud_prefix, moniker))
}

/// Cloud-prefix subdir name used in depot fact-file layout
/// (`<cloud_prefix>/<moniker>.depot`). Derived from RBRD_CLOUD_PREFIX with
/// the structural trailing `-` stripped so it matches the filesystem layout
/// emitted by zrbgp_depot_state_emit.
pub(crate) fn rbtdrk_cloud_prefix_subdir(root: &Path) -> Result<String, String> {
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
/// `max + 1` is also the safety boundary between the lifecycle and the
/// freehold: a lifecycle stand-up always picks a moniker ABOVE the standing
/// freehold's, so its tear-down never reaches the freehold.
///
/// Caller contract: `list_result` MUST be from a freshly-invoked depot_list
/// that ran in the current process. The fact-file scan IS the collision
/// check — stale state means picking a colliding moniker. Do not reuse a
/// `list_result` across cases or pass an operator-cached value.
pub(crate) fn rbtdrk_pick_next_moniker(
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

/// Wrapper invocation: call `rbtdri_invoke_global` and tee stdout/stderr to
/// `dir/<label>-stdout.txt` / `dir/<label>-stderr.txt` for diagnostic review.
/// Returns `Ok(InvokeResult)` regardless of exit code; callers decide what
/// counts as failure.
pub(crate) fn rbtdrk_invoke_logged(
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

/// rbrr.env exists. Sanity precondition — the establish cases presume the regime
/// has been initialized at least to the marshal-zero blank-template shape.
pub(crate) fn rbtdrk_probe_rbrr_present() -> Result<(), String> {
    let root = rbtdrk_probe_root()?;
    let rbrr = root.join(RBTDGC_RBRR_FILE);
    if !rbrr.exists() {
        return Err(format!("rbrr.env not found at {}", rbrr.display()));
    }
    Ok(())
}

/// Freehold depot moniker installed in rbrd.env. Established by the ensure case;
/// absence means freehold-ensure didn't run or rbrd.env was rewritten.
pub(crate) fn rbtdrk_probe_freehold_moniker() -> Result<(), String> {
    let root = rbtdrk_probe_root()?;
    let rbrd = root.join(RBTDGC_RBRD_FILE);
    let tincture = rbtdrk_burs_tincture()?;
    let family_stem = rbtdrk_family_stem(&tincture);
    let moniker =
        rbtdrk_read_env_value(&rbrd, RBTDRK_FIELD_RBRD_DEPOT_MONIKER).unwrap_or_default();
    if !moniker.starts_with(&family_stem) {
        return Err(format!(
            "{}={:?} does not begin with '{}' — freehold depot moniker not installed",
            RBTDRK_FIELD_RBRD_DEPOT_MONIKER, moniker, family_stem
        ));
    }
    Ok(())
}
