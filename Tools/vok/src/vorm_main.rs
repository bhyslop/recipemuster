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
