# batchmate-cli
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

Other commands (`batchmate events` etc.) are planned but not implemented yet.

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
