r"""Load a list of IRs into EMPTY user IR slots, verifying each one.

    .venv\Scripts\python.exe irbulk.py logs\ir-picks.json [--dry-run] [--only 9-20]

Picks file: [{"slot", "name", "source", ...}], source relative to
D:/impulse responses, "<zip>!<member>" for files inside zips. WAVs are read
in place (zips are never extracted) and converted by the library converter
(target/release/examples/irconv.exe, WAV bytes on stdin).

Safety: every slot is read first and skipped unless empty (or --replace is
given, which overwrites slots in use). Each load is read
back in full; results go to logs/irbulk-results.json, and slots already
loaded and verified there are skipped on a re-run.
"""
import argparse
import json
import subprocess
import time
import zipfile
from pathlib import Path

from ir200probe import Pedal, logger, LOGS, pack, unpack

ROOT = Path("D:/impulse responses")
CONV = Path(__file__).resolve().parents[2] / "target" / "release" / "examples" / "irconv.exe"
NAME_LEN = 32
GAP = 0.008
RESULTS = LOGS / "irbulk-results.json"


def read_source(src: str) -> bytes:
    if "!" in src:
        z, member = src.split("!", 1)
        with zipfile.ZipFile(ROOT / z) as zf:
            return zf.read(member)
    return (ROOT / src).read_bytes()


def convert(wav: bytes) -> list[int]:
    r = subprocess.run([str(CONV), "-"], input=wav, capture_output=True)
    if r.returncode:
        raise ValueError(r.stderr.decode(errors="replace").strip())
    return [w & 0xFFFFFFFF for w in json.loads(r.stdout)]


def nibbles(word: int, n: int) -> list[int]:
    return [(word >> (4 * (n - 1 - i))) & 0xF for i in range(n)]


def from_nibbles(d) -> int:
    v = 0
    for x in d:
        v = (v << 4) | (x & 0xF)
    return v


def read_run(p: Pedal, addr: int, total: int) -> bytes:
    """Read `total` bytes from packed address `addr`, 240 per RQ1."""
    got = bytearray()
    while len(got) < total:
        k = min(240, total - len(got))
        _, d = p.read(unpack(addr + len(got)), k, quiet_ms=40)
        if not d:
            break
        got += d
    return bytes(got)


def load_slot(p: Pedal, log, slot: int, words: list[int], name: str, fname: bytes,
              replace: bool = False) -> str:
    base = pack(0x68000000) + (slot - 1) * pack(0x00020000)
    _, fl = p.read(unpack(base + 0x20), 4, quiet_ms=60)
    if len(fl) != 4:
        return "could not read slot"
    if from_nibbles(fl) != 0 and not replace:
        return "slot not empty"
    name_b = list(name.encode("ascii").ljust(NAME_LEN, b" "))
    fn = [n for b in fname for n in nibbles(b, 2)]
    co = [n for w in words for n in nibbles(w, 8)]
    p.dt1(unpack(base), name_b)
    time.sleep(GAP)
    for i in range(0, len(fn), 240):
        p.dt1(unpack(base + 0x24 + i), fn[i:i + 240])
        time.sleep(GAP)
    coef = base + pack(0x0224)
    for i in range(0, len(co), 240):
        p.dt1(unpack(coef + i), co[i:i + 240])
        time.sleep(GAP)
    time.sleep(0.3)
    if list(read_run(p, base, NAME_LEN)) != name_b:
        return "name mismatch"
    if list(read_run(p, base + 0x24, len(fn))) != fn:
        return "file name mismatch"
    back = read_run(p, coef, len(co))
    if list(back) != co:
        bad = sum(a != b for a, b in zip(back, co)) + abs(len(back) - len(co))
        return f"coefficient mismatch ({bad} bytes)"
    _, fl = p.read(unpack(base + 0x20), 4, quiet_ms=60)
    if from_nibbles(fl) != len(fname):
        return f"length not set ({from_nibbles(fl)})"
    return "ok"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("picks")
    ap.add_argument("--dry-run", action="store_true", help="convert only, no MIDI")
    ap.add_argument("--only", help="slot range, e.g. 9-20")
    ap.add_argument("--replace", action="store_true", help="overwrite slots in use")
    a = ap.parse_args()
    picks = json.load(open(a.picks, encoding="utf-8"))
    if a.only:
        lo, hi = (int(x) for x in a.only.split("-"))
        picks = [x for x in picks if lo <= x["slot"] <= hi]
    results = json.load(open(RESULTS)) if RESULTS.exists() else {}

    jobs = []
    for x in picks:
        slot, name = int(x["slot"]), x["name"]
        assert 1 <= slot <= 128 and len(name) <= NAME_LEN
        assert all(0x20 <= ord(c) < 0x7F for c in name), name
        done = results.get(str(slot), {})
        if done.get("result") == "ok" and done.get("source") == x["source"]:
            continue
        try:
            words = convert(read_source(x["source"]))
        except Exception as e:
            print(f"slot {slot}: CONVERT FAILED {e} ({x['source']})")
            continue
        fname = x["source"].replace("!", "/").split("/")[-1].encode("utf-8")[:127]
        jobs.append((slot, name, fname, words, x["source"]))
    print(f"{len(jobs)} to load")
    if a.dry_run:
        return

    log = logger(LOGS / "irbulk.log")
    p = Pedal(log)
    try:
        p.identity()
        p.editor_on()
        for slot, name, fname, words, src in jobs:
            t = time.time()
            r = load_slot(p, log, slot, words, name, fname, a.replace)
            results[str(slot)] = {"name": name, "source": src, "result": r}
            json.dump(results, open(RESULTS, "w"), indent=1)
            print(f"slot {slot:3d} {name:16s} {r} ({time.time() - t:.1f}s)", flush=True)
    finally:
        p.close()


if __name__ == "__main__":
    main()
