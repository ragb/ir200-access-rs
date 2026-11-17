"""Read user IR slots (name, file name, coefficients). Read-only."""
import sys, json
sys.argv = sys.argv[:1] + sys.argv[1:]
from ir200probe import Pedal, logger, LOGS, pack, unpack, hexs
from datetime import datetime
slots = [int(x) for x in sys.argv[1:]] or [1]
log = logger(LOGS / "irdump.log")
p = Pedal(log)
out = {}
try:
    p.identity(); p.editor_on()
    for s in slots:
        base = unpack(pack(0x68000000) + (s - 1) * pack(0x00020000))
        _, name = p.read(base, 32)
        _, fl = p.read(unpack(pack(base) + 0x20), 4)
        n = (fl[0] << 12) | (fl[1] << 8) | (fl[2] << 4) | fl[3]
        _, fn = p.read(unpack(pack(base) + 0x24), 2 * n)
        fname = bytes((fn[2*i] << 4) | fn[2*i+1] for i in range(n)).decode("utf-8", "replace")
        coef = bytearray()
        addr = pack(base) + pack(0x224)
        total = 3354 * 8
        while len(coef) < total:
            k = min(240, total - len(coef))
            _, d = p.read(unpack(addr + len(coef)), k, quiet_ms=120)
            if not d: break
            coef += d
        vals = []
        for i in range(len(coef) // 8):
            v = 0
            for x in coef[8*i:8*i+8]: v = (v << 4) | (x & 0xF)
            if v >= 1 << 31: v -= 1 << 32
            vals.append(v)
        log(f"slot {s}: name {name!r} file {fname!r} coefs {len(vals)} first {vals[:12]} max {max(map(abs, vals)) if vals else None}")
        out[s] = {"name": name.decode('ascii','replace'), "file": fname, "coef": vals}
finally:
    p.close()
json.dump(out, open(LOGS / "irs.json", "w"))
