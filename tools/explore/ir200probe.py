"""Hardware probe for the BOSS IR-200, built from reference/ir200_reader.py.

Throwaway exploration code: not shipped. Findings go to docs/sysex-notes.md,
raw dumps to ir200-core/tests/fixtures/.

    .venv\\Scripts\\python.exe ir200probe.py ports
    .venv\\Scripts\\python.exe ir200probe.py read            # settle questions 1, 2, 6; capture fixtures
    .venv\\Scripts\\python.exe ir200probe.py listen 120      # log what the pedal pushes, questions 3-5
    .venv\\Scripts\\python.exe ir200probe.py rq1 20001000 9  # one read

Safety: editor communication mode (7F 00 00 01) is always switched off again
on exit. The only other write is `irname`, which writes one user IR's 32-byte
name, reads it back, then restores the original bytes and verifies them.
"""

from __future__ import annotations

import json
import sys
import time
from datetime import datetime
from pathlib import Path

import mido

mido.set_backend("mido.backends.rtmidi")

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
FIXTURES = ROOT / "ir200-core" / "tests" / "fixtures"
LOGS = HERE / "logs"

ROLAND = 0x41
MODEL = [0x00, 0x00, 0x00, 0x00, 0x06]
RQ1, DT1 = 0x11, 0x12

COMM_LEVEL = 0x7F000000
COMM_MODE = 0x7F000001

# Forbidden: never written by this script.
FORBIDDEN = {0x7F000002, 0x7F000200, 0x7F000104, 0x7F010003}


def pack(a: int) -> int:
    return ((a >> 3) & 0x0FE00000) | ((a >> 2) & 0x001FC000) | ((a >> 1) & 0x3F80) | (a & 0x7F)


def unpack(v: int) -> int:
    return ((v & 0x0FE00000) << 3) | ((v & 0x001FC000) << 2) | ((v & 0x3F80) << 1) | (v & 0x7F)


def b4(v: int) -> list[int]:
    return [(v >> 24) & 0x7F, (v >> 16) & 0x7F, (v >> 8) & 0x7F, v & 0x7F]


def hexs(d) -> str:
    return " ".join(f"{b:02X}" for b in d)


def load_names() -> dict[int, tuple]:
    """Parameter table generated from IR Loader (reference/params.py)."""
    ns: dict = {}
    src = ROOT / "reference" / "params.py"
    if src.exists():
        exec(src.read_text(encoding="utf-8"), ns)
    return ns.get("PARAMS", {})


PARAMS = load_names()


def describe(addr: int, data: list[int]) -> str:
    """Name every known parameter inside a DT1 (temporary memory map; stored
    memories are mapped onto it)."""
    a28 = pack(addr)
    if 0x20100000 <= addr <= 0x3000FFFF:
        mem = (a28 - pack(0x20000000)) // pack(0x00100000)
        a28 -= mem * pack(0x00100000)
        prefix = f"[memory {mem}] "
    else:
        prefix = ""
    out = []
    for p_addr, (kind, n, ofs, mn, mx, name) in PARAMS.items():
        p28 = pack(p_addr)
        if a28 <= p28 and p28 + n <= a28 + len(data):
            raw = data[p28 - a28 : p28 - a28 + n]
            if kind == "i7":
                v = raw[0] - ofs
            elif kind == "n":
                v = 0
                for x in raw:
                    v = (v << 4) | (x & 0x0F)
                v -= ofs
            elif kind == "i14":
                v = ((raw[0] << 7) | raw[1]) - ofs
            else:
                v = repr(bytes(raw).decode("ascii", "replace"))
            out.append(f"{name}={v}")
    return prefix + (", ".join(out) if out else "(no known parameter)")


class Pedal:
    def __init__(self, log):
        ins = [n for n in mido.get_input_names() if "IR-200" in n.upper()]
        outs = [n for n in mido.get_output_names() if "IR-200" in n.upper()]
        if not ins or not outs:
            sys.exit(f"IR-200 not found. Inputs {mido.get_input_names()}, outputs {mido.get_output_names()}. Is IR Loader closed?")
        self.inp = mido.open_input(ins[0])
        self.out = mido.open_output(outs[0])
        self.dev = 0x10
        self.log = log
        self.editor_mode = False
        log(f"ports in={ins[0]!r} out={outs[0]!r}")

    def close(self):
        if self.editor_mode:
            self.dt1(COMM_MODE, [0])
            time.sleep(0.2)
            self.log("editor mode OFF")
        self.inp.close()
        self.out.close()

    def send(self, data: list[int]):
        self.out.send(mido.Message("sysex", data=data))
        # IR Loader paces as if over a 312,500 bps link.
        time.sleep((len(data) + 2) * 10 / 312_500 + 0.002)

    def frame(self, cmd, addr, body):
        a = b4(addr)
        ck = (128 - (sum(a) + sum(body)) % 128) & 0x7F
        return [ROLAND, self.dev, *MODEL, cmd, *a, *body, ck]

    def rq1(self, addr: int, size: int):
        self.send(self.frame(RQ1, addr, b4(unpack(size))))

    def dt1(self, addr: int, data: list[int]):
        if addr in FORBIDDEN:
            raise RuntimeError(f"refusing to write {addr:08X}")
        self.send(self.frame(DT1, addr, data))

    def drain(self, quiet_ms=300, max_ms=5000) -> list[list[int]]:
        """Collect inbound sysex until the line is quiet."""
        got = []
        start = last = time.monotonic()
        while True:
            msg = self.inp.poll()
            now = time.monotonic()
            if msg is None:
                if now - last > quiet_ms / 1000 or now - start > max_ms / 1000:
                    return got
                time.sleep(0.002)
                continue
            if msg.type in ("clock", "active_sensing"):
                continue
            last = now
            if msg.type == "sysex":
                got.append(list(msg.data))
            else:
                self.log(f"non-sysex: {msg}")

    @staticmethod
    def parse_dt1(d):
        if len(d) >= 13 and d[0] == ROLAND and d[2:7] == MODEL and d[7] == DT1:
            a = (d[8] << 24) | (d[9] << 16) | (d[10] << 8) | d[11]
            return a, d[12:-1]
        return None

    def read(self, addr: int, size: int, quiet_ms=300) -> tuple[list[list[int]], bytes]:
        """RQ1 and return (raw frames, reassembled data)."""
        self.rq1(addr, size)
        frames = self.drain(quiet_ms)
        data = bytearray()
        want = pack(addr)
        for f in frames:
            p = self.parse_dt1(f)
            if p and pack(p[0]) == want:
                data += bytes(p[1])
                want += len(p[1])
        return frames, bytes(data)

    def identity(self):
        self.send([0x7E, 0x7F, 0x06, 0x01])
        for f in self.drain(400):
            if f[0] == 0x7E and f[2:4] == [0x06, 0x02]:
                self.dev = f[1]
                return f
        return None

    def editor_on(self):
        _, level = self.read(COMM_LEVEL, 1)
        self.log(f"communication level: {hexs(level)}")
        self.dt1(COMM_MODE, [1])
        self.editor_mode = True
        self.log("editor mode ON")


def logger(path: Path):
    path.parent.mkdir(parents=True, exist_ok=True)
    fh = path.open("a", encoding="utf-8")

    def log(line: str):
        stamp = datetime.now().strftime("%H:%M:%S.%f")[:-3]
        fh.write(f"{stamp} {line}\n")
        fh.flush()
        print(stamp, line, flush=True)

    return log


def syx(frames: list[list[int]]) -> bytes:
    return b"".join(bytes([0xF0, *f, 0xF7]) for f in frames)


# Leaf blocks (absolute wire address, length) from IR Loader's map.
SYSTEM_BLOCKS = [
    (0x10000000, 3), (0x10001000, 3), (0x10002000, 1), (0x10003000, 22), (0x10004000, 22),
    (0x10005000, 1), (0x10006000, 1), (0x10007000, 1), (0x10010000, 2), (0x10011000, 19),
    (0x10012000, 18), (0x10013000, 7), (0x10014000, 11), (0x10015000, 5), (0x10016000, 6),
    (0x10017000, 6), (0x10020000, 6), (0x10021000, 6), (0x10022000, 15),
]
MEMORY_BLOCKS = [
    (0x00000000, 16), (0x00001000, 9), (0x00002000, 2), (0x00003000, 8), (0x00004000, 3),
    (0x00005000, 6), (0x00006000, 6), (0x00007000, 5), (0x00010000, 1), (0x00011000, 22),
    (0x00012000, 22), (0x00013000, 1), (0x00014000, 6), (0x00015000, 6), (0x00016000, 6),
    (0x00017000, 6), (0x00020000, 15),
]


def memory_base(n: int) -> int:
    return unpack(pack(0x20000000) + n * pack(0x00100000))


def read_blocks(p: Pedal, blocks, base=0):
    frames, total = [], 0
    for off, n in blocks:
        addr = unpack(pack(base) + pack(off)) if base else off
        f, d = p.read(addr, n)
        frames += f
        if len(d) != n:
            p.log(f"  block {addr:08X}: expected {n} bytes, got {len(d)} in {len(f)} frame(s)")
        total += len(d)
    return frames, total


def cmd_read():
    log = logger(LOGS / f"read-{datetime.now():%Y%m%d-%H%M%S}.log")
    p = Pedal(log)
    try:
        reply = p.identity()
        log(f"Q1 identity reply: {hexs([0xF0, *reply, 0xF7]) if reply else 'NONE'}")

        # Q: does RQ1 work before editor mode?
        f, d = p.read(0x00000000, 2)
        log(f"RQ1 setup BEFORE editor mode: {len(f)} frame(s), data {hexs(d)}")

        p.editor_on()
        FIXTURES.mkdir(parents=True, exist_ok=True)
        summary = {"identity": hexs(reply or [])}

        f, d = p.read(0x00000000, 2)
        (FIXTURES / "setup.syx").write_bytes(syx(f))
        log(f"setup: {hexs(d)} -> {describe(0, list(d))}")

        f, n = read_blocks(p, SYSTEM_BLOCKS)
        (FIXTURES / "system.syx").write_bytes(syx(f))
        log(f"system: {len(f)} frames, {n} bytes")

        f, n = read_blocks(p, MEMORY_BLOCKS, 0x20000000)
        (FIXTURES / "temporary.syx").write_bytes(syx(f))
        log(f"temporary memory: {len(f)} frames, {n} bytes")
        for fr in f:
            pr = p.parse_dt1(fr)
            if pr:
                log(f"  {pr[0]:08X}: {describe(pr[0], pr[1])}")

        # Q6: stored memory addressing; factory memories also calibrate the
        # amp and cab type lists against the sound list.
        for m in list(range(1, 33)) + [128]:
            f, n = read_blocks(p, MEMORY_BLOCKS, memory_base(m))
            (FIXTURES / f"memory-{m:03}.syx").write_bytes(syx(f))
            fr = [p.parse_dt1(x) for x in f]
            name = next((bytes(d).decode("ascii", "replace") for a, d in filter(None, fr) if a == memory_base(m)), "?")
            amp = next((d for a, d in filter(None, fr) if a == unpack(pack(memory_base(m)) + pack(0x1000))), None)
            cab_a = next((d for a, d in filter(None, fr) if a == unpack(pack(memory_base(m)) + pack(0x5000))), None)
            cab_b = next((d for a, d in filter(None, fr) if a == unpack(pack(memory_base(m)) + pack(0x6000))), None)

            def cab(c):
                return None if c is None else (c[0] << 12) | (c[1] << 8) | (c[2] << 4) | c[3]

            log(f"memory {m:3} @ {memory_base(m):08X}: {len(f)} frames, {n} bytes, name {name!r}, "
                f"amp_type {amp[0] if amp else None}, cab A {cab(cab_a)}, cab B {cab(cab_b)}")

        f, d = p.read(0x50000000, 256)
        (FIXTURES / "pc-map.syx").write_bytes(syx(f))
        log(f"pc map: {len(f)} frames, {len(d)} bytes; first entries {hexs(d[:16])}")

        f, d = p.read(0x40000000, 337)
        (FIXTURES / "cab-info.syx").write_bytes(syx(f))
        log(f"cab info: {len(f)} frames, {len(d)} bytes; favourites at {[i for i, x in enumerate(d) if x]}")

        f, d = p.read(0x60000000, 128)
        (FIXTURES / "ir-info.syx").write_bytes(syx(f))
        log(f"ir info: {len(f)} frames, {len(d)} bytes: {hexs(d)}")

        names = []
        for i in range(128):
            base = unpack(pack(0x68000000) + i * pack(0x00020000))
            _, nm = p.read(base, 32, quiet_ms=150)
            _, fl = p.read(unpack(pack(base) + 0x20), 4, quiet_ms=150)
            used = ((fl[0] << 12) | (fl[1] << 8) | (fl[2] << 4) | fl[3]) if len(fl) == 4 else None
            names.append((i + 1, nm.decode("ascii", "replace").rstrip(), used))
        log("user IRs (slot, name, file-name length): " + "; ".join(f"{s}:{n!r}:{u}" for s, n, u in names if u))
        summary["user_irs"] = names

        # Q2: oversized request: whole temporary memory span in one RQ1.
        f, d = p.read(0x20000000, pack(0x00020000) + 15, quiet_ms=600)
        log(f"Q2 oversized RQ1 (temporary memory, {pack(0x00020000) + 15} bytes): {len(f)} frame(s): "
            + "; ".join(f"{p.parse_dt1(x)[0]:08X}+{len(p.parse_dt1(x)[1])}" for x in f if p.parse_dt1(x)))
        f, d = p.read(0x10000000, 64, quiet_ms=600)
        log(f"Q2 RQ1 across system sub-blocks (10000000, 64 bytes): {len(f)} frame(s): "
            + "; ".join(f"{p.parse_dt1(x)[0]:08X}+{len(p.parse_dt1(x)[1])}" for x in f if p.parse_dt1(x)))

        (LOGS / "summary.json").write_text(json.dumps(summary, indent=1))
    finally:
        p.close()


def cmd_listen(seconds: int):
    log = logger(LOGS / f"listen-{datetime.now():%Y%m%d-%H%M%S}.log")
    p = Pedal(log)
    try:
        reply = p.identity()
        log(f"identity: {hexs(reply or [])}")
        p.editor_on()
        log(f"listening {seconds} s")
        end = time.monotonic() + seconds
        while time.monotonic() < end:
            msg = p.inp.poll()
            if msg is None:
                time.sleep(0.003)
                continue
            if msg.type in ("clock", "active_sensing"):
                continue
            if msg.type == "sysex":
                d = list(msg.data)
                pr = p.parse_dt1(d)
                if pr:
                    log(f"DT1 {pr[0]:08X} [{hexs(pr[1])}] {describe(*pr)}")
                else:
                    log(f"SYSEX {hexs(d)}")
            else:
                log(f"MIDI {msg}")
        log("done")
    finally:
        p.close()


def cmd_rq1(addr: str, size: str):
    log = logger(LOGS / "rq1.log")
    p = Pedal(log)
    try:
        p.identity()
        p.editor_on()
        f, d = p.read(int(addr, 16), int(size))
        for x in f:
            log(hexs(x))
        log(f"data: {hexs(d)}")
    finally:
        p.close()


def cmd_irname(slot: int, name: str):
    """Rename user IR `slot`, verify, then restore the exact original name."""
    assert 1 <= slot <= 128 and len(name) <= 32 and name.isascii()
    log = logger(LOGS / "irname.log")
    p = Pedal(log)
    base = unpack(pack(0x68000000) + (slot - 1) * pack(0x00020000))
    try:
        p.identity()
        p.editor_on()
        _, orig = p.read(base, 32)
        log(f"original: {hexs(orig)} {orig!r}")
        assert len(orig) == 32, "could not read the original name"
        new = list(name.encode().ljust(32, b" "))
        p.dt1(base, new)
        time.sleep(0.2)
        _, got = p.read(base, 32)
        log(f"after rename: {got!r} -> {'OK' if list(got) == new else 'MISMATCH'}")
        p.dt1(base, list(orig))
        time.sleep(0.2)
        _, back = p.read(base, 32)
        log(f"restored: {back!r} -> {'OK' if back == orig else 'MISMATCH'}")
    finally:
        p.close()


if __name__ == "__main__":
    a = sys.argv[1:]
    if not a or a[0] == "ports":
        print("in:", mido.get_input_names())
        print("out:", mido.get_output_names())
    elif a[0] == "read":
        cmd_read()
    elif a[0] == "listen":
        cmd_listen(int(a[1]) if len(a) > 1 else 120)
    elif a[0] == "rq1":
        cmd_rq1(a[1], a[2])
    elif a[0] == "irname":
        cmd_irname(int(a[1]), a[2])
    else:
        sys.exit(__doc__)
