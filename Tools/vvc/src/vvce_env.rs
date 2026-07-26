// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Environment - BUD environment validation and access
//!
//! Provides validated access to BUD environment variables that are
//! guaranteed to be set when vvx is invoked via tabtarget.
//!
//! Validates on first access and panics if any required variable is missing.
//! This is intentional fail-fast behavior - vvx should never run outside
//! the BUK tabtarget context.

use std::path::PathBuf;
use std::sync::OnceLock;

/// BUD environment - validated on first access
pub struct VvcEnv {
    /// Ephemeral temp directory for intermediate files (BURD_TEMP_DIR)
    pub temp_dir: PathBuf,
    /// Output directory for command results (BURD_OUTPUT_DIR)
    pub output_dir: PathBuf,
    /// Invocation identity timestamp-pid-random (BURD_NOW_STAMP)
    pub now_stamp: String,
    /// Git describe output for provenance (BURD_GIT_CONTEXT)
    pub git_context: String,
}

/// Singleton storage for validated environment
static ENV: OnceLock<VvcEnv> = OnceLock::new();

/// Get validated BUD environment.
///
/// On first call, validates all required environment variables and caches
/// the result. Panics if any required variable is missing or invalid.
///
/// Subsequent calls return the cached reference.
///
/// # Panics
///
/// Panics if any of these environment variables are missing:
/// - `BURD_TEMP_DIR` - must be set and point to an existing directory
/// - `BURD_OUTPUT_DIR` - must be set and point to an existing directory
/// - `BURD_NOW_STAMP` - must be set (non-empty)
/// - `BURD_GIT_CONTEXT` - must be set (non-empty)
///
/// This is intentional fail-fast behavior. vvx should only run via
/// BUK tabtarget which guarantees these variables are set.
pub fn vvce_env() -> &'static VvcEnv {
    ENV.get_or_init(|| zvvce_validate_env())
}

/// Validate environment and construct VvcEnv, or panic with clear error
fn zvvce_validate_env() -> VvcEnv {
    let mut missing: Vec<&str> = Vec::new();
    let mut invalid: Vec<String> = Vec::new();

    // Validate BURD_TEMP_DIR (canonicalize to absolute path for test compatibility)
    let temp_dir = match std::env::var("BURD_TEMP_DIR") {
        Ok(v) if !v.is_empty() => {
            let path = PathBuf::from(&v);
            match path.canonicalize() {
                Ok(abs_path) => {
                    if !abs_path.is_dir() {
                        invalid.push(format!("BURD_TEMP_DIR='{}' is not a directory", v));
                    }
                    abs_path
                }
                Err(e) => {
                    invalid.push(format!("BURD_TEMP_DIR='{}' cannot be resolved: {}", v, e));
                    path
                }
            }
        }
        Ok(_) => {
            missing.push("BURD_TEMP_DIR");
            PathBuf::new()
        }
        Err(_) => {
            missing.push("BURD_TEMP_DIR");
            PathBuf::new()
        }
    };

    // Validate BURD_OUTPUT_DIR (canonicalize to absolute path for consistency)
    let output_dir = match std::env::var("BURD_OUTPUT_DIR") {
        Ok(v) if !v.is_empty() => {
            let path = PathBuf::from(&v);
            match path.canonicalize() {
                Ok(abs_path) => {
                    if !abs_path.is_dir() {
                        invalid.push(format!("BURD_OUTPUT_DIR='{}' is not a directory", v));
                    }
                    abs_path
                }
                Err(e) => {
                    invalid.push(format!("BURD_OUTPUT_DIR='{}' cannot be resolved: {}", v, e));
                    path
                }
            }
        }
        Ok(_) => {
            missing.push("BURD_OUTPUT_DIR");
            PathBuf::new()
        }
        Err(_) => {
            missing.push("BURD_OUTPUT_DIR");
            PathBuf::new()
        }
    };

    // Validate BURD_NOW_STAMP
    let now_stamp = match std::env::var("BURD_NOW_STAMP") {
        Ok(v) if !v.is_empty() => v,
        _ => {
            missing.push("BURD_NOW_STAMP");
            String::new()
        }
    };

    // Validate BURD_GIT_CONTEXT
    let git_context = match std::env::var("BURD_GIT_CONTEXT") {
        Ok(v) if !v.is_empty() => v,
        _ => {
            missing.push("BURD_GIT_CONTEXT");
            String::new()
        }
    };

    // Report all errors at once
    if !missing.is_empty() || !invalid.is_empty() {
        let mut msg = String::from("vvx: BUD environment validation failed\n");
        msg.push_str("\nvvx must be invoked via BUK tabtarget (tt/vvw-r.RunVVX.sh)\n");

        if !missing.is_empty() {
            msg.push_str("\nMissing environment variables:\n");
            for var in &missing {
                msg.push_str(&format!("  - {}\n", var));
            }
        }

        if !invalid.is_empty() {
            msg.push_str("\nInvalid environment variables:\n");
            for err in &invalid {
                msg.push_str(&format!("  - {}\n", err));
            }
        }

        panic!("{}", msg);
    }

    VvcEnv {
        temp_dir,
        output_dir,
        now_stamp,
        git_context,
    }
}

/// Create a Command for invoking git as a subprocess.
///
/// Disconnects stdin to prevent child process from inheriting the
/// MCP server's stdin fd, which causes transport deadlock.
pub fn vvce_git_command(args: &[&str]) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    cmd.args(args);
    cmd.stdin(std::process::Stdio::null());
    cmd
}

/// Create a Command for invoking claude as a subprocess.
///
/// Removes the CLAUDECODE environment variable to prevent the
/// nested-session guard (v2.1.39+) from blocking subprocess invocations
/// that are legitimate tool usage, not interactive nesting.
/// Disconnects stdin to prevent MCP transport deadlock.
pub fn vvce_claude_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("claude");
    cmd.env_remove("CLAUDECODE");
    cmd.stdin(std::process::Stdio::null());
    cmd
}

/// Run a command to completion under a deadline — the bounded sibling of
/// `Command::output()` (JJr_9m4: every remote-reaching subprocess wait carries
/// a deadline).
///
/// Returns `Ok(Some(output))` when the child exits within the deadline,
/// `Ok(None)` when the deadline expired (the child is killed and reaped —
/// no zombie survives the expiry), and `Err` only on a spawn failure, which
/// stays the caller's precondition territory exactly as with `output()`.
///
/// Stdout and stderr are drained on their own threads while the wait polls
/// `try_wait`, so a child filling a pipe can never deadlock against the
/// deadline watching it. `stdin_data`, when given, is written on its own
/// thread for the same reason; when absent the caller's own stdin
/// configuration stands.
pub fn vvce_output_deadline(
    cmd: &mut std::process::Command,
    stdin_data: Option<Vec<u8>>,
    deadline: std::time::Duration,
) -> std::io::Result<Option<std::process::Output>> {
    use std::io::Read;
    use std::io::Write;

    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    if stdin_data.is_some() {
        cmd.stdin(std::process::Stdio::piped());
    }
    let mut child = cmd.spawn()?;

    let stdin_thread = stdin_data.map(|data| {
        let mut stdin_pipe = child.stdin.take().expect("piped stdin must be present on a fresh child");
        std::thread::spawn(move || {
            let _ = stdin_pipe.write_all(&data);
            // stdin_pipe drops here, closing the pipe so the child sees EOF.
        })
    });

    let mut stdout_pipe = child.stdout.take().expect("piped stdout must be present on a fresh child");
    let mut stderr_pipe = child.stderr.take().expect("piped stderr must be present on a fresh child");
    let stdout_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut buf);
        buf
    });
    let stderr_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr_pipe.read_to_end(&mut buf);
        buf
    });

    let started = std::time::Instant::now();
    let poll = std::time::Duration::from_millis(50);
    let status = loop {
        match child.try_wait()? {
            Some(status) => break Some(status),
            None => {
                if started.elapsed() >= deadline {
                    break None;
                }
                std::thread::sleep(poll);
            }
        }
    };

    let status = match status {
        Some(status) => status,
        None => {
            // Expired: kill, then reap — kill() on an already-dead child is a
            // benign race, and wait() guarantees no zombie either way. The
            // drain threads are deliberately NOT joined here: a grandchild the
            // kill cannot reach may still hold the pipes open, and joining
            // would hand the stall right back to the caller the deadline just
            // rescued. The threads exit on their own when the pipes close.
            let _ = child.kill();
            let _ = child.wait();
            drop(stdin_thread);
            drop(stdout_thread);
            drop(stderr_thread);
            return Ok(None);
        }
    };

    if let Some(t) = stdin_thread {
        let _ = t.join();
    }
    let stdout = stdout_thread.join().unwrap_or_default();
    let stderr = stderr_thread.join().unwrap_or_default();
    Ok(Some(std::process::Output { status, stdout, stderr }))
}

#[cfg(test)]
mod tests {
    // Note: These tests are intentionally minimal because they would
    // require manipulating environment variables, which is problematic
    // in parallel test execution. The validation logic is straightforward
    // and the real test is integration: does vvx fail properly when
    // invoked outside BUK context?

    #[test]
    fn test_vvcenv_struct_fields() {
        // Just verify the struct compiles with expected fields
        use super::VvcEnv;
        use std::path::PathBuf;

        let _env = VvcEnv {
            temp_dir: PathBuf::from("/tmp"),
            output_dir: PathBuf::from("/out"),
            now_stamp: "20260118-120000-1234-567".to_string(),
            git_context: "v1.0.0-5-gabc1234".to_string(),
        };
    }
}
