# lightnvr-rs: the generated Rust build

`lightnvr-rs` is the LightNVR backend mechanically translated from C with
[C2Rust](https://github.com/immunant/c2rust). It serves the same web UI, API,
configuration and database as `lightnvr`, and is published next to the C
artifacts on every release.

**C is the source of truth.** The Rust in `rust/` is build output that is
checked in so it can be reviewed, built and released without the translator.
Nobody edits it by hand.

## Contributing

Change the C. That's it.

When your pull request touches `src/`, `include/`, `CMakeLists.txt`, `cmake/`
or the translation tooling, the **lightnvr-rs** workflow:

1. regenerates `rust/` from your C in a pinned container,
2. builds `lightnvr-rs`,
3. boots it against a disposable fixture and runs the authentication and
   discovery API contracts,
4. commits the regenerated `rust/` back to your branch as `github-actions[bot]`.

If step 1–3 fails, your C change doesn't translate, build or boot as Rust, and the
log says which file. For fork PRs, CI can't push to your branch: it still runs
steps 1–3, and `rust/` is regenerated in a bot PR after merge.

Two checks keep `rust/` generated-only:

- **Drift:** `rust/` must be byte-identical to a fresh generation, so a hand
  edit is overwritten by the bot commit.
- **Guard:** the PR fails if any non-bot commit touches `rust/`. Drop the
  change (`git checkout origin/main -- rust/`) and fix it in C.

## Where things live

| Path | What |
| --- | --- |
| `rust/` | Generated crate (`Cargo.toml`, `main.rs`, `generated/*.rs`, `MANIFEST.json`). Do not edit. |
| `rust/MANIFEST.json` | Every module, with its C source hash and Rust hash. |
| `tools/rust-transpile/generate.py` | The generator. `--check` verifies; `--native` builds the bundled static C libs. |
| `tools/rust-transpile/repairs.py` | Reviewed mechanical fixes for compiler errors in raw C2Rust output. |
| `tools/rust-transpile/toolchain.json` | C2Rust revision, Rust nightly, Clang version, CMake feature set. |
| `tools/rust-transpile/Dockerfile` | Pinned base image and Debian snapshot. System headers end up in the Rust, so this pins the output bytes. |
| `.github/workflows/lightnvr-rs.yml` | PR and `main` conversion pipeline. |
| `.github/workflows/lightnvr-rs-release.yml` | Tag release: `lightnvr-rs_<ver>_sid_amd64.tar.gz`. |

## Running it locally

Needs Docker. Output matches CI because it uses the same image.

```sh
tools/rust-transpile/docker.sh                    # regenerate rust/
tools/rust-transpile/docker.sh generate --check   # is rust/ current?
tools/rust-transpile/docker.sh build              # regenerate, release build, smoke test
```

The binary is `rust/target/release/lightnvr-rs` and takes the same `-c
lightnvr.ini` as `lightnvr`. It links the container's Debian sid libraries, so
run it in the image (`docker.sh bash`) or on a matching Debian sid host.

## When a C change breaks translation

- **C2Rust can't translate a file:** usually a construct it doesn't support
  (some GNU extensions, inline asm, unusual macros). Simplify the C, or
  guard it so the configured build doesn't use it.
- **The Rust doesn't compile:** C2Rust emitted something rustc rejects. If the
  fix is mechanical and safe, add it to `repairs.py`, keyed by C file. Repairs
  fail loudly when their target disappears, so stale ones get cleaned up.
- **A repair is reported stale:** the C it targeted changed. Update or remove
  that entry in `repairs.py`.

## Changing the toolchain

Bumping anything in `toolchain.json` or the `Dockerfile` (C2Rust, Rust nightly,
Clang, base image, Debian snapshot, feature flags) regenerates every module.
Do it in its own PR so the diff is attributable to the toolchain, not a feature.

## Limits

- Unsafe, C-shaped Rust. Compiling does not mean memory safety or parity.
- One configuration: Linux x86_64, SOD/go2rtc/MQTT/TLS/S3 on, LiteRT and Sentry
  off, YAML validation stubbed. arm64 would need its own generated tree.
- Native libraries (FFmpeg, SQLite, libuv, curl, mbedTLS, cJSON, mosquitto,
  llhttp, inih, FAAC) stay native.
- Real-camera recording and long-running behavior are not yet covered by
  CI. Only startup, auth, discovery and shutdown are.
- `src/tools/*` and `src/sod/sod_ingenic_mxu.c` translate on every run but are
  not linked into the binary.
