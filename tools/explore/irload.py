"""Load a WAV into an EMPTY user IR slot with our converter, then verify.

    .venv\\Scripts\\python.exe irload.py SLOT WAV [--name NAME] [--channel 0|1]

Writes the same DT1 sequence IR Loader does (docs/ir-conversion.md): name,
file name, coefficients, 8 ms apart. Refuses a slot that is already in use.
Then reads everything back and compares it byte for byte.
"""
import argparse
import json
import time
from pathlib import Path

from ir200probe import Pedal, logger, LOGS, pack, unpack
import irconv

NAME_LEN = 32
FILE_OFF = 0x24  # +00 24
LEN_OFF = 0x20  # +00 20
COEF_OFF = pack(0x0224)  # +02 24
GAP = 0.008


def nibbles(word: int, n: int) -> list[int]:
    return [(word >> (4 * (n - 1 - i))) & 0xF for i in range(n)]


def from_nibbles(d: bytes) -> int:
    v = 0
    for x in d:
        v = (v << 4) | (x & 0xF)
    return v


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("slot", type=int)
    ap.add_argument("wav")
    ap.add_argument("--name")
    ap.add_argument("--channel", type=int, default=0)
    a = ap.parse_args()
    assert 1 <= a.slot <= 128

    wav = Path(a.wav)
    values = irconv.convert(str(wav), channel=a.channel)
    words = irconv.to_words(values)
    assert len(words) == 3354
    name = (a.name or wav.stem[:16])[:NAME_LEN]
    assert name.isascii()
    fname = wav.name.encode("utf-8")[:127]

    log = logger(LOGS / "irload.log")
    p = Pedal(log)
    base = pack(0x68000000) + (a.slot - 1) * pack(0x00020000)
    try:
        p.identity()
        p.editor_on()
        _, fl = p.read(unpack(base + LEN_OFF), 4)
        used = from_nibbles(fl)
        log(f"slot {a.slot} file-name length before: {used}")
        if len(fl) != 4 or used != 0:
            raise SystemExit(f"slot {a.slot} is not empty (length {used}); refusing to write")

        # 1. name
        p.dt1(unpack(base), list(name.encode().ljust(NAME_LEN, b" ")))
        time.sleep(GAP)
        # 2. file name, two nibbles per byte, 120 bytes per message
        nib = [n for b in fname for n in nibbles(b, 2)]
        for i in range(0, len(nib), 240):
            p.dt1(unpack(base + FILE_OFF + i), nib[i:i + 240])
            time.sleep(GAP)
        # 3. coefficients, 30 values (240 bytes) per message
        cn = [n for w in words for n in nibbles(w, 8)]
        for i in range(0, len(cn), 240):
            p.dt1(unpack(base + COEF_OFF + i), cn[i:i + 240])
            time.sleep(GAP)
        log(f"wrote name {name!r}, file {fname!r}, {len(words)} coefficients")
        time.sleep(0.5)

        # Verify.
        _, rn = p.read(unpack(base), NAME_LEN)
        _, fl = p.read(unpack(base + LEN_OFF), 4)
        n = from_nibbles(fl)
        _, fn = p.read(unpack(base + FILE_OFF), 2 * len(fname))
        got = bytearray()
        total = len(cn)
        while len(got) < total:
            k = min(240, total - len(got))
            _, d = p.read(unpack(base + COEF_OFF + len(got)), k, quiet_ms=120)
            if not d:
                break
            got += d
        back = [from_nibbles(got[8 * i:8 * i + 8]) for i in range(len(got) // 8)]
        ok_name = rn == name.encode().ljust(NAME_LEN, b" ")
        ok_file = list(fn) == nib
        same = sum(x == y for x, y in zip(back, words))
        log(f"name read back: {rn!r} {'OK' if ok_name else 'MISMATCH'}")
        log(f"file-name length read back: {n} (file name is {len(fname)} bytes)")
        log(f"file name read back: {'OK' if ok_file else 'MISMATCH'}")
        log(f"coefficients identical: {same}/{len(words)} (read {len(back)})")
        json.dump({"slot": a.slot, "name": rn.decode("ascii", "replace"), "len": n,
                   "coef": back}, open(LOGS / f"irload-slot{a.slot}.json", "w"))
    finally:
        p.close()


if __name__ == "__main__":
    main()
