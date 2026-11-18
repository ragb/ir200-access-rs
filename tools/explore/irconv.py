"""Convert a WAV impulse response into the 3354 float32 values an IR-200
user-IR slot stores (the coefficient block at slot base + 02 24).

Clean-room reimplementation of what BOSS IR-200 IR Loader computes; see
docs/ir-conversion.md for the algorithm and what has been verified.

    python irconv.py IR.wav                      # summary
    python irconv.py IR.wav --channel 1          # right channel of a stereo file
    python irconv.py IR.wav --compare logs/irs.json --slot 1
    python irconv.py IR.wav --out coef.json      # write the 3354 values

`convert(path)` returns a list of 3354 Python floats, each exactly
representable as float32. `to_words()` turns them into the unsigned 32-bit
patterns that go on the wire (8 nibbles each, high nibble first).
"""

from __future__ import annotations

import argparse
import json
import math
import struct
import sys
from dataclasses import dataclass

import numpy as np

# --- Constants -------------------------------------------------------------

FS = 96000.0              # internal sample rate
MAX_SAMPLES = 48000       # at most 0.5 s at 96 kHz is used
SILENCE = 2.0 ** -9       # leading samples below this (channel 0) are dropped
PI = 3.14159265358979      # IR Loader's literal (not math.pi)
SINC_HALF_WIDTH = 1024    # resampler: taps either side of the centre
NORM_LO = 0.0020833333333333333   # normalisation band: N/480 .. N/320 bins,
NORM_HI = 0.003125                # i.e. 200 Hz .. 300 Hz at 96 kHz
HB_HALF_WIDTH = 100       # half-band filter: taps either side
MAX_TAPS = 4080           # direct FIR span searched (taps at 96 kHz)
NUM_TAPS = 1640           # non-zero FIR taps stored
TAIL_SEGMENTS = 24
TAIL_SEARCH = 2880        # crossing search window before the last tap
TAIL_STEP = 72            # the window start moves this much per segment
POS_SCALE = 2.0 ** -14    # tap positions are stored as index * 2^-14
NUM_COEFS = 2 * NUM_TAPS + 3 * TAIL_SEGMENTS + 2   # 3354

REGIONS = [
    ("fir_values", 0, NUM_TAPS),
    ("fir_positions", NUM_TAPS, 2 * NUM_TAPS),
    ("tail_delays", 2 * NUM_TAPS, 2 * NUM_TAPS + 24),
    ("tail_sources", 2 * NUM_TAPS + 24, 2 * NUM_TAPS + 48),
    ("tail_gains", 2 * NUM_TAPS + 48, 2 * NUM_TAPS + 72),
    ("lead_trim", NUM_COEFS - 2, NUM_COEFS - 1),
    ("constant_1", NUM_COEFS - 1, NUM_COEFS),
]


class UnsupportedWav(ValueError):
    pass


# --- WAV reading -------------------------------------------------------------

@dataclass
class Wav:
    format_tag: int
    channels: int
    rate: int
    bits: int
    frames: np.ndarray   # shape (n, channels), float64, full scale = 1.0


def read_wav(path: str) -> Wav:
    """Read a RIFF/WAVE file the way IR Loader accepts it.

    Accepted: format tag 1 (PCM, 8/16/24/32 bit) or 3 (IEEE float, 32 bit),
    1 or 2 channels. WAVE_FORMAT_EXTENSIBLE (0xFFFE) is rejected, as IR Loader
    does. Any sample rate is accepted.
    """
    with open(path, "rb") as f:
        data = f.read()
    if data[:4] != b"RIFF" or data[8:12] != b"WAVE":
        raise UnsupportedWav("not a RIFF/WAVE file")
    fmt = None
    pcm = None
    pos = 12
    while pos + 8 <= len(data):
        cid = data[pos:pos + 4]
        size = struct.unpack_from("<I", data, pos + 4)[0]
        body = data[pos + 8:pos + 8 + size]
        if cid == b"fmt " and fmt is None:
            fmt = body
        elif cid == b"data" and fmt is not None and pcm is None:
            pcm = body
        pos += 8 + size + (size & 1)
    if fmt is None or pcm is None:
        raise UnsupportedWav("missing fmt or data chunk")
    tag, ch, rate, _avg, align, bits = struct.unpack_from("<HHIIHH", fmt, 0)
    if ch not in (1, 2):
        raise UnsupportedWav(f"{ch} channels")
    n = len(pcm) // align
    raw = np.frombuffer(pcm[:n * align], dtype=np.uint8).reshape(n, align)
    if tag == 3 and bits == 32:
        x = raw.copy().view("<f4").astype(np.float64)
    elif tag == 1 and bits == 8:
        x = (raw.astype(np.float64) - 128.0) * (1.0 / 128.0)
    elif tag == 1 and bits == 16:
        x = raw.copy().view("<i2").astype(np.float64) * (2.0 ** -15)
    elif tag == 1 and bits == 24:
        b = raw.reshape(n, ch, 3).astype(np.int32)
        v = (b[..., 0] << 8) | (b[..., 1] << 16) | (b[..., 2] << 24)
        x = (v >> 8).astype(np.float64) * (2.0 ** -23)
    elif tag == 1 and bits == 32:
        x = raw.copy().view("<i4").astype(np.float64) * (2.0 ** -31)
    else:
        raise UnsupportedWav(f"format tag {tag}, {bits} bit")
    return Wav(tag, ch, rate, bits, x.reshape(n, ch))


# --- Conversion steps --------------------------------------------------------

def trim_and_select(wav: Wav, channel: int) -> tuple[np.ndarray, int]:
    """Drop leading silence (judged on channel 0), pick a channel, limit to
    0.5 s. Returns (samples, lead_trim) where lead_trim is the number of
    dropped samples expressed in 96 kHz samples (truncated)."""
    fs = wav.rate
    left = np.abs(wav.frames[:, 0])
    loud = np.nonzero(left >= SILENCE)[0]
    skipped = int(loud[0]) if len(loud) else len(left)
    count = len(left) - skipped
    lead_trim = int(skipped * FS / fs)
    limit = int(fs * float(MAX_SAMPLES) / FS + 0.5)
    if count > limit:
        count = limit
    elif count > MAX_SAMPLES:
        count = MAX_SAMPLES
    ch = channel if wav.channels == 2 else 0
    return wav.frames[skipped:skipped + count, ch].copy(), lead_trim


def resample_to_96k(x: np.ndarray, fs: int) -> np.ndarray:
    """Truncated-sinc resampler (rectangular window, +/-1024 input taps).
    Upsampling scales the result by fs/96000; downsampling does not."""
    org = len(x)
    if float(fs) == FS:
        return x.copy()
    conv = int(FS / fs * org + 0.5)
    if conv > MAX_SAMPLES:
        conv = MAX_SAMPLES
        org = int(fs * float(MAX_SAMPLES) / FS + 0.5)
    fsd = float(fs)
    down = org > conv
    out = np.empty(conv)
    for n in range(conv):
        pos = n * fsd / FS
        c = int(pos + 0.5)
        lo = max(0, c - SINC_HALF_WIDTH)
        hi = min(c + SINC_HALF_WIDTH, org - 1)
        if lo > hi:
            out[n] = 0.0
            continue
        j = np.arange(lo, hi + 1, dtype=np.float64)
        if down:
            a = (j * FS / fsd - n) * PI
        else:
            a = (j - pos) * PI
        with np.errstate(invalid="ignore", divide="ignore"):
            s = np.where(a == 0.0, 1.0, np.sin(a) / a)
        acc = float(np.sum(s * x[lo:hi + 1]))
        out[n] = acc if down else (fsd / FS) * acc
    return out


def normalise(y: np.ndarray) -> tuple[np.ndarray, float]:
    """Scale so the mean FFT magnitude over 200-300 Hz is 1. The FFT is
    zero-padded to 2^ceil(log2(35 * len)) points."""
    m = len(y)
    ex = math.ceil(math.log(m * 35.0) / math.log(2.0))
    n = int(2.0 ** ex)
    spec = np.fft.rfft(y, n)
    lo = int(n * NORM_LO + 0.5)
    hi = int(n * NORM_HI + 0.5)
    total = 0.0
    for k in range(lo, hi):
        total += math.sqrt(spec[k].real ** 2 + spec[k].imag ** 2)
    ref = (hi - lo) / total
    # IR Loader scales the spectrum and inverse-transforms. Mathematically
    # that is y * ref; doing the round trip reproduces the ~1e-16 FFT noise
    # it leaves in silent stretches (visible only in negligible tail gains).
    return np.fft.irfft(spec * ref, n)[:m], ref


def halfband(y: np.ndarray) -> np.ndarray:
    """Half-band low-pass (sinc(pi k/2), Hann window over +/-100 taps),
    evaluated on even taps only; odd taps are zero."""
    m = len(y)
    k = np.arange(-HB_HALF_WIDTH, HB_HALF_WIDTH + 1)
    a = k * PI
    half = a * 0.5
    with np.errstate(invalid="ignore", divide="ignore"):
        sinc = np.where(half == 0.0, 1.0, np.sin(half) / half)
    win = np.cos(a / 100.0) * 0.5 + 0.5
    out = np.zeros(m)
    pad = np.concatenate([np.zeros(HB_HALF_WIDTH), y, np.zeros(HB_HALF_WIDTH)])
    for i in range(0, m, 2):
        seg = pad[i:i + 2 * HB_HALF_WIDTH + 1]
        out[i] = float(np.sum(seg * sinc * win))
    return out


def sparsify(c: np.ndarray, taps: int) -> None:
    """Bisect a magnitude threshold so exactly NUM_TAPS of the first `taps`
    coefficients survive; zero the rest (in place)."""
    a = np.abs(c[:taps])
    lo, hi = 0.0, 1.0
    while True:
        th = (hi + lo) * 0.5
        keep = a > th
        cnt = int(np.count_nonzero(keep))
        if cnt == NUM_TAPS:
            break
        if cnt > NUM_TAPS:
            lo = th
        else:
            hi = th
        if hi - lo < 1e-15:
            keep = a > hi
            break
        if th < 1e-15:
            break
    c[:taps][~keep] = 0.0


def tail_model(c: np.ndarray, taps: int, m: int):
    """Find the late zero crossings and fit 24 delayed, scaled copies of the
    FIR's end section to the part of the IR beyond the FIR."""
    nz = np.nonzero(c[:taps])[0]
    last = int(nz[-1]) if len(nz) else 0

    def rising(x):   # c[x] < 0 <= c[x+2]
        return x >= 0 and c[x + 2] >= 0.0 and c[x] < 0.0

    def falling(x):  # c[x] > 0 >= c[x+2]
        return x >= 0 and c[x + 2] <= 0.0 and c[x] > 0.0

    def last_crossing(pred):
        for cnt in range(TAIL_SEARCH):
            x = last - 2 - cnt
            if x < 0:
                break
            if pred(x):
                return x + 2
        return 0

    a = last_crossing(rising)
    b = last_crossing(falling)
    pred = rising if a > b else falling
    d0 = a if a > b else b

    delay = [0] * TAIL_SEGMENTS
    source = [0] * TAIL_SEGMENTS
    gain = [0.0] * TAIL_SEGMENTS
    delay[0] = d0
    for r in range(TAIL_SEGMENTS):
        base = last - TAIL_SEARCH + TAIL_STEP * r
        width = TAIL_SEARCH - TAIL_STEP * r
        found = None
        for k in range(width):
            if pred(base + k - 2):
                found = base + k
                break
        if found is None:
            continue
        source[r] = found
        if r + 1 < TAIL_SEGMENTS:
            delay[r + 1] = d0 + delay[r] - found
            if delay[r + 1] > m:
                delay[r + 1] = m
                gain[r] = 0.0
        if d0 - found == 0:
            gain[r] = 0.0
            continue
        num = 0.0
        den = 0.0
        for k in range(d0 - found):
            den += abs(c[found + k])
            idx = delay[r] + k
            if idx < m:
                num += abs(c[idx])
        gain[r] = num / den
    return d0, delay, source, gain


def convert_samples(x: np.ndarray, fs: int, lead_trim: int = 0) -> list[float]:
    y = resample_to_96k(x, fs)
    m = len(y)
    y, _ref = normalise(y)
    c = halfband(y)
    taps = min(m, MAX_TAPS)
    sparsify(c, taps)
    d0, delay, source, gain = tail_model(c, taps, m)

    f32 = np.float32
    values: list[float] = []
    positions: list[float] = []
    for i in range(min(taps, d0)):
        if c[i] != 0.0:
            values.append(float(f32(c[i])))
            positions.append(float(f32(f32(POS_SCALE) * f32(i))))
    values += [0.0] * (NUM_TAPS - len(values))
    positions += [float(f32(POS_SCALE * MAX_TAPS))] * (NUM_TAPS - len(positions))
    out = values[:NUM_TAPS] + positions[:NUM_TAPS]
    out += [float(f32(d - 2)) if d > 2 else 1.0 for d in delay]
    out += [float(f32(float(f32(s)) * POS_SCALE)) for s in source]
    out += [float(f32(g)) for g in gain]
    out += [float(f32(lead_trim)), 1.0]
    assert len(out) == NUM_COEFS
    return out


def convert(wav_path: str, channel: int = 0) -> list[float]:
    """WAV file -> 3354 float32-representable values, as IR Loader stores."""
    wav = read_wav(wav_path)
    x, trim = trim_and_select(wav, channel)
    return convert_samples(x, wav.rate, trim)


def to_words(values: list[float]) -> list[int]:
    return [struct.unpack("<I", struct.pack("<f", v))[0] for v in values]


# --- CLI ---------------------------------------------------------------------

def _load_slot(path: str, slot: str) -> np.ndarray:
    d = json.load(open(path, encoding="utf-8"))
    words = np.array(d[slot]["coef"], dtype=np.int64).astype(np.uint32)
    return words.view(np.float32).astype(np.float64)


def summary(v: list[float]) -> None:
    a = np.array(v)
    n_fir = int(np.count_nonzero(a[:NUM_TAPS]))
    print(f"values: {len(a)}  non-zero FIR taps: {n_fir}  "
          f"peak |tap|: {np.max(np.abs(a[:NUM_TAPS])):.6f}")
    pos = a[NUM_TAPS:2 * NUM_TAPS] / POS_SCALE
    print(f"tap span: {int(pos[0])} .. {int(pos[n_fir - 1]) if n_fir else 0} (96 kHz samples)")
    print("tail delays:", [int(x) for x in a[3280:3304]])
    print("tail sources:", [int(round(x / POS_SCALE)) for x in a[3304:3328]])
    print("tail gains:", [f"{x:.4g}" for x in a[3328:3352]])
    print("lead trim:", a[3352], " last:", a[3353])


def compare(v: list[float], ref: np.ndarray) -> None:
    a = np.array(v)
    print(f"{'region':14s} {'max abs err':>12s} {'bit-exact':>10s}")
    wa, wr = np.array(to_words(v), dtype=np.uint32), ref.astype(np.float32).view(np.uint32)
    for name, lo, hi in REGIONS:
        err = float(np.max(np.abs(a[lo:hi] - ref[lo:hi])))
        same = int(np.sum(wa[lo:hi] == wr[lo:hi]))
        print(f"{name:14s} {err:12.3g} {same:5d}/{hi - lo}")
    print(f"{'all':14s} {float(np.max(np.abs(a - ref))):12.3g} {int(np.sum(wa == wr)):5d}/{len(a)}")


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("wav")
    p.add_argument("--channel", type=int, default=0, help="0 = left, 1 = right (stereo only)")
    p.add_argument("--compare", metavar="IRS_JSON", help="compare with a slot dump (logs/irs.json)")
    p.add_argument("--slot", default="1")
    p.add_argument("--out", help="write the values as JSON")
    args = p.parse_args(argv)
    try:
        v = convert(args.wav, args.channel)
    except UnsupportedWav as e:
        print(f"unsupported WAV: {e}", file=sys.stderr)
        return 2
    summary(v)
    if args.compare:
        compare(v, _load_slot(args.compare, args.slot))
    if args.out:
        json.dump(v, open(args.out, "w"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
