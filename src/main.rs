use std::{path::PathBuf, process::ExitCode};

use anyhow::{Result, bail};
use clap::{Args, ColorChoice, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Manage dotfiles safely",
    long_about = None,
    color = ColorChoice::Never,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize the configuration and source tree.
    Init(InitArgs),

    /// Install all managed files.
    Install(InstallArgs),

    /// Add files to the source tree.
    Add(AddArgs),

    /// Remove files from management.
    Remove(RemoveArgs),

    /// Show the status of managed files.
    Status(StatusArgs),
}

#[derive(Debug, Args)]
struct InitArgs {
    #[command(flatten)]
    dry_run: DryRunArgs,

    /// Print the fallback configuration.
    #[arg(long, conflicts_with = "dry_run")]
    print: bool,
}

#[derive(Debug, Args)]
struct InstallArgs {
    #[command(flatten)]
    config: ConfigArgs,

    #[command(flatten)]
    dry_run: DryRunArgs,
}

#[derive(Debug, Args)]
struct AddArgs {
    #[command(flatten)]
    config: ConfigArgs,

    #[command(flatten)]
    dry_run: DryRunArgs,

    /// Install the added files.
    #[arg(long, conflicts_with = "update")]
    install: bool,

    /// Update existing managed files.
    #[arg(long)]
    update: bool,

    /// Paths to add.
    #[arg(required = true, num_args = 1.., value_name = "PATH")]
    paths: Vec<PathBuf>,
}

#[derive(Debug, Args)]
struct RemoveArgs {
    #[command(flatten)]
    config: ConfigArgs,

    #[command(flatten)]
    dry_run: DryRunArgs,

    /// Paths to remove.
    #[arg(required = true, num_args = 1.., value_name = "PATH")]
    paths: Vec<PathBuf>,
}

#[derive(Debug, Args)]
struct StatusArgs {
    #[command(flatten)]
    config: ConfigArgs,
}

#[derive(Debug, Args)]
struct ConfigArgs {
    /// Use a specific configuration file.
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct DryRunArgs {
    /// Show the plan without changing the file system.
    #[arg(long)]
    dry_run: bool,
}

fn main() -> ExitCode {
    match Cli::try_parse() {
        Ok(cli) => report_result(dispatch(cli.command)),
        Err(error) => report_clap_error(&error),
    }
}

fn dispatch(command: Command) -> Result<()> {
    match command {
        Command::Init(InitArgs { dry_run, print }) => {
            let _ = (dry_run.dry_run, print);
            unimplemented_command("init")
        }
        Command::Install(InstallArgs { config, dry_run }) => {
            let _ = (config.config, dry_run.dry_run);
            unimplemented_command("install")
        }
        Command::Add(AddArgs {
            config,
            dry_run,
            install,
            update,
            paths,
        }) => {
            let _ = (config.config, dry_run.dry_run, install, update, paths);
            unimplemented_command("add")
        }
        Command::Remove(RemoveArgs {
            config,
            dry_run,
            paths,
        }) => {
            let _ = (config.config, dry_run.dry_run, paths);
            unimplemented_command("remove")
        }
        Command::Status(StatusArgs { config }) => {
            let _ = config.config;
            unimplemented_command("status")
        }
    }
}

fn unimplemented_command(name: &str) -> Result<()> {
    bail!("the '{name}' command is not implemented yet")
}

fn report_result(result: Result<()>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn report_clap_error(error: &clap::Error) -> ExitCode {
    let is_error = error.use_stderr();

    if let Err(print_error) = error.print() {
        eprintln!("Error: failed to write command-line output: {print_error}");
        return ExitCode::FAILURE;
    }

    if is_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
