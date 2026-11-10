use std::time::{self, SystemTime};

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use icalendar::{Calendar, Component, DatePerhapsTime};

#[derive(Clone, Debug)]
pub struct RCEvent {
    pub summary: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub zoom_url: String,
}

fn convert_ical_time(raw: DatePerhapsTime) -> Result<DateTime<Utc>> {
    match raw {
        DatePerhapsTime::Date(_dt) => Err(anyhow!("Full day event unsupported for now")),
        DatePerhapsTime::DateTime(dtm) => dtm
            .as_dt_with_tz()
            .and_then(|dt| Some(dt.with_timezone(&Utc)))
            .ok_or(anyhow!("Datetime conversion failed")),
    }
}

pub fn list_upcoming_events(calendar: Calendar, limit: usize) -> Vec<RCEvent> {
    let mut events_list = calendar
        .events()
        .filter(|event| {
            event.get_summary().is_some()
                && event.get_start().is_some()
                && event.get_end().is_some()
        })
        .map(|event| {
            let start_raw = event.get_start().unwrap();
            let end_raw = event.get_end().unwrap();
            RCEvent {
                summary: event.get_summary().unwrap_or_default().to_string(),
                description: event.get_description().unwrap_or_default().to_string(),
                start_time: convert_ical_time(start_raw).unwrap(),
                end_time: convert_ical_time(end_raw).unwrap(),
                zoom_url: event
                    .property_value("X-CONFERENCE-URL")
                    .unwrap_or_default()
                    .to_string(),
            }
        })
        .filter(|event| event.end_time >= Utc::now())
        .collect::<Vec<RCEvent>>();
    events_list.sort_by_key(|event| event.start_time);
    events_list[..limit].to_owned()
}
