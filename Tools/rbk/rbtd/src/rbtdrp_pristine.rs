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
use crate::rbtdrb_probe::{rbtdrb_assert, rbtdrb_Probe};
use crate::rbtdrc_crucible::rbtdrc_with_ctx;
use crate::rbtdre_engine::{rbtdre_Disposition, rbtdre_Fixture, rbtdre_Section, rbtdre_Verdict};
use crate::rbtdri_invocation::{
    rbtdri_invoke_global, rbtdri_invoke_imprint, rbtdri_read_burv_fact, rbtdri_Context,
    RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP, RBTDRI_BURV_OUTPUT_SUBDIR,
};
use crate::rbtdrm_manifest::{
    RBTDRM_COLOPHON_ACCESS_PROBE, RBTDRM_COLOPHON_DEPOT_LEVY, RBTDRM_COLOPHON_DEPOT_LIST,
    RBTDRM_COLOPHON_DEPOT_UNMAKE, RBTDRM_COLOPHON_GOV_DIVEST_DIRECTOR,
    RBTDRM_COLOPHON_GOV_DIVEST_RETRIEVER, RBTDRM_COLOPHON_GOV_INVEST_DIRECTOR,
    RBTDRM_COLOPHON_GOV_INVEST_RETRIEVER, RBTDRM_COLOPHON_GOV_MANTLE,
    RBTDRM_FIXTURE_PRISTINE_LIFECYCLE,
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

/// Family stem for the fused arc section. One depot shared across all three
/// arc cases; the stem keeps arc monikers separable from other fixture
/// families. Cases pick a numeric six-digit suffix at runtime by walking
/// emitted depot fact files and incrementing past the highest existing
/// suffix per family.
const RBTDRP_FAMILY_STEM_ARC: &str = "pristl";

/// Static identities for the SA cycle case. The invest colophons compose
/// SA account names as `<role>-<identity>`; these are stable across runs
/// because each run uses a fresh throwaway depot project.
const RBTDRP_IDENTITY_RETRIEVER: &str = "pristl-ret";
const RBTDRP_IDENTITY_DIRECTOR: &str = "pristl-dir";

/// Lowest valid numeric suffix for autodetected monikers. Six-digit width
/// keeps composed project_id length predictable: cloud_prefix(<=11) +
/// "d-"(2) + family(6) + suffix(6) = <=25, well under the 30-char GCP limit.
const RBTDRP_FAMILY_NUMERIC_FLOOR: u32 = 100000;

/// Six-digit numeric suffix width — enforced when parsing existing fact-file
/// names so e.g. "pristq01" (legacy two-digit) is ignored rather than parsed
/// as 1 and shadowing real entries.
const RBTDRP_FAMILY_NUMERIC_WIDTH: usize = 6;

/// Fact-file extension mirror of RBCC_fact_ext_depot in rbcc_Constants.sh.
/// rbgp_depot_list emits `<moniker>.depot` files with state content.
const RBTDRP_FACT_EXT_DEPOT: &str = "depot";

/// Fact-file extension mirror of RBCC_fact_ext_depot_project in
/// rbcc_Constants.sh. rbgp_depot_list emits `<moniker>.depot-project` files
/// with project_id content.
const RBTDRP_FACT_EXT_DEPOT_PROJECT: &str = "depot-project";

/// Fact-file name for the governor SA email (mirror of
/// RBGP_FACT_GOVERNOR_SA_EMAIL from rbgc_Constants.sh). Read from the mantle
/// invocation's BURV output.
const RBTDRP_FACT_GOVERNOR_SA_EMAIL: &str = "rbgp_fact_governor_sa_email";

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

// ── Probes ───────────────────────────────────────────────────
//
// rbtdrb_Probe.check is `fn() -> Result<(), String>` with no parameters,
// so the probe reads the project root from current_dir() — theurge always
// launches from the project root.

/// Live-disqualify case probe: depot levied (RBRR_CLOUD_PREFIX +
/// RBRR_DEPOT_MONIKER both non-blank, RBDC kindle composes a non-empty
/// project_id). Established by case 2 (rbtdrp_depot_stand_up).
fn rbtdrp_probe_depot_levied() -> Result<(), String> {
    let root = std::env::current_dir()
        .map_err(|e| format!("cannot resolve project root: {}", e))?;
    let rbrr = root.join(RBTDRP_RBRR_FILE);

    let cloud = rbtdrp_read_env_value(&rbrr, RBTDRP_FIELD_RBRR_CLOUD_PREFIX).unwrap_or_default();
    if cloud.is_empty() {
        return Err(format!(
            "{} blank in {} — throwaway prefixes not installed",
            RBTDRP_FIELD_RBRR_CLOUD_PREFIX,
            rbrr.display()
        ));
    }

    let moniker = rbtdrp_read_env_value(&rbrr, RBTDRP_FIELD_RBRR_DEPOT_MONIKER).unwrap_or_default();
    if moniker.is_empty() {
        return Err(format!(
            "{} blank in {} — depot stand-up did not run",
            RBTDRP_FIELD_RBRR_DEPOT_MONIKER,
            rbrr.display()
        ));
    }

    Ok(())
}

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

/// Pick the next free moniker for `family_stem` by walking the depot_list
/// invocation's BURV output dir for `<family>NNNNNN.depot` files, parsing
/// the numeric suffix from each, and returning `<family><max + 1>`. Returns
/// `<family>RBTDRP_FAMILY_NUMERIC_FLOOR` when no matching files exist or the
/// output dir is missing (first run, no prior depots).
fn rbtdrp_pick_next_moniker(
    list_result: &crate::rbtdri_invocation::rbtdri_InvokeResult,
    family_stem: &str,
) -> Result<String, String> {
    let dir = list_result
        .burv_output
        .join(crate::rbtdri_invocation::RBTDRI_BURV_OUTPUT_SUBDIR);

    let entries = match std::fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => {
            return Ok(format!("{}{}", family_stem, RBTDRP_FAMILY_NUMERIC_FLOOR));
        }
    };

    let suffix_ext = format!(".{}", RBTDRP_FACT_EXT_DEPOT);
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
        if numeric.len() != RBTDRP_FAMILY_NUMERIC_WIDTH {
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
        None => RBTDRP_FAMILY_NUMERIC_FLOOR,
    };
    Ok(format!("{}{}", family_stem, next))
}

/// Compose the depot project_id from kindled regime values. Mirrors RBDC's
/// derivation: `${RBRR_CLOUD_PREFIX}d-${moniker}`. Used by cases 2 and 3 to
/// recover the project_id post-levy without re-reading a fact file (the
/// pre-collapse `rbgp_fact_depot_project_id` producer is gone).
fn rbtdrp_compose_project_id(root: &Path, moniker: &str) -> Result<String, String> {
    let rbrr = root.join(RBTDRP_RBRR_FILE);
    let cloud_prefix = rbtdrp_read_env_value(&rbrr, RBTDRP_FIELD_RBRR_CLOUD_PREFIX)
        .ok_or_else(|| format!("RBRR_CLOUD_PREFIX missing from {}", rbrr.display()))?;
    Ok(format!("{}d-{}", cloud_prefix, moniker))
}

/// Set `RBRR_DEPOT_MONIKER` in rbrr.env to `moniker` and commit. Cases 2-3
/// use this to install a moniker that drives `RBDC_DEPOT_PROJECT_ID` via
/// kindle derivation. The commit is separate from the throwaway-prefix
/// commit so the audit trail shows the moniker landing as its own pace.
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

// ── §2 pristine-lifecycle-arc (stand-up → SA cycle → tear-down) ─────────────

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

/// Resolve the canonical RBRA path for a role by reading RBRR_SECRETS_DIR
/// from rbrr.env and joining `<role>/rbra.env`.
fn rbtdrp_canonical_rbra(root: &Path, role: &str) -> Result<PathBuf, String> {
    let rbrr = root.join(RBTDRP_RBRR_FILE);
    let secrets_dir = rbtdrp_read_env_value(&rbrr, "RBRR_SECRETS_DIR")
        .ok_or_else(|| format!("RBRR_SECRETS_DIR missing from {}", rbrr.display()))?;
    if secrets_dir.is_empty() {
        return Err(format!("RBRR_SECRETS_DIR is blank in {}", rbrr.display()));
    }
    Ok(rbtdrp_resolve(root, &secrets_dir).join(role).join(RBTDRP_RBRA_FILE))
}

// ── Case 2: depot stand-up ───────────────────────────────────

/// Case 2 — depot stand-up. Installs throwaway prefixes, picks the next free
/// moniker in the `pristl` family, levies the depot, re-lists to refresh
/// facts, reads project_id from the `<moniker>.depot-project` fact file, and
/// cross-checks it against the RBDC compose derivation. The moniker survives
/// in rbrr.env for cases 3 and 4.
fn rbtdrp_depot_stand_up(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrp_depot_stand_up_impl(ctx, dir))
}

fn rbtdrp_depot_stand_up_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    if let Err(e) = rbtdrp_install_throwaway_prefixes(&root) {
        return rbtdre_Verdict::Fail(format!("install throwaway prefixes: {}", e));
    }

    let list_pre = match rbtdrp_invoke_logged(
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

    let moniker = match rbtdrp_pick_next_moniker(&list_pre, RBTDRP_FAMILY_STEM_ARC) {
        Ok(m) => m,
        Err(e) => return rbtdre_Verdict::Fail(format!("pick next moniker: {}", e)),
    };
    if let Err(e) = rbtdrp_install_depot_moniker(&root, &moniker) {
        return rbtdre_Verdict::Fail(format!("install depot moniker: {}", e));
    }

    let levy = match rbtdrp_invoke_logged(
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

    let fact_path = list_present
        .burv_output
        .join(RBTDRI_BURV_OUTPUT_SUBDIR)
        .join(format!("{}.{}", moniker, RBTDRP_FACT_EXT_DEPOT_PROJECT));
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

    let composed_project_id = match rbtdrp_compose_project_id(&root, &moniker) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("compose project_id: {}", e)),
    };
    if composed_project_id != fact_project_id {
        return rbtdre_Verdict::Fail(format!(
            "project_id mismatch: RBDC compose='{}' vs depot-list fact='{}' \
             (RBDC kindle derivation diverged from payor creation)",
            composed_project_id, fact_project_id
        ));
    }

    rbtdre_Verdict::Pass
}

// ── Case 3: SA cycle ─────────────────────────────────────────

/// Case 3 — SA cycle. Pre-condition: depot stood up by case 2 (moniker in
/// rbrr.env). Mantles governor (RBRA lands in BURV output; copy to canonical),
/// then for each role: invest → copy assay → canonical → access-probe.
/// Divests both roles in reverse order, verifies BBAAN's
/// divest-deletes-production-RBRA contract via canonical-path absence checks.
/// Best-effort cleanup of assay file at the end.
fn rbtdrp_sa_cycle(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrp_sa_cycle_impl(ctx, dir))
}

fn rbtdrp_sa_cycle_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let rbrr = root.join(RBTDRP_RBRR_FILE);
    let moniker = match rbtdrp_read_env_value(&rbrr, RBTDRP_FIELD_RBRR_DEPOT_MONIKER) {
        Some(m) if !m.is_empty() => m,
        _ => {
            return rbtdre_Verdict::Fail(
                "case 2 (stand-up) did not run or rbrr.env is missing the moniker \
                 (RBRR_DEPOT_MONIKER is blank)"
                    .to_string(),
            )
        }
    };
    let _ = std::fs::write(dir.join("moniker.txt"), &moniker);

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

    let governor_email = match rbtdri_read_burv_fact(&mantle, RBTDRP_FACT_GOVERNOR_SA_EMAIL) {
        Ok(s) => s,
        Err(e) => return rbtdre_Verdict::Fail(format!("read governor SA email fact: {}", e)),
    };
    let _ = std::fs::write(dir.join("governor-sa-email.txt"), &governor_email);

    let assay_canonical = match rbtdrp_canonical_rbra(&root, "assay") {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical assay RBRA path: {}", e)),
    };

    if !assay_canonical.exists() {
        return rbtdre_Verdict::Fail(format!(
            "assay RBRA absent after governor mantle: {}",
            assay_canonical.display()
        ));
    }

    let governor_canonical = match rbtdrp_canonical_rbra(&root, "governor") {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical governor RBRA path: {}", e)),
    };
    if let Some(parent) = governor_canonical.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return rbtdre_Verdict::Fail(format!(
                "create governor RBRA dir {}: {}",
                parent.display(),
                e
            ));
        }
    }
    if let Err(e) = std::fs::copy(&assay_canonical, &governor_canonical) {
        return rbtdre_Verdict::Fail(format!(
            "copy assay RBRA → governor canonical {}: {}",
            governor_canonical.display(),
            e
        ));
    }

    // Retriever: invest → assay → canonical → access-probe.
    let invest_ret = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_GOV_INVEST_RETRIEVER,
        &[RBTDRP_IDENTITY_RETRIEVER],
        &[],
        dir,
        "invest-retriever",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("invest retriever: {}", e)),
    };
    if invest_ret.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "invest retriever exit {}\n{}",
            invest_ret.exit_code, invest_ret.stderr
        ));
    }

    if !assay_canonical.exists() {
        return rbtdre_Verdict::Fail(format!(
            "assay RBRA absent after invest-retriever: {}",
            assay_canonical.display()
        ));
    }

    let retriever_canonical = match rbtdrp_canonical_rbra(&root, "retriever") {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical retriever RBRA path: {}", e)),
    };
    if let Some(parent) = retriever_canonical.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return rbtdre_Verdict::Fail(format!(
                "create retriever RBRA dir {}: {}",
                parent.display(),
                e
            ));
        }
    }
    if let Err(e) = std::fs::copy(&assay_canonical, &retriever_canonical) {
        return rbtdre_Verdict::Fail(format!(
            "copy assay RBRA → retriever canonical {}: {}",
            retriever_canonical.display(),
            e
        ));
    }

    let probe_ret = match rbtdri_invoke_imprint(
        ctx,
        RBTDRM_COLOPHON_ACCESS_PROBE,
        "retriever",
        &[],
    ) {
        Ok(r) => r,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!("access-probe retriever invocation: {}", e))
        }
    };
    let _ = std::fs::write(dir.join("probe-retriever-stdout.txt"), &probe_ret.stdout);
    let _ = std::fs::write(dir.join("probe-retriever-stderr.txt"), &probe_ret.stderr);
    if probe_ret.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "access-probe retriever exit {}\n{}",
            probe_ret.exit_code, probe_ret.stderr
        ));
    }

    // Director: invest → assay → canonical → access-probe.
    let invest_dir = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_GOV_INVEST_DIRECTOR,
        &[RBTDRP_IDENTITY_DIRECTOR],
        &[],
        dir,
        "invest-director",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("invest director: {}", e)),
    };
    if invest_dir.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "invest director exit {}\n{}",
            invest_dir.exit_code, invest_dir.stderr
        ));
    }

    if !assay_canonical.exists() {
        return rbtdre_Verdict::Fail(format!(
            "assay RBRA absent after invest-director: {}",
            assay_canonical.display()
        ));
    }
    let director_canonical = match rbtdrp_canonical_rbra(&root, "director") {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("canonical director RBRA path: {}", e)),
    };
    if let Some(parent) = director_canonical.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return rbtdre_Verdict::Fail(format!(
                "create director RBRA dir {}: {}",
                parent.display(),
                e
            ));
        }
    }
    if let Err(e) = std::fs::copy(&assay_canonical, &director_canonical) {
        return rbtdre_Verdict::Fail(format!(
            "copy assay RBRA → director canonical {}: {}",
            director_canonical.display(),
            e
        ));
    }

    let probe_dir = match rbtdri_invoke_imprint(
        ctx,
        RBTDRM_COLOPHON_ACCESS_PROBE,
        "director",
        &[],
    ) {
        Ok(r) => r,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!("access-probe director invocation: {}", e))
        }
    };
    let _ = std::fs::write(dir.join("probe-director-stdout.txt"), &probe_dir.stdout);
    let _ = std::fs::write(dir.join("probe-director-stderr.txt"), &probe_dir.stderr);
    if probe_dir.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "access-probe director exit {}\n{}",
            probe_dir.exit_code, probe_dir.stderr
        ));
    }

    // Divests in reverse order.
    let divest_dir = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_GOV_DIVEST_DIRECTOR,
        &[RBTDRP_IDENTITY_DIRECTOR],
        &[],
        dir,
        "divest-director",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("divest director: {}", e)),
    };
    if divest_dir.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "divest director exit {}\n{}",
            divest_dir.exit_code, divest_dir.stderr
        ));
    }
    if director_canonical.exists() {
        return rbtdre_Verdict::Fail(format!(
            "director canonical RBRA still present after divest: {} \
             (BBAAN divest-deletes-production-RBRA contract violated)",
            director_canonical.display()
        ));
    }

    let divest_ret = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_GOV_DIVEST_RETRIEVER,
        &[RBTDRP_IDENTITY_RETRIEVER],
        &[],
        dir,
        "divest-retriever",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("divest retriever: {}", e)),
    };
    if divest_ret.exit_code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "divest retriever exit {}\n{}",
            divest_ret.exit_code, divest_ret.stderr
        ));
    }
    if retriever_canonical.exists() {
        return rbtdre_Verdict::Fail(format!(
            "retriever canonical RBRA still present after divest: {} \
             (BBAAN divest-deletes-production-RBRA contract violated)",
            retriever_canonical.display()
        ));
    }

    if assay_canonical.exists() {
        let _ = std::fs::remove_file(&assay_canonical);
    }

    rbtdre_Verdict::Pass
}

// ── Case 4 (new): live-disqualify refusal ────────────────────

/// Recovery-diagnostic substring emitted by `rbgp_depot_unmake`'s
/// live-disqualify branch (rbgp_Payor.sh:944-948). The branch names
/// `RBRR_DEPOT_MONIKER` rename or `rbw-MZ` as recovery paths; the
/// assertion matches on the field-name token, which is invariant
/// across cosmetic message edits.
const RBTDRP_LIVE_DISQUALIFY_RECOVERY: &str = "RBRR_DEPOT_MONIKER";

/// Case 4 (new) — live-disqualify refusal. Pre-condition: depot levied
/// by case 2 (probe asserts both RBRR_CLOUD_PREFIX and RBRR_DEPOT_MONIKER
/// are non-blank). Composes the live RBDC_DEPOT_PROJECT_ID and invokes
/// `rbw-dU` with it as $1; expects non-zero exit + recovery diagnostic
/// naming `RBRR_DEPOT_MONIKER` (BBAA9 contract). The refusal lands
/// before authenticate (rbgp_Payor.sh:944-948), so no GCP traffic
/// occurs — assertion is on exit-code + diagnostic shape only.
fn rbtdrp_depot_live_disqualify(dir: &Path) -> rbtdre_Verdict {
    let probe = rbtdrb_Probe {
        name: "depot levied (RBRR_CLOUD_PREFIX + RBRR_DEPOT_MONIKER set)",
        check: rbtdrp_probe_depot_levied,
        remediation: "rerun case 2 (rbtdrp_depot_stand_up) before this case",
    };
    if let Err(v) = rbtdrb_assert(&probe) {
        return v;
    }
    rbtdrc_with_ctx(|ctx| rbtdrp_depot_live_disqualify_impl(ctx, dir))
}

fn rbtdrp_depot_live_disqualify_impl(
    ctx: &mut rbtdri_Context,
    dir: &Path,
) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let rbrr = root.join(RBTDRP_RBRR_FILE);
    let moniker = match rbtdrp_read_env_value(&rbrr, RBTDRP_FIELD_RBRR_DEPOT_MONIKER) {
        Some(m) if !m.is_empty() => m,
        _ => {
            return rbtdre_Verdict::Fail(
                "RBRR_DEPOT_MONIKER blank — probe should have caught this".to_string(),
            )
        }
    };

    let project_id = match rbtdrp_compose_project_id(&root, &moniker) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("compose project_id: {}", e)),
    };
    let _ = std::fs::write(dir.join("live-project-id.txt"), &project_id);

    let result = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_UNMAKE,
        &[&project_id],
        &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
        dir,
        "live-disqualify",
    ) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("rbw-dU invocation: {}", e)),
    };

    if result.exit_code == 0 {
        return rbtdre_Verdict::Fail(format!(
            "rbw-dU '{}' exited 0 — BBAA9 live-disqualify contract violated \
             (refusal must die when target == RBDC_DEPOT_PROJECT_ID)",
            project_id
        ));
    }

    // BUW dispatch merges stderr→stdout; assertion checks combined output.
    let combined = format!("{}{}", result.stdout, result.stderr);
    if !combined.contains(RBTDRP_LIVE_DISQUALIFY_RECOVERY) {
        return rbtdre_Verdict::Fail(format!(
            "rbw-dU live-disqualify diagnostic did not name '{}' as recovery path\n\
             stdout:\n{}\n\nstderr:\n{}",
            RBTDRP_LIVE_DISQUALIFY_RECOVERY, result.stdout, result.stderr
        ));
    }

    rbtdre_Verdict::Pass
}

// ── Case 5: depot tear-down ──────────────────────────────────

/// Case 5 — depot tear-down. Pre-condition: depot exists from case 2. Reads
/// moniker from rbrr.env, unmakes the depot (BURE_CONFIRM=skip), re-lists,
/// and verifies the depot is absent or in DELETE_REQUESTED state via
/// fact-file content read (no stdout-grep).
//
// CLUE: BBAA9 changed `rbgp_depot_unmake` to require a depot project ID
// folio (channel=param1; folio arrives via BUZ_FOLIO, NOT $1 — buz_exec_lookup
// extracts $1 into BUZ_FOLIO and removes it from $@). The current call below
// passes &[] (empty args), which now hits the empty-arg refusal branch
// (rbgp_Payor.sh:941-946). The natural fix — passing the composed project_id
// as $1 — additionally trips the live-disqualify guard (rbgp_Payor.sh:948-
// 952) since the throwaway depot IS the live RBRR-selected target at this
// point. Recovery per the live-disqualify diagnostic: rename
// RBRR_DEPOT_MONIKER (or run rbw-MZ) before invoking unmake. Operator
// designs the proper tear-down shape via the live-GCP exercise; until then
// this case fails at the empty-arg refusal under StateProgressing fail-fast.
fn rbtdrp_depot_tear_down(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrp_depot_tear_down_impl(ctx, dir))
}

fn rbtdrp_depot_tear_down_impl(ctx: &mut rbtdri_Context, dir: &Path) -> rbtdre_Verdict {
    let root = ctx.project_root().to_path_buf();

    let rbrr = root.join(RBTDRP_RBRR_FILE);
    let moniker = match rbtdrp_read_env_value(&rbrr, RBTDRP_FIELD_RBRR_DEPOT_MONIKER) {
        Some(m) if !m.is_empty() => m,
        _ => {
            return rbtdre_Verdict::Fail(
                "case 2 (stand-up) did not run or rbrr.env is missing the moniker \
                 (RBRR_DEPOT_MONIKER is blank)"
                    .to_string(),
            )
        }
    };

    let project_id = match rbtdrp_compose_project_id(&root, &moniker) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(format!("compose project_id: {}", e)),
    };
    let _ = std::fs::write(dir.join("project-id.txt"), &project_id);

    let unmake = match rbtdrp_invoke_logged(
        ctx,
        RBTDRM_COLOPHON_DEPOT_UNMAKE,
        &[],
        &[(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)],
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

    let depot_fact_path = list_after
        .burv_output
        .join(RBTDRI_BURV_OUTPUT_SUBDIR)
        .join(format!("{}.{}", moniker, RBTDRP_FACT_EXT_DEPOT));

    if !depot_fact_path.exists() {
        return rbtdre_Verdict::Pass;
    }

    let depot_state = match std::fs::read_to_string(&depot_fact_path) {
        Ok(s) => s.trim().to_string(),
        Err(e) => {
            return rbtdre_Verdict::Fail(format!(
                "read depot fact '{}': {}",
                depot_fact_path.display(),
                e
            ))
        }
    };

    if depot_state == RBTDRP_DELETE_REQUESTED {
        return rbtdre_Verdict::Pass;
    }

    rbtdre_Verdict::Fail(format!(
        "depot '{}' (project '{}') still present with unexpected state '{}' after unmake \
         (expected absent or '{}')",
        moniker, project_id, depot_state, RBTDRP_DELETE_REQUESTED
    ))
}

// ── Section registry ─────────────────────────────────────────

pub static RBTDRP_SECTIONS_PRISTINE_LIFECYCLE: &[rbtdre_Section] = &[
    rbtdre_Section {
        name: "pristine-lifecycle-gate",
        cases: &[case!(rbtdrp_marshal_zero_attestation)],
    },
    rbtdre_Section {
        name: "pristine-lifecycle-arc",
        cases: &[
            case!(rbtdrp_depot_stand_up),
            case!(rbtdrp_sa_cycle),
            case!(rbtdrp_depot_live_disqualify),
            case!(rbtdrp_depot_tear_down),
        ],
    },
];

pub static RBTDRP_FIXTURE_PRISTINE_LIFECYCLE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_PRISTINE_LIFECYCLE,
    disposition: rbtdre_Disposition::StateProgressing,
    setup: None,
    teardown: None,
    sections: RBTDRP_SECTIONS_PRISTINE_LIFECYCLE,
};
