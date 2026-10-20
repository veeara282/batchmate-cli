use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum EventsCommand {
    /// List upcoming events on the Recurse Center calendar
    List(ListArgs),
}

#[derive(Args)]
pub struct ListArgs {
    /// Maximum number of events to show
    #[arg(short, long, default_value_t = 10)]
    pub limit: usize,
}

pub fn run(command: EventsCommand) {
    match command {
        EventsCommand::List(args) => list(args),
    }
}

fn list(args: ListArgs) {
    // TODO: fetch upcoming events from the private calendar
    println!("Listing up to {} upcoming events", args.limit);
}
