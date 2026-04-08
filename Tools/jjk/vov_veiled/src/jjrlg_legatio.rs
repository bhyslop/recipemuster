// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Legatio — Remote dispatch via SSH
//!
//! Implements remote execution at two identity tiers:
//!
//! Legatio (session):
//! - `jjx_bind`  — SSH probe, RELDIR validation, legatio minting
//! - `jjx_send`  — synchronous command execution via legatio
//! - `jjx_plant` — reset fundus workspace to exact commit
//! - `jjx_fetch` — read single file from fundus
//!
//! Pensum (async job):
//! - `jjx_relay` — nohup dispatch, pensum minting, BURX capture
//! - `jjx_check` — probe or poll pensum status via BURX
//!
//! SSH transport uses platform `ssh` binary via std::process::Command.
//! No Rust SSH crate — platform openssh inherits user's ~/.ssh/config,
//! known_hosts, and agent forwarding.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use vvc::{vvco_out, vvco_err, vvco_Output};

// ============================================================================
// Constants
// ============================================================================

/// Legatio state file prefix within officium directory
const LEGATIO_FILE_PREFIX: &str = "legatio_";

/// Legatio state file extension
const LEGATIO_FILE_EXT: &str = ".json";

/// Officia root directory (mirrors jjrm_mcp.rs constant)
const OFFICIA_DIR: &str = ".claude/jjm/officia";

/// Officium sun prefix character (mirrors jjrm_mcp.rs constant)
const OFFICIUM_SUN_PREFIX: char = '\u{2609}'; // ☉

// Command name constants — RCG String Boundary Discipline
const JJRLG_CMD_NAME_BIND: &str = "jjx_bind";
const JJRLG_CMD_NAME_SEND: &str = "jjx_send";
const JJRLG_CMD_NAME_PLANT: &str = "jjx_plant";
const JJRLG_CMD_NAME_FETCH: &str = "jjx_fetch";
const JJRLG_CMD_NAME_RELAY: &str = "jjx_relay";
const JJRLG_CMD_NAME_CHECK: &str = "jjx_check";

/// Pensum state file prefix within officium directory
const PENSUM_FILE_PREFIX: &str = "pensum_";

/// Pensum state file extension
const PENSUM_FILE_EXT: &str = ".json";

/// Max retries for initial burx.env read after relay launch
const BURX_INITIAL_POLL_MAX_RETRIES: u32 = 10;

/// Delay between initial burx.env read retries (seconds)
const BURX_INITIAL_POLL_DELAY_SECS: u64 = 1;

/// Poll interval for jjx_check with timeout>0 (seconds)
const BURX_CHECK_POLL_INTERVAL_SECS: u64 = 2;

/// Pensum seeds file within officium directory (keyed by firemark)
const PENSUM_SEEDS_FILE: &str = "pensum_seeds.json";

// ============================================================================
// Legatio state
// ============================================================================

/// Persisted state for a legatio (SSH session to a fundus).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrlg_LegatioState {
    pub host: String,
    pub user: String,
    pub reldir: String,
    pub output_root_dir: String,
}

/// Persisted state for a pensum (async remote dispatch job).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrlg_PensumState {
    pub legatio: String,
    pub firemark: String,
    pub temp_dir: String,
    pub pid: String,
    pub tabtarget: String,
    pub timeout: u64,
    pub began_at: String,
}

// ============================================================================
// RELDIR validation (Layer 1 — Rust constant checks)
// ============================================================================

/// Validate RELDIR against Layer 1 safety constraints.
///
/// - Must not be empty
/// - Must not start with `/` (must be relative to home)
/// - Must not start with `.` (no relative traversal)
/// - Must contain at least one `/` (forces subdirectory depth)
pub fn jjrlg_validate_reldir(reldir: &str) -> Result<(), String> {
    if reldir.is_empty() {
        return Err("RELDIR must not be empty".to_string());
    }
    if reldir.starts_with('/') {
        return Err("RELDIR must not start with '/' (must be relative to home)".to_string());
    }
    if reldir.starts_with('.') {
        return Err("RELDIR must not start with '.' (no relative traversal)".to_string());
    }
    if !reldir.contains('/') {
        return Err("RELDIR must contain at least one '/' (forces subdirectory depth)".to_string());
    }
    Ok(())
}

/// Validate a git ref (commit SHA, branch, tag) contains only safe characters.
fn zjjrlg_validate_git_ref(s: &str) -> Result<(), String> {
    if s.is_empty() {
        return Err("git ref must not be empty".to_string());
    }
    if s.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '-' | '_' | '.')) {
        Ok(())
    } else {
        Err(format!("git ref contains invalid characters: '{}'", s))
    }
}

// ============================================================================
// SSH execution
// ============================================================================

/// Result of an SSH command execution.
pub struct jjrlg_SshResult {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// Execute a command on a remote host via SSH.
///
/// Uses BatchMode=yes (no password prompts) and ConnectTimeout=10.
/// The command string is passed as a single argument to ssh, which
/// the remote sshd passes to the user's login shell for interpretation.
fn zjjrlg_ssh_exec(host: &str, user: &str, command: &str) -> Result<jjrlg_SshResult, String> {
    let target = format!("{}@{}", user, host);
    let output = std::process::Command::new("ssh")
        .arg("-o").arg("BatchMode=yes")
        .arg("-o").arg("ConnectTimeout=10")
        .arg(&target)
        .arg(command)
        .output()
        .map_err(|e| format!("SSH execution failed: {}", e))?;

    Ok(jjrlg_SshResult {
        exit_code: output.status.code().unwrap_or(-1),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

// ============================================================================
// Shell quoting
// ============================================================================

/// Single-quote a string for POSIX shell.
///
/// Wraps in single quotes, escaping embedded single quotes via the
/// standard `'\''` pattern (end quote, escaped quote, begin quote).
fn zjjrlg_shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

// ============================================================================
// Legatio file management
// ============================================================================

/// Resolve officium ID to its directory path.
fn zjjrlg_officium_dir(officium_id: &str) -> PathBuf {
    let bare_id = officium_id.trim_start_matches(OFFICIUM_SUN_PREFIX);
    PathBuf::from(OFFICIA_DIR).join(bare_id)
}

/// Resolve legatio token to its state file path within an officium.
fn zjjrlg_legatio_path(officium_dir: &Path, token: &str) -> PathBuf {
    officium_dir.join(format!("{}{}{}", LEGATIO_FILE_PREFIX, token, LEGATIO_FILE_EXT))
}

/// Mint a new legatio token by scanning existing files in the officium.
///
/// Token format: `L0`, `L1`, `L2`, ... (incrementing within officium).
fn zjjrlg_mint_token(officium_dir: &Path) -> String {
    let mut max_num: i32 = -1;
    if let Ok(entries) = std::fs::read_dir(officium_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(rest) = name.strip_prefix(LEGATIO_FILE_PREFIX) {
                if let Some(num_str) = rest.strip_suffix(LEGATIO_FILE_EXT) {
                    // Token is "L0", "L1", etc. — strip the L prefix for parsing.
                    if let Some(digits) = num_str.strip_prefix('L') {
                        if let Ok(num) = digits.parse::<i32>() {
                            if num > max_num { max_num = num; }
                        }
                    }
                }
            }
        }
    }
    format!("L{}", max_num + 1)
}

/// Load persisted legatio state from the officium directory.
fn zjjrlg_load_legatio(officium_dir: &Path, token: &str) -> Result<jjrlg_LegatioState, String> {
    let path = zjjrlg_legatio_path(officium_dir, token);
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read legatio '{}': {}", token, e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Cannot parse legatio '{}': {}", token, e))
}

/// Persist legatio state to the officium directory.
fn zjjrlg_save_legatio(officium_dir: &Path, token: &str, state: &jjrlg_LegatioState) -> Result<(), String> {
    let path = zjjrlg_legatio_path(officium_dir, token);
    let json = serde_json::to_string_pretty(state)
        .map_err(|e| format!("Cannot serialize legatio: {}", e))?;
    std::fs::write(&path, json.as_bytes())
        .map_err(|e| format!("Cannot write legatio '{}': {}", token, e))
}

/// Resolve pensum token to its state file path within an officium.
fn zjjrlg_pensum_path(officium_dir: &Path, token: &str) -> PathBuf {
    officium_dir.join(format!("{}{}{}", PENSUM_FILE_PREFIX, token, PENSUM_FILE_EXT))
}

/// Load persisted pensum state from the officium directory.
fn zjjrlg_load_pensum(officium_dir: &Path, token: &str) -> Result<jjrlg_PensumState, String> {
    let path = zjjrlg_pensum_path(officium_dir, token);
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read pensum '{}': {}", token, e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Cannot parse pensum '{}': {}", token, e))
}

/// Persist pensum state to the officium directory.
fn zjjrlg_save_pensum(officium_dir: &Path, token: &str, state: &jjrlg_PensumState) -> Result<(), String> {
    let path = zjjrlg_pensum_path(officium_dir, token);
    let json = serde_json::to_string_pretty(state)
        .map_err(|e| format!("Cannot serialize pensum: {}", e))?;
    std::fs::write(&path, json.as_bytes())
        .map_err(|e| format!("Cannot write pensum '{}': {}", token, e))
}

// ============================================================================
// jjx_bind — SSH probe + RELDIR validation + legatio minting
// ============================================================================

pub struct jjrlg_BindArgs {
    pub host: String,
    pub user: String,
    pub reldir: String,
}

/// Bind a legatio: validate RELDIR, SSH probe the fundus, cache regime
/// paths, mint and persist legatio token.
pub fn jjrlg_run_bind(args: jjrlg_BindArgs, officium_id: &str) -> (i32, String) {
    let cn = JJRLG_CMD_NAME_BIND;
    let mut output = vvco_Output::buffer();

    // Layer 1: Rust constant validation
    if let Err(e) = jjrlg_validate_reldir(&args.reldir) {
        vvco_err!(output, "{}: RELDIR validation failed (Layer 1): {}", cn, e);
        return (1, output.vvco_finish());
    }

    let officium_dir = zjjrlg_officium_dir(officium_id);
    if !officium_dir.is_dir() {
        vvco_err!(output, "{}: officium directory not found", cn);
        return (1, output.vvco_finish());
    }

    // Layer 3: SSH probe with resolved-path safety check
    //
    // Validates on the fundus that:
    // 1. RELDIR resolves to a real directory
    // 2. Resolved path is strictly under $HOME
    // 3. Resolved path has at least 2 components under $HOME
    let probe_script = format!(
        concat!(
            "z_resolved=\"$(cd \"$HOME/{reldir}\" 2>/dev/null && pwd)\" || exit 99\n",
            "case \"$z_resolved\" in\n",
            "  \"$HOME\"/*) ;;\n",
            "  *) echo \"FATAL: resolved dir not under HOME\"; exit 99 ;;\n",
            "esac\n",
            "z_depth=\"${{z_resolved#\"$HOME\"/}}\"\n",
            "case \"$z_depth\" in\n",
            "  */*) ;;\n",
            "  *) echo \"FATAL: resolved dir too shallow under HOME\"; exit 99 ;;\n",
            "esac\n",
            "echo \"PROBE_OK:$z_resolved\""
        ),
        reldir = args.reldir
    );

    vvco_out!(output, "Probing {}@{}:~/{} ...", args.user, args.host, args.reldir);

    let probe_result = match zjjrlg_ssh_exec(&args.host, &args.user, &probe_script) {
        Ok(r) => r,
        Err(e) => {
            vvco_err!(output, "{}: SSH probe failed: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    if probe_result.exit_code != 0 {
        let stderr = String::from_utf8_lossy(&probe_result.stderr);
        let stdout = String::from_utf8_lossy(&probe_result.stdout);
        vvco_err!(output, "{}: SSH probe failed (exit {})", cn, probe_result.exit_code);
        if !stdout.trim().is_empty() { vvco_err!(output, "  stdout: {}", stdout.trim()); }
        if !stderr.trim().is_empty() { vvco_err!(output, "  stderr: {}", stderr.trim()); }
        return (1, output.vvco_finish());
    }

    let stdout = String::from_utf8_lossy(&probe_result.stdout);
    if !stdout.contains("PROBE_OK:") {
        vvco_err!(output, "{}: probe did not return PROBE_OK sentinel", cn);
        return (1, output.vvco_finish());
    }

    vvco_out!(output, "RELDIR probe passed.");

    // Read BURC_OUTPUT_ROOT_DIR from fundus .buk/burc.env
    let burc_cmd = format!(
        "cd \"$HOME/{}\" && cat .buk/burc.env 2>/dev/null | grep '^BURC_OUTPUT_ROOT_DIR=' | head -1",
        args.reldir
    );

    let burc_result = match zjjrlg_ssh_exec(&args.host, &args.user, &burc_cmd) {
        Ok(r) => r,
        Err(e) => {
            vvco_err!(output, "{}: failed to read burc.env: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let burc_stdout = String::from_utf8_lossy(&burc_result.stdout);
    let output_root_dir = burc_stdout.trim()
        .strip_prefix("BURC_OUTPUT_ROOT_DIR=")
        .unwrap_or("")
        .trim_matches('"')
        .trim_matches('\'')
        .to_string();

    if output_root_dir.is_empty() {
        vvco_err!(output, "{}: BURC_OUTPUT_ROOT_DIR not found in fundus .buk/burc.env", cn);
        return (1, output.vvco_finish());
    }

    vvco_out!(output, "Fundus output root: {}", output_root_dir);

    // Mint legatio token and persist state
    let token = zjjrlg_mint_token(&officium_dir);
    let state = jjrlg_LegatioState {
        host: args.host.clone(),
        user: args.user.clone(),
        reldir: args.reldir.clone(),
        output_root_dir: output_root_dir.clone(),
    };

    if let Err(e) = zjjrlg_save_legatio(&officium_dir, &token, &state) {
        vvco_err!(output, "{}: {}", cn, e);
        return (1, output.vvco_finish());
    }

    vvco_out!(output, "Legatio {} bound to {}@{}:~/{}", token, state.user, state.host, state.reldir);
    vvco_out!(output, "Output root: {}", output_root_dir);

    (0, output.vvco_finish())
}

// ============================================================================
// jjx_send — synchronous exec via legatio
// ============================================================================

pub struct jjrlg_SendArgs {
    pub legatio: String,
    pub command: String,
}

/// Execute a command synchronously on the fundus via a legatio.
///
/// The command string is interpreted by `bash -c` on the fundus after
/// `cd RELDIR`. Returns 0 with remote exit code in output on SSH success;
/// returns 1 on SSH transport failure.
pub fn jjrlg_run_send(args: jjrlg_SendArgs, officium_id: &str) -> (i32, String) {
    let cn = JJRLG_CMD_NAME_SEND;
    let mut output = vvco_Output::buffer();

    let officium_dir = zjjrlg_officium_dir(officium_id);
    let state = match zjjrlg_load_legatio(&officium_dir, &args.legatio) {
        Ok(s) => s,
        Err(e) => {
            vvco_err!(output, "{}: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let remote_cmd = format!(
        "cd \"$HOME/{}\" && bash -c {}",
        state.reldir,
        zjjrlg_shell_quote(&args.command)
    );

    let result = match zjjrlg_ssh_exec(&state.host, &state.user, &remote_cmd) {
        Ok(r) => r,
        Err(e) => {
            vvco_err!(output, "{}: SSH failed: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    if !stdout.is_empty() {
        vvco_out!(output, "{}", stdout);
    }
    if !stderr.is_empty() {
        vvco_err!(output, "{}", stderr);
    }
    vvco_out!(output, "Exit: {}", result.exit_code);

    // Return 0 — SSH transport succeeded. Remote exit code is in the output.
    (0, output.vvco_finish())
}

// ============================================================================
// jjx_plant — reset fundus workspace to exact commit
// ============================================================================

pub struct jjrlg_PlantArgs {
    pub legatio: String,
    pub commit: String,
}

/// Reset the fundus workspace to an exact git commit.
///
/// Runs `git fetch origin && git reset --hard <commit> && git clean -dxf`
/// over SSH. Fail-fast on any step.
pub fn jjrlg_run_plant(args: jjrlg_PlantArgs, officium_id: &str) -> (i32, String) {
    let cn = JJRLG_CMD_NAME_PLANT;
    let mut output = vvco_Output::buffer();

    // Validate git ref contains only safe characters
    if let Err(e) = zjjrlg_validate_git_ref(&args.commit) {
        vvco_err!(output, "{}: {}", cn, e);
        return (1, output.vvco_finish());
    }

    let officium_dir = zjjrlg_officium_dir(officium_id);
    let state = match zjjrlg_load_legatio(&officium_dir, &args.legatio) {
        Ok(s) => s,
        Err(e) => {
            vvco_err!(output, "{}: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let plant_script = format!(
        concat!(
            "cd \"$HOME/{reldir}\" || exit 1\n",
            "git fetch origin || exit 2\n",
            "git reset --hard {commit} || exit 3\n",
            "git clean -dxf || exit 4\n",
            "echo \"PLANT_OK:$(git rev-parse HEAD)\""
        ),
        reldir = state.reldir,
        commit = args.commit
    );

    vvco_out!(output, "Planting {} on {}@{}:~/{} ...",
        args.commit, state.user, state.host, state.reldir);

    let result = match zjjrlg_ssh_exec(&state.host, &state.user, &plant_script) {
        Ok(r) => r,
        Err(e) => {
            vvco_err!(output, "{}: SSH failed: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    if result.exit_code != 0 {
        vvco_err!(output, "{}: failed (exit {})", cn, result.exit_code);
        if !stdout.trim().is_empty() { vvco_err!(output, "{}", stdout.trim()); }
        if !stderr.trim().is_empty() { vvco_err!(output, "{}", stderr.trim()); }
        return (1, output.vvco_finish());
    }

    if let Some(line) = stdout.lines().find(|l| l.starts_with("PLANT_OK:")) {
        vvco_out!(output, "Plant succeeded. HEAD now at {}", &line["PLANT_OK:".len()..]);
    } else {
        vvco_out!(output, "Plant succeeded.");
    }

    (0, output.vvco_finish())
}

// ============================================================================
// jjx_fetch — read single file from fundus
// ============================================================================

pub struct jjrlg_FetchArgs {
    pub legatio: String,
    pub path: String,
}

/// Read a single file from the fundus via SSH.
///
/// Path is relative to RELDIR if it doesn't start with `/`,
/// or absolute if it does. Returns file content as text (lossy UTF-8).
pub fn jjrlg_run_fetch(args: jjrlg_FetchArgs, officium_id: &str) -> (i32, String) {
    let cn = JJRLG_CMD_NAME_FETCH;
    let mut output = vvco_Output::buffer();

    let officium_dir = zjjrlg_officium_dir(officium_id);
    let state = match zjjrlg_load_legatio(&officium_dir, &args.legatio) {
        Ok(s) => s,
        Err(e) => {
            vvco_err!(output, "{}: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Absolute paths pass through; relative paths resolve against RELDIR
    let cat_cmd = if args.path.starts_with('/') {
        format!("cat {}", zjjrlg_shell_quote(&args.path))
    } else {
        format!(
            "cd \"$HOME/{}\" && cat {}",
            state.reldir,
            zjjrlg_shell_quote(&args.path)
        )
    };

    let result = match zjjrlg_ssh_exec(&state.host, &state.user, &cat_cmd) {
        Ok(r) => r,
        Err(e) => {
            vvco_err!(output, "{}: SSH failed: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    if result.exit_code != 0 {
        let stderr = String::from_utf8_lossy(&result.stderr);
        vvco_err!(output, "{}: failed (exit {}): {}", cn, result.exit_code, stderr.trim());
        return (1, output.vvco_finish());
    }

    let content = String::from_utf8_lossy(&result.stdout);
    vvco_out!(output, "{}", content);

    (0, output.vvco_finish())
}

// ============================================================================
// BURX parsing
// ============================================================================

/// Parse burx.env content as key=value pairs.
fn zjjrlg_parse_burx(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

// ============================================================================
// Tabtarget validation
// ============================================================================

/// Validate a tabtarget is a safe bare filename in tt/.
fn zjjrlg_validate_tabtarget(s: &str) -> Result<(), String> {
    if s.is_empty() {
        return Err("tabtarget must not be empty".to_string());
    }
    if s.contains('/') || s.contains("..") {
        return Err("tabtarget must be a bare filename (no path components)".to_string());
    }
    if !s.ends_with(".sh") {
        return Err("tabtarget must end with .sh".to_string());
    }
    Ok(())
}

// ============================================================================
// Nohup wrapper construction
// ============================================================================

/// Build the nohup wrapper script for async dispatch on the fundus.
///
/// The wrapper:
/// 1. Backgrounds the tabtarget with BURE_LABEL set
/// 2. Starts a watchdog that kills the tabtarget after timeout seconds
/// 3. Waits for the tabtarget to finish (or be killed)
/// 4. Cleans up the watchdog
fn zjjrlg_build_nohup_script(reldir: &str, pensum: &str, timeout: u64, tabtarget: &str) -> String {
    format!("\
cd \"$HOME/{}\" || exit 1
nohup bash -c 'export BURE_LABEL=\"{}\"; \
./tt/{} & z_child=$!; \
(sleep {} && kill -TERM $z_child 2>/dev/null) & z_watchdog=$!; \
wait $z_child; z_rc=$?; \
kill $z_watchdog 2>/dev/null; \
exit $z_rc' < /dev/null > /dev/null 2>&1 &
echo RELAY_OK",
        reldir, pensum, tabtarget, timeout
    )
}

// ============================================================================
// Pensum minting (officium-local storage)
// ============================================================================

/// Mint a pensum token via officium-local seed storage.
///
/// Reads/writes pensum_seeds.json in the officium directory (keyed by firemark).
/// No gallops lock, no git commit — pensum tokens are ephemeral correlation labels.
fn zjjrlg_mint_pensum_local(
    firemark_input: &str,
    officium_dir: &Path,
) -> Result<crate::jjrf_favor::jjrf_Pensum, String> {
    use crate::jjrf_favor::{jjrf_Firemark, jjrf_Pensum, JJRF_PENSUM_SENTINEL};
    use crate::jjru_util::zjjrg_increment_seed;

    let firemark = jjrf_Firemark::jjrf_parse(firemark_input)
        .map_err(|e| format!("Invalid firemark: {}", e))?;
    let firemark_key = firemark.jjrf_display();

    let seeds_path = officium_dir.join(PENSUM_SEEDS_FILE);

    // Load existing seeds or start empty
    let mut seeds: HashMap<String, String> = if seeds_path.exists() {
        let content = std::fs::read_to_string(&seeds_path)
            .map_err(|e| format!("Cannot read pensum seeds: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Cannot parse pensum seeds: {}", e))?
    } else {
        HashMap::new()
    };

    // Get or initialize seed for this heat
    let seed = seeds.entry(firemark_key.clone())
        .or_insert_with(|| crate::jjrt_types::JJRT_PENSUM_SEED_INIT.to_string());

    let pensum = jjrf_Pensum(format!("{}{}{}",
        firemark.jjrf_as_str(),
        JJRF_PENSUM_SENTINEL,
        seed,
    ));

    // Increment seed
    *seed = zjjrg_increment_seed(seed);

    // Persist seeds back
    let json = serde_json::to_string_pretty(&seeds)
        .map_err(|e| format!("Cannot serialize pensum seeds: {}", e))?;
    std::fs::write(&seeds_path, json.as_bytes())
        .map_err(|e| format!("Cannot write pensum seeds: {}", e))?;

    Ok(pensum)
}

// ============================================================================
// jjx_relay — async dispatch via nohup on fundus
// ============================================================================

pub struct jjrlg_RelayArgs {
    pub legatio: String,
    pub tabtarget: String,
    pub timeout: u64,
    pub firemark: String,
}

/// Verify curia git state is clean and pushed before remote dispatch.
/// Returns Ok(()) if ready, Err(message) if not.
fn zjjrlg_require_curia_ready() -> Result<(), String> {
    use crate::jjrt_types::JJRG_UNKNOWN_BASIS;

    let z_abbrev_len = JJRG_UNKNOWN_BASIS.len();

    // Check working tree is clean
    let z_status = vvc::vvce_git_command(&["status", "--porcelain"])
        .output()
        .map_err(|e| format!("git status failed: {}", e))?;
    let z_status_text = String::from_utf8_lossy(&z_status.stdout);
    let z_dirty_count = z_status_text.lines().filter(|l| !l.is_empty()).count();
    if z_dirty_count > 0 {
        return Err(format!(
            "curia working tree is dirty ({} files). Commit before dispatching.", z_dirty_count
        ));
    }

    // Check HEAD is pushed to origin
    let z_head = vvc::vvce_git_command(&["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("git rev-parse HEAD failed: {}", e))?;
    let z_head_sha = String::from_utf8_lossy(&z_head.stdout).trim().to_string();

    let z_branch = vvc::vvce_git_command(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|e| format!("git rev-parse --abbrev-ref failed: {}", e))?;
    let z_branch_name = String::from_utf8_lossy(&z_branch.stdout).trim().to_string();

    let z_remote_ref = format!("origin/{}", z_branch_name);
    let z_remote = vvc::vvce_git_command(&["rev-parse", &z_remote_ref])
        .output()
        .map_err(|e| format!("git rev-parse {} failed: {}", z_remote_ref, e))?;
    let z_remote_sha = String::from_utf8_lossy(&z_remote.stdout).trim().to_string();

    if z_head_sha != z_remote_sha {
        return Err(format!(
            "curia HEAD {} is not pushed to {} (remote {}). Push before dispatching.",
            &z_head_sha[..z_abbrev_len.min(z_head_sha.len())],
            z_remote_ref,
            &z_remote_sha[..z_abbrev_len.min(z_remote_sha.len())]
        ));
    }

    Ok(())
}

/// Launch an async dispatch on the fundus via nohup.
///
/// Mints a pensum token, launches the tabtarget via nohup wrapper over SSH,
/// reads initial burx.env to capture BURX_TEMP_DIR, persists pensum state.
pub fn jjrlg_run_relay(args: jjrlg_RelayArgs, officium_id: &str) -> (i32, String) {
    let cn = JJRLG_CMD_NAME_RELAY;
    let mut output = vvco_Output::buffer();

    // Curia readiness gate: clean tree + pushed HEAD
    if let Err(e) = zjjrlg_require_curia_ready() {
        vvco_err!(output, "{}: relay refused: {}", cn, e);
        return (1, output.vvco_finish());
    }

    if let Err(e) = zjjrlg_validate_tabtarget(&args.tabtarget) {
        vvco_err!(output, "{}: {}", cn, e);
        return (1, output.vvco_finish());
    }

    let officium_dir = zjjrlg_officium_dir(officium_id);
    if !officium_dir.is_dir() {
        vvco_err!(output, "{}: officium directory not found", cn);
        return (1, output.vvco_finish());
    }

    // Load legatio state
    let state = match zjjrlg_load_legatio(&officium_dir, &args.legatio) {
        Ok(s) => s,
        Err(e) => {
            vvco_err!(output, "{}: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Mint pensum: officium-local seed, no gallops lock
    let pensum = match zjjrlg_mint_pensum_local(&args.firemark, &officium_dir) {
        Ok(p) => p,
        Err(e) => {
            vvco_err!(output, "{}: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let pensum_token = pensum.jjrf_as_str().to_string();
    let pensum_display = pensum.jjrf_display();

    vvco_out!(output, "Relaying {} via {} ({})", args.tabtarget, args.legatio, pensum_display);

    // SSH #1: Launch nohup wrapper on fundus
    let nohup_script = zjjrlg_build_nohup_script(
        &state.reldir, &pensum_token, args.timeout, &args.tabtarget,
    );

    let launch = match zjjrlg_ssh_exec(&state.host, &state.user, &nohup_script) {
        Ok(r) => r,
        Err(e) => {
            vvco_err!(output, "{}: SSH launch failed: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let launch_stdout = String::from_utf8_lossy(&launch.stdout);
    if launch.exit_code != 0 || !launch_stdout.contains("RELAY_OK") {
        let stderr = String::from_utf8_lossy(&launch.stderr);
        vvco_err!(output, "{}: nohup launch failed (exit {})", cn, launch.exit_code);
        if !stderr.trim().is_empty() { vvco_err!(output, "  {}", stderr.trim()); }
        return (1, output.vvco_finish());
    }

    vvco_out!(output, "Dispatch launched on fundus.");

    // SSH #2: Read initial burx.env — poll with label matching to avoid stale reads
    let burx_script = format!("\
cd \"$HOME/{}\" || exit 1
z_retries=0
while [ $z_retries -lt {} ]; do
  if [ -f '{}/current/burx.env' ]; then
    z_label=$(grep '^BURX_LABEL=' '{}/current/burx.env' 2>/dev/null | head -1 | cut -d= -f2)
    if [ \"$z_label\" = '{}' ]; then
      cat '{}/current/burx.env'
      exit 0
    fi
  fi
  sleep {}
  z_retries=$((z_retries + 1))
done
exit 1",
        state.reldir,
        BURX_INITIAL_POLL_MAX_RETRIES,
        state.output_root_dir, state.output_root_dir, pensum_token,
        state.output_root_dir,
        BURX_INITIAL_POLL_DELAY_SECS,
    );

    let burx = match zjjrlg_ssh_exec(&state.host, &state.user, &burx_script) {
        Ok(r) => r,
        Err(e) => {
            vvco_err!(output, "{}: SSH burx read failed: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    if burx.exit_code != 0 {
        vvco_err!(output, "{}: burx.env with matching label not found after {} retries",
            cn, BURX_INITIAL_POLL_MAX_RETRIES);
        return (1, output.vvco_finish());
    }

    let burx_content = String::from_utf8_lossy(&burx.stdout);
    let burx_fields = zjjrlg_parse_burx(&burx_content);

    let temp_dir = match burx_fields.get("BURX_TEMP_DIR") {
        Some(d) if !d.is_empty() => d.clone(),
        _ => {
            vvco_err!(output, "{}: BURX_TEMP_DIR not found in burx.env", cn);
            return (1, output.vvco_finish());
        }
    };

    let pid = match burx_fields.get("BURX_PID") {
        Some(p) if !p.is_empty() => p.clone(),
        _ => {
            vvco_err!(output, "{}: BURX_PID not found in burx.env", cn);
            return (1, output.vvco_finish());
        }
    };

    let began_at = burx_fields.get("BURX_BEGAN_AT").cloned().unwrap_or_default();

    // Persist pensum state
    let pensum_state = jjrlg_PensumState {
        legatio: args.legatio.clone(),
        firemark: args.firemark.clone(),
        temp_dir: temp_dir.clone(),
        pid,
        tabtarget: args.tabtarget.clone(),
        timeout: args.timeout,
        began_at,
    };

    if let Err(e) = zjjrlg_save_pensum(&officium_dir, &pensum_token, &pensum_state) {
        vvco_err!(output, "{}: {}", cn, e);
        return (1, output.vvco_finish());
    }

    vvco_out!(output, "Pensum {} active.", pensum_display);
    vvco_out!(output, "Remote temp dir: {}", temp_dir);

    (0, output.vvco_finish())
}

// ============================================================================
// Pensum probe (shared by jjx_check)
// ============================================================================

/// Result of a single pensum probe via SSH.
struct zjjrlg_ProbeResult {
    report: String,
    burx_fields: HashMap<String, String>,
    files: Vec<String>,
}

/// Execute a single probe of a pensum's status via SSH.
///
/// Reads burx.env from the pensum's durable temp dir, checks for
/// BURX_EXIT_STATUS (terminal), probes PID via kill -0, and lists
/// files on terminal status.
fn zjjrlg_probe_pensum(
    legatio_state: &jjrlg_LegatioState,
    pensum_state: &jjrlg_PensumState,
) -> Result<zjjrlg_ProbeResult, String> {
    let probe_script = format!("\
if [ ! -f '{temp_dir}/burx.env' ]; then
  echo 'JJRLG_REPORT:lost'
  exit 0
fi
cat '{temp_dir}/burx.env'
echo 'JJRLG_BURX_END'
z_es=$(grep '^BURX_EXIT_STATUS=' '{temp_dir}/burx.env' 2>/dev/null | head -1 | cut -d= -f2)
if [ -n \"$z_es\" ]; then
  echo 'JJRLG_REPORT:stopped'
  echo 'JJRLG_FILES_BEGIN'
  ls -1 '{temp_dir}/'
  echo 'JJRLG_FILES_END'
else
  if kill -0 {pid} 2>/dev/null; then
    echo 'JJRLG_REPORT:running'
  else
    echo 'JJRLG_REPORT:orphaned'
  fi
fi",
        temp_dir = pensum_state.temp_dir,
        pid = pensum_state.pid,
    );

    let result = zjjrlg_ssh_exec(
        &legatio_state.host,
        &legatio_state.user,
        &probe_script,
    ).map_err(|e| format!("SSH probe failed: {}", e))?;

    let stdout = String::from_utf8_lossy(&result.stdout);

    // Parse report line
    let report = stdout.lines()
        .find(|l| l.starts_with("JJRLG_REPORT:"))
        .map(|l| l["JJRLG_REPORT:".len()..].to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Parse BURX fields (lines before JJRLG_BURX_END, excluding sentinel lines)
    let burx_content: String = stdout.lines()
        .take_while(|l| !l.starts_with("JJRLG_BURX_END"))
        .filter(|l| !l.starts_with("JJRLG_"))
        .collect::<Vec<_>>()
        .join("\n");
    let burx_fields = zjjrlg_parse_burx(&burx_content);

    // Parse file list (between JJRLG_FILES_BEGIN and JJRLG_FILES_END)
    let mut in_files = false;
    let mut files = Vec::new();
    for line in stdout.lines() {
        if line == "JJRLG_FILES_BEGIN" {
            in_files = true;
            continue;
        }
        if line == "JJRLG_FILES_END" {
            break;
        }
        if in_files && !line.is_empty() {
            files.push(line.to_string());
        }
    }

    Ok(zjjrlg_ProbeResult { report, burx_fields, files })
}

/// Format probe result into output buffer.
fn zjjrlg_format_probe_output(pensum_display: &str, probe: &zjjrlg_ProbeResult, output: &mut vvco_Output) {
    vvco_out!(output, "Pensum: {}", pensum_display);
    vvco_out!(output, "Report: {}", probe.report);

    for key in &["BURX_PID", "BURX_BEGAN_AT", "BURX_TABTARGET", "BURX_TEMP_DIR",
                  "BURX_TRANSCRIPT", "BURX_LOG_HIST", "BURX_LABEL",
                  "BURX_EXIT_STATUS", "BURX_ENDED_AT"] {
        if let Some(val) = probe.burx_fields.get(*key) {
            if !val.is_empty() {
                vvco_out!(output, "{}: {}", key, val);
            }
        }
    }

    if !probe.files.is_empty() {
        vvco_out!(output, "Files:");
        for f in &probe.files {
            vvco_out!(output, "  {}", f);
        }
    }
}

// ============================================================================
// jjx_check — probe or poll pensum status
// ============================================================================

pub struct jjrlg_CheckArgs {
    pub pensum: String,
    pub timeout: u64,
}

/// Probe or poll a pensum's status on the fundus.
///
/// timeout=0: instant probe (single SSH round-trip).
/// timeout>0: poll until terminal state or timeout expires.
///
/// Returns BURX fields + liveness report (running/orphaned/stopped/lost).
/// On terminal status (stopped), also returns file list from BURX_TEMP_DIR.
pub fn jjrlg_run_check(args: jjrlg_CheckArgs, officium_id: &str) -> (i32, String) {
    let cn = JJRLG_CMD_NAME_CHECK;
    let mut output = vvco_Output::buffer();

    let officium_dir = zjjrlg_officium_dir(officium_id);

    let parsed = match crate::jjrf_favor::jjrf_Pensum::jjrf_parse(&args.pensum) {
        Ok(p) => p,
        Err(e) => {
            vvco_err!(output, "{}: invalid pensum: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let pensum_token = parsed.jjrf_as_str().to_string();
    let pensum_display = parsed.jjrf_display();

    let pensum_state = match zjjrlg_load_pensum(&officium_dir, &pensum_token) {
        Ok(s) => s,
        Err(e) => {
            vvco_err!(output, "{}: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let legatio_state = match zjjrlg_load_legatio(&officium_dir, &pensum_state.legatio) {
        Ok(s) => s,
        Err(e) => {
            vvco_err!(output, "{}: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let deadline = if args.timeout > 0 {
        Some(std::time::Instant::now() + std::time::Duration::from_secs(args.timeout))
    } else {
        None
    };

    loop {
        let probe = match zjjrlg_probe_pensum(&legatio_state, &pensum_state) {
            Ok(p) => p,
            Err(e) => {
                vvco_err!(output, "{}: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        let is_terminal = probe.report == "stopped" || probe.report == "lost";

        if is_terminal || deadline.is_none() {
            zjjrlg_format_probe_output(&pensum_display, &probe, &mut output);
            return (0, output.vvco_finish());
        }

        if let Some(dl) = deadline {
            if std::time::Instant::now() >= dl {
                zjjrlg_format_probe_output(&pensum_display, &probe, &mut output);
                return (0, output.vvco_finish());
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(BURX_CHECK_POLL_INTERVAL_SECS));
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_reldir_valid() {
        assert!(jjrlg_validate_reldir("projects/rbm_alpha_recipemuster").is_ok());
        assert!(jjrlg_validate_reldir("a/b").is_ok());
        assert!(jjrlg_validate_reldir("deep/nested/path/here").is_ok());
    }

    #[test]
    fn test_validate_reldir_empty() {
        assert!(jjrlg_validate_reldir("").is_err());
    }

    #[test]
    fn test_validate_reldir_absolute() {
        assert!(jjrlg_validate_reldir("/usr/local").is_err());
    }

    #[test]
    fn test_validate_reldir_dot() {
        assert!(jjrlg_validate_reldir("../escape").is_err());
        assert!(jjrlg_validate_reldir("./here").is_err());
    }

    #[test]
    fn test_validate_reldir_no_slash() {
        assert!(jjrlg_validate_reldir("projects").is_err());
    }

    #[test]
    fn test_validate_git_ref() {
        assert!(zjjrlg_validate_git_ref("abc123").is_ok());
        assert!(zjjrlg_validate_git_ref("main").is_ok());
        assert!(zjjrlg_validate_git_ref("origin/main").is_ok());
        assert!(zjjrlg_validate_git_ref("v1.2.3").is_ok());
        assert!(zjjrlg_validate_git_ref("feature-branch").is_ok());
        assert!(zjjrlg_validate_git_ref("").is_err());
        assert!(zjjrlg_validate_git_ref("$(whoami)").is_err());
        assert!(zjjrlg_validate_git_ref("ref; rm -rf /").is_err());
    }

    #[test]
    fn test_shell_quote() {
        assert_eq!(zjjrlg_shell_quote("hello"), "'hello'");
        assert_eq!(zjjrlg_shell_quote("it's"), "'it'\\''s'");
        assert_eq!(zjjrlg_shell_quote(""), "''");
        assert_eq!(zjjrlg_shell_quote("a b c"), "'a b c'");
    }

    #[test]
    fn test_mint_token_empty_dir() {
        let dir = std::env::temp_dir().join("jjrlg_test_mint_empty");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(zjjrlg_mint_token(&dir), "L0");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_mint_token_with_existing() {
        let dir = std::env::temp_dir().join("jjrlg_test_mint_existing");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("legatio_L0.json"), "{}").unwrap();
        std::fs::write(dir.join("legatio_L1.json"), "{}").unwrap();
        assert_eq!(zjjrlg_mint_token(&dir), "L2");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_validate_tabtarget_valid() {
        assert!(zjjrlg_validate_tabtarget("rbw-tf.TestFixture.regime-validation.sh").is_ok());
        assert!(zjjrlg_validate_tabtarget("vow-b.Build.sh").is_ok());
    }

    #[test]
    fn test_validate_tabtarget_empty() {
        assert!(zjjrlg_validate_tabtarget("").is_err());
    }

    #[test]
    fn test_validate_tabtarget_path_traversal() {
        assert!(zjjrlg_validate_tabtarget("../escape.sh").is_err());
        assert!(zjjrlg_validate_tabtarget("sub/dir.sh").is_err());
    }

    #[test]
    fn test_validate_tabtarget_no_sh_extension() {
        assert!(zjjrlg_validate_tabtarget("something.py").is_err());
        assert!(zjjrlg_validate_tabtarget("noext").is_err());
    }

    #[test]
    fn test_parse_burx_fields() {
        let content = "BURX_PID=12345\nBURX_BEGAN_AT=20260403-102137.482951000\nBURX_TEMP_DIR=/home/user/.buk/temp-20260403\nBURX_LABEL=A4%AA";
        let fields = zjjrlg_parse_burx(content);
        assert_eq!(fields.get("BURX_PID").unwrap(), "12345");
        assert_eq!(fields.get("BURX_BEGAN_AT").unwrap(), "20260403-102137.482951000");
        assert_eq!(fields.get("BURX_TEMP_DIR").unwrap(), "/home/user/.buk/temp-20260403");
        assert_eq!(fields.get("BURX_LABEL").unwrap(), "A4%AA");
    }

    #[test]
    fn test_parse_burx_with_terminal_fields() {
        let content = "BURX_PID=12345\nBURX_EXIT_STATUS=0\nBURX_ENDED_AT=20260403-102200";
        let fields = zjjrlg_parse_burx(content);
        assert_eq!(fields.get("BURX_EXIT_STATUS").unwrap(), "0");
        assert_eq!(fields.get("BURX_ENDED_AT").unwrap(), "20260403-102200");
    }

    #[test]
    fn test_parse_burx_empty() {
        let fields = zjjrlg_parse_burx("");
        assert!(fields.is_empty());
    }

    #[test]
    fn test_nohup_script_structure() {
        let script = zjjrlg_build_nohup_script("projects/test", "A4%AA", 300, "rbw-tf.Test.sh");
        assert!(script.contains("BURE_LABEL"));
        assert!(script.contains("A4%AA"));
        assert!(script.contains("tt/rbw-tf.Test.sh"));
        assert!(script.contains("sleep 300"));
        assert!(script.contains("RELAY_OK"));
        assert!(script.contains("nohup"));
        assert!(script.contains("< /dev/null"));
    }

    #[test]
    fn test_pensum_state_roundtrip() {
        let dir = std::env::temp_dir().join("jjrlg_test_pensum_roundtrip");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let state = jjrlg_PensumState {
            legatio: "L0".to_string(),
            firemark: "A4".to_string(),
            temp_dir: "/home/test/.buk/temp-20260403".to_string(),
            pid: "12345".to_string(),
            tabtarget: "rbw-tf.Test.sh".to_string(),
            timeout: 300,
            began_at: "20260403-102137".to_string(),
        };

        zjjrlg_save_pensum(&dir, "A4%AA", &state).unwrap();
        let loaded = zjjrlg_load_pensum(&dir, "A4%AA").unwrap();
        assert_eq!(loaded.legatio, "L0");
        assert_eq!(loaded.temp_dir, "/home/test/.buk/temp-20260403");
        assert_eq!(loaded.pid, "12345");
        assert_eq!(loaded.timeout, 300);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
