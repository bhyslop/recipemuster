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
use std::sync::OnceLock;

use crate::rbtdre_engine::rbtdre_Verdict;
use crate::rbtdgc_consts::RBTDGC_CRUCIBLE_BARK;
use crate::rbtdrx_platform::{rbtdrx_is_cygwin, rbtdrx_native_to_posix, rbtdrx_posix_to_native};

/// Ifrit binary name inside the bottle container.
const RBTDRI_IFRIT_BINARY: &str = "rbid";

/// BUK dispatch output subdirectory — tabtargets write facts to BURV_OUTPUT_ROOT_DIR/current.
/// Matches BURD_OUTPUT_DIR = "${BURC_OUTPUT_ROOT_DIR}/current" from bud_dispatch.sh.
pub const RBTDRI_BURV_OUTPUT_SUBDIR: &str = "current";

/// Env var name read by `buc_require` (buc_command.sh:335) to bypass interactive
/// confirmation prompts in non-interactive contexts (test fixtures, automation).
pub const RBTDRI_BURE_CONFIRM_KEY: &str = "BURE_CONFIRM";

/// BURE tweak-slot env var (BUS0 Tweak Mechanism) — the single test-seam
/// channel every tabtarget inherits. The credless guard rides this slot for
/// fast-tier fixtures; case-supplied tweaks ride it everywhere else.
pub const RBTDRI_BURE_TWEAK_NAME_KEY: &str = "BURE_TWEAK_NAME";

/// BURE tweak-value env var — the payload paired with `BURE_TWEAK_NAME`. The
/// regime-poison tweak reads it as `VAR=value` (set) or bare `VAR` (unset).
pub const RBTDRI_BURE_TWEAK_VALUE_KEY: &str = "BURE_TWEAK_VALUE";

/// Value paired with `RBTDRI_BURE_CONFIRM_KEY` to skip the confirmation prompt.
pub const RBTDRI_BURE_CONFIRM_SKIP: &str = "skip";

/// BUK dispatch env var carrying the temp root for theurge — anchors BURV
/// per-invoke temp dirs under temp-buk/ rather than /tmp/. Required: theurge
/// fails at startup if unset. Set by bud_dispatch.sh on every tabtarget call.
pub const RBTDRI_BURD_TEMP_DIR_KEY: &str = "BURD_TEMP_DIR";

/// Canonical Cygwin bash in POSIX form. theurge nativizes this via RBTDRX
/// (cygpath) to launch scripts: a bare "bash" from a Windows-native binary
/// resolves through `CreateProcess` to System32's WSL launcher, never Cygwin's.
const RBTDRI_CYGWIN_BASH_POSIX: &str = "/bin/bash";

/// BURV invoke-directory name from a zero-based invoke count.
///
/// Single source of truth for the `invoke-NNNNN` naming pattern; tests call
/// this rather than hand-expanding the literal.
pub fn rbtdri_invoke_dir_name(invoke_num: u32) -> String {
    format!("invoke-{:05}", invoke_num)
}

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
///
/// `fixture` carries the fixture name. For crucible fixtures (tadmor/srjcl/
/// pluml) the fixture name happens to equal a valid nameplate moniker — that's
/// the convention crucible-scoped tabtargets exploit when imprinting (e.g.,
/// `rbw-cC.Charge.{fixture}.sh`). Non-crucible fixtures (regime-*, calibrant-*,
/// canonical-*, etc.) carry their fixture name in this slot too, but no
/// nameplate-shaped consumer reads them.
pub struct rbtdri_Context {
    pub(crate) project_root: PathBuf,
    pub(crate) fixture: String,
    pub(crate) burv_temp_root: PathBuf,
    pub(crate) burv_output_root: PathBuf,
    pub(crate) invoke_count: u32,
    /// One-shot flag: when set, the NEXT invoke reuses the immediately-prior
    /// invoke's BURV root instead of minting a fresh one (see
    /// `chain_next_invoke`). Consumed and cleared by `rbtdri_invoke_impl`.
    pub(crate) chain_next: bool,
}

impl rbtdri_Context {
    pub fn new(
        project_root: &Path,
        fixture: &str,
        burv_temp_root: &Path,
        burv_output_root: &Path,
    ) -> Self {
        Self {
            project_root: project_root.to_path_buf(),
            fixture: fixture.to_string(),
            burv_temp_root: burv_temp_root.to_path_buf(),
            burv_output_root: burv_output_root.to_path_buf(),
            invoke_count: 0,
            chain_next: false,
        }
    }

    pub fn fixture(&self) -> &str {
        &self.fixture
    }

    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    /// Mark the NEXT tabtarget invocation to chain off the immediately-prior
    /// invoke's BURV root, rather than running in fresh isolation.
    ///
    /// Theurge gives every invoke its own `BURV_OUTPUT_ROOT_DIR`, so
    /// `bud_dispatch`'s start-of-dispatch `current/`->`previous/` promotion never
    /// crosses invokes — each invoke's `previous/` is empty. That suits isolated
    /// operations but breaks the depth-1 cross-tabtarget chain a real operator
    /// gets for free by sharing one `../output-buk` root: the chaining fact one
    /// tabtarget writes to `current/` never reaches the next tabtarget's
    /// `previous/`. The bole derived-pull base-anchor election is the consumer —
    /// `ensconce` writes the touchmark to `current/`, the following `ordain`
    /// reads it from `previous/`.
    ///
    /// Calling this before such a pair makes the next invoke reuse the prior
    /// invoke's root, so the prior invoke's `current/` is promoted into this
    /// invoke's `previous/` — replicating the operator flow for exactly the
    /// invokes that need it, leaving every other invoke's isolation intact.
    /// One-shot: consumed by the next invoke and cleared. Depth-1 only — bud
    /// keeps a single generation, so only the immediate predecessor is visible.
    pub fn chain_next_invoke(&mut self) {
        self.chain_next = true;
    }

    /// Read the suite-monotonic BURV invoke counter. The suite loop reads it
    /// after each fixture and seeds the next Context, so per-invoke dir names
    /// stay unique across fixtures (see set_invoke_count).
    pub fn invoke_count(&self) -> u32 {
        self.invoke_count
    }

    /// Seed the BURV invoke counter so this fixture's invokes continue the
    /// suite-monotonic sequence rather than restarting at 0. Crate-internal code
    /// mutates the field directly; the bin crate must go through this setter
    /// because the field is pub(crate) and so not visible across the lib/bin
    /// boundary.
    pub fn set_invoke_count(&mut self, count: u32) {
        self.invoke_count = count;
    }
}

// ── Credless guard ───────────────────────────────────────────

thread_local! {
    /// Fast-tier credless guard arm state. Thread-local (not process-global)
    /// to match the rbtdrc context channel: cases, hooks, and direct-Command
    /// helpers all run on the thread that installed the context, and unit
    /// tests on parallel threads cannot interfere with each other.
    static RBTDRI_CREDLESS_ARMED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Arm or disarm the credless guard for the current thread. Armed by
/// `rbtdrc_set_context` from the fixture's `credless` field and disarmed by
/// `rbtdrc_take_context`, so the guard rides every invocation of a fast-tier
/// fixture's cases regardless of which suite hosts the fixture.
pub fn rbtdri_arm_credless(armed: bool) {
    RBTDRI_CREDLESS_ARMED.with(|c| c.set(armed));
}

/// Read the current thread's credless guard arm state.
pub fn rbtdri_credless_armed() -> bool {
    RBTDRI_CREDLESS_ARMED.with(|c| c.get())
}

// ── Tabtarget discovery ──────────────────────────────────────

/// Find the tabtarget script for a colophon + imprint (nameplate or role).
///
/// Scans tt/ for files matching `{colophon}.*.{imprint}.sh`.
/// Returns error if zero or multiple matches — exactly one must exist.
pub fn rbtdri_find_tabtarget(
    project_root: &Path,
    colophon: &str,
    imprint: &str,
) -> Result<PathBuf, String> {
    let tt_dir = project_root.join("tt");
    let prefix = format!("{}.", colophon);
    let suffix = format!(".{}.sh", imprint);

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
            "rbtdri: no tabtarget for colophon '{}' imprint '{}'",
            colophon, imprint
        )),
        1 => Ok(matches.into_iter().next().unwrap()),
        n => Err(format!(
            "rbtdri: {} tabtargets match colophon '{}' imprint '{}' — expected exactly one",
            n, colophon, imprint
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

static RBTDRI_BASH_PROGRAM: OnceLock<String> = OnceLock::new();

/// The bash program theurge launches scripts with, resolved once per process.
///
/// On Cygwin a bare `"bash"` from a Windows-native binary resolves through
/// `CreateProcess` to System32's WSL launcher, never Cygwin's bash — so we
/// nativize the canonical Cygwin bash (`/bin/bash`) via RBTDRX's cygpath. That
/// keeps cygpath inside theurge's existing Rust dependency rather than kit bash,
/// and cygpath itself resolves correctly from a native binary (no System32 twin,
/// unlike bash). Off Cygwin — and as a fallback if cygpath fails — it is "bash".
pub fn rbtdri_bash_program() -> &'static str {
    RBTDRI_BASH_PROGRAM
        .get_or_init(|| {
            if rbtdrx_is_cygwin() {
                match rbtdrx_posix_to_native(RBTDRI_CYGWIN_BASH_POSIX) {
                    Ok(p) => p.to_string_lossy().into_owned(),
                    Err(_) => "bash".to_string(),
                }
            } else {
                "bash".to_string()
            }
        })
        .as_str()
}

/// Build a `Command` that launches a tabtarget — a bash `.sh` — portably.
///
/// A Windows-native theurge (`x86_64-pc-windows-gnu`) cannot `CreateProcess` a
/// `.sh` directly: Rust's `Command` on Windows launches only `.exe`. So we run
/// the script through bash — via `rbtdri_bash_program()` so the right bash is
/// chosen on Cygwin — with the script path rendered to POSIX form (RBTDRX). On
/// Linux/macOS that conversion is identity and the bash program is just "bash",
/// so the call site is unconditional. Callers chain `.args(...)`,
/// `.current_dir(...)`, and `.env(...)` as on any `Command::new` result.
///
/// The credless guard lands here — the one constructor every tabtarget launch
/// goes through, including the direct-Command case helpers that bypass
/// `rbtdri_invoke*` — so a fast-tier fixture cannot spawn an unguarded
/// tabtarget by construction.
pub fn rbtdri_tabtarget_command(tabtarget: &Path) -> Command {
    let mut cmd = Command::new(rbtdri_bash_program());
    cmd.arg(rbtdrx_native_to_posix(tabtarget));
    if rbtdri_credless_armed() {
        cmd.env(
            RBTDRI_BURE_TWEAK_NAME_KEY,
            crate::rbtdgc_consts::RBTDGC_TWEAK_CREDLESS_GUARD,
        );
    }
    cmd
}

/// Internal: execute a resolved tabtarget with BURV isolation and optional extra env vars.
///
/// Honors the context's one-shot `chain_next` flag (see
/// `rbtdri_Context::chain_next_invoke`): when set, this invoke reuses the
/// immediately-prior invoke's BURV root instead of minting a fresh one — so
/// `bud_dispatch` promotes that invoke's `current/` into this invoke's
/// `previous/`. The flag is consumed and cleared here.
fn rbtdri_invoke_impl(
    ctx: &mut rbtdri_Context,
    tabtarget: &Path,
    args: &[&str],
    extra_env: &[(&str, &str)],
) -> Result<rbtdri_InvokeResult, String> {
    let invoke_num = if std::mem::take(&mut ctx.chain_next) {
        // Chain off the immediately-prior invoke: reuse its root (do NOT mint a
        // fresh one or bump the counter), so bud's promotion carries that
        // invoke's current/ into this one's previous/. Depth-1 by construction.
        ctx.invoke_count.checked_sub(1).ok_or_else(|| {
            "rbtdri: chain_next_invoke set with no prior invoke to chain from".to_string()
        })?
    } else {
        let n = ctx.invoke_count;
        ctx.invoke_count += 1;
        n
    };

    let dir_name = rbtdri_invoke_dir_name(invoke_num);
    let burv_output = ctx.burv_output_root.join(&dir_name);
    let burv_temp = ctx.burv_temp_root.join(&dir_name);

    std::fs::create_dir_all(&burv_output)
        .map_err(|e| format!("rbtdri: failed to create BURV output dir: {}", e))?;
    std::fs::create_dir_all(&burv_temp)
        .map_err(|e| format!("rbtdri: failed to create BURV temp dir: {}", e))?;

    // Tweak-slot conflict gate (BUS0): under the credless guard the single
    // tweak slot belongs to the guard — a fast-tier case supplying its own
    // tweak has self-identified as not belonging in fast. Fail loud rather
    // than letting the case silently overwrite the guard.
    if rbtdri_credless_armed()
        && extra_env.iter().any(|(k, _)| *k == RBTDRI_BURE_TWEAK_NAME_KEY)
    {
        return Err(format!(
            "rbtdri: fixture '{}' is fast-tier credless — its tweak slot belongs to \
             the credless guard, so a case may not set {} (a case needing a seam \
             does not belong in fast)",
            ctx.fixture, RBTDRI_BURE_TWEAK_NAME_KEY
        ));
    }

    let mut cmd = rbtdri_tabtarget_command(tabtarget);
    cmd.args(args)
        .current_dir(&ctx.project_root)
        .env("BURV_OUTPUT_ROOT_DIR", rbtdrx_native_to_posix(&burv_output))
        .env("BURV_TEMP_ROOT_DIR", rbtdrx_native_to_posix(&burv_temp));

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

/// Invoke a fixture-imprinted tabtarget (colophon + ctx.fixture). For crucible
/// fixtures the fixture name is also a nameplate moniker, which is the
/// imprint shape this resolves against.
pub fn rbtdri_invoke(
    ctx: &mut rbtdri_Context,
    colophon: &str,
    args: &[&str],
) -> Result<rbtdri_InvokeResult, String> {
    let tabtarget = rbtdri_find_tabtarget(&ctx.project_root, colophon, &ctx.fixture)?;
    rbtdri_invoke_impl(ctx, &tabtarget, args, &[])
}

/// Invoke a fixture-imprinted tabtarget (like `rbtdri_invoke`) with extra
/// environment variables threaded into the child process — e.g. BURD_NO_LOG
/// to keep BUK dispatch from folding the tabtarget's stderr into stdout.
pub fn rbtdri_invoke_env(
    ctx: &mut rbtdri_Context,
    colophon: &str,
    args: &[&str],
    extra_env: &[(&str, &str)],
) -> Result<rbtdri_InvokeResult, String> {
    let tabtarget = rbtdri_find_tabtarget(&ctx.project_root, colophon, &ctx.fixture)?;
    rbtdri_invoke_impl(ctx, &tabtarget, args, extra_env)
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

/// Invoke a tabtarget with an explicit imprint (overrides ctx.fixture for discovery).
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

/// Enumerate multi-fact files in a tabtarget's BURV output directory.
/// Multi-facts follow the convention `<root>.<ext>` written by buf_write_fact_multi;
/// returns the sorted list of roots whose files have the requested extension.
/// Returns an empty Vec if no matching files exist.
pub fn rbtdri_read_burv_facts_multi(
    result: &rbtdri_InvokeResult,
    extension: &str,
) -> Result<Vec<String>, String> {
    let dir = result.burv_output.join(RBTDRI_BURV_OUTPUT_SUBDIR);
    let entries = std::fs::read_dir(&dir).map_err(|e| {
        format!(
            "rbtdri: cannot enumerate fact dir {}: {}",
            dir.display(),
            e
        )
    })?;
    let suffix = format!(".{}", extension);
    let mut roots: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter_map(|name| name.strip_suffix(&suffix).map(str::to_string))
        .collect();
    roots.sort();
    Ok(roots)
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
    let result = rbtdri_invoke(ctx, RBTDGC_CRUCIBLE_BARK, &[RBTDRI_IFRIT_BINARY, attack_selector])?;
    Ok(rbtdri_parse_ifrit_verdict(&result.stdout, result.exit_code))
}
