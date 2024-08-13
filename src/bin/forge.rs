/// Forge
///
/// New design for a software development environment
///
/// Forge will run as a daemon on the host, and whenever it detects a directory that has a Forgefile,
/// the daemon will subscribe to file system changes.
///
/// Forge CLI will be responsible for dealing with the Forge daemon
///
/// Forgefiles will be TOML with a block for `build`, `test`, and `run`
use std::{option::Option, path::PathBuf};

use clap::{Parser, Subcommand};

use forge::{
    protocol::{ForgeCommand, ForgeRequest},
    socket,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<CliCommands>,
}

// TODO: If I give this some thought, I think I can combine CliCommands and ForgeCommands into a single
// enum. The design pattern that I want to loosely follow anyway is sending all commands to the daemon
// so I should just unify their designs now.
//
// One thing that I'll probably want to do is tighten up the API so that the directory is Option<PathBuf>
// in Clap and PathBuf in the Server protocol.
#[derive(Subcommand)]
enum CliCommands {
    /// Show the status of the current directory
    Status {
        /// If None, show the status of the project in the current directory.
        /// If Some(pathbuf), show the status of the project in that directory.
        #[arg(short, long, value_name = "DIR")]
        directory: Option<PathBuf>,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(CliCommands::Status { directory }) => {
            run_status(directory);
        }
        None => {}
    }

    Ok(())
}

fn run_status(directory: &Option<PathBuf>) {
    let d = match &directory {
        Some(dir) => dir,
        None => &std::env::current_dir().unwrap(),
    };

    let req = ForgeRequest {
        command: ForgeCommand::Status,
        // TODO: this is an unneccesary copy
        directory: d.to_path_buf(),
    };

    socket::send_to_socket(&req).unwrap();
}

// Commands that we want to support:
//
// `status` This should return a list of each of the running subprocesses for the
// tasks configured in the Forgefile here.

// Helpful blog post about managing stdout: https://andrewra.dev/2019/08/05/testing-in-rust-writing-to-stdout/
