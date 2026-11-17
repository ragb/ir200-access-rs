"""Extract IR-200 IR Loader's embedded web UI (HTML/JS) from its exe.

The exe carries a plain, unencrypted zip of its html/ folder. This slices it
out from the first local-file header to the end-of-central-directory record.

    py extract_ir_loader_ui.py "C:\\Program Files (x86)\\BOSS\\IR-200 IR Loader\\IR-200 IR Loader.exe" out_dir
"""
import io, struct, sys, zipfile

exe, out = sys.argv[1], sys.argv[2]
d = open(exe, "rb").read()
start = d.find(b"PK\x03\x04")
eocd = d.rfind(b"PK\x05\x06")
comment_len = struct.unpack("<H", d[eocd + 20:eocd + 22])[0]
zipfile.ZipFile(io.BytesIO(d[start:eocd + 22 + comment_len])).extractall(out)
print("extracted to", out)
