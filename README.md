# ir200-access-rs

SysEx codec, CLI and WASM bindings for the **BOSS IR-200** Amp & IR Cabinet
pedal. It is one of the device "access" crates behind the
[able-midi](https://github.com/ragb/able-midi) accessible editor, built on
[`midi-access-kit`](https://github.com/ragb/midi-access-kit).

The IR-200 has no published MIDI implementation. The parameter map comes from
BOSS IR Loader's own address map and is generated into Rust rather than typed
by hand. Value names come from the owner's manual and the BOSS sound list.
Verified on a real pedal running firmware 1.10: identity, every area dumped and
re-encoded byte-exact, write-and-verify, saving to a stored memory across a
power cycle, and live change reports. See [`docs/sysex-notes.md`](docs/sysex-notes.md).

## Crates

| Crate | What it is |
|-------|-----------|
| **`ir200-core`** | Pure codec (`Device` impl, typed models, SysEx framing, live change descriptions). No I/O; compiles for `wasm32-unknown-unknown`. |
| **`ir200`** | CLI (`ir200`) and MCP server (`ir200-mcp`): the generic `midi-access-cli` / `midi-access-mcp` engines dispatched through `Device for Ir200`. |
| **`ir200-wasm`** | `wasm-bindgen` + TypeScript bindings for the editor. |

## Areas

| Area | Address | Contents |
|------|---------|----------|
| `setup` | `00 00 00 00` | current memory number (0-based) |
| `system` | `10 00 00 00` (19 blocks) | modules, phones, global EQ A/B, output select, memory extent, USB, MIDI, preferences, input settings, footswitch / CTL / EXP defaults |
| `edit` | `20 00 00 00` (17 blocks) | the temporary memory: name, amp, noise suppressor, send/return, cabinets A/B, ambience, EQ A/B, input setting, footswitch / CTL / EXP |
| `pc-map` | `50 00 00 00` | memory (1–128) for each Program Change |

Stored memories 1–128 live at `20 10 00 00` + (n − 1) × `00 10 00 00`. The CLI
reaches them through `--store` and `load`; the wasm layer reads and writes any
of them directly, and also reads and renames user IR slots.

## CLI

```
ir200 dump <area> -o <file>             # read an area off the pedal into YAML
ir200 sync <area> -i <file> [--verify]  # write it back (edit = the temporary memory)
ir200 sync edit -i <file> --store <n>   # also save it to memory n (1-128)
ir200 load <n>                          # select memory n
ir200 show | lint | diff | schema | catalog | resolve   # offline
ir200 identity | ports
```

Value names work anywhere a number does (`amp_type: X-HI GAIN`,
`cab_type: 412 BRIT STK DYN57-S`, `rx_channel: OFF`); `ir200 resolve` shows the
numbers they become.

## MCP server

`ir200-mcp` serves the parameter metadata, catalogs, schemas and factory
defaults to an assistant over the Model Context Protocol, plus `dump` / `sync`
when given a port:

```sh
cargo install --path ir200 --bin ir200-mcp
claude mcp add ir200 -- ir200-mcp --port IR-200
```

```json
{ "mcpServers": { "ir200": { "command": "ir200-mcp", "args": ["--port", "IR-200"] } } }
```

`sync edit` writes the temporary memory only; saving to memory 1�128 needs the
explicit `store` argument. `--read-only` serves everything but refuses writes.
The server never writes the `7F` command area (factory reset, IR data init,
patch write, temp clear).

## Wire protocol

Roland SysEx, 5-byte model ID `00 00 00 00 06`, device ID `10`, RQ1 `11` / DT1
`12`, Roland checksum over address and data. Addresses and sizes are four 7-bit
bytes. The pedal answers an RQ1 with one DT1 per parameter block in range, in
frames of up to 241 bytes. Reads need no handshake. Editor communication mode
(`7F 00 00 01` = 1) makes the pedal push a DT1 for every knob and memory
change; `ir200_core::describe` turns those into labelled, spoken values.

## How the map is generated

```sh
py reference/extract_ir_loader_ui.py "C:\Program Files (x86)\BOSS\IR-200 IR Loader\IR-200 IR Loader.exe" reference/ir-loader-ui
py tools/gen/gen_map.py reference/ir-loader-ui/html/js/config/address_map.js > ir200-core/src/map.rs
```

IR Loader's files are Roland's and are never committed; `map.rs` is.

## Not done yet

Loading new IRs. The pedal stores a converted form of each WAV (filtered float
samples plus a tail model), produced by IR Loader's native code. Reading and
renaming user IR slots works.

## Development

```sh
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets
cd ir200-wasm && wasm-pack build --target web --release
tools/regen-schemas.sh
```

Hardware probes live in `tools/explore/` (Python, `mido`); they never write
anything but the editor-mode switch.

## License

MIT.
