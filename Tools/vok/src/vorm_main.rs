//! VOK Main - Voce Viva Rust binary entry point
//!
//! Provides core VOK commands and delegates kit commands via external subcommand pattern.
//! JJK commands (jjx_*) are handled entirely by the jjk crate when the feature is enabled.

use clap::Parser;
use std::ffi::OsString;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "vvr")]
#[command(version)]
#[command(about = "Voce Viva Rust - Platform utilities for Claude Code kits")]
#[command(allow_external_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Pre-commit size validation
    Guard(GuardArgs),

    /// Atomic commit with lock, stage, guard, and optional claude message
    #[command(name = "vvx_commit")]
    VvxCommit(CommitArgs),

    /// Push with lock (prevents concurrent push/commit)
    #[command(name = "vvx_push")]
    VvxPush(PushArgs),

    /// Force-break a stuck lock (use after crash)
    #[command(name = "vvx_unlock")]
    VvxUnlock,

    /// External subcommands (delegated to kit CLIs)
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

/// Arguments for guard command (wraps vvc)
#[derive(clap::Args, Debug)]
struct GuardArgs {
    /// Size limit in bytes (default: 500000)
    #[arg(long, default_value = "500000")]
    limit: u64,

    /// Warning threshold in bytes (default: 250000)
    #[arg(long, default_value = "250000")]
    warn: u64,
}

/// Arguments for vvx_commit command (wraps vvc)
#[derive(clap::Args, Debug)]
struct CommitArgs {
    /// Prefix string to prepend to commit message
    #[arg(long)]
    prefix: Option<String>,

    /// Commit message; if absent, invoke claude to generate from diff
    #[arg(short, long)]
    message: Option<String>,

    /// Allow empty commits
    #[arg(long)]
    allow_empty: bool,

    /// Skip 'git add -A' (respect pre-staged files)
    #[arg(long)]
    no_stage: bool,
}

/// Arguments for vvx_push command
#[derive(clap::Args, Debug)]
struct PushArgs {
    /// Remote name (default: origin)
    #[arg(long, default_value = "origin")]
    remote: String,

    /// Branch name (default: current branch)
    #[arg(long)]
    branch: Option<String>,

    /// Force push (use with caution)
    #[arg(long)]
    force: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Some(Commands::Guard(args)) => run_guard(args),
        Some(Commands::VvxCommit(args)) => run_commit(args),
        Some(Commands::VvxPush(args)) => run_push(args),
        Some(Commands::VvxUnlock) => run_unlock(),
        Some(Commands::External(args)) => dispatch_external(args),
        None => {
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            0
        }
    };

    ExitCode::from(exit_code as u8)
}

/// Run guard command using vvc
fn run_guard(args: GuardArgs) -> i32 {
    let vvc_args = vvc::vvcg_GuardArgs {
        limit: args.limit,
        warn: args.warn,
    };
    vvc::guard(&vvc_args)
}

/// Run commit command using vvc
fn run_commit(args: CommitArgs) -> i32 {
    let vvc_args = vvc::vvcc_CommitArgs {
        prefix: args.prefix,
        message: args.message,
        allow_empty: args.allow_empty,
        no_stage: args.no_stage,
    };
    vvc::commit(&vvc_args)
}

/// Dispatch external subcommands to appropriate kit CLIs
fn dispatch_external(args: Vec<OsString>) -> i32 {
    if args.is_empty() {
        eprintln!("vvx: error: no subcommand provided");
        return 1;
    }

    let cmd_name = args[0].to_string_lossy();

    // Delegate to JJK if available and command matches
    #[cfg(feature = "jjk")]
    if jjk::is_jjk_command(&cmd_name) {
        return jjk::dispatch(&args);
    }

    // Unknown external subcommand
    eprintln!("vvx: error: unknown command '{}'", cmd_name);
    eprintln!("Run 'vvx --help' for available commands");
    1
}

/// Lock reference path for push operations (same as commit to prevent concurrent ops)
const LOCK_REF: &str = "refs/vvg/locks/vvx";

/// Force-break a stuck lock
fn run_unlock() -> i32 {
    use std::process::Command;

    // Check if lock exists
    let check = Command::new("git")
        .args(["show-ref", "--verify", "--quiet", LOCK_REF])
        .status();

    match check {
        Ok(status) if status.success() => {
            // Lock exists, delete it
            let delete = Command::new("git")
                .args(["update-ref", "-d", LOCK_REF])
                .output();

            match delete {
                Ok(output) if output.status.success() => {
                    eprintln!("unlock: lock broken successfully");
                    0
                }
                Ok(output) => {
                    eprintln!("unlock: error: failed to delete lock: {}",
                        String::from_utf8_lossy(&output.stderr));
                    1
                }
                Err(e) => {
                    eprintln!("unlock: error: {}", e);
                    1
                }
            }
        }
        Ok(_) => {
            // Lock doesn't exist
            eprintln!("unlock: no lock held");
            0
        }
        Err(e) => {
            eprintln!("unlock: error checking lock: {}", e);
            1
        }
    }
}

fn run_push(args: PushArgs) -> i32 {
    use std::process::Command;

    eprintln!("push: acquiring lock...");
    let head_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output();

    let lock_value = match head_output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "0000000000000000000000000000000000000000".to_string(),
    };

    let lock_result = Command::new("git")
        .args(["update-ref", LOCK_REF, &lock_value, ""])
        .output();

    match lock_result {
        Ok(output) if output.status.success() => {
            eprintln!("push: lock acquired");
        }
        _ => {
            eprintln!("push: error: Another operation in progress - lock held");
            return 1;
        }
    }

    let result = run_push_workflow(&args);

    let _ = Command::new("git")
        .args(["update-ref", "-d", LOCK_REF])
        .output();
    eprintln!("push: lock released");

    match result {
        Ok(()) => {
            eprintln!("push: success");
            0
        }
        Err(e) => {
            eprintln!("push: error: {}", e);
            1
        }
    }
}

fn run_push_workflow(args: &PushArgs) -> Result<(), String> {
    use std::process::Command;

    let branch = match &args.branch {
        Some(b) => b.clone(),
        None => {
            let output = Command::new("git")
                .args(["rev-parse", "--abbrev-ref", "HEAD"])
                .output()
                .map_err(|e| format!("Failed to get current branch: {}", e))?;

            if !output.status.success() {
                return Err("Failed to determine current branch".to_string());
            }

            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
    };

    eprintln!("push: pushing {} to {}", branch, args.remote);

    let mut push_args = vec!["push", &args.remote, &branch];
    if args.force {
        push_args.push("--force");
    }

    let output = Command::new("git")
        .args(&push_args)
        .output()
        .map_err(|e| format!("Failed to run git push: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git push failed: {}", stderr));
    }

    Ok(())
}
