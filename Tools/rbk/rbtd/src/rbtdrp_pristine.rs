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
// RBTDRP — pristine-lifecycle fixture for theurge release qualification
//
// Case 1 (marshal-zero-attestation) is the entry gate for `rbw-tP`. It asserts
// that the working tree was just zeroed by `rbw-MZ` (rblm_zero) — five
// violation classes are checked and ALL surfaced in one aggregated diagnostic.
// Failure short-circuits subsequent cases via the per-fixture fail_fast switch.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::case;
use crate::rbtdrc_crucible::rbtdrc_with_ctx;
use crate::rbtdre_engine::{rbtdre_Section, rbtdre_Verdict};
use crate::rbtdri_invocation::{rbtdri_invoke_global, rbtdri_read_burv_fact, rbtdri_Context};
use crate::rbtdrm_manifest::{
    RBTDRM_COLOPHON_DEPOT_LEVY, RBTDRM_COLOPHON_DEPOT_LIST, RBTDRM_COLOPHON_DEPOT_UNMAKE,
    RBTDRM_COLOPHON_GOV_FORFEIT, RBTDRM_COLOPHON_GOV_LIST_SAS, RBTDRM_COLOPHON_GOV_MANTLE,
};

/// RBRR field names referenced by the pristine-lifecycle fixture. Field
/// identifiers extracted as consts so the blank-field array and the
/// throwaway-prefix install helper share a single definition site.
const RBTDRP_FIELD_RBRR_CLOUD_PREFIX: &str = "RBRR_CLOUD_PREFIX";
const RBTDRP_FIELD_RBRR_RUNTIME_PREFIX: &str = "RBRR_RUNTIME_PREFIX";
const RBTDRP_FIELD_RBRR_DEPOT_MONIKER: &str = "RBRR_DEPOT_MONIKER";

/// Site-specific RBRR fields that rblm_zero blanks. These three fields define
/// the depot-bound site identity (project ID, GAR repo, pool stem, and bucket
/// derive from CLOUD_PREFIX + DEPOT_MONIKER at kindle); an empty value is the
/// post-marshal-zero invariant.
const RBTDRP_RBRR_BLANK_FIELDS: &[&str] = &[
    RBTDRP_FIELD_RBRR_CLOUD_PREFIX,
    RBTDRP_FIELD_RBRR_RUNTIME_PREFIX,
    RBTDRP_FIELD_RBRR_DEPOT_MONIKER,
];

/// Throwaway RBRR prefix values stamped into rbrr.env by
/// `rbtdrp_install_throwaway_prefixes`. Pristine-lifecycle cases that need
/// non-blank prefixes (depot/governor/retriever/director lifecycle) call the
/// helper to install these values; the marker shape distinguishes throwaway
/// from operator-chosen canonical values.
pub(crate) const RBTDRP_THROWAWAY_CLOUD_PREFIX: &str = "prlc-";
pub(crate) const RBTDRP_THROWAWAY_RUNTIME_PREFIX: &str = "prlr-";

/// Throwaway depot monikers used by cases 2 and 3. Distinct values keep the
/// two cases' depots separable when both run in the same fixture pass.
const RBTDRP_DEPOT_MONIKER_LIFECYCLE: &str = "pristq01";
const RBTDRP_DEPOT_MONIKER_GOVERNOR: &str = "pristg01";

/// Fact-file names emitted by the rbgp tabtargets (mirror of bash consts in
/// rbgc_Constants.sh). Cases read these values from the BURV output dir of
/// the producing invocation.
const RBTDRP_FACT_DEPOT_PROJECT_ID: &str = "rbgp_fact_depot_project_id";
const RBTDRP_FACT_GOVERNOR_SA_EMAIL: &str = "rbgp_fact_governor_sa_email";

/// BURE confirmation override (mirrors buc_command.sh): when set to "skip",
/// `buc_require` accepts the operation without interactive prompt.
const RBTDRP_BURE_CONFIRM_KEY: &str = "BURE_CONFIRM";
const RBTDRP_BURE_CONFIRM_SKIP: &str = "skip";

/// `DELETE_REQUESTED` lifecycle state — appears in `rbgp_depot_list` output
/// after a soft-delete, used by case 2 to relax the post-unmake assertion.
const RBTDRP_DELETE_REQUESTED: &str = "DELETE_REQUESTED";

/// Roles whose RBRA credential files rblm_zero deletes.
const RBTDRP_RBRA_ROLES: &[&str] = &["governor", "director", "retriever", "assay"];

/// Nameplate hallmark fields rblm_zero blanks.
const RBTDRP_RBRN_BLANK_FIELDS: &[&str] = &["RBRN_SENTRY_HALLMARK", "RBRN_BOTTLE_HALLMARK"];

/// File-relative constants matching rbbc_constants.sh / rbcc_Constants.sh.
const RBTDRP_DOT_DIR: &str = ".rbk";
const RBTDRP_RBRR_FILE: &str = ".rbk/rbrr.env";
const RBTDRP_RBRA_FILE: &str = "rbra.env";
const RBTDRP_RBRN_FILE: &str = "rbrn.env";
const RBTDRP_RBRV_FILE: &str = "rbrv.env";

// ── Helpers ──────────────────────────────────────────────────

/// Read an env file and return the value for `key`, or None if the key is
/// absent. The bash regime files use `KEY=value` lines (unquoted); comment
/// and blank lines are skipped. The trailing newline is stripped from value.
fn rbtdrp_read_env_value(path: &Path, key: &str) -> Option<String> {
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
fn rbtdrp_resolve(root: &Path, raw: &str) -> PathBuf {
    if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        root.join(raw)
    }
}

/// Class A — working tree clean (`git status --porcelain` empty).
fn rbtdrp_check_tree_clean(root: &Path, violations: &mut Vec<String>) {
    match Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(root)
        .output()
    {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let trimmed = stdout.trim();
            if !trimmed.is_empty() {
                violations.push(format!(
                    "working tree not clean — uncommitted changes:\n{}",
                    trimmed
                ));
            }
        }
        Ok(out) => violations.push(format!(
            "git status --porcelain failed (exit {}): {}",
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stderr).trim()
        )),
        Err(e) => violations.push(format!("git status invocation failed: {}", e)),
    }
}

/// Class B — five site-specific RBRR fields are blank.
fn rbtdrp_check_rbrr_fields(rbrr: &Path, violations: &mut Vec<String>) {
    for field in RBTDRP_RBRR_BLANK_FIELDS {
        if let Some(value) = rbtdrp_read_env_value(rbrr, field) {
            if !value.is_empty() {
                violations.push(format!(
                    "RBRR field non-blank: {}={} (in {})",
                    field,
                    value,
                    rbrr.display()
                ));
            }
        }
    }
}

/// Class C — no RBRA credential files at the four role paths.
fn rbtdrp_check_rbra_files(root: &Path, secrets_dir: &str, violations: &mut Vec<String>) {
    if secrets_dir.is_empty() {
        violations.push(
            "RBRR_SECRETS_DIR is blank — cannot resolve credential paths".to_string(),
        );
        return;
    }
    let secrets = rbtdrp_resolve(root, secrets_dir);
    for role in RBTDRP_RBRA_ROLES {
        let path = secrets.join(role).join(RBTDRP_RBRA_FILE);
        if path.exists() {
            violations.push(format!("RBRA credential present: {}", path.display()));
        }
    }
}

/// Class D — every nameplate's RBRN_SENTRY_HALLMARK and RBRN_BOTTLE_HALLMARK
/// is blank.
fn rbtdrp_check_nameplate_hallmarks(root: &Path, violations: &mut Vec<String>) {
    let dot_dir = root.join(RBTDRP_DOT_DIR);
    let entries = match std::fs::read_dir(&dot_dir) {
        Ok(e) => e,
        Err(e) => {
            violations.push(format!("cannot read {}: {}", dot_dir.display(), e));
            return;
        }
    };
    for entry in entries.flatten() {
        let np_dir = entry.path();
        if !np_dir.is_dir() {
            continue;
        }
        let rbrn = np_dir.join(RBTDRP_RBRN_FILE);
        if !rbrn.exists() {
            continue;
        }
        for field in RBTDRP_RBRN_BLANK_FIELDS {
            if let Some(value) = rbtdrp_read_env_value(&rbrn, field) {
                if !value.is_empty() {
                    violations.push(format!(
                        "nameplate hallmark non-blank: {}={} (in {})",
                        field,
                        value,
                        rbrn.display()
                    ));
                }
            }
        }
    }
}

/// Class E — every vessel rbrv.env has RBRV_RELIQUARY blank and every
/// RBRV_IMAGE_*_ANCHOR field blank.
fn rbtdrp_check_vessel_depot_fields(
    root: &Path,
    vessel_dir: &str,
    violations: &mut Vec<String>,
) {
    if vessel_dir.is_empty() {
        violations.push("RBRR_VESSEL_DIR is blank — cannot scan vessels".to_string());
        return;
    }
    let vroot = rbtdrp_resolve(root, vessel_dir);
    let entries = match std::fs::read_dir(&vroot) {
        Ok(e) => e,
        Err(e) => {
            violations.push(format!("cannot read {}: {}", vroot.display(), e));
            return;
        }
    };
    for entry in entries.flatten() {
        let v_dir = entry.path();
        if !v_dir.is_dir() {
            continue;
        }
        let rbrv = v_dir.join(RBTDRP_RBRV_FILE);
        if !rbrv.exists() {
            continue;
        }
        rbtdrp_scan_rbrv_file(&rbrv, violations);
    }
}

/// Scan a single rbrv.env for non-blank RBRV_RELIQUARY and RBRV_IMAGE_*_ANCHOR
/// fields.
fn rbtdrp_scan_rbrv_file(rbrv: &Path, violations: &mut Vec<String>) {
    let content = match std::fs::read_to_string(rbrv) {
        Ok(c) => c,
        Err(e) => {
            violations.push(format!("cannot read {}: {}", rbrv.display(), e));
            return;
        }
    };
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        let (key, value) = match trimmed.split_once('=') {
            Some(kv) => kv,
            None => continue,
        };
        let is_reliquary = key == "RBRV_RELIQUARY";
        let is_anchor = key.starts_with("RBRV_IMAGE_") && key.ends_with("_ANCHOR");
        if (is_reliquary || is_anchor) && !value.is_empty() {
            violations.push(format!(
                "vessel depot-scoped field non-blank: {}={} (in {})",
                key,
                value,
                rbrv.display()
            ));
        }
    }
}

// ── Env-file rewrite + git commit helpers ────────────────────

/// Rewrite each line in `content` whose key matches an entry in `pairs`
/// (matched as `KEY=`) to `KEY=value`. Lines whose keys are not in `pairs`
/// pass through unchanged. The trailing-newline shape is preserved.
fn rbtdrp_replace_env_fields(content: &str, pairs: &[(&str, &str)]) -> String {
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

/// `git add <file> && git commit -m <message>` from `root`. Bubbles the
/// underlying stderr on either step's failure.
fn rbtdrp_git_add_and_commit(root: &Path, file: &str, message: &str) -> Result<(), String> {
    let add = Command::new("git")
        .args(["add", file])
        .current_dir(root)
        .output()
        .map_err(|e| format!("rbtdrp: git add invocation failed: {}", e))?;
    if !add.status.success() {
        return Err(format!(
            "rbtdrp: git add failed (exit {}): {}",
            add.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&add.stderr).trim()
        ));
    }

    let commit = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(root)
        .output()
        .map_err(|e| format!("rbtdrp: git commit invocation failed: {}", e))?;
    if !commit.status.success() {
        return Err(format!(
            "rbtdrp: git commit failed (exit {}): {}",
            commit.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&commit.stderr).trim()
        ));
    }

    Ok(())
}

// ── Throwaway-prefix install ─────────────────────────────────

/// Idempotently install throwaway RBRR prefixes into `.rbk/rbrr.env`.
///
/// Pristine-lifecycle cases that need non-blank prefixes (depot/governor/
/// retriever/director lifecycle) call this as their first step. The helper
/// reads current values; if `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX`
/// already match the throwaway markers, returns `Ok(())` with no side
/// effect. Otherwise rewrites both lines in place, then `git add` +
/// `git commit` the change. One commit per fixture run regardless of which
/// case is the first to call it.
///
/// HEAD walks off marshal-zero after the commit; recovery is `rbw-MZ`,
/// matching the pristine fixture's start-over-from-zero failure mode.
pub(crate) fn rbtdrp_install_throwaway_prefixes(root: &Path) -> Result<(), String> {
    let rbrr = root.join(RBTDRP_RBRR_FILE);
    let cloud = rbtdrp_read_env_value(&rbrr, RBTDRP_FIELD_RBRR_CLOUD_PREFIX).unwrap_or_default();
    let runtime =
        rbtdrp_read_env_value(&rbrr, RBTDRP_FIELD_RBRR_RUNTIME_PREFIX).unwrap_or_default();

    if cloud == RBTDRP_THROWAWAY_CLOUD_PREFIX && runtime == RBTDRP_THROWAWAY_RUNTIME_PREFIX {
        return Ok(());
    }

    let content = std::fs::read_to_string(&rbrr)
        .map_err(|e| format!("rbtdrp: read {}: {}", rbrr.display(), e))?;
    let new_content = rbtdrp_replace_env_fields(
        &content,
        &[
            (RBTDRP_FIELD_RBRR_CLOUD_PREFIX, RBTDRP_THROWAWAY_CLOUD_PREFIX),
            (
                RBTDRP_FIELD_RBRR_RUNTIME_PREFIX,
                RBTDRP_THROWAWAY_RUNTIME_PREFIX,
            ),
        ],
    );
    std::fs::write(&rbrr, &new_content)
        .map_err(|e| format!("rbtdrp: write {}: {}", rbrr.display(), e))?;

    let commit_msg = format!(
        "pristine-lifecycle fixture: install throwaway RBRR prefixes ({}/{})",
        RBTDRP_THROWAWAY_CLOUD_PREFIX, RBTDRP_THROWAWAY_RUNTIME_PREFIX
    );
    rbtdrp_git_add_and_commit(root, RBTDRP_RBRR_FILE, &commit_msg)
}

/// Set `RBRR_DEPOT_MONIKER` in rbrr.env to `moniker` and commit. Step-5
/// callers (pristine cases 2-3) use this to install a moniker that drives
/// `RBDC_DEPOT_PROJECT_ID` via kindle derivation. The commit is separate
/// from the throwaway-prefix commit so the audit trail shows the moniker
/// landing as its own pace.
fn rbtdrp_install_depot_moniker(root: &Path, moniker: &str) -> Result<(), String> {
    let rbrr = root.join(RBTDRP_RBRR_FILE);
    let content = std::fs::read_to_string(&rbrr)
        .map_err(|e| format!("rbtdrp: read {}: {}", rbrr.display(), e))?;
    let new_content = rbtdrp_replace_env_fields(
        &content,
        &[(RBTDRP_FIELD_RBRR_DEPOT_MONIKER, moniker)],
    );
    std::fs::write(&rbrr, &new_content)
        .map_err(|e| format!("rbtdrp: write {}: {}", rbrr.display(), e))?;

    let commit_msg = format!(
        "pristine-lifecycle fixture: set {}={}",
        RBTDRP_FIELD_RBRR_DEPOT_MONIKER, moniker
    );
    rbtdrp_git_add_and_commit(root, RBTDRP_RBRR_FILE, &commit_msg)
}

// ── Case ─────────────────────────────────────────────────────

/// Case 1 — marshal-zero attestation. Aggregates all five violation classes
/// into a single diagnostic. A passing run is the proof that `rbw-MZ` was
/// just executed and committed.
fn rbtdrp_marshal_zero_attestation(_dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot resolve project root: {}", e)),
    };

    let rbrr = root.join(RBTDRP_RBRR_FILE);
    if !rbrr.exists() {
        return rbtdre_Verdict::Fail(format!("RBRR file not found: {}", rbrr.display()));
    }

    let secrets_dir = rbtdrp_read_env_value(&rbrr, "RBRR_SECRETS_DIR").unwrap_or_default();
    let vessel_dir = rbtdrp_read_env_value(&rbrr, "RBRR_VESSEL_DIR").unwrap_or_default();

    let mut violations: Vec<String> = Vec::new();

    rbtdrp_check_tree_clean(&root, &mut violations);
    rbtdrp_check_rbrr_fields(&rbrr, &mut violations);
    rbtdrp_check_rbra_files(&root, &secrets_dir, &mut violations);
    rbtdrp_check_nameplate_hallmarks(&root, &mut violations);
    rbtdrp_check_vessel_depot_fields(&root, &vessel_dir, &mut violations);

    if violations.is_empty() {
        return rbtdre_Verdict::Pass;
    }

    let body = violations
        .iter()
        .map(|v| format!("  - {}", v))
        .collect::<Vec<_>>()
        .join("\n");
    rbtdre_Verdict::Fail(format!(
        "marshal-zero attestation failed — {} violation(s):\n{}\n\n\
         remedy: tt/rbw-MZ.MarshalZeroes.sh, then commit and rerun.",
        violations.len(),
        body
    ))
}

// ── Cases 2 & 3 — depot and governor lifecycles ──────────────

/// Wrapper invocation: call `rbtdri_invoke_global` and tee stdout/stderr to
/// `dir/<label>-stdout.txt` / `dir/<label>-stderr.txt` for diagnostic review.
/// Returns `Ok(InvokeResult)` regardless of exit code; callers decide what
/// counts as failure.
fn rbtdrp_invoke_logged(
    ctx: &mut rbtdri_Context,
    colophon: &str,
    args: &[&str],
    extra_env: &[(&str, &str)],
    dir: &Path,
    label: &str,
) -> Result<crate::rbtdri_invocation::rbtdri_InvokeResult, String> {
    let result = rbtdri_invoke_global(ctx, colophon, args, extra_env)?;
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &result.stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &result.stderr);
    Ok(result)
}

/// Case 2 — depot-lifecycle. Levies a throwaway depot, asserts it appears
/// in `rbgp_depot_list`, soft-deletes it, then asserts it is absent or in
/// `DELETE_REQUESTED` state.
///
/// The depot project_id is captured from the levy invocation's
/// `rbgp_fact_depot_project_id` fact file rather than scraped from stdout.
fn rbtdrp_depot_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrp_depot_lifecycle_impl(ctx, dir))
}

fn rbtdrp_depot_lifecycle_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    if let Err(e) = rbtdrp_install_throwaway_prefixes(&root) {
        return rbtdre_Verdict::Fail(format!("install throwaway prefixes: {}", e));
    }

    let levy = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_LEVY,
        &[RBTDRP_DEPOT_MONIKER_LIFECYCLE],
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

    let project_id = match rbtdri_read_burv_fact(&levy, RBTDRP_FACT_DEPOT_PROJECT_ID) {
        Ok(s) => s,
        Err(e) => return rbtdre_Verdict::Fail(format!("read depot project_id fact: {}", e)),
    };
    let _ = std::fs::write(dir.join("project-id.txt"), &project_id);

    let list_present = match rbtdrp_invoke_logged(
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
    if !list_present.stdout.contains(&project_id) {
        return rbtdre_Verdict::Fail(format!(
            "project_id '{}' missing from depot list after levy:\n{}",
            project_id, list_present.stdout
        ));
    }

    let unmake = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_UNMAKE,
        &[&project_id],
        &[(RBTDRP_BURE_CONFIRM_KEY, RBTDRP_BURE_CONFIRM_SKIP)],
        dir,
        "unmake",
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

    let list_after = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_LIST,
        &[],
        &[],
        dir,
        "list-after",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("depot list (after unmake): {}", e)),
    };
    if list_after.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "depot list (after unmake) exit {}\n{}",
            list_after.exit_code, list_after.stderr
        ));
    }
    for line in list_after.stdout.lines() {
        if line.contains(&project_id) && !line.contains(RBTDRP_DELETE_REQUESTED) {
            return rbtdre_Verdict::Fail(format!(
                "project_id '{}' still present without {} after unmake:\n{}",
                project_id, RBTDRP_DELETE_REQUESTED, line
            ));
        }
    }

    rbtdre_Verdict::Pass
}

/// Case 3 — governor-lifecycle. Levies its own throwaway depot (so
/// `RBRR_DEPOT_PROJECT_ID` becomes set, satisfying the precondition for
/// `rbgp_governor_mantle`), mantles a governor, asserts it appears in
/// `rbgg_list_service_accounts`, forfeits it, asserts absent, then unmakes
/// the depot to clean up.
///
/// Governor lifecycle uses a moniker distinct from case 2 so both cases'
/// depots are separable when the full fixture runs.
fn rbtdrp_governor_lifecycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrp_governor_lifecycle_impl(ctx, dir))
}

fn rbtdrp_governor_lifecycle_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    if let Err(e) = rbtdrp_install_throwaway_prefixes(&root) {
        return rbtdre_Verdict::Fail(format!("install throwaway prefixes: {}", e));
    }

    let levy = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_LEVY,
        &[RBTDRP_DEPOT_MONIKER_GOVERNOR],
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

    let project_id = match rbtdri_read_burv_fact(&levy, RBTDRP_FACT_DEPOT_PROJECT_ID) {
        Ok(s) => s,
        Err(e) => return rbtdre_Verdict::Fail(format!("read depot project_id fact: {}", e)),
    };
    let _ = std::fs::write(dir.join("project-id.txt"), &project_id);

    // STEP-5-PENDING: post-collapse, case 3 should set RBRR_DEPOT_MONIKER
    // before invoking levy (so RBDC_DEPOT_PROJECT_ID drives all derived
    // names). Step 1 leaves the previous edit-and-commit pattern in place
    // semantically inverted — fixture run is service-tier-broken until
    // Step 5 reshapes case 3's ordering.
    if let Err(e) = rbtdrp_install_depot_moniker(&root, &project_id) {
        return rbtdre_Verdict::Fail(format!("set RBRR_DEPOT_MONIKER: {}", e));
    }

    let mantle = match rbtdrp_invoke_logged(
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

    let sa_email = match rbtdri_read_burv_fact(&mantle, RBTDRP_FACT_GOVERNOR_SA_EMAIL) {
        Ok(s) => s,
        Err(e) => return rbtdre_Verdict::Fail(format!("read governor SA email fact: {}", e)),
    };
    let _ = std::fs::write(dir.join("sa-email.txt"), &sa_email);

    let list_present = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_GOV_LIST_SAS,
        &[],
        &[],
        dir,
        "list-present",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("list SAs (after mantle): {}", e)),
    };
    if list_present.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "list SAs (after mantle) exit {}\n{}",
            list_present.exit_code, list_present.stderr
        ));
    }
    if !list_present.stdout.contains(&sa_email) {
        return rbtdre_Verdict::Fail(format!(
            "governor SA '{}' missing from list after mantle:\n{}",
            sa_email, list_present.stdout
        ));
    }

    let forfeit = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_GOV_FORFEIT,
        &[&sa_email],
        &[],
        dir,
        "forfeit",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("governor forfeit: {}", e)),
    };
    if forfeit.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "governor forfeit exit {}\n{}",
            forfeit.exit_code, forfeit.stderr
        ));
    }

    let list_after = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_GOV_LIST_SAS,
        &[],
        &[],
        dir,
        "list-after",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("list SAs (after forfeit): {}", e)),
    };
    if list_after.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "list SAs (after forfeit) exit {}\n{}",
            list_after.exit_code, list_after.stderr
        ));
    }
    if list_after.stdout.contains(&sa_email) {
        return rbtdre_Verdict::Fail(format!(
            "governor SA '{}' still present after forfeit:\n{}",
            sa_email, list_after.stdout
        ));
    }

    let unmake = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_UNMAKE,
        &[&project_id],
        &[(RBTDRP_BURE_CONFIRM_KEY, RBTDRP_BURE_CONFIRM_SKIP)],
        dir,
        "unmake",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("depot unmake (cleanup): {}", e)),
    };
    if unmake.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "depot unmake (cleanup) exit {}\n{}",
            unmake.exit_code, unmake.stderr
        ));
    }

    rbtdre_Verdict::Pass
}

// ── Section registry ─────────────────────────────────────────

pub static RBTDRP_SECTIONS_PRISTINE_LIFECYCLE: &[rbtdre_Section] = &[
    rbtdre_Section {
        name: "pristine-lifecycle-gate",
        cases: &[case!(rbtdrp_marshal_zero_attestation)],
    },
    rbtdre_Section {
        name: "pristine-lifecycle-simple",
        cases: &[
            case!(rbtdrp_depot_lifecycle),
            case!(rbtdrp_governor_lifecycle),
        ],
    },
];
