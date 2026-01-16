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

use crate::jjrf_favor::Firemark;
use crate::jjrg_gallops::{Gallops, PaceState, read_stdin, read_stdin_optional};
use crate::jjrn_notch::{ChalkMarker, format_notch_prefix, format_chalk_message, validate_chalk_args};

/// JJK subcommands - all jjx_* commands
#[derive(Parser)]
#[command(name = "jjx")]
#[command(about = "Job Jockey Kit commands")]
pub enum JjxCommands {
    /// JJ-aware commit with heat/pace context prefix
    #[command(name = "jjx_notch")]
    Notch(NotchArgs),

    /// Empty commit marking a steeplechase event
    #[command(name = "jjx_chalk")]
    Chalk(ChalkArgs),

    /// Parse git history for steeplechase entries
    #[command(name = "jjx_rein")]
    Rein(ReinArgs),

    /// Validate Gallops JSON schema
    #[command(name = "jjx_validate")]
    Validate(ValidateArgs),

    /// List all Heats with summary information
    #[command(name = "jjx_muster")]
    Muster(MusterArgs),

    /// Return context needed to saddle up on a Heat
    #[command(name = "jjx_saddle")]
    Saddle(SaddleArgs),

    /// Display comprehensive Heat status for project review
    #[command(name = "jjx_parade")]
    Parade(ParadeArgs),

    /// Extract complete Heat data for archival trophy
    #[command(name = "jjx_retire")]
    Retire(RetireArgs),

    /// Create a new Heat with empty Pace structure
    #[command(name = "jjx_nominate")]
    Nominate(NominateArgs),

    /// Add a new Pace to a Heat
    #[command(name = "jjx_slate")]
    Slate(SlateArgs),

    /// Reorder Paces within a Heat
    #[command(name = "jjx_rail")]
    Rail(RailArgs),

    /// Add a new Tack to a Pace
    #[command(name = "jjx_tally")]
    Tally(TallyArgs),

    /// Move a Pace from one Heat to another
    #[command(name = "jjx_draft")]
    Draft(DraftArgs),
}

/// Arguments for jjx_notch command
#[derive(clap::Args, Debug)]
pub struct NotchArgs {
    /// Active Heat identity (Firemark)
    pub firemark: String,

    /// Silks of the current pace
    #[arg(long)]
    pub pace: String,

    /// Commit message (if absent, claude generates from diff)
    #[arg(short, long)]
    pub message: Option<String>,
}

/// Arguments for jjx_chalk command
#[derive(clap::Args, Debug)]
pub struct ChalkArgs {
    /// Active Heat identity (Firemark)
    pub firemark: String,

    /// Marker type: APPROACH, WRAP, FLY, or DISCUSSION
    #[arg(long)]
    pub marker: String,

    /// Marker description text
    #[arg(long)]
    pub description: String,

    /// Pace silks (required for APPROACH, WRAP, FLY; optional for DISCUSSION)
    #[arg(short, long)]
    pub pace: Option<String>,
}

/// Arguments for jjx_rein command
#[derive(clap::Args, Debug)]
struct ReinArgs {
    /// Target Heat identity (Firemark)
    firemark: String,

    /// Repository brand identifier (from arcanum installation)
    #[arg(long)]
    brand: String,

    /// Maximum entries to return
    #[arg(long, default_value = "50")]
    limit: usize,
}

/// Arguments for jjx_validate command
#[derive(clap::Args, Debug)]
struct ValidateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,
}

/// Arguments for jjx_muster command
#[derive(clap::Args, Debug)]
struct MusterArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Filter by Heat status (current or retired)
    #[arg(long)]
    status: Option<String>,
}

/// Arguments for jjx_saddle command
#[derive(clap::Args, Debug)]
struct SaddleArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,
}

/// Output format modes for jjx_parade
#[derive(clap::ValueEnum, Clone, Copy, Debug, Default)]
enum ParadeFormatArg {
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
struct ParadeArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,

    /// Output format mode
    #[arg(long, value_enum, default_value = "full")]
    format: ParadeFormatArg,

    /// Target Pace coronet (required for --format detail)
    #[arg(long)]
    pace: Option<String>,
}

/// Arguments for jjx_retire command
#[derive(clap::Args, Debug)]
struct RetireArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,
}

/// Arguments for jjx_nominate command
#[derive(clap::Args, Debug)]
struct NominateArgs {
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
struct SlateArgs {
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
struct RailArgs {
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
struct TallyArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: PathBuf,

    /// Target Pace identity (Coronet)
    coronet: String,

    /// Target state (rough, primed, complete, abandoned)
    #[arg(long)]
    state: Option<String>,

    /// Execution guidance (required if state is primed)
    #[arg(long, short = 'd')]
    direction: Option<String>,
}

/// Arguments for jjx_draft command
#[derive(clap::Args, Debug)]
struct DraftArgs {
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
pub fn dispatch(args: &[OsString]) -> i32 {
    // Parse the subcommand and arguments
    let parsed = match JjxCommands::try_parse_from(args) {
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
        JjxCommands::Notch(args) => run_notch(args),
        JjxCommands::Chalk(args) => run_chalk(args),
        JjxCommands::Rein(args) => run_rein(args),
        JjxCommands::Validate(args) => run_validate(args),
        JjxCommands::Muster(args) => run_muster(args),
        JjxCommands::Saddle(args) => run_saddle(args),
        JjxCommands::Parade(args) => run_parade(args),
        JjxCommands::Retire(args) => run_retire(args),
        JjxCommands::Nominate(args) => run_nominate(args),
        JjxCommands::Slate(args) => run_slate(args),
        JjxCommands::Rail(args) => run_rail(args),
        JjxCommands::Tally(args) => run_tally(args),
        JjxCommands::Draft(args) => run_draft(args),
    }
}

/// Check if a command name is a JJK command
pub fn is_jjk_command(name: &str) -> bool {
    name.starts_with("jjx_")
}

// ============================================================================
// Notch/Chalk implementations (use vvc for commits)
// ============================================================================

fn run_notch(args: NotchArgs) -> i32 {
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_notch: error: {}", e);
            return 1;
        }
    };

    let prefix = format_notch_prefix(&firemark, &args.pace);

    let commit_args = vvc::CommitArgs {
        prefix: Some(prefix),
        message: args.message,
        allow_empty: false,
        no_stage: false,
    };

    vvc::commit(&commit_args)
}

fn run_chalk(args: ChalkArgs) -> i32 {
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_chalk: error: {}", e);
            return 1;
        }
    };

    let marker = match ChalkMarker::parse(&args.marker) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("jjx_chalk: error: {}", e);
            return 1;
        }
    };

    if let Err(e) = validate_chalk_args(marker, args.pace.as_deref()) {
        eprintln!("jjx_chalk: error: {}", e);
        return 1;
    }

    let message = format_chalk_message(&firemark, marker, args.pace.as_deref(), &args.description);

    let commit_args = vvc::CommitArgs {
        prefix: None,
        message: Some(message),
        allow_empty: true,
        no_stage: true,
    };

    vvc::commit(&commit_args)
}

// ============================================================================
// Command implementations (pure JJK operations)
// ============================================================================

fn run_rein(args: ReinArgs) -> i32 {
    use crate::jjrs_steeplechase::{ReinArgs as LibReinArgs, run};

    let rein_args = LibReinArgs {
        firemark: args.firemark,
        brand: args.brand,
        limit: args.limit,
    };

    run(rein_args)
}

fn run_validate(args: ValidateArgs) -> i32 {
    let gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_validate: error loading Gallops: {}", e);
            return 1;
        }
    };

    match gallops.validate() {
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

fn run_muster(args: MusterArgs) -> i32 {
    use crate::jjrg_gallops::HeatStatus;
    use crate::jjrq_query::{MusterArgs as LibMusterArgs, run_muster as lib_run_muster};

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

fn run_saddle(args: SaddleArgs) -> i32 {
    use crate::jjrq_query::{SaddleArgs as LibSaddleArgs, run_saddle as lib_run_saddle};

    let firemark = match Firemark::parse(&args.firemark) {
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

fn run_parade(args: ParadeArgs) -> i32 {
    use crate::jjrq_query::{ParadeArgs as LibParadeArgs, ParadeFormat, run_parade as lib_run_parade};

    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_parade: error: {}", e);
            return 1;
        }
    };

    let format = match args.format {
        ParadeFormatArg::Overview => ParadeFormat::Overview,
        ParadeFormatArg::Order => ParadeFormat::Order,
        ParadeFormatArg::Detail => ParadeFormat::Detail,
        ParadeFormatArg::Full => ParadeFormat::Full,
    };

    let parade_args = LibParadeArgs {
        file: args.file,
        firemark,
        format,
        pace: args.pace,
    };

    lib_run_parade(parade_args)
}

fn run_retire(args: RetireArgs) -> i32 {
    use crate::jjrq_query::{RetireArgs as LibRetireArgs, run_retire as lib_run_retire};

    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    let retire_args = LibRetireArgs {
        file: args.file,
        firemark,
    };

    lib_run_retire(retire_args)
}

fn run_nominate(args: NominateArgs) -> i32 {
    use crate::jjrg_gallops::NominateArgs as LibNominateArgs;
    use std::path::Path;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::CommitLock::acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_nominate: error: {}", e);
            return 1;
        }
    };

    let mut gallops = if args.file.exists() {
        match Gallops::load(&args.file) {
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

    match gallops.nominate(nominate_args, base_path) {
        Ok(result) => {
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_nominate: error saving Gallops: {}", e);
                return 1;
            }

            // Commit while holding lock
            let commit_args = vvc::CommitArgs {
                message: Some(format!("Nominate: {} {}", silks, result.firemark)),
                ..Default::default()
            };
            match lock.commit(&commit_args) {
                Ok(hash) => eprintln!("jjx_nominate: committed {}", &hash[..8]),
                Err(e) => eprintln!("jjx_nominate: commit warning: {}", e),
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

fn run_slate(args: SlateArgs) -> i32 {
    use crate::jjrg_gallops::SlateArgs as LibSlateArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::CommitLock::acquire() {
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

    let mut gallops = match Gallops::load(&args.file) {
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

    match gallops.slate(slate_args) {
        Ok(result) => {
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_slate: error saving Gallops: {}", e);
                return 1;
            }

            // Commit while holding lock
            let commit_args = vvc::CommitArgs {
                message: Some(format!("Slate: {} in ₣{}", silks, firemark)),
                ..Default::default()
            };
            match lock.commit(&commit_args) {
                Ok(hash) => eprintln!("jjx_slate: committed {}", &hash[..8]),
                Err(e) => eprintln!("jjx_slate: commit warning: {}", e),
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

fn run_rail(args: RailArgs) -> i32 {
    use crate::jjrg_gallops::RailArgs as LibRailArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::CommitLock::acquire() {
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

    let mut gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_rail: error loading Gallops: {}", e);
            return 1;
        }
    };

    let firemark = args.firemark.clone();
    let rail_args = LibRailArgs {
        firemark: args.firemark,
        order,
        move_coronet: args.r#move,
        before: args.before,
        after: args.after,
        first: args.first,
        last: args.last,
    };

    match gallops.rail(rail_args) {
        Ok(new_order) => {
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_rail: error saving Gallops: {}", e);
                return 1;
            }

            // Commit while holding lock
            let commit_args = vvc::CommitArgs {
                message: Some(format!("Rail: reorder ₣{}", firemark)),
                ..Default::default()
            };
            match lock.commit(&commit_args) {
                Ok(hash) => eprintln!("jjx_rail: committed {}", &hash[..8]),
                Err(e) => eprintln!("jjx_rail: commit warning: {}", e),
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

fn run_tally(args: TallyArgs) -> i32 {
    use crate::jjrg_gallops::TallyArgs as LibTallyArgs;
    use crate::jjrf_favor::Coronet;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::CommitLock::acquire() {
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
            "primed" => Some(PaceState::Primed),
            "complete" => Some(PaceState::Complete),
            "abandoned" => Some(PaceState::Abandoned),
            _ => {
                eprintln!("jjx_tally: error: invalid state '{}', must be rough, primed, complete, or abandoned", s);
                return 1;
            }
        },
        None => None,
    };

    let mut gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_tally: error loading Gallops: {}", e);
            return 1;
        }
    };

    // Get silks for commit message before we move args
    let coronet_str = args.coronet.clone();
    let silks = Coronet::parse(&coronet_str)
        .ok()
        .and_then(|c| {
            let firemark = c.parent_firemark().display();
            gallops.heats.get(&firemark)
                .and_then(|h| h.paces.get(&c.display()))
                .map(|p| p.silks.clone())
        })
        .unwrap_or_else(|| coronet_str.clone());

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state,
        direction: args.direction,
        text,
    };

    match gallops.tally(tally_args) {
        Ok(()) => {
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_tally: error saving Gallops: {}", e);
                return 1;
            }

            // Commit while holding lock
            let commit_args = vvc::CommitArgs {
                message: Some(format!("Tally: {}", silks)),
                ..Default::default()
            };
            match lock.commit(&commit_args) {
                Ok(hash) => eprintln!("jjx_tally: committed {}", &hash[..8]),
                Err(e) => eprintln!("jjx_tally: commit warning: {}", e),
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

fn run_draft(args: DraftArgs) -> i32 {
    use crate::jjrg_gallops::DraftArgs as LibDraftArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::CommitLock::acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_draft: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::load(&args.file) {
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

    match gallops.draft(draft_args) {
        Ok(result) => {
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_draft: error saving Gallops: {}", e);
                return 1;
            }

            // Commit while holding lock
            let commit_args = vvc::CommitArgs {
                message: Some(format!("Draft: {} → ₣{}", coronet, to)),
                ..Default::default()
            };
            match lock.commit(&commit_args) {
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
