"""Host-process components for the INT_01 X1 lightNVR <-> YA-WAMF E2E suite.

Every component is owned by the suite: a real Mosquitto broker, the lightNVR
binary under test, the YA-WAMF backend (branch under test, pinned classifier,
see ``yawamf_launcher.py``), an HTTP MPEG-TS feeder source and a loopback HTTP
object detector. Nothing here talks to a network service outside 127.0.0.1.

Only the standard library plus Pillow/numpy is used, so the suite runs from
YA-WAMF's own venv (``backend/.venv/bin/python``).
"""
from __future__ import annotations

import base64
import hashlib
import io
import json
import os
import signal
import socket
import sqlite3
import subprocess
import threading
import time
import urllib.error
import urllib.request
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any, Callable, Optional

import numpy as np
from PIL import Image


def free_port() -> int:
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


def wait_for(predicate: Callable[[], Any], timeout: float, interval: float = 0.5, what: str = "condition") -> Any:
    deadline = time.monotonic() + timeout
    last_error: Optional[BaseException] = None
    while time.monotonic() < deadline:
        try:
            value = predicate()
            if value:
                return value
        except Exception as error:  # noqa: BLE001 - polled until the deadline
            last_error = error
        time.sleep(interval)
    raise TimeoutError(f"timed out after {timeout:.0f}s waiting for {what}" + (f" (last error: {last_error})" if last_error else ""))


class HttpError(Exception):
    def __init__(self, status: int, body: bytes, headers: dict[str, str]):
        super().__init__(f"HTTP {status}: {body[:300]!r}")
        self.status, self.body, self.headers = status, body, headers

    def json(self) -> Any:
        return json.loads(self.body)


def http(method: str, url: str, body: Any = None, headers: Optional[dict[str, str]] = None, timeout: float = 15.0) -> tuple[int, dict[str, str], bytes]:
    data = None
    request_headers = dict(headers or {})
    if body is not None:
        data = json.dumps(body).encode()
        request_headers["Content-Type"] = "application/json"
    request = urllib.request.Request(url, data=data, method=method, headers=request_headers)
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return response.status, dict(response.headers), response.read()
    except urllib.error.HTTPError as error:
        return error.code, dict(error.headers), error.read()


def http_json(method: str, url: str, body: Any = None, headers: Optional[dict[str, str]] = None, expect: tuple[int, ...] = (200, 201), timeout: float = 15.0) -> Any:
    status, response_headers, payload = http(method, url, body, headers, timeout)
    if status not in expect:
        raise HttpError(status, payload, response_headers)
    return json.loads(payload) if payload else None


def basic(username: str, password: str) -> dict[str, str]:
    return {"Authorization": "Basic " + base64.b64encode(f"{username}:{password}".encode()).decode()}


# --------------------------------------------------------------------------- broker


class Mosquitto:
    """A real Mosquitto 2.x broker on a fixed loopback port; stop/start keeps the port."""

    def __init__(self, workdir: Path, port: int, binary: str = "mosquitto"):
        self.workdir, self.port, self.binary = workdir, port, binary
        self.proc: Optional[subprocess.Popen] = None
        # No config file: Ubuntu's AppArmor profile for /usr/sbin/mosquitto only lets it
        # read configs under /etc/mosquitto. "-p" without "-c" is Mosquitto 2.x local-only
        # mode: loopback listener, anonymous clients, no persistence.
        self.log = workdir / "mosquitto.log"

    def version(self) -> str:
        out = subprocess.run([self.binary, "-h"], capture_output=True, text=True).stdout
        return out.splitlines()[0].strip() if out else "unknown"

    def start(self) -> None:
        assert self.proc is None or self.proc.poll() is not None
        self.proc = subprocess.Popen([self.binary, "-p", str(self.port), "-v"], stdout=open(self.log, "ab"), stderr=subprocess.STDOUT)
        wait_for(self._accepts, 10, 0.1, "mosquitto to listen")

    def _accepts(self) -> bool:
        with socket.socket() as sock:
            sock.settimeout(0.5)
            return sock.connect_ex(("127.0.0.1", self.port)) == 0

    def stop(self) -> None:
        if self.proc and self.proc.poll() is None:
            self.proc.terminate()
            self.proc.wait(10)
        self.proc = None

    def publish(self, topic: str, payload: bytes, qos: int = 1) -> None:
        subprocess.run(["mosquitto_pub", "-h", "127.0.0.1", "-p", str(self.port), "-q", str(qos), "-t", topic, "-s"], input=payload, check=True, timeout=10)


class MqttCapture:
    """mosquitto_sub on the observation topic; records (topic, payload) as published by lightNVR."""

    def __init__(self, workdir: Path, port: int, topic: str):
        self.path = workdir / "mqtt-capture.jsonl"
        self.port, self.topic = port, topic
        self.proc: Optional[subprocess.Popen] = None
        self._lock = threading.Lock()
        self.messages: list[tuple[str, bytes]] = []
        self._reader: Optional[threading.Thread] = None

    def start(self) -> None:
        """(Re)subscribe; the messages captured so far are kept."""
        self.stop()
        # -F '%t %x': topic, then the payload as hex (binary-safe, one line per message).
        self.proc = subprocess.Popen(["mosquitto_sub", "-h", "127.0.0.1", "-p", str(self.port), "-q", "1", "-i", "x1-capture", "-t", self.topic, "-F", "%t %x"],
                                     stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        self._reader = threading.Thread(target=self._read, daemon=True)
        self._reader.start()

    def _read(self) -> None:
        assert self.proc and self.proc.stdout
        with open(self.path, "a", encoding="utf-8") as sink:
            for raw in self.proc.stdout:
                line = raw.decode().strip()
                if " " not in line:
                    continue
                topic, hexed = line.split(" ", 1)
                payload = bytes.fromhex(hexed)
                with self._lock:
                    self.messages.append((topic, payload))
                sink.write(json.dumps({"t": time.time(), "topic": topic, "payload": payload.decode("utf-8", "replace")}) + "\n")
                sink.flush()

    def envelopes(self) -> list[dict]:
        with self._lock:
            return [json.loads(payload) for _topic, payload in self.messages]

    def raw(self) -> list[tuple[str, bytes]]:
        with self._lock:
            return list(self.messages)

    def stop(self) -> None:
        if self.proc and self.proc.poll() is None:
            self.proc.terminate()
            self.proc.wait(5)


# --------------------------------------------------------------------------- feeder source


SCENES: dict[str, str] = {
    # 640x360 "garden" with one or two saturated squares as birds. The detector
    # finds them by colour; the pinned classifier names them by colour.
    "red_static": "color=c=0x3c6e3c:s=640x360:r=10[bg];color=c=0xdc1414:s=64x64:r=10[red];[bg][red]overlay=x=200:y=150:shortest=1",
    # The red bird crosses the frame at 240 px/s: one detection interval apart,
    # it is ~240 px away, so a frame other than the inference frame misses it.
    "red_moving": "color=c=0x3c6e3c:s=640x360:r=10[bg];color=c=0xdc1414:s=64x64:r=10[red];[bg][red]overlay=x='16+mod(t*240\\,544)':y=150:shortest=1",
    "two_birds": "color=c=0x3c6e3c:s=640x360:r=10[bg];color=c=0xdc1414:s=64x64:r=10[red];color=c=0x1414dc:s=64x64:r=10[blue];[bg][red]overlay=x=120:y=140[a];[a][blue]overlay=x=430:y=170:shortest=1",
}


def render_scene(workdir: Path, name: str, seconds: int = 300) -> Path:
    out = workdir / f"{name}.ts"
    if not out.exists():
        graph = SCENES[name]
        subprocess.run(["ffmpeg", "-nostdin", "-hide_banner", "-loglevel", "error", "-y", "-filter_complex", f"{graph}[v]", "-map", "[v]",
                        "-t", str(seconds), "-c:v", "libx264", "-preset", "ultrafast", "-pix_fmt", "yuv420p", "-g", "10", "-f", "mpegts", str(out)], check=True, timeout=300)
    return out


def render_image_scene(workdir: Path, name: str, image: Path, seconds: int = 300) -> Path:
    """A real bird photo (scaled to 200 px high) crossing a grey 640x360 frame at 120 px/s."""
    out = workdir / f"{name}.ts"
    if not out.exists():
        graph = ("color=c=0x8a8f94:s=640x360:r=10[bg];[0:v]scale=-2:200,format=yuv420p[bird];"
                 "[bg][bird]overlay=x='mod(t*120\\,500)-20':y=80:shortest=1[v]")
        subprocess.run(["ffmpeg", "-nostdin", "-hide_banner", "-loglevel", "error", "-y", "-loop", "1", "-framerate", "10", "-i", str(image),
                        "-filter_complex", graph, "-map", "[v]", "-t", str(seconds), "-c:v", "libx264", "-preset", "ultrafast", "-pix_fmt", "yuv420p",
                        "-g", "10", "-f", "mpegts", str(out)], check=True, timeout=300)
    return out


class FeederServer:
    """Serves each scene as an endless HTTP MPEG-TS stream at roughly its real bitrate."""

    def __init__(self, scenes: dict[str, Path], seconds: int):
        self.scenes = {name: path.read_bytes() for name, path in scenes.items()}
        self.seconds = seconds
        outer = self

        class Handler(BaseHTTPRequestHandler):
            def log_message(self, *_args):
                pass

            def do_GET(self):  # noqa: N802
                name = self.path.strip("/").removesuffix(".ts")
                data = outer.scenes.get(name)
                if data is None:
                    self.send_error(404)
                    return
                self.send_response(200)
                self.send_header("Content-Type", "video/mp2t")
                self.end_headers()
                rate = max(len(data) / outer.seconds, 188 * 50)  # bytes per second
                chunk = 188 * 64
                try:
                    while True:
                        start = time.monotonic()
                        for offset in range(0, len(data), chunk):
                            self.wfile.write(data[offset:offset + chunk])
                            target = start + (offset + chunk) / rate
                            delay = target - time.monotonic()
                            if delay > 0:
                                time.sleep(delay)
                        # End of clip: close, so the client reconnects at a clean start.
                        return
                except (BrokenPipeError, ConnectionResetError):
                    return

        self.server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.server.daemon_threads = True
        self.port = self.server.server_address[1]
        threading.Thread(target=self.server.serve_forever, daemon=True).start()

    def url(self, name: str) -> str:
        return f"http://127.0.0.1:{self.port}/{name}.ts"

    def close(self) -> None:
        self.server.shutdown()


# --------------------------------------------------------------------------- detector


def find_birds(jpeg: bytes) -> list[dict]:
    """Boxes of the saturated red and blue squares, normalized to the submitted image."""
    image = np.asarray(Image.open(io.BytesIO(jpeg)).convert("RGB")).astype(np.int16)
    height, width = image.shape[:2]
    r, g, b = image[..., 0], image[..., 1], image[..., 2]
    boxes = []
    for colour, mask in (("red", (r > 150) & (g < 90) & (b < 90)), ("blue", (b > 150) & (r < 90) & (g < 90))):
        ys, xs = np.nonzero(mask)
        if len(xs) < 200:
            continue
        x0, x1, y0, y1 = xs.min(), xs.max() + 1, ys.min(), ys.max() + 1
        boxes.append({"colour": colour, "x_min": x0 / width, "y_min": y0 / height, "x_max": x1 / width, "y_max": y1 / height,
                      "px": [int(x0), int(y0), int(x1), int(y1)]})
    return boxes


class Detector:
    """Loopback light-object-detect style detector, armed per camera key.

    ``POST /detect/<key>`` always locates the birds in the submitted JPEG, but
    reports them only while ``arm(key, frames)`` has frames left, so each
    scenario creates an exact number of observation batches.
    """

    def __init__(self, workdir: Path):
        self.log_path = workdir / "detector.jsonl"
        self._lock = threading.Lock()
        self.quota: dict[str, int] = {}
        self.reported: list[dict] = []   # every request that returned birds
        self.requests = 0
        outer = self

        class Handler(BaseHTTPRequestHandler):
            def log_message(self, *_args):
                pass

            def do_POST(self):  # noqa: N802
                length = int(self.headers.get("Content-Length") or 0)
                body = self.rfile.read(length)
                key = self.path.split("?", 1)[0].strip("/").split("/")[-1]
                start, end = body.find(b"\xff\xd8\xff"), body.rfind(b"\xff\xd9")
                detections: list[dict] = []
                record = None
                if start >= 0 and end > start:
                    jpeg = body[start:end + 2]
                    with outer._lock:
                        outer.requests += 1
                        armed = outer.quota.get(key, 0) > 0
                        birds = find_birds(jpeg) if armed else []
                        if armed and birds:
                            outer.quota[key] -= 1
                            record = {"t": time.time(), "key": key, "sha256": hashlib.sha256(jpeg).hexdigest(), "bytes": len(jpeg), "birds": birds}
                            outer.reported.append(record)
                    if record:
                        with open(outer.log_path, "a", encoding="utf-8") as sink:
                            sink.write(json.dumps(record) + "\n")
                    detections = [{"label": "bird", "confidence": 0.9 if bird["colour"] == "red" else 0.85,
                                   "x_min": bird["x_min"], "y_min": bird["y_min"], "x_max": bird["x_max"], "y_max": bird["y_max"]} for bird in birds]
                payload = json.dumps({"model": "x1-colour-detector", "model_version": "1.0", "detections": detections}).encode()
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(payload)))
                self.end_headers()
                self.wfile.write(payload)

        self.server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.server.daemon_threads = True
        self.port = self.server.server_address[1]
        threading.Thread(target=self.server.serve_forever, daemon=True).start()

    def url(self, key: str) -> str:
        return f"http://127.0.0.1:{self.port}/detect/{key}"

    def arm(self, key: str, frames: int) -> None:
        with self._lock:
            self.quota[key] = self.quota.get(key, 0) + frames

    def remaining(self, key: str) -> int:
        with self._lock:
            return self.quota.get(key, 0)

    def disarm(self, key: str) -> None:
        with self._lock:
            self.quota[key] = 0

    def reports(self, key: Optional[str] = None) -> list[dict]:
        with self._lock:
            return [r for r in self.reported if key is None or r["key"] == key]

    def close(self) -> None:
        self.server.shutdown()


# --------------------------------------------------------------------------- lightNVR


class LightNvr:
    ADMIN_PASSWORD = "X1-E2E-Admin-123!"

    def __init__(self, workdir: Path, binary: Path, web_root: Path, port: int, mqtt_port: int, topic_prefix: str = "lightnvr"):
        self.dir, self.binary, self.port = workdir, binary, port
        for sub in ("recordings", "models"):
            (workdir / sub).mkdir(parents=True, exist_ok=True)
        self.base = f"http://127.0.0.1:{port}"
        self.db_path = workdir / "lightnvr.db"
        self.log_path = workdir / "server.log"
        self.proc: Optional[subprocess.Popen] = None
        self.generation = 0
        # The [mqtt] block is the recipe's step 3.1, with the suite's broker address.
        (workdir / "lightnvr.ini").write_text(
            f"[general]\npid_file = {workdir}/lightnvr.pid\nlog_file = {workdir}/server.log\nlog_level = 2\nsyslog_enabled = false\n"
            f"[storage]\npath = {workdir}/recordings\nmax_size = 0\nrecord_mp4_directly = false\n"
            f"[database]\npath = {self.db_path}\n"
            f"[web]\nport = {port}\nbind_ip = 127.0.0.1\nroot = {web_root}\nauth_enabled = true\nusername = admin\npassword = admin\nweb_thread_pool_size = 8\n"
            f"[streams]\nmax_streams = 16\n[models]\npath = {workdir}/models\n[go2rtc]\nenabled = false\nauto_start = false\n"
            f"[mqtt]\nenabled = true\nbroker_host = 127.0.0.1\nbroker_port = {mqtt_port}\ntopic_prefix = {topic_prefix}\nqos = 1\n")
        self.admin = basic("admin", "admin")

    def _log_offset(self) -> int:
        return self.log_path.stat().st_size if self.log_path.exists() else 0

    def start(self) -> None:
        assert self.proc is None or self.proc.poll() is not None
        offset = self._log_offset()
        env = {k: v for k, v in os.environ.items() if not k.startswith("LIGHTNVR_")}
        self.proc = subprocess.Popen([str(self.binary), "-c", str(self.dir / "lightnvr.ini")], cwd=str(self.dir), env=env,
                                     stdout=open(self.dir / "process.log", "ab"), stderr=subprocess.STDOUT)
        self.generation += 1

        def ready() -> bool:
            if self.proc.poll() is not None:
                raise RuntimeError(f"lightNVR exited during startup ({self.proc.returncode})")
            log = self.log_path.read_bytes()[offset:].decode("utf-8", "replace") if self.log_path.exists() else ""
            if f"Listening on 127.0.0.1:{self.port}" not in log:
                return False
            status, _h, _b = http("GET", f"{self.base}/api/streams", timeout=2)
            return status == 401
        wait_for(ready, 60, 0.2, "lightNVR to listen")
        if self.generation == 1:
            # First start: replace the bootstrap password, as an operator would.
            http_json("PUT", f"{self.base}/api/auth/users/1/password", {"old_password": "admin", "new_password": self.ADMIN_PASSWORD}, self.admin)
            self.admin = basic("admin", self.ADMIN_PASSWORD)
            http_json("POST", f"{self.base}/api/setup/status", {"complete": True}, self.admin)

    def log_since(self, offset: int) -> str:
        return self.log_path.read_bytes()[offset:].decode("utf-8", "replace")

    def stop(self) -> None:
        if self.proc and self.proc.poll() is None:
            self.proc.send_signal(signal.SIGTERM)
            try:
                self.proc.wait(30)
            except subprocess.TimeoutExpired:
                self.proc.kill()
                self.proc.wait()
        self.proc = None

    def kill(self) -> None:
        assert self.proc and self.proc.poll() is None
        self.proc.send_signal(signal.SIGKILL)
        self.proc.wait(15)
        self.proc = None

    def running(self) -> bool:
        return bool(self.proc and self.proc.poll() is None)

    def api(self, method: str, route: str, body: Any = None, expect: tuple[int, ...] = (200, 201), headers: Optional[dict[str, str]] = None) -> Any:
        return http_json(method, f"{self.base}{route}", body, headers or self.admin, expect)

    def sql(self, query: str, params: tuple = ()) -> list[tuple]:
        with sqlite3.connect(self.db_path, timeout=10) as conn:
            return conn.execute(query, params).fetchall()

    def version(self) -> str:
        try:
            return self.api("GET", "/api/system/info").get("version", "unknown")
        except Exception:  # noqa: BLE001
            return "unknown"


# --------------------------------------------------------------------------- YA-WAMF


class YaWamf:
    def __init__(self, workdir: Path, repo: Path, port: int, env: dict[str, str], launcher: Path):
        self.dir, self.repo, self.port, self.launcher = workdir, repo, port, launcher
        self.backend = repo / "backend"
        self.python = self.backend / ".venv" / "bin" / "python"
        for sub in ("data", "config"):
            (workdir / sub).mkdir(parents=True, exist_ok=True)
        self.db_path = workdir / "data" / "speciesid.db"
        self.classifier_log = workdir / "classifier.jsonl"
        self.log_path = workdir / "yawamf.log"
        self.base = f"http://127.0.0.1:{port}"
        self.env = {
            "CONFIG_FILE": str(workdir / "config" / "config.json"),
            "DATA_DIR": str(workdir / "data"),
            "DB_PATH": str(self.db_path),
            "SPECIES_CATALOG_PATH": str(workdir / "data" / "species_catalog.db"),
            "MEDIA_CACHE_DIR": str(workdir / "data" / "media_cache"),
            "MODEL_DIR": str(workdir / "data" / "models"),
            "YAWAMF_E2E_CLASSIFIER_LOG": str(self.classifier_log),
            "PATH": f"{self.backend / '.venv' / 'bin'}:{os.environ.get('PATH', '')}",
            "HOME": os.environ.get("HOME", str(workdir)),
            **env,
        }
        self.proc: Optional[subprocess.Popen] = None

    def start(self, wait_mqtt: bool = True) -> None:
        assert self.proc is None or self.proc.poll() is not None
        offset = self.log_path.stat().st_size if self.log_path.exists() else 0
        self.proc = subprocess.Popen([str(self.python), str(self.launcher), "--backend", str(self.backend), "--port", str(self.port)],
                                     cwd=str(self.dir), env=self.env, stdout=open(self.log_path, "ab"), stderr=subprocess.STDOUT)

        def ready() -> bool:
            if self.proc.poll() is not None:
                raise RuntimeError(f"YA-WAMF exited during startup ({self.proc.returncode})")
            status, _h, _b = http("GET", f"{self.base}/health", timeout=3)
            if status != 200:
                return False
            return not wait_mqtt or "Connected to MQTT" in self.log_since(offset)
        wait_for(ready, 120, 0.5, "YA-WAMF to start and subscribe")

    def log_since(self, offset: int) -> str:
        return self.log_path.read_bytes()[offset:].decode("utf-8", "replace") if self.log_path.exists() else ""

    def log_size(self) -> int:
        return self.log_path.stat().st_size if self.log_path.exists() else 0

    def stop(self) -> None:
        if self.proc and self.proc.poll() is None:
            self.proc.send_signal(signal.SIGTERM)
            try:
                self.proc.wait(60)
            except subprocess.TimeoutExpired:
                self.proc.kill()
                self.proc.wait()
        self.proc = None

    def kill(self) -> None:
        assert self.proc and self.proc.poll() is None
        self.proc.send_signal(signal.SIGKILL)
        self.proc.wait(15)
        self.proc = None

    def api(self, method: str, route: str, body: Any = None, expect: tuple[int, ...] = (200, 201), timeout: float = 60) -> Any:
        return http_json(method, f"{self.base}{route}", body, None, expect, timeout)

    def sql(self, query: str, params: tuple = ()) -> list[tuple]:
        with sqlite3.connect(self.db_path, timeout=10) as conn:
            return conn.execute(query, params).fetchall()

    def detections(self) -> list[dict]:
        rows = self.sql("SELECT frigate_event, external_observation_id, source_kind, source_instance, source_camera_id, camera_name, "
                        "scientific_name, display_name, score, notification_intent, notification_outcome FROM detections WHERE source_kind = 'lightnvr'")
        keys = ["event", "observation_id", "source_kind", "source_instance", "camera_uuid", "camera_name", "scientific_name", "display_name", "score", "intent", "outcome"]
        return [dict(zip(keys, row)) for row in rows]

    def classifier_calls(self) -> list[dict]:
        if not self.classifier_log.exists():
            return []
        return [json.loads(line) for line in self.classifier_log.read_text().splitlines() if line.strip()]
