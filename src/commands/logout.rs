use anyhow::Result;
use clap::Args;

use crate::modules::settings::clear_settings;

#[derive(Args)]
pub struct LogoutArgs {}

pub fn run(_args: LogoutArgs) -> Result<()> {
    if clear_settings()? {
        println!("Logged out. Your settings have been deleted.");
    } else {
        println!("You're already logged out (no settings file found).");
    }
    Ok(())
}
