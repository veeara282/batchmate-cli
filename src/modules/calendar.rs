use anyhow::{Result, anyhow};
use icalendar::Calendar;
use ureq;

use crate::modules::settings::read_settings;

pub fn load_calendar() -> Result<Calendar> {
    let settings = read_settings()?;
    let calendar_data = ureq::get(settings.calendar_url)
        .call()?
        .body_mut()
        .read_to_string()?;
    let parsed = calendar_data
        .parse()
        .map_err(|_e| anyhow!("Calendar parsing failed"))?;
    Ok(parsed)
}
