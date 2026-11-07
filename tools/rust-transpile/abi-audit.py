#!/usr/bin/env python3
"""Fail if lightnvr-rs has an extern declaration that really disagrees across modules.

C2Rust translates each C file on its own, so every module carries its own copy
of shared header types (sqlite3, FILE, ...). rustc reports every repeated
extern as clashing because those copies are distinct nominal types. That is
harmless as long as the signatures match once module paths are stripped. A
signature that still differs after that means two C files disagree about a
function's ABI, which is undefined behavior; fix the C declaration.

  tools/rust-transpile/abi-audit.py [--manifest-path rust/Cargo.toml]
"""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys

REPO = Path(__file__).resolve().parents[2]
SIG = re.compile(r"expected `([^`]*)`\s*found `([^`]*)`")


def normalize(signature):
    return re.sub(r"\bsrc__\w+?::", "", signature).replace("_IO_FILE", "FILE")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--manifest-path", default=str(REPO / "rust/Cargo.toml"))
    a = ap.parse_args()
    target = Path(a.manifest_path).resolve().parent / "target/abi-audit"
    env = dict(os.environ, RUSTFLAGS="--force-warn clashing_extern_declarations",
               CARGO_TARGET_DIR=str(target))
    proc = subprocess.run(["cargo", "check", "--release", "--manifest-path", a.manifest_path,
                           "--message-format", "json"], env=env, stdout=subprocess.PIPE,
                          # rustup resolves rust-toolchain.toml from the working directory.
                          cwd=Path(a.manifest_path).resolve().parent,
                          stderr=subprocess.PIPE, text=True)
    nominal, real = 0, []
    for line in proc.stdout.splitlines():
        msg = json.loads(line)
        if msg.get("reason") != "compiler-message":
            continue
        diag = msg["message"]
        if (diag.get("code") or {}).get("code") != "clashing_extern_declarations":
            continue
        notes = " ".join(child["message"] for child in diag["children"])
        match = SIG.search(notes)
        if match and normalize(match.group(1)) == normalize(match.group(2)):
            nominal += 1
        else:
            real.append(diag["rendered"])
    if proc.returncode:
        sys.exit(proc.stderr[-6000:] + "\ncargo check failed")
    for rendered in real:
        print(rendered, file=sys.stderr)
    print(f"abi-audit: {nominal} nominal clashes (identical after module paths), {len(real)} real mismatches")
    if real:
        sys.exit("abi-audit: C files disagree about an extern signature; fix the C declaration.")


if __name__ == "__main__":
    main()
