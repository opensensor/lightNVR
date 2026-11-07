# Media protocol dispositions

These eight registered routes are intentionally classified `media-protocol` in
`coverage.json`, separate from the personalized JSON OpenAPI document. Their
payloads include streaming transports and fork-owned diagnostic data; they are
not empty JSON envelopes. Browser traffic uses the LightNVR origin. Managed
upstream HTTP/RTSP is loopback-only and must use the compatible maintained fork.
See [deployment requirements](../../CONFIGURATION.md#media-authority-deployment-requirements).

| Registration | Request and authority | Successful payload |
| --- | --- | --- |
| `GET /hls/#/#` | `/hls/{camera-name}/{filename}`, `live.view` on the camera; one basename, no slash, percent escape or `..` | 200 HLS playlist (`application/vnd.apple.mpegurl`) or MPEG-TS segment (`video/mp2t`). While starting, a missing playlist is an empty valid playlist; missing segments are 404. Without `audio.listen`, native MPEG-TS is remuxed to video only; unsupported or oversized video-only segments are 403. |
| `GET /go2rtc/api/ws` | One `src` camera and WebSocket upgrade; `live.view`; per-direction audio lease | 101 upgraded transport. JSON commands negotiate WebRTC (`webrtc`, `webrtc/offer`, `webrtc/candidate`) or MSE/MP4 (`mse`, `mp4`); subsequent frames are SDP/candidates or fMP4 bytes. Other modules and publishing commands are denied. |
| `POST /go2rtc/api/webrtc` | One `src` camera, SDP offer body; `live.view`; per-direction audio lease | 201 SDP answer for `application/sdp`, as used by the browser. Legacy raw SDP, JSON offer/answer, and form/base64 signaling variants return 200. All variants negotiate only authorized tracks; actual media travels over ICE/DTLS/SRTP. |
| `GET /go2rtc/api/stream.m3u8` | One `src` camera, optional `mp4`, `video`, `audio` selectors; `live.view` and audio lease | 200 HLS playlist. Codec selectors cannot widen audio authority. Session identifiers returned in the playlist are bound to the originating credential fingerprint and camera. |
| `GET /go2rtc/api/hls/*` | Only `playlist.m3u8`, `init.mp4`, `segment.ts`, `segment.m4s`; `id` required, optional `n`; originating credential and camera authority | 200 playlist, MP4 initialization, MPEG-TS or fragmented MP4 segment. This wildcard registration is not arbitrary filesystem access or an unrestricted go2rtc proxy. |
| `GET /go2rtc/api/frame.jpeg` | One `src` camera, optional `cache`, `t`, `width`, `height`, `w`, `h`, `rotate`; `snapshot.create` | 200 JPEG image. This route creates a live snapshot; it does not grant live audio. |
| `GET /go2rtc/api/streams` | Global `system.admin`; optional single `src` | Fork-owned JSON diagnostic data keyed by stream, with producer/consumer codec and connection details. This administrative response can contain configured source information and is not a viewer inventory contract. |
| `POST /go2rtc/api/reload` | Global `system.admin`; no query parameters | Fork-owned reload response. The managed fork currently has no reload handler, so 404 is expected; this compatibility registration is not a promise of a supported reload operation. |

LightNVR rejects unsupported routes (404), methods (405), duplicate or unknown
query keys and malformed encodings (400). WebSocket requests without an upgrade
are 400. Authentication or camera/session authority failures are 401/403;
missing cameras are 404. Capacity or unavailable authority can produce 503;
unavailable or incompatible upstream services produce 502. Upstream error bodies
can be text, so callers must not assume all error payloads are JSON.

Audio permissions are intersected with instance-wide `audio_disabled` and the
current authenticated principal's camera/action grants. The fork independently
renews leases every five seconds with a two-second callback timeout. Any rights
change or renewal failure closes the associated media; native HLS instead has
its separately documented 30-second media-authorization cache. Already-delivered
or buffered media cannot be revoked.

## Evidence and limits

[Media authority evidence](media-authority-evidence.md) records actual HTTP and
WebSocket SDP, DTLS/SRTP video/audio/backchannel packets, MSE fMP4 payloads,
MPEG-TS/fMP4 HLS demuxing, and established-media revocation. Native
`test_go2rtc_proxy_query`, `test_go2rtc_proxy_routing`, `test_go2rtc_proxy_thread`
and `test_go2rtc_websocket` cover strict path/query forwarding, HLS session
credential binding, policy acknowledgments, and upgrades. Native
`test_media_authority` demuxes owned H.264/AAC media through the real native HLS
handler, including instance audio policy overriding a cached grant.

These fixtures use owned local peers and generated media. They do not establish
physical camera interoperability, multicast discovery, vendor backchannel
support, or remote TURN-provider availability. No test contacts a user camera.
