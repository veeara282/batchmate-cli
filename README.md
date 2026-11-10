# Batchmate CLI
A command-line utility for joining Recurse Center Zoom rooms from the terminal.

## Usage

```
batchmate join <room>
```

Opens the Zoom room in your default browser. `<room>` can be any of:

| You type | Joins |
| --- | --- |
| `aegis`, `couches`, `edos`, `genera`, `midori`, `verve` | the room with that name |
| `pspace` or `presentation-space` | Presentation Space |
| `ps1` – `ps5` or `pairing-station-1` – `pairing-station-5` | the numbered Pairing Station |

Room names are case-insensitive, and the long names can be written in kebab-case (`presentation-space`, recommended) or snake_case (`presentation_space`). The shorthands (`pspace`, `ps1`, etc.) are the quickest to type.

For example:
```
batchmate join verve
batchmate join ps2
```

### Events

```
batchmate login
batchmate events list [--limit <n>]
```

`batchmate login` prompts you to paste your Recurse Center calendar feed URL (found at <https://www.recurse.com/settings/calendar>) and saves it to a `settings.json` file in your user config directory. You only need to do this once.

`batchmate events list` shows upcoming and ongoing events from the Recurse Center calendar, including their start and end times (in your local time zone) and Zoom links. By default it shows up to 10 events; use `-l`/`--limit` to change that:

```
batchmate events list --limit 5
```

If you haven't run `batchmate login` yet, `batchmate events list` will tell you to do so.

## Authentication

`batchmate join` relies on your browser for authentication. It opens the room's recurse.com link in your default browser, and recurse.com checks that you're signed in to your Recurse account before redirecting you to the Zoom link. If you aren't signed in, you'll be redirected to the recurse.com login page. You don't need to link your account to `batchmate` to use this command.

`batchmate events` accesses your Recurse Center calendar via an iCalendar URL with your unique access token. You'll need to run `batchmate login` and enter your unique calendar URL before you can run this command. Treat this URL like a password: anyone with it can read your calendar.

## Installation

Requires [Rust](https://rustup.rs) (edition 2024, so a recent stable toolchain).

```
git clone https://github.com/veeara282/batchmate-cli.git
cd batchmate-cli
cargo install --path .
```

This builds the project in release mode and installs the `batchmate` binary to `~/.cargo/bin` (`%USERPROFILE%\.cargo\bin` on Windows), which rustup adds to your `PATH`. If `batchmate` isn't found, make sure that directory is on your `PATH` and open a new terminal.

Verify with:
```
batchmate --help
```

To update, pull the latest changes and re-run `cargo install --path .`. To remove it, run `cargo uninstall batchmate-cli`.

## Development

Run without installing:
```
cargo run -- join <room>
```

## Acknowledgments

The Batchmate CLI was inspired by the [original Batchmate app](https://batchmate.app/) – see its source code [on GitHub](https://github.com/rc-batchmates/batchmate-app).
