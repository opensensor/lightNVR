"""INT_01 X3: P2 classification feedback end to end (A12, A13, A7-annotate).

Mixed into ``run_e2e.Suite``. YA-WAMF runs with ``LIGHTNVR__WRITEBACK_ENABLED=true``
(the recipe's optional P2 step), so each detection it classifies is written
back to lightNVR as ``<writer_namespace>/species`` through
``PUT /api/observations/{id}/classifications/{namespace}/{name}``.

Everything is read from where each product keeps it: YA-WAMF's ``detections``
and ``processing_job_state`` rows, lightNVR's ``observation_classifications``
rows, ``GET /api/observations/{id}``, ``GET /api/detection/results/{stream}``
(the L15 ``observation`` enrichment) and the MQTT capture.
"""
from __future__ import annotations

import json
import time
import urllib.parse
import uuid
from typing import Any

from components import http, wait_for

ANNOTATION_KEY = "yawamf/species"
MESSAGE_NS = uuid.UUID("0bc01005-a431-522b-9124-a114f67d187a")
WRITEBACK_PIPELINE = "lightnvr_writeback"
INTENT_COLUMNS = ("notification_intent", "notification_intent_at", "notification_outcome", "notification_outcome_at")
REQUEST_FIELDS = ("classifier", "model_version", "label", "taxonomy_id", "confidence", "classified_at", "input_asset_id", "input_revision", "kind")


def message_id(source: str, observation: str, revision: int) -> str:
    return str(uuid.uuid5(MESSAGE_NS, f"{source}|{observation}|{revision}"))


class P2Scenarios:
    """A12 / A13 / A7-annotate. Expects the attributes ``run_e2e.Suite`` sets up in ``deploy()``."""

    # The owner's correction: the fixture bird is red (Prunella), the owner says it is a robin.
    CORRECTION = "Erithacus rubecula"
    # A13 keeps lightNVR down past the first retry (30 s after the failed attempt), so two attempts fail.
    OUTAGE_SECONDS = 45

    # ------------------------------------------------------------------ helpers

    def lnvr_annotations(self, observation_id: str) -> list[dict]:
        rows = self.lnvr.sql("SELECT key, kind, label, confidence, annotation_revision, writer_namespace, input_revision, classifier "
                             "FROM observation_classifications WHERE observation_uuid = ? ORDER BY key", (observation_id,))
        keys = ("key", "kind", "label", "confidence", "annotation_revision", "writer_namespace", "input_revision", "classifier")
        return [dict(zip(keys, row)) for row in rows]

    def lnvr_species(self, observation_id: str) -> dict | None:
        rows = [r for r in self.lnvr_annotations(observation_id) if r["key"] == ANNOTATION_KEY]
        return rows[0] if rows else None

    def ya_detection(self, event: str) -> dict:
        cols = ("frigate_event", "display_name", "scientific_name", "common_name", "category_name", "score", "manual_tagged", *INTENT_COLUMNS)
        rows = self.ya.sql(f"SELECT {', '.join(cols)} FROM detections WHERE frigate_event = ?", (event,))
        return dict(zip(cols, rows[0])) if rows else {}

    def writeback_job(self, event: str) -> dict:
        rows = self.ya.sql("SELECT status, attempt_count, retry_after, last_error, revision FROM processing_job_state WHERE pipeline = ? AND event_id = ?",
                           (WRITEBACK_PIPELINE, event))
        return dict(zip(("status", "attempt_count", "retry_after", "last_error", "revision"), rows[0])) if rows else {}

    @staticmethod
    def expected_label(row: dict) -> str:
        """The label W10 writes: the first usable of common, scientific, category, then display name."""
        for name in ("common_name", "scientific_name", "category_name", "display_name"):
            if isinstance(row.get(name), str) and row[name].strip():
                return row[name]
        raise AssertionError(f"detection has no species name: {row}")

    def observation_full(self, observation_id: str, headers: dict | None = None) -> dict:
        return self.lnvr.api("GET", f"/api/observations/{observation_id}", headers=headers or self.token_headers)

    def stored_annotation(self, observation_id: str) -> dict:
        obs = self.observation_full(observation_id)
        for kind in ("owner_confirmed", "automated"):
            for entry in obs.get("classifications", {}).get(kind, []):
                if entry["key"] == ANNOTATION_KEY:
                    return entry
        raise AssertionError(f"{ANNOTATION_KEY} missing from GET /api/observations/{observation_id}: {obs.get('classifications')}")

    def put_annotation(self, observation_id: str, body: dict, token: str | None = None, name: str = "species", namespace: str = "yawamf") -> tuple[int, dict]:
        status, _h, payload = http("PUT", f"{self.lnvr.base}/api/observations/{observation_id}/classifications/{namespace}/{name}", body,
                                   {"Authorization": f"Bearer {token or self.token}"})
        return status, (json.loads(payload) if payload else {})

    def detection_link(self, camera: str, observation_id: str) -> dict:
        """The L15 enrichment on lightNVR's detection results for this observation's detection row."""
        observed_ms = self.lnvr.sql("SELECT observed_at_ms FROM object_observations WHERE uuid = ?", (observation_id,))[0][0]
        second = observed_ms // 1000
        query = urllib.parse.urlencode({"start": second - 5, "end": second + 5})
        body = self.lnvr.api("GET", f"/api/detection/results/{urllib.parse.quote(camera)}?{query}")
        linked = [d for d in body.get("detections", []) if (d.get("observation") or {}).get("id") == observation_id]
        if not linked:
            raise AssertionError(f"no detection result linked to {observation_id}: {json.dumps(body)[:600]}")
        return linked[0]

    def classification_envelopes(self, observation_id: str) -> list[dict]:
        return [e for e in self.capture.envelopes() if e["data"]["observation_id"] == observation_id and e["data"]["change_cause"] == "classification"]

    def wait_species(self, observation_id: str, predicate, timeout: float, what: str) -> dict:
        return wait_for(lambda: (lambda row: row if row and predicate(row) else None)(self.lnvr_species(observation_id)), timeout, 1.0, what)

    def one_bird(self, camera: str, key: str) -> tuple[dict, dict]:
        """Arm one frame, wait for the ready observation and its YA-WAMF row."""
        before = {o["id"] for o in self.observations(camera)}
        self.detector.arm(key, 1)
        items = [o for o in self.settled(camera, len(before) + 1, 90) if o["id"] not in before]
        if len(items) != 1 or items[0]["evidence"]["state"] != "ready":
            raise AssertionError(f"expected one new ready observation on {camera}: {items}")
        obs = items[0]
        row = self.wait_rows(camera, {obs["id"]}, 90)
        return obs, next(r for r in row if r["observation_id"] == obs["id"])

    # ------------------------------------------------------------------ A12

    def a12(self) -> dict:
        """Species in both products; replays and stale revisions keep one annotation; owner precedence; no loop."""
        camera, key = "feeder-wb", "a12"
        out: dict[str, Any] = {}
        obs, row = self.one_bird(camera, key)
        event = row["event"]
        calls_before = len(self.classifier_calls(camera))

        # 1. Species in both products: YA-WAMF's row, then lightNVR's annotation, observation and detection results.
        ya = self.ya_detection(event)
        label = self.expected_label(ya)
        stored = self.wait_species(obs["id"], lambda r: r["kind"] == "automated", 120, "the automated write-back on lightNVR")
        expect_(stored["label"] == label and abs(stored["confidence"] - ya["score"]) < 1e-6 and stored["writer_namespace"] == "yawamf",
                f"lightNVR stored {stored}, YA-WAMF has {ya}")
        expect_(stored["input_revision"] == 2 and stored["annotation_revision"] == 1, f"annotation provenance {stored}")
        full = self.observation_full(obs["id"])
        expect_(full["label"] == "bird" and abs(full["confidence"] - obs["confidence"]) < 1e-9, f"detector fields changed: {full['label']} {full['confidence']}")
        expect_([a["key"] for a in full["classifications"]["automated"]] == [ANNOTATION_KEY] and not full["classifications"]["owner_confirmed"],
                f"GET classifications {full['classifications']}")
        expect_("writer_user_id" not in json.dumps(full), "writer_user_id leaked")
        link = self.detection_link(camera, obs["id"])
        expect_(link["label"] == "bird" and link["observation"]["classification"]["label"] == label and link["observation"]["classification"]["kind"] == "automated"
                and link["observation"]["owner_confirmed"] is False, f"detection results enrichment {link}")
        job = self.writeback_job(event)
        expect_(job.get("status") == "succeeded", f"write-back job {job}")
        out["species_in_both"] = {"observation_id": obs["id"], "yawamf_event": event, "yawamf": {k: ya[k] for k in ("display_name", "scientific_name", "common_name", "score", "manual_tagged")},
                                  "lightnvr_row": stored, "lightnvr_get_classifications": full["classifications"],
                                  "lightnvr_detection_result": {k: link.get(k) for k in ("label", "confidence", "observation")}, "writeback_job": job}

        # 2. The classification change reaches the broker (revision 3, deterministic ID) and starts no second cycle in YA-WAMF.
        wait_for(lambda: self.classification_envelopes(obs["id"]), 60, 1.0, "the classification change on the broker")
        env = self.classification_envelopes(obs["id"])[0]
        expect_(env["id"] == message_id(self.source_id, obs["id"], 3) and env["data"]["revision"] == 3 and env["data"]["label"] == "bird",
                f"classification envelope {env}")
        time.sleep(10)  # YA-WAMF has received and handled it by now
        after = self.ya_detection(event)
        rows = [r for r in self.yawamf_rows(camera) if r["observation_id"] == obs["id"]]
        expect_(len(rows) == 1, f"{len(rows)} YA-WAMF rows after the classification change")
        expect_(len(self.classifier_calls(camera)) == calls_before, f"classifier re-ran on the classification change: {len(self.classifier_calls(camera)) - calls_before}")
        expect_(all(after[c] == ya[c] for c in INTENT_COLUMNS), f"notification intent changed: {ya} -> {after}")
        expect_(self.writeback_job(event).get("revision") == job.get("revision"), "the classification change queued another write-back")
        out["no_feedback_loop"] = {"classification_message_id": env["id"], "observation_revision": env["data"]["revision"],
                                   "classifier_calls_for_observation": calls_before, "intent": {c: after[c] for c in INTENT_COLUMNS},
                                   "writeback_job_revision": job.get("revision")}

        # 3. Replays and stale revisions: one annotation per (observation, key), conflicts explicit, detector output intact.
        replay = [(t, p) for t, p in self.capture.raw() if json.loads(p)["data"]["observation_id"] == obs["id"]]
        for _ in range(10):
            for topic, payload in replay:
                self.broker.publish(topic, payload)
        current = self.stored_annotation(obs["id"])
        same = {k: current[k] for k in REQUEST_FIELDS} | {"expected_revision": current["annotation_revision"]}
        retry_statuses = [self.put_annotation(obs["id"], same) for _ in range(5)]
        expect_(all(s == 200 and b.get("changed") is False for s, b in retry_statuses), f"identical retries {retry_statuses[:2]}")
        stale = dict(same, label="Turdus merula", confidence=0.51, expected_revision=0)
        stale_status, stale_body = self.put_annotation(obs["id"], stale)
        expect_(stale_status == 409 and stale_body.get("code") == "revision_conflict" and stale_body.get("current", {}).get("label") == label,
                f"stale revision {stale_status} {stale_body}")
        ahead_status, ahead_body = self.put_annotation(obs["id"], dict(stale, expected_revision=current["annotation_revision"] + 5))
        expect_(ahead_status == 409 and ahead_body.get("code") == "revision_conflict", f"future revision {ahead_status} {ahead_body}")
        time.sleep(10)
        anns = [a for a in self.lnvr_annotations(obs["id"]) if a["key"] == ANNOTATION_KEY]
        expect_(len(anns) == 1 and anns[0]["annotation_revision"] == 1 and anns[0]["label"] == label, f"annotations after replay {anns}")
        expect_(self.lnvr.sql("SELECT revision FROM object_observations WHERE uuid = ?", (obs["id"],))[0][0] == 3, "observation revision moved on a no-op or a conflict")
        expect_(len(self.classification_envelopes(obs["id"])) >= 1 and {e["id"] for e in self.classification_envelopes(obs["id"])} == {env["id"]},
                "a no-op or a conflict published a new revision")
        expect_(len([r for r in self.yawamf_rows(camera) if r["observation_id"] == obs["id"]]) == 1 and len(self.classifier_calls(camera)) == calls_before,
                "replay re-created or re-classified the detection")
        out["replays_and_stale_revisions"] = {"mqtt_messages_replayed": len(replay) * 10, "identical_put_retries": [s for s, _b in retry_statuses],
                                              "stale_revision": {"status": stale_status, "code": stale_body.get("code"), "current_label": stale_body.get("current", {}).get("label")},
                                              "future_revision": {"status": ahead_status, "code": ahead_body.get("code")},
                                              "annotations_after": anns, "observation_revision_after": 3}

        # 4. Owner correction in YA-WAMF -> owner_confirmed on lightNVR.
        corrected = self.ya.api("PATCH", f"/api/events/{urllib.parse.quote(event, safe='')}", {"display_name": self.CORRECTION}, timeout=120)
        ya_owner = self.ya_detection(event)
        owner_label = self.expected_label(ya_owner)
        expect_(ya_owner["manual_tagged"] in (1, True) and owner_label != label, f"YA-WAMF correction {corrected} -> {ya_owner}")
        owner = self.wait_species(obs["id"], lambda r: r["kind"] == "owner_confirmed", 120, "the owner correction on lightNVR")
        expect_(owner["label"] == owner_label and owner["confidence"] == 1.0 and owner["annotation_revision"] == 2, f"owner row {owner}")
        link = self.detection_link(camera, obs["id"])
        expect_(link["observation"]["owner_confirmed"] is True and link["observation"]["classification"]["kind"] == "owner_confirmed"
                and link["observation"]["classification"]["label"] == owner_label and link["label"] == "bird", f"enrichment after the correction {link}")
        full = self.observation_full(obs["id"])
        expect_([a["key"] for a in full["classifications"]["owner_confirmed"]] == [ANNOTATION_KEY] and not full["classifications"]["automated"],
                f"classifications after the correction {full['classifications']}")
        wait_for(lambda: any(e["data"]["revision"] == 4 for e in self.classification_envelopes(obs["id"])), 60, 1.0, "the owner change on the broker")
        out["owner_correction"] = {"patch_response": {k: corrected.get(k) for k in ("status", "old_species", "new_species", "manual_tagged")},
                                   "yawamf": {k: ya_owner[k] for k in ("display_name", "scientific_name", "common_name", "manual_tagged")},
                                   "lightnvr_row": owner, "lightnvr_detection_result": link["observation"]}

        # 5. A later automated result does not override it.
        #    a) on lightNVR: an automated PUT from the same writer is refused;
        current = self.stored_annotation(obs["id"])
        auto = {k: current[k] for k in REQUEST_FIELDS} | {"kind": "automated", "label": label, "confidence": 0.97, "model_version": "x1-colour-rule-v1",
                                                            "classified_at": time.strftime("%Y-%m-%dT%H:%M:%S.000Z", time.gmtime()),
                                                            "expected_revision": current["annotation_revision"]}
        locked_status, locked_body = self.put_annotation(obs["id"], auto)
        expect_(locked_status == 409 and locked_body.get("code") == "owner_decision_locked", f"automated over owner: {locked_status} {locked_body}")
        #    b) in YA-WAMF: the detection reverts to an automated result (as a later automated reclassification would
        #       leave it) and its write-back job runs again. The worker reads lightNVR first and drops the write.
        self.ya.sql("UPDATE detections SET manual_tagged = 0, display_name = ?, scientific_name = ?, common_name = ?, score = 0.97 WHERE frigate_event = ?",
                          (label, ya["scientific_name"], ya["common_name"], event))
        self.ya.sql("UPDATE processing_job_state SET status = 'queued', retry_after = NULL, last_error = NULL, attempt_count = 0, "
                          "revision = revision + 1, updated_at = CURRENT_TIMESTAMP WHERE pipeline = ? AND event_id = ?", (WRITEBACK_PIPELINE, event))
        job = wait_for(lambda: (lambda j: j if j.get("status") not in ("queued", "retryable") else None)(self.writeback_job(event)), 90, 1.0,
                       "the automated write-back attempt (worker poll is 60 s)")
        expect_(job["status"] == "terminal" and "owner_decision_locked" in (job.get("last_error") or ""), f"automated attempt over owner decision: {job}")
        held = self.lnvr_species(obs["id"])
        expect_(held["kind"] == "owner_confirmed" and held["label"] == owner_label and held["annotation_revision"] == 2, f"owner decision overridden: {held}")
        # Put YA-WAMF's owner decision back, so later scenarios see the real state.
        self.ya.sql("UPDATE detections SET manual_tagged = 1, display_name = ?, scientific_name = ?, common_name = ?, score = ? WHERE frigate_event = ?",
                          (ya_owner["display_name"], ya_owner["scientific_name"], ya_owner["common_name"], ya_owner["score"], event))
        expect_(len(self.classifier_calls(camera)) == calls_before, "the owner path ran the classifier")
        final = self.ya_detection(event)
        expect_(all(final[c] == ya[c] for c in INTENT_COLUMNS), f"notification intent changed by the correction or write-back: {ya} -> {final}")
        out["automated_after_owner"] = {"lightnvr_put": {"status": locked_status, "code": locked_body.get("code"), "current_kind": locked_body.get("current", {}).get("kind")},
                                        "yawamf_writeback_job": job, "lightnvr_row_after": held}
        out["totals"] = {"classifier_calls_for_observation": calls_before, "yawamf_rows": 1, "intent": {c: final[c] for c in INTENT_COLUMNS},
                         "classification_revisions_published": sorted({e["data"]["revision"] for e in self.classification_envelopes(obs["id"])})}
        self.a12_obs = obs
        return out

    # ------------------------------------------------------------------ A13

    def a13(self) -> dict:
        """A lightNVR outage during write-back leaves the YA-WAMF detection intact; write-back completes after it returns."""
        camera, key = "feeder-wb-outage", "a13"
        out: dict[str, Any] = {}
        obs, row = self.one_bird(camera, key)
        event = row["event"]
        auto = self.wait_species(obs["id"], lambda r: r["kind"] == "automated", 120, "the automated write-back before the outage")
        calls_before = len(self.classifier_calls(camera))
        before = self.ya_detection(event)

        self.lnvr.stop()
        down = time.time()
        try:
            corrected = self.ya.api("PATCH", f"/api/events/{urllib.parse.quote(event, safe='')}", {"display_name": self.CORRECTION}, timeout=120)
            during = self.ya_detection(event)
            owner_label = self.expected_label(during)
            expect_(during["manual_tagged"] in (1, True) and owner_label != auto["label"], f"the correction was not saved while lightNVR was down: {during}")
            job = wait_for(lambda: (lambda j: j if j.get("status") == "retryable" else None)(self.writeback_job(event)), 60, 1.0,
                           "a failed write-back attempt while lightNVR is down")
            expect_((job.get("last_error") or "").startswith("retry:source_unreachable"), f"outage job {job}")
            # The YA-WAMF result stays usable: the event API serves it, and health counts it as waiting.
            listed = self.ya.api("GET", "/api/events?" + urllib.parse.urlencode({"event_id": event}))
            listed = listed if isinstance(listed, list) else listed.get("events", listed.get("items", []))
            expect_(len(listed) == 1, f"GET /api/events?event_id= during the outage: {listed}")
            api_row = listed[0]
            health = self.ya.api("GET", "/health").get("classification_writeback", {})
            rows = [r for r in self.yawamf_rows(camera) if r["observation_id"] == obs["id"]]
            expect_(len(rows) == 1, f"{len(rows)} rows during the outage")
            # Keep lightNVR down a while longer: the result stays put and the job keeps waiting.
            time.sleep(self.OUTAGE_SECONDS)
            still = self.ya_detection(event)
            expect_(still == during, f"the YA-WAMF detection changed during the outage: {during} -> {still}")
            job_late = self.writeback_job(event)
            expect_(job_late.get("status") == "retryable", f"job gave up during the outage: {job_late}")
            outage_seconds = round(time.time() - down, 1)
        finally:
            self.lnvr.start()
        up = time.time()
        owner = self.wait_species(obs["id"], lambda r: r["kind"] == "owner_confirmed", 240, "the write-back after lightNVR returned")
        converged = round(time.time() - up, 1)
        expect_(owner["label"] == owner_label and owner["annotation_revision"] == 2, f"after the outage {owner}")
        final_job = self.writeback_job(event)
        expect_(final_job.get("status") == "succeeded", f"job after the outage {final_job}")
        link = self.detection_link(camera, obs["id"])
        expect_(link["observation"]["owner_confirmed"] is True and link["observation"]["classification"]["label"] == owner_label, f"enrichment {link}")
        time.sleep(10)
        after = self.ya_detection(event)
        expect_(len([r for r in self.yawamf_rows(camera) if r["observation_id"] == obs["id"]]) == 1, "duplicate rows after the outage")
        expect_(len(self.classifier_calls(camera)) == calls_before, "the outage or the retry ran the classifier")
        expect_(all(after[c] == before[c] for c in INTENT_COLUMNS), f"notification intent changed: {before} -> {after}")
        expect_(after["manual_tagged"] in (1, True) and self.expected_label(after) == owner_label, f"YA-WAMF detection changed: {after}")
        out.update({"observation_id": obs["id"], "yawamf_event": event, "automated_before_outage": auto,
                    "during_outage": {"patch_response": {k: corrected.get(k) for k in ("status", "new_species", "manual_tagged")},
                                      "yawamf_detection": {k: during[k] for k in ("display_name", "scientific_name", "manual_tagged")},
                                      "event_api_species": {k: api_row.get(k) for k in ("display_name", "scientific_name", "manual_tagged")},
                                      "writeback_job": job, "writeback_job_end_of_outage": job_late, "health_classification_writeback": health,
                                      "seconds_down": outage_seconds},
                    "after_restart": {"lightnvr_row": owner, "writeback_job": final_job, "converged_seconds_after_restart": converged,
                                      "lightnvr_detection_result": link["observation"]},
                    "classifier_calls_for_observation": calls_before, "intent": {c: after[c] for c in INTENT_COLUMNS}})
        return out

    # ------------------------------------------------------------------ A7-annotate

    def a7_annotate(self) -> dict:
        """A token without observations.annotate gets 403 on PUT and writes nothing; with it, the PUT succeeds."""
        obs = getattr(self, "a12_obs", None) or next(iter(self.ready("feeder-wb")), None)
        expect_(obs is not None, "A7-annotate needs a ready observation on feeder-wb (run A12 first)")
        scope = {"type": "selector", "selector": {"version": 1, "expression": {"op": "camera_uuid", "values": list(self.cameras.values())}}}
        no_annotate = self.lnvr.api("POST", f"/api/authorization/users/{self.user_id}/tokens", {
            "description": "X3 read-only probe", "expires_at": int(time.time()) + 86400, "actions": ["live.view", "snapshot.create"], "scope": scope})["secret"]
        # A token's selector must equal one of its user's grants, so the other-camera case is a second
        # integration user holding the same role on feeder-east only.
        other_scope = {"type": "selector", "selector": {"version": 1, "expression": {"op": "camera_uuid", "values": [self.cameras["feeder-east"]]}}}
        other = self.lnvr.api("POST", "/api/auth/users", {"username": "yawamf-east", "password": "X3-Yawamf-East-Integration-1!", "role": 2, "is_active": True})
        other_id = other.get("id") or other.get("user", {}).get("id")
        policy = self.lnvr.api("GET", f"/api/authorization/users/{other_id}")
        self.lnvr.api("PUT", f"/api/authorization/users/{other_id}", {"mode": "policy", "expected_policy_version": policy["policy_version"],
                                                                     "grants": [{"role_uuid": self.role_uuid, "scope": other_scope}]})
        other_camera = self.lnvr.api("POST", f"/api/authorization/users/{other_id}/tokens", {
            "description": "X3 other-camera probe", "expires_at": int(time.time()) + 86400,
            "actions": ["live.view", "snapshot.create", "observations.annotate"], "scope": other_scope})["secret"]
        other_caps = self.lnvr.api("GET", "/api/integrations/capabilities", headers={"Authorization": f"Bearer {other_camera}"})
        other_ns = other_caps["principal"]["writer_namespace"]
        before_rows = self.lnvr_annotations(obs["id"])
        expect_(self.lnvr_species(obs["id"]) is not None, f"no {ANNOTATION_KEY} written back for {obs['id']}: {before_rows}")
        current = self.stored_annotation(obs["id"])
        before_rev = self.lnvr.sql("SELECT revision FROM object_observations WHERE uuid = ?", (obs["id"],))[0][0]
        body = {k: current[k] for k in REQUEST_FIELDS} | {"label": "Parus major", "expected_revision": current["annotation_revision"]}
        probe = {"classifier": "x3-probe", "model_version": None, "label": "Parus major", "taxonomy_id": None, "confidence": 0.5,
                 "classified_at": time.strftime("%Y-%m-%dT%H:%M:%S.000Z", time.gmtime()), "input_asset_id": obs["evidence"]["asset_id"],
                 "input_revision": 2, "kind": "automated", "expected_revision": 0}
        denied = self.put_annotation(obs["id"], body, token=no_annotate)
        denied_new = self.put_annotation(obs["id"], probe, token=no_annotate, name="x3probe")
        hidden = self.put_annotation(obs["id"], probe, token=other_camera, name="x3probe", namespace=other_ns)
        expect_(denied[0] == 403 and denied[1].get("code") == "forbidden", f"no-annotate token on an existing key: {denied}")
        expect_(denied_new[0] == 403 and denied_new[1].get("code") == "forbidden", f"no-annotate token on a new key: {denied_new}")
        expect_(hidden[0] == 404 and hidden[1].get("code") == "observation_not_found", f"token scoped to another camera: {hidden}")
        expect_(self.lnvr_annotations(obs["id"]) == before_rows, "a refused PUT wrote a row")
        expect_(self.lnvr.sql("SELECT revision FROM object_observations WHERE uuid = ?", (obs["id"],))[0][0] == before_rev, "a refused PUT bumped the revision")
        allowed = self.put_annotation(obs["id"], probe, name="x3probe")
        expect_(allowed[0] == 201 and allowed[1].get("changed") is True and allowed[1]["annotation"]["key"] == "yawamf/x3probe", f"annotate token: {allowed}")
        again = self.put_annotation(obs["id"], probe, name="x3probe")
        expect_(again[0] == 200 and again[1].get("changed") is False, f"annotate token retry: {again}")
        audit = self.lnvr.sql("SELECT outcome, details_json FROM audit_events WHERE action = 'observations.annotate' ORDER BY id DESC LIMIT 6")
        results = []
        for outcome, details in audit:
            try:
                results.append({"outcome": outcome, "result": json.loads(details or "{}").get("context", {}).get("result")})
            except ValueError:
                results.append({"outcome": outcome})
        species = self.lnvr_species(obs["id"])
        expect_(species["kind"] == "owner_confirmed", f"the probe touched {ANNOTATION_KEY}: {species}")
        return {"observation_id": obs["id"], "without_annotate": {"existing_key": denied, "new_key": denied_new},
                "annotate_other_camera_scope": {"writer_namespace": other_ns, "response": hidden}, "with_annotate": {"create": {"status": allowed[0], "changed": allowed[1].get("changed")},
                                                                          "retry": {"status": again[0], "changed": again[1].get("changed")}},
                "rows_unchanged_by_refusals": True, "audit_latest": results}


def expect_(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)
