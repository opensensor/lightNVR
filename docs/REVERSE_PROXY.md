# Reverse Proxy & HTTPS

LightNVR's HTTP server does not terminate TLS itself.  For HTTPS — and for any deployment that exposes the dashboard to a network you don't fully trust — run a reverse proxy (nginx, Caddy, Traefik, HAProxy, an ingress controller, …) in front and let it handle certificates.

This doc covers the two things that aren't obvious if you've never put a reverse proxy in front of an NVR:

1. **You have to forward two upstream services**, not one — lightNVR on port 8080 and go2rtc on port 1984.
2. **WebSocket upgrade headers are mandatory** for the `/go2rtc/*` path or MSE live view and WebRTC SDP exchange will fail silently.

## Upstream services

| Path on the reverse proxy | Upstream | Why |
| --- | --- | --- |
| `/` (everything except `/go2rtc/`) | `http://127.0.0.1:8080` | lightNVR's HTTP server. Serves the dashboard, REST API (`/api/*`), HLS segments (`/hls/*`), and lightNVR's internal go2rtc HTTP-only proxy at `/go2rtc/api/streams`, `/go2rtc/api/stream.m3u8`, `/go2rtc/api/hls/*`, `/go2rtc/api/frame.jpeg`. |
| `/go2rtc/*` | `http://127.0.0.1:1984` | go2rtc's own HTTP/WS server. Required because lightNVR's built-in `/go2rtc/*` proxy is HTTP-only (uses libcurl) and cannot handle WebSocket upgrades — and **MSE live view and WebRTC SDP exchange both need WebSocket**. |

> **Why two upstreams instead of one?** lightNVR proxies a curated subset of go2rtc endpoints internally (HLS segments and snapshots) because those benefit from lightNVR's concurrency limiter and auth. Everything else — `/go2rtc/api/ws` for MSE, `/go2rtc/api/webrtc` SDP, and the go2rtc admin UI — needs to reach go2rtc directly. When you're proxying HTTPS, the cleanest fix is to route the whole `/go2rtc/*` prefix to go2rtc and let the frontend (which detects `https:` and switches routing automatically) pick the right path.  See `web/js/utils/settings-utils.js#getGo2rtcBaseUrl` / `#getGo2rtcWebSocketUrl` for the client-side logic.

## The trust setting

LightNVR refuses to honor `X-Forwarded-For` / `X-Real-IP` headers by default — otherwise any client could spoof a source IP for the audit log, login allow-list, and rate limiter.  Tell lightNVR which proxies it can trust by setting `trusted_proxy_cidrs` in `/etc/lightnvr/lightnvr.ini`:

```ini
[web]
trusted_proxy_cidrs = 127.0.0.1/32,::1/128
```

The value is a comma- or newline-separated list of IPv4/IPv6 CIDRs that may legitimately set the forwarded headers.  When the request arrives from one of those addresses, lightNVR uses the leftmost untrusted IP in `X-Forwarded-For` (or `X-Real-IP` if present) as the client address.  If your reverse proxy runs on the same host, `127.0.0.1/32,::1/128` is correct; if it runs in a separate container on a Docker bridge, use that bridge's CIDR (e.g. `172.18.0.0/16`); for Kubernetes, the pod or node CIDR.

## Example: nginx

This config — adapted from a working deployment by @dpw13 — terminates HTTPS, forwards `/` to lightNVR, and forwards `/go2rtc/*` to go2rtc.  WebSocket upgrades are wired up on both locations because lightNVR uses WS for some live-view paths and go2rtc uses WS for MSE + WebRTC.

```nginx
# Redirect HTTP → HTTPS
server {
    listen 80;
    listen [::]:80;
    server_name nvr.example.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name nvr.example.com;

    ssl_certificate     /etc/ssl/certs/nvr.crt;
    ssl_certificate_key /etc/ssl/private/nvr.key;
    ssl_protocols       TLSv1.2 TLSv1.3;

    # HSTS — opt in once you're sure the cert chain is good.
    # add_header Strict-Transport-Security "max-age=63072000; includeSubDomains" always;

    # WebSocket upgrade headers travel through the upstream connection.
    proxy_http_version 1.1;
    proxy_set_header   Upgrade           $http_upgrade;
    proxy_set_header   Connection        "upgrade";
    proxy_set_header   Host              $host;
    proxy_set_header   X-Real-IP         $remote_addr;
    proxy_set_header   X-Forwarded-For   $proxy_add_x_forwarded_for;
    proxy_set_header   X-Forwarded-Proto $scheme;

    # Long-lived streams (MSE WebSockets, HLS pulls) must not time out at
    # nginx's default 60s read timeout.
    proxy_read_timeout 3600s;
    proxy_send_timeout 3600s;

    # Everything except /go2rtc/* → lightNVR
    location / {
        proxy_pass http://127.0.0.1:8080;
    }

    # /go2rtc/* → go2rtc (bypasses lightNVR's HTTP-only internal proxy
    # so MSE WebSockets and WebRTC SDP can be upgraded)
    location /go2rtc/ {
        proxy_pass http://127.0.0.1:1984;
    }
}
```

If your nginx fronts containers, the upstreams will be container names or service IPs instead of `127.0.0.1`.

## Example: Caddy

Caddy handles WebSocket upgrades, HSTS, and Let's Encrypt certificates without any extra knobs.

```caddy
nvr.example.com {
    encode zstd gzip

    # /go2rtc/* must come BEFORE the catch-all
    handle /go2rtc/* {
        reverse_proxy 127.0.0.1:1984 {
            transport http {
                read_timeout 1h
                write_timeout 1h
            }
        }
    }

    handle {
        reverse_proxy 127.0.0.1:8080
    }
}
```

## Example: Traefik

For a Docker/Kubernetes deployment that's already using Traefik, use two routers — one for `/go2rtc/*` and one for everything else — so the path-based split routes WebSocket traffic to the right service.  Compose example:

```yaml
services:
  lightnvr:
    image: ghcr.io/opensensor/lightnvr:latest
    labels:
      # Catch-all → lightNVR's HTTP port
      traefik.enable: "true"
      traefik.http.routers.lightnvr.rule: "Host(`nvr.example.com`)"
      traefik.http.routers.lightnvr.tls: "true"
      traefik.http.routers.lightnvr.tls.certresolver: "letsencrypt"
      traefik.http.services.lightnvr.loadbalancer.server.port: "8080"

      # /go2rtc/* → go2rtc's port on the same container
      # Higher priority wins when both rules match the host.
      traefik.http.routers.go2rtc.rule: "Host(`nvr.example.com`) && PathPrefix(`/go2rtc`)"
      traefik.http.routers.go2rtc.priority: "100"
      traefik.http.routers.go2rtc.tls: "true"
      traefik.http.routers.go2rtc.tls.certresolver: "letsencrypt"
      traefik.http.routers.go2rtc.service: "go2rtc"
      traefik.http.services.go2rtc.loadbalancer.server.port: "1984"
```

Traefik upgrades WebSockets by default; no extra middleware is needed.

## Verification

After starting the proxy, walk through this checklist with the browser dev tools open:

1. **Dashboard loads over `https://`** — confirms the `/` route reaches lightNVR.
2. **Live view shows video** (any backend — WebRTC, HLS, or MSE) — confirms at least the matching subset of go2rtc routing works.
3. **Open the Network tab and reload Live View** — there should be no requests to `:1984` or `:8555`.  Every request should hit your reverse proxy hostname over `https:` / `wss:`.  A request to `:1984` over plain `http://` is a mixed-content bug and the browser will refuse it.
4. **Check `wss://yourhost/go2rtc/api/ws?src=<stream>`** — this is the MSE WebSocket.  Status should be `101 Switching Protocols`.  If it's `404` or hangs, your `/go2rtc/*` route is missing or doesn't upgrade.
5. **Switch a camera to WebRTC** in the live view — the WebRTC SDP exchange is over POST to `/go2rtc/api/webrtc`, so this fails the same way an MSE WebSocket would if `/go2rtc/*` isn't routed.
6. **Open Settings → Users and check the audit log** — recent logins should show your laptop's actual IP, not `127.0.0.1`.  If they show `127.0.0.1`, `trusted_proxy_cidrs` isn't set or doesn't cover the proxy's source address.

## Common pitfalls

- **Forgot `Upgrade` / `Connection` headers.** MSE live view fails to start; the WebSocket request returns 200 (or HTML) instead of 101.  In nginx, this is `proxy_set_header Upgrade $http_upgrade; proxy_set_header Connection "upgrade";` plus `proxy_http_version 1.1;` in the appropriate scope.
- **Routed only `/` to lightNVR and skipped `/go2rtc/*`.** HLS and snapshots work (those go through lightNVR's internal HTTP-only proxy), but anything live (MSE/WebRTC) breaks the moment WebSocket is needed.
- **Mixed `http:` / `https:` content.** The frontend detects `window.location.protocol` and switches its go2rtc URLs to `https://yourhost/go2rtc`.  If you serve the dashboard over `https:` but the browser ends up requesting `http://yourhost:1984/...` directly, your reverse proxy isn't intercepting `/go2rtc/*` — re-check the location/router ordering.
- **`trusted_proxy_cidrs` left empty.** Audit log and rate limiter see every request as coming from the proxy (`127.0.0.1` or the container bridge IP).  Login allow-lists effectively disable themselves because everyone looks local.
- **`proxy_read_timeout` at nginx's 60s default.** A WebSocket sitting idle (e.g., a paused live view, or a sub-stream with low frame rate) gets torn down after a minute.  Bump it to something like an hour for the proxy paths that carry streaming traffic.
- **Reverse proxy on a separate host.** The proxy's source IP, not `127.0.0.1`, needs to be in `trusted_proxy_cidrs`.  In Docker that's the bridge gateway (often `172.17.0.1/32`); in Kubernetes that's the pod or node CIDR.

## Related

- `web/js/utils/settings-utils.js` — frontend logic that decides between direct go2rtc port and reverse-proxy paths based on `window.location.protocol`.
- `src/web/httpd_utils.c` — server-side `X-Forwarded-For` / `X-Real-IP` handling, gated by `trusted_proxy_cidrs`.
- [Configuration Guide](CONFIGURATION.md) — full `[web]` section reference, including `trusted_proxy_cidrs`.
- [Docker Deployment](DOCKER.md) — when running lightNVR + go2rtc as containers behind a host-level proxy.
- [go2rtc Config Override](GO2RTC_CONFIG_OVERRIDE.md) — if you need to change go2rtc's listen address or base path, edit `override.yaml` rather than hand-editing the generated `go2rtc.yaml`.
