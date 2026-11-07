# S1 media and audio authority receipt

Implemented against LightNVR main `84ed0174`, with a supporting maintained-go2rtc
fork commit `ba99c7e` (`codex/media-authority`). This slice makes `audio.listen` and `audio.talk` enforced actions;
`live.view` alone preserves video playback without camera audio or microphone
transmission. No physical cameras or user instances were contacted.

## Enforcement boundary

- LightNVR issues a random opaque lease after camera-scoped signaling
  authorization. Only the opaque lease and local TLS selection reach go2rtc;
  user passwords, cookies, API keys and camera credentials do not.
- The lease binds the canonical source (including managed `_sub` resolution),
  immutable camera UUID and original authenticated request. Renewal evaluates
  current account/session/Basic/key/scoped-token validity and current owner,
  action and camera policy. Client codec selectors and SDP cannot widen it.
- Managed launch's final config pins `api.media_authority_port`, alongside the
  existing loopback-only HTTP/RTSP listeners. One new internal registration,
  `GET /api/internal/media-lease`, accepts actual loopback peers plus opaque
  lease only. It is explicitly excluded from personalized public discovery.
- The fork filters real consumer/producer media before binding tracks for HTTP
  and WebSocket WebRTC, WS MSE/MP4, MPEG-TS HLS and fMP4 HLS. Browser video
  publishing and unsupported WS module commands are rejected. Only granted
  microphone backchannels bind to the selected camera.
- Every established fork peer/transport/HLS session renews every five seconds,
  with a two-second callback deadline. Denial, callback failure or rights change
  closes media within seven seconds, subject to process scheduling. HTTP peers
  renew independently after signaling completes. WS disconnect closes its
  associated WebRTC peers. HTTP HLS session removal stops follow-up segments.
- Both native proxy implementations require the fork's media-policy response
  acknowledgment. Older or independently managed forks without authority
  configuration fail closed for these paths. Independent deployments need the
  compatible fork and `api.media_authority_port` pointing to LightNVR.
- Native `/hls/...` reuses the existing at-most-30-second media authorization
  window. Without `audio.listen`, it remuxes owned MPEG-TS segments to video only
  and rebuilds container metadata. Native writers emit MPEG-TS; legacy native
  fMP4 segments fail closed for video-only users. Authorized audio is unchanged.
  Segments use private/no-store caching. Input/output segment size is capped at
  32 MiB; unsupported, oversized or unremuxable video-only segments fail closed.
- Registry capacity is 256 leases; idle leases expire after 20 seconds. Failed
  signaling cannot consume unbounded slots. Authentication material remains
  only in bounded local process memory; expired slots are reclaimed on issuance.

## RED → GREEN

A controlled RED run used the same real HTTP WebRTC fixture with the original
unfiltered `ExchangeSDP` behavior (passing no media lease to the exchange). The
video-only assertion failed: **21 unauthorized audio packets arrived**. The
policy-enabled implementation passed the same assertion. This was an isolated
local source substitution, restored before the final tests/commit; it was not
an end-to-end checkout of the entire old LightNVR application.

The GREEN fixture exchanges real SDP over HTTP and WebSocket with owned local
Pion peers, sends DTLS/SRTP video/audio and a microphone backchannel, and counts
packets at the browser and camera sink. Representative packet counts before
revocation:

| Authority | HTTP video/listen/talk | WS video/listen/talk |
| --- | --- | --- |
| live.view only | 21 / 0 / 0 | 21 / 0 / 0 |
| + audio.listen | 22 / 22 / 0 | 21 / 21 / 0 |
| + audio.talk | 21 / 0 / 19 | 21 / 0 / 21 |
| + both | 22 / 22 / 22 | 22 / 22 / 22 |

After authority revocation, all eight cases had stopped actual media delivery
by the seven-second check and delivered no further packets during the next
300 ms. This measures packet delivery, not merely closure of signaling.

Additional evidence:

- Instance-wide `audio_disabled` takes precedence over both audio rights on every
  lease refresh and native HLS request, including already-cached audio grants.
  Two additional regression cases failed before this intersection: the lease
  retained both audio rights, and native HLS selected the original A/V segment.
  After the fix, the actual handler remuxes the owned segment; the FFmpeg demuxer
  finds video and no audio. The nine-case suite passes under ASan/UBSan.

- Native real SQLite authorization tests: Basic audio direction independence,
  live-view revocation, source/loopback/opaque-token/idle-expiry binding,
  immutable camera identity on `_sub` name reassignment, disabled accounts,
  session revocation, legacy-key rotation, scoped-token action intersection and
  revocation.
- Native FFmpeg-owned H.264 test pattern plus AAC tone: input contains audio;
  remuxed transport stream contains video and no audio, inspected through the
  real FFmpeg demuxer. No transcoder, physical camera or network source is used
  by the production remux path.
- Fork HLS tests inspect actual MPEG-TS demuxer streams and fMP4 initialization
  tracks/payloads for both listen states; MSE tests inspect actual fMP4 output.
- Actual WS upgrade rejects missing leases, blocks unsupported commands, and
  closes the upgraded connection after authority revocation.
- Real owned managed-go2rtc process tests pass for final listener/config policy
  and two-config merging, using the newly built fork binary.

## Validation

Native Debug configuration enables ASan/UBSan automatically:

```sh
cmake -S . -B build/media-authority -DBUILD_TESTS=ON \
  -DENABLE_LITERT=OFF -DENABLE_SOD=OFF -DENABLE_SENTRY=OFF \
  -DENABLE_MQTT=OFF -DCMAKE_BUILD_TYPE=Debug \
  '-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
cmake --build build/media-authority -j6
ctest --test-dir build/media-authority --output-on-failure \
  -R 'test_media_authority|test_go2rtc_(proxy_query|websocket|proxy_routing|proxy_thread|process_config_generation)$'
GO2RTC_TEST_BINARY="$PWD/build/media-authority/go2rtc" \
  ctest --test-dir build/media-authority --output-on-failure \
  -R 'test_go2rtc_managed_listeners|test_go2rtc_two_config_merge'
```

Focused native tests passed, including all nine policy/remux cases. Managed
fork process checks passed (2/2). Fork validation passed:

```sh
go test -race ./internal/mediaauth ./internal/api/ws ./internal/webrtc \
  ./internal/mp4 ./internal/hls ./pkg/core ./pkg/webrtc -count=1 -timeout=180s
CGO_ENABLED=0 go build -buildvcs=false -o ../build/media-authority/go2rtc .
```

Real concurrent media tests exposed and fixed existing fork races in track
removal, A/V counters, MP4 keyframe gating and async SDP-answer synchronization.
The final seven-package race run passes, including all HTTP/WS packet cases.
CI now runs these fork media packages with the race detector and the native
owned HLS/policy fixture.

A broader `go test ./...` was also attempted. It fails existing expectations in
`internal/ffmpeg`, `pkg/aac`, `pkg/creds`, `pkg/hap/camera` and `pkg/hap/tlv8`.
All five failing packages reproduce unchanged in an isolated archive of the
original fork commit. They are not claimed as passing or fixed by this slice.

## Limits

The seven-second bound applies to future delivery from the managed fork. Bytes
already received or buffered by a client cannot be revoked. Native HLS retains
the separately documented 30-second authorization-cache bound. Tests establish
protocol/media enforcement with owned sources; they do not claim interoperability
with any particular physical camera's microphone/backchannel implementation.
