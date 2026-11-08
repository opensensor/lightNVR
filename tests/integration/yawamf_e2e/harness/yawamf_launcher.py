"""Start the YA-WAMF backend with a pinned, deterministic bird classifier.

Test-only. The YA-WAMF code is the branch under test, unmodified; the only
substitution is ``ClassifierService.classify_async_live``, which this launcher
replaces with a colour rule so the E2E suite needs no downloaded model:

- at least 10% of the crop's pixels are saturated red (R > 140 and R exceeds
  G and B by 60) -> ``Prunella modularis`` (0.93)
- otherwise at least 10% saturated blue (the same rule on B) ->
  ``Erithacus rubecula`` (0.93)
- anything else (a crop that missed the bird) -> ``[]`` (nothing to admit)

So a correctly aligned crop of a red or blue fixture bird is identified, and a
misaligned crop (background) yields no species. Every call is appended to
``$YAWAMF_E2E_CLASSIFIER_LOG`` (JSON lines) so the suite can count classifier
calls and see the colour of each crop.

Run with YA-WAMF's own venv, from any directory:
    <yawamf>/backend/.venv/bin/python yawamf_launcher.py --backend <yawamf>/backend --port 8099
"""
from __future__ import annotations

import argparse
import json
import os
import sys
import time

PINNED_CLASSIFIER = "x1-colour-rule-v1"
RED_SPECIES = "Prunella modularis"
BLUE_SPECIES = "Erithacus rubecula"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--backend", required=True)
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, required=True)
    args = parser.parse_args()

    sys.path.insert(0, args.backend)
    os.chdir(args.backend)

    import numpy as np  # noqa: E402
    from PIL import ImageStat  # noqa: E402

    from app.services import classifier_service as cs  # noqa: E402

    log_path = os.environ.get("YAWAMF_E2E_CLASSIFIER_LOG")

    async def classify_async_live(self, image, camera_name=None, model_id=None, input_context=None, queue_timeout_seconds=None):
        rgb = image.convert("RGB")
        red, green, blue = ImageStat.Stat(rgb).mean
        pixels = np.asarray(rgb).astype(int)
        r, g, b = pixels[..., 0], pixels[..., 1], pixels[..., 2]
        red_fraction = float(((r > 140) & (r > g + 60) & (r > b + 60)).mean())
        blue_fraction = float(((b > 140) & (b > r + 60) & (b > g + 60)).mean())
        if red_fraction >= 0.10:
            results = [{"label": RED_SPECIES, "score": 0.93, "index": 1}]
        elif blue_fraction >= 0.10:
            results = [{"label": BLUE_SPECIES, "score": 0.93, "index": 2}]
        else:
            results = []
        if log_path:
            with open(log_path, "a", encoding="utf-8") as handle:
                handle.write(json.dumps({
                    "t": time.time(), "camera": camera_name, "size": list(rgb.size),
                    "mean_rgb": [round(red, 1), round(green, 1), round(blue, 1)],
                    "red_fraction": round(red_fraction, 3), "blue_fraction": round(blue_fraction, 3),
                    "result": results[0]["label"] if results else None,
                }) + "\n")
        return results

    cs.ClassifierService.classify_async_live = classify_async_live

    import uvicorn  # noqa: E402

    uvicorn.run("app.main:app", host=args.host, port=args.port, log_level="info")


if __name__ == "__main__":
    main()
