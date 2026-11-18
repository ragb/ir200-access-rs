# IR-200 user IRs: WAV conversion and loading

How BOSS IR-200 IR Loader (1.0.0 build 72, Windows) turns a WAV impulse
response into the 3354 values it stores in a user-IR slot, and how it sends
them to the pedal. Worked out from IR Loader's JavaScript and its native
converter (read with the shipped debug symbols), then reimplemented in
`tools/explore/irconv.py`.

Status labels used below:

- **verified**: reproduced bit for bit against a real slot dump (slot 1 of the
  user's pedal, `tools/explore/logs/irs.json`, converted from a 96 kHz
  24-bit mono WAV).
- **read**: taken from the program code, not yet checked against data.
- **inferred**: an interpretation, not directly visible in the code.

## Summary

The pedal does not store the WAV. It stores a **sparse FIR** (1640 taps
picked from the first 4080 samples at 96 kHz) plus a **tail model** (24
delayed, scaled copies of the end of that FIR) that approximates the IR up to
0.5 s. IR Loader works at 96 kHz internally.

```
WAV ─► drop leading silence, pick channel, cap at 0.5 s
    ─► resample to 96 kHz (truncated sinc)              (skipped for 96 kHz)
    ─► normalise: mean |FFT| over 200–300 Hz = 1
    ─► half-band low-pass, keep even taps only
    ─► keep the 1640 largest of the first 4080 taps
    ─► tail model: 24 × (delay, source position, gain)
    ─► 3354 float32 values
```

## WAV requirements

From the WAV reader and the converter entry point (**read**):

| Property | Accepted | Otherwise |
|---|---|---|
| Container | RIFF/WAVE with `fmt ` and `data` chunks (other chunks, e.g. `JUNK`, are skipped) | "file error" / "unsupported" |
| Format tag | 1 (integer PCM) or 3 (IEEE float). **`WAVE_FORMAT_EXTENSIBLE` (0xFFFE) is rejected**, even when it wraps plain PCM. | "unsupported data" |
| Bit depth | PCM 8, 16, 24, 32; float 32 | "unsupported data" |
| Channels | 1 or 2. For stereo the user picks L or R; only that channel is converted. | "unsupported data" |
| Sample rate | any; resampled to 96 kHz | |
| Length | any; only the first 0.5 s after the leading silence is used (at most 48000 samples at 96 kHz) | |

Samples are scaled to ±1.0 full scale (8-bit: (b−128)/128; 16-bit: /2^15;
24-bit: /2^23; 32-bit int: /2^31; float as is). Only the leading-silence test
depends on this scale; the rest of the chain is scale-invariant.

The UI first asks the native side for a "stat" of the file (format, channels,
bits, rate, length, plus ACID-style tempo fields from a generic Roland WAV
utility). The UI only uses `result` (non-zero = readable) and `channels` (1 =
convert directly, 2 = ask L/R, anything else = "unsupported data"). The other
rejections happen during conversion. The stat routine itself was not fully
decoded.

## Algorithm

All arithmetic is IEEE double; the result is rounded to float32 at the end.
Constants are given exactly as the program uses them. Note that IR Loader's
π is the literal `3.14159265358979` (slightly less than `math.pi`) and its FFT
tables use `6.283185307` for 2π; `irconv.py` uses the same π.

### 1. Trim and select (verified for 96 kHz)

- Frames are scanned from the start; while `|left sample| < 2^-9` (always the
  **first** channel, even if the right channel was chosen) the frame is
  dropped. `skipped` = number of dropped frames.
- `lead_trim = trunc(skipped × 96000 / fs)` is kept for the output.
- Remaining length `n` is capped: if `n > trunc(fs × 48000/96000 + 0.5)` it
  becomes that (0.5 s); else if `n > 48000` it becomes 48000.
- The chosen channel's samples are copied out.

### 2. Resample to 96 kHz (read; not exercised by the verified case)

If `fs == 96000` the samples pass through unchanged. Otherwise
`m = trunc(96000/fs × n + 0.5)`, capped at 48000 (and then `n` is recomputed
as `trunc(fs/2 + 0.5)`). Output sample `k` (0 ≤ k < m) is a truncated-sinc
interpolation, no window, using input samples `j` within ±1024 of
`c = trunc(k·fs/96000 + 0.5)` (clipped to `0 … n−1`):

- **upsampling** (`n ≤ m`): `x = k·fs/96000`,
  `y[k] = (fs/96000) · Σ x_in[j] · sinc((j − x)π)`. Note the extra
  `fs/96000` factor: the level drops by that ratio (it is undone by step 3).
- **downsampling** (`n > m`): `y[k] = Σ x_in[j] · sinc((j·96000/fs − k)π)`,
  no anti-alias gain correction.

with `sinc(a) = sin(a)/a`, `sinc(0) = 1`.

### 3. Normalise (verified)

- FFT size `N = 2^ceil(log2(35·m))` (zero padded; e.g. m = 48000 gives
  N = 2^21).
- `lo = trunc(N/480 + 0.5)`, `hi = trunc(N·0.003125 + 0.5)` (the constants
  are `0.0020833333333333333` and `0.003125`): bins covering 200–300 Hz at
  96 kHz.
- `ref = (hi − lo) / Σ_{lo ≤ k < hi} |X[k]|`, i.e. the reciprocal of the mean
  magnitude in that band.
- The spectrum is multiplied by `ref` and transformed back. That equals
  `y · ref` plus ~1e-16 round-off; the round-off is visible only in tail gains
  that should be zero (step 6). Their FFT is an in-house radix-2 real FFT;
  `irconv.py` uses numpy's, so those ~1e-16 values differ (harmless).

### 4. Half-band filter, even taps (verified)

For every even index `i < m`:

```
h[i] = Σ_{j = max(0, i−100)}^{min(m−1, i+100)} y[j] · sinc((j−i)·π/2) · (0.5 + 0.5·cos((j−i)·π/100))
```

Odd indices are 0. (A Hann-windowed half-band low-pass with cutoff 24 kHz; its
DC gain is about 2, which zeroing the odd taps halves again.) In the stored
data the taps therefore sit on even positions only.

### 5. Pick 1640 taps (verified)

`taps = min(m, 4080)`. Bisection on a magnitude threshold, starting with
`lo = 0`, `hi = 1`:

```
loop:
  th = (hi + lo) / 2
  keep[i] = |h[i]| > th  for i < taps;  count them
  count == 1640 → done
  count > 1640  → lo = th   else hi = th
  hi − lo < 1e-15 → keep[i] = |h[i]| > hi; done
  th < 1e-15 → done
```

Taps not kept (below `taps`) are set to 0. Taps at or above `taps` are left as
they are; the tail model reads them.

### 6. Tail model (verified)

`last` = highest index `< taps` with a non-zero tap.

Two kinds of zero crossing, on the even grid (`x`, `x+2`):

- rising at `x+2`: `h[x] < 0` and `h[x+2] ≥ 0`
- falling at `x+2`: `h[x] > 0` and `h[x+2] ≤ 0`

Scanning back from `x = last−2` (at most 2880 steps), `A` = the first rising
crossing found, `B` = the first falling one (0 if none). The later of the two
is `D0` (`A` if `A > B`, else `B`), and its kind is used for everything below.

For segment `r = 0 … 23`, with `base = last − 2880 + 72r`:

1. Search forward over `x+2 ∈ [base, last)` for the first crossing of the
   chosen kind; call its position `S[r]` (the **source**). If none, the
   segment keeps delay (if already set), source 0 and gain 0.
2. `delay[0] = D0`. When `S[r]` is found and `r < 23`:
   `delay[r+1] = D0 + delay[r] − S[r]`, clipped to `m` (the IR length).
3. The segment is `L = D0 − S[r]` samples long (the stretch of FIR from the
   crossing to the end of the stored FIR). Gain:
   `gain[r] = Σ_{k<L, delay[r]+k<m} |h[delay[r]+k]| / Σ_{k<L} |h[S[r]+k]|`
   (sum of absolute values of the real IR at the segment's place in the tail,
   over the same sum for the FIR stretch it will be copied from). If `L = 0`,
   gain 0.

So the tail beyond `D0` is modelled as back-to-back copies of the FIR's own
end section `h[S[r] … D0)`, each placed at `delay[r]` and scaled by
`gain[r]`. The search window start moves 72 samples later per segment, so the
copies tend to get shorter. (**inferred**: this is what the pedal plays; it is
the natural reading of the data, but the DSP side is firmware and was not
examined.)

### 7. Output layout (verified)

3354 float32 values, little-endian in IR Loader's temporary file, sent as
`INTEGER8x4` (the 32-bit pattern as 8 nibbles, high nibble first):

| Index | Count | Content |
|---|---|---|
| 0 | 1640 | FIR tap values: every kept non-zero `h[i]` with `i < min(taps, D0)`, in index order; padded with 0.0 |
| 1640 | 1640 | Their positions: `float32(2^-14) × float32(i)` (single-precision product); padding 4080 × 2^-14 = 0.2490234375 |
| 3280 | 24 | Tail delays: `delay[r] − 2` if `delay[r] > 2`, else 1.0 |
| 3304 | 24 | Tail sources: `S[r] × 2^-14` |
| 3328 | 24 | Tail gains: `gain[r]` |
| 3352 | 1 | `lead_trim` (leading silence removed, in 96 kHz samples) |
| 3353 | 1 | 1.0 |

Notes:

- Taps after `D0` are dropped even if they were among the 1640, so the FIR
  block can hold fewer than 1640 values (slot 1 has 1639).
- Positions are in 96 kHz samples scaled by 2^-14 (4080 × 2^-14 < 0.25).
- Values that are exactly 0 are never written to the FIR block, even if kept.

### Constant table in the binary (`testData`)

The converter class has a static 3354-value float array (`testData`) that no
code references. Its layout matches the table above: FIR value 1.0 in the
first entry, all other FIR values 0, every position the padding value
0.2490234375, a set of tail delays and sources from some real IR, all tail
gains 0, and 0.0 in the last two values. It is a hand-made placeholder (a
pass-through response), not an input/output test pair for the converter, so
it cannot validate the algorithm. A copy is in
`tools/explore/logs/irconv/testData.npy` (gitignored).

## Native call interface (read)

IR Loader is a CEF app. The JavaScript (`ir_data_loader.js`) talks to native
code through `$native.app.control(<command>:<json>)`, and the native side
answers by firing an app event whose argument is a JSON string:

| JS call | Sent | Reply event / fields |
|---|---|---|
| `stat(path, cb)` | `wavstat:{"path": …}` | event `wavstat`: `result`, `format`, `channels`, `bits`, `sampleRate`, `samples`, … |
| `load(path, ch, cb)` | `convert:{"path": …, "ch": 0 or 1}` | event `convert` with `load`: `result` (non-zero = ok), `path` (output file), or `load: "unsupported"` / `"notfound"` |
| `progress(cb)` | `convert:{"req":"progress"}` | `progress`: 0–100 |
| `cancel(cb)` | `convert:{"req":"cancel"}` | |

The conversion runs on a worker thread and writes the 3354 float32 values to
`%TEMP%\RLD00000.tmp`. The JS then reads that file (`$native.fs.readData`),
assembles little-endian 32-bit words and sends them. `ch` is the channel
index for stereo files (0 = L, 1 = R); for mono it is 0.

## MIDI sequence to load a slot (read)

`irDataLoader.apply(index, name, filename, path)`, index 0-based. Slot base
`B = 68 00 00 00 + index × 00 02 00 00` in 7-bit address arithmetic. All
messages are DT1 (`F0 41 10 00 00 00 00 06 12 <addr4> <data> <sum> F7`).

1. **Name**: one DT1 at `B`, 32 bytes: the name, space padded, each character
   as its char code (no filtering on this path; the rename dialog limits
   input to printable ASCII and line feed). On load the name is the WAV file
   name without extension, cut to 16 characters, prefixed with `[L]` or `[R]`
   for stereo files.
2. **File name**: DT1s at `B + 00 24`: the file name (with extension) as UTF-8,
   shortened by whole characters until it is under 128 bytes, each byte as two
   nibbles (high first). 240 nibbles (120 bytes) per message, the address
   advancing by 240 per message. **The length field at `B + 00 20` is not
   written**; it is read back afterwards, so the pedal must set it itself
   (**inferred**: slot 1 has the right length, 44).
3. **Coefficients**: DT1s at `B + 02 24`: each value as 8 nibbles, 30 values
   (240 bytes) per message, the address advancing by 240 (7-bit carry) per
   message: 112 messages, the last carrying 24 values.
4. All messages are queued and sent one every **8 ms** (`setTimeout(80/10)`).
   No reply is awaited for these DT1s.
5. Afterwards IR Loader re-reads the slot's name and file name with RQ1 to
   refresh the list.

It does **not** send IR data init (`7F 00 02 00`) before loading, does not
touch IR info (`60 00 00 00`), and sends no other `7F` command.

**Delete** is the only use of IR data init: DT1 `7F 00 02 00` with two data
bytes `(index >> 7) & 7F, index & 7F` (0-based slot), and IR Loader waits for
the pedal to answer with a DT1 at the same address before re-reading the
names. **Rename** is step 1 alone.

## Verification

Slot 1 of the user's pedal ("TF 65 TWIN 2X12 JENSN C12N 57 U87 70-30.wav")
against the three copies of that IR the user has:

| Source WAV | Result |
|---|---|
| 96 kHz | **3338 of 3354 values bit-identical.** The 16 others are tail gains of segments that fall in silence: ~4e-16 in the pedal, ~5e-16 in ours (FFT round-off; max abs difference 1.3e-16). FIR values and positions, delays, sources, lead trim (3) all exact. |
| 48 kHz | not a match (lead trim 4 vs 3, FIR differs by up to 0.017) |
| 44.1 kHz | not a match (lead trim 2 vs 3) |

So slot 1 was loaded from the 96 kHz file, and steps 1 and 3–7 are verified.
Step 2 (resampling) only runs for other rates and is implemented from the code
alone.

Synthetic checks (`irconv.py` on generated 16-bit WAVs):

- 96 kHz unit impulse of 0.5 after 5 zero samples: lead trim 5, normalised to
  1.0 at tap 0, the other half-band even taps are ~1e-17 so bisection stops
  early (10 non-zero taps, all but the first negligible).
- 48 kHz unit impulse: 1073 taps; first value 0.996 (the ±1024-tap truncated
  sinc ripples).
- 96 kHz exponentially decaying noise: 1639 taps, tail gains decreasing
  0.38, 0.15, 0.06, 0.03 …

## Open points

- **Resampling (step 2)** is unverified. One experiment settles it: load the
  same IR's 48 kHz (or 44.1 kHz) file into a spare slot with IR Loader, dump
  that slot, and run
  `python tools/explore/irconv.py <that wav> --compare logs/irs.json --slot <n>`.
- Whether the pedal sets the file-name length itself (step 2 of the MIDI
  sequence) needs a write test.
- The exact behaviour of the stat routine (which files the UI rejects before
  converting) was not fully decoded; the conversion-time rules above are.
- What the pedal's DSP does with the tail model is inferred from the data.

## How this was obtained

The debug symbols name the converter classes and give their field layouts
(e.g. 24-entry arrays for delays, sources and gains). The symbol addresses
are slightly out of step with the shipped executable (by 0x30–0x1050 bytes
in places), so functions were located through call targets rather than the
symbol table. Disassembly listings and extracted tables are kept locally under
`tools/explore/logs/irconv/` (gitignored, not to be committed).
