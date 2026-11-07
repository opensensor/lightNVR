#!/usr/bin/env python3
"""Generate the lightnvr-rs crate (rust/) from the C sources.

C is the source of truth. rust/ is build output that happens to be checked in:
it must always be byte-identical to what this script produces, and nobody
edits it by hand. CI regenerates it on every pull request.

  tools/rust-transpile/generate.py            # regenerate rust/ in place
  tools/rust-transpile/generate.py --check    # fail if rust/ is stale
  tools/rust-transpile/generate.py --native   # also build static C libs for linking

Run inside the pinned image (tools/rust-transpile/Dockerfile) for output that
matches CI: system headers (glibc, FFmpeg, ...) are inlined into the Rust, so a
different host produces different bytes.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import difflib
import filecmp
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile

import repairs

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
PINS = json.loads((HERE / "toolchain.json").read_text())
CRATE = "lightnvr-rs"
GENERATOR_VERSION = 1
# Placeholders swapped for build-time values so commit/date never enter rust/.
BUILD_PLACEHOLDERS = {
    "LIGHTNVR_BUILD_DATE": "@@LIGHTNVR_RS_BUILD_DATE@@",
    "LIGHTNVR_GIT_COMMIT": "@@LIGHTNVR_RS_GIT_COMMIT@@",
}
PLACEHOLDER_WIDTH = len(next(iter(BUILD_PLACEHOLDERS.values()))) + 1
assert all(len(v) + 1 == PLACEHOLDER_WIDTH for v in BUILD_PLACEHOLDERS.values())
ENV = dict(os.environ, LC_ALL="C", TZ="UTC", RUSTUP_AUTO_INSTALL="0")


def log(*args):
    print("[rust-transpile]", *args, file=sys.stderr, flush=True)


def run(cmd, **kw):
    log("$", shlex.join(str(c) for c in cmd))
    return subprocess.run(cmd, env=ENV, check=True, **kw)


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def clang():
    for name in (f"clang-{PINS['clang_major']}", "clang"):
        if shutil.which(name):
            return shutil.which(name)
    sys.exit(f"clang {PINS['clang_major']} not found")


def configure(build):
    # Always configure from scratch: cached find_* results from another host,
    # image or checkout path would silently change the translated bytes.
    shutil.rmtree(build, ignore_errors=True)
    cmd = ["cmake", "-S", REPO, "-B", build, "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
           f"-DCMAKE_C_COMPILER={clang()}"]
    cxx = shutil.which(f"clang++-{PINS['clang_major']}") or shutil.which("clang++")
    if cxx:
        cmd.append(f"-DCMAKE_CXX_COMPILER={cxx}")
    cmd += [f"-D{k}={v}" for k, v in PINS["cmake_options"].items()]
    # A fixed commit keeps CMake from calling git; the value is replaced below.
    env = dict(ENV, LIGHTNVR_GIT_COMMIT="0000000")
    log("$", shlex.join(str(c) for c in cmd))
    subprocess.run(cmd, env=env, check=True, stdout=subprocess.DEVNULL)
    version = build / "include/core/version.h"
    text = version.read_text()
    for macro, placeholder in BUILD_PLACEHOLDERS.items():
        text, n = re.subn(rf'(#define {macro}) "[^"]*"', rf'\1 "{placeholder}"', text)
        assert n == 1, (macro, n)
    version.write_text(text)


def build_native(build):
    cmd = ["cmake", "--build", build, "--parallel", str(os.cpu_count() or 2), "--target",
           *PINS["native_static_libraries"]]
    log("$", shlex.join(str(c) for c in cmd))
    p = subprocess.run(cmd, env=ENV, capture_output=True, text=True)
    if p.returncode:
        sys.exit(p.stdout[-8000:] + p.stderr[-8000:] + "\nnative static library build failed")


def compile_entries(build):
    by_file = {}
    for entry in json.loads((build / "compile_commands.json").read_text()):
        by_file.setdefault(entry["file"], []).append(entry)
    return by_file


def translation_args(entry, rel, build):
    args = shlex.split(entry["command"]) if "command" in entry else list(entry["arguments"])
    args += ["-include", str(HERE / "glibc-compat.h"),
             # Keep __FILE__ and friends free of checkout/build locations.
             f"-fmacro-prefix-map={REPO}/=", f"-fmacro-prefix-map={build}/=build/"]
    args += ["-D" + d for d in PINS["translation_defines"] + PINS["per_file_defines"].get(rel, [])]
    return args


def translate(c2rust, build, work, entries, template, source):
    rel = source.relative_to(REPO).as_posix()
    choices = entries.get(str(source), [])
    entry = next((e for e in choices if "lightnvr_lib.dir" in e.get("output", "")),
                 choices[0] if choices else template)
    args = translation_args(entry, rel, build)
    if not choices:  # Not part of this configuration; translate with library flags.
        args = [str(source) if a == entry["file"] else a for a in args]
    unit = work / rel.removesuffix(".c")
    unit.mkdir(parents=True)
    db = [{"directory": entry["directory"], "file": str(source), "arguments": args}]
    (unit / "compile_commands.json").write_text(json.dumps(db, indent=2))
    with (unit / "translation.log").open("w") as out:
        code = subprocess.run([c2rust, "transpile", "--edition", PINS["rust_edition"],
                               "--fail-on-error", "--emit-build-files", "--output-dir", unit / "rust",
                               unit / "compile_commands.json"], cwd=unit, env=ENV,
                              stdout=out, stderr=subprocess.STDOUT, timeout=600).returncode
    targets = [e.get("output", "") for e in choices]
    main = any(t in o for o in targets for t in PINS["main_executable_targets"])
    return {"file": rel, "ok": code == 0, "main": main, "unit": unit,
            "log": unit / "translation.log"}


def build_info_repair(text):
    """Swap configure-time placeholders for build.rs-provided constants."""
    for macro, placeholder in BUILD_PLACEHOLDERS.items():
        literal = f'b"{placeholder}\\0"'
        text = text.replace(literal, f"crate::build_info::{macro}")
    return text


# Lints that only restate faithfully translated C, silenced so real compiler
# output stays readable. Per-file translation gives each module its own copy
# of every header type, so identical extern declarations "clash" nominally;
# tools/rust-transpile/abi-audit.py re-enables that lint and fails on any
# clash that differs after the module prefixes are stripped.
QUIET_LINTS = [
    "#![allow(clashing_extern_declarations)]",
    "#![allow(unused_variables)]",
    "#![allow(unused_imports)]",
    "#![allow(unused_assignments)]",
    "#![allow(unused_parens)]",
    "#![allow(unused_must_use)]",
    "#![allow(unpredictable_function_pointer_comparisons)]",
    "#![allow(ambiguous_wide_pointer_comparisons)]",
    "#![allow(internal_features)]",
]

CARGO_TOML = """# @generated by tools/rust-transpile/generate.py. DO NOT EDIT.
[package]
name = "{crate}"
version = "{version}"
edition = "{edition}"
publish = false
license = "GPL-3.0-only"
description = "LightNVR backend mechanically translated from C by C2Rust"

[workspace]

[[bin]]
name = "{crate}"
path = "main.rs"

[profile.dev]
debug = 0

[profile.release]
debug = 0

[dependencies]
{deps}"""

BUILD_RS = r"""// @generated by tools/rust-transpile/generate.py. DO NOT EDIT.
use std::{env, path::PathBuf, process::Command};

const WIDTH: usize = {width};

fn value(name: &str, fallback: impl FnOnce() -> Option<String>) -> String {
    println!("cargo:rerun-if-env-changed={name}");
    env::var(name).ok().filter(|v| !v.is_empty()).or_else(fallback).unwrap_or_default()
}

fn output(cmd: &mut Command) -> Option<String> {
    let out = cmd.output().ok().filter(|o| o.status.success())?;
    Some(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

/// NUL-padded C string with the same array width the C header produced.
fn c_array(name: &str, text: &str) -> String {
    let mut bytes: Vec<u8> = text.bytes().filter(|b| *b != 0).take(WIDTH - 1).collect();
    bytes.resize(WIDTH, 0);
    let body: String = bytes.iter().map(|b| format!("\\x{b:02x}")).collect();
    format!("pub const {name}: &[u8; WIDTH] = b\"{body}\";\n")
}

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let native = env::var("LIGHTNVR_NATIVE_LIB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest.join("../build/rust-transpile"));
    println!("cargo:rerun-if-env-changed=LIGHTNVR_NATIVE_LIB_DIR");
    println!("cargo:rustc-link-search=native={}", native.display());
    for lib in {static_libs} {
        println!("cargo:rustc-link-lib=static={lib}");
    }
    for lib in {system_libs} {
        println!("cargo:rustc-link-lib={lib}");
    }
    let commit = value("LIGHTNVR_GIT_COMMIT", || {
        output(Command::new("git").args(["rev-parse", "--short=7", "HEAD"]).current_dir(&manifest))
    });
    let date = value("LIGHTNVR_BUILD_DATE", || output(Command::new("date").args(["-u", "+%Y-%m-%d"])));
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build_info.rs");
    let text = format!("const WIDTH: usize = {WIDTH};\n{}{}",
        c_array("LIGHTNVR_GIT_COMMIT", &commit.to_lowercase()), c_array("LIGHTNVR_BUILD_DATE", &date));
    std::fs::write(out, text).unwrap();
}
"""

README = """# lightnvr-rs — generated, do not edit

Everything in this directory is produced by `tools/rust-transpile/generate.py`
from the C sources under `src/` and `include/`. **C is the source of truth.**

- Change behavior in C. The `lightnvr-rs` CI workflow regenerates this crate
  on your pull request and pushes the result for you.
- Hand edits are rejected: CI fails any PR whose `rust/` differs from a fresh
  generation, or whose human-authored commits touch `rust/`.
- Mechanical compiler fixes live in `tools/rust-transpile/repairs.py`.
  Toolchain pins live in `tools/rust-transpile/toolchain.json`.

`MANIFEST.json` maps every module to the hash of the C file it came from.
The output is unsafe, C-shaped Rust that links the same native libraries
(FFmpeg, SQLite, libuv, curl, mbedTLS, ...). Compiling does not imply safety
or behavioral parity with the C build.

Build (after `generate.py --native` has built the bundled static C libs):

    cargo build --release --manifest-path rust/Cargo.toml
"""


def assemble(out, results, raw_root):
    gen = out / "generated"
    gen.mkdir(parents=True)
    deps, attrs, modules, manifest = {}, set(), [], []
    import tomllib
    for r in sorted((r for r in results if r["main"]), key=lambda r: r["file"]):
        rust = r["unit"] / "rust"
        deps.update(tomllib.loads((rust / "Cargo.toml").read_text())["dependencies"])
        attrs.update(re.findall(r"^#!\[.*\]$", (rust / "lib.rs").read_text(), re.M))
        raw = (rust / "src" / (Path(r["file"]).stem + ".rs")).read_text()
        leaks = (f"{REPO}/src/", f"{REPO}/include/", f"{REPO}/build/", f"{REPO}/external/", str(raw_root))
        for root in leaks:
            if root in raw:
                raise SystemExit(f"{r['file']}: generated Rust embeds local path {root}")
        text = build_info_repair(repairs.apply(r["file"], raw))
        ident = r["file"].removesuffix(".c").replace("/", "__")
        (gen / f"{ident}.rs").write_text(text)
        modules.append(f'#[path = "generated/{ident}.rs"]\npub mod {ident};')
        manifest.append({"c": r["file"], "c_sha256": sha256(REPO / r["file"]),
                         "rust": f"generated/{ident}.rs",
                         "rust_sha256": hashlib.sha256(text.encode()).hexdigest(),
                         "adapted": text != raw})
    externs = "\n".join(f"#[macro_use]\nextern crate {k.replace('-', '_')};" for k in sorted(deps))
    (out / "main.rs").write_text(
        "// @generated by tools/rust-transpile/generate.py. DO NOT EDIT.\n"
        + "\n".join(sorted(attrs | set(QUIET_LINTS))) + "\n" + externs + "\n"
        + "pub mod build_info {\n    include!(concat!(env!(\"OUT_DIR\"), \"/build_info.rs\"));\n}\n"
        + "\n".join(modules) + "\nfn main() {\n    src__core__main::main();\n}\n")
    version = re.search(r'project\(\s*\w+\s+VERSION\s+([0-9.]+)', (REPO / "CMakeLists.txt").read_text())
    (out / "Cargo.toml").write_text(CARGO_TOML.format(
        crate=CRATE, version=version.group(1) if version else "0.0.0", edition=PINS["rust_edition"],
        deps="".join(f'{k} = "{v}"\n' for k, v in sorted(deps.items()))))
    (out / "build.rs").write_text(BUILD_RS
        .replace("{static_libs}", json.dumps(PINS["native_static_libraries"]))
        .replace("{system_libs}", json.dumps(PINS["native_system_libraries"]))
        .replace("{width}", str(PLACEHOLDER_WIDTH)))
    (out / "rust-toolchain.toml").write_text(
        f'# @generated. DO NOT EDIT.\n[toolchain]\nchannel = "{PINS["rust_toolchain"]}"\n')
    (out / "README.md").write_text(README)
    (out / ".gitignore").write_text("/target/\n")
    (out / "MANIFEST.json").write_text(json.dumps({
        "generator_version": GENERATOR_VERSION,
        "toolchain": {k: PINS[k] for k in ("c2rust_revision", "rust_toolchain", "rust_edition", "clang_major")},
        "toolchain_json_sha256": sha256(HERE / "toolchain.json"),
        "repairs_sha256": sha256(HERE / "repairs.py"),
        "translated_not_linked": sorted(r["file"] for r in results if not r["main"]),
        "modules": manifest,
    }, indent=2) + "\n")


def tree(root):
    return sorted(p.relative_to(root).as_posix() for p in root.rglob("*")
                  if p.is_file() and "target" not in p.relative_to(root).parts)


def compare(expected, actual):
    """Return human-readable drift between generated `expected` and checked-in `actual`."""
    # Cargo.lock is carried forward, not generated; the commit guard protects it.
    want, have = set(tree(expected)) - {"Cargo.lock"}, set(tree(actual)) - {"Cargo.lock"}
    problems = [f"missing: rust/{p}" for p in sorted(want - have)]
    problems += [f"unexpected: rust/{p}" for p in sorted(have - want)]
    for p in sorted(want & have):
        if not filecmp.cmp(expected / p, actual / p, shallow=False):
            a = (actual / p).read_text(errors="replace").splitlines(True)
            b = (expected / p).read_text(errors="replace").splitlines(True)
            diff = list(difflib.unified_diff(a, b, f"rust/{p} (checked in)", f"rust/{p} (generated)", n=1))
            problems.append(f"differs: rust/{p}\n" + "".join(diff[:40]))
    return problems


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", type=Path, default=REPO / "rust")
    ap.add_argument("--build-dir", type=Path, default=REPO / "build/rust-transpile")
    ap.add_argument("--c2rust", default=os.environ.get("C2RUST", "c2rust"))
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 2)
    ap.add_argument("--check", action="store_true", help="verify rust/ is current; change nothing")
    ap.add_argument("--native", action="store_true", help="build bundled static C libraries for linking")
    ap.add_argument("--keep-work", action="store_true", help="retain per-file translation logs")
    a = ap.parse_args()
    out, build = a.out.resolve(), a.build_dir.resolve()
    if os.environ.get("LIGHTNVR_RS_TRANSPILE_IMAGE") != "1":
        log("WARNING: not running in the pinned image (tools/rust-transpile/docker.sh).")
        log("WARNING: host headers are inlined into the Rust; output will differ from CI and")
        log("WARNING: mismatched headers (e.g. stale FFmpeg in /usr/local) yield broken structs.")

    configure(build)
    if a.native:
        build_native(build)
    entries = compile_entries(build)
    template = entries[str(REPO / "src/core/authorization.c")][0]
    sources = sorted((REPO / "src").rglob("*.c"))
    work = Path(tempfile.mkdtemp(prefix="lightnvr-rs-"))
    try:
        log(f"translating {len(sources)} C files with {a.jobs} jobs")
        with ThreadPoolExecutor(a.jobs) as pool:
            results = list(pool.map(lambda s: translate(a.c2rust, build, work / "units", entries, template, s),
                                    sources))
        failed = [r for r in results if not r["ok"]]
        for r in failed:
            print(f"::error file={r['file']}::C2Rust could not translate {r['file']}\n"
                  + r["log"].read_text()[-4000:], file=sys.stderr)
        if failed:
            sys.exit(f"{len(failed)} C file(s) failed translation")
        staged = work / "rust"
        assemble(staged, results, work)
        if (out / "Cargo.lock").exists():
            shutil.copy2(out / "Cargo.lock", staged / "Cargo.lock")
        if a.check:
            problems = compare(staged, out)
            for p in problems:
                print(p, file=sys.stderr)
            if problems:
                sys.exit(f"rust/ is stale or hand-edited ({len(problems)} file(s)). "
                         "Run tools/rust-transpile/generate.py in the pinned image.")
            log("rust/ is current")
        else:
            out.mkdir(parents=True, exist_ok=True)
            for child in out.iterdir():  # Keep cargo's target/ cache.
                if child.name != "target":
                    shutil.rmtree(child) if child.is_dir() else child.unlink()
            shutil.copytree(staged, out, dirs_exist_ok=True)
            log(f"wrote {out} ({sum(r['main'] for r in results)} modules)")
    finally:
        if a.keep_work:
            log("translation work retained at", work)
        else:
            shutil.rmtree(work, ignore_errors=True)


if __name__ == "__main__":
    main()
