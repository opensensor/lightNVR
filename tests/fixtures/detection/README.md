# Sampled detection frames

`keyframes.h264` and `keyframes.h265` contain three closed GOPs at 5 fps,
64×64: two seconds each of black, white, and red. Both use two B-frames, so
a decoder may buffer the sampled keyframe until explicitly drained. The test
skips intervening packets, checks every sample's RGB color, and uses a
multi-threaded decoder to cover frame-threading delay too.

Regenerate with FFmpeg built with libx264 and libx265:

```sh
for codec in 264 265; do
  params='scenecut=0:open-gop=0:keyint=10:min-keyint=10'
  if [ "$codec" = 265 ]; then
    params="log-level=error:pools=1:$params"
  fi
  ffmpeg -hide_banner -loglevel error -y \
    -f lavfi -i 'color=black:s=64x64:r=5:d=2' \
    -f lavfi -i 'color=white:s=64x64:r=5:d=2' \
    -f lavfi -i 'color=red:s=64x64:r=5:d=2' \
    -filter_complex '[0:v][1:v][2:v]concat=n=3:v=1:a=0[out]' \
    -map '[out]' -c:v "libx$codec" -g 10 -bf 2 \
    "-x$codec-params" "$params" \
    "tests/fixtures/detection/keyframes.h$codec"
done
```

`keyframes.mjpeg` covers the immediate-output, intra-only case with one frame
per color. Regenerate it with:

```sh
ffmpeg -hide_banner -loglevel error -y \
  -f lavfi -i 'color=black:s=64x64:r=1:d=1' \
  -f lavfi -i 'color=white:s=64x64:r=1:d=1' \
  -f lavfi -i 'color=red:s=64x64:r=1:d=1' \
  -filter_complex '[0:v][1:v][2:v]concat=n=3:v=1:a=0[out]' \
  -map '[out]' -c:v mjpeg -pix_fmt yuvj420p -threads 1 \
  tests/fixtures/detection/keyframes.mjpeg
```

These synthetic fixtures need no camera, network, or encoder at test runtime.
