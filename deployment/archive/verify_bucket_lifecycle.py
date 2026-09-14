#!/usr/bin/env python3
"""Publish a short-lived lifecycle inspection for bucket-scoped NVR credentials.

Run in the trusted control plane with boto3 and operator AWS credentials. Never
mount those operator credentials into the tenant. Refresh this file every 15–30
minutes and deliver it atomically beside the NVR's ordinary credentials file.
"""

import argparse
import json
import os
from pathlib import Path
import re
import tempfile
import time
from urllib.parse import urlsplit
import xml.etree.ElementTree as ET


def lifecycle_xml(configuration):
    """Reject every lifecycle action except bounded incomplete-upload cleanup."""
    root = ET.Element("LifecycleConfiguration")
    if set(configuration) - {"Rules", "ResponseMetadata"}:
        raise ValueError("Unrecognized lifecycle configuration")
    rules = configuration.get("Rules", [])
    if not isinstance(rules, list):
        raise ValueError("Lifecycle Rules must be a list")
    for rule in rules:
        if not isinstance(rule, dict) or set(rule) - {
            "ID", "Status", "Filter", "Prefix", "AbortIncompleteMultipartUpload"
        }:
            raise ValueError("Object expiry, transitions and unknown lifecycle actions are unsupported")
        abort = rule.get("AbortIncompleteMultipartUpload", {})
        days = abort.get("DaysAfterInitiation") if isinstance(abort, dict) else None
        if (not isinstance(abort, dict) or set(abort) != {"DaysAfterInitiation"}
                or type(days) is not int or not 1 <= days <= 7):
            raise ValueError("Each rule must only abort incomplete uploads after 1–7 days")
        # Filters and status only narrow this abort action; they cannot expire
        # completed objects. Preserve them in the inspection for operator review.
        element = ET.SubElement(root, "Rule")
        for key, value in rule.items():
            append_xml(element, key, value)
    return ET.tostring(root, encoding="unicode")


def append_xml(parent, key, value):
    element = ET.SubElement(parent, key)
    if isinstance(value, dict):
        for child, content in value.items():
            append_xml(element, child, content)
    elif isinstance(value, list):
        for item in value:
            append_xml(element, "Tag" if key == "Tags" else "Item", item)
    else:
        element.text = str(value)


def inspect_bucket(client, endpoint, region, bucket):
    from botocore.exceptions import ClientError

    checked_at = int(time.time())
    versioning = client.get_bucket_versioning(Bucket=bucket)
    if set(versioning) - {"ResponseMetadata"}:
        raise ValueError("Use a never-versioned bucket; suspended versioning is unsupported")
    try:
        configuration = client.get_bucket_lifecycle_configuration(Bucket=bucket)
    except ClientError as error:
        if (error.response.get("Error", {}).get("Code") != "NoSuchLifecycleConfiguration"
                or error.response.get("ResponseMetadata", {}).get("HTTPStatusCode") != 404):
            raise
        configuration = {}
    else:
        if "Rules" not in configuration:
            raise ValueError("Successful lifecycle inspection did not contain Rules")
    return {"endpoint": endpoint, "region": region, "bucket": bucket,
            "checked_at": checked_at, "lifecycle_configuration_xml": lifecycle_xml(configuration)}


def write_inspection(path, inspection):
    encoded = json.dumps(inspection).encode()
    if len(encoded) > 8192:
        raise ValueError("Lifecycle inspection exceeds the NVR's 8192-byte limit")
    path = Path(path)
    fd, temporary = tempfile.mkstemp(prefix=f".{path.name}-", dir=path.parent)
    try:
        with os.fdopen(fd, "wb") as output:
            output.write(encoded)
            output.flush()
            os.fsync(output.fileno())
        os.replace(temporary, path)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def main():
    import boto3
    from botocore.config import Config
    from botocore.exceptions import BotoCoreError, ClientError

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--endpoint", required=True)
    parser.add_argument("--region", required=True)
    parser.add_argument("--bucket", required=True)
    parser.add_argument("--output", required=True, help="e.g. /out/instance.lifecycle.json")
    args = parser.parse_args()
    url = urlsplit(args.endpoint)
    if (url.scheme != "https" or not url.hostname or url.username or url.password
            or url.path not in ("", "/") or url.query or url.fragment):
        parser.error("Endpoint must be an HTTPS origin")
    if not re.fullmatch(r"[a-zA-Z0-9._-]+", args.bucket):
        parser.error("Invalid bucket name")
    client = boto3.client("s3", endpoint_url=args.endpoint, region_name=args.region,
                         config=Config(signature_version="s3v4", s3={"addressing_style": "path"},
                                       connect_timeout=10, read_timeout=30,
                                       retries={"max_attempts": 2}))
    try:
        inspection = inspect_bucket(client, args.endpoint, args.region, args.bucket)
        write_inspection(args.output, inspection)
    except (BotoCoreError, ClientError, ValueError, OSError) as error:
        # Do not print provider bodies, request headers, or credentials. Retain
        # the prior file: it will expire instead of publishing an unsafe update.
        parser.exit(1, f"Bucket inspection failed ({type(error).__name__}); no verification published\n")
    print(f"Lifecycle verified for {args.bucket}; valid for at most one hour")


if __name__ == "__main__":
    main()
