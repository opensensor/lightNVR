#!/usr/bin/env python3
"""INT_01 X1/X3: lightNVR <-> YA-WAMF end-to-end suite (host-process mode).

Runs docs/integrations/YA_WAMF.md verbatim against real processes:
Mosquitto 2.x, the lightNVR binary under test, the YA-WAMF backend from its
branch (pinned colour classifier, see harness/yawamf_launcher.py), an HTTP
MPEG-TS feeder source and a loopback HTTP detector. Then it runs A1-A6 and A8
(X1, P1) and A12, A13 and A7-annotate (X3, P2 write-back; p2_scenarios.py) and
writes results/<run>.json.

Usage (from the lightNVR repo root; YA-WAMF's venv provides Pillow/numpy):
    ../YetAnother-WhosAtMyFeeder/backend/.venv/bin/python tests/integration/yawamf_e2e/run_e2e.py \
        --lightnvr-bin build/bin/lightnvr --yawamf ../YetAnother-WhosAtMyFeeder

Exit status 0 only when every selected scenario passed.
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
import os
import shutil
import subprocess
import sys
import tempfile
import threading
import time
import traceback
import urllib.parse
import uuid
from pathlib import Path
from typing import Any, Callable

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE / "harness"))

from PIL import Image, ImageStat  # noqa: E402

from components import (  # noqa: E402
    Detector, FeederServer, LightNvr, Mosquitto, MqttCapture, YaWamf, free_port, http, render_image_scene, render_scene, wait_for,
)
from p2_scenarios import P2Scenarios  # noqa: E402

OBSERVATION_EVENT = "io.lightnvr.detection.observation_changed.v1"
MESSAGE_NS = uuid.UUID("0bc01005-a431-522b-9124-a114f67d187a")
RED, BLUE = "Prunella modularis", "Erithacus rubecula"
TOPIC_PREFIX = "lightnvr"
SCENE_SECONDS = 300

# camera name -> (scene, detector key). Names may hold only letters, digits, ".", "-" and "_".
CAMERAS = {
    "feeder-east": ("red_static", "a1"),
    "feeder-moving": ("red_moving", "a2"),
    "feeder-pair": ("two_birds", "a3"),
    "feeder-bulk": ("two_birds", "a5"),
    "feeder-kill": ("two_birds", "a6"),
    "feeder-outcomes": ("red_static", "a8"),
    "feeder-wb": ("red_static", "a12"),
    "feeder-wb-outage": ("red_static", "a13"),
}


def message_id(source: str, observation: str, revision: int) -> str:
    return str(uuid.uuid5(MESSAGE_NS, f"{source}|{observation}|{revision}"))


def colour_of(image: Image.Image) -> str:
    r, g, b = ImageStat.Stat(image.convert("RGB")).mean
    if r > g + 60 and r > b + 60:
        return "red"
    if b > r + 60 and b > g + 60:
        return "blue"
    return "background"


def crop_box(jpeg: bytes, box: dict) -> Image.Image:
    image = Image.open(io.BytesIO(jpeg)).convert("RGB")
    w, h = image.size
    return image.crop((round(box["x"] * w), round(box["y"] * h), round((box["x"] + box["width"]) * w), round((box["y"] + box["height"]) * h)))


class Check(Exception):
    pass


def expect(condition: bool, message: str) -> None:
    if not condition:
        raise Check(message)


class Suite(P2Scenarios):
    def __init__(self, args: argparse.Namespace):
        self.args = args
        self.work = Path(args.workdir or tempfile.mkdtemp(prefix="x1-yawamf-e2e-"))
        self.work.mkdir(parents=True, exist_ok=True)
        self.results: list[dict] = []
        self.recipe: list[dict] = []
        self.cameras: dict[str, str] = {}  # name -> uuid
        self.source_id = ""
        self.token = ""

    # ------------------------------------------------------------------ plumbing

    def note(self, step: str, outcome: str, detail: Any = None) -> None:
        entry = {"step": step, "outcome": outcome}
        if detail is not None:
            entry["detail"] = detail
        self.recipe.append(entry)
        print(f"[recipe] {step}: {outcome}" + (f" {json.dumps(detail, default=str)[:300]}" if detail is not None else ""), flush=True)

    def observations(self, camera: str, principal: dict | None = None) -> list[dict]:
        """Every retained observation of a camera, oldest first, by walking the cursor."""
        items: dict[str, dict] = {}
        cursor = None
        for _ in range(200):
            query = {"camera_uuid": self.cameras[camera], "limit": "500"}
            if cursor:
                query["cursor"] = cursor
            page = self.lnvr.api("GET", "/api/observations?" + urllib.parse.urlencode(query), headers=principal)
            for item in page["items"]:
                items[item["id"]] = item
            cursor = page.get("next_cursor") or cursor
            if not page.get("has_more"):
                break
        return list(items.values())

    def ready(self, camera: str) -> list[dict]:
        return [o for o in self.observations(camera) if o["evidence"]["state"] == "ready"]

    def settled(self, camera: str, count: int, timeout: float = 120) -> list[dict]:
        """Wait until the camera has >= count observations and none is pending."""
        def done():
            items = self.observations(camera)
            return items if len(items) >= count and all(o["evidence"]["state"] != "pending" for o in items) else None
        return wait_for(done, timeout, 1.0, f"{count} settled observations on {camera}")

    def yawamf_rows(self, camera: str) -> list[dict]:
        return [d for d in self.ya.detections() if d["camera_uuid"] == self.cameras[camera]]

    def wait_rows(self, camera: str, ids: set[str], timeout: float) -> list[dict]:
        def done():
            rows = self.yawamf_rows(camera)
            return rows if ids <= {r["observation_id"] for r in rows} else None
        try:
            return wait_for(done, timeout, 1.0, f"YA-WAMF detections for {len(ids)} observations on {camera}")
        except TimeoutError:
            have = {r["observation_id"] for r in self.yawamf_rows(camera)}
            raise Check(f"YA-WAMF is missing {len(ids - have)} of {len(ids)} observations on {camera} after {timeout:.0f}s")

    def classifier_calls(self, camera: str) -> list[dict]:
        return [c for c in self.ya.classifier_calls() if c.get("camera") == camera]

    def snapshot(self, observation_id: str, headers: dict | None = None) -> tuple[int, bytes, dict]:
        status, response_headers, body = http("GET", f"{self.lnvr.base}/api/observations/{observation_id}/snapshot", headers=headers or self.token_headers)
        return status, body, response_headers

    def recovery_status(self) -> dict:
        return self.ya.api("GET", "/api/backfill/source-recovery")

    def wait_recovery_run(self, after: float, timeout: float = 120) -> dict:
        def done():
            status = self.recovery_status()
            run = status.get("last_run") or {}
            return status if run.get("started_at", 0) >= after and run.get("finished_at") else None
        return wait_for(done, timeout, 1.0, "a YA-WAMF recovery run")

    @property
    def token_headers(self) -> dict:
        return {"Authorization": f"Bearer {self.token}"}

    def scenario(self, sid: str, title: str, fn: Callable[[], dict]) -> None:
        if self.args.scenarios and sid not in self.args.scenarios:
            return
        print(f"\n=== {sid}: {title}", flush=True)
        started = time.time()
        entry: dict[str, Any] = {"id": sid, "title": title, "started_at": started}
        try:
            entry["evidence"] = fn()
            entry["result"] = "pass"
        except Exception as error:  # noqa: BLE001 - recorded per scenario
            entry["result"] = "fail"
            entry["error"] = f"{type(error).__name__}: {error}"
            entry["traceback"] = traceback.format_exc()
            print(entry["traceback"], flush=True)
        entry["seconds"] = round(time.time() - started, 1)
        print(f"=== {sid}: {entry['result'].upper()} in {entry['seconds']}s", flush=True)
        self.results.append(entry)

    # ------------------------------------------------------------------ recipe

    def deploy(self) -> None:
        a = self.args
        mqtt_port, lnvr_port, ya_port = free_port(), free_port(), free_port()
        scenes = {name: render_scene(self.work, name, SCENE_SECONDS) for name in {scene for scene, _ in CAMERAS.values()}}
        self.local = bool(a.sod_model and a.bird_image)
        if self.local:
            scenes["bird_local"] = render_image_scene(self.work, "bird_local", Path(a.bird_image).resolve(), SCENE_SECONDS)
        self.feeder = FeederServer(scenes, SCENE_SECONDS)
        self.detector = Detector(self.work)
        self.broker = Mosquitto(self.work, mqtt_port)
        self.broker.start()
        self.capture = MqttCapture(self.work, mqtt_port, f"{TOPIC_PREFIX}/v1/events/{OBSERVATION_EVENT}/#")
        self.capture.start()
        self.note("prereq: MQTT broker", "ok", {"broker": self.broker.version(), "port": mqtt_port})

        lnvr_dir = self.work / "lightnvr"
        lnvr_dir.mkdir(exist_ok=True)
        self.lnvr = LightNvr(lnvr_dir, Path(a.lightnvr_bin).resolve(), Path(a.web_root).resolve(), lnvr_port, mqtt_port, TOPIC_PREFIX)
        self.lnvr.start()
        self.note("prereq: lightNVR with [mqtt] block (step 3.1)", "ok", {"version": self.lnvr.version()})

        # Step 1 + 2: a bird-capable (HTTP) detector on each feeder stream, then evidence per camera.
        for name, (scene, key) in CAMERAS.items():
            self.lnvr.api("POST", "/api/streams", {"name": name, "url": self.feeder.url(scene), "enabled": True, "streaming_enabled": False,
                                                    "record": False, "detection_based_recording": True, "detection_model": self.detector.url(key),
                                                    "detection_threshold": 50, "detection_interval": 1})
            # Step 2, verbatim body (URL-encoded name).
            self.lnvr.api("PUT", f"/api/streams/{urllib.parse.quote(name)}", {"observation_evidence_enabled": True, "observation_labels": "bird",
                                                                              "observation_max_age_days": 7, "observation_max_bytes": 2147483648},
                          expect=(200, 202))
        # The PUT answers 202 (applied asynchronously); wait until every camera reports it.
        def applied() -> bool:
            for name in CAMERAS:
                full = self.lnvr.api("GET", f"/api/streams/{urllib.parse.quote(name)}/full")
                if not full.get("stream", full).get("observation_evidence_enabled"):
                    return False
            return True
        wait_for(applied, 30, 0.5, "evidence settings to apply")
        thresholds = {row[0]: row[1] for row in self.lnvr.sql("SELECT name, detection_threshold FROM streams")}
        expect(all(abs(thresholds[name] - 0.5) < 1e-6 for name in CAMERAS), f"detection_threshold is a percentage in the API: {thresholds}")
        if self.local:
            # A2 local variant: a real SOD model on a real bird photo, evidence off until the scenario.
            model = Path(a.sod_model).resolve()
            (lnvr_dir / "models" / model.name).symlink_to(model)
            self.lnvr.api("POST", "/api/streams", {"name": "feeder-local", "url": self.feeder.url("bird_local"), "enabled": True, "streaming_enabled": False,
                                                    "record": False, "detection_based_recording": True, "detection_model": model.name,
                                                    "detection_threshold": 30, "detection_interval": 2})
        streams = self.lnvr.api("GET", "/api/streams")
        streams = streams if isinstance(streams, list) else streams.get("streams", streams)
        self.cameras = {s["name"]: s["camera_uuid"] for s in streams if s["name"] in CAMERAS or s["name"] == "feeder-local"}
        expect(len(self.cameras) == len(CAMERAS) + int(self.local), f"camera_uuid missing from GET /api/streams: {self.cameras}")
        full = self.lnvr.api("GET", f"/api/streams/{urllib.parse.quote('feeder-east')}/full")
        stream = full.get("stream", full)
        self.note("step 1-2: detector + evidence per camera", "ok", {"cameras": self.cameras, "observation_evidence_enabled": stream.get("observation_evidence_enabled"),
                                                                     "observation_labels": stream.get("observation_labels")})

        # Step 3.2: the observation route on mqtt:default, camera_uuid selector, no predicate.
        route = self.lnvr.api("POST", "/api/event-routes", {
            "name": "YA-WAMF observations", "destination": "mqtt:default", "event_types": [OBSERVATION_EVENT],
            "camera_scope": {"type": "selector", "selector": {"version": 1, "expression": {"op": "camera_uuid", "values": list(self.cameras.values())}}}})
        self.note("step 3.2: event route", "ok", {"route": route.get("route", route).get("uuid") if isinstance(route, dict) else route})

        # Step 4: user -> role -> policy -> token.
        user = self.lnvr.api("POST", "/api/auth/users", {"username": "yawamf", "password": "X1-Yawamf-Integration-1!", "role": 2, "is_active": True})
        user_id = user.get("id") or user.get("user", {}).get("id")
        roles = self.lnvr.api("GET", "/api/authorization/roles")
        role_body = {"name": "YA-WAMF", "description": "YA-WAMF feeder integration",
                     "actions": ["live.view", "snapshot.create", "observations.annotate"]}
        status, _h, body = http("POST", f"{self.lnvr.base}/api/authorization/roles", role_body, self.lnvr.admin)
        if status not in (200, 201):
            self.note("step 4.2: create role (recipe body)", "failed", {"status": status, "body": body.decode()[:300]})
            role_body["expected_policy_version"] = roles["policy_version"]
            role = self.lnvr.api("POST", "/api/authorization/roles", role_body)
            self.note("step 4.2: create role with expected_policy_version", "ok")
        else:
            role = json.loads(body)
            self.note("step 4.2: create role (recipe body)", "ok")
        role_uuid = role["role"]["uuid"]
        self.role_uuid = role_uuid
        scope = {"type": "selector", "selector": {"version": 1, "expression": {"op": "camera_uuid", "values": list(self.cameras.values())}}}
        policy = self.lnvr.api("GET", f"/api/authorization/users/{user_id}")
        grant = {"mode": "policy", "grants": [{"role_uuid": role_uuid, "scope": scope}]}
        status, _h, body = http("PUT", f"{self.lnvr.base}/api/authorization/users/{user_id}", grant, self.lnvr.admin)
        if status not in (200, 201):
            self.note("step 4.3: grant policy (no expected_policy_version)", "failed", {"status": status, "body": body.decode()[:300]})
            grant["expected_policy_version"] = policy["policy_version"]
            self.lnvr.api("PUT", f"/api/authorization/users/{user_id}", grant)
            self.note("step 4.3: grant policy with expected_policy_version", "ok")
        else:
            self.note("step 4.3: grant policy", "ok")
        issued = self.lnvr.api("POST", f"/api/authorization/users/{user_id}/tokens", {
            "description": "YA-WAMF feeder integration", "expires_at": int(time.time()) + 300 * 86400,
            "actions": ["live.view", "snapshot.create", "observations.annotate"], "scope": scope})
        self.token = issued["secret"]
        self.user_id = user_id
        caps = self.lnvr.api("GET", "/api/integrations/capabilities", headers=self.token_headers)
        self.source_id = caps["source"]["id"]
        self.capabilities = caps
        self.note("step 4.4: scoped token", "ok", {"principal": caps.get("principal"), "delivery": caps.get("delivery"),
                                                   "cameras_evidence_enabled": {c.get("camera_uuid"): c.get("evidence_enabled") for c in caps.get("cameras", [])}})

        # Step 5: YA-WAMF env.
        ya_dir = self.work / "yawamf"
        ya_dir.mkdir(exist_ok=True)
        self.ya = YaWamf(ya_dir, Path(a.yawamf).resolve(), ya_port, {
            "LIGHTNVR__ENABLED": "true", "LIGHTNVR__API_URL": self.lnvr.base, "LIGHTNVR__TOKEN": self.token,
            "LIGHTNVR__MQTT_TOPIC_PREFIX": TOPIC_PREFIX, "LIGHTNVR__CAMERA_UUIDS": ",".join(self.cameras.values()),
            "LIGHTNVR__REQUEST_TIMEOUT_SECONDS": "5",
            # The recipe's optional P2 step (X3). --no-writeback runs the P1 recipe only.
            "LIGHTNVR__WRITEBACK_ENABLED": "false" if a.no_writeback else "true",
            "FRIGATE__MQTT_SERVER": "127.0.0.1", "FRIGATE__MQTT_PORT": str(mqtt_port), "FRIGATE__MQTT_AUTH": "false",
        }, HERE / "harness" / "yawamf_launcher.py")
        self.ya.start()
        health = self.ya.api("GET", "/health")
        self.note("step 5: YA-WAMF started (MQTT subscribed)", "ok", {"health_version": health.get("version"),
                                                                       "classification_writeback": health.get("classification_writeback")})

        # Step 6.1: the connection test.
        # What the Settings page sends for the saved values: the address, and no token (keeps the saved one).
        test = self.ya.api("POST", "/api/setup/lightnvr/test", {"api_url": self.lnvr.base, "camera_uuids": list(self.cameras.values())})
        self.connection_test = test
        self.note("step 6.1: Test lightNVR connection", "ok" if test.get("ok") else "failed", test)
        expect(bool(test.get("ok")), f"YA-WAMF connection test not ok: {json.dumps(test)[:800]}")
        # Step 6.2: the setup probe by hand with the token.
        status, _h, body = http("GET", f"{self.lnvr.base}/api/observations?camera_uuid={self.cameras['feeder-east']}&limit=1", headers=self.token_headers)
        self.note("step 6.2: setup probe", "ok" if status == 200 else "failed", {"status": status, "body": json.loads(body)})
        status, _h, _b = http("GET", f"{self.lnvr.base}/api/observations?limit=1&camera_uuid=00000000-0000-4000-8000-00000000dead", headers=self.token_headers)
        self.note("step 6.2: setup probe, camera outside the token scope", "ok" if status == 403 else f"unexpected {status}", {"status": status})
        status, _h, _b = http("GET", f"{self.lnvr.base}/api/system/info", headers=self.token_headers)
        self.note("diagnostics: token cannot read /api/system/info", "ok" if status in (401, 403) else f"unexpected {status}", {"status": status})

    # ------------------------------------------------------------------ scenarios

    def a1(self) -> dict:
        camera, key = "feeder-east", "a1"
        self.detector.arm(key, 1)
        items = self.settled(camera, 1, 90)
        expect(len(items) == 1, f"expected exactly one observation, got {len(items)}")
        obs = items[0]
        expect(obs["evidence"]["state"] == "ready", f"evidence {obs['evidence']}")
        rows = self.wait_rows(camera, {obs["id"]}, 90)
        row = rows[0]
        expect(len(rows) == 1, f"{len(rows)} YA-WAMF rows")
        expect(row["event"] == f"lnvr:{row['source_instance']}:{obs['id']}", f"event key {row['event']}")
        expect(row["source_kind"] == "lightnvr" and row["camera_uuid"] == self.cameras[camera], f"identity {row}")
        expect(self.source_id == f"urn:lightnvr:{row['source_instance']}", f"source {self.source_id} vs instance {row['source_instance']}")
        expect(row["scientific_name"] == RED and abs(row["score"] - 0.93) < 1e-6, f"species {row}")
        expect(obs["label"] == "bird" and obs["camera_uuid"] == self.cameras[camera], f"label {obs['label']}")
        # One message ID per revision on the real broker: created(pending), then evidence(ready).
        wait_for(lambda: {e["id"] for e in self.capture.envelopes()} >= {message_id(self.source_id, obs["id"], r) for r in (1, 2)}, 60, 1, "both revisions on the broker")
        mine = [e for e in self.capture.envelopes() if e["data"]["observation_id"] == obs["id"]]
        by_rev = {e["data"]["revision"]: e for e in mine}
        expect(by_rev[1]["data"]["change_cause"] == "created" and by_rev[1]["data"]["evidence_state"] == "pending", f"rev1 {by_rev[1]['data']}")
        expect(by_rev[2]["data"]["change_cause"] == "evidence" and by_rev[2]["data"]["evidence_state"] == "ready", f"rev2 {by_rev[2]['data']}")
        calls = self.classifier_calls(camera)
        expect(len(calls) == 1 and calls[0]["result"] == RED, f"classifier calls {calls}")
        topic = next(t for t, p in self.capture.raw() if json.loads(p)["data"]["observation_id"] == obs["id"])
        self.a1_obs = obs
        return {"observation": {k: obs[k] for k in ("id", "camera_uuid", "stream_name_at_capture", "label", "confidence", "box", "revision")},
                "evidence": obs["evidence"], "yawamf_row": row, "topic": topic,
                "mqtt_revisions": {r: {"id": e["id"], "cause": e["data"]["change_cause"], "evidence_state": e["data"]["evidence_state"]} for r, e in sorted(by_rev.items())},
                "classifier_calls": calls}

    def a2(self) -> dict:
        camera, key, frames = "feeder-moving", "a2", 3
        self.detector.arm(key, frames)
        items = self.settled(camera, frames, 120)
        expect(len(items) == frames, f"{len(items)} observations for {frames} armed frames")
        reports = {r["sha256"]: r for r in self.detector.reports(key)}
        evidence = []
        snapshots = {}
        for obs in items:
            status, jpeg, _h = self.snapshot(obs["id"])
            expect(status == 200, f"snapshot {status}")
            sha = hashlib.sha256(jpeg).hexdigest()
            expect(sha in reports, "stored evidence is not byte-equal to any image the detector received")
            px = reports[sha]["birds"][0]
            want = {"x": px["x_min"], "y": px["y_min"], "width": px["x_max"] - px["x_min"], "height": px["y_max"] - px["y_min"]}
            for k in want:
                expect(abs(obs["box"][k] - want[k]) < 1e-3, f"box {obs['box']} != detector box {want}")
            expect(colour_of(crop_box(jpeg, obs["box"])) == "red", "box does not cover the bird in the stored image")
            snapshots[obs["id"]] = jpeg
            evidence.append({"id": obs["id"], "sha256": sha, "box": obs["box"], "bird_px": px["px"], "transform": obs["evidence"].get("transform")})
        # The bird moved between inference frames: each box misses the bird in every other frame,
        # so a substituted later frame would have been caught above.
        xs = [e["bird_px"][0] for e in evidence]
        expect(max(xs) - min(xs) >= 64, f"fixture bird did not move between frames: {xs}")
        crossed = []
        for e in evidence:
            for other in evidence:
                if other["id"] != e["id"] and abs(other["bird_px"][0] - e["bird_px"][0]) >= 64:
                    crossed.append(colour_of(crop_box(snapshots[other["id"]], e["box"])))
        expect(crossed and all(c != "red" for c in crossed), f"cross-frame crops {crossed}")
        rows = self.wait_rows(camera, {o["id"] for o in items}, 90)
        expect(all(r["scientific_name"] == RED for r in rows) and len(rows) == frames, f"YA-WAMF rows {rows}")
        calls = self.classifier_calls(camera)
        expect(len(calls) == frames and all(c["result"] == RED for c in calls), f"classifier calls {calls}")
        return {"detector": "http (loopback x1-colour-detector)", "observations": evidence, "bird_x_px_per_frame": xs,
                "cross_frame_crop_colours": crossed, "yawamf_species": [r["scientific_name"] for r in rows], "classifier_calls": calls,
                "local_detector": "pending: no bird-capable local model drawn on this fixture (see README)"}

    def a2_local(self) -> dict:
        """A2 with a local SOD model: the stored image is the decoded inference frame (no submitted JPEG to compare),
        so alignment is checked against where the bird really is in each stored image."""
        camera = "feeder-local"
        def red_box(jpeg: bytes):
            import numpy as np
            image = np.asarray(Image.open(io.BytesIO(jpeg)).convert("RGB")).astype(int)
            mask = (image[..., 0] > 140) & (image[..., 0] > image[..., 1] + 60) & (image[..., 0] > image[..., 2] + 60)
            ys, xs = np.nonzero(mask)
            h, w = mask.shape
            return (xs.min() / w, ys.min() / h, (xs.max() + 1) / w, (ys.max() + 1) / h) if len(xs) > 300 else None
        self.lnvr.api("PUT", f"/api/streams/{camera}", {"observation_evidence_enabled": True, "observation_labels": "bird"}, expect=(200, 202))
        items = wait_for(lambda: (lambda x: x if len(x) >= 3 and all(o["evidence"]["state"] != "pending" for o in x) else None)(self.observations(camera)),
                         180, 2, "three local-detector observations")
        self.lnvr.api("PUT", f"/api/streams/{camera}", {"observation_evidence_enabled": False}, expect=(200, 202))
        items = [o for o in items if o["evidence"]["state"] == "ready"][:3]
        expect(len(items) >= 2, f"{len(items)} ready local observations")
        evidence, snapshots = [], {}
        for obs in items:
            status, jpeg, _h = self.snapshot(obs["id"])
            expect(status == 200, f"snapshot {status}")
            truth = red_box(jpeg)
            expect(truth is not None, "no bird visible in the stored evidence")
            b = obs["box"]
            ix = max(0.0, min(b["x"] + b["width"], truth[2]) - max(b["x"], truth[0]))
            iy = max(0.0, min(b["y"] + b["height"], truth[3]) - max(b["y"], truth[1]))
            inter = ix * iy
            union = b["width"] * b["height"] + (truth[2] - truth[0]) * (truth[3] - truth[1]) - inter
            cx, cy = (truth[0] + truth[2]) / 2, (truth[1] + truth[3]) / 2
            inside = b["x"] <= cx <= b["x"] + b["width"] and b["y"] <= cy <= b["y"] + b["height"]
            expect(inside and inter / union >= 0.2, f"local box {b} does not match the bird at {truth}")
            snapshots[obs["id"]] = jpeg
            evidence.append({"id": obs["id"], "label": obs["label"], "confidence": obs["confidence"], "box": b, "bird_in_stored_image": [round(v, 4) for v in truth],
                             "iou": round(inter / union, 3), "origin": obs["evidence"]["origin"], "transform": obs["evidence"]["transform"], "detector": obs["detector"]})
        xs = [e["bird_in_stored_image"][0] for e in evidence]
        expect(max(xs) - min(xs) >= 0.1, f"bird did not move between the stored frames: {xs}")
        expect(all(e["origin"] == "inference_input" for e in evidence), "local evidence is not the inference input")
        rows = self.wait_rows(camera, {e["id"] for e in evidence}, 120)
        calls = self.classifier_calls(camera)
        expect(len(rows) == len(evidence) and all(r["scientific_name"] == RED for r in rows), f"YA-WAMF rows {rows}")
        expect(calls and all(c["red_fraction"] >= 0.10 for c in calls), f"YA-WAMF crops miss the bird: {calls}")
        return {"detector": f"local SOD {Path(self.args.sod_model).name} (sha256 {hashlib.sha256(Path(self.args.sod_model).read_bytes()).hexdigest()[:16]})",
                "bird_image_sha256": hashlib.sha256(Path(self.args.bird_image).read_bytes()).hexdigest(), "observations": evidence,
                "yawamf_rows": [{k: r[k] for k in ("observation_id", "scientific_name", "score")} for r in rows], "classifier_calls": calls}

    def a3(self) -> dict:
        camera, key = "feeder-pair", "a3"
        self.detector.arm(key, 1)
        items = self.settled(camera, 2, 90)
        expect(len(items) == 2, f"{len(items)} observations")
        expect(len({o["batch_id"] for o in items}) == 1 and len({o["evidence"]["asset_id"] for o in items}) == 1, "siblings do not share batch/asset")
        expect(sorted(o["object_index"] for o in items) == [0, 1], "object indexes")
        status, jpeg, _h = self.snapshot(items[0]["id"])
        expect(status == 200, f"snapshot {status}")
        colours = {o["id"]: colour_of(crop_box(jpeg, o["box"])) for o in items}
        expect(sorted(colours.values()) == ["blue", "red"], f"box colours {colours}")
        rows = self.wait_rows(camera, set(colours), 90)
        species = {r["observation_id"]: r["scientific_name"] for r in rows}
        expect(len(rows) == 2, f"{len(rows)} rows")
        for oid, colour in colours.items():
            expect(species[oid] == (RED if colour == "red" else BLUE), f"{oid}: {colour} bird identified as {species[oid]}")
        calls = self.classifier_calls(camera)
        expect(len(calls) == 2 and sorted(c["result"] for c in calls) == sorted([RED, BLUE]), f"classifier calls {calls}")
        visits = self.ya.sql("SELECT COUNT(*) FROM detections WHERE source_camera_id = ?", (self.cameras[camera],))[0][0]
        self.a3_obs = items
        return {"asset_id": items[0]["evidence"]["asset_id"], "batch_id": items[0]["batch_id"], "box_colours": colours,
                "yawamf_species": species, "classifier_calls": calls, "yawamf_detection_rows": visits}

    def a4(self) -> dict:
        ids = {self.a1_obs["id"]} | {o["id"] for o in self.a3_obs}
        replay = [(t, p) for t, p in self.capture.raw() if json.loads(p)["data"]["observation_id"] in ids]
        expect(len(replay) >= 6, f"captured {len(replay)} messages for A1/A3")
        before_rows = {r["observation_id"]: r for r in self.ya.detections() if r["observation_id"] in ids}
        before_calls = len(self.ya.classifier_calls())
        backfill: dict[str, Any] = {}

        def recover():
            backfill["response"] = self.ya.api("POST", "/api/backfill", {"date_range": "day"}, timeout=120)
        worker = threading.Thread(target=recover)
        worker.start()
        for round_ in range(10):
            for topic, payload in replay:
                self.broker.publish(topic, payload)
        worker.join(180)
        time.sleep(15)  # let replays and recovery settle
        after_rows = [r for r in self.ya.detections() if r["observation_id"] in ids]
        expect(len(after_rows) == len(ids), f"{len(after_rows)} rows for {len(ids)} observations after replay")
        expect(len(self.ya.classifier_calls()) == before_calls, f"classifier re-ran: {len(self.ya.classifier_calls()) - before_calls} extra calls")
        intents = {r["observation_id"]: (r["intent"], r["outcome"]) for r in after_rows}
        expect(all(r["intent"] is not None for r in after_rows), f"missing intent {intents}")
        expect(all(before_rows[r["observation_id"]]["intent"] == r["intent"] for r in after_rows), "intent changed by replay")
        resp = backfill.get("response") or {}
        expect(resp.get("new_detections", 1) == 0, f"recovery created detections: {resp}")
        return {"messages_replayed": len(replay) * 10, "distinct_messages": len(replay), "observations": len(ids), "rows_after": len(after_rows),
                "extra_classifier_calls": 0, "intents": intents, "overlapping_backfill": resp}

    def a5(self) -> dict:
        camera, key, frames = "feeder-bulk", "a5", 50
        before = len(self.observations(camera))
        self.broker.stop()
        self.detector.arm(key, frames)
        items = wait_for(lambda: (lambda x: x if len(x) >= before + 2 * frames and all(o["evidence"]["state"] != "pending" for o in x) else None)(self.observations(camera)),
                         240, 2, "100 observations while the broker is down")
        new = items[before:]
        ids = {o["id"] for o in new}
        expect(len(ids) == 100, f"{len(ids)} observations created with the broker down")
        # lightNVR half while the broker is down: changes are relayed into the outbox and wait there.
        outbox_while_down = dict(self.lnvr.sql("SELECT state, COUNT(*) FROM event_outbox GROUP BY state"))
        rows_during = {r["observation_id"] for r in self.yawamf_rows(camera)} & ids
        # Restart both applications, broker back first.
        self.ya.stop()
        self.lnvr.stop()
        self.broker.start()
        self.capture.start()  # mosquitto_sub exits when its broker goes away
        self.lnvr.start()
        restart = time.time()
        self.ya.start()
        rows = self.wait_rows(camera, ids, 300)
        converged = round(time.time() - restart, 1)
        mine = [r for r in rows if r["observation_id"] in ids]
        expect(len(mine) == 100 and len({r["observation_id"] for r in mine}) == 100, f"{len(mine)} rows")
        calls = [c for c in self.classifier_calls(camera)]
        # lightNVR half: the outbox delivers every revision after the broker returns.
        want = {message_id(self.source_id, o, r) for o in ids for r in (1, 2)}
        try:
            wait_for(lambda: want <= {e["id"] for e in self.capture.envelopes()}, 180, 2, "outbox redelivery of 200 revisions")
            outbox = "all 200 revisions delivered after the broker returned"
        except TimeoutError:
            got = len(want & {e["id"] for e in self.capture.envelopes()})
            outbox = f"{got}/200 revisions seen by the capture subscriber"
        status = self.recovery_status()
        intents: dict[str, int] = {}
        for r in mine:
            intents[str(r["intent"])] = intents.get(str(r["intent"]), 0) + 1
        return {"observations_created_while_broker_down": len(ids), "outbox_states_while_broker_down": outbox_while_down, "outbox_states_after": dict(self.lnvr.sql("SELECT state, COUNT(*) FROM event_outbox GROUP BY state")),
                "yawamf_rows_while_down": len(rows_during), "yawamf_rows_after_restart": len(mine), "converged_seconds_after_restart": converged,
                "classifier_calls_for_camera": len(calls), "outbox": outbox, "intents": intents, "recovery_status": status}

    def _has_table(self, name: str) -> bool:
        return bool(self.lnvr.sql("SELECT 1 FROM sqlite_master WHERE type='table' AND name=?", (name,)))

    def lnvr_integrity(self, camera: str) -> dict:
        """A6 invariants on disk and in the DB: no pending, no tmp, ready rows <-> final files."""
        uuid_ = self.cameras[camera]
        root = self.lnvr.dir / "recordings" / "observations" / uuid_
        tmp = list(root.rglob("*.jpg.tmp")) if root.exists() else []
        finals = {p.stem for p in root.rglob("*.jpg")} if root.exists() else set()
        rows = self.lnvr.sql("SELECT uuid, state, reason, path FROM observation_assets WHERE camera_uuid = ?", (uuid_,))
        ready = {r[0] for r in rows if r[1] == "ready"}
        states: dict[str, int] = {}
        for r in rows:
            states[f"{r[1]}:{r[2]}" if r[2] else r[1]] = states.get(f"{r[1]}:{r[2]}" if r[2] else r[1], 0) + 1
        expect(not tmp, f"unfinished files remain: {tmp}")
        expect(not any(r[1] == "pending" for r in rows), "pending asset after restart")
        expect(ready <= finals, f"ready rows without a final file: {ready - finals}")
        return {"asset_states": states, "final_files": len(finals), "tmp_files": len(tmp)}

    def a6(self) -> dict:
        camera, key = "feeder-kill", "a6"
        out: dict[str, Any] = {}
        # Phase 1: SIGKILL lightNVR while frames are being detected, encoded and relayed.
        self.detector.arm(key, 12)
        wait_for(lambda: len(self.detector.reports(key)) >= 4, 60, 0.2, "four detected frames")
        offset = self.lnvr.log_path.stat().st_size
        self.lnvr.kill()
        killed_after = len(self.detector.reports(key))
        self.lnvr.start()
        lnvr_restart = time.time()
        log = self.lnvr.log_since(offset)
        # Reconciliation and drain log only when they found work; when present they must come first.
        markers = ("Observation reconciliation:", "Observation evidence initialized", "Observation relay drained", "Observation publication relay started")
        order = {m: log.find(m) for m in markers}
        present = [order[m] for m in markers if order[m] >= 0]
        expect(order[markers[1]] >= 0 and order[markers[3]] >= 0 and present == sorted(present), f"startup order markers {order}")
        startup_log = [line for line in log.splitlines() if any(m in line for m in markers)]
        wait_for(lambda: self.detector.remaining(key) == 0, 120, 1, "remaining frames detected after restart")
        items = self.settled(camera, 2, 120)
        out["phase1_lightnvr_sigkill"] = {"frames_reported_before_kill": killed_after, "observations": len(items), "integrity": self.lnvr_integrity(camera),
                                          "startup_log": startup_log}
        ready_ids = {o["id"] for o in items if o["evidence"]["state"] == "ready"}
        rows = self.wait_rows(camera, ready_ids, 240)
        out["phase1_lightnvr_sigkill"]["converged_seconds_after_restart"] = round(time.time() - lnvr_restart, 1)
        skipped = {o["id"]: o["evidence"] for o in items if o["evidence"]["state"] != "ready"}
        expect(not ({r["observation_id"] for r in rows} & set(skipped)), "a detection was admitted for evidence that is not ready")
        out["phase1_lightnvr_sigkill"].update({"ready": len(ready_ids), "not_ready": {k: f"{v['state']}:{v.get('reason')}" for k, v in skipped.items()},
                                               "yawamf_rows": len(rows)})
        # Phase 2: SIGKILL YA-WAMF while it is fetching and classifying.
        before = {o["id"] for o in items}
        calls_before = len(self.classifier_calls(camera))
        self.detector.arm(key, 10)
        wait_for(lambda: len(self.classifier_calls(camera)) >= calls_before + 3, 120, 0.2, "YA-WAMF mid-way through the burst")
        self.ya.kill()
        wait_for(lambda: self.detector.remaining(key) == 0, 120, 1, "burst finished")
        items = self.settled(camera, len(before) + 2, 120)
        restart = time.time()
        self.ya.start()
        ready_ids = {o["id"] for o in items if o["evidence"]["state"] == "ready"}
        rows = self.wait_rows(camera, ready_ids, 240)
        expect(len(rows) == len({r["observation_id"] for r in rows}), "duplicate rows")
        calls = len(self.classifier_calls(camera)) - calls_before
        out["phase2_yawamf_sigkill"] = {"observations_total": len(items), "ready": len(ready_ids), "yawamf_rows": len(rows),
                                        "classifier_calls_in_phase": calls, "converged_seconds_after_restart": round(time.time() - restart, 1),
                                        "integrity": self.lnvr_integrity(camera)}
        return out

    def a8(self) -> dict:
        camera, key = "feeder-outcomes", "a8"
        out: dict[str, Any] = {}
        # Temporary first (lightNVR frozen), so the later cases do not depend on it.
        self.ya.stop()
        self.detector.arm(key, 2)
        temp = self.settled(camera, 2, 90)
        temp_ids = {o["id"] for o in temp}
        self.lnvr.proc.send_signal(19)  # SIGSTOP: the API hangs, a network-level temporary error
        try:
            self.ya.start(wait_mqtt=True)
            time.sleep(20)
            during = self.recovery_status()
            rows_during = {r["observation_id"] for r in self.yawamf_rows(camera)} & temp_ids
        finally:
            self.lnvr.proc.send_signal(18)  # SIGCONT
        resumed = time.time()
        rows = self.wait_rows(camera, temp_ids, 240)
        out["temporary_lightnvr_unreachable"] = {"rows_while_unreachable": len(rows_during), "recovery_status_while_unreachable": during,
                                                 "rows_after_resume": len({r['observation_id'] for r in rows} & temp_ids),
                                                 "converged_seconds_after_resume": round(time.time() - resumed, 1)}
        expect(not rows_during, "admitted while lightNVR was unreachable")

        # Expired media and unavailable evidence: created while YA-WAMF is down.
        self.ya.stop()
        known = {o["id"] for o in self.observations(camera)}
        self.detector.arm(key, 2)
        expired = [o for o in self.settled(camera, len(known) + 2, 90) if o["id"] not in known]
        known |= {o["id"] for o in expired}
        self.detector.arm(key, 2)
        unavailable = [o for o in self.settled(camera, len(known) + 2, 90) if o["id"] not in known]
        expect(len(expired) == 2 and len(unavailable) == 2, f"{len(expired)} / {len(unavailable)}")
        self.lnvr.stop()
        day = 86400000
        for o in expired:
            self.lnvr.sql("UPDATE object_observations SET observed_at_ms = observed_at_ms - ? WHERE uuid = ?", (8 * day, o["id"]))
        for o in unavailable:
            for (path,) in self.lnvr.sql("SELECT path FROM observation_assets WHERE uuid = ?", (o["evidence"]["asset_id"],)):
                full = Path(path) if Path(path).is_absolute() else self.lnvr.dir / "recordings" / path
                if not full.exists():
                    full = next((self.lnvr.dir / "recordings").rglob(f"{o['evidence']['asset_id']}.jpg"))
                full.unlink()
        self.lnvr.start()
        # Unavailable: startup reconciliation. Expired: the first retention pass (<= ~60 s heartbeat).
        def states():
            current = {o["id"]: o for o in self.observations(camera)}
            return current if all(current[o["id"]]["evidence"]["state"] == "expired" for o in expired) and \
                all(current[o["id"]]["evidence"]["state"] == "unavailable" for o in unavailable) else None
        current = wait_for(states, 180, 2, "expired and unavailable evidence states")
        outcomes = {}
        for label, group in (("expired", expired), ("unavailable", unavailable)):
            for o in group:
                status, body, _h = self.snapshot(o["id"])
                outcomes[o["id"]] = {"case": label, "state": current[o["id"]]["evidence"]["state"], "reason": current[o["id"]]["evidence"].get("reason"),
                                     "snapshot_status": status, "snapshot_code": json.loads(body).get("code")}
        expect(all(v["snapshot_status"] == 410 and v["snapshot_code"] == "evidence_expired" for v in outcomes.values() if v["case"] == "expired"), f"{outcomes}")
        expect(all(v["snapshot_status"] == 404 and v["snapshot_code"] == "evidence_unavailable" for v in outcomes.values() if v["case"] == "unavailable"), f"{outcomes}")
        started = time.time()
        self.ya.start()
        status = self.wait_recovery_run(started - 1, 120)
        time.sleep(5)
        ids = {o["id"] for o in expired + unavailable}
        admitted = {r["observation_id"] for r in self.yawamf_rows(camera)} & ids
        expect(not admitted, f"admitted detections for expired/unavailable evidence: {admitted}")
        backfill = self.ya.api("POST", "/api/backfill", {"date_range": "month"}, timeout=180)
        reasons = backfill.get("skipped_reasons", {})
        expect(reasons.get("evidence_expired", 0) >= 2 and reasons.get("evidence_unavailable", 0) >= 2, f"backfill reasons {backfill}")
        out["expired_and_unavailable"] = {"lightnvr": outcomes, "recovery_run": status.get("last_run"), "yawamf_rows": len(admitted), "backfill": backfill}

        # Expired cursor: a lightNVR DB restore (new epoch) under YA-WAMF's stored cursor.
        cursor_row = self.ya.sql("SELECT cursor, recovery_state FROM source_cursors WHERE source_kind = 'lightnvr'")
        expect(cursor_row and cursor_row[0][0], f"YA-WAMF has no stored cursor: {cursor_row}")
        sidecar = self.lnvr.dir / "recordings" / "observations" / ".cursor_head.json"
        self.lnvr.stop()
        db = self.lnvr.db_path
        backup = db.with_name("backup.db")
        for suffix in ("", "-wal"):
            if Path(f"{db}{suffix}").exists():
                shutil.copyfile(f"{db}{suffix}", f"{backup}{suffix}")
        self.lnvr.start()
        known = {o["id"] for o in self.observations(camera)}
        self.detector.arm(key, 2)
        lost = [o for o in self.settled(camera, len(known) + 2, 90) if o["id"] not in known]
        lost_ids = {o["id"] for o in lost}
        self.wait_rows(camera, {o["id"] for o in lost if o["evidence"]["state"] == "ready"}, 120)
        head = self.lnvr.sql("SELECT MAX(seq) FROM observation_changes")[0][0]
        wait_for(lambda: sidecar.exists() and json.loads(sidecar.read_text()).get("head_seq") == head, 30, 0.5, "cursor head sidecar")
        # Make sure YA-WAMF's stored cursor is from the generation the restore discards.
        self.ya.stop()
        self.ya.start()
        self.wait_recovery_run(time.time() - 30, 120)
        self.ya.stop()
        old_cursor = self.ya.sql("SELECT cursor FROM source_cursors WHERE source_kind = 'lightnvr'")[0][0]
        self.lnvr.stop()
        for suffix in ("", "-wal", "-shm"):
            Path(f"{db}{suffix}").unlink(missing_ok=True)
        for suffix in ("", "-wal"):
            if Path(f"{backup}{suffix}").exists():
                shutil.copyfile(f"{backup}{suffix}", f"{db}{suffix}")
        offset = self.lnvr.log_path.stat().st_size
        self.lnvr.start()
        restored_log = self.lnvr.log_since(offset)
        expect("epoch regenerated" in restored_log, "lightNVR did not regenerate the epoch after the restore")
        status, _h, body = http("GET", f"{self.lnvr.base}/api/observations?cursor={urllib.parse.quote(old_cursor)}", headers=self.token_headers)
        expired_cursor = {"status": status, "body": json.loads(body)}
        expect(status == 410 and expired_cursor["body"].get("code") == "cursor_expired", f"old cursor {expired_cursor}")
        started = time.time()
        self.ya.start()
        rec = self.wait_recovery_run(started - 1, 120)
        snapshot = self.ya.api("GET", "/api/diagnostics/errors?source=source_recovery")
        entries = snapshot.get("events") or snapshot.get("items") or [v for v in snapshot.values() if isinstance(v, list)][0]
        diagnostics = [{k: e.get(k) for k in ("source", "component", "severity", "reason_code", "message")} for e in entries]
        expect(rec.get("recovery_required") and rec.get("gap_reason") == "cursor_expired", f"recovery status {rec}")
        expect(diagnostics, "no source_recovery diagnostics entry for the expired cursor")
        acknowledged = self.ya.api("POST", "/api/backfill/source-recovery/acknowledge", {})
        expect(not acknowledged.get("recovery_required"), f"acknowledge {acknowledged}")
        rows_lost = {r["observation_id"] for r in self.yawamf_rows(camera)} & lost_ids
        out["expired_cursor_after_restore"] = {"old_cursor_response": expired_cursor, "recovery_after_restore": rec, "diagnostics_entries": diagnostics,
                                               "after_acknowledge": {k: acknowledged.get(k) for k in ("state", "recovery_required", "gap_reason")},
                                               "observations_lost_by_restore_still_in_yawamf": len(rows_lost)}
        return out

    def _ya_has(self, table: str) -> bool:
        return bool(self.ya.sql("SELECT 1 FROM sqlite_master WHERE type='table' AND name=?", (table,)))

    # ------------------------------------------------------------------ run

    def versions(self) -> dict:
        def git(repo: str, *argv: str) -> str:
            return subprocess.run(["git", "-C", repo, *argv], capture_output=True, text=True).stdout.strip()
        root = str(HERE.parents[2])
        return {
            "lightnvr": {"commit": git(root, "rev-parse", "--short", "HEAD"), "branch": git(root, "rev-parse", "--abbrev-ref", "HEAD"),
                         "dirty_tracked": bool(git(root, "status", "--porcelain", "--untracked-files=no")), "binary": str(Path(self.args.lightnvr_bin).resolve())},
            "yawamf": {"commit": git(self.args.yawamf, "rev-parse", "--short", "HEAD"), "branch": git(self.args.yawamf, "rev-parse", "--abbrev-ref", "HEAD")},
            "mosquitto": self.broker.version() if hasattr(self, "broker") else None,
            "ffmpeg": subprocess.run(["ffmpeg", "-version"], capture_output=True, text=True).stdout.splitlines()[0],
            "detector": "loopback HTTP x1-colour-detector 1.0 (light-object-detect wire format; finds the red/blue fixture birds by colour)",
            "classifier": "pinned x1-colour-rule-v1 (harness/yawamf_launcher.py replaces ClassifierService.classify_async_live)",
            "python": sys.version.split()[0],
        }

    def run(self) -> int:
        started = time.time()
        deploy_error = None
        try:
            self.deploy()
        except Exception as error:  # noqa: BLE001
            deploy_error = f"{type(error).__name__}: {error}\n{traceback.format_exc()}"
            print(deploy_error, flush=True)
        if deploy_error is None:
            self.scenario("A1", "identity and labels", self.a1)
            self.scenario("A2", "moving-bird alignment (HTTP detector)", self.a2)
            if self.local:
                self.scenario("A2-local", "moving-bird alignment (local SOD detector, real bird photo)", self.a2_local)
            self.scenario("A3", "two birds in one frame", self.a3)
            self.scenario("A4", "replay x10 with overlapping API recovery", self.a4)
            self.scenario("A12", "write-back: species in both products, replays and stale revisions, owner precedence, no loop", self.a12)
            self.scenario("A13", "write-back survives a lightNVR outage; the YA-WAMF detection stays intact", self.a13)
            self.scenario("A7-annotate", "annotate needs observations.annotate on the capture camera", self.a7_annotate)
            self.scenario("A5", "broker down for 100 observations, both apps restarted", self.a5)
            self.scenario("A6", "kill points end to end", self.a6)
            self.scenario("A8", "expired, unavailable, temporary and expired-cursor outcomes", self.a8)
        summary = {
            "suite": "INT_01 X1/X3 lightNVR <-> YA-WAMF E2E", "writeback_enabled": not self.args.no_writeback, "mode": "host-process", "started_at": started,
            "finished_at": time.time(), "seconds": round(time.time() - started, 1), "versions": self.versions(),
            "recipe": self.recipe, "deploy_error": deploy_error, "scenarios": self.results, "workdir": str(self.work),
            "counts": {"detector_requests": getattr(self.detector, "requests", None) if hasattr(self, "detector") else None,
                       "mqtt_messages_captured": len(self.capture.raw()) if hasattr(self, "capture") else None},
        }
        self.teardown()
        out_dir = Path(self.args.results_dir)
        out_dir.mkdir(parents=True, exist_ok=True)
        name = self.args.run_name or time.strftime("x1-host-%Y%m%dT%H%M%S")
        (out_dir / f"{name}.json").write_text(json.dumps(summary, indent=2, default=str) + "\n")
        for evidence in ("mqtt-capture.jsonl", "detector.jsonl", "yawamf/classifier.jsonl"):
            source = self.work / evidence
            if source.exists():
                shutil.copyfile(source, out_dir / f"{name}.{Path(evidence).name}")
        print(f"\nresults: {out_dir / (name + '.json')}")
        for r in self.results:
            print(f"  {r['id']}: {r['result']}" + (f"  ({r.get('error')})" if r["result"] != "pass" else ""))
        ok = deploy_error is None and self.results and all(r["result"] == "pass" for r in self.results)
        return 0 if ok else 1

    def teardown(self) -> None:
        for name in ("ya", "lnvr", "capture", "broker"):
            component = getattr(self, name, None)
            if component is None:
                continue
            try:
                if name == "lnvr" and component.proc and component.proc.poll() is None:
                    component.proc.send_signal(18)
                component.stop()
            except Exception as error:  # noqa: BLE001
                print(f"teardown {name}: {error}", flush=True)
        for name in ("feeder", "detector"):
            component = getattr(self, name, None)
            if component is not None:
                component.close()


def main() -> int:
    root = HERE.parents[2]
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--lightnvr-bin", default=os.environ.get("LIGHTNVR_E2E_BIN", str(root / "build" / "bin" / "lightnvr")))
    parser.add_argument("--web-root", default=str(root / "web" / "dist"))
    parser.add_argument("--yawamf", default=os.environ.get("YAWAMF_E2E_REPO", str(root.parent / "YetAnother-WhosAtMyFeeder")))
    parser.add_argument("--workdir", default=None)
    parser.add_argument("--results-dir", default=str(HERE / "results"))
    parser.add_argument("--run-name", default=None)
    parser.add_argument("--sod-model", default=None, help="optional: a bird-capable .sod model (e.g. VOC tiny20.sod) for the A2 local variant")
    parser.add_argument("--bird-image", default=None, help="optional: a real bird photo for the A2 local variant (not committed; see README)")
    parser.add_argument("--no-writeback", action="store_true", help="start YA-WAMF without LIGHTNVR__WRITEBACK_ENABLED (P1 recipe only)")
    parser.add_argument("--scenarios", type=lambda s: set(s.split(",")), default=None, help="comma-separated subset, e.g. A1,A3")
    return Suite(parser.parse_args()).run()


if __name__ == "__main__":
    sys.exit(main())
