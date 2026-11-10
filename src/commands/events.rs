use anyhow::Result;
use chrono::{Local, Utc};
use clap::{Args, Subcommand};
use nu_ansi_term::Color::{Blue, Green, Red};

use crate::modules::calendar::load_calendar;
use crate::modules::event::{RCEvent, list_upcoming_events};

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

pub fn run(command: EventsCommand) -> Result<()> {
    match command {
        EventsCommand::List(args) => list(args),
    }
}

fn list(args: ListArgs) -> Result<()> {
    match load_calendar() {
        Ok(parsed_calendar) => {
            println!("Listing up to {} upcoming events", args.limit);
            let future_and_ongoing_events: Vec<RCEvent> =
                list_upcoming_events(parsed_calendar, args.limit);

            for event in future_and_ongoing_events {
                println!("|-{}", Red.bold().paint(event.summary));
                println!(
                    "| {} at: {}",
                    if event.start_time <= Utc::now() {
                        "Started"
                    } else {
                        "Starts"
                    },
                    event.start_time.with_timezone(&Local)
                );
                println!("| Ends at: {}", event.end_time.with_timezone(&Local));
                println!("| Hub location: {}", Green.paint(event.location));
                println!("| Join: {}", Blue.underline().paint(event.zoom_url));
            }

            Ok(())
        }
        Err(error) => {
            eprintln!("{} {:#}", Red.paint("Error:"), error);
            std::process::exit(1);
        }
    }
}
