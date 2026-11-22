//! `ir200-mcp` — the IR-200's MCP server.
//!
//! Exposes the IR-200's parameter metadata, value catalogs, JSON Schemas, and
//! factory defaults to an assistant over the Model Context Protocol, and — given
//! a port — lets it read the pedal's areas, edit them, and write them back.
//!
//! ```json
//! { "mcpServers": {
//!     "ir200": { "command": "ir200-mcp", "args": ["--port", "IR-200"] }
//! }}
//! ```
//!
//! Without `--port` only the offline authoring tools are available. `sync` of
//! the `edit` area writes the pedal's temporary memory (`20 00 00 00`), which is
//! what it plays and what a memory change discards; saving to stored memory
//! 1–128 is a separate, explicit `store` argument on that tool, and copies the
//! document into that memory's own address range. Nothing the server sends
//! addresses the `7F` command area (running mode / factory reset, IR data init,
//! patch write, temp clear). `--read-only` refuses writes altogether.

use clap::Parser;
use midi_access_mcp::MidiConfig;

#[derive(Parser)]
#[command(name = "ir200-mcp", version, about = "MCP server for the BOSS IR-200.")]
struct Cli {
    /// MIDI port name substring for both directions (e.g. "IR-200").
    #[arg(long)]
    port: Option<String>,
    /// MIDI input port substring (overrides --port).
    #[arg(long)]
    input_port: Option<String>,
    /// MIDI output port substring (overrides --port).
    #[arg(long)]
    output_port: Option<String>,
    /// Device number (0..=15) — 0 is the IR-200's SysEx device ID 10H.
    #[arg(long, default_value_t = 0)]
    device: u8,
    /// Serve the reference tools and `dump`, but refuse to write to the device.
    #[arg(long)]
    read_only: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    midi_access_mcp::serve_with::<ir200_core::Ir200>(MidiConfig {
        input_port: cli.input_port.or_else(|| cli.port.clone()),
        output_port: cli.output_port.or(cli.port),
        device: cli.device,
        read_only: cli.read_only,
    })
}
