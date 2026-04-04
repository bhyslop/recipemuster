// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Legatio — Remote dispatch via SSH
//!
//! Implements the legatio (session) tier of remote execution:
//! - `jjx_bind`  — SSH probe, RELDIR validation, legatio minting
//! - `jjx_send`  — synchronous command execution via legatio
//! - `jjx_plant` — reset fundus workspace to exact commit
//! - `jjx_fetch` — read single file from fundus
//!
//! SSH transport uses platform `ssh` binary via std::process::Command.
//! No Rust SSH crate — platform openssh inherits user's ~/.ssh/config,
//! known_hosts, and agent forwarding.

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use vvc::{vvco_out, vvco_err, vvco_Output};

// ============================================================================
// Constants
// ============================================================================

/// Default user for fundus SSH connections
pub const JJRLG_FUNDUS_DEFAULT_USER: &str = "rbtest";

/// Default relative directory for fundus projects
pub const JJRLG_FUNDUS_DEFAULT_RELDIR: &str = "projects/rbm_alpha_recipemuster";

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
}
