import hashlib
import pathlib
import sys

if len(sys.argv) != 3:
    raise SystemExit("usage: generate_checksums.py DIRECTORY OUTPUT")
root = pathlib.Path(sys.argv[1])
output = pathlib.Path(sys.argv[2])
if not root.is_dir():
    raise SystemExit(f"artifact directory does not exist: {root}")
files = sorted(path for path in root.rglob("*") if path.is_file())
if not files:
    raise SystemExit(f"no artifacts found under: {root}")
lines = []
for path in files:
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    lines.append(f"{digest}  {path.relative_to(root).as_posix()}")
output.write_text("\n".join(lines) + "\n", encoding="utf-8")
