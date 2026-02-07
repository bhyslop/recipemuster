// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOK Main - Voce Viva Rust binary entry point
//!
//! Provides core VOK commands and delegates kit commands via external subcommand pattern.
//! JJK commands (jjx_*) are handled entirely by the jjk crate when the feature is enabled.

use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;
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

    /// Create invitatory commit to open new officium
    #[command(name = "vvx_invitatory")]
    VvxInvitatory,

    /// Push with lock (prevents concurrent push/commit)
    #[command(name = "vvx_push")]
    VvxPush(PushArgs),

    /// Force-break a stuck lock (use after crash)
    #[command(name = "vvx_unlock")]
    VvxUnlock,

    /// Collect kit assets to staging directory
    #[command(name = "release_collect")]
    ReleaseCollect(ReleaseCollectArgs),

    /// Brand staging directory with hallmark
    #[command(name = "release_brand")]
    ReleaseBrand(ReleaseBrandArgs),

    /// Install kit assets from parcel to target repo
    #[command(name = "vvx_emplace")]
    VvxEmplace(EmplaceArgs),

    /// Remove kit assets from target repo
    #[command(name = "vvx_vacate")]
    VvxVacate(VacateArgs),

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

/// Arguments for release_collect command
#[derive(clap::Args, Debug)]
struct ReleaseCollectArgs {
    /// Staging directory to populate
    #[arg(long)]
    staging: PathBuf,

    /// Source Tools/ directory
    #[arg(long)]
    tools_dir: PathBuf,

    /// Path to vvi_install.sh
    #[arg(long)]
    install_script: PathBuf,

    /// Comma-separated list of kit IDs (from BURC_MANAGED_KITS)
    #[arg(long)]
    managed_kits: String,
}

/// Arguments for release_brand command
#[derive(clap::Args, Debug)]
struct ReleaseBrandArgs {
    /// Staging directory with collected assets
    #[arg(long)]
    staging: PathBuf,

    /// Path to vovr_registry.json
    #[arg(long)]
    registry: PathBuf,

    /// Current git commit SHA
    #[arg(long)]
    commit: String,

    /// Comma-separated list of kit IDs (from BURC_MANAGED_KITS)
    #[arg(long)]
    managed_kits: String,
}

/// Arguments for vvx_emplace command
#[derive(clap::Args, Debug)]
struct EmplaceArgs {
    /// Extracted parcel directory (contains vvbf_brand.json, kits/)
    #[arg(long)]
    parcel: PathBuf,

    /// Path to target repo's burc.env file
    #[arg(long)]
    burc: PathBuf,
}

/// Arguments for vvx_vacate command
#[derive(clap::Args, Debug)]
struct VacateArgs {
    /// Path to target repo's burc.env file
    #[arg(long)]
    burc: PathBuf,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Some(Commands::Guard(args)) => run_guard(args),
        Some(Commands::VvxCommit(args)) => run_commit(args),
        Some(Commands::VvxInvitatory) => run_invitatory().await,
        Some(Commands::VvxPush(args)) => run_push(args),
        Some(Commands::VvxUnlock) => run_unlock(),
        Some(Commands::ReleaseCollect(args)) => run_release_collect(args),
        Some(Commands::ReleaseBrand(args)) => run_release_brand(args),
        Some(Commands::VvxEmplace(args)) => run_emplace(args),
        Some(Commands::VvxVacate(args)) => run_vacate(args),
        Some(Commands::External(args)) => dispatch_external(args).await,
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
    vvc::guard(&vvc_args, None)
}

/// Run commit command using vvc
fn run_commit(args: CommitArgs) -> i32 {
    let vvc_args = vvc::vvcc_CommitArgs {
        prefix: args.prefix,
        message: args.message,
        allow_empty: args.allow_empty,
        no_stage: args.no_stage,
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };
    vvc::commit(&vvc_args)
}

/// Run invitatory command using vvc
async fn run_invitatory() -> i32 {
    match vvc::vvcp_invitatory().await {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("invitatory: error: {}", e);
            1
        }
    }
}

/// Dispatch external subcommands to appropriate kit CLIs
async fn dispatch_external(args: Vec<OsString>) -> i32 {
    if args.is_empty() {
        eprintln!("vvx: error: no subcommand provided");
        return 1;
    }

    let cmd_name = args[0].to_string_lossy();

    // Delegate to JJK if available and command matches
    #[cfg(feature = "jjk")]
    if jjk::jjrx_is_jjk_command(&cmd_name) {
        return jjk::jjrx_dispatch(&args).await;
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
            // Lock acquired
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

/// Run release_collect command
fn run_release_collect(args: ReleaseCollectArgs) -> i32 {
    eprintln!("release_collect: collecting assets...");
    eprintln!("  staging: {}", args.staging.display());
    eprintln!("  tools_dir: {}", args.tools_dir.display());
    eprintln!("  install_script: {}", args.install_script.display());

    // Parse comma-separated kit list
    let managed_kits: Vec<String> = args.managed_kits
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    match vof::vofr_collect(&args.tools_dir, &args.staging, &args.install_script, &managed_kits) {
        Ok(result) => {
            // Output JSON for bash to parse
            let mut output = serde_json::Map::new();
            output.insert("total_files".to_string(), serde_json::Value::Number(result.total_files.into()));
            output.insert("commands_routed".to_string(), serde_json::Value::Number(result.commands_routed.into()));

            let kit_counts: serde_json::Map<String, serde_json::Value> = result.kit_counts
                .into_iter()
                .map(|(k, v)| (k, serde_json::Value::Number(v.into())))
                .collect();
            output.insert("kit_counts".to_string(), serde_json::Value::Object(kit_counts));

            println!("{}", serde_json::to_string_pretty(&serde_json::Value::Object(output)).unwrap());
            eprintln!("release_collect: success - {} files collected", result.total_files);
            0
        }
        Err(e) => {
            eprintln!("release_collect: error: {}", e);
            1
        }
    }
}

/// Run release_brand command
fn run_release_brand(args: ReleaseBrandArgs) -> i32 {
    eprintln!("release_brand: branding staging...");
    eprintln!("  staging: {}", args.staging.display());
    eprintln!("  registry: {}", args.registry.display());
    eprintln!("  commit: {}", args.commit);

    // Parse comma-separated kit list
    let managed_kits: Vec<String> = args.managed_kits
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    match vof::vofr_brand(&args.staging, &args.registry, &args.commit, &managed_kits) {
        Ok(result) => {
            // Output hallmark for bash to use in tarball name
            println!("{}", result.hallmark);
            if result.is_new {
                eprintln!("release_brand: allocated new hallmark {}", result.hallmark);
            } else {
                eprintln!("release_brand: reusing existing hallmark {}", result.hallmark);
            }
            eprintln!("release_brand: super-SHA: {}", result.super_sha);
            0
        }
        Err(e) => {
            eprintln!("release_brand: error: {}", e);
            1
        }
    }
}

/// Run vvx_emplace command
fn run_emplace(args: EmplaceArgs) -> i32 {
    eprintln!("emplace: installing kit assets...");
    eprintln!("  parcel: {}", args.parcel.display());
    eprintln!("  burc: {}", args.burc.display());

    let emplace_args = vof::vofe_EmplaceArgs {
        parcel_dir: args.parcel,
        burc_path: args.burc,
    };

    match vof::vofe_emplace(&emplace_args) {
        Ok(result) => {
            // Output JSON summary
            let mut output = serde_json::Map::new();
            output.insert("hallmark".to_string(), serde_json::Value::Number(result.hallmark.into()));

            let kits: Vec<serde_json::Value> = result.kits_installed
                .iter()
                .map(|k| serde_json::Value::String(k.clone()))
                .collect();
            output.insert("kits_installed".to_string(), serde_json::Value::Array(kits));

            output.insert("files_copied".to_string(), serde_json::Value::Number(result.files_copied.into()));
            output.insert("commands_routed".to_string(), serde_json::Value::Number(result.commands_routed.into()));
            output.insert("hooks_routed".to_string(), serde_json::Value::Number(result.hooks_routed.into()));

            let sections: Vec<serde_json::Value> = result.claude_sections_updated
                .iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect();
            output.insert("claude_sections_updated".to_string(), serde_json::Value::Array(sections));

            println!("{}", serde_json::to_string_pretty(&serde_json::Value::Object(output)).unwrap());

            eprintln!("emplace: success - {} files, {} commands, {} hooks",
                result.files_copied, result.commands_routed, result.hooks_routed);
            0
        }
        Err(e) => {
            eprintln!("emplace: error: {}", e);
            1
        }
    }
}

/// Run vvx_vacate command
fn run_vacate(args: VacateArgs) -> i32 {
    eprintln!("vacate: removing kit assets...");
    eprintln!("  burc: {}", args.burc.display());

    let vacate_args = vof::vofe_VacateArgs {
        burc_path: args.burc,
    };

    match vof::vofe_vacate(&vacate_args) {
        Ok(result) => {
            // Output JSON summary
            let mut output = serde_json::Map::new();

            let kits: Vec<serde_json::Value> = result.kits_removed
                .iter()
                .map(|k| serde_json::Value::String(k.clone()))
                .collect();
            output.insert("kits_removed".to_string(), serde_json::Value::Array(kits));

            output.insert("files_deleted".to_string(), serde_json::Value::Number(result.files_deleted.into()));
            output.insert("commands_removed".to_string(), serde_json::Value::Number(result.commands_removed.into()));
            output.insert("hooks_removed".to_string(), serde_json::Value::Number(result.hooks_removed.into()));

            let sections: Vec<serde_json::Value> = result.claude_sections_collapsed
                .iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect();
            output.insert("claude_sections_collapsed".to_string(), serde_json::Value::Array(sections));

            println!("{}", serde_json::to_string_pretty(&serde_json::Value::Object(output)).unwrap());

            eprintln!("vacate: success - {} files, {} commands, {} hooks removed",
                result.files_deleted, result.commands_removed, result.hooks_removed);
            0
        }
        Err(e) => {
            eprintln!("vacate: error: {}", e);
            1
        }
    }
}
