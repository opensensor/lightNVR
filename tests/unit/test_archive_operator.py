#!/usr/bin/env python3
"""Operator inspection validation and atomic publication without cloud access."""
import importlib.util
import json
import os
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch
import xml.etree.ElementTree as ET

path = Path(__file__).resolve().parents[2] / "deployment/archive/verify_bucket_lifecycle.py"
spec = importlib.util.spec_from_file_location("archive_operator", path)
operator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(operator)


class ProviderError(Exception):
    def __init__(self, code, status):
        self.response = {"Error": {"Code": code}, "ResponseMetadata": {"HTTPStatusCode": status}}


class LifecycleInspectionTests(unittest.TestCase):
    def inspect(self, client):
        with patch.dict("sys.modules", {"botocore.exceptions": SimpleNamespace(ClientError=ProviderError)}):
            return operator.inspect_bucket(client, "https://s3.example", "region", "bucket")

    def test_only_explicit_missing_lifecycle_is_accepted(self):
        client = Mock()
        client.get_bucket_versioning.return_value = {"ResponseMetadata": {}}
        client.get_bucket_lifecycle_configuration.side_effect = ProviderError("NoSuchLifecycleConfiguration", 404)
        proof = self.inspect(client)
        self.assertEqual("bucket", proof["bucket"])
        self.assertEqual("https://s3.example", proof["endpoint"])
        self.assertGreater(proof["checked_at"], 0)
        self.assertEqual(0, len(ET.fromstring(proof["lifecycle_configuration_xml"])))
        for code, status in (("AccessDenied", 403), ("NoSuchBucket", 404), ("InternalError", 500),
                             ("NoSuchLifecycleConfiguration", 403)):
            client.get_bucket_lifecycle_configuration.side_effect = ProviderError(code, status)
            with self.subTest(code=code, status=status), self.assertRaises(ProviderError):
                self.inspect(client)

    def test_versioned_or_suspended_bucket_is_rejected_before_lifecycle(self):
        for status in ("Enabled", "Suspended"):
            client = Mock()
            client.get_bucket_versioning.return_value = {"Status": status}
            with self.subTest(status=status), self.assertRaises(ValueError):
                self.inspect(client)
            client.get_bucket_lifecycle_configuration.assert_not_called()

    def test_success_without_lifecycle_rules_is_not_treated_as_missing(self):
        client = Mock()
        client.get_bucket_versioning.return_value = {}
        client.get_bucket_lifecycle_configuration.return_value = {"ResponseMetadata": {}}
        with self.assertRaises(ValueError):
            self.inspect(client)

    def test_abort_rule_and_absent_configuration(self):
        self.assertEqual("LifecycleConfiguration", ET.fromstring(operator.lifecycle_xml({})).tag)
        rule = {"ID": "cleanup", "Status": "Enabled", "Filter": {"Prefix": "recordings/"},
                "AbortIncompleteMultipartUpload": {"DaysAfterInitiation": 7}}
        xml = ET.fromstring(operator.lifecycle_xml({"Rules": [rule]}))
        self.assertEqual("7", xml.findtext("Rule/AbortIncompleteMultipartUpload/DaysAfterInitiation"))
        self.assertEqual("recordings/", xml.findtext("Rule/Filter/Prefix"))

    def test_expiry_and_transitions_rejected_even_alongside_abort(self):
        for action in ("Expiration", "Transition", "NoncurrentVersionExpiration", "UnknownAction"):
            with self.subTest(action=action), self.assertRaises(ValueError):
                operator.lifecycle_xml({"Rules": [{"AbortIncompleteMultipartUpload": {
                    "DaysAfterInitiation": 7}, action: {"Days": 30}}]})

    def test_malformed_or_unbounded_abort_rejected(self):
        for days in (0, 8, -1, 1.5, "7", True, None):
            with self.subTest(days=days), self.assertRaises(ValueError):
                operator.lifecycle_xml({"Rules": [{"AbortIncompleteMultipartUpload": {
                    "DaysAfterInitiation": days}}]})
        for config in ({"UnknownAction": {}}, {"Rules": {}}, {"Rules": [None]}, {"Rules": [{}]}):
            with self.subTest(config=config), self.assertRaises(ValueError):
                operator.lifecycle_xml(config)

    def test_atomic_private_replacement_and_oversize_preserves_previous(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "instance.lifecycle.json"
            output.write_text("old")
            os.chmod(output, 0o644)
            operator.write_inspection(output, {"checked_at": 123})
            self.assertEqual({"checked_at": 123}, json.loads(output.read_text()))
            self.assertEqual(0o600, output.stat().st_mode & 0o777)
            with self.assertRaises(ValueError):
                operator.write_inspection(output, {"oversize": "x" * 8192})
            self.assertEqual({"checked_at": 123}, json.loads(output.read_text()))
            self.assertEqual([output], list(Path(directory).iterdir()))


if __name__ == "__main__":
    unittest.main()
