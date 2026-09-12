#!/usr/bin/env python3
"""Verify actual pre-trigger footage using loopback RTSP and simulated ONVIF.

Requires a built lightnvr, go2rtc, ffmpeg and ffprobe. Runs for about 45 seconds.
All processes and files belong to a new /tmp/lightnvr-prebuffer-* directory;
logs, MP4 footage and result.json are retained for inspection.
"""

import argparse
import base64
import json
import os
from pathlib import Path
import signal
import socket
import sqlite3
import subprocess
import tempfile
import threading
import time
import urllib.request
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


ROOT = Path(__file__).resolve().parents[2]


def unused_port():
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


def wait_for(check, timeout=20):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        try:
            value = check()
            if value:
                return value
        except (OSError, ValueError, sqlite3.Error):
            pass
        time.sleep(0.2)
    raise RuntimeError(f"Timed out after {timeout}s: {check}")


class Camera(BaseHTTPRequestHandler):
    motion = threading.Event()

    def log_message(self, *args):
        pass

    def do_POST(self):
        body = self.rfile.read(int(self.headers.get("Content-Length", 0))).decode()
        base = f"http://127.0.0.1:{self.server.server_port}"
        if "GetServices" in body:
            reply = (
                "<GetServicesResponse><Service>"
                "<Namespace>http://www.onvif.org/ver10/events/wsdl</Namespace>"
                f"<XAddr>{base}/events</XAddr></Service></GetServicesResponse>"
            )
        elif "CreatePullPointSubscription" in body:
            reply = (
                "<CreatePullPointSubscriptionResponse><SubscriptionReference>"
                f"<wsa:Address>{base}/pull</wsa:Address></SubscriptionReference>"
                "<CurrentTime>2026-09-12T12:00:00Z</CurrentTime>"
                "<TerminationTime>2026-09-12T12:10:00Z</TerminationTime>"
                "</CreatePullPointSubscriptionResponse>"
            )
        elif "PullMessages" in body:
            time.sleep(0.2)
            reply = "<PullMessagesResponse>"
            if self.motion.is_set():
                reply += (
                    "<NotificationMessage>"
                    "<Topic>tns1:RuleEngine/CellMotionDetector/Motion</Topic>"
                    "<Message><Message><Data>"
                    '<SimpleItem Name="IsMotion" Value="true"/>'
                    "</Data></Message></Message></NotificationMessage>"
                )
            reply += "</PullMessagesResponse>"
        else:
            reply = "<UnsubscribeResponse/>"
        payload = (
            '<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" '
            'xmlns:wsa="http://www.w3.org/2005/08/addressing">'
            f"<s:Body>{reply}</s:Body></s:Envelope>"
        ).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/soap+xml")
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lightnvr", type=Path, default=ROOT / "build/bin/lightnvr")
    parser.add_argument("--go2rtc", type=Path, default=ROOT / "go2rtc/go2rtc")
    args = parser.parse_args()
    for binary in (args.lightnvr, args.go2rtc):
        if not binary.is_file():
            parser.error(f"Missing binary: {binary}")

    run = Path(tempfile.mkdtemp(prefix="lightnvr-prebuffer-"))
    print(f"Artifacts: {run}", flush=True)
    api, rtsp, web = unused_port(), unused_port(), unused_port()
    camera = ThreadingHTTPServer(("127.0.0.1", 0), Camera)
    threading.Thread(target=camera.serve_forever, daemon=True).start()
    processes, logs = [], []

    def start(command, label):
        log = open(run / f"{label}.log", "wb")
        logs.append(log)
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT,
                                   start_new_session=True, cwd=ROOT)
        processes.append(process)

    def request(path, data=None):
        headers = {"Authorization": "Basic " + base64.b64encode(b"admin:admin").decode()}
        if data is not None:
            headers["Content-Type"] = "application/json"
        req = urllib.request.Request(
            f"http://127.0.0.1:{web}{path}", headers=headers,
            data=json.dumps(data).encode() if data is not None else None,
        )
        with urllib.request.urlopen(req, timeout=20) as response:
            return json.load(response)

    def source_seconds():
        progress = run / "progress"
        rows = progress.read_text().splitlines() if progress.exists() else []
        values = [int(row.split("=")[1]) / 1e6 for row in rows
                  if row.startswith("out_time_us=") and "N/A" not in row]
        return values[-1] if values else 0

    def recordings():
        with sqlite3.connect(f"file:{run}/lightnvr.db?mode=ro", uri=True) as db:
            db.row_factory = sqlite3.Row
            return [dict(row) for row in db.execute(
                "SELECT id,file_path,start_time,end_time,is_complete FROM recordings "
                "WHERE stream_name='prebuffer' ORDER BY id"
            )]

    def sample_color(media, offset):
        return list(subprocess.check_output([
            "ffmpeg", "-v", "error", "-ss", str(offset), "-i", media,
            "-frames:v", "1", "-vf", "scale=1:1", "-pix_fmt", "rgb24",
            "-f", "rawvideo", "pipe:1",
        ], timeout=15))

    try:
        (run / "go2rtc.yaml").write_text(
            f'api:\n  listen: "127.0.0.1:{api}"\n'
            f'rtsp:\n  listen: "127.0.0.1:{rtsp}"\n'
            'webrtc:\n  listen: ""\nstreams:\n  fixture: []\n'
        )
        config = (ROOT / "config/lightnvr-test.ini").read_text()
        config = config.replace("/tmp/lightnvr-test", str(run))
        config = config.replace("18080", str(web)).replace("./web/dist", str(ROOT / "web/dist"))
        # The separate loopback go2rtc process supplies RTSP; lightnvr reads it directly.
        config = config.replace("enabled = true\nbinary_path", "enabled = false\nbinary_path")
        (run / "test.ini").write_text(config)
        for subdir in ("recordings/mp4", "models"):
            (run / subdir).mkdir(parents=True, exist_ok=True)
        start([str(args.go2rtc.resolve()), "-c", str(run / "go2rtc.yaml")], "go2rtc")

        def go2rtc_ready():
            with urllib.request.urlopen(f"http://127.0.0.1:{api}/api", timeout=1) as response:
                return response.status == 200

        wait_for(go2rtc_ready)
        start([str(args.lightnvr.resolve()), "-c", str(run / "test.ini")], "application")
        wait_for(lambda: request("/api/system"))
        # Red before source t=12, blue thereafter. A trigger at t>=15 can only
        # produce a red first frame if the recording includes pre-trigger media.
        start([
            "ffmpeg", "-hide_banner", "-loglevel", "warning", "-re", "-f", "lavfi",
            "-i", "color=c=red:s=320x240:r=10", "-vf",
            "drawbox=color=blue:t=fill:enable='gte(t,12)'", "-t", "65", "-an",
            "-c:v", "libx264", "-preset", "ultrafast", "-tune", "zerolatency",
            "-g", "10", "-keyint_min", "10", "-sc_threshold", "0",
            "-progress", str(run / "progress"), "-f", "rtsp", "-rtsp_transport", "tcp",
            f"rtsp://127.0.0.1:{rtsp}/fixture",
        ], "source")
        wait_for(lambda: source_seconds() > 0)
        request("/api/streams", {
            "name": "prebuffer", "url": f"rtsp://127.0.0.1:{rtsp}/fixture",
            "enabled": True, "streaming_enabled": False, "record": False,
            "record_audio": False, "detection_based_recording": True,
            "detection_model": "onvif", "detection_threshold": 50,
            "detection_interval": 1, "pre_detection_buffer": 8,
            "post_detection_buffer": 3, "onvif_port": camera.server_port,
        })
        wait_for(lambda: source_seconds() >= 15, timeout=25)
        assert not recordings(), "Recording started before any simulated motion"
        trigger = time.time()
        Camera.motion.set()
        print(f"Motion start: epoch={trigger}, source_seconds={source_seconds()}", flush=True)
        wait_for(lambda: source_seconds() >= 18, timeout=10)
        Camera.motion.clear()
        print("Waiting for ONVIF hold, detection grace and post-buffer", flush=True)
        rows = wait_for(lambda: [row for row in recordings() if row["is_complete"]], timeout=40)
        assert len(rows) == 1, f"Expected one detection clip: {rows}"
        row = rows[0]
        media = row["file_path"]
        probe = json.loads(subprocess.check_output(
            ["ffprobe", "-v", "error", "-show_format", "-of", "json", media], timeout=15))
        duration = float(probe["format"]["duration"])
        first, last = sample_color(media, 0), sample_color(media, duration - 1)
        result = dict(trigger_epoch=trigger, first_frame_rgb=first, last_frame_rgb=last,
                      duration_seconds=duration, recording=row)
        (run / "result.json").write_text(json.dumps(result, indent=2) + "\n")
        print(json.dumps(result, indent=2), flush=True)
        assert len(first) == 3 and first[0] > 200 and first[2] < 50, "Missing red pre-trigger footage"
        assert len(last) == 3 and last[2] > 200 and last[0] < 50, "Missing blue footage after trigger"
        assert trigger - row["start_time"] >= 4, "Missing pre-trigger metadata interval"
        assert duration >= 15, "Recorded media is too short"
    finally:
        for process in reversed(processes):
            if process.poll() is None:
                os.killpg(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=25)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait()
        camera.shutdown()
        camera.server_close()
        for log in logs:
            log.close()


if __name__ == "__main__":
    main()
