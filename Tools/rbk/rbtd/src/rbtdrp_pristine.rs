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
use crate::rbtdre_engine::{rbtdre_Section, rbtdre_Verdict};

/// Site-specific RBRR fields that rblm_zero blanks. These five fields define
/// the depot-bound site identity; an empty value is the post-marshal-zero
/// invariant.
const RBTDRP_RBRR_BLANK_FIELDS: &[&str] = &[
    "RBRR_DEPOT_PROJECT_ID",
    "RBRR_GAR_REPOSITORY",
    "RBRR_GCB_POOL_STEM",
    "RBRR_CLOUD_PREFIX",
    "RBRR_RUNTIME_PREFIX",
];

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

// ── Section registry ─────────────────────────────────────────

pub static RBTDRP_SECTIONS_PRISTINE_LIFECYCLE: &[rbtdre_Section] = &[rbtdre_Section {
    name: "pristine-lifecycle-gate",
    cases: &[case!(rbtdrp_marshal_zero_attestation)],
}];
