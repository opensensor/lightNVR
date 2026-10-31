# SPDX-License-Identifier: GPL-3.0-or-later
"""Tests for scripts/quality/findings.py (T6: finding identity and exception policy).

Run: python3 -m unittest discover -s tests/quality -p test_findings.py
The fixture inventory in fixtures/findings/base was generated with the T1
generator's fingerprint formulas, independently of findings.py, so a passing
baseline proves the checker reproduces the T1 identity scheme.
"""
import contextlib
import hashlib
import io
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
SCRIPT = REPO / "scripts" / "quality" / "findings.py"
BASE = HERE / "fixtures" / "findings" / "base"
ESLINT_LOG = HERE / "fixtures" / "findings" / "eslint-log" / "web-jsx-a11y.log"

sys.path.insert(0, str(SCRIPT.parent))
import findings  # noqa: E402  (RED: fails until scripts/quality/findings.py exists)

TODAY = "2026-11-01"
CT_REL = "actions/static-analysis-100/clang-tidy-report/clang-tidy-report.txt"
CC_REL = "actions/static-analysis-100/cppcheck-reports/cppcheck-report.xml"
ES_REL = "actions/static-analysis-100/eslint-web.json"
TRIVY_REL = "actions/container-scan-200/trivy-results.sarif"
CODEQL_REL = "github/codeql-abcdef12.sarif"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


class Case(unittest.TestCase):
    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="t6-"))
        shutil.copytree(BASE, self.tmp / "base")
        self.d = self.tmp / "base"
        self.reports = self.d / "reports"

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    # helpers -------------------------------------------------------------
    def load(self, name):
        return json.loads((self.d / name).read_text())

    def save(self, name, obj):
        (self.d / name).write_text(json.dumps(obj, indent=2))

    def rj(self, rel):
        return json.loads((self.reports / rel).read_text())

    def wj(self, rel, obj):
        p = self.reports / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(json.dumps(obj, indent=2))

    def evaluate(self, today=TODAY, release=False):
        return findings.evaluate(
            reports_dir=self.reports,
            inventory=self.load("inventory.json"),
            scanners=self.load("scanners.json"),
            policy=self.load("policy.json"),
            today=today,
            release=release,
            reports_key="fixture/reports",
        )

    def kinds(self, result):
        return sorted({f["kind"] for f in result["failures"]})

    def cli(self, *extra, today=TODAY):
        cmd = [sys.executable, "-I", str(SCRIPT), "--reports", str(self.reports),
               "--inventory", str(self.d / "inventory.json"),
               "--scanners", str(self.d / "scanners.json"),
               "--policy", str(self.d / "policy.json"), "--today", today, *extra]
        return subprocess.run(cmd, capture_output=True, text=True, cwd=str(REPO))


class BaselineTests(Case):
    def test_baseline_passes(self):
        r = self.evaluate()
        self.assertEqual(r["failures"], [], r["failures"])
        self.assertEqual(r["status"], "pass")
        for sid, s in r["scanners"].items():
            self.assertEqual(s["status"], "ok", (sid, s))

    def test_cli_check_exit_zero_on_pass(self):
        p = self.cli("--check")
        self.assertEqual(p.returncode, 0, p.stdout + p.stderr)
        self.assertIn("RESULT: PASS", p.stdout)

    def test_valid_empty_report_passes(self):
        r = self.evaluate()
        trivy = r["scanners"]["trivy-container"]
        self.assertEqual(trivy["status"], "ok")
        self.assertEqual(trivy["observed"]["itemized"], 0)
        self.assertTrue(trivy["valid_empty"])


class NewFindingTests(Case):
    def test_new_finding_fails_even_when_another_disappears(self):
        s = self.rj(CODEQL_REL)
        s["runs"][0]["results"][0]["ruleId"] = "cpp/uncontrolled-allocation-size"
        s["runs"][0]["results"][0]["partialFingerprints"]["primaryLocationLineHash"] = "9999888877776666:1"
        self.wj(CODEQL_REL, s)
        r = self.evaluate()
        self.assertEqual(r["status"], "fail")
        new = [f for f in r["failures"] if f["kind"] == "new-finding"]
        self.assertEqual(len(new), 1)
        self.assertIn("cpp/uncontrolled-allocation-size", new[0]["fingerprint"])
        resolved = [i for i in r["info"] if i["kind"] == "resolved"]
        self.assertTrue(any("cpp/sql-injection" in i["fingerprint"] for i in resolved))
        # result counts are equal; the checker must not net them out
        self.assertEqual(r["scanners"]["codeql-cpp"]["observed"]["itemized"], 2)

    def test_new_clang_tidy_message_fails(self):
        p = self.reports / CT_REL
        p.write_text(p.read_text() + "/__w/lightNVR/lightNVR/src/core/c.c:44:1: warning: "
                     "Call to function 'strcpy' is insecure [clang-analyzer-security.insecureAPI.strcpy]\n")
        r = self.evaluate()
        self.assertIn("new-finding", self.kinds(r))

    def test_bucket_increase_fails_even_if_other_bucket_decreases(self):
        es = self.rj(ES_REL)
        msgs = es[0]["messages"]
        msgs.append(dict(msgs[0], line=20))  # click-events +1
        es[0]["messages"] = [m for m in msgs if m["ruleId"] != "jsx-a11y/no-autofocus"]  # autofocus -1
        self.wj(ES_REL, es)
        r = self.evaluate()
        self.assertEqual(r["status"], "fail")
        inc = [f for f in r["failures"] if f["kind"] == "bucket-increase"]
        self.assertEqual(len(inc), 1)
        self.assertIn("click-events-have-key-events", inc[0]["fingerprint"])
        self.assertIn("2 -> 3", inc[0]["detail"])
        self.assertTrue(any(i["kind"] in ("bucket-decrease", "resolved") for i in r["info"]))

    def test_new_bucket_fails(self):
        es = self.rj(ES_REL)
        es[1]["messages"].append({"ruleId": "jsx-a11y/alt-text", "severity": 1, "message": "m", "line": 1, "column": 1})
        self.wj(ES_REL, es)
        r = self.evaluate()
        self.assertIn("bucket-new", self.kinds(r))


class IdentityTests(Case):
    def test_line_shift_keeps_fingerprints(self):
        before = self.evaluate()
        import re
        ct = self.reports / CT_REL
        ct.write_text(re.sub(r"(\.c):(\d+):", lambda m: f"{m[1]}:{int(m[2]) + 7}:", ct.read_text()))
        cc = self.reports / CC_REL
        cc.write_text(re.sub(r'line="(\d+)"', lambda m: f'line="{int(m[1]) + 7}"', cc.read_text()))
        s = self.rj(CODEQL_REL)
        for res in s["runs"][0]["results"]:
            res["locations"][0]["physicalLocation"]["region"]["startLine"] += 7
        self.wj(CODEQL_REL, s)
        after = self.evaluate()
        self.assertEqual(after["failures"], [])
        for sid in ("clang-tidy", "cppcheck", "codeql-cpp"):
            self.assertEqual(before["scanners"][sid]["fingerprints"], after["scanners"][sid]["fingerprints"], sid)

    def test_line_tool_fingerprint_matches_t1_scheme(self):
        got = findings.line_tool_fingerprint("clang-tidy", "bugprone-x", "src/a.c", "value 42 is 7", 1)
        h = hashlib.sha256("bugprone-x|src/a.c|value N is N|1".encode()).hexdigest()[:16]
        self.assertEqual(got, f"clang-tidy:bugprone-x:src/a.c:{h}")

    def test_rename_requires_explicit_policy_map(self):
        ct = self.reports / CT_REL
        ct.write_text(ct.read_text().replace("src/core/b.c", "src/core/b_renamed.c"))
        r = self.evaluate()
        self.assertIn("new-finding", self.kinds(r))
        pol = self.load("policy.json")
        pol["path_renames"] = [{"from": "src/core/b.c", "to": "src/core/b_renamed.c",
                                "reason": "test rename", "approved_by": "test"}]
        self.save("policy.json", pol)
        r = self.evaluate()
        self.assertEqual(r["failures"], [], r["failures"])
        self.assertTrue(any(i["kind"] == "renamed" for i in r["info"]))

    def test_directory_rename_prefix(self):
        es = self.rj(ES_REL)
        es[0]["filePath"] = es[0]["filePath"].replace("web/js/x.jsx", "web/src/x.jsx")
        self.wj(ES_REL, es)
        pol = self.load("policy.json")
        pol["path_renames"] = [{"from": "web/js/", "to": "web/src/", "reason": "dir move", "approved_by": "test"}]
        self.save("policy.json", pol)
        self.assertEqual(self.evaluate()["failures"], [])


class CoverageTests(Case):
    def test_missing_report_fails(self):
        (self.reports / TRIVY_REL).unlink()
        r = self.evaluate()
        self.assertEqual(r["status"], "fail")
        self.assertIn("missing-report", self.kinds(r))
        self.assertEqual(r["scanners"]["trivy-container"]["status"], "fail")

    def test_missing_one_variant_of_brace_convention_fails(self):
        (self.reports / "npm/web-audit-prod.json").unlink()
        r = self.evaluate()
        self.assertIn("missing-report", self.kinds(r))

    def test_ambiguous_report_fails_unless_pinned(self):
        shutil.copy(self.reports / CODEQL_REL, self.reports / "github/codeql-12345678.sarif")
        r = self.evaluate()
        self.assertIn("ambiguous-report", self.kinds(r))
        pol = self.load("policy.json")
        pol["report_pins"] = [{"reports_dir": "fixture/reports", "scanner": "codeql-cpp",
                               "paths": [CODEQL_REL], "reason": "test"}]
        self.save("policy.json", pol)
        self.assertEqual(self.evaluate()["failures"], [])

    def test_crashed_clang_tidy_fails(self):
        ct = self.reports / CT_REL
        lines = ct.read_text().splitlines()
        cut = [l for l in lines if not l.startswith("[2/2]")][:5]
        ct.write_text("\n".join(cut + ["Stack dump:", "Segmentation fault (core dumped)"]) + "\n")
        r = self.evaluate()
        self.assertIn("scanner-error", self.kinds(r))

    def test_truncated_clang_tidy_fails(self):
        ct = self.reports / CT_REL
        ct.write_text("".join(l for l in ct.read_text().splitlines(True) if "[2/2]" not in l))
        r = self.evaluate()
        self.assertIn("truncated-report", self.kinds(r))

    def test_clang_diagnostic_error_is_scanner_error(self):
        ct = self.reports / CT_REL
        ct.write_text(ct.read_text() + "/__w/lightNVR/lightNVR/src/core/c.c:1:10: error: 'foo.h' file not found [clang-diagnostic-error]\n")
        self.assertIn("scanner-error", self.kinds(self.evaluate()))

    def test_empty_clang_tidy_report_is_empty_scope(self):
        (self.reports / CT_REL).write_text("")
        r = self.evaluate()
        self.assertIn("empty-scope", self.kinds(r))

    def test_trivy_without_target_is_empty_scope(self):
        t = self.rj(TRIVY_REL)
        t["runs"][0]["properties"] = {}
        self.wj(TRIVY_REL, t)
        self.assertIn("empty-scope", self.kinds(self.evaluate()))

    def test_eslint_json_with_no_files_is_empty_scope(self):
        self.wj(ES_REL, [])
        self.assertIn("empty-scope", self.kinds(self.evaluate()))

    def test_eslint_fatal_parse_error_is_scanner_error(self):
        es = self.rj(ES_REL)
        es[1]["messages"].append({"ruleId": None, "fatal": True, "severity": 2, "message": "Parsing error"})
        self.wj(ES_REL, es)
        self.assertIn("scanner-error", self.kinds(self.evaluate()))

    def test_npm_zero_dependencies_is_empty_scope(self):
        a = self.rj("npm/web-audit-prod.json")
        a["metadata"]["dependencies"]["total"] = 0
        self.wj("npm/web-audit-prod.json", a)
        self.assertIn("empty-scope", self.kinds(self.evaluate()))

    def test_empty_cppcheck_is_empty_scope(self):
        (self.reports / CC_REL).write_text('<?xml version="1.0"?>\n<results version="2"><cppcheck version="2.13.0"/><errors></errors></results>\n')
        self.assertIn("empty-scope", self.kinds(self.evaluate()))

    def test_cppcheck_critical_errors_marker_is_scanner_error(self):
        cc = self.reports / CC_REL
        cc.write_text(cc.read_text().replace("Active checkers: 106/592", "Active checkers: There was critical errors"))
        self.assertIn("scanner-error", self.kinds(self.evaluate()))

    def test_unparseable_report_is_invalid_not_crash(self):
        (self.reports / CODEQL_REL).write_text("{not json")
        r = self.evaluate()
        self.assertIn("invalid-report", self.kinds(r))

    def test_sarif_execution_failure_is_scanner_error(self):
        s = self.rj(CODEQL_REL)
        s["runs"][0]["invocations"] = [{"executionSuccessful": False,
                                        "toolExecutionNotifications": [{"level": "error", "message": {"text": "extractor crashed"}}]}]
        self.wj(CODEQL_REL, s)
        self.assertIn("scanner-error", self.kinds(self.evaluate()))

    def test_sarif_wrong_category_is_scope_mismatch(self):
        s = self.rj(CODEQL_REL)
        s["runs"][0]["automationDetails"]["id"] = "/language:javascript-typescript/"
        self.wj(CODEQL_REL, s)
        self.assertIn("scope-mismatch", self.kinds(self.evaluate()))

    def test_api_error_body_is_not_an_empty_result(self):
        self.wj("github/dependabot-open.json", {"message": "Dependabot alerts are disabled for this repository.", "status": "403"})
        r = self.evaluate()
        self.assertIn("scanner-error", self.kinds(r))

    def test_unconfigured_scanner_is_not_scanned(self):
        sc = self.load("scanners.json")
        sc["scanners"].append({"id": "secret-scanning", "report_format": "json", "access_status": "disabled",
                               "report_path_convention": "build/eng-artifacts/<task>/github/secret-scanning-alerts.json",
                               "fingerprint_scheme": "secret"})
        self.save("scanners.json", sc)
        r = self.evaluate()
        self.assertIn("not-scanned", self.kinds(r))
        self.assertEqual(r["status"], "fail")


class ExitStatusTests(Case):
    def manifest(self, entries):
        self.wj("run-manifest.json", {"schema_version": 1, "revision": "abcdef12", "scanners": entries})

    def test_npm_error_key_fails_even_if_exit_zero(self):
        self.wj("npm/web-audit-all.json", {"error": {"code": "ENOLOCK", "summary": "lockfile missing"}})
        self.manifest({"npm-audit-web": {"exit_code": 0, "status": "success"}})
        r = self.evaluate()
        self.assertIn("scanner-error", self.kinds(r))
        p = self.cli("--check")
        self.assertEqual(p.returncode, 1, p.stdout)

    def test_npm_exit_one_with_findings_is_not_error(self):
        self.manifest({"npm-audit-web": {"exit_code": 1, "status": "success"}})
        self.assertEqual(self.evaluate()["failures"], [])

    def test_recorded_failure_status_fails_despite_valid_report(self):
        self.manifest({"trivy-container": {"exit_code": 0, "status": "failure"}})
        r = self.evaluate()
        self.assertIn("scanner-error", self.kinds(r))

    def test_recorded_nonzero_exit_fails(self):
        self.manifest({"clang-tidy": {"exit_code": 139, "status": "success"}})
        self.assertIn("scanner-error", self.kinds(self.evaluate()))

    def test_manifest_files_scanned_zero_is_empty_scope(self):
        self.manifest({"codeql-cpp": {"exit_code": 0, "status": "success", "files_scanned": 0}})
        self.assertIn("empty-scope", self.kinds(self.evaluate()))

    def test_required_manifest_missing_fails(self):
        pol = self.load("policy.json")
        pol["require_run_manifest"] = True
        self.save("policy.json", pol)
        self.assertIn("missing-run-manifest", self.kinds(self.evaluate()))

    def test_internal_error_never_exits_zero(self):
        orig = findings.evaluate
        try:
            def boom(**kw):
                raise RuntimeError("synthetic checker bug")
            findings.evaluate = boom
            err = io.StringIO()
            with contextlib.redirect_stderr(err), contextlib.redirect_stdout(io.StringIO()):
                rc = findings.main(["--check", "--reports", str(self.reports),
                                    "--inventory", str(self.d / "inventory.json"),
                                    "--scanners", str(self.d / "scanners.json"),
                                    "--policy", str(self.d / "policy.json"), "--today", TODAY])
        finally:
            findings.evaluate = orig
        self.assertEqual(rc, 3)
        self.assertIn("synthetic checker bug", err.getvalue())

    def test_missing_reports_dir_is_nonzero(self):
        p = subprocess.run([sys.executable, "-I", str(SCRIPT), "--check", "--reports", str(self.tmp / "nope"),
                            "--inventory", str(self.d / "inventory.json"), "--scanners", str(self.d / "scanners.json"),
                            "--policy", str(self.d / "policy.json"), "--today", TODAY], capture_output=True, text=True)
        self.assertNotEqual(p.returncode, 0)


class ExceptionTests(Case):
    def test_unexpired_exception_passes(self):
        self.assertEqual(self.evaluate(today="2027-01-31")["failures"], [])  # expiry day is still valid

    def test_expired_exception_fails(self):
        r = self.evaluate(today="2027-02-01")
        exp = [f for f in r["failures"] if f["kind"] == "expired-exception"]
        self.assertEqual(len(exp), 1)
        self.assertIn("cpp/path-injection", exp[0]["fingerprint"])
        p = self.cli("--check", today="2027-02-01")
        self.assertEqual(p.returncode, 1)

    def test_exception_without_expiry_is_inventory_error(self):
        inv = self.load("inventory.json")
        inv["buckets"][0]["exception_expiry"] = None
        self.save("inventory.json", inv)
        self.assertIn("inventory-error", self.kinds(self.evaluate()))

    def test_unknown_disposition_is_inventory_error(self):
        inv = self.load("inventory.json")
        inv["findings"][0]["disposition"] = "ignore-forever"
        self.save("inventory.json", inv)
        self.assertIn("inventory-error", self.kinds(self.evaluate()))

    def test_duplicate_fingerprint_is_inventory_error(self):
        inv = self.load("inventory.json")
        inv["findings"].append(dict(inv["findings"][0], id="DUP"))
        self.save("inventory.json", inv)
        self.assertIn("inventory-error", self.kinds(self.evaluate()))

    def test_release_mode_blocks_untriaged(self):
        r = self.evaluate(release=True)
        self.assertIn("release-blocker", self.kinds(r))


class GapPolicyTests(Case):
    def add_secret_scanner(self):
        sc = self.load("scanners.json")
        sc["scanners"].append({"id": "secret-scanning", "report_format": "json", "access_status": "disabled",
                               "report_path_convention": "build/eng-artifacts/<task>/github/secret-scanning-alerts.json",
                               "fingerprint_scheme": "secret"})
        self.save("scanners.json", sc)

    def gap(self, expires):
        pol = self.load("policy.json")
        pol["accepted_gaps"] = [{"scanner": "secret-scanning", "condition": "not-scanned", "owner": "maintainers/security",
                                 "expires": expires, "reason": "disabled upstream", "coverage_gap_ref": "GAP-x"}]
        self.save("policy.json", pol)

    def test_accepted_gap_is_visible_and_passes(self):
        self.add_secret_scanner()
        self.gap("2026-12-31")
        r = self.evaluate()
        self.assertEqual(r["failures"], [])
        self.assertEqual(r["scanners"]["secret-scanning"]["status"], "accepted-gap")
        self.assertEqual(len(r["accepted_gaps"]), 1)
        p = self.cli("--check")
        self.assertEqual(p.returncode, 0)
        self.assertIn("ACCEPTED GAP", p.stdout)
        self.assertIn("NOT SCANNED", p.stdout)

    def test_expired_gap_fails(self):
        self.add_secret_scanner()
        self.gap("2026-10-01")
        r = self.evaluate()
        self.assertIn("not-scanned", self.kinds(r))
        self.assertTrue(any("expired" in f["detail"] for f in r["failures"]))

    def test_release_mode_ignores_accepted_gaps(self):
        self.add_secret_scanner()
        self.gap("2026-12-31")
        self.assertIn("not-scanned", self.kinds(self.evaluate(release=True)))

    def test_gap_without_owner_is_policy_error(self):
        self.add_secret_scanner()
        self.gap("2026-12-31")
        pol = self.load("policy.json")
        del pol["accepted_gaps"][0]["owner"]
        self.save("policy.json", pol)
        self.assertIn("policy-error", self.kinds(self.evaluate()))

    def test_gap_scoped_to_other_reports_dir_does_not_apply(self):
        self.add_secret_scanner()
        self.gap("2026-12-31")
        pol = self.load("policy.json")
        pol["accepted_gaps"][0]["reports_dirs"] = ["build/eng-artifacts/T1"]
        self.save("policy.json", pol)
        self.assertIn("not-scanned", self.kinds(self.evaluate()))


class ProvenanceTests(Case):
    def test_tool_version_change_is_surfaced(self):
        s = self.rj(CODEQL_REL)
        s["runs"][0]["tool"]["driver"]["semanticVersion"] = "2.28.0"
        self.wj(CODEQL_REL, s)
        r = self.evaluate()
        self.assertEqual(r["failures"], [])
        ch = [c for c in r["provenance_changes"] if c["scanner"] == "codeql-cpp"]
        self.assertEqual(len(ch), 1)
        self.assertEqual((ch[0]["expected"], ch[0]["observed"]), ("2.27.3", "2.28.0"))
        p = self.cli("--check")
        self.assertIn("PROVENANCE CHANGED", p.stdout)
        self.assertIn("2.27.3 -> 2.28.0", p.stdout)

    def test_db_version_from_manifest_is_reported(self):
        self.wj("run-manifest.json", {"schema_version": 1, "scanners": {
            "trivy-container": {"exit_code": 0, "status": "success", "db_version": "2026-10-31T00:00:00Z"}}})
        r = self.evaluate()
        self.assertEqual(r["scanners"]["trivy-container"]["provenance"]["db_version"], "2026-10-31T00:00:00Z")


class FormatTests(Case):
    def test_eslint_stylish_log_matches_json_buckets(self):
        (self.reports / ES_REL).unlink()
        shutil.copy(ESLINT_LOG, self.reports / "actions/static-analysis-100/eslint-web.json.log")
        sc = self.load("scanners.json")
        for s in sc["scanners"]:
            if s["id"] == "eslint-web":
                s["report_path_convention"] = "build/eng-artifacts/<task>/actions/static-analysis-<run_id>/eslint-web.json.log"
                s["report_format"] = "eslint stylish text in job log only (no artifact)"
        self.save("scanners.json", sc)
        self.assertEqual(self.evaluate()["failures"], [])

    def test_eslint_log_summary_mismatch_is_invalid(self):
        text = ESLINT_LOG.read_text().replace("3 problems (0 errors, 3 warnings)", "4 problems (0 errors, 4 warnings)")
        (self.reports / ES_REL).unlink()
        (self.reports / "actions/static-analysis-100/eslint-web.json.log").write_text(text)
        sc = self.load("scanners.json")
        for s in sc["scanners"]:
            if s["id"] == "eslint-web":
                s["report_path_convention"] = "build/eng-artifacts/<task>/actions/static-analysis-<run_id>/eslint-web.json.log"
                s["report_format"] = "eslint stylish text in job log only (no artifact)"
        self.save("scanners.json", sc)
        self.assertIn("invalid-report", self.kinds(self.evaluate()))

    def test_npm_prod_exposure_escalation_fails(self):
        self.wj("npm/web-audit-prod.json", self.rj("npm/web-audit-all.json"))
        self.assertIn("exposure-escalation", self.kinds(self.evaluate()))


class BaselineEditTests(Case):
    def add_new_codeql(self):
        s = self.rj(CODEQL_REL)
        new = json.loads(json.dumps(s["runs"][0]["results"][0]))
        new["ruleId"] = "cpp/toctou-race-condition"
        new["partialFingerprints"]["primaryLocationLineHash"] = "1234123412341234:1"
        s["runs"][0]["results"].append(new)
        self.wj(CODEQL_REL, s)

    def test_check_never_modifies_inventory(self):
        self.add_new_codeql()
        inv = self.d / "inventory.json"
        before = sha(inv)
        p = self.cli("--check")
        self.assertEqual(p.returncode, 1, p.stdout)
        self.assertEqual(sha(inv), before)
        self.assertEqual(self.cli("--check").returncode, 1)  # still failing: nothing auto-baselined

    def test_update_baseline_requires_reason(self):
        self.add_new_codeql()
        p = self.cli("--update-baseline")
        self.assertNotEqual(p.returncode, 0)

    def test_update_baseline_is_explicit_and_recorded(self):
        self.add_new_codeql()
        p = self.cli("--update-baseline", "--reason", "accept new toctou for triage (test)")
        self.assertEqual(p.returncode, 0, p.stdout + p.stderr)
        inv = self.load("inventory.json")
        rec = [f for f in inv["findings"] if "toctou" in f["fingerprint"]]
        self.assertEqual(len(rec), 1)
        self.assertEqual(rec[0]["disposition"], "needs-triage")
        self.assertEqual(inv["baseline_edits"][-1]["reason"], "accept new toctou for triage (test)")
        self.assertEqual(self.cli("--check").returncode, 0)

    def test_update_baseline_refuses_on_coverage_failure(self):
        (self.reports / TRIVY_REL).unlink()
        inv = self.d / "inventory.json"
        before = sha(inv)
        p = self.cli("--update-baseline", "--reason", "x")
        self.assertNotEqual(p.returncode, 0)
        self.assertEqual(sha(inv), before)

    def test_check_and_update_are_mutually_exclusive(self):
        p = self.cli("--check", "--update-baseline", "--reason", "x")
        self.assertEqual(p.returncode, 2)


if __name__ == "__main__":
    unittest.main()
