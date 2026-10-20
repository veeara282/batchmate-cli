use clap::Args;

#[derive(Args)]
pub struct JoinArgs {
    /// Name of the room, e.g. `verve` or `ps1`
    pub room: String,
}

pub fn run(args: JoinArgs) {
    // TODO: look up the Zoom URL for the room and open it
    println!("Joining room: {}", args.room);
}
