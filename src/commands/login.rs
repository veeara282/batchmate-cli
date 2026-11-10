use anyhow::Result;
use clap::Args;

use text_io::read;

use crate::modules::settings::{Settings, write_settings};

#[derive(Args)]
pub struct LoginArgs {}

pub fn run(_args: LoginArgs) -> Result<()> {
    println!("Paste your RC calendar feed URL here:");
    let input_url: String = read!();
    // some validation should happen here
    let new_settings = Settings {
        calendar_url: input_url,
    };
    write_settings(&new_settings)?;
    println!("Settings saved!");
    Ok(())
}
