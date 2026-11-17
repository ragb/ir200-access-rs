"""Generate ir200-core/src/map.rs from IR Loader's address_map.js.

IR Loader's address map is the only specification of the IR-200's SysEx
parameters. This turns it into Rust consts so the codec never carries a
hand-typed offset or range:

    py tools/gen/gen_map.py reference/ir-loader-js/address_map.js > ir200-core/src/map.rs

`address_map.js` is Roland's code and is not committed (see .gitignore);
re-extract it with reference/extract_ir_loader_ui.py. The generated map.rs is
committed.

Address conventions (from IR Loader's converter.js): addresses in the JS are
written in *wire* form (four 7-bit bytes, e.g. 0x10003000); arithmetic happens
on the packed 28-bit value ("nibble()") and is converted back ("_7bitize()").
"""

import re
import sys

ENC = {
    "INTEGER1x7": ("I7", 1),
    "INTEGER2x4": ("N2", 2),
    "INTEGER4x4": ("N4", 4),
    "INTEGER2x7": ("I14", 2),
    "INTEGER8x4": ("N8", 8),
}


def pack(wire):
    """Wire address (4 x 7-bit bytes in a u32) -> 28-bit integer."""
    return (
        ((wire >> 3) & 0x0FE00000)
        | ((wire >> 2) & 0x001FC000)
        | ((wire >> 1) & 0x00003F80)
        | (wire & 0x7F)
    )


def unpack(v):
    """28-bit integer -> wire address."""
    return (
        ((v & 0x0FE00000) << 3)
        | ((v & 0x001FC000) << 2)
        | ((v & 0x00003F80) << 1)
        | (v & 0x7F)
    )


def parse(src):
    blocks = {}
    for m in re.finditer(r"var (\w+) = \[(.*?)\n\t\];", src, re.S):
        params = []
        for p in re.finditer(
            r"addr:(0x[0-9A-Fa-f]+),\s*size:\s*(\w+),\s*ofs:\s*(-?\d+),\s*"
            r"init:\s*('[^']*'|-?\d+),\s*min:\s*(-?\d+),\s*max:\s*(-?\d+),\s*name:'PRMID_(\w+)'",
            m.group(2),
        ):
            a, sz, ofs, init, mn, mx, name = p.groups()
            if sz in ENC:
                enc, n = ENC[sz]
                init_v = int(init)
            else:
                enc, n = "TEXT", int(sz)
                init_v = None
            params.append(
                dict(off=pack(int(a, 16)), enc=enc, n=n, ofs=int(ofs), min=int(mn),
                     max=int(mx), id=name, init=init_v, text_init=init if init_v is None else None)
            )
        kids = [
            (int(a, 16), c)
            for a, c in re.findall(r"addr:(0x[0-9A-Fa-f]+),\s*size:\s*0,\s*child:(\w+),", m.group(2))
        ]
        blocks[m.group(1)] = (params, kids)
    return blocks


def leaves(blocks, group, base):
    """Yield (block_name, absolute_wire_addr, params) for each leaf block."""
    params, kids = blocks[group]
    if params:
        yield group, base, params
    for a, child in kids:
        yield from leaves(blocks, child, unpack(pack(base) + pack(a)))


def rust_ident(s):
    s = s.upper()
    return s if not s[0].isdigit() else "_" + s


def main():
    src = open(sys.argv[1], encoding="utf-8").read()
    blocks = parse(src)

    roots = [
        ("SETUP", 0x00000000),
        ("SYSTEM", 0x10000000),
        ("PATCH", 0x20000000),
        ("MIDI", 0x50000000),
    ]
    out = []
    w = out.append
    w("//! Parameter address map, **generated** by `tools/gen/gen_map.py` from IR")
    w("//! Loader's `address_map.js` (the IR-200's only parameter specification).")
    w("//! Do not edit by hand: regenerate instead.")
    w("//!")
    w("//! One module per leaf block. `BLOCK` is the block's absolute wire address")
    w("//! (memory blocks are given at the temporary memory, `20 00 00 00`) and byte")
    w("//! length; every other const is one parameter with its offset inside the")
    w("//! block, wire encoding, display offset (`display = raw - ofs`), display")
    w("//! range and IR Loader's init value.")
    w("")
    w("#![allow(dead_code)]")
    w("#![cfg_attr(rustfmt, rustfmt_skip)]")
    w("")
    w("use crate::codec::{Block, Enc, P};")
    w("")
    flat = []
    for root, base in roots:
        for name, addr, params in leaves(blocks, root, base):
            last = params[-1]
            length = last["off"] + last["n"]
            mod = name.lower()
            w(f"/// `{name}` at `{' '.join(f'{(addr >> s) & 0x7F:02X}' for s in (24, 16, 8, 0))}`.")
            w(f"pub mod {mod} {{")
            w("    use super::*;")
            w(f"    pub const BLOCK: Block = Block {{ name: \"{name}\", addr: 0x{addr:08X}, len: {length} }};")
            prefix = name + "_"
            for p in params:
                if p["enc"] == "TEXT":
                    w(f"    /// Text, {p['n']} chars. IR Loader init: {p['text_init']}")
                    w(f"    pub const {rust_ident(p['id'][len(prefix):] if p['id'].startswith(prefix) else p['id'])}: P = P {{ id: \"{p['id']}\", block: &BLOCK, off: {p['off']}, enc: Enc::Text({p['n']}), ofs: 0, min: 32, max: 127, init: 32 }};")
                    continue
                local = p["id"][len(prefix):] if p["id"].startswith(prefix) else p["id"]
                w(
                    f"    pub const {rust_ident(local)}: P = P {{ id: \"{p['id']}\", block: &BLOCK, off: {p['off']}, "
                    f"enc: Enc::{p['enc']}, ofs: {p['ofs']}, min: {p['min']}, max: {p['max']}, init: {p['init']} }};"
                )
                flat.append(f"{mod}::{rust_ident(local)}")
            w("}")
            w("")
    w("/// Every numeric parameter in SETUP, SYSTEM, the temporary memory and the")
    w("/// PC map, in address order. Used to describe single-parameter DT1s.")
    w("pub static ALL: &[P] = &[")
    for f in flat:
        w(f"    {f},")
    w("];")
    print("\n".join(out))


if __name__ == "__main__":
    main()
