# IR-200 SysEx notes

Running discovery log. The IR-200 has no published MIDI Implementation; the
parameter map is IR Loader's own (`address_map.js`, turned into
`ir200-core/src/map.rs` by `tools/gen/gen_map.py`). Everything marked
**verified** was checked on a real pedal (firmware 1.10) with
`tools/explore/ir200probe.py` and the `ir200` CLI.

## Session 1, 2026-11-17

### Identity (verified)

```
→ F0 7E 7F 06 01 F7
← F0 7E 10 06 02 41 06 04 00 00 00 00 00 00 F7
```

Device ID `10`, family code `06 04`, family number `00 00`, revision bytes all
`00` (firmware 1.10 still reports zeros).

### Editor mode is not needed for reads (verified)

RQ1 to `00 00 00 00` answered before `7F 00 00 01 = 01` was sent. Editor
communication level (`7F 00 00 00`) reads `00`, as IR Loader expects.

Editor mode is what makes the pedal **push** its own changes (below). The CLI
never enables it; the editor does, for live announcements, and turns it off on
disconnect.

### Request sizes (verified)

- An RQ1 is answered with one DT1 **per defined leaf block** inside the
  requested range. Gaps between blocks are skipped, not padded.
- RQ1 `20 00 00 00`, size `00 02 00 0F` (the whole temporary memory span)
  returns all 17 memory blocks as 17 DT1s. One request reads a whole memory.
- RQ1 `10 00 00 00` size 64 returns only the 3-byte MODULE block (the other
  system blocks start beyond 64 bytes).
- Large blocks come back in **241-byte** DT1s: a 256-byte PC-map read arrives
  as 241 + 15. IR coefficient reads return at most 240 bytes per RQ1.
- The CLI reads one block per RQ1 and splits writes at 128 bytes, as IR
  Loader does. `sync --verify` passes for `edit`, `system` and `pc-map`
  (the PC map write is two 128-byte DT1s).

### Stored memories (verified)

Memory *n* (1–128) base = `20 00 00 00` + *n* × `00 10 00 00` (7-bit carry):
memory 1 `20 10 00 00`, 8 `21 00 00 00`, 9 `21 10 00 00`, 128 `30 00 00 00`.
All 33 read memories decoded and re-encoded byte-exact (fixtures).

**Saving** by writing a memory's blocks straight to its stored address works
and survives a power cycle (memory 128 renamed, power cycled, read back, then
restored). The patch-write command `7F 00 01 04` is not needed.

### Current memory (verified)

`SETUP_COM_PATCH_NUMBER` (`00 00 00 00`, two nibbles) is **0-based**: `03 0C` =
60 = memory 61. Writing it (DT1) switches memory; the pedal loads the stored
memory into the temporary memory (unsaved edits are discarded).

### Program Change map (verified)

Entries store the memory number **1-based**: a stock entry reads PC#n = n
(PC#128 = 128). One customised map held `0` for PC#2; meaning unknown.

### Live changes in editor mode (verified)

With editor mode on, every panel action is pushed as a DT1:

| Action | Pedal sends |
|---|---|
| GAIN, LEVEL, BASS, MIDDLE, TREBLE knobs | one DT1 per step, e.g. `20 00 10 01 [3D]` = gain 61 (~100 per second while turning) |
| AMP knob | `20 00 10 00 [08]` (amp type) |
| CABINET knob, A/B link on | `20 00 50 00` and `20 00 60 00`, four nibbles each |
| Opening a knob's edit screen / returning to play | `7F 00 00 02 [02]` / `[01]` (running mode: 2 = edit, 1 = play) |
| Memory up/down (footswitch or MEMORY knob) | `00 00 00 00 [03 0D]` (new memory number), then a Program Change on the transmit channel ~100 ms later. **No memory dump**: read the temporary memory after a change. |

Duplicate DT1s with the same value occur; debounce before announcing.

Menu (system) changes were not tested: the pedal's menus need the display.

### Value names (verified against factory memories 1–32)

Factory memories match the BOSS sound list, which calibrates the numbering:

- **Amp type**: 1 NATURAL, 2 JC-120, 3 TWIN COMBO, 4 DIAMOND AMP, 5 TWEED
  COMBO, 6 X-HI GAIN, 7 BRITISH STACK, 8 BGR UB METAL, 9 NATURAL BASS,
  10 X-DRIVE BASS, 11 CONCERT BASS. 0 has no known name.
- **Cab type**: `1 + 12 × cabinet + mic`. Cabinets in stored order: 112
  NATURAL, 212 TWIN, 212 DIAMOND, 212 JC-120, 410 TWEED, 412 BGR UB, 412 CLASS
  ST, 412 BRIT STK, B118 BAS REF, B215 WIDE, B410 TWEETER, B810 A 81 (BRIT STK,
  B118 and B410 by elimination). Mics: DYN57, DYN421, CND451, RBN121,
  D57+R121, CND87, each -S then -L. Celestion IRs are 145–155 in sound-list
  order. 156–208: unknown (no preset uses them). User IR *n* is 208 + *n*
  (IR Loader: USER IR 1 = 209).
- Cab favourites (`40 00 00 00`, 337 flags) ship set on the first IR of every
  cabinet and several Celestion IRs.

### User IRs

- IR info (`60 00 00 00`, 128 bytes) read `00 01 … 7F`: identity slot order.
- Slot *n* base = `68 00 00 00` + (*n* − 1) × `00 02 00 00`: name (32 chars) at
  +0, file-name length (`INTEGER4x4`, 0 = empty slot) at +`00 20`, file name
  (UTF-8, two nibbles per byte) at +`00 24`, coefficients at +`02 24`.
- Names may contain control bytes (one slot's name has a line feed).
- **Renaming** (verified): one DT1 of the 32 space-padded name bytes at the
  slot base. The pedal takes it immediately and reads back the new name; no
  command or IR-data init is needed. Tested on slot 1 and restored
  (`ir200probe.py irname`).
- **Loading** (verified): name DT1, file-name nibble DT1s, then 112
  coefficient DT1s (30 values each), 8 ms apart, as in
  `docs/ir-conversion.md`. The pedal sets the file-name length itself (43 for
  a 43-byte name), which marks the slot used. Our converter's output for the
  96 kHz TF 65 Twin WAV, written to empty slot 128 by `irload.py`, read back
  bit-identical. Slot 2 was already in use (length 52); `irload.py` refused it.
- **Coefficients are not the WAV samples.** Each slot holds 3354 values
  (8 nibbles each); as IEEE-754 float32 the first ~3270 are a filtered impulse
  (peak ~0.13), followed by ~80 values that look like a tail model: rising
  frequency points 4048 … 47995, gains, near-zero values, and `3.0, 1.0` at
  the end. IR Loader computes this in native code (`$native.app.control`), not
  in its JavaScript. Reproduced in `tools/explore/irconv.py`; see
  `docs/ir-conversion.md`.

### Not yet tested

- Whether the pedal pushes DT1s for menu changes.
- What `PATCH_SELECT` (`7F 00 01 00`) and `PATCH_STORED` (`7F 00 01 02`) do.
