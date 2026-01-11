use clap::Parser;
use std::process::ExitCode;

#[path = "vorc_core.rs"]
mod vorc_core;
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
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Some(Commands::Guard(args)) => vorg_guard::run(args),
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
