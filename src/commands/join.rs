use anyhow::{Result, anyhow};
use clap::Args;
use regex::Regex;

#[derive(Args)]
pub struct JoinArgs {
    /// Name of the room, e.g. `verve` or `ps1`
    pub room: String,
}

/// Converts a user-supplied Zoom room name to its canonical form, which is the
/// path segment used in the redirect URL, e.g. `https://www.recurse.com/zoom/pairing_station_1`.
///
/// Matching is case-insensitive, and hyphens are treated as underscores. Accepts:
/// - Named rooms: `aegis`, `couches`, `edos`, `genera`, `midori`, `verve`
/// - `pspace` as a shorthand for `presentation_space`
/// - Pairing stations 1-5, as `ps1` or `pairing_station_1` (e.g. `ps1` -> `pairing_station_1`)
///
/// Returns an error if the name doesn't match any known room.
pub fn get_canonical_room_name(name: String) -> Result<String> {
    // Convert to lowercase and replace hyphens with underscores to match the URL format
    let name_lower = name.to_lowercase().replace("-", "_");
    match name_lower.as_str() {
        "aegis" | "couches" | "edos" | "genera" | "midori" | "verve" => Ok(name_lower),
        "pspace" | "presentation_space" => Ok(String::from("presentation_space")),
        _ => {
            let ps_regex = Regex::new("^(ps|pairing_station_)([1-5])$").unwrap();
            if let Some(caps) = ps_regex.captures(&name_lower) {
                return Ok(format!("pairing_station_{}", &caps[2]));
            }
            Err(anyhow!("Invalid room name: {}", name))
        }
    }
}

pub fn run(args: JoinArgs) -> Result<()> {
    // TODO: look up the Zoom URL for the room and open it
    let canonical_room = get_canonical_room_name(args.room)?;
    println!("Joining room: {}", canonical_room);
    Ok(())
}
