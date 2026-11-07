# lightnvr-rs — generated, do not edit

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
