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
    Commit(vorc_commit::CommitArgs),

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

/// Arguments for jjx_parade command
#[cfg(feature = "jjk")]
#[derive(clap::Args, Debug)]
struct JjxParadeArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    file: std::path::PathBuf,

    /// Target Heat identity (Firemark)
    firemark: String,

    /// Include tack details for complete/abandoned paces
    #[arg(long)]
    full: bool,
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

    /// New order of Coronets (space-separated or JSON array)
    #[arg(trailing_var_arg = true)]
    order: Vec<String>,
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
        Some(Commands::Commit(args)) => vorc_commit::run(args),

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
    use jjk::jjrq_query::{ParadeArgs, run_parade};

    // Parse firemark
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_parade: error: {}", e);
            return 1;
        }
    };

    let parade_args = ParadeArgs {
        file: args.file,
        firemark,
        full: args.full,
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

    // Load the Gallops file
    let mut gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_nominate: error loading Gallops: {}", e);
            return 1;
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
    };

    // Execute rail
    match gallops.rail(rail_args) {
        Ok(()) => {
            // Save atomically
            if let Err(e) = gallops.save(&args.file) {
                eprintln!("jjx_rail: error saving Gallops: {}", e);
                return 1;
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
