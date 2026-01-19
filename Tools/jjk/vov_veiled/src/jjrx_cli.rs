// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK CLI - Command line interface for Job Jockey Kit
//!
//! This module owns all jjx_* command definitions and dispatch logic.
//! VOK delegates unknown subcommands here via external_subcommand pattern.
//!
//! All commands are handled entirely here, including notch/chalk which
//! use the vvc crate for commit operations.

use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::jjrf_favor::{jjrf_Coronet as Coronet, jjrf_Firemark as Firemark};
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_PaceState as PaceState, jjrg_read_stdin as read_stdin, jjrg_read_stdin_optional as read_stdin_optional};
use crate::jjrn_notch::{jjrn_ChalkMarker as ChalkMarker, jjrn_HeatAction as HeatAction, jjrn_format_notch_prefix as format_notch_prefix, jjrn_format_chalk_message as format_chalk_message, jjrn_format_heat_message as format_heat_message, jjrn_format_heat_discussion as format_heat_discussion};

/// JJK subcommands - all jjx_* commands
#[derive(Parser)]
#[command(name = "jjx")]
#[command(about = "Job Jockey Kit commands")]
pub enum jjrx_JjxCommands {
    /// JJ-aware commit with heat/pace context prefix
    #[command(name = "jjx_notch")]
    Notch(jjrx_NotchArgs),

    /// Empty commit marking a steeplechase event
    #[command(name = "jjx_chalk")]
    Chalk(jjrx_ChalkArgs),

    /// Parse git history for steeplechase entries
    #[command(name = "jjx_rein")]
    Rein(zjjrx_ReinArgs),

    /// Validate Gallops JSON schema
    #[command(name = "jjx_validate")]
    Validate(zjjrx_ValidateArgs),

    /// List all Heats with summary information
    #[command(name = "jjx_muster")]
    Muster(zjjrx_MusterArgs),

    /// Return context needed to saddle up on a Heat
    #[command(name = "jjx_saddle")]
    Saddle(zjjrx_SaddleArgs),

    /// Display comprehensive Heat status for project review
    #[command(name = "jjx_parade")]
    Parade(zjjrx_ParadeArgs),

    /// Extract complete Heat data for archival trophy
    #[command(name = "jjx_retire")]
    Retire(zjjrx_RetireArgs),

    /// Create a new Heat with empty Pace structure
    #[command(name = "jjx_nominate")]
    Nominate(zjjrx_NominateArgs),

    /// Add a new Pace to a Heat
    #[command(name = "jjx_slate")]
    Slate(zjjrx_SlateArgs),

    /// Reorder Paces within a Heat
    #[command(name = "jjx_rail")]
    Rail(zjjrx_RailArgs),

    /// Add a new Tack to a Pace
    #[command(name = "jjx_tally")]
    Tally(zjjrx_TallyArgs),

    /// Move a Pace from one Heat to another
    #[command(name = "jjx_draft")]
    Draft(zjjrx_DraftArgs),
}

/// Arguments for jjx_notch command
#[derive(clap::Args, Debug)]
pub struct jjrx_NotchArgs {
    /// Pace identity (Coronet) - embeds parent Heat
    pub coronet: String,

    /// Size limit in bytes (overrides default 50KB guard)
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Arguments for jjx_chalk command
#[derive(clap::Args, Debug)]
pub struct jjrx_ChalkArgs {
    /// Identity: Coronet (pace-level) or Firemark (heat-level discussion only)
    pub identity: String,

    /// Marker type: A(pproach), W(rap), F(ly), d(iscussion)
    #[arg(long)]
    pub marker: String,

    /// Marker description text
    #[arg(long)]
    pub description: String,
}

/// Arguments for jjx_rein command
#[derive(clap::Args, Debug)]
struct zjjrx_ReinArgs {
    /// Target Heat identity (Firemark)
    firemark: String,

    /// Maximum entries to return
    #[arg(long, default_value = "50")]
    limit: usize,
}

/// Arguments for jjx_validate command
#[derive(clap::Args, Debug)]
struct zjjrx_ValidateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,
}

/// Arguments for jjx_muster command
#[derive(clap::Args, Debug)]
struct zjjrx_MusterArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Filter by Heat status (current or retired)
    #[arg(long)]
    status: Option<String>,
}

/// Arguments for jjx_saddle command
#[derive(clap::Args, Debug)]
struct zjjrx_SaddleArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,
}

/// Output format modes for jjx_parade
#[derive(clap::ValueEnum, Clone, Copy, Debug, Default)]
enum zjjrx_ParadeFormatArg {
    /// One line per pace: [state] silks (coronet)
    Overview,
    /// Numbered list: N. [state] silks (coronet)
    Order,
    /// Full tack text for one pace (requires --pace)
    Detail,
    /// Paddock + all paces with tack text
    #[default]
    Full,
}

/// Arguments for jjx_parade command
#[derive(clap::Args, Debug)]
struct zjjrx_ParadeArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,

    /// Output format mode
    #[arg(long, value_enum, default_value = "full")]
    format: zjjrx_ParadeFormatArg,

    /// Target Pace coronet (required for --format detail)
    #[arg(long)]
    pace: Option<String>,

    /// Show only remaining paces (exclude complete/abandoned)
    #[arg(long)]
    remaining: bool,
}

/// Arguments for jjx_retire command
#[derive(clap::Args, Debug)]
struct zjjrx_RetireArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,

    /// Execute the retire (write trophy, remove from gallops, delete paddock, commit)
    #[arg(long)]
    execute: bool,
}

/// Arguments for jjx_nominate command
#[derive(clap::Args, Debug)]
struct zjjrx_NominateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Kebab-case display name for the Heat
    #[arg(long, short = 's')]
    silks: String,

    /// Creation date in YYMMDD format
    #[arg(long, short = 'c')]
    created: String,
}

/// Arguments for jjx_slate command
#[derive(clap::Args, Debug)]
struct zjjrx_SlateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,

    /// Kebab-case display name for the Pace
    #[arg(long, short = 's')]
    silks: String,

    /// Insert before specified Coronet
    #[arg(long, conflicts_with_all = ["after", "first"])]
    before: Option<String>,

    /// Insert after specified Coronet
    #[arg(long, conflicts_with_all = ["before", "first"])]
    after: Option<String>,

    /// Insert at beginning of Heat
    #[arg(long, conflicts_with_all = ["before", "after"])]
    first: bool,
}

/// Arguments for jjx_rail command
#[derive(clap::Args, Debug)]
struct zjjrx_RailArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,

    /// New order of Coronets (space-separated or JSON array) - order mode
    #[arg(trailing_var_arg = true)]
    order: Vec<String>,

    // Move mode arguments

    /// Coronet to relocate within order - triggers move mode
    #[arg(long, short = 'm')]
    r#move: Option<String>,

    /// Move before specified Coronet
    #[arg(long, conflicts_with_all = ["after", "first", "last"])]
    before: Option<String>,

    /// Move after specified Coronet
    #[arg(long, conflicts_with_all = ["before", "first", "last"])]
    after: Option<String>,

    /// Move to beginning of order
    #[arg(long, conflicts_with_all = ["before", "after", "last"])]
    first: bool,

    /// Move to end of order
    #[arg(long, conflicts_with_all = ["before", "after", "first"])]
    last: bool,
}

/// Arguments for jjx_tally command
#[derive(clap::Args, Debug)]
struct zjjrx_TallyArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Pace identity (Coronet)
    coronet: String,

    /// Target state (rough, bridled, complete, abandoned)
    #[arg(long)]
    state: Option<String>,

    /// Execution guidance (required if state is bridled)
    #[arg(long, short = 'd')]
    direction: Option<String>,

    /// Kebab-case display name (if provided, new Tack uses this value; otherwise inherits)
    #[arg(long, short = 's')]
    silks: Option<String>,
}

/// Arguments for jjx_draft command
#[derive(clap::Args, Debug)]
struct zjjrx_DraftArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Pace identity to move (Coronet)
    coronet: String,

    /// Destination Heat identity (Firemark)
    #[arg(long)]
    to: String,

    /// Insert before specified Coronet in destination
    #[arg(long, conflicts_with_all = ["after", "first"])]
    before: Option<String>,

    /// Insert after specified Coronet in destination
    #[arg(long, conflicts_with_all = ["before", "first"])]
    after: Option<String>,

    /// Insert at beginning of destination Heat
    #[arg(long, conflicts_with_all = ["before", "after"])]
    first: bool,
}

// ============================================================================
// Public dispatch API
// ============================================================================

/// Dispatch JJK commands from raw arguments.
///
/// Called by VOK when it receives an external subcommand starting with "jjx_".
/// The first element of `args` should be the subcommand name (e.g., "jjx_muster").
///
/// Returns exit code (0 for success, non-zero for failure).
pub fn jjrx_dispatch(args: &[OsString]) -> i32 {
    // Prepend synthetic binary name for clap parsing.
    // Clap expects args[0] to be the binary name (used in help text).
    // VOK passes ["jjx_muster", ...] so we prepend "jjx" to get ["jjx", "jjx_muster", ...].
    let mut full_args = vec![OsString::from("jjx")];
    full_args.extend(args.iter().cloned());

    // Parse the subcommand and arguments
    let parsed = match jjrx_JjxCommands::try_parse_from(&full_args) {
        Ok(cmd) => cmd,
        Err(e) => {
            // Let clap handle help/version/error display
            e.print().ok();
            return if e.kind() == clap::error::ErrorKind::DisplayHelp
                   || e.kind() == clap::error::ErrorKind::DisplayVersion {
                0
            } else {
                1
            };
        }
    };

    match parsed {
        jjrx_JjxCommands::Notch(args) => zjjrx_run_notch(args),
        jjrx_JjxCommands::Chalk(args) => zjjrx_run_chalk(args),
        jjrx_JjxCommands::Rein(args) => zjjrx_run_rein(args),
        jjrx_JjxCommands::Validate(args) => zjjrx_run_validate(args),
        jjrx_JjxCommands::Muster(args) => zjjrx_run_muster(args),
        jjrx_JjxCommands::Saddle(args) => zjjrx_run_saddle(args),
        jjrx_JjxCommands::Parade(args) => zjjrx_run_parade(args),
        jjrx_JjxCommands::Retire(args) => zjjrx_run_retire(args),
        jjrx_JjxCommands::Nominate(args) => zjjrx_run_nominate(args),
        jjrx_JjxCommands::Slate(args) => zjjrx_run_slate(args),
        jjrx_JjxCommands::Rail(args) => zjjrx_run_rail(args),
        jjrx_JjxCommands::Tally(args) => zjjrx_run_tally(args),
        jjrx_JjxCommands::Draft(args) => zjjrx_run_draft(args),
    }
}

/// Check if a command name is a JJK command
pub fn jjrx_is_jjk_command(name: &str) -> bool {
    name.starts_with("jjx_")
}

// ============================================================================
// Notch/Chalk implementations (use vvc for commits)
// ============================================================================

fn zjjrx_run_notch(args: jjrx_NotchArgs) -> i32 {
    let coronet = match Coronet::jjrf_parse(&args.coronet) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("jjx_notch: error: {}", e);
            return 1;
        }
    };

    let prefix = format_notch_prefix(&coronet);

    let commit_args = vvc::vvcc_CommitArgs {
        prefix: Some(prefix),
        message: None,
        allow_empty: false,
        no_stage: false,
        size_limit: args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT),
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    vvc::commit(&commit_args)
}

fn zjjrx_run_chalk(args: jjrx_ChalkArgs) -> i32 {
    let marker = match ChalkMarker::jjrn_parse(&args.marker) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("jjx_chalk: error: {}", e);
            return 1;
        }
    };

    // Try parsing as Coronet first (5 base64 chars), then as Firemark (2 base64 chars)
    let identity = args.identity.strip_prefix('₢').or_else(|| args.identity.strip_prefix('₣')).unwrap_or(&args.identity);

    let message = if identity.len() == 5 {
        // Coronet - pace-level chalk
        let coronet = match Coronet::jjrf_parse(&args.identity) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("jjx_chalk: error: {}", e);
                return 1;
            }
        };
        format_chalk_message(&coronet, marker, &args.description)
    } else if identity.len() == 2 {
        // Firemark - heat-level (discussion only)
        if marker.jjrn_requires_pace() {
            eprintln!("jjx_chalk: error: {} marker requires a Coronet (pace identity), not a Firemark", marker.jjrn_as_str());
            return 1;
        }
        let firemark = match Firemark::jjrf_parse(&args.identity) {
            Ok(fm) => fm,
            Err(e) => {
                eprintln!("jjx_chalk: error: {}", e);
                return 1;
            }
        };
        format_heat_discussion(&firemark, &args.description)
    } else {
        eprintln!("jjx_chalk: error: identity must be Coronet (5 chars) or Firemark (2 chars), got {} chars", identity.len());
        return 1;
    };

    let commit_args = vvc::vvcc_CommitArgs {
        prefix: None,
        message: Some(message),
        allow_empty: true,
        no_stage: true,
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    vvc::commit(&commit_args)
}

// ============================================================================
// Command implementations (pure JJK operations)
// ============================================================================

fn zjjrx_run_rein(args: zjjrx_ReinArgs) -> i32 {
    use crate::jjrs_steeplechase::{jjrs_ReinArgs as LibReinArgs, jjrs_run as run};

    let rein_args = LibReinArgs {
        firemark: args.firemark,
        limit: args.limit,
    };

    run(rein_args)
}

fn zjjrx_run_validate(args: zjjrx_ValidateArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_validate: error loading Gallops: {}", e);
            return 1;
        }
    };

    match gallops.jjrg_validate() {
        Ok(()) => {
            println!("Gallops validation passed");
            0
        }
        Err(errors) => {
            eprintln!("jjx_validate: validation failed with {} error(s):", errors.len());
            for error in errors {
                eprintln!("  - {}", error);
            }
            1
        }
    }
}

fn zjjrx_run_muster(args: zjjrx_MusterArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_HeatStatus as HeatStatus;
    use crate::jjrq_query::{jjrq_MusterArgs as LibMusterArgs, jjrq_run_muster as lib_run_muster};

    let status = match &args.status {
        Some(s) => match s.to_lowercase().as_str() {
            "current" => Some(HeatStatus::Current),
            "retired" => Some(HeatStatus::Retired),
            _ => {
                eprintln!("jjx_muster: error: invalid status '{}', must be 'current' or 'retired'", s);
                return 1;
            }
        },
        None => None,
    };

    let muster_args = LibMusterArgs {
        file: args.file,
        status,
    };

    lib_run_muster(muster_args)
}

fn zjjrx_run_saddle(args: zjjrx_SaddleArgs) -> i32 {
    use crate::jjrq_query::{jjrq_SaddleArgs as LibSaddleArgs, jjrq_run_saddle as lib_run_saddle};

    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_saddle: error: {}", e);
            return 1;
        }
    };

    let saddle_args = LibSaddleArgs {
        file: args.file,
        firemark,
    };

    lib_run_saddle(saddle_args)
}

fn zjjrx_run_parade(args: zjjrx_ParadeArgs) -> i32 {
    use crate::jjrq_query::{jjrq_ParadeArgs as LibParadeArgs, jjrq_ParadeFormat as ParadeFormat, jjrq_run_parade as lib_run_parade};

    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_parade: error: {}", e);
            return 1;
        }
    };

    let format = match args.format {
        zjjrx_ParadeFormatArg::Overview => ParadeFormat::Overview,
        zjjrx_ParadeFormatArg::Order => ParadeFormat::Order,
        zjjrx_ParadeFormatArg::Detail => ParadeFormat::Detail,
        zjjrx_ParadeFormatArg::Full => ParadeFormat::Full,
    };

    let parade_args = LibParadeArgs {
        file: args.file,
        firemark,
        format,
        pace: args.pace,
        remaining: args.remaining,
    };

    lib_run_parade(parade_args)
}

fn zjjrx_run_retire(args: zjjrx_RetireArgs) -> i32 {
    use crate::jjrc_core::jjrc_timestamp_date;
    use crate::jjrg_gallops::jjrg_RetireArgs as LibRetireArgs;
    use crate::jjrs_steeplechase::{jjrs_ReinArgs as ReinArgs, jjrs_get_entries as get_entries};
    use std::path::Path;

    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    // Load gallops
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_retire: error loading Gallops: {}", e);
            return 1;
        }
    };

    // Get steeplechase entries (filtered by firemark identity)
    let rein_args = ReinArgs {
        firemark: args.firemark.clone(),
        limit: 1000,
    };
    let steeplechase = match get_entries(&rein_args) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("jjx_retire: warning: could not get steeplechase: {}", e);
            Vec::new()
        }
    };

    // Compute base path from gallops file
    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    // If --execute not specified, output trophy markdown preview
    if !args.execute {
        // Read paddock content
        let firemark_key = firemark.jjrf_display();
        let heat = match gallops.heats.get(&firemark_key) {
            Some(h) => h,
            None => {
                eprintln!("jjx_retire: error: Heat '{}' not found", firemark_key);
                return 1;
            }
        };
        let paddock_path = base_path.join(&heat.paddock_file);
        let paddock_content = match std::fs::read_to_string(&paddock_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("jjx_retire: error reading paddock: {}", e);
                return 1;
            }
        };

        // Build and output trophy preview
        let today = jjrc_timestamp_date();
        match gallops.jjrg_build_trophy_preview(&args.firemark, &paddock_content, &today, &steeplechase) {
            Ok(markdown) => {
                println!("{}", markdown);
                return 0;
            }
            Err(e) => {
                eprintln!("jjx_retire: error: {}", e);
                return 1;
            }
        }
    }

    // --execute: perform the actual retire operation

    // Need mutable gallops for execute path
    let mut gallops = gallops;

    // Acquire lock FIRST
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    // Execute retire
    let retire_args = LibRetireArgs {
        firemark: args.firemark.clone(),
        today: jjrc_timestamp_date(),
    };

    let result = match gallops.jjrg_retire(retire_args, base_path, &steeplechase) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    // Save gallops
    if let Err(e) = gallops.jjrg_save(&args.file) {
        eprintln!("jjx_retire: error saving Gallops: {}", e);
        return 1;
    }

    // Commit using vvcm_commit with explicit file list
    // Files: gallops.json, trophy file (created), paddock file (deleted - git add handles this)
    let gallops_path = args.file.to_string_lossy().to_string();
    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![
            gallops_path,
            result.trophy_path.clone(),
            result.paddock_path.clone(),
        ],
        message: format_heat_message(&firemark, HeatAction::Retire, &result.silks),
        size_limit: 200000,  // 200KB - trophy files can be large
        warn_limit: 100000,
    };

    match vvc::machine_commit(&lock, &commit_args) {
        Ok(hash) => {
            eprintln!("jjx_retire: committed {}", &hash[..8]);
        }
        Err(e) => {
            eprintln!("jjx_retire: commit warning: {}", e);
        }
    }

    // Output result
    println!("trophy: {}", result.trophy_path);
    println!("Heat {} retired successfully", result.firemark);

    0
    // lock released here
}

fn zjjrx_run_nominate(args: zjjrx_NominateArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_NominateArgs as LibNominateArgs;
    use std::path::Path;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_nominate: error: {}", e);
            return 1;
        }
    };

    let mut gallops = if args.file.exists() {
        match Gallops::jjrg_load(&args.file) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("jjx_nominate: error loading Gallops: {}", e);
                return 1;
            }
        }
    } else {
        if let Some(parent) = args.file.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("jjx_nominate: error creating directory: {}", e);
                return 1;
            }
        }
        Gallops {
            next_heat_seed: "AA".to_string(),
            heats: std::collections::BTreeMap::new(),
        }
    };

    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    let silks = args.silks.clone();
    let nominate_args = LibNominateArgs {
        silks: args.silks,
        created: args.created,
    };

    match gallops.jjrg_nominate(nominate_args, base_path) {
        Ok(result) => {
            if let Err(e) = gallops.jjrg_save(&args.file) {
                eprintln!("jjx_nominate: error saving Gallops: {}", e);
                return 1;
            }

            // Commit using vvcm_commit with explicit file list
            let fm = Firemark::jjrf_parse(&result.firemark).expect("nominate returned invalid firemark");
            let gallops_path = args.file.to_string_lossy().to_string();
            let paddock_path = format!(".claude/jjm/jjp_{}.md", fm.jjrf_as_str());
            let commit_args = vvc::vvcm_CommitArgs {
                files: vec![
                    gallops_path,
                    paddock_path,
                ],
                message: format_heat_message(&fm, HeatAction::Nominate, &silks),
                size_limit: 50000,
                warn_limit: 30000,
            };

            match vvc::machine_commit(&lock, &commit_args) {
                Ok(hash) => {
                    eprintln!("jjx_nominate: committed {}", &hash[..8]);
                }
                Err(e) => {
                    eprintln!("jjx_nominate: commit warning: {}", e);
                }
            }

            println!("{}", result.firemark);
            0
        }
        Err(e) => {
            eprintln!("jjx_nominate: error: {}", e);
            1
        }
    }
    // lock released here
}

fn zjjrx_run_slate(args: zjjrx_SlateArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_SlateArgs as LibSlateArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_slate: error: {}", e);
            return 1;
        }
    };

    let text = match read_stdin() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("jjx_slate: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_slate: error loading Gallops: {}", e);
            return 1;
        }
    };

    let firemark = args.firemark.clone();
    let silks = args.silks.clone();
    let slate_args = LibSlateArgs {
        firemark: args.firemark,
        silks: args.silks,
        text,
        before: args.before,
        after: args.after,
        first: args.first,
    };

    match gallops.jjrg_slate(slate_args) {
        Ok(result) => {
            if let Err(e) = gallops.jjrg_save(&args.file) {
                eprintln!("jjx_slate: error saving Gallops: {}", e);
                return 1;
            }

            // Commit using vvcm_commit with explicit file list
            let fm = Firemark::jjrf_parse(&firemark).expect("slate given invalid firemark");
            let gallops_path = args.file.to_string_lossy().to_string();
            let paddock_path = format!(".claude/jjm/jjp_{}.md", fm.jjrf_as_str());
            let commit_args = vvc::vvcm_CommitArgs {
                files: vec![
                    gallops_path,
                    paddock_path,
                ],
                message: format_heat_message(&fm, HeatAction::Slate, &silks),
                size_limit: 50000,
                warn_limit: 30000,
            };

            match vvc::machine_commit(&lock, &commit_args) {
                Ok(hash) => {
                    eprintln!("jjx_slate: committed {}", &hash[..8]);
                }
                Err(e) => {
                    eprintln!("jjx_slate: commit warning: {}", e);
                }
            }

            println!("{}", result.coronet);
            0
        }
        Err(e) => {
            eprintln!("jjx_slate: error: {}", e);
            1
        }
    }
    // lock released here
}

fn zjjrx_run_rail(args: zjjrx_RailArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_RailArgs as LibRailArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_rail: error: {}", e);
            return 1;
        }
    };

    let order: Vec<String> = if args.order.len() == 1 && args.order[0].starts_with('[') {
        match serde_json::from_str(&args.order[0]) {
            Ok(v) => v,
            Err(_) => args.order.clone(),
        }
    } else {
        args.order.clone()
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_rail: error loading Gallops: {}", e);
            return 1;
        }
    };

    let firemark = args.firemark.clone();
    let move_coronet = args.r#move.clone();
    let move_before = args.before.clone();
    let move_after = args.after.clone();
    let move_first = args.first;
    let move_last = args.last;
    let rail_args = LibRailArgs {
        firemark: args.firemark,
        order,
        move_coronet: args.r#move,
        before: args.before,
        after: args.after,
        first: args.first,
        last: args.last,
    };

    match gallops.jjrg_rail(rail_args) {
        Ok(new_order) => {
            if let Err(e) = gallops.jjrg_save(&args.file) {
                eprintln!("jjx_rail: error saving Gallops: {}", e);
                return 1;
            }

            // Compute descriptive subject for commit message
            let subject = if let Some(ref moved) = move_coronet {
                // Move mode: describe where the pace was moved
                let target = if move_first { "to first".to_string() }
                    else if move_last { "to last".to_string() }
                    else if let Some(ref b) = move_before { format!("before {}", b) }
                    else if let Some(ref a) = move_after { format!("after {}", a) }
                    else { "???".to_string() };
                format!("moved {} {}", moved, target)
            } else {
                // Order mode: list the new order
                format!("order: {}", new_order.join(", "))
            };

            // Commit using vvcm_commit with explicit file list
            let fm = Firemark::jjrf_parse(&firemark).expect("rail given invalid firemark");
            let gallops_path = args.file.to_string_lossy().to_string();
            let paddock_path = format!(".claude/jjm/jjp_{}.md", fm.jjrf_as_str());
            let commit_args = vvc::vvcm_CommitArgs {
                files: vec![
                    gallops_path,
                    paddock_path,
                ],
                message: format_heat_message(&fm, HeatAction::Rail, &subject),
                size_limit: 50000,
                warn_limit: 30000,
            };

            match vvc::machine_commit(&lock, &commit_args) {
                Ok(hash) => {
                    eprintln!("jjx_rail: committed {}", &hash[..8]);
                }
                Err(e) => {
                    eprintln!("jjx_rail: commit warning: {}", e);
                }
            }

            for coronet in new_order {
                println!("{}", coronet);
            }
            0
        }
        Err(e) => {
            eprintln!("jjx_rail: error: {}", e);
            1
        }
    }
    // lock released here
}

fn zjjrx_run_tally(args: zjjrx_TallyArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            return 1;
        }
    };

    let text = match read_stdin_optional() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            return 1;
        }
    };

    let state = match &args.state {
        Some(s) => match s.to_lowercase().as_str() {
            "rough" => Some(PaceState::Rough),
            "bridled" => Some(PaceState::Bridled),
            "complete" => Some(PaceState::Complete),
            "abandoned" => Some(PaceState::Abandoned),
            _ => {
                eprintln!("jjx_tally: error: invalid state '{}', must be rough, bridled, complete, or abandoned", s);
                return 1;
            }
        },
        None => None,
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_tally: error loading Gallops: {}", e);
            return 1;
        }
    };

    // Get firemark and silks for commit message before we move args
    let coronet_str = args.coronet.clone();
    let (fm, silks) = match Coronet::jjrf_parse(&coronet_str) {
        Ok(c) => {
            let parent_fm = c.jjrf_parent_firemark();
            let silks = gallops.heats.get(&parent_fm.jjrf_display())
                .and_then(|h| h.paces.get(&c.jjrf_display()))
                .and_then(|p| p.tacks.first().map(|t| t.silks.clone()))
                .unwrap_or_else(|| coronet_str.clone());
            (parent_fm, silks)
        }
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            return 1;
        }
    };

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state,
        direction: args.direction,
        text,
        silks: args.silks,
    };

    match gallops.jjrg_tally(tally_args) {
        Ok(()) => {
            if let Err(e) = gallops.jjrg_save(&args.file) {
                eprintln!("jjx_tally: error saving Gallops: {}", e);
                return 1;
            }

            // Commit using vvcm_commit with explicit file list
            let gallops_path = args.file.to_string_lossy().to_string();
            let paddock_path = format!(".claude/jjm/jjp_{}.md", fm.jjrf_as_str());
            let commit_args = vvc::vvcm_CommitArgs {
                files: vec![
                    gallops_path,
                    paddock_path,
                ],
                message: format_heat_message(&fm, HeatAction::Tally, &silks),
                size_limit: 50000,
                warn_limit: 30000,
            };

            match vvc::machine_commit(&lock, &commit_args) {
                Ok(hash) => {
                    eprintln!("jjx_tally: committed {}", &hash[..8]);
                }
                Err(e) => {
                    eprintln!("jjx_tally: commit warning: {}", e);
                }
            }

            0
        }
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            1
        }
    }
    // lock released here
}

fn zjjrx_run_draft(args: zjjrx_DraftArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_DraftArgs as LibDraftArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_draft: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_draft: error loading Gallops: {}", e);
            return 1;
        }
    };

    let coronet = args.coronet.clone();
    let to = args.to.clone();
    let draft_args = LibDraftArgs {
        coronet: args.coronet,
        to: args.to,
        before: args.before,
        after: args.after,
        first: args.first,
    };

    match gallops.jjrg_draft(draft_args) {
        Ok(result) => {
            if let Err(e) = gallops.jjrg_save(&args.file) {
                eprintln!("jjx_draft: error saving Gallops: {}", e);
                return 1;
            }

            // Commit while holding lock - use destination firemark as identity
            let dest_fm = Firemark::jjrf_parse(&to).expect("draft given invalid destination firemark");
            let commit_args = vvc::vvcc_CommitArgs {
                prefix: None,
                message: Some(format_heat_message(&dest_fm, HeatAction::Draft, &format!("{} → {}", coronet, result.new_coronet))),
                allow_empty: false,
                no_stage: false,
                size_limit: vvc::VVCG_SIZE_LIMIT,
                warn_limit: vvc::VVCG_WARN_LIMIT,
            };
            match lock.vvcc_commit(&commit_args) {
                Ok(hash) => eprintln!("jjx_draft: committed {}", &hash[..8]),
                Err(e) => eprintln!("jjx_draft: commit warning: {}", e),
            }

            println!("{}", result.new_coronet);
            0
        }
        Err(e) => {
            eprintln!("jjx_draft: error: {}", e);
            1
        }
    }
    // lock released here
}
