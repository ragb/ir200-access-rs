import re, sys, json
src = open(sys.argv[1], encoding='utf-8').read()
sizes = {'INTEGER1x7':('i7',1),'INTEGER2x4':('n',2),'INTEGER4x4':('n',4),'INTEGER2x7':('i14',2),'INTEGER8x4':('n',8)}
blocks = {}
for m in re.finditer(r'var (\w+) = \[(.*?)\n\t\];', src, re.S):
    params = []
    for p in re.finditer(r"addr:(0x[0-9A-Fa-f]+),\s*size:\s*(\w+),\s*ofs:\s*(-?\d+),\s*init:\s*('[^']*'|-?\d+),\s*min:\s*(-?\d+),\s*max:\s*(-?\d+),\s*name:'PRMID_(\w+)'", m.group(2)):
        a,sz,ofs,_,mn,mx,name = p.groups()
        kind,n = sizes[sz] if sz in sizes else ('text', int(sz))
        params.append((int(a,16), kind, n, int(ofs), int(mn), int(mx), name))
    kids = re.findall(r"addr:(0x[0-9A-Fa-f]+),\s*size:\s*0,\s*child:(\w+),", m.group(2))
    blocks[m.group(1)] = (params, [(int(a,16),c) for a,c in kids])
def flat(block, base):
    params, kids = blocks[block]
    for a,k,n,o,mn,mx,name in params: yield base+a,k,n,o,mn,mx,name
    for a,c in kids: yield from flat(c, base+a)
out = []
for root, base in (('SETUP',0x00000000),('SYSTEM',0x10000000),('PATCH',0x20000000)):
    out += list(flat(root, base))
print(len(out), file=sys.stderr)
print("PARAMS = {")
for a,k,n,o,mn,mx,name in out:
    print(f"    0x{a:08X}: ({k!r}, {n}, {o}, {mn}, {mx}, {name!r}),")
print("}")
