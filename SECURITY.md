# Security Policy

## About LightNVR

LightNVR is a US-developed open-source Network Video Recorder licensed under the [GNU General Public License v3.0](LICENSE). It is developed and maintained by [OpenSensor Engineering](https://github.com/opensensor).

## Why C?

LightNVR is written in C deliberately to support deployment on embedded MIPS and ARM devices with older Linux kernels where modern language toolchains (e.g., Rust) are not available. Memory safety is maintained through:

- **Static Analysis**: Every pull request is scanned with `clang-tidy` (bugprone, cert, security, and concurrency checks) and `cppcheck` (full analysis with XML reporting).
- **Sanitizers**: Debug CI builds run with AddressSanitizer and UndefinedBehaviorSanitizer (`-fsanitize=address,undefined`) to catch memory leaks, buffer overflows, use-after-free, and undefined behavior at runtime.
- **Compiler Hardening**: Production builds use `-Wall -Wextra -Werror=format-security -Werror=implicit-function-declaration -Wformat=2 -Wshadow` to catch common mistakes at compile time.
- **CodeQL SAST**: GitHub's CodeQL semantic analysis runs on every PR, with results visible in the Security tab.
- **Container Scanning**: Published Docker images are scanned with Trivy for known CVEs in all dependencies (OpenSSL, FFmpeg, system libraries, etc.).
- **Unit & Integration Tests**: Comprehensive Unity-based C unit tests and Playwright-based UI integration tests run on every PR with code coverage tracking.

## NDAA Compliance

LightNVR contains **no foreign-manufactured components**. The entire codebase is:

- Open source and publicly auditable under GPL-3.0
- Developed in the United States
- Built from source in CI with full provenance (GitHub Actions build logs, SBOM via container scanning)
- Free of any proprietary blobs or binary-only dependencies

All third-party dependencies are well-known open-source libraries (FFmpeg, SQLite, cJSON, mbedTLS, libuv, libcurl) with transparent supply chains.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |
| < latest | :x:               |

Only the latest release receives security updates. We recommend always running the most recent version.

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them responsibly via email:

📧 **matt@opensensor.io**

Please include:

1. A description of the vulnerability
2. Steps to reproduce the issue
3. Potential impact assessment
4. Any suggested fixes (optional but appreciated)

### What to Expect

- **Acknowledgment**: Within 48 hours of your report
- **Assessment**: We will evaluate the severity and impact within 5 business days
- **Fix Timeline**: Critical vulnerabilities will be patched within 7 days; others within 30 days
- **Credit**: We will credit reporters in release notes (unless you prefer to remain anonymous)

## Security Tooling in CI

| Tool | What It Catches | Status |
|------|----------------|--------|
| `clang-tidy` | Buffer overflows, null dereferences, concurrency bugs, CERT C violations | Reporting (path to blocking) |
| `cppcheck` | Memory leaks, uninitialized variables, API misuse | Reporting |
| AddressSanitizer | Runtime memory errors, heap overflows, use-after-free, leaks | Blocking |
| UBSan | Undefined behavior (integer overflow, null pointer dereference, etc.) | Blocking |
| CodeQL | Semantic SAST (injection, crypto misuse, data flow vulnerabilities) | Reporting via Security tab |
| Trivy | Known CVEs in Docker image dependencies | Reporting via SARIF |
| Compiler warnings | Format string bugs, implicit declarations, shadowed variables | Blocking (`-Werror=...`) |

## Hardening Measures

- The application runs as a non-root user in Docker deployments
- SQLite databases use parameterized queries to prevent SQL injection
- Authentication is enforced on all API endpoints
- TLS support is available via mbedTLS, OpenSSL, or WolfSSL
- ONVIF authentication uses WS-Security with nonce and timestamp validation


## Media access and revocation

LightNVR's authenticated playback routes enforce camera permissions and separate
`audio.listen` and `audio.talk` rights. Managed go2rtc API and RTSP listeners are
loopback-only; exposing a separate service, tunnel, or bridge requires its own
access controls. Local processes and administrator-configured bridges remain
trusted. Recorded footage is governed by recording permissions, not live audio
permissions.

Use the compatible maintained go2rtc fork, including for externally managed
services. It validates opaque media leases against LightNVR on `127.0.0.1` in
the same network namespace. LightNVR must listen on an address reachable there;
a web listener bound only to a non-loopback interface causes playback to fail
closed. Incompatible forks and unavailable callbacks cannot silently allow
playback. See [deployment configuration](docs/CONFIGURATION.md#media-authority-deployment-requirements).

The fork checks established WebRTC, MSE, and HLS authority every five seconds,
with a two-second callback timeout. Permission changes or callback failure close
media within seven seconds under normal process scheduling. Native `/hls/...`
requests have a separate, at-most-30-second authorization cache. No revocation
can remove media already delivered to a client or buffered by its player.
