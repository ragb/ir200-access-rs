r"""List every user IR slot: name, file name. Read-only.

    .venv\Scripts\python.exe irslots.py      # writes logs/irslots.json
"""
import json
from ir200probe import Pedal, logger, LOGS, pack, unpack

log = logger(LOGS / "irslots.log")
p = Pedal(log)
out = {}
try:
    p.identity()
    p.editor_on()
    for s in range(1, 129):
        base = pack(0x68000000) + (s - 1) * pack(0x00020000)
        _, d = p.read(unpack(base), 36, quiet_ms=80)
        name, fl = d[:32], d[32:36]
        n = (fl[0] << 12) | (fl[1] << 8) | (fl[2] << 4) | fl[3] if len(fl) == 4 else -1
        fname = ""
        if n > 0:
            _, fn = p.read(unpack(base + 0x24), 2 * n, quiet_ms=80)
            fname = bytes((fn[2*i] << 4) | fn[2*i+1] for i in range(len(fn) // 2)).decode("utf-8", "replace")
        out[s] = {"name": name.decode("ascii", "replace"), "len": n, "file": fname}
finally:
    p.close()
json.dump(out, open(LOGS / "irslots.json", "w"), indent=1)
used = {k: v for k, v in out.items() if v["len"] > 0}
print("used", len(used), "empty", sum(v["len"] == 0 for v in out.values()), "unknown", sum(v["len"] < 0 for v in out.values()))
for k, v in used.items():
    print(k, repr(v["name"].strip()), v["file"])
