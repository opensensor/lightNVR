#!/usr/bin/env python3
"""Check that documentation points at things that exist.

Two classes of breakage, both of which have shipped to users before:

  1. Relative markdown links to files that do not exist - usually because a
     document was moved or a feature (and its doc) was removed.
  2. Paths the docs tell you to run or edit - scripts/*.sh, config/*.ini -
     that are no longer in the repository.

Nothing builds documentation, so neither is caught by any other job.

Run locally with no arguments; exits non-zero if anything is broken.
Emits GitHub Actions annotations when GITHUB_ACTIONS is set.
"""

import os
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

# Vendored or generated trees we do not own.
SKIP_PREFIXES = (
    "node_modules/",
    "third_party/",
    "go2rtc/",
    "web/node_modules/",
    ".claude/",
    "build/",
)

LINK_RE = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
# Paths a reader is told to execute or edit.
REFERENCED_PATH_RE = re.compile(r"\b(scripts/[\w.-]+\.sh|config/[\w.-]+\.ini)\b")

# Link targets we cannot resolve statically.
EXTERNAL_PREFIXES = ("http://", "https://", "mailto:", "#", "tel:")
PLACEHOLDER_CHARS = ("<", ">", "$", "{")

in_actions = bool(os.environ.get("GITHUB_ACTIONS"))


def report(path: str, line: int, message: str) -> None:
    if in_actions:
        print(f"::error file={path},line={line}::{message}")
    else:
        print(f"{path}:{line}: {message}")


def _git(*args: str) -> list[str]:
    return subprocess.run(
        ["git", *args], cwd=REPO, capture_output=True, text=True, check=True
    ).stdout.split()


def markdown_files() -> list[Path]:
    # Tracked files, plus untracked ones that are not gitignored - so a new
    # document with a broken link fails before it lands. Both git queries
    # respect .gitignore, which keeps local build/ and vendor trees out.
    found = set(_git("ls-files", "*.md"))
    found |= set(_git("ls-files", "--others", "--exclude-standard", "*.md"))

    return sorted(
        REPO / p
        for p in found
        if not p.startswith(SKIP_PREFIXES) and (REPO / p).is_file()
    )


def check() -> int:
    broken = 0

    for path in markdown_files():
        rel = path.relative_to(REPO).as_posix()
        try:
            lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError as exc:
            report(rel, 1, f"could not read: {exc}")
            broken += 1
            continue

        for lineno, line in enumerate(lines, 1):
            for target in LINK_RE.findall(line):
                target = target.strip().split(" ", 1)[0]  # drop optional "title"

                if not target or target.startswith(EXTERNAL_PREFIXES):
                    continue
                if any(c in target for c in PLACEHOLDER_CHARS):
                    continue  # e.g. lightnvr_<version>_<arch>.deb

                file_part = target.split("#", 1)[0]
                if not file_part:
                    continue

                if file_part.startswith("/"):
                    resolved = REPO / file_part.lstrip("/")
                else:
                    resolved = path.parent / file_part

                if not resolved.exists():
                    report(rel, lineno, f"broken link -> {target}")
                    broken += 1

            for ref in REFERENCED_PATH_RE.findall(line):
                if not (REPO / ref).exists():
                    report(rel, lineno, f"references '{ref}', which is not in the repository")
                    broken += 1

    if broken:
        print(f"\n{broken} broken documentation reference(s).", file=sys.stderr)
        return 1

    print("All documentation links and referenced paths resolve.")
    return 0


if __name__ == "__main__":
    sys.exit(check())
