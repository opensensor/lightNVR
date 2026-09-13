#!/usr/bin/env python3
"""Local S3 protocol fixture: validates SigV4 and injects deterministic failures."""
import hashlib
import hmac
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import threading
import uuid
import xml.etree.ElementTree as ET
from urllib.parse import parse_qsl, quote, unquote, urlsplit


class S3Handler(BaseHTTPRequestHandler):
    objects = {}
    uploads = {}
    part_attempts = {}
    failed_deletions = set()
    expired_completions = set()
    expired_parts = set()
    lock = threading.Lock()

    def log_message(self, *_):
        pass

    def respond(self, status, data=b'', headers=None):
        self.send_response(status)
        self.send_header('Content-Length', str(len(data)))
        for key, value in (headers or {}).items():
            self.send_header(key, value)
        self.end_headers()
        if self.command != 'HEAD':
            self.wfile.write(data)

    def authorized(self, body):
        match = re.fullmatch(r'AWS4-HMAC-SHA256 Credential=([^/]+)/([^,]+), SignedHeaders=([^,]+), Signature=([0-9a-f]+)',
                             self.headers.get('Authorization', ''))
        if not match or match[1] != 'fixture-access':
            return False
        scope = match[2]
        date, region, service, terminator = scope.split('/')
        if service != 's3' or terminator != 'aws4_request':
            return False
        payload_hash = self.headers.get('x-amz-content-sha256', hashlib.sha256(body).hexdigest())
        if payload_hash != 'UNSIGNED-PAYLOAD' and payload_hash != hashlib.sha256(body).hexdigest():
            return False
        url = urlsplit(self.path)
        query = '&'.join(f'{quote(k, safe="~")}={quote(v, safe="~")}'
                         for k, v in sorted(parse_qsl(url.query, keep_blank_values=True)))
        canonical_headers = ''.join(f'{key}:{" ".join(self.headers.get(key, "").split())}\n'
                                    for key in match[3].split(';'))
        canonical = '\n'.join([self.command, url.path, query, canonical_headers, match[3], payload_hash])
        string_to_sign = '\n'.join(['AWS4-HMAC-SHA256', self.headers['x-amz-date'], scope,
                                    hashlib.sha256(canonical.encode()).hexdigest()])
        key = b'AWS4fixture-secret'
        for component in [date, region, service, terminator]:
            key = hmac.new(key, component.encode(), hashlib.sha256).digest()
        expected = hmac.new(key, string_to_sign.encode(), hashlib.sha256).hexdigest()
        return hmac.compare_digest(expected, match[4])

    def handle_request(self):
        length = int(self.headers.get('Content-Length', '0'))
        if length > 16 * 1024 * 1024:
            return self.respond(413)
        body = self.rfile.read(length) if length else b''
        url = urlsplit(self.path)
        path = unquote(url.path).strip('/')
        bucket, _, key = path.partition('/')
        anonymous_public = bucket == 'public' and self.command == 'GET' and not self.headers.get('Authorization')
        if not self.authorized(body) and not anonymous_public:
            return self.respond(403)
        if not key:
            if url.query == 'versioning':
                value = b'<Status>Enabled</Status>' if bucket == 'versioned' else b''
                return self.respond(200, b'<VersioningConfiguration>' + value + b'</VersioningConfiguration>')
            if url.query == 'lifecycle':
                if bucket == 'expiry':
                    return self.respond(200, b'<LifecycleConfiguration><Rule><Status>Enabled</Status><Expiration><Days>1</Days></Expiration></Rule></LifecycleConfiguration>')
                if bucket == 'abort-only':
                    return self.respond(200, b'<LifecycleConfiguration><Rule><ID>cleanup</ID><Filter><Prefix/></Filter><Status>Enabled</Status><AbortIncompleteMultipartUpload><DaysAfterInitiation>7</DaysAfterInitiation></AbortIncompleteMultipartUpload></Rule></LifecycleConfiguration>')
                return self.respond(404)
            return self.respond(200)
        identity = (bucket, key)
        query = dict(parse_qsl(url.query, keep_blank_values=True))
        with self.lock:
            if 'uploads' in query and self.command == 'POST':
                upload_id = str(uuid.uuid4())
                self.uploads[upload_id] = {'identity': identity, 'parts': {}}
                return self.respond(200, f'<InitiateMultipartUploadResult><UploadId>{upload_id}</UploadId></InitiateMultipartUploadResult>'.encode())
            if 'uploadId' in query:
                upload_id = query['uploadId']
                upload = self.uploads.get(upload_id)
                if upload is None or upload['identity'] != identity:
                    return self.respond(404)
                if self.command == 'DELETE':
                    del self.uploads[upload_id]
                    return self.respond(204)
                if self.command == 'PUT':
                    part = int(query['partNumber'])
                    if bucket == 'multipart-part-expired' and part == 2 and identity not in self.expired_parts:
                        self.expired_parts.add(identity)
                        del self.uploads[upload_id]
                        return self.respond(404, b'<Error><Code>NoSuchUpload</Code></Error>')
                    attempt = self.part_attempts.get((identity, part), 0) + 1
                    self.part_attempts[identity, part] = attempt
                    # Fail once after a durable first part. Retrying part 1 is
                    # forbidden so a non-resuming implementation cannot pass.
                    if bucket == 'multipart-retry' and ((part == 2 and attempt == 1) or (part == 1 and attempt > 1)):
                        return self.respond(503)
                    etag = '"' + hashlib.md5(body).hexdigest() + '"'
                    upload['parts'][part] = (etag, body)
                    return self.respond(200, headers={'ETag': etag})
                if self.command == 'POST':
                    if bucket in ('multipart-expired', 'multipart-expired-200') and identity not in self.expired_completions:
                        self.expired_completions.add(identity)
                        del self.uploads[upload_id]
                        return self.respond(404 if bucket == 'multipart-expired' else 200,
                                            b'<Error><Code>NoSuchUpload</Code></Error>')
                    manifest = ET.fromstring(body)
                    values = []
                    for part in manifest.findall('Part'):
                        stored = upload['parts'].get(int(part.findtext('PartNumber')))
                        if stored is None or stored[0] != part.findtext('ETag'):
                            return self.respond(200, b'<Error><Code>InvalidPart</Code></Error>')
                        values.append(stored[1])
                    self.objects[identity] = b''.join(values)
                    del self.uploads[upload_id]
                    if bucket == 'multipart-ambiguous':
                        return self.respond(503)
                    return self.respond(200, b'<CompleteMultipartUploadResult/>')
            if self.command == 'PUT':
                self.objects[identity] = body
                return self.respond(200)
            if self.command == 'DELETE':
                if bucket == 'delete-retry' and '/recordings/' in key and identity not in self.failed_deletions:
                    self.failed_deletions.add(identity)
                    return self.respond(503)
                self.objects.pop(identity, None)
                return self.respond(204)
            value = self.objects.get(identity)
        if value is None:
            return self.respond(404)
        if bucket == 'corrupt' and '/recordings/' in key and self.command == 'GET':
            value = b'!' + value[1:]
        requested_range = self.headers.get('Range')
        if requested_range:
            match = re.fullmatch(r'bytes=(\d+)-(\d+)', requested_range)
            if not match:
                return self.respond(416)
            start, end = map(int, match.groups())
            if start > end or end >= len(value):
                return self.respond(416)
            return self.respond(206, value[start:end+1], {'Content-Range': f'bytes {start}-{end}/{len(value)}'})
        return self.respond(200, value)

    do_GET = handle_request
    do_HEAD = handle_request
    do_PUT = handle_request
    do_POST = handle_request
    do_DELETE = handle_request


def main():
    with tempfile.TemporaryDirectory(prefix='lightnvr-s3-fixture-') as directory:
        credential = Path(directory) / 'fixture'
        credential.write_text('{"access_key_id":"fixture-access","secret_access_key":"fixture-secret","session_token":"fixture-session-token"}')
        credential.chmod(0o600)
        server = ThreadingHTTPServer(('127.0.0.1', 0), S3Handler)
        threading.Thread(target=server.serve_forever, daemon=True).start()
        env = dict(os.environ, LIGHTNVR_ARCHIVE_CREDENTIALS_DIR=directory,
                   LIGHTNVR_ARCHIVE_ALLOW_HTTP='1',
                   LIGHTNVR_TEST_S3_ENDPOINT=f'http://127.0.0.1:{server.server_port}')
        try:
            result = subprocess.run(sys.argv[1:], env=env, timeout=120)
        finally:
            server.shutdown()
            server.server_close()
        return result.returncode


if __name__ == '__main__':
    sys.exit(main())
