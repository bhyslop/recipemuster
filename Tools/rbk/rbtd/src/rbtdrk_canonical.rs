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
// inherit:
//   1. depot_levy        — install canonical RBRR prefixes; levy a fresh canest depot
//   2. governor_mantle   — mantle governor against the canonical depot
//   3. retriever_invest  — invest a canonical retriever SA + access-probe
//   4. director_invest   — invest a canonical director SA + access-probe
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
use crate::rbtdre_engine::{rbtdre_Section, rbtdre_Verdict};
use crate::rbtdri_invocation::{
    rbtdri_invoke_global, rbtdri_invoke_imprint, rbtdri_read_burv_fact, rbtdri_Context,
    rbtdri_InvokeResult, RBTDRI_BURV_OUTPUT_SUBDIR,
};
use crate::rbtdrm_manifest::{
    RBTDRM_COLOPHON_ACCESS_PROBE, RBTDRM_COLOPHON_DEPOT_LEVY, RBTDRM_COLOPHON_DEPOT_LIST,
    RBTDRM_COLOPHON_GOV_INVEST_DIRECTOR, RBTDRM_COLOPHON_GOV_INVEST_RETRIEVER,
    RBTDRM_COLOPHON_GOV_MANTLE,
};

// ── Canonical-fixture identities ─────────────────────────────

/// Canonical RBRR prefix markers installed by case 1. Distinct from
/// pristine's throwaway prefixes (prlc-/prlr-) — case 2's probe detects
/// canonical state by reading the moniker's family stem from rbrr.env.
pub(crate) const RBTDRK_CANONICAL_CLOUD_PREFIX: &str = "canc-";
pub(crate) const RBTDRK_CANONICAL_RUNTIME_PREFIX: &str = "canr-";

/// Family stem for canonical depots; six-digit auto-increment suffix per run.
/// Depots persist post-success for operator inspection; reruns pick the next
/// free suffix by walking depot_list output. Era-bumped past the prior
/// `canest` family to side-step pending-delete projectId reservations from
/// burned-bridges teardown.
pub(crate) const RBTDRK_FAMILY_STEM: &str = "canest2";

/// Static identities for the canonical SA cycle. Stable across runs because
/// each run uses a fresh canest depot project.
const RBTDRK_IDENTITY_RETRIEVER: &str = "canest-ret";
const RBTDRK_IDENTITY_DIRECTOR: &str = "canest-dir";

const RBTDRK_FAMILY_NUMERIC_FLOOR: u32 = 100000;
const RBTDRK_FAMILY_NUMERIC_WIDTH: usize = 6;

const RBTDRK_FACT_EXT_DEPOT: &str = "depot";
const RBTDRK_FACT_EXT_DEPOT_PROJECT: &str = "depot-project";
const RBTDRK_FACT_GOVERNOR_SA_EMAIL: &str = "rbgp_fact_governor_sa_email";

const RBTDRK_FIELD_RBRR_CLOUD_PREFIX: &str = "RBRR_CLOUD_PREFIX";
const RBTDRK_FIELD_RBRR_RUNTIME_PREFIX: &str = "RBRR_RUNTIME_PREFIX";
const RBTDRK_FIELD_RBRR_DEPOT_MONIKER: &str = "RBRR_DEPOT_MONIKER";
const RBTDRK_FIELD_RBRR_SECRETS_DIR: &str = "RBRR_SECRETS_DIR";

const RBTDRK_RBRR_FILE: &str = ".rbk/rbrr.env";
const RBTDRK_RBRA_FILE: &str = "rbra.env";

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
    let add = Command::new("git")
        .args(["add", file])
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
    let rbrr = root.join(RBTDRK_RBRR_FILE);
    let secrets_dir = rbtdrk_read_env_value(&rbrr, RBTDRK_FIELD_RBRR_SECRETS_DIR)
        .ok_or_else(|| format!("RBRR_SECRETS_DIR missing from {}", rbrr.display()))?;
    if secrets_dir.is_empty() {
        return Err(format!("RBRR_SECRETS_DIR is blank in {}", rbrr.display()));
    }
    Ok(rbtdrk_resolve(root, &secrets_dir)
        .join(role)
        .join(RBTDRK_RBRA_FILE))
}

// ── Canonical-prefix install (case 1) ───────────────────────

/// Idempotently install canc-/canr- prefixes into rbrr.env. Returns Ok
/// without committing when prefixes already match the canonical markers;
/// otherwise rewrites both lines and commits.
pub(crate) fn rbtdrk_install_canonical_prefixes(root: &Path) -> Result<(), String> {
    let rbrr = root.join(RBTDRK_RBRR_FILE);
    let cloud = rbtdrk_read_env_value(&rbrr, RBTDRK_FIELD_RBRR_CLOUD_PREFIX).unwrap_or_default();
    let runtime =
        rbtdrk_read_env_value(&rbrr, RBTDRK_FIELD_RBRR_RUNTIME_PREFIX).unwrap_or_default();
    if cloud == RBTDRK_CANONICAL_CLOUD_PREFIX && runtime == RBTDRK_CANONICAL_RUNTIME_PREFIX {
        return Ok(());
    }
    let content = std::fs::read_to_string(&rbrr)
        .map_err(|e| format!("rbtdrk: read {}: {}", rbrr.display(), e))?;
    let new_content = rbtdrk_replace_env_fields(
        &content,
        &[
            (
                RBTDRK_FIELD_RBRR_CLOUD_PREFIX,
                RBTDRK_CANONICAL_CLOUD_PREFIX,
            ),
            (
                RBTDRK_FIELD_RBRR_RUNTIME_PREFIX,
                RBTDRK_CANONICAL_RUNTIME_PREFIX,
            ),
        ],
    );
    std::fs::write(&rbrr, &new_content)
        .map_err(|e| format!("rbtdrk: write {}: {}", rbrr.display(), e))?;
    let commit_msg = format!(
        "canonical-establish fixture: install canonical RBRR prefixes ({}/{})",
        RBTDRK_CANONICAL_CLOUD_PREFIX, RBTDRK_CANONICAL_RUNTIME_PREFIX
    );
    rbtdrk_git_add_and_commit(root, RBTDRK_RBRR_FILE, &commit_msg)
}

fn rbtdrk_install_depot_moniker(root: &Path, moniker: &str) -> Result<(), String> {
    let rbrr = root.join(RBTDRK_RBRR_FILE);
    let content = std::fs::read_to_string(&rbrr)
        .map_err(|e| format!("rbtdrk: read {}: {}", rbrr.display(), e))?;
    let new_content =
        rbtdrk_replace_env_fields(&content, &[(RBTDRK_FIELD_RBRR_DEPOT_MONIKER, moniker)]);
    std::fs::write(&rbrr, &new_content)
        .map_err(|e| format!("rbtdrk: write {}: {}", rbrr.display(), e))?;
    let commit_msg = format!(
        "canonical-establish fixture: set {}={}",
        RBTDRK_FIELD_RBRR_DEPOT_MONIKER, moniker
    );
    rbtdrk_git_add_and_commit(root, RBTDRK_RBRR_FILE, &commit_msg)
}

/// Compose depot project_id from kindled regime values: <CLOUD>d-<moniker>.
fn rbtdrk_compose_project_id(root: &Path, moniker: &str) -> Result<String, String> {
    let rbrr = root.join(RBTDRK_RBRR_FILE);
    let cloud_prefix = rbtdrk_read_env_value(&rbrr, RBTDRK_FIELD_RBRR_CLOUD_PREFIX)
        .ok_or_else(|| format!("RBRR_CLOUD_PREFIX missing from {}", rbrr.display()))?;
    Ok(format!("{}d-{}", cloud_prefix, moniker))
}

/// Pick the next free moniker for `family_stem` by walking the depot_list
/// invocation's BURV output dir for `<family>NNNNNN.depot` files. Returns
/// `<family>RBTDRK_FAMILY_NUMERIC_FLOOR` when no matching files exist.
///
/// Caller contract: `list_result` MUST be from a freshly-invoked depot_list
/// that ran in the current process. The fact-file scan IS the collision
/// check — stale state means picking a colliding moniker. Do not reuse a
/// `list_result` across cases or pass an operator-cached value.
fn rbtdrk_pick_next_moniker(
    list_result: &rbtdri_InvokeResult,
    family_stem: &str,
) -> Result<String, String> {
    let dir = list_result.burv_output.join(RBTDRI_BURV_OUTPUT_SUBDIR);
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
    let rbrr = root.join(RBTDRK_RBRR_FILE);
    if !rbrr.exists() {
        return Err(format!("rbrr.env not found at {}", rbrr.display()));
    }
    Ok(())
}

/// Case 2 probe: canonical depot moniker installed in rbrr.env. Established
/// by case 1; absence means depot-levy didn't run or rbrr.env was rewritten.
fn rbtdrk_probe_canonical_moniker() -> Result<(), String> {
    let root = rbtdrk_probe_root()?;
    let rbrr = root.join(RBTDRK_RBRR_FILE);
    let moniker =
        rbtdrk_read_env_value(&rbrr, RBTDRK_FIELD_RBRR_DEPOT_MONIKER).unwrap_or_default();
    if !moniker.starts_with(RBTDRK_FAMILY_STEM) {
        return Err(format!(
            "{}={:?} does not begin with '{}' — canonical depot moniker not installed",
            RBTDRK_FIELD_RBRR_DEPOT_MONIKER, moniker, RBTDRK_FAMILY_STEM
        ));
    }
    Ok(())
}

/// Cases 3 and 4 probe: governor RBRA file present at the canonical path.
/// Established by case 2's mantle + canonical-copy step.
fn rbtdrk_probe_governor_rbra() -> Result<(), String> {
    let root = rbtdrk_probe_root()?;
    let path = rbtdrk_canonical_rbra(&root, "governor")?;
    if !path.exists() {
        return Err(format!("governor RBRA absent at {}", path.display()));
    }
    Ok(())
}

// ── Cases ────────────────────────────────────────────────────

/// Case 1 — canonical depot levy. Installs canc-/canr- prefixes, picks a
/// fresh canest moniker, levies the depot, cross-checks project_id from
/// fact-file against the RBDC compose derivation.
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
        RBTDRM_COLOPHON_DEPOT_LIST,
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

    let moniker = match rbtdrk_pick_next_moniker(&list_pre, RBTDRK_FAMILY_STEM) {
        Ok(m) => m,
        Err(e) => return rbtdre_Verdict::Fail(format!("pick next moniker: {}", e)),
    };
    if let Err(e) = rbtdrk_install_depot_moniker(&root, &moniker) {
        return rbtdre_Verdict::Fail(format!("install depot moniker: {}", e));
    }

    let levy = match rbtdrk_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_LEVY,
        &[],
        &[],
        dir,
        "levy",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("depot levy: {}", e)),
    };
    if levy.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "depot levy exit {}\n{}",
            levy.exit_code, levy.stderr
        ));
    }

    let list_present = match rbtdrk_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_LIST,
        &[],
        &[],
        dir,
        "list-present",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("depot list (after levy): {}", e)),
    };
    if list_present.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "depot list (after levy) exit {}\n{}",
            list_present.exit_code, list_present.stderr
        ));
    }

    let fact_path = list_present
        .burv_output
        .join(RBTDRI_BURV_OUTPUT_SUBDIR)
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

/// Case 2 — governor mantle. Mantles governor against the canonical depot,
/// reads the SA email fact, copies governor RBRA from BURV output to the
/// canonical path under RBRR_SECRETS_DIR.
fn rbtdrk_governor_mantle(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "canonical depot moniker installed",
        check: rbtdrk_probe_canonical_moniker,
        remediation: "rerun rbtdrk_depot_levy or the full canonical-establish fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrk_governor_mantle_impl(ctx, dir))
}

fn rbtdrk_governor_mantle_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let assay = match rbtdrk_canonical_rbra(&root, "assay") {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical assay RBRA path: {}", e)),
    };

    let mantle = match rbtdrk_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_GOV_MANTLE,
        &[],
        &[],
        dir,
        "mantle",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("governor mantle: {}", e)),
    };
    if mantle.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "governor mantle exit {}\n{}",
            mantle.exit_code, mantle.stderr
        ));
    }

    let email = match rbtdri_read_burv_fact(&mantle, RBTDRK_FACT_GOVERNOR_SA_EMAIL) {
        Ok(s) => s,
        Err(e) => return rbtdre_Verdict::Fail(format!("read governor SA email fact: {}", e)),
    };
    let _ = std::fs::write(dir.join("governor-sa-email.txt"), &email);

    if !assay.exists() {
        return rbtdre_Verdict::Fail(format!(
            "assay RBRA absent after governor mantle: {}",
            assay.display()
        ));
    }

    let canonical = match rbtdrk_canonical_rbra(&root, "governor") {
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

/// Case 3 — retriever invest + access-probe. The access-probe doubles as the
/// IAM-propagation gate: the invest tabtarget is responsible for waiting on
/// propagation before exiting (or the probe iterates internally). No separate
/// propagation-wait case.
fn rbtdrk_retriever_invest(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_mantle or the full canonical-establish fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| {
        rbtdrk_role_invest_impl(
            ctx,
            dir,
            RBTDRM_COLOPHON_GOV_INVEST_RETRIEVER,
            RBTDRK_IDENTITY_RETRIEVER,
            "retriever",
        )
    })
}

/// Case 4 — director invest + access-probe.
fn rbtdrk_director_invest(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "governor RBRA present",
        check: rbtdrk_probe_governor_rbra,
        remediation: "rerun rbtdrk_governor_mantle or the full canonical-establish fixture",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| {
        rbtdrk_role_invest_impl(
            ctx,
            dir,
            RBTDRM_COLOPHON_GOV_INVEST_DIRECTOR,
            RBTDRK_IDENTITY_DIRECTOR,
            "director",
        )
    })
}

/// Shared invest body for retriever and director: invest → assert assay
/// dropped by the invest tabtarget → copy to canonical role path → access-probe.
fn rbtdrk_role_invest_impl(
    ctx: &mut rbtdri_Context,
    dir: &Path,
    invest_colophon: &str,
    identity: &str,
    role: &str,
) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let assay = match rbtdrk_canonical_rbra(&root, "assay") {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical assay RBRA path: {}", e)),
    };

    let label_invest = format!("invest-{}", role);
    let invest = match rbtdrk_invoke_logged(
        ctx,
        invest_colophon,
        &[identity],
        &[],
        dir,
        &label_invest,
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("invest {}: {}", role, e)),
    };
    if invest.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "invest {} exit {}\n{}",
            role, invest.exit_code, invest.stderr
        ));
    }

    if !assay.exists() {
        return rbtdre_Verdict::Fail(format!(
            "assay RBRA absent after invest-{}: {}",
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

    let probe_result = match rbtdri_invoke_imprint(ctx, RBTDRM_COLOPHON_ACCESS_PROBE, role, &[]) {
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

pub static RBTDRK_SECTIONS_CANONICAL_ESTABLISH: &[rbtdre_Section] = &[rbtdre_Section {
    name: "canonical-establish-arc",
    cases: &[
        case!(rbtdrk_depot_levy),
        case!(rbtdrk_governor_mantle),
        case!(rbtdrk_retriever_invest),
        case!(rbtdrk_director_invest),
    ],
}];
