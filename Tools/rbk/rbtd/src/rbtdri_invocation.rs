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
// RBTDRI — tabtarget invocation layer for theurge
//
// Theurge invokes bottle operations exclusively through tabtargets, never
// reimplementing bash command logic. This module provides:
//
//   1. Tabtarget discovery — imprint-scoped, global, or nameplate-scoped
//   2. Tabtarget execution with BURV isolation — per-invocation output/temp dirs
//   3. Ifrit verdict parsing — extract verdict from ifrit stdout + exit code
//   4. BURV fact file reading — extract structured output from tabtarget results

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::rbtdre_engine::rbtdre_Verdict;
use crate::rbtdrm_manifest::RBTDRM_COLOPHON_BARK;

/// Ifrit binary name inside the bottle container.
const RBTDRI_IFRIT_BINARY: &str = "rbid";

/// BUK dispatch output subdirectory — tabtargets write facts to BURV_OUTPUT_ROOT_DIR/current.
/// Matches BURD_OUTPUT_DIR = "${BURC_OUTPUT_ROOT_DIR}/current" from bud_dispatch.sh.
pub const RBTDRI_BURV_OUTPUT_SUBDIR: &str = "current";

// ── Invocation result ────────────────────────────────────────

/// Captured output from a tabtarget invocation.
#[derive(Debug)]
pub struct rbtdri_InvokeResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub burv_output: PathBuf,
}

// ── Invocation context ───────────────────────────────────────

/// Per-case invocation context. Tracks BURV isolation state so each tabtarget
/// invocation within a case gets its own output and temp directories, matching
/// the zbuto_invoke() pattern from buto_operations.sh.
pub struct rbtdri_Context {
    pub(crate) project_root: PathBuf,
    pub(crate) nameplate: String,
    pub(crate) burv_root: PathBuf,
    pub(crate) invoke_count: u32,
}

impl rbtdri_Context {
    pub fn new(project_root: &Path, nameplate: &str, burv_root: &Path) -> Self {
        Self {
            project_root: project_root.to_path_buf(),
            nameplate: nameplate.to_string(),
            burv_root: burv_root.to_path_buf(),
            invoke_count: 0,
        }
    }

    pub fn nameplate(&self) -> &str {
        &self.nameplate
    }

    pub fn project_root(&self) -> &Path {
        &self.project_root
    }
}

// ── Tabtarget discovery ──────────────────────────────────────

/// Find the tabtarget script for a colophon + imprint (nameplate or role).
///
/// Scans tt/ for files matching `{colophon}.*.{imprint}.sh`.
/// Returns error if zero or multiple matches — exactly one must exist.
pub fn rbtdri_find_tabtarget(
    project_root: &Path,
    colophon: &str,
    nameplate: &str,
) -> Result<PathBuf, String> {
    let tt_dir = project_root.join("tt");
    let prefix = format!("{}.", colophon);
    let suffix = format!(".{}.sh", nameplate);

    let entries = std::fs::read_dir(&tt_dir)
        .map_err(|e| format!("rbtdri: cannot read tt/ directory: {}", e))?;

    let matches: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                name.starts_with(&prefix) && name.ends_with(&suffix)
            } else {
                false
            }
        })
        .collect();

    match matches.len() {
        0 => Err(format!(
            "rbtdri: no tabtarget for colophon '{}' nameplate '{}'",
            colophon, nameplate
        )),
        1 => Ok(matches.into_iter().next().unwrap()),
        n => Err(format!(
            "rbtdri: {} tabtargets match colophon '{}' nameplate '{}' — expected exactly one",
            n, colophon, nameplate
        )),
    }
}

/// Find a global tabtarget (no imprint suffix).
///
/// Scans tt/ for files matching `{colophon}.{frontispiece}.sh` (exactly two dots).
/// Rejects files with imprint suffixes (three+ dots).
pub fn rbtdri_find_tabtarget_global(
    project_root: &Path,
    colophon: &str,
) -> Result<PathBuf, String> {
    let tt_dir = project_root.join("tt");
    let prefix = format!("{}.", colophon);

    let entries = std::fs::read_dir(&tt_dir)
        .map_err(|e| format!("rbtdri: cannot read tt/ directory: {}", e))?;

    let matches: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(&prefix) && name.ends_with(".sh") {
                    // Global: no imprint — exactly one part between colophon and .sh
                    let middle = &name[prefix.len()..name.len() - 3]; // strip ".sh"
                    !middle.contains('.')
                } else {
                    false
                }
            } else {
                false
            }
        })
        .collect();

    match matches.len() {
        0 => Err(format!(
            "rbtdri: no global tabtarget for colophon '{}'",
            colophon
        )),
        1 => Ok(matches.into_iter().next().unwrap()),
        n => Err(format!(
            "rbtdri: {} global tabtargets match colophon '{}' — expected exactly one",
            n, colophon
        )),
    }
}

// ── Tabtarget invocation with BURV isolation ─────────────────

/// Internal: execute a resolved tabtarget with BURV isolation and optional extra env vars.
fn rbtdri_invoke_impl(
    ctx: &mut rbtdri_Context,
    tabtarget: &Path,
    args: &[&str],
    extra_env: &[(&str, &str)],
) -> Result<rbtdri_InvokeResult, String> {
    let invoke_num = ctx.invoke_count;
    ctx.invoke_count += 1;

    let invoke_dir = ctx.burv_root.join(format!("invoke-{:05}", invoke_num));
    let burv_output = invoke_dir.join("output");
    let burv_temp = invoke_dir.join("temp");

    std::fs::create_dir_all(&burv_output)
        .map_err(|e| format!("rbtdri: failed to create BURV output dir: {}", e))?;
    std::fs::create_dir_all(&burv_temp)
        .map_err(|e| format!("rbtdri: failed to create BURV temp dir: {}", e))?;

    let mut cmd = Command::new(tabtarget);
    cmd.args(args)
        .current_dir(&ctx.project_root)
        .env("BURV_OUTPUT_ROOT_DIR", &burv_output)
        .env("BURV_TEMP_ROOT_DIR", &burv_temp);

    for (key, value) in extra_env {
        cmd.env(key, value);
    }

    let output = cmd
        .output()
        .map_err(|e| format!("rbtdri: failed to execute '{}': {}", tabtarget.display(), e))?;

    Ok(rbtdri_InvokeResult {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        exit_code: output.status.code().unwrap_or(-1),
        burv_output,
    })
}

/// Invoke a nameplate-scoped tabtarget (colophon + ctx.nameplate).
pub fn rbtdri_invoke(
    ctx: &mut rbtdri_Context,
    colophon: &str,
    args: &[&str],
) -> Result<rbtdri_InvokeResult, String> {
    let tabtarget = rbtdri_find_tabtarget(&ctx.project_root, colophon, &ctx.nameplate)?;
    rbtdri_invoke_impl(ctx, &tabtarget, args, &[])
}

/// Invoke a global tabtarget (no imprint) with optional extra environment variables.
pub fn rbtdri_invoke_global(
    ctx: &mut rbtdri_Context,
    colophon: &str,
    args: &[&str],
    extra_env: &[(&str, &str)],
) -> Result<rbtdri_InvokeResult, String> {
    let tabtarget = rbtdri_find_tabtarget_global(&ctx.project_root, colophon)?;
    rbtdri_invoke_impl(ctx, &tabtarget, args, extra_env)
}

/// Invoke a tabtarget with an explicit imprint (overrides ctx.nameplate for discovery).
pub fn rbtdri_invoke_imprint(
    ctx: &mut rbtdri_Context,
    colophon: &str,
    imprint: &str,
    args: &[&str],
) -> Result<rbtdri_InvokeResult, String> {
    let tabtarget = rbtdri_find_tabtarget(&ctx.project_root, colophon, imprint)?;
    rbtdri_invoke_impl(ctx, &tabtarget, args, &[])
}

// ── BURV fact file reading ───────────────────────────────────

/// Read a fact file from a tabtarget's BURV output directory.
/// Fact files are single-line values written by tabtargets to BURD_OUTPUT_DIR,
/// which is BURV_OUTPUT_ROOT_DIR/current per BUK dispatch convention.
pub fn rbtdri_read_burv_fact(
    result: &rbtdri_InvokeResult,
    fact_name: &str,
) -> Result<String, String> {
    let path = result.burv_output.join(RBTDRI_BURV_OUTPUT_SUBDIR).join(fact_name);
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("rbtdri: cannot read fact '{}' from {}: {}", fact_name, path.display(), e))?;
    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        return Err(format!("rbtdri: fact '{}' is empty in {}", fact_name, path.display()));
    }
    Ok(trimmed)
}

// ── Ifrit verdict parsing ────────────────────────────────────

/// Ifrit verdict wire protocol: ifrit prints exactly one line matching
/// `IFRIT_VERDICT: PASS` or `IFRIT_VERDICT: FAIL <detail>` to stdout.
/// Missing verdict line is always a failure — no silent pass-through.
pub fn rbtdri_parse_ifrit_verdict(stdout: &str, exit_code: i32) -> rbtdre_Verdict {
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("IFRIT_VERDICT:") {
            let rest = rest.trim();
            if rest.starts_with("PASS") {
                return rbtdre_Verdict::Pass;
            }
            if let Some(detail) = rest.strip_prefix("FAIL") {
                let detail = detail.trim();
                if detail.is_empty() {
                    return rbtdre_Verdict::Fail("ifrit reported failure".to_string());
                }
                return rbtdre_Verdict::Fail(detail.to_string());
            }
        }
    }

    if exit_code == 0 {
        rbtdre_Verdict::Fail("ifrit exited 0 but no verdict line found".to_string())
    } else {
        rbtdre_Verdict::Fail(format!("ifrit exited {} with no verdict line", exit_code))
    }
}

/// Invoke the ifrit binary inside a charged bottle via the bark tabtarget.
/// The attack selector argument tells ifrit which attack module to run.
/// Returns a test verdict based on ifrit's stdout verdict line and exit code.
pub fn rbtdri_invoke_ifrit(
    ctx: &mut rbtdri_Context,
    attack_selector: &str,
) -> Result<rbtdre_Verdict, String> {
    let result = rbtdri_invoke(ctx, RBTDRM_COLOPHON_BARK, &[RBTDRI_IFRIT_BINARY, attack_selector])?;
    Ok(rbtdri_parse_ifrit_verdict(&result.stdout, result.exit_code))
}
