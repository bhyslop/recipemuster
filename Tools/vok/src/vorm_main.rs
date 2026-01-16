use clap::Parser;
use std::process::ExitCode;

#[path = "vorc_core.rs"]
mod vorc_core;
#[path = "vorc_commit.rs"]
mod vorc_commit;
#[path = "vorg_guard.rs"]
mod vorg_guard;

#[derive(Parser)]
#[command(name = "vvr")]
#[command(version)]
#[command(about = "Voce Viva Rust - Platform utilities for Claude Code kits")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Pre-commit size validation
    Guard(vorg_guard::GuardArgs),
    /// Atomic commit with lock, stage, guard, and optional claude message
    #[command(name = "vvx_commit")]
    VvxCommit(vorc_commit::CommitArgs),
    /// Push with lock (prevents concurrent push/commit)
    #[command(name = "vvx_push")]
    VvxPush(VvxPushArgs),

    // JJK commands (only available with jjk feature)
    #[cfg(feature = "jjk")]
    /// JJ-aware commit with heat/pace context prefix
    #[command(name = "jjx_notch")]
    JjxNotch(JjxNotchArgs),

    #[cfg(feature = "jjk")]
    /// Empty commit marking a steeplechase event
    #[command(name = "jjx_chalk")]
    JjxChalk(JjxChalkArgs),

    #[cfg(feature = "jjk")]
    /// Parse git history for steeplechase entries
    #[command(name = "jjx_rein")]
    JjxRein(JjxReinArgs),

    #[cfg(feature = "jjk")]
    /// Validate Gallops JSON schema
    #[command(name = "jjx_validate")]
    JjxValidate(JjxValidateArgs),

    #[cfg(feature = "jjk")]
    /// List all Heats with summary information
    #[command(name = "jjx_muster")]
    JjxMuster(JjxMusterArgs),

    #[cfg(feature = "jjk")]
    /// Return context needed to saddle up on a Heat
    #[command(name = "jjx_saddle")]
    JjxSaddle(JjxSaddleArgs),

    #[cfg(feature = "jjk")]
    /// Display comprehensive Heat status for project review
    #[command(name = "jjx_parade")]
    JjxParade(JjxParadeArgs),

    #[cfg(feature = "jjk")]
    /// Extract complete Heat data for archival trophy
    #[command(name = "jjx_retire")]
    JjxRetire(JjxRetireArgs),

    #[cfg(feature = "jjk")]
    /// Create a new Heat with empty Pace structure
    #[command(name = "jjx_nominate")]
    JjxNominate(JjxNominateArgs),

    #[cfg(feature = "jjk")]
    /// Add a new Pace to a Heat
    #[command(name = "jjx_slate")]
    JjxSlate(JjxSlateArgs),

    #[cfg(feature = "jjk")]
    /// Reorder Paces within a Heat
    #[command(name = "jjx_rail")]
    JjxRail(JjxRailArgs),

    #[cfg(feature = "jjk")]
    /// Add a new Tack to a Pace
    #[command(name = "jjx_tally")]
    JjxTally(JjxTallyArgs),
}

/// Arguments for vvx_push command
#[derive(clap::Args, Debug)]
struct VvxPushArgs {
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

/// Arguments for jjx_notch command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxNotchArgs {
    /// Active Heat identity (Firemark)
    firemark: String,

    /// Silks of the current pace
    #[arg(long)]
    pace: String,

    /// Commit message (if absent, claude generates from diff)
    #[arg(short, long)]
    message: Option<String>,
}

/// Arguments for jjx_chalk command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxChalkArgs {
    /// Active Heat identity (Firemark)
    firemark: String,

    /// Marker type: APPROACH, WRAP, FLY, or DISCUSSION
    #[arg(long)]
    marker: String,

    /// Marker description text
    #[arg(long)]
    description: String,

    /// Pace silks (required for APPROACH, WRAP, FLY; optional for DISCUSSION)
    #[arg(short, long)]
    pace: Option<String>,
}

/// Arguments for jjx_rein command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxReinArgs {
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
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxValidateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,
}

/// Arguments for jjx_muster command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxMusterArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

    /// Filter by Heat status (current or retired)
    #[arg(long)]
    status: Option<String>,
}

/// Arguments for jjx_saddle command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxSaddleArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,
}

/// Output format modes for jjx_parade
#[cfg(feature = "jjk")]
#[derive(clap::ValueEnum, Clone, Copy, Debug, Default)]
enum ParadeFormatArg {
    /// One line per pace: [state] silks (₢coronet)
    Overview,
    /// Numbered list: N. [state] silks (₢coronet)
    Order,
    /// Full tack text for one pace (requires --pace)
    Detail,
    /// Paddock + all paces with tack text
    #[default]
    Full,
}

/// Arguments for jjx_parade command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxParadeArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

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
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxRetireArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,
}

/// Arguments for jjx_nominate command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxNominateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

    /// Kebab-case display name for the Heat
    #[arg(long, short = 's')]
    silks: String,

    /// Creation date in YYMMDD format
    #[arg(long, short = 'c')]
    created: String,
}

/// Arguments for jjx_slate command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxSlateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

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
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxRailArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

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
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxTallyArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

    /// Target Pace identity (Coronet)
    coronet: String,

    /// Target state (rough, primed, complete, abandoned)
    #[arg(long)]
    state: Option<String>,

    /// Execution guidance (required if state is primed)
    #[arg(long, short = 'd')]
    direction: Option<String>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Some(Commands::Guard(args)) => vorg_guard::run(args),
        Some(Commands::VvxCommit(args)) => vorc_commit::run(args),
        Some(Commands::VvxPush(args)) => run_vvx_push(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxNotch(args)) => run_jjx_notch(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxChalk(args)) => run_jjx_chalk(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxRein(args)) => run_jjx_rein(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxValidate(args)) => run_jjx_validate(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxMuster(args)) => run_jjx_muster(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxSaddle(args)) => run_jjx_saddle(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxParade(args)) => run_jjx_parade(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxRetire(args)) => run_jjx_retire(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxNominate(args)) => run_jjx_nominate(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxSlate(args)) => run_jjx_slate(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxRail(args)) => run_jjx_rail(args),

        #[cfg(feature = "jjk")]
        Some(Commands::JjxTally(args)) => run_jjx_tally(args),

        None => {
            // No subcommand - clap handles --help and --version
            // For bare invocation, show help
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            0
        }
    };

    ExitCode::from(exit_code as u8)
}

#[cfg(feature = "jjk")]
fn run_jjx_notch(args: JjxNotchArgs) -> i32 {
    use jjk::jjrf_favor::Firemark;
    use jjk::jjrn_notch::{NotchArgs, run_notch};

    // Parse firemark
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_notch: error: {}", e);
            return 1;
        }
    };

    let notch_args = NotchArgs {
        firemark,
        pace_silks: args.pace,
        message: args.message,
    };

    run_notch(notch_args)
}

#[cfg(feature = "jjk")]
fn run_jjx_chalk(args: JjxChalkArgs) -> i32 {
    use jjk::jjrf_favor::Firemark;
    use jjk::jjrn_notch::{ChalkArgs, ChalkMarker, run_chalk};

    // Parse firemark
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_chalk: error: {}", e);
            return 1;
        }
    };

    // Parse marker
    let marker = match ChalkMarker::parse(&args.marker) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("jjx_chalk: error: {}", e);
            return 1;
        }
    };

    let chalk_args = ChalkArgs {
        firemark,
        marker,
        description: args.description,
        pace: args.pace,
    };

    run_chalk(chalk_args)
}

#[cfg(feature = "jjk")]
fn run_jjx_rein(args: JjxReinArgs) -> i32 {
    use jjk::jjrs_steeplechase::{ReinArgs, run};

    let rein_args = ReinArgs {
        firemark: args.firemark,
        brand: args.brand,
        limit: args.limit,
    };

    run(rein_args)
}

#[cfg(feature = "jjk")]
fn run_jjx_validate(args: JjxValidateArgs) -> i32 {
    use jjk::jjrg_gallops::Gallops;

    // Load the Gallops file
    let gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_validate: error loading Gallops: {}", e);
            return 1;
        }
    };

    // Validate the structure
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

#[cfg(feature = "jjk")]
fn run_jjx_muster(args: JjxMusterArgs) -> i32 {
    use jjk::jjrg_gallops::HeatStatus;
    use jjk::jjrq_query::{MusterArgs, run_muster};

    // Parse status filter if provided
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

    let muster_args = MusterArgs {
        file: args.file,
        status,
    };

    run_muster(muster_args)
}

#[cfg(feature = "jjk")]
fn run_jjx_saddle(args: JjxSaddleArgs) -> i32 {
    use jjk::jjrf_favor::Firemark;
    use jjk::jjrq_query::{SaddleArgs, run_saddle};

    // Parse firemark
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_saddle: error: {}", e);
            return 1;
        }
    };

    let saddle_args = SaddleArgs {
        file: args.file,
        firemark,
    };

    run_saddle(saddle_args)
}

#[cfg(feature = "jjk")]
fn run_jjx_parade(args: JjxParadeArgs) -> i32 {
    use jjk::jjrf_favor::Firemark;
    use jjk::jjrq_query::{ParadeArgs, ParadeFormat, run_parade};

    // Parse firemark
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_parade: error: {}", e);
            return 1;
        }
    };

    // Convert Clap enum to library enum
    let format = match args.format {
        ParadeFormatArg::Overview => ParadeFormat::Overview,
        ParadeFormatArg::Order => ParadeFormat::Order,
        ParadeFormatArg::Detail => ParadeFormat::Detail,
        ParadeFormatArg::Full => ParadeFormat::Full,
    };

    let parade_args = ParadeArgs {
        file: args.file,
        firemark,
        format,
        pace: args.pace,
    };

    run_parade(parade_args)
}

#[cfg(feature = "jjk")]
fn run_jjx_retire(args: JjxRetireArgs) -> i32 {
    use jjk::jjrf_favor::Firemark;
    use jjk::jjrq_query::{RetireArgs, run_retire};

    // Parse firemark
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    let retire_args = RetireArgs {
        file: args.file,
        firemark,
    };

    run_retire(retire_args)
}

#[cfg(feature = "jjk")]
fn run_jjx_nominate(args: JjxNominateArgs) -> i32 {
    use jjk::jjrg_gallops::{Gallops, NominateArgs};
    use std::path::Path;

    // Load or create the Gallops file
    let mut gallops = if args.file.exists() {
        match Gallops::load(&args.file) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("jjx_nominate: error loading Gallops: {}", e);
                return 1;
            }
        }
    } else {
        // Create parent directory if needed
        if let Some(parent) = args.file.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("jjx_nominate: error creating directory: {}", e);
                return 1;
            }
        }
        // Initialize new Gallops with seed AA and empty heats
        Gallops {
            next_heat_seed: "AA".to_string(),
            heats: std::collections::HashMap::new(),
        }
    };

    // Determine base path (parent of gallops file, which is in .claude/jjm/)
    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    let nominate_args = NominateArgs {
        silks: args.silks,
        created: args.created,
    };

    // Execute nominate
    match gallops.nominate(nominate_args, base_path) {
        Ok(result) => {
            // Save atomically
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_nominate: error saving Gallops: {}", e);
                return 1;
            }
            println!("{}", result.firemark);
            0
        }
        Err(e) => {
            eprintln!("jjx_nominate: error: {}", e);
            1
        }
    }
}

#[cfg(feature = "jjk")]
fn run_jjx_slate(args: JjxSlateArgs) -> i32 {
    use jjk::jjrg_gallops::{Gallops, SlateArgs, read_stdin};

    // Read tack text from stdin
    let text = match read_stdin() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("jjx_slate: error: {}", e);
            return 1;
        }
    };

    // Load the Gallops file
    let mut gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_slate: error loading Gallops: {}", e);
            return 1;
        }
    };

    let slate_args = SlateArgs {
        firemark: args.firemark,
        silks: args.silks,
        text,
        before: args.before,
        after: args.after,
        first: args.first,
    };

    // Execute slate
    match gallops.slate(slate_args) {
        Ok(result) => {
            // Save atomically
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_slate: error saving Gallops: {}", e);
                return 1;
            }
            println!("{}", result.coronet);
            0
        }
        Err(e) => {
            eprintln!("jjx_slate: error: {}", e);
            1
        }
    }
}

#[cfg(feature = "jjk")]
fn run_jjx_rail(args: JjxRailArgs) -> i32 {
    use jjk::jjrg_gallops::{Gallops, RailArgs};

    // Parse order - handle both space-separated and JSON array
    let order: Vec<String> = if args.order.len() == 1 && args.order[0].starts_with('[') {
        // Try to parse as JSON array
        match serde_json::from_str(&args.order[0]) {
            Ok(v) => v,
            Err(_) => args.order.clone(),
        }
    } else {
        args.order.clone()
    };

    // Load the Gallops file
    let mut gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_rail: error loading Gallops: {}", e);
            return 1;
        }
    };

    let rail_args = RailArgs {
        firemark: args.firemark,
        order,
        move_coronet: args.r#move,
        before: args.before,
        after: args.after,
        first: args.first,
        last: args.last,
    };

    // Execute rail
    match gallops.rail(rail_args) {
        Ok(new_order) => {
            // Save atomically
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_rail: error saving Gallops: {}", e);
                return 1;
            }
            // Output new order, one coronet per line
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
}

#[cfg(feature = "jjk")]
fn run_jjx_tally(args: JjxTallyArgs) -> i32 {
    use jjk::jjrg_gallops::{Gallops, TallyArgs, PaceState, read_stdin_optional};

    // Read optional text from stdin
    let text = match read_stdin_optional() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            return 1;
        }
    };

    // Parse state if provided
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

    // Load the Gallops file
    let mut gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_tally: error loading Gallops: {}", e);
            return 1;
        }
    };

    let tally_args = TallyArgs {
        coronet: args.coronet,
        state,
        direction: args.direction,
        text,
    };

    // Execute tally
    match gallops.tally(tally_args) {
        Ok(()) => {
            // Save atomically
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_tally: error saving Gallops: {}", e);
                return 1;
            }
            0
        }
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            1
        }
    }
}

/// Lock reference path for push operations (same as commit to prevent concurrent ops)
const PUSH_LOCK_REF: &str = "refs/vvg/locks/vvx";

fn run_vvx_push(args: VvxPushArgs) -> i32 {
    use std::process::Command;

    // Step 1: Acquire lock (same lock as commit to prevent concurrent commit/push)
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
        .args(["update-ref", PUSH_LOCK_REF, &lock_value, ""])
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

    // Execute push workflow (lock will be released on any exit path)
    let result = run_push_workflow(&args);

    // Release lock
    let _ = Command::new("git")
        .args(["update-ref", "-d", PUSH_LOCK_REF])
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

fn run_push_workflow(args: &VvxPushArgs) -> Result<(), String> {
    use std::process::Command;

    // Determine branch to push
    let branch = match &args.branch {
        Some(b) => b.clone(),
        None => {
            // Get current branch name
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

    // Build push command
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
