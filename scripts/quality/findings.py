#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
"""Finding identity and exception policy checker (engineering-foundation T6).

Normalizes raw scanner reports (SARIF from CodeQL/Trivy, clang-tidy text,
cppcheck XML v2, eslint JSON or stylish job log, npm audit JSON, GitHub
Dependabot / Code Quality / security-advisory / secret-scanning REST JSON,
sanitizer ctest log) into line-independent fingerprints, then compares them with
the reviewed inventory (quality/findings.json) under quality/findings-policy.json.

Usage (from the repo root; Python 3 stdlib only):
  python3 scripts/quality/findings.py --check --reports build/eng-artifacts/<task> \
      [--inventory quality/findings.json] [--scanners quality/scanners.json] \
      [--policy quality/findings-policy.json] [--today YYYY-MM-DD] [--release] \
      [--json-out FILE]
  python3 scripts/quality/findings.py --update-baseline --reason "why" --reports ...

Expected scanners come from quality/scanners.json (report_path_convention,
report_format, scope, access_status).  A reports directory may also contain
run-manifest.json written by the pipeline:
  {"schema_version": 1, "revision": "<sha>",
   "scanners": {"<scanner id>": {"exit_code": 0, "status": "success",
       "reports": ["relative/path"], "tool_version": "...", "db_version": "...",
       "files_scanned": 123}}}
When require_run_manifest is enabled, --expected-revision is mandatory and each
entry must also include its revision, timezone-aware started_at/completed_at,
actual command exit records, report_sha256, and a supported scope with exact
files/file_sha256 (or immutable image targets). scanner_run.py produces this
metadata. Missing, stale, tampered or candidate-only evidence fails closed.
Recorded non-success status or an exit code outside the policy's
accepted_exit_codes is a scanner error even when a report file exists.

Exit codes (the checker's own status, independent of any scanner exit code):
  0  pass: every expected report present, valid, in scope, and no policy failure
  1  policy failure (new finding, bucket increase, expired exception, missing /
     crashed / empty-scope / invalid report, inventory or policy error)
  2  usage or unreadable input (bad arguments, missing inventory/policy/reports dir)
  3  internal checker error (traceback printed; never treated as a verdict)
--check never writes the inventory.  --update-baseline is the only writer, and it
requires --reason, refuses while any coverage/config failure exists, and appends
an auditable baseline_edits record.
"""
import argparse
import collections
import datetime as dt
import hashlib
import json
import os
import re
import sys
import traceback
import xml.etree.ElementTree as ET
from pathlib import Path

EXIT_PASS, EXIT_FAIL, EXIT_USAGE, EXIT_INTERNAL = 0, 1, 2, 3

KIND_CLASS = {
    # coverage: the scan itself cannot be trusted as a complete result
    "missing-report": "coverage", "not-scanned": "coverage", "ambiguous-report": "coverage",
    "invalid-report": "coverage", "scanner-error": "coverage", "truncated-report": "coverage",
    "empty-scope": "coverage", "scope-mismatch": "coverage", "missing-run-manifest": "coverage",
    "invalid-run-manifest": "coverage",
    # findings: the scan is valid and reports something not accepted by the inventory
    "new-finding": "finding", "bucket-new": "finding", "bucket-increase": "finding",
    "exposure-escalation": "finding", "release-blocker": "finding", "gate-failed": "finding",
    # exceptions
    "expired-exception": "exception",
    # configuration of the gate itself
    "inventory-error": "config", "policy-error": "config",
}
GAP_LABEL = {
    "not-scanned": "NOT SCANNED", "missing-report": "REPORT MISSING", "scanner-error": "SCANNER ERROR",
    "empty-scope": "EMPTY SCOPE", "truncated-report": "TRUNCATED REPORT", "invalid-report": "INVALID REPORT",
    "ambiguous-report": "AMBIGUOUS REPORT", "scope-mismatch": "SCOPE MISMATCH",
    "missing-run-manifest": "NO RUN MANIFEST",
}
FATAL_KINDS = {"missing-report", "not-scanned", "ambiguous-report", "invalid-report", "empty-scope", "scope-mismatch", "invalid-run-manifest", "missing-run-manifest"}
DEFAULT_STRIP_PREFIXES = ["/__w/lightNVR/lightNVR/", "/home/runner/work/lightNVR/lightNVR/"]
SUCCESS_STATUSES = {"success", "succeeded", "ok", "passed", "completed"}


# --------------------------------------------------------------------------
# identity helpers (identical to the T1 generator, build/eng-artifacts/T1/generator)
# --------------------------------------------------------------------------
def fp(*parts):
    return hashlib.sha256("|".join(str(p) for p in parts).encode()).hexdigest()[:16]


def norm_msg(m):
    return re.sub(r"\d+", "N", (m or "").strip())


def line_tool_fingerprint(tool, rule, path, msg, occurrence):
    """T1 'line-tool' scheme: no line numbers, digits in messages normalized."""
    return f"{tool}:{rule}:{path}:{fp(rule, path, norm_msg(msg), occurrence)}"


class UsageError(Exception):
    pass


class ReportIssue(Exception):
    def __init__(self, kind, detail, marker=None):
        super().__init__(detail)
        self.kind, self.detail, self.marker = kind, detail, marker


def parse_date(value, what):
    if isinstance(value, dt.date):
        return value
    try:
        return dt.date.fromisoformat(str(value))
    except ValueError:
        raise ValueError(f"{what}: invalid ISO date {value!r}")


# --------------------------------------------------------------------------
# observation container
# --------------------------------------------------------------------------
class Obs:
    def __init__(self, scanner, ctx):
        self.scanner = scanner
        self.ctx = ctx
        self.items = {}                     # fingerprint -> record
        self.buckets = collections.Counter()  # fingerprint -> count
        self.bucket_meta = {}
        self.variants = collections.defaultdict(set)
        self.provenance = {}
        self.issues = []                    # dicts kind/detail/marker/fatal
        self.scope_evidence = None          # description of non-empty scope, if known
        self.zero_ok = False                # True when the format itself proves scope for empty results
        self.notes = []

    def issue(self, kind, detail, marker=None):
        self.issues.append({"kind": kind, "detail": detail, "marker": marker, "fatal": kind in FATAL_KINDS})

    def item(self, fingerprint, rule, path, line=None, extra=None):
        rec = {"fingerprint": fingerprint, "rule_id": rule, "location": {"path": path, "line": line}}
        if extra:
            rec.update(extra)
        self.items[fingerprint] = rec
        return rec

    def bucket(self, fingerprint, rule, path, n=1):
        self.buckets[fingerprint] += n
        self.bucket_meta[fingerprint] = {"rule_id": rule, "path": path}

    def canon(self, path):
        return self.ctx.canon(path)


class Ctx:
    def __init__(self, policy):
        self.strip = list(policy.get("strip_path_prefixes") or DEFAULT_STRIP_PREFIXES)
        self.renames = []
        for r in policy.get("path_renames") or []:
            self.renames.append((r["from"], r["to"]))
        self.renamed = {}

    def canon(self, path):
        if path is None:
            return None
        p = str(path)
        for pre in self.strip:
            if p.startswith(pre):
                p = p[len(pre):]
                break
        for old, new in self.renames:
            if new.endswith("/") and p.startswith(new):
                self.renamed[p] = old + p[len(new):]
                return old + p[len(new):]
            if p == new:
                self.renamed[p] = old
                return old
        return p


def read_json(path):
    try:
        with open(path, encoding="utf-8") as fh:
            return json.load(fh)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as e:
        raise ReportIssue("invalid-report", f"{path.name}: unparseable JSON ({e.__class__.__name__}: {e})")


def read_text(path):
    try:
        return path.read_text(encoding="utf-8", errors="replace")
    except OSError as e:
        raise ReportIssue("invalid-report", f"{path.name}: unreadable ({e})")


def api_error_check(doc, path):
    if isinstance(doc, dict) and "message" in doc and ("status" in doc or "documentation_url" in doc):
        status = doc.get("status", "?")
        raise ReportIssue("scanner-error", f"{path.name}: API error body, not a result (HTTP {status}: {doc['message']})",
                          marker=f"api-error-{status}")


# --------------------------------------------------------------------------
# normalizers
# --------------------------------------------------------------------------
def _sarif_runs(path):
    doc = read_json(path)
    runs = doc.get("runs") if isinstance(doc, dict) else None
    if not isinstance(runs, list) or not runs:
        raise ReportIssue("invalid-report", f"{path.name}: SARIF without runs")
    return runs


def _sarif_common(run, obs, path):
    drv = ((run.get("tool") or {}).get("driver")) or {}
    if not drv.get("name"):
        raise ReportIssue("invalid-report", f"{path.name}: SARIF run without tool.driver.name")
    obs.provenance.setdefault("tool", drv.get("name"))
    ver = drv.get("semanticVersion") or drv.get("version")
    if ver:
        obs.provenance["version"] = ver
    exts = [f"{e.get('name')} {e.get('semanticVersion') or e.get('version') or '?'}"
            for e in (run.get("tool") or {}).get("extensions") or []]
    if exts:
        obs.provenance["extensions"] = exts
    for v in run.get("versionControlProvenance") or []:
        if v.get("revisionId"):
            obs.provenance["revision"] = v["revisionId"][:12]
    for inv in run.get("invocations") or []:
        if inv.get("executionSuccessful") is False:
            obs.issue("scanner-error", f"{path.name}: invocation executionSuccessful=false", "execution-unsuccessful")
        for n in inv.get("toolExecutionNotifications") or []:
            if n.get("level") == "error":
                obs.issue("scanner-error", f"{path.name}: tool error notification: "
                          f"{((n.get('message') or {}).get('text') or '')[:160]}", "tool-error-notification")
    if not isinstance(run.get("results"), list):
        raise ReportIssue("invalid-report", f"{path.name}: SARIF run without results array")


def norm_codeql(paths, sc, obs):
    want = ((sc.get("scope") or {}).get("category") or "").rstrip("/")
    for path in paths:
        for run in _sarif_runs(path):
            _sarif_common(run, obs, path)
            cat = ((run.get("automationDetails") or {}).get("id") or "").rstrip("/")
            if want and cat and not cat.startswith(want):
                obs.issue("scope-mismatch", f"{path.name}: SARIF category {cat!r} != expected {want!r}")
            if run.get("artifacts"):
                obs.scope_evidence = f"{len(run['artifacts'])} SARIF artifacts"
            for res in run["results"]:
                rule = res.get("ruleId") or (res.get("rule") or {}).get("id")
                loc = res["locations"][0]["physicalLocation"]
                p = obs.canon(loc["artifactLocation"]["uri"])
                line = (loc.get("region") or {}).get("startLine")
                lh = (res.get("partialFingerprints") or {}).get("primaryLocationLineHash")
                if lh:
                    f = f"codeql:{rule}:{p}:{lh}"
                else:
                    f = f"codeql:{rule}:{p}:msg-{fp(norm_msg((res.get('message') or {}).get('text')))}"
                    obs.notes.append(f"{f}: no primaryLocationLineHash; message-hash fallback")
                obs.item(f, rule, p, line)


def norm_trivy(paths, sc, obs):
    want = ((sc.get("scope") or {}).get("image") or "").split(" ")[0]  # "lightnvr:scan built from ./Dockerfile"
    for path in paths:
        for run in _sarif_runs(path):
            _sarif_common(run, obs, path)
            props = run.get("properties") or {}
            target = props.get("imageName") or props.get("imageID") or props.get("ArtifactName")
            if not target:
                obs.issue("empty-scope", f"{path.name}: no scan target recorded (properties.imageName/imageID missing)")
                continue
            if want and props.get("imageName") and props["imageName"] != want:
                obs.issue("scope-mismatch", f"{path.name}: scanned image {props['imageName']!r} != expected {want!r}")
            obs.scope_evidence = f"image {props.get('imageName')} {str(props.get('imageID', ''))[:19]}"
            obs.zero_ok = True
            for res in run["results"]:
                rule = res.get("ruleId")
                loc = ((res.get("locations") or [{}])[0].get("physicalLocation") or {})
                uri = obs.canon((loc.get("artifactLocation") or {}).get("uri") or target)
                m = re.search(r"Package: (\S+)", (res.get("message") or {}).get("text") or "")
                pkg = m.group(1) if m else ""
                obs.item(f"trivy:{uri}:{pkg}:{rule}", rule, uri, None, {"package": pkg})


CT_RE = re.compile(r"^(?P<file>\S+?):(?P<line>\d+):(?P<col>\d+): (?P<sev>warning|error): (?P<msg>.*) \[(?P<chk>[A-Za-z0-9.,_-]+)\]$")
CT_PROGRESS = re.compile(r"^\[(\d+)/(\d+)\] Processing file (\S+?)\.?$")
CRASH_MARKERS = ("Stack dump:", "PLEASE submit a bug report", "Segmentation fault", "LLVM ERROR:",
                 "Aborted (core dumped)", "terminate called after throwing")


def norm_clang_tidy(paths, sc, obs):
    for path in paths:
        text = read_text(path)
        items, files, max_idx, total = [], set(), 0, 0
        for raw in text.splitlines():
            m = CT_PROGRESS.match(raw)
            if m:
                max_idx, total = max(max_idx, int(m[1])), max(total, int(m[2]))
                files.add(m[3])
                continue
            for marker in CRASH_MARKERS:
                if raw.startswith(marker) or marker in raw[:80]:
                    obs.issue("scanner-error", f"{path.name}: crash marker {marker!r}", "crash")
            m = CT_RE.match(raw)
            if m:
                items.append((obs.canon(m["file"]), int(m["line"]), m["sev"], m["chk"], m["msg"]))
        if not files and not items:
            obs.issue("empty-scope", f"{path.name}: no files processed and no diagnostics (empty or crashed run)")
            continue
        if not files:
            obs.issue("truncated-report", f"{path.name}: diagnostics without run-clang-tidy progress lines")
        elif max_idx < total:
            obs.issue("truncated-report", f"{path.name}: progress stopped at {max_idx}/{total}")
        obs.scope_evidence = f"{len(files)} files processed (declared {total})"
        obs.provenance["files_scanned"] = len(files)
        if any(i[3] == "clang-diagnostic-error" for i in items):
            n = sum(1 for i in items if i[3] == "clang-diagnostic-error")
            obs.issue("scanner-error", f"{path.name}: {n} clang-diagnostic-error (translation units not fully analyzed)",
                      "compile-error")
        items.sort()
        occ = collections.Counter()
        for p, line, sev, chk, msg in items:
            key = (chk, p, norm_msg(msg))
            occ[key] += 1
            obs.item(line_tool_fingerprint("clang-tidy", chk, p, msg, occ[key]), chk, p, line, {"severity": sev})


CPPCHECK_INTERNAL = {"internalError", "internalAstError", "cppcheckError", "premiumInternalError", "syntaxErrorInternal"}


def norm_cppcheck(paths, sc, obs):
    for path in paths:
        try:
            root = ET.parse(path).getroot()
        except (ET.ParseError, OSError) as e:
            raise ReportIssue("invalid-report", f"{path.name}: unparseable XML ({e})")
        if root.tag != "results" or root.get("version") != "2":
            raise ReportIssue("invalid-report", f"{path.name}: not cppcheck XML v2")
        ver = root.find("cppcheck")
        if ver is None or not ver.get("version"):
            raise ReportIssue("invalid-report", f"{path.name}: no <cppcheck version>")
        obs.provenance.update(tool="cppcheck", version=ver.get("version"))
        errs_el = root.find("errors")
        if errs_el is None:
            raise ReportIssue("invalid-report", f"{path.name}: no <errors> element")
        errs = errs_el.findall("error")
        if not errs:
            obs.issue("empty-scope", f"{path.name}: no <error> entries at all (not even checkersReport); "
                      "cannot distinguish a clean scan from an empty scope")
            continue
        files = {loc.get("file") for e in errs for loc in e.findall("location")}
        obs.scope_evidence = f"{len(files)} files with diagnostics"
        for e in errs:
            if e.get("id") in CPPCHECK_INTERNAL:
                obs.issue("scanner-error", f"{path.name}: cppcheck {e.get('id')}: {e.get('msg', '')[:120]}", "internal-error")
            if e.get("id") == "checkersReport" and "critical error" in (e.get("msg") or ""):
                obs.issue("scanner-error", f"{path.name}: checkersReport says 'There was critical errors' "
                          "(some files/configurations not analyzed)", "critical-errors")

        def sort_key(e):
            loc = e.find("location")
            return (loc.get("file") if loc is not None else "", int(loc.get("line")) if loc is not None else 0, e.get("id"))

        occ = collections.Counter()
        for e in sorted(errs, key=sort_key):
            loc = e.find("location")
            p = obs.canon(loc.get("file") if loc is not None else (e.get("file0") or ""))
            line = int(loc.get("line")) if loc is not None else None
            sev, rid = e.get("severity"), e.get("id")
            if sev in ("style", "information"):
                obs.bucket(f"cppcheck-bucket:{rid}:{p}", rid, p)
                continue
            key = (rid, p, norm_msg(e.get("msg")))
            occ[key] += 1
            obs.item(f"cppcheck:{rid}:{p}:{fp(*key, occ[key])}", rid, p, line, {"severity": sev})


ES_STEP = "Report JSX accessibility findings"
ES_FILE = re.compile(r"(?:^|Z )(?P<f>/\S+\.(?:js|jsx|ts|tsx|mjs|cjs))\s*$")
ES_WARN = re.compile(r"\s(\d+):(\d+)\s+(warning|error)\s+.*\s(\S+)$")
ES_SUMMARY = re.compile(r"✖ (\d+) problems? \((\d+) errors?, (\d+) warnings?\)")


def norm_eslint(paths, sc, obs):
    obs.provenance.setdefault("tool", "eslint")
    for path in paths:
        text = read_text(path)
        if text.lstrip().startswith("["):
            doc = read_json(path)
            if not isinstance(doc, list):
                raise ReportIssue("invalid-report", f"{path.name}: eslint JSON is not a list")
            if not doc:
                obs.issue("empty-scope", f"{path.name}: eslint JSON lists no linted files")
                continue
            obs.scope_evidence = f"{len(doc)} files linted"
            obs.zero_ok = True
            obs.provenance["files_scanned"] = len(doc)
            for fr in doc:
                p = obs.canon(fr.get("filePath"))
                for m in fr.get("messages") or []:
                    if m.get("fatal"):
                        obs.issue("scanner-error", f"{path.name}: fatal eslint message in {p}: {str(m.get('message'))[:120]}",
                                  "fatal-parse-error")
                        continue
                    rule = m.get("ruleId")
                    if not rule:
                        obs.notes.append(f"eslint message without ruleId in {p}: {str(m.get('message'))[:80]}")
                        continue
                    obs.bucket(f"eslint-bucket:{rule}:{p}", rule, p)
            continue
        # stylish text inside a GitHub Actions job log
        lines = text.splitlines()
        if any(ES_STEP in l for l in lines):
            lines = [l for l in lines if ES_STEP in l]
        if any("Oops! Something went wrong" in l for l in lines):
            obs.issue("scanner-error", f"{path.name}: eslint crashed ('Oops! Something went wrong')", "crash")
        invoked = any(re.search(r"> eslint\b", l) for l in lines)
        cur, n, summary = None, 0, None
        for l in lines:
            m = ES_SUMMARY.search(l)
            if m:
                summary = (int(m[1]), int(m[2]), int(m[3]))
                continue
            m = ES_FILE.search(l)
            if m:
                cur = obs.canon(m["f"])
                continue
            m = ES_WARN.search(l)
            if m and cur:
                obs.bucket(f"eslint-bucket:{m.group(4)}:{cur}", m.group(4), cur)
                n += 1
        if summary is not None:
            if summary[0] != n:
                raise ReportIssue("invalid-report", f"{path.name}: parsed {n} problems but eslint summary says {summary[0]} "
                                  "(log format drift; refusing partial parse)")
            obs.scope_evidence = f"eslint summary {summary[0]} problems"
        elif n:
            raise ReportIssue("invalid-report", f"{path.name}: {n} problems parsed but no eslint summary line (truncated log?)")
        elif not invoked:
            obs.issue("empty-scope", f"{path.name}: no eslint invocation found in log")
        else:
            obs.scope_evidence = "eslint invoked; no problems reported"
            obs.zero_ok = True


def _npm_lockfile(sc):
    lock = (sc.get("scope") or {}).get("lockfile")
    if lock:
        return lock
    sid = sc["id"]
    return "package-lock.json" if sid.endswith("-root") else f"{sid.split('npm-audit-', 1)[-1]}/package-lock.json"


def norm_npm_audit(paths, sc, obs):
    lock = _npm_lockfile(sc)
    obs.provenance.setdefault("tool", "npm audit")
    for path in paths:
        variant = "prod" if re.search(r"-prod\b", path.name) else "all"
        doc = read_json(path)
        if not isinstance(doc, dict):
            raise ReportIssue("invalid-report", f"{path.name}: npm audit JSON is not an object")
        if "error" in doc:
            err = doc["error"] if isinstance(doc["error"], dict) else {"summary": str(doc["error"])}
            raise ReportIssue("scanner-error", f"{path.name}: npm audit error {err.get('code', '')}: {str(err.get('summary', ''))[:160]}",
                              marker=f"npm-error-{err.get('code', 'unknown')}")
        if doc.get("auditReportVersion") != 2:
            raise ReportIssue("invalid-report", f"{path.name}: auditReportVersion {doc.get('auditReportVersion')!r} != 2")
        obs.provenance["report_format_version"] = 2
        total = (((doc.get("metadata") or {}).get("dependencies")) or {}).get("total")
        if not total:
            obs.issue("empty-scope", f"{path.name}: metadata.dependencies.total={total!r} (no dependencies audited)")
            continue
        obs.scope_evidence = f"{total} dependencies audited"
        obs.zero_ok = True
        for name, v in (doc.get("vulnerabilities") or {}).items():
            for via in v.get("via") or []:
                if isinstance(via, dict):
                    ghsa = str(via.get("url", "")).rsplit("/", 1)[-1] or str(via.get("source"))
                    f = f"npm-audit:{lock}:{name}:{ghsa}"
                    obs.item(f, ghsa, lock, None, {"package": name})
                    obs.variants[f].add(variant)


def _api_list(path):
    doc = read_json(path)
    api_error_check(doc, path)
    if not isinstance(doc, list):
        raise ReportIssue("invalid-report", f"{path.name}: expected a JSON list of alerts")
    return doc


def norm_dependabot(paths, sc, obs):
    obs.provenance.setdefault("tool", "GitHub Dependabot alerts")
    for path in paths:
        for a in _api_list(path):
            if a.get("state") not in (None, "open"):
                continue
            dep, adv = a["dependency"], a["security_advisory"]
            m = obs.canon(dep["manifest_path"])
            f = f"dependabot:{m}:{dep['package']['name']}:{adv['ghsa_id']}"
            obs.item(f, adv["ghsa_id"], m, None, {"package": dep["package"]["name"]})
        obs.zero_ok = True


def norm_code_quality(paths, sc, obs):
    obs.provenance.setdefault("tool", "GitHub Code Quality")
    for path in paths:
        for a in _api_list(path):
            if a.get("state") not in (None, "open"):
                continue
            loc = a["location"]
            p = obs.canon(loc["path"])
            f = f"code-quality:{a['rule']['id']}:{p}:{fp(norm_msg(a['message']['text']))}"
            obs.item(f, a["rule"]["id"], p, loc.get("start_line"))
        obs.zero_ok = True


def norm_advisories(paths, sc, obs):
    obs.provenance.setdefault("tool", "GitHub repository security advisories")
    for path in paths:
        for a in _api_list(path):
            f = f"repo-advisory:{hashlib.sha256(a['ghsa_id'].encode()).hexdigest()[:16]}"
            obs.item(f, "repository-security-advisory", None, None)  # never echo private IDs
        obs.zero_ok = True


def norm_secret(paths, sc, obs):
    obs.provenance.setdefault("tool", "GitHub secret scanning")
    for path in paths:
        for a in _api_list(path):
            if a.get("state") not in (None, "open"):
                continue
            # never the secret value
            f = f"secret:{a.get('secret_type', 'unknown')}:alert-{a.get('number')}"
            obs.item(f, a.get("secret_type"), None, None)
        obs.zero_ok = True


def norm_sanitizer(paths, sc, obs):
    obs.provenance.setdefault("tool", "ctest + ASan/UBSan")
    for path in paths:
        text = read_text(path)
        for marker in ("ERROR: AddressSanitizer", "ERROR: LeakSanitizer", "runtime error:"):
            if marker in text:
                obs.issue("gate-failed", f"{path.name}: sanitizer report {marker!r}", "sanitizer")
        if "No tests were found" in text:
            obs.issue("empty-scope", f"{path.name}: ctest found no tests")
            continue
        m = re.search(r"(\d+)% tests passed, (\d+) tests? failed out of (\d+)", text)
        if not m:
            raise ReportIssue("invalid-report", f"{path.name}: no ctest summary line (crashed or truncated log)")
        if int(m[3]) == 0:
            obs.issue("empty-scope", f"{path.name}: ctest ran 0 tests")
        elif int(m[2]):
            obs.issue("gate-failed", f"{path.name}: {m[2]} of {m[3]} tests failed", "ctest")
        obs.scope_evidence = f"{m[3]} tests"
        obs.zero_ok = True


def choose_normalizer(sc):
    sid, fmt, kind = sc["id"], (sc.get("report_format") or "").lower(), (sc.get("kind") or "").lower()
    if sid.startswith("trivy") or (fmt.startswith("sarif") and kind.startswith("container")):
        return norm_trivy
    if fmt.startswith("sarif"):
        return norm_codeql
    if sid.startswith("clang-tidy") or "clang-diagnostic" in fmt:
        return norm_clang_tidy
    if sid.startswith("cppcheck") or "cppcheck-xml" in fmt:
        return norm_cppcheck
    if sid.startswith("eslint") or fmt.startswith("eslint"):
        return norm_eslint
    if sid.startswith("npm-audit") or "npm-audit" in fmt:
        return norm_npm_audit
    if "dependabot" in sid or "dependabot" in fmt:
        return norm_dependabot
    if "code-quality" in sid or "code-quality" in fmt:
        return norm_code_quality
    if "advisor" in sid:
        return norm_advisories
    if "secret" in sid:
        return norm_secret
    if "sanitizer" in sid:
        return norm_sanitizer
    return None


# --------------------------------------------------------------------------
# report discovery
# --------------------------------------------------------------------------
def expand_convention(conv):
    conv = conv.split(" (")[0].strip()
    conv = re.sub(r"^build/eng-artifacts/<task>/", "", conv)
    m = re.search(r"\{([^{}]*,[^{}]*)\}", conv)
    if not m:
        return [conv]
    out = []
    for alt in m[1].split(","):
        out.extend(expand_convention(conv[:m.start()] + alt + conv[m.end():]))
    return out


def convention_regex(rel):
    rx = ""
    for part in re.split(r"(<[a-z0-9_]+>)", rel):
        if part == "<sha8>":
            rx += "[0-9a-f]{7,40}"
        elif part == "<run_id>":
            rx += "[0-9]+"
        elif part.startswith("<") and part.endswith(">"):
            rx += "[^/]+"
        else:
            rx += re.escape(part)
    return re.compile(rx)


def discover(reports_dir, sc, pins, manifest_entry):
    """Return (list of paths, list of issues, source label)."""
    if pins:
        rels, src = pins, "policy report_pins"
    elif manifest_entry and manifest_entry.get("reports"):
        rels, src = manifest_entry["reports"], "run-manifest.json"
    else:
        conv = sc.get("report_path_convention")
        if not conv:
            return [], [("missing-report", "scanners.json has no report_path_convention")], "none"
        paths, issues = [], []
        for rel in expand_convention(conv):
            rx = convention_regex(rel)
            glob = re.sub(r"<[a-z0-9_]+>", "*", rel)
            found = sorted(p for p in reports_dir.glob(glob)
                           if p.is_file() and rx.fullmatch(p.relative_to(reports_dir).as_posix()))
            if not found:
                issues.append(("missing-report", f"no report matching {rel}"))
            elif len(found) > 1:
                issues.append(("ambiguous-report", f"{len(found)} reports match {rel}: "
                               + ", ".join(p.relative_to(reports_dir).as_posix() for p in found)
                               + " (pin one in findings-policy.json report_pins or list it in run-manifest.json)"))
            else:
                paths.append(found[0])
        return paths, issues, "convention"
    paths, issues = [], []
    for rel in rels:
        try:
            p = _manifest_path(reports_dir, rel)
        except ValueError as error:
            issues.append(("invalid-run-manifest", str(error)))
            continue
        if p.is_file():
            paths.append(p)
        else:
            issues.append(("missing-report", f"{rel} ({src}) does not exist"))
    return paths, issues, src


# --------------------------------------------------------------------------
# evaluation
# --------------------------------------------------------------------------
def _version_tokens(s):
    return re.findall(r"\d+(?:\.\d+)+", str(s or ""))


def _manifest_path(base, relative):
    """Do not follow a report manifest outside its artifact directory."""
    if not isinstance(relative, str) or not relative or "\\" in relative or Path(relative).is_absolute() \
            or any(part in ("..", ".", "") for part in relative.split("/")):
        raise ValueError("unsafe manifest report path")
    target = Path(base).resolve()
    for part in relative.split("/"):
        target = target / part
        if target.is_symlink():
            raise ValueError(f"symlink is not report evidence: {relative}")
    if not target.resolve().is_relative_to(Path(base).resolve()):
        raise ValueError("manifest report escapes report directory")
    return target


def _manifest_entry_errors(entry, reports_dir, revision, accepted_codes):
    errors = []
    if not isinstance(entry, dict):
        return ["scanner manifest entry must be an object"]
    for key in ("tool_version", "status"):
        if not isinstance(entry.get(key), str) or not entry[key].strip():
            errors.append(f"required manifest {key} is missing/empty")
    if type(entry.get("exit_code")) is not int:
        errors.append("required manifest exit_code must be an integer")
    if entry.get("revision") != revision:
        errors.append("scanner manifest revision does not match run revision")
    try:
        started = dt.datetime.fromisoformat(entry["started_at"])
        completed = dt.datetime.fromisoformat(entry["completed_at"])
        if not started.tzinfo or not completed.tzinfo or completed < started:
            raise ValueError("invalid interval")
    except (KeyError, TypeError, ValueError):
        errors.append("manifest requires a timezone-aware ordered execution interval")
    commands = entry.get("commands")
    if not isinstance(commands, list) or not commands:
        errors.append("manifest requires actual command execution records")
    else:
        for command in commands:
            if not isinstance(command, dict) or type(command.get("exit_code")) is not int \
                    or command.get("exit_code") not in accepted_codes or command.get("timed_out") is not False \
                    or not isinstance(command.get("argv"), list) or not command["argv"] \
                    or any(not isinstance(arg, str) for arg in command["argv"]):
                errors.append("manifest command failed, timed out, or lacks an exit status")
    reports = entry.get("reports")
    hashes = entry.get("report_sha256")
    if not isinstance(reports, list) or not reports or any(not isinstance(x, str) for x in reports) \
            or len(set(reports)) != len(reports) or not isinstance(hashes, dict) or set(hashes) != set(reports):
        errors.append("manifest requires unique report paths and matching report hashes")
    else:
        for relative in reports:
            try:
                file = _manifest_path(reports_dir, relative)
                if not re.fullmatch(r"[0-9a-f]{64}", str(hashes[relative])) or hashlib.sha256(file.read_bytes()).hexdigest() != hashes[relative]:
                    errors.append(f"report hash mismatch: {relative}")
            except (OSError, ValueError) as exc:
                errors.append(f"invalid manifest report: {exc}")
    scope = entry.get("scope")
    if not isinstance(scope, dict):
        errors.append("manifest requires actual scope evidence")
        return errors
    kind = scope.get("kind")
    if kind == "image":
        targets = scope.get("targets")
        if not isinstance(targets, list) or not targets or any(not isinstance(x, dict) or not x.get("name") or not x.get("digest") for x in targets):
            errors.append("image scope requires tool-reported names and immutable digests")
    elif isinstance(kind, str) and kind in {"eslint-json", "invocation-files", "npm-lockfiles", "file-list-report", "codeql-source-archive", "cppcheck-progress"}:
        files, hashes = scope.get("files"), scope.get("file_sha256")
        if not isinstance(files, list) or not files or any(not isinstance(x, str) for x in files) \
                or len(set(files)) != len(files) or not isinstance(hashes, dict) or set(hashes) != set(files):
            errors.append("scope requires nonempty unique actual scanned files and source hashes")
        else:
            if type(entry.get("files_scanned")) is not int or entry["files_scanned"] != len(files):
                errors.append("files_scanned must equal the actual scanned-file list length")
            for name in files:
                if Path(name).is_absolute() or "\\" in name or any(x in (".", "..", "") for x in name.split("/")) \
                        or not re.fullmatch(r"[0-9a-f]{64}", str(hashes[name])):
                    errors.append("scope contains invalid source path/hash")
        if kind == "codeql-source-archive" and not re.fullmatch(r"[0-9a-f]{64}", str(scope.get("archive_sha256", ""))):
            errors.append("CodeQL scope requires its actual source archive hash")
    else:
        errors.append("candidate-only or unsupported evidence cannot prove scanned scope")
    evidence = scope.get("evidence_sha256", {})
    if not isinstance(evidence, dict):
        errors.append("scope evidence hashes must be an object")
        evidence = {}
    for relative, expected in evidence.items():
        try:
            if hashlib.sha256(_manifest_path(reports_dir, relative).read_bytes()).hexdigest() != expected:
                errors.append(f"scope evidence hash mismatch: {relative}")
        except (OSError, ValueError) as exc:
            errors.append(f"invalid scope evidence: {exc}")
    return errors


def evaluate(reports_dir, inventory, scanners, policy, today, release=False, reports_key=None, expected_revision=None):
    reports_dir = Path(reports_dir)
    today = parse_date(today, "today")
    reports_key = reports_key or reports_dir.as_posix()
    failures, info, accepted, prov_changes = [], [], [], []

    def fail(kind, detail, scanner=None, fingerprint=None):
        failures.append({"kind": kind, "class": KIND_CLASS.get(kind, "config"), "scanner": scanner,
                         "fingerprint": fingerprint, "detail": detail})

    dispositions = policy.get("dispositions") or {}
    if not dispositions:
        fail("policy-error", "policy has no dispositions table")
    warn_days = int(policy.get("expiry_warning_days", 30))

    def approved_exception(record, where, kind, scanner=None, fingerprint=None):
        """Approval is recorded explicitly; a proposed owner/expiry is not consent."""
        def reject(detail):
            fail(kind, f"{where}: {detail}", scanner, fingerprint)
            return False

        for key in ("owner", "reason"):
            if not isinstance(record.get(key), str) or not record[key].strip():
                return reject(f"exception requires a nonempty {key}")
        approval = record.get("approval")
        if not isinstance(approval, dict) or approval.get("status") != "approved":
            return reject("exception has no recorded approval (proposals are not acceptance)")
        if record.get("status") not in (None, "approved"):
            return reject("exception status conflicts with recorded approval")
        for key in ("approved_by", "approved_on", "evidence"):
            if not isinstance(approval.get(key), str) or not approval[key].strip():
                return reject(f"approval requires a nonempty {key}")
        try:
            approved_on = parse_date(approval["approved_on"], f"{where}.approval.approved_on")
        except ValueError as exc:
            return reject(str(exc))
        if approved_on > today:
            return reject("approval date is in the future")
        return True

    # ---- policy: accepted gaps ----
    gaps = []
    for i, g in enumerate(policy.get("accepted_gaps") or []):
        missing = [k for k in ("scanner", "condition", "owner", "expires", "reason") if not g.get(k)]
        if missing:
            fail("policy-error", f"accepted_gaps[{i}] missing {', '.join(missing)}", g.get("scanner"))
            continue
        try:
            g = dict(g, _expires=parse_date(g["expires"], f"accepted_gaps[{i}].expires"))
        except ValueError as e:
            fail("policy-error", str(e), g.get("scanner"))
            continue
        if not approved_exception(g, f"accepted_gaps[{i}]", "policy-error", g.get("scanner")):
            continue
        g["_used"] = False
        gaps.append(g)

    # ---- inventory validation and exception expiry ----
    inv_items, inv_buckets = {}, {}
    expected_ids = {s["id"] for s in scanners.get("scanners") or []}
    for coll, store in (("findings", inv_items), ("buckets", inv_buckets)):
        for idx, r in enumerate(inventory.get(coll) or []):
            f = r.get("fingerprint")
            where = f"{coll}[{idx}] {r.get('id', '')}".strip()
            if not f:
                fail("inventory-error", f"{where}: no fingerprint")
                continue
            if f in inv_items or f in inv_buckets:
                fail("inventory-error", f"{where}: duplicate fingerprint", r.get("scanner_id"), f)
                continue
            store[f] = r
            if r.get("scanner_id") not in expected_ids:
                fail("inventory-error", f"{where}: scanner_id {r.get('scanner_id')!r} not in scanners.json", r.get("scanner_id"), f)
            d = r.get("disposition")
            if d not in dispositions:
                fail("inventory-error", f"{where}: unknown disposition {d!r}", r.get("scanner_id"), f)
                continue
            exp = r.get("exception_expiry")
            if dispositions[d].get("exception"):
                approved_exception(r, where, "inventory-error", r.get("scanner_id"), f)
            if dispositions[d].get("exception") and not exp:
                fail("inventory-error", f"{where}: disposition {d!r} is an exception but has no exception_expiry "
                     "(unbounded exceptions are not allowed)", r.get("scanner_id"), f)
            if exp:
                try:
                    e = parse_date(exp, f"{where}.exception_expiry")
                except ValueError as err:
                    fail("inventory-error", str(err), r.get("scanner_id"), f)
                    continue
                if today > e:
                    fail("expired-exception", f"{where}: {d} exception expired {exp} (owner {r.get('owner', '?')}); "
                         "re-review and edit the inventory explicitly", r.get("scanner_id"), f)
                elif (e - today).days <= warn_days:
                    info.append({"kind": "expiring-soon", "scanner": r.get("scanner_id"), "fingerprint": f,
                                 "detail": f"exception expires {exp}"})
            if coll == "buckets" and (not isinstance(r.get("count"), int) or r["count"] < 0):
                fail("inventory-error", f"{where}: bucket count must be a non-negative integer", r.get("scanner_id"), f)

    # ---- run manifest ----
    manifest, manifest_path = None, reports_dir / "run-manifest.json"
    if manifest_path.is_symlink():
        fail("invalid-run-manifest", "run-manifest.json must not be a symlink")
    elif manifest_path.is_file():
        try:
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            if not isinstance(manifest, dict) or not isinstance(manifest.get("scanners"), dict):
                raise ValueError("'scanners' must be an object")
        except (ValueError, OSError) as e:
            fail("invalid-run-manifest", f"run-manifest.json unreadable: {e}")
            manifest = None
    manifest_errors = []
    if policy.get("require_run_manifest"):
        if not expected_revision:
            manifest_errors.append("required provenance needs an explicit --expected-revision")
        if manifest:
            if type(manifest.get("schema_version")) is not int or manifest.get("schema_version") != 1:
                manifest_errors.append("unsupported run-manifest schema_version")
            revision = manifest.get("revision")
            if not isinstance(revision, str) or not re.fullmatch(r"(?:[0-9a-f]{40}|[0-9a-f]{64})", revision):
                manifest_errors.append("run-manifest revision must be a full commit hash")
            if revision != expected_revision:
                manifest_errors.append("run-manifest revision does not match expected revision")
        for detail in manifest_errors:
            fail("invalid-run-manifest", detail)
    accepted_exit = policy.get("accepted_exit_codes") or {}
    scope_min = policy.get("scope_minimums") or {}
    pins_all = [p for p in policy.get("report_pins") or [] if p.get("reports_dir") == reports_key]

    ctx = Ctx(policy)
    scanner_results, observed = {}, {}
    for sc in scanners.get("scanners") or []:
        sid = sc["id"]
        obs = Obs(sc, ctx)
        mentry = (manifest or {}).get("scanners", {}).get(sid) if manifest else None
        if policy.get("require_run_manifest") and mentry is None:
            obs.issue("missing-run-manifest", "no run-manifest.json entry (exit code/status not recorded)")
        if policy.get("require_run_manifest") and mentry is not None:
            for detail in _manifest_entry_errors(mentry, reports_dir, (manifest or {}).get("revision"),
                                                accepted_exit.get(sid, accepted_exit.get("default", [0]))):
                obs.issue("invalid-run-manifest", detail)
        pins = next((p["paths"] for p in pins_all if p.get("scanner") == sid), None)
        if policy.get("require_run_manifest") and pins and isinstance(mentry, dict) \
                and pins != mentry.get("reports"):
            obs.issue("invalid-run-manifest", "policy report pins differ from required hashed reports")
        if policy.get("require_run_manifest") and (manifest_errors or any(x["fatal"] for x in obs.issues)):
            paths, d_issues, source = [], [], "invalid required run-manifest"
            if not isinstance(mentry, dict):
                mentry = None
        else:
            paths, d_issues, source = discover(reports_dir, sc, pins, mentry)
        not_configured = str(sc.get("access_status", "")).split(" ")[0] in ("not-configured", "disabled")
        for kind, detail in d_issues:
            if kind == "missing-report" and not_configured and not paths:
                kind = "not-scanned"
                detail = f"{detail}; scanners.json access_status={sc.get('access_status')}: {sc.get('reason', '')}".strip()
            obs.issue(kind, detail)
        if mentry:
            st = mentry.get("status")
            if st is not None and str(st).lower() not in SUCCESS_STATUSES:
                obs.issue("scanner-error", f"run-manifest status={st!r}", f"status-{st}")
            ok_codes = accepted_exit.get(sid, accepted_exit.get("default", [0]))
            if "exit_code" in mentry and mentry["exit_code"] not in ok_codes:
                obs.issue("scanner-error", f"run-manifest exit_code={mentry['exit_code']} not in accepted {ok_codes}",
                          f"exit-code-{mentry['exit_code']}")
            for k in ("tool_version", "db_version", "revision"):
                if mentry.get(k) is not None:
                    obs.provenance[{"tool_version": "version"}.get(k, k)] = mentry[k]
            if manifest.get("revision") and "revision" not in obs.provenance:
                obs.provenance["revision"] = manifest["revision"]
            if mentry.get("files_scanned") is not None:
                obs.provenance["files_scanned"] = mentry["files_scanned"]
                if mentry["files_scanned"] == 0:
                    obs.issue("empty-scope", "run-manifest files_scanned=0")
                else:
                    obs.scope_evidence = obs.scope_evidence or f"{mentry['files_scanned']} files (run-manifest)"
        normalizer = choose_normalizer(sc)
        if paths and not any(i["fatal"] for i in obs.issues):
            if normalizer is None:
                obs.issue("invalid-report", f"no normalizer for report_format {sc.get('report_format')!r}")
            else:
                try:
                    normalizer(paths, sc, obs)
                except ReportIssue as e:
                    obs.issue(e.kind, e.detail, e.marker)
                except (KeyError, IndexError, TypeError, ValueError, AttributeError) as e:
                    obs.issue("invalid-report", f"report structure not as expected for {sc.get('report_format')!r}: "
                              f"{e.__class__.__name__}: {e}")
        # zero results need scope evidence
        if paths and not any(i["fatal"] for i in obs.issues) and not obs.items and not obs.buckets \
                and not obs.zero_ok and not obs.scope_evidence:
            obs.issue("empty-scope", "zero results and no evidence of a non-empty scan scope "
                      "(record files_scanned in run-manifest.json)")
        mins = (scope_min.get(sid) or {}).get("files")
        fs = obs.provenance.get("files_scanned")
        if mins and isinstance(fs, int) and 0 < fs < mins:
            obs.issue("scope-mismatch", f"scanned {fs} files < policy scope_minimums {mins}")

        # provenance comparison
        exp_ver = sc.get("tool_version")
        got_ver = obs.provenance.get("version")
        if exp_ver and got_ver and str(got_ver) not in _version_tokens(exp_ver) and str(got_ver) != str(exp_ver):
            toks = _version_tokens(exp_ver)
            prov_changes.append({"scanner": sid, "field": "tool_version",
                                 "expected": toks[0] if toks else exp_ver, "observed": str(got_ver)})
        if sc.get("db_version") and obs.provenance.get("db_version") and sc["db_version"] != obs.provenance["db_version"]:
            prov_changes.append({"scanner": sid, "field": "db_version", "expected": sc["db_version"],
                                 "observed": obs.provenance["db_version"]})

        # issues -> failures or accepted gaps
        status = "ok"
        for iss in obs.issues:
            gap = None
            for g in gaps:
                if g["scanner"] == sid and g["condition"] == iss["kind"] \
                        and (not g.get("marker") or g["marker"] == iss.get("marker")) \
                        and (not g.get("reports_dirs") or reports_key in g["reports_dirs"]):
                    gap = g
                    break
            if gap:
                gap["_used"] = True
            if gap and not release and today <= gap["_expires"]:
                accepted.append({"scanner": sid, "kind": iss["kind"], "label": GAP_LABEL.get(iss["kind"], iss["kind"]),
                                 "detail": iss["detail"], "owner": gap["owner"], "expires": gap["expires"],
                                 "reason": gap["reason"], "coverage_gap_ref": gap.get("coverage_gap_ref"),
                                 "status": gap.get("status")})
                if status == "ok":
                    status = "accepted-gap"
                continue
            detail = iss["detail"]
            if gap and release:
                detail += f" [accepted gap ignored in --release mode; owner {gap['owner']}]"
            elif gap:
                detail += f" [accepted gap expired {gap['expires']}; owner {gap['owner']}]"
            fail(iss["kind"], detail, sid)
            status = "fail"

        fatal = any(i["fatal"] for i in obs.issues)
        partial = bool(obs.issues)
        compared = bool(paths) and not fatal
        scanner_results[sid] = {
            "status": status, "report_source": source,
            "reports": [p.relative_to(reports_dir).as_posix() if p.is_relative_to(reports_dir) else str(p) for p in paths],
            "issues": obs.issues, "provenance": obs.provenance, "scope_evidence": obs.scope_evidence,
            "compared": compared,
            "observed": {"itemized": len(obs.items), "buckets": len(obs.buckets),
                         "bucket_total": sum(obs.buckets.values())},
            "valid_empty": compared and not obs.items and not obs.buckets and not obs.issues,
            "fingerprints": sorted(obs.items) + sorted(obs.buckets),
            "notes": obs.notes[:50],
        }
        if not compared:
            continue
        observed[sid] = obs

        # ---- comparison (identity, never net counts) ----
        for f, rec in sorted(obs.items.items()):
            inv = inv_items.get(f)
            if inv is None:
                loc = rec["location"]
                fail("new-finding", f"{rec['rule_id']} at {loc.get('path')}"
                     + (f":{loc['line']}" if loc.get("line") else "") + " not in inventory", sid, f)
                continue
            if release and (dispositions.get(inv.get("disposition")) or {}).get("release_blocking"):
                fail("release-blocker", f"present with disposition {inv.get('disposition')!r}", sid, f)
            if "prod" in obs.variants.get(f, ()) and inv.get("category") == "build-only":
                fail("exposure-escalation", "inventoried as build-only but reported by the production (--omit=dev) audit", sid, f)
        for f, n in sorted(obs.buckets.items()):
            inv = inv_buckets.get(f)
            if inv is None:
                fail("bucket-new", f"new bucket with {n} occurrence(s) not in inventory", sid, f)
                continue
            base = inv.get("count", 0)
            if n > base:
                fail("bucket-increase", f"count {base} -> {n} (+{n - base})", sid, f)
            elif n < base:
                info.append({"kind": "bucket-decrease", "scanner": sid, "fingerprint": f,
                             "detail": f"count {base} -> {n} (baseline edit needed to lock in progress)"})
            if release and (dispositions.get(inv.get("disposition")) or {}).get("release_blocking"):
                fail("release-blocker", f"bucket present with disposition {inv.get('disposition')!r}", sid, f)
        label = "unobserved-partial-scan" if partial else "resolved"
        for f, r in inv_items.items():
            if r.get("scanner_id") == sid and f not in obs.items:
                # T1 'codeql-alert:<n>' identities have no instance in the analysed scope (e.g. ENABLE_GO2RTC=OFF);
                # a report can never contain them, so absence is not evidence of a fix.
                kind = "unobservable-identity" if f.startswith("codeql-alert:") else label
                info.append({"kind": kind, "scanner": sid, "fingerprint": f,
                             "detail": f"{r.get('disposition')} record not reported by this scan"})
        for f, r in inv_buckets.items():
            if r.get("scanner_id") == sid and f not in obs.buckets:
                info.append({"kind": label, "scanner": sid, "fingerprint": f,
                             "detail": f"bucket (count {r.get('count')}) not reported by this scan"})

    for old_new in sorted(ctx.renamed.items()):
        info.append({"kind": "renamed", "scanner": None, "fingerprint": None,
                     "detail": f"path {old_new[0]} identified as {old_new[1]} via policy path_renames"})
    for g in gaps:
        if not g["_used"] and (not g.get("reports_dirs") or reports_key in g["reports_dirs"]):
            info.append({"kind": "unused-gap", "scanner": g["scanner"], "fingerprint": None,
                         "detail": f"accepted gap for condition {g['condition']!r} did not apply; remove it from policy"})

    by_class = collections.Counter(f["class"] for f in failures)
    return {
        "schema_version": 1,
        "status": "fail" if failures else "pass",
        "mode": "release" if release else "check",
        "today": today.isoformat(),
        "reports_dir": reports_key,
        "run_manifest": manifest_path.name if manifest else None,
        "failures": failures,
        "failure_counts": dict(by_class),
        "accepted_gaps": accepted,
        "provenance_changes": prov_changes,
        "info": info,
        "scanners": scanner_results,
        "_observed": observed,
    }


# --------------------------------------------------------------------------
# rendering
# --------------------------------------------------------------------------
def _prov_str(p):
    s = " ".join(str(x) for x in (p.get("tool"), p.get("version")) if x)
    if p.get("extensions"):
        s += " (+" + ", ".join(p["extensions"][:2]) + ")"
    if p.get("db_version"):
        s += f" db {p['db_version']}"
    if p.get("revision"):
        s += f" @{str(p['revision'])[:8]}"
    return s or "version not recorded in report"


def render(result, out):
    w = out.write
    w(f"findings policy check  mode={result['mode']}  reports={result['reports_dir']}  today={result['today']}"
      f"  run-manifest={'yes' if result['run_manifest'] else 'no'}\n\nSCANNERS\n")
    tag = {"ok": "OK  ", "accepted-gap": "GAP ", "fail": "FAIL"}
    for sid, s in result["scanners"].items():
        o = s["observed"]
        w(f"  [{tag[s['status']]}] {sid:<28} items={o['itemized']:<4} buckets={o['buckets']}/{o['bucket_total']:<5} "
          f"{_prov_str(s['provenance'])}\n")
        for r in s["reports"]:
            w(f"           report: {r} ({s['report_source']})\n")
        if s["valid_empty"]:
            w(f"           valid empty result (scope: {s['scope_evidence']})\n")
        for i in s["issues"]:
            w(f"           {i['kind']}: {i['detail']}\n")
    if result["provenance_changes"]:
        w("\nPROVENANCE CHANGED (tool/database version differs from scanners.json; review result drift)\n")
        for c in result["provenance_changes"]:
            w(f"  PROVENANCE CHANGED {c['scanner']}: {c['field']} {c['expected']} -> {c['observed']}\n")
    if result["accepted_gaps"]:
        w(f"\nACCEPTED GAPS ({len(result['accepted_gaps'])}) - known coverage gaps, NOT clean results\n")
        for g in result["accepted_gaps"]:
            w(f"  ACCEPTED GAP {g['scanner']}: {g['label']} - {g['detail']}\n"
              f"      owner {g['owner']}, expires {g['expires']}"
              + (f", ref {g['coverage_gap_ref']}" if g.get("coverage_gap_ref") else "")
              + (f", {g['status']}" if g.get("status") else "") + f"; reason: {g['reason']}\n")
    if result["failures"]:
        w(f"\nFAILURES ({len(result['failures'])})\n")
        for cls in ("coverage", "finding", "exception", "config"):
            fs = [f for f in result["failures"] if f["class"] == cls]
            if not fs:
                continue
            w(f"  [{cls}] {len(fs)}\n")
            for f in fs[:200]:
                w(f"    {f['kind']}: {f['scanner'] or '-'} {f['fingerprint'] or ''} {f['detail']}\n")
            if len(fs) > 200:
                w(f"    ... {len(fs) - 200} more (see --json-out)\n")
    cnt = collections.Counter(i["kind"] for i in result["info"])
    if cnt:
        w("\nINFO (non-failing)\n")
        for k, n in sorted(cnt.items()):
            w(f"  {k}: {n}\n")
        for i in [i for i in result["info"] if i["kind"] in ("renamed", "unused-gap", "bucket-decrease", "resolved")][:40]:
            w(f"    {i['kind']}: {i['scanner'] or '-'} {i['fingerprint'] or ''} {i['detail']}\n")
    fc = result["failure_counts"]
    if result["status"] == "pass":
        w(f"\nRESULT: PASS (0 policy failures; {len(result['accepted_gaps'])} accepted coverage gap(s) listed above are not clean results)\n")
    else:
        only_gaps = set(fc) == {"coverage"}
        w(f"\nRESULT: FAIL - {len(result['failures'])} policy failure(s) "
          f"(coverage {fc.get('coverage', 0)}, findings {fc.get('finding', 0)}, "
          f"expired exceptions {fc.get('exception', 0)}, inventory/policy {fc.get('config', 0)})"
          + ("; all failures are report-coverage gaps, not new findings" if only_gaps else "") + "\n")


# --------------------------------------------------------------------------
# explicit baseline edit
# --------------------------------------------------------------------------
def dump_compact(obj, path):
    """Same layout as the T1 generator: one record per line for list-of-record keys."""
    parts = []
    for k, v in obj.items():
        if isinstance(v, list) and v and isinstance(v[0], dict):
            body = ",\n".join("    " + json.dumps(x, separators=(",", ":")) for x in v)
            parts.append(f"  {json.dumps(k)}: [\n{body}\n  ]")
        else:
            parts.append(f"  {json.dumps(k)}: " + json.dumps(v, indent=2).replace("\n", "\n  "))
    tmp = Path(str(path) + ".tmp")
    tmp.write_text("{\n" + ",\n".join(parts) + "\n}\n", encoding="utf-8")
    os.replace(tmp, path)


def update_baseline(inventory, result, reason, today):
    blocking = [f for f in result["failures"] if f["class"] in ("coverage", "config")]
    if blocking:
        raise UsageError(f"refusing --update-baseline: {len(blocking)} coverage/config failure(s) must be fixed first "
                         "(a missing or crashed report cannot be baselined)")
    findings = inventory.setdefault("findings", [])
    buckets = inventory.setdefault("buckets", [])
    edit = {"date": today, "reason": reason, "reports_dir": result["reports_dir"],
            "added": [], "removed": [], "bucket_changes": []}
    inv_f = {r["fingerprint"]: r for r in findings}
    inv_b = {r["fingerprint"]: r for r in buckets}
    remove = set()
    for sid, obs in result["_observed"].items():
        full = not obs.issues
        for f, rec in sorted(obs.items.items()):
            if f not in inv_f:
                findings.append({"id": f"{sid.upper()}-{fp(f)}", "fingerprint": f, "scanner_id": sid,
                                 "rule_id": rec["rule_id"], "location": rec["location"],
                                 "disposition": "needs-triage", "exception_expiry": None, "first_seen": today,
                                 "evidence": f"added by explicit --update-baseline on {today}: {reason}"})
                edit["added"].append(f)
        for f, n in sorted(obs.buckets.items()):
            meta = obs.bucket_meta[f]
            if f not in inv_b:
                buckets.append({"id": f"{sid.upper()}-BUCKET-{fp(f)}", "fingerprint": f, "scanner_id": sid,
                                "rule_id": meta["rule_id"], "path": meta["path"], "count": n,
                                "disposition": "needs-triage", "exception_expiry": None})
                edit["bucket_changes"].append({"fingerprint": f, "from": 0, "to": n})
            elif inv_b[f].get("count") != n:
                edit["bucket_changes"].append({"fingerprint": f, "from": inv_b[f].get("count"), "to": n})
                inv_b[f]["count"] = n
        if full:
            for f, r in inv_f.items():
                if r.get("scanner_id") == sid and f not in obs.items:
                    remove.add(f)
                    edit["removed"].append(f)
            for f, r in inv_b.items():
                if r.get("scanner_id") == sid and f not in obs.buckets:
                    remove.add(f)
                    edit["bucket_changes"].append({"fingerprint": f, "from": r.get("count"), "to": 0})
    inventory["findings"] = [r for r in findings if r["fingerprint"] not in remove]
    inventory["buckets"] = [r for r in buckets if r["fingerprint"] not in remove]
    edit["summary"] = {"added": len(edit["added"]), "removed": len(edit["removed"]),
                       "bucket_changes": len(edit["bucket_changes"])}
    inventory.setdefault("baseline_edits", []).append(edit)
    return edit


# --------------------------------------------------------------------------
# CLI
# --------------------------------------------------------------------------
def _load(path, what):
    try:
        with open(path, encoding="utf-8") as fh:
            return json.load(fh)
    except (OSError, json.JSONDecodeError) as e:
        raise UsageError(f"cannot read {what} {path}: {e}")


def build_parser():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    mode = ap.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true", help="compare reports with the inventory (never writes)")
    mode.add_argument("--update-baseline", action="store_true",
                      help="explicitly rewrite the inventory from the reports (requires --reason)")
    ap.add_argument("--reports", required=True, help="report directory, e.g. build/eng-artifacts/T33")
    ap.add_argument("--inventory", default="quality/findings.json")
    ap.add_argument("--scanners", default="quality/scanners.json")
    ap.add_argument("--policy", default="quality/findings-policy.json")
    ap.add_argument("--today", help="override today's date (YYYY-MM-DD) for expiry checks")
    ap.add_argument("--release", action="store_true",
                    help="release policy: accepted gaps do not apply; release-blocking dispositions fail")
    ap.add_argument("--json-out", help="write the machine-readable result here")
    ap.add_argument("--expected-revision", help="full immutable revision to compare with required run provenance")
    ap.add_argument("--reason", help="justification recorded with --update-baseline")
    return ap


def main(argv=None):
    ap = build_parser()
    args = ap.parse_args(argv)  # argparse exits 2 on usage errors
    if args.update_baseline and not (args.reason or "").strip():
        ap.error("--update-baseline requires a non-empty --reason")
    try:
        reports = Path(args.reports)
        if not reports.is_dir():
            raise UsageError(f"reports directory {reports} does not exist (no scan results at all)")
        try:
            today = parse_date(args.today, "--today") if args.today else dt.date.today()
        except ValueError as e:
            raise UsageError(str(e))
        inventory = _load(args.inventory, "inventory")
        scanners = _load(args.scanners, "scanners manifest")
        policy = _load(args.policy, "policy")
        try:
            key = Path(os.path.relpath(reports.resolve(), Path.cwd().resolve())).as_posix()
        except ValueError:
            key = reports.resolve().as_posix()
        result = evaluate(reports_dir=reports, inventory=inventory, scanners=scanners, policy=policy,
                          today=today, release=args.release, reports_key=key, expected_revision=args.expected_revision)
        render(result, sys.stdout)
        if args.json_out:
            Path(args.json_out).parent.mkdir(parents=True, exist_ok=True)
            Path(args.json_out).write_text(json.dumps({k: v for k, v in result.items() if not k.startswith("_")},
                                                      indent=1) + "\n", encoding="utf-8")
        if args.update_baseline:
            edit = update_baseline(inventory, result, args.reason.strip(), today.isoformat())
            dump_compact(inventory, args.inventory)
            print(f"\nBASELINE UPDATED explicitly: +{edit['summary']['added']} findings, "
                  f"-{edit['summary']['removed']} findings, {edit['summary']['bucket_changes']} bucket changes "
                  f"written to {args.inventory}; review the diff (expired exceptions are not touched).")
            return EXIT_PASS
        return EXIT_PASS if result["status"] == "pass" else EXIT_FAIL
    except UsageError as e:
        print(f"findings.py: error: {e}", file=sys.stderr)
        print("RESULT: ERROR - invalid input; no policy verdict was produced", file=sys.stdout)
        return EXIT_USAGE
    except Exception:  # never let a checker bug become exit 0
        traceback.print_exc()
        print("RESULT: ERROR - internal checker failure (exit 3); this is not a policy verdict", file=sys.stdout)
        return EXIT_INTERNAL


if __name__ == "__main__":
    sys.exit(main())
