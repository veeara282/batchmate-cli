use anyhow::Result;
use clap::Args;
use nu_ansi_term::Color::Blue;

use text_io::read;

use crate::modules::settings::{Settings, write_settings};

#[derive(Args)]
pub struct LoginArgs {}

pub fn run(_args: LoginArgs) -> Result<()> {
    println!(
        "Paste your RC calendar feed URL below (visit {}):",
        Blue.paint("https://www.recurse.com/settings/calendar")
    );
    let input_url: String = read!();
    // some validation should happen here
    let new_settings = Settings {
        calendar_url: input_url,
    };
    write_settings(&new_settings)?;
    println!("Settings saved!");
    Ok(())
}
