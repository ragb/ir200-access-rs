# ir200-access-rs: handoff prompt

Start a session in `C:\Users\ruian\Documents\dev\ir200-access-rs` and paste
everything below the line.

---

Build **`ir200-access-rs`**: a SysEx codec, CLI and WASM bindings for the
**BOSS IR-200** (Amp & IR Cabinet pedal), as another device "access" crate on
`midi-access-kit`, then add the IR-200 to the able-midi editor. Follow the
existing crates exactly; the IR-200 is new only in its address map.

## Where things are

| What | Where |
|---|---|
| This crate (new, empty except `reference/` and this file) | `C:\Users\ruian\Documents\dev\ir200-access-rs` |
| Template to copy: layout, Cargo workspace, CLI, wasm, docs, CI | `..\re202-access-rs` (github.com/ragb/re202-access-rs) |
| Second template, closest in shape (BOSS, setup/system/temporary/stored areas, PC map) | `..\md-500-access-rs` |
| Shared kit: `Device` trait, codec helpers, generic CLI engine, MCP server | `..\midi-access-kit` (github.com/ragb/midi-access-kit, pinned by tag in each crate's workspace Cargo.toml) |
| Other access crates for reference | `..\gr55-access`, `..\ck-access-rs`, `..\minilog-access-rs`, `..\ml10x-access-rs` |
| The editor to integrate into: able-midi (SvelteKit, loads each device's wasm) | `..\midi-ccess` (local folder name; its remote is github.com/ragb/able-midi) |
| BOSS IR-200 IR Loader, installed | `C:\Program Files (x86)\BOSS\IR-200 IR Loader\` |
| Python prototype that speaks pedal changes (works the protocol end to end) | `reference\ir200_reader.py` |
| IR Loader's own JS, extracted (Roland's code: read it, never commit it) | `reference\ir-loader-js\` |

Read `..\re202-access-rs\README.md`, its `docs\sysex-notes.md` and
`..\md-500-access-rs\README.md` before writing code, and match their
structure: `ir200-core` (pure codec, no I/O, wasm-compatible), `ir200` (CLI =
`midi_access_cli::run::<Ir200>()`), `ir200-wasm`, `schemas\`, `docs\sysex-notes.md`
as a running discovery log, `tools\explore\` for Python probes, fixtures from
real device dumps under `ir200-core\tests\fixtures\`.

## Source of truth

There is no published MIDI Implementation for the IR-200. The spec is IR
Loader's own address map. IR Loader is a CEF app whose UI is HTML/JS stored
as a plain, unencrypted zip inside `IR-200 IR Loader.exe`.
`reference\extract_ir_loader_ui.py` re-extracts it (output is gitignored).
The files that matter, already copied to `reference\ir-loader-js\`:

- `address_map.js`: every parameter: address, encoding, offset, init, min, max, name.
- `product_setting.js`: model ID, device ID, family code, timings, IR data addresses.
- `address_const.js`: identity strings and the `7F xx xx xx` command addresses.
- `midi_controller.js`, `converter.js`, `constant.js`: framing, checksum, address 7-bit packing.
- `midi_connect_controller.js`: the connect handshake.
- `parameter.js`, `midi_observe_controller.js`: how inbound DT1s update the model.

`reference\mkparams.py` turns `address_map.js` into a flat table
(`reference\params.py`, 249 parameters across SETUP, SYSTEM and the
temporary memory). Generate the Rust tables from `address_map.js` the same
way rather than hand-typing them.

The installed folder also has `IR-200 IR Loader.pdb` (full debug symbols) if
the native MIDI layer ever matters; so far it hasn't.

Do not go down the firmware route. The system program (`ir200_sys_v110.zip`,
files `IR-200_1.BIN` and `IR-200_2.BIN`, header `ESC4`, names `BG774_1_UPD`
and `BG774_2_UPD`) is encrypted. This project is MIDI only.

## Protocol (from IR Loader, not yet hardware-verified)

Framing is standard Roland, the same as the RE-202 and MD-500:

| Field | Value |
|---|---|
| Header | `F0 41 <dev> 00 00 00 00 06 <cmd>`: model ID `00 00 00 00 06` (5 bytes) |
| Device ID | `10` default; taken from byte 2 of the identity reply. Inbound DT1 may also use `7F`. |
| RQ1 | `11`, 4-byte address + 4-byte size, both 7-bit per byte |
| DT1 | `12`, 4-byte address + data |
| Checksum | `(128 - (sum of address + data/size bytes) % 128) & 0x7F` |
| Identity | Request `F0 7E 7F 06 01 F7`. Reply starts `F0 7E <dev> 06 02 41 06 04` (family code `06 04`, family number `00 00`). |

Address arithmetic: addresses are four 7-bit bytes. To add an offset, pack to
28 bits (`b0<<21 | b1<<14 | b2<<7 | b3`), add, unpack. IR Loader calls these
`nibble()` and `_7bitize()`. The kit may already have this; check `codec`.

Connect handshake, as IR Loader does it:

1. Identity request; check the reply; adopt its device ID.
2. RQ1 `7F 00 00 00` size 1: editor communication level. IR Loader expects `0`.
3. DT1 `7F 00 00 01` = `01`: **editor communication mode on**.
4. Read whatever is needed.
5. On disconnect, DT1 `7F 00 00 01` = `00`. Always do this, including on errors and Ctrl+C.

IR Loader paces outgoing messages as if over a 312,500 bps link (`baudrate:
31250*10`, gap = bytes × 10000 / baud ms). It uses a 15 s RQ1 timeout and
waits up to 5 s for multi-message DT1 replies.

### Value encodings (`constant.js`)

| Name | Bytes | Decoding |
|---|---|---|
| `INTEGER1x7` | 1 | the byte |
| `INTEGER2x4` | 2 | nibbles, high first: `(a<<4)\|b` |
| `INTEGER4x4` | 4 | 4 nibbles, high first |
| `INTEGER8x4` | 8 | 8 nibbles (IR coefficients) |
| `INTEGER2x7` | 2 | `(a<<7)\|b` |
| number | n | ASCII text, space-padded |

Displayed value = raw − `ofs`. So `ofs: 20` is a ±20 dB gain stored as
0–40, and `ofs: -1` is a 1-based number stored 0-based.

### Address map

| Area | Address | Contents |
|---|---|---|
| setup | `00 00 00 00` | current memory number (`INTEGER2x4`, 0–127) |
| system | `10 00 00 00` | sub-blocks at +`00 00`, `10 00` … `70 00`, then `01 00 00` … `02 20 00`: module on/off (amp, cab, ambience); phones; global EQ link + EQ A/B (parametric + 10-band graphic, ±20); output link + output A/B type (0–10); memory extent; USB levels and loopback; MIDI (thru, rx/tx channel, PC/CC in/out, CC numbers per function); preferences (auto off, LCD contrast, display mode, panel lock, cab display, play cursor, EXP hold); input presets (10 × 0–20); control defaults for FSW down/up, CTL1/2, EXP |
| temporary memory (edit buffer) | `20 00 00 00` | name (16 chars); amp `20 00 10 00` (type 0–11, gain 0–120, level, bass, middle, treble, gain switch 0–2, solo on/off, solo level); NS `20 00`; send/return `30 00`; cab `40 00` (link, out mode, adjust); cab A `50 00` and cab B `60 00` (cab type `INTEGER4x4` 0–336, mic level, direct mix); ambience `70 00`; EQ `01 00 00` / A `01 10 00` / B `01 20 00`; input pref `01 30 00`; FSW down/up, CTL1/2 `01 40 00` … `01 70 00`; EXP `02 00 00` |
| stored memories 1–128 | `20 10 00 00` … `30 00 00 00` | same layout as temporary. Memory n base = `0x20000000 + ((n/8)<<24) + ((n%8)<<20)` in wire form |
| cab info | `40 00 00 00` | 337 favourite flags, one per cab type |
| MIDI PC map | `50 00 00 00` | 128 × `INTEGER2x4`: which memory each program change selects |
| IR info | `60 00 00 00` | 128 slot indices (IR slot order) |
| IR data | `68 00 00 00`, one block per user IR: slot i (0-based) base = unpack(pack(`68 00 00 00`) + i × pack(`00 02 00 00`)), as `ir_data_loader.js` computes it | name (32 chars) at +0, file name length + name at +`00 20`, coefficients (3354 × `INTEGER8x4`) at +`02 24` |
| commands | `7F 00 00 00` | `…00` comm level, `…01` comm mode, `…02` running mode (1 play, **4 factory reset**), `…03` comm revision, `7F 00 01 00` patch select, `…01 02` patch stored, `…01 04` patch write, `7F 01 00 03` patch temp clear, `7F 00 02 00` IR data init |

The map has no value names (amp types, cab types, ambience types, output
types, send/return modes, footswitch functions); IR Loader never shows them.
Get them from the IR-200 owner's manual / parameter guide and put them in
catalogs, as the other crates do.

## Safety rules while exploring

- Reads (RQ1) are always fine. The only write needed for reading is the editor-mode switch, and it must be switched off again.
- Never send DT1 to `7F 00 00 02` (running mode: `04` is factory reset), `7F 00 02 00` (IR data init), `7F 00 01 04` (patch write) or `7F 01 00 03` (temp clear) without asking me first.
- Writing to the temporary memory is fine once reads round-trip; writes to stored memories and IR data only with `--verify` and after asking.
- On Windows only one program can open the pedal's MIDI port: IR Loader must be closed.

## Open questions to settle on hardware first

Log each answer in `docs\sysex-notes.md`, as in the RE-202 notes:

1. Does the identity reply match `F0 7E 10 06 02 41 06 04 00 00 …`? What are the revision bytes?
2. Does RQ1 on a whole area return one DT1 or several? Are oversized requests truncated, as on the RE-202?
3. **In editor mode, does the pedal push a DT1 for each panel change** (knobs, memory up/down, menu settings)? This decides what live screen-reader feedback able-midi can give. The Python prototype logs every inbound message, so use it to check.
4. On a memory change, does the pedal send the setup patch number, a full temporary-memory dump, a program change, or several of these?
5. Do system-area changes made on the pedal's menus arrive as DT1 too?
6. Stored memory addressing: confirm the formula above by reading memory 1, 8, 9 and 128.

## Then: able-midi

Look at how `..\midi-ccess` loads the RE-202 and MD-500 wasm (`npm run
fetch-wasm`, `scripts\`, `src\`). Add the IR-200 the same way. If question 3
comes back yes, add live change announcements in the editor's screen-reader
output, as the prototype does: memory number and name on memory change,
"Amp gain 72" on a knob turn.

## First steps

1. Scaffold the workspace from re202-access-rs (pin the same kit tag). Generate the parameter tables from `reference\ir-loader-js\address_map.js`.
2. Implement `Device for Ir200` with areas `setup`, `system`, `edit` (temporary memory) and `pc-map`; per-slot memories and IR names in the wasm layer, as the RE-202 and MD-500 do.
3. `tools\explore\`: port the handshake and logging from `reference\ir200_reader.py`; capture fixtures for each area.
4. Round-trip tests against the fixtures, then CLI `dump` / `show` / `diff` on the real pedal.
5. able-midi integration.
