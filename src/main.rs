mod commands;

use anyhow::Result;

use clap::{Parser, Subcommand};

use commands::{events, join};

#[derive(Parser)]
#[command(name = "batchmate", version, about = "Recurse Center batchmate CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Join a Zoom room, e.g. `verve` or `ps1`
    Join(join::JoinArgs),
    /// Work with the Recurse Center calendar
    Events {
        #[command(subcommand)]
        command: events::EventsCommand,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Join(args) => join::run(args),
        Command::Events { command } => events::run(command),
    }
}
