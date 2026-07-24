// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOK Main - Voce Viva Rust binary entry point
//!
//! Provides core VOK commands and MCP stdio server for jjx_* tool access.
//! JJK commands are accessed exclusively via MCP transport (no CLI dispatch).

use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

// RCG output discipline: all emission via vvc::vvco_Output — no direct println!/eprintln!
use vvc::{vvco_Output, vvco_err, vvco_out};

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

    /// Collect kit assets to staging directory
    #[command(name = "release_collect")]
    ReleaseCollect(ReleaseCollectArgs),

    /// Brand staging directory with brand
    #[command(name = "release_brand")]
    ReleaseBrand(ReleaseBrandArgs),

    /// Install kit assets from parcel to target repo
    #[command(name = "vvx_emplace")]
    VvxEmplace(EmplaceArgs),

    /// Remove kit assets from target repo
    #[command(name = "vvx_vacate")]
    VvxVacate(VacateArgs),

    /// Freshen CLAUDE.md managed sections from kit forge templates
    #[command(name = "vvx_freshen")]
    VvxFreshen(FreshenArgs),

    /// Diagnostic: derive and print this process's emblem window reference and
    /// file path from the environment (paneboard emblem feature). Writes nothing.
    #[command(name = "vvx_emblem_probe", hide = true)]
    VvxEmblemProbe,

    /// Start MCP stdio server for jjx_* tool access
    #[command(name = "mcp")]
    Mcp,

    /// JJ dispatch spine: plan, board, and launch a session through a door
    /// (invoked by the jjy_ trampolines via the kit's jjw tabtargets)
    #[command(name = "jjx_dispatch")]
    JjxDispatch(DispatchArgs),

    /// Cashier a derelict JJ blotter lock-holder: sight every lock and report,
    /// or (with --break) clear a stranded one. Operator-deliberate — the confirm
    /// gate lives in the tabtarget door, and this verb is deliberately NOT an
    /// MCP command (JJSVD jjdd_cashier).
    #[command(name = "jjx_cashier")]
    JjxCashier(CashierArgs),

    /// Found the studbook from nothing (JJSAS Founding-and-cutover): compose the
    /// live-state import and seed both tenants (gallops + pedigrees) in one
    /// genesis commit against the studbook's remote. Operator ceremony door,
    /// deliberately NOT an MCP command — fronted by the jjw-bf tabtarget.
    #[command(name = "jjx_found")]
    JjxFound(FoundArgs),

    /// External subcommands (delegated to kit CLIs)
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

/// Arguments for jjx_cashier — the door's two modes. Sight-and-report is the
/// default and is read-only; `--break` mutates and rides the door's gate.
#[derive(clap::Args, Debug)]
struct CashierArgs {
    /// The operator's invocation directory (cwd elects the clone)
    #[arg(long)]
    cwd: PathBuf,

    /// Break the sighted locks, rather than only reporting them
    #[arg(long = "break")]
    do_break: bool,
}

/// Arguments for jjx_found — the studbook founding door. The operator's
/// invocation directory elects the hippodrome being founded: its origin is
/// identified for the sire pedigree's address (the same canonicalization
/// dispatch derives), its live gallops seeds the import, and its parent serves
/// as the infield root the studbook lands beside.
#[derive(clap::Args, Debug)]
struct FoundArgs {
    /// The operator's invocation directory (elects the hippodrome to found against)
    #[arg(long)]
    cwd: PathBuf,

    /// The sire's trunk — the pedigree's line of work (a durable record value)
    #[arg(long, default_value = "main")]
    trunk: String,

    /// Resolve and print the founding plan, then stop before touching git
    #[arg(long)]
    dry_run: bool,
}

/// Arguments for jjx_dispatch — the spine's CLI face. The door captures the
/// operator's invocation directory (cwd elects the clone) before BUK dispatch
/// self-anchors, and hands it through explicitly.
#[derive(clap::Args, Debug)]
struct DispatchArgs {
    /// Which door: saddle or lunge
    #[arg(long)]
    door: String,

    /// Dispatch target: a coronet or firemark (glyphs tolerated)
    #[arg(long)]
    target: String,

    /// The operator's invocation directory (captured once at the trampoline)
    #[arg(long)]
    cwd: PathBuf,

    /// The kit repo root (source of vvx for the session-scoped MCP config)
    #[arg(long)]
    kit_root: PathBuf,

    /// Print the resolved plan and stop before boarding and launch
    #[arg(long)]
    dry_run: bool,
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

/// Arguments for vvx_freshen command
#[derive(clap::Args, Debug)]
struct FreshenArgs {
    /// Path to burc.env file (resolves project_root and tools_dir)
    #[arg(long)]
    burc: PathBuf,
}

/// Environment gate — panics if Claude Code environment is misconfigured.
/// Called unconditionally before any argument parsing or library use.
fn vosr_init() {
    if std::env::var("CLAUDE_CODE_DISABLE_AUTO_MEMORY").as_deref() != Ok("1") {
        panic!("CLAUDE_CODE_DISABLE_AUTO_MEMORY must be 1");
    }
    if std::env::var("MAX_MCP_OUTPUT_TOKENS").as_deref() != Ok("200000") {
        panic!("MAX_MCP_OUTPUT_TOKENS must be 200000");
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    vosr_init();
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Some(Commands::Guard(args)) => run_guard(args),
        Some(Commands::VvxCommit(args)) => run_commit(args),
        Some(Commands::VvxPush(args)) => run_push(args),
        Some(Commands::VvxUnlock) => run_unlock(),
        Some(Commands::ReleaseCollect(args)) => run_release_collect(args),
        Some(Commands::ReleaseBrand(args)) => run_release_brand(args),
        Some(Commands::VvxEmplace(args)) => run_emplace(args),
        Some(Commands::VvxVacate(args)) => run_vacate(args),
        Some(Commands::VvxFreshen(args)) => run_freshen(args),
        Some(Commands::VvxEmblemProbe) => run_emblem_probe(),
        Some(Commands::Mcp) => run_mcp().await,
        Some(Commands::JjxDispatch(args)) => run_dispatch(args),
        Some(Commands::JjxCashier(args)) => run_cashier(args),
        Some(Commands::JjxFound(args)) => run_found(args),
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
    let mut out = vvco_Output::console();
    vvc::guard(&vvc_args, None, &mut out)
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
    let mut out = vvco_Output::console();
    vvc::commit(&vvc_args, &mut out)
}

/// Diagnostic: derive and print this process's emblem target — the iTerm window
/// reference and the emblem file path vvx would write — from the environment.
/// Reads only the environment and writes nothing. Confirms that
/// ITERM_SESSION_ID actually inherits into a running vvx (the same Claude Code
/// child chain the MCP server rides), rather than only into a manual proof.
fn run_emblem_probe() -> i32 {
    let mut out = vvco_Output::console();
    let raw = std::env::var("ITERM_SESSION_ID");
    vvco_out!(
        out,
        "ITERM_SESSION_ID: {}",
        match &raw {
            Ok(v) => v.as_str(),
            Err(_) => "(unset — not under iTerm)",
        }
    );

    #[cfg(feature = "jjk")]
    {
        match jjk::jjrm_mcp::jjrm_iterm_window_ref() {
            Some(window_ref) => {
                vvco_out!(out, "window reference: {}", window_ref);
                match jjk::jjrm_mcp::jjrm_emblem_root() {
                    Some(root) => {
                        let file = jjk::jjrm_mcp::jjrm_emblem_path(&root, &window_ref);
                        vvco_out!(out, "emblem root:      {}", root.display());
                        vvco_out!(out, "emblem file:      {}", file.display());
                    }
                    None => vvco_out!(out, "emblem root:      (skipped — HOME unset)"),
                }
            }
            None => vvco_out!(out, "window reference: (skipped — no usable iTerm session)"),
        }
        0
    }
    #[cfg(not(feature = "jjk"))]
    {
        vvco_err!(out, "vvx_emblem_probe: error: jjk feature not enabled");
        1
    }
}

/// Run MCP stdio server
async fn run_mcp() -> i32 {
    let mut out = vvco_Output::console();
    #[cfg(feature = "jjk")]
    {
        match jjk::jjrm_mcp::jjrm_serve_stdio().await {
            Ok(()) => 0,
            Err(e) => {
                vvco_err!(out, "mcp: error: {}", e);
                1
            }
        }
    }
    #[cfg(not(feature = "jjk"))]
    {
        vvco_err!(out, "mcp: error: jjk feature not enabled");
        1
    }
}

/// Run the JJ dispatch spine (jjy_ door path). The spine resolves and composes
/// but does not launch: it returns the report and, when a session is ready, the
/// composed command. We print the report here FIRST, then hand the terminal to
/// the session, so the door's whole report precedes the session it introduces
/// (JJSVD "Report precedes launch").
fn run_dispatch(args: DispatchArgs) -> i32 {
    let mut out = vvco_Output::console();
    #[cfg(feature = "jjk")]
    {
        use jjk::jjrds_spine::jjrds_Outcome;
        let door = match args.door.as_str() {
            "saddle" => jjk::jjrds_spine::jjrds_Door::Saddle,
            "lunge" => jjk::jjrds_spine::jjrds_Door::Lunge,
            other => {
                vvco_err!(out, "jjx_dispatch: error: unknown door '{}' — saddle or lunge", other);
                return 1;
            }
        };
        let (outcome, text) = jjk::jjrds_spine::jjrds_run(door, &args.target, &args.cwd, &args.kit_root, args.dry_run);
        if !text.is_empty() {
            vvco_out!(out, "{}", text.trim_end());
        }
        match outcome {
            jjrds_Outcome::Done(code) => code,
            jjrds_Outcome::Launch { mut cmd, billet_root, trunk } => {
                let code = match cmd.status() {
                    Ok(status) => status.code().unwrap_or(1),
                    Err(e) => {
                        vvco_err!(out, "jjx_dispatch: stirrup failed to launch claude: {}", e);
                        return 1;
                    }
                };
                // The stile's trailing step: the session has returned and this
                // driver is still standing, outside the billet, as its parent —
                // exactly the geometry the trailing step rides (JJSVD "The
                // stile"). The session's own exit code is the dispatch's; the
                // step trouble-reports and never masks it.
                let report = jjk::jjrds_spine::jjrds_trailing_step(&jjk::jjrfg_plaingit::jjrfg_PlainGit, &billet_root, &trunk);
                if !report.is_empty() {
                    vvco_out!(out, "{}", report.trim_end());
                }
                code
            }
        }
    }
    #[cfg(not(feature = "jjk"))]
    {
        let _ = args;
        vvco_err!(out, "jjx_dispatch: error: jjk feature not enabled");
        1
    }
}

/// Run the cashier door (JJSVD `jjdd_cashier`). Sight-and-report always runs and
/// always exits 0 — a read that reports "no lock held" is a success, not a
/// failure. `--break` then clears each held lock through the break sequence,
/// which sights afresh and plucks against exactly what it sighted.
fn run_cashier(args: CashierArgs) -> i32 {
    let mut out = vvco_Output::console();
    #[cfg(feature = "jjk")]
    {
        use jjk::jjrdc_cashier::{jjrdc_any_held, jjrdc_cashier_store, jjrdc_held_stores, jjrdc_infield_root, jjrdc_report, jjrdc_sight};

        let infield_root = match jjrdc_infield_root(&args.cwd) {
            Ok(root) => root,
            Err(rejection) => {
                vvco_err!(out, "jjx_cashier: {}", rejection);
                return 1;
            }
        };

        let sightings = jjrdc_sight(&infield_root);

        // The report is the sight mode's whole product. In break mode the door
        // has ALREADY shown it — it is what the operator answered the gate on —
        // so re-rendering it here would print the same ten lines twice around a
        // single confirmation, burying the one line that says what was cleared.
        if !args.do_break {
            vvco_out!(out, "{}", jjrdc_report(&sightings));
            return 0;
        }
        if !jjrdc_any_held(&sightings) {
            vvco_out!(out, "\nNothing to cashier — no lock is held.");
            return 0;
        }

        let mut code = 0;
        for store in jjrdc_held_stores(&sightings) {
            match jjrdc_cashier_store(&infield_root, &store) {
                Ok(Some(cleared)) => vvco_out!(out, "\ncashiered the {} lock, held by: {}", store, cleared),
                Ok(None) => vvco_out!(out, "\nthe {} lock was already free — nothing to cashier", store),
                Err(rejection) => {
                    // A LockBroken here means the guidon changed between the
                    // report and the pluck — someone else's break, or a fresh
                    // stake. The lock the operator saw is not the lock now
                    // flying, so this refuses rather than clearing a stranger's.
                    vvco_err!(out, "\njjx_cashier: {} not cashiered: {}", store, rejection);
                    code = 1;
                }
            }
        }
        code
    }
    #[cfg(not(feature = "jjk"))]
    {
        let _ = args;
        vvco_err!(out, "jjx_cashier: error: jjk feature not enabled");
        1
    }
}

/// Run the studbook founding door (JJSAS Founding-and-cutover). Elects the
/// hippodrome from the invocation cwd, derives the sire pedigree's address from
/// that hippodrome's own identity (the same canonicalized origin dispatch later
/// derives — seed and lookup cannot drift), reads its live gallops for the
/// import, and composes the single deterministic found+import+seed act. The real
/// found is the cutover ceremony's act; `--dry-run` resolves and prints the plan
/// without touching git.
fn run_found(args: FoundArgs) -> i32 {
    let mut out = vvco_Output::console();
    #[cfg(feature = "jjk")]
    {
        use jjk::jjrc_core::JJRC_DEFAULT_GALLOPS_PATH;
        use jjk::jjrdc_cashier::jjrdc_infield_root;
        use jjk::jjrds_spine::JJRDS_KIND_PLAIN_GIT;
        use jjk::jjrfg_plaingit::jjrfg_PlainGit;
        use jjk::jjrfr_farrier::{jjrfr_FarrierCore, jjrfr_Seat};
        use jjk::jjrvb_blotter::{jjdb_founding_import, jjdb_found_studbook, jjdb_studbook_config, jjdb_SireSeed};

        // Elect the hippodrome and derive the sire address from its own identity
        // — the same canonicalized origin dispatch derives, so the seeded
        // address is byte-identical to the key the lookup will match against.
        let identity = match jjrfg_PlainGit.jjrfr_identify(&args.cwd) {
            Ok(id) => id,
            Err(rejection) => {
                vvco_err!(out, "jjx_found: {}", rejection);
                return 1;
            }
        };
        let address = match identity.upstream_key.clone() {
            Some(addr) => addr,
            None => {
                vvco_err!(out, "jjx_found: the hippodrome at {} has no origin remote — no sire address to seed", identity.root.display());
                return 1;
            }
        };
        // The primary hippodrome root: a billet worktree (Partition seat) climbs
        // to its primary, so the live gallops and the infield resolve from one
        // consistent root rather than the partition's own checkout.
        let hippodrome_root = match &identity.seat {
            jjrfr_Seat::Primary => identity.root.clone(),
            jjrfr_Seat::Partition { primary_root } => primary_root.clone(),
        };
        let infield_root = match jjrdc_infield_root(&args.cwd) {
            Ok(root) => root,
            Err(rejection) => {
                vvco_err!(out, "jjx_found: {}", rejection);
                return 1;
            }
        };
        let config = jjdb_studbook_config(&infield_root);

        // The live in-repo gallops seeds the import (read-only — jjdr_hark never
        // re-saves the source store).
        let live_path = hippodrome_root.join(JJRC_DEFAULT_GALLOPS_PATH);
        let live_bytes = match std::fs::read(&live_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                vvco_err!(out, "jjx_found: could not read the live gallops at {}: {}", live_path.display(), e);
                return 1;
            }
        };
        let live = match jjk::jjri_io::jjdr_hark(&live_bytes) {
            Ok(validated) => validated,
            Err(e) => {
                vvco_err!(out, "jjx_found: the live gallops at {} does not validate: {}", live_path.display(), e);
                return 1;
            }
        };

        let sire = jjdb_SireSeed {
            kind: JJRDS_KIND_PLAIN_GIT.to_string(),
            address,
            trunk: args.trunk.clone(),
        };

        if args.dry_run {
            // Run the pure import so the confirmed plan matches the act — the
            // seeded heat count is post-filter, not the raw live count.
            let (seeded, behind) = match jjdb_founding_import(live.inner(), None) {
                Ok(seed) => {
                    let seeded = seed.heats.len();
                    (seeded, live.inner().heats.len() - seeded)
                }
                Err(e) => {
                    vvco_err!(out, "jjx_found: {}", e);
                    return 1;
                }
            };
            vvco_out!(out, "jjx_found (dry run): would found the studbook, then stop before touching git");
            vvco_out!(out, "  infield root:  {}", infield_root.display());
            vvco_out!(out, "  studbook root: {}{}", config.local_root.display(),
                if config.local_root.exists() { "  [ALREADY STANDS — founding will refuse; recreate-clean first]" } else { "" });
            vvco_out!(out, "  remote:        {}", config.remote_url);
            vvco_out!(out, "  sire address:  {} (kind {}, trunk {})", sire.address, sire.kind, sire.trunk);
            vvco_out!(out, "  live gallops:  {} heat(s) seed, {} retired left behind ({})", seeded, behind, live_path.display());
            return 0;
        }

        match jjdb_found_studbook(&config, live.inner(), &sire) {
            Ok(sha) => {
                vvco_out!(out, "jjx_found: founded the studbook at {} ({})", config.local_root.display(), sha);
                0
            }
            Err(e) => {
                vvco_err!(out, "jjx_found: {}", e);
                1
            }
        }
    }
    #[cfg(not(feature = "jjk"))]
    {
        let _ = args;
        vvco_err!(out, "jjx_found: error: jjk feature not enabled");
        1
    }
}

/// Dispatch external subcommands to appropriate kit CLIs
async fn dispatch_external(args: Vec<OsString>) -> i32 {
    let mut out = vvco_Output::console();
    if args.is_empty() {
        vvco_err!(out, "vvx: error: no subcommand provided");
        return 1;
    }

    let cmd_name = args[0].to_string_lossy();

    // Unknown external subcommand
    vvco_err!(out, "vvx: error: unknown command '{}'", cmd_name);
    vvco_err!(out, "Run 'vvx --help' for available commands");
    1
}

/// Lock reference path for push operations (same as commit to prevent concurrent ops)
const LOCK_REF: &str = "refs/vvg/locks/vvx";

/// Force-break a stuck lock
fn run_unlock() -> i32 {
    use std::process::Command;

    let mut out = vvco_Output::console();

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
                    vvco_err!(out, "unlock: lock broken successfully");
                    0
                }
                Ok(output) => {
                    vvco_err!(out, "unlock: error: failed to delete lock: {}",
                        String::from_utf8_lossy(&output.stderr));
                    1
                }
                Err(e) => {
                    vvco_err!(out, "unlock: error: {}", e);
                    1
                }
            }
        }
        Ok(_) => {
            // Lock doesn't exist
            vvco_err!(out, "unlock: no lock held");
            0
        }
        Err(e) => {
            vvco_err!(out, "unlock: error checking lock: {}", e);
            1
        }
    }
}

fn run_push(args: PushArgs) -> i32 {
    use std::process::Command;

    let mut out = vvco_Output::console();
    vvco_err!(out, "push: acquiring lock...");
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
            vvco_err!(out, "push: error: Another operation in progress - lock held");
            return 1;
        }
    }

    let result = run_push_workflow(&args);

    let _ = Command::new("git")
        .args(["update-ref", "-d", LOCK_REF])
        .output();

    match result {
        Ok(()) => {
            vvco_err!(out, "push: success");
            0
        }
        Err(e) => {
            vvco_err!(out, "push: error: {}", e);
            1
        }
    }
}

fn run_push_workflow(args: &PushArgs) -> Result<(), String> {
    use std::process::Command;

    let mut out = vvco_Output::console();

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

    vvco_err!(out, "push: pushing {} to {}", branch, args.remote);

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
    let mut out = vvco_Output::console();
    vvco_err!(out, "release_collect: collecting assets...");
    vvco_err!(out, "  staging: {}", args.staging.display());
    vvco_err!(out, "  tools_dir: {}", args.tools_dir.display());
    vvco_err!(out, "  install_script: {}", args.install_script.display());

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

            vvco_out!(out, "{}", serde_json::to_string_pretty(&serde_json::Value::Object(output)).unwrap());
            vvco_err!(out, "release_collect: success - {} files collected", result.total_files);
            0
        }
        Err(e) => {
            vvco_err!(out, "release_collect: error: {}", e);
            1
        }
    }
}

/// Run release_brand command
fn run_release_brand(args: ReleaseBrandArgs) -> i32 {
    let mut out = vvco_Output::console();
    vvco_err!(out, "release_brand: branding staging...");
    vvco_err!(out, "  staging: {}", args.staging.display());
    vvco_err!(out, "  registry: {}", args.registry.display());
    vvco_err!(out, "  commit: {}", args.commit);

    // Parse comma-separated kit list
    let managed_kits: Vec<String> = args.managed_kits
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    match vof::vofr_brand(&args.staging, &args.registry, &args.commit, &managed_kits) {
        Ok(result) => {
            // Output brand for bash to use in tarball name
            vvco_out!(out, "{}", result.brand);
            if result.is_new {
                vvco_err!(out, "release_brand: allocated new brand {}", result.brand);
            } else {
                vvco_err!(out, "release_brand: reusing existing brand {}", result.brand);
            }
            vvco_err!(out, "release_brand: super-SHA: {}", result.super_sha);
            0
        }
        Err(e) => {
            vvco_err!(out, "release_brand: error: {}", e);
            1
        }
    }
}

/// Run vvx_emplace command
fn run_emplace(args: EmplaceArgs) -> i32 {
    let mut out = vvco_Output::console();
    vvco_err!(out, "emplace: installing kit assets...");
    vvco_err!(out, "  parcel: {}", args.parcel.display());
    vvco_err!(out, "  burc: {}", args.burc.display());

    let emplace_args = vof::vofe_EmplaceArgs {
        parcel_dir: args.parcel,
        burc_path: args.burc,
    };

    match vof::vofe_emplace(&emplace_args) {
        Ok(result) => {
            // Output JSON summary
            let mut output = serde_json::Map::new();
            output.insert("brand".to_string(), serde_json::Value::Number(result.brand.into()));

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

            vvco_out!(out, "{}", serde_json::to_string_pretty(&serde_json::Value::Object(output)).unwrap());

            vvco_err!(out, "emplace: success - {} files, {} commands, {} hooks",
                result.files_copied, result.commands_routed, result.hooks_routed);
            0
        }
        Err(e) => {
            vvco_err!(out, "emplace: error: {}", e);
            1
        }
    }
}

/// Run vvx_vacate command
fn run_vacate(args: VacateArgs) -> i32 {
    let mut out = vvco_Output::console();
    vvco_err!(out, "vacate: removing kit assets...");
    vvco_err!(out, "  burc: {}", args.burc.display());

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

            vvco_out!(out, "{}", serde_json::to_string_pretty(&serde_json::Value::Object(output)).unwrap());

            vvco_err!(out, "vacate: success - {} files, {} commands, {} hooks removed",
                result.files_deleted, result.commands_removed, result.hooks_removed);
            0
        }
        Err(e) => {
            vvco_err!(out, "vacate: error: {}", e);
            1
        }
    }
}

/// Run vvx_freshen command
fn run_freshen(args: FreshenArgs) -> i32 {
    let mut out = vvco_Output::console();
    vvco_err!(out, "freshen: updating CLAUDE.md from forge templates...");
    vvco_err!(out, "  burc: {}", args.burc.display());

    match vof::vofe_freshen_forge(&args.burc) {
        Ok(result) => {
            let total = result.updated.len() + result.expanded.len() + result.appended.len();
            vvco_err!(out, "freshen: success - {} sections processed ({} updated, {} expanded, {} appended)",
                total, result.updated.len(), result.expanded.len(), result.appended.len());
            0
        }
        Err(e) => {
            vvco_err!(out, "freshen: error: {}", e);
            1
        }
    }
}
