# Reverse Proxy & HTTPS

Keep the managed go2rtc API bound to `127.0.0.1:1984`. Do not publish port 1984 or route browser requests directly to it. All browser media URLs use the dashboard origin.

LightNVR's authenticated proxy serves MSE WebSockets, WebRTC signaling, HLS and snapshots, with per-camera permissions. All modes work directly on the dashboard port (normally 8080). An HTTPS gateway forwards every path to LightNVR and preserves WebSocket upgrades:

| Public path | Upstream |
| --- | --- |
| `/go2rtc/api/ws` | LightNVR port 8080 (authorized WebSocket upgrade) |
| `/go2rtc/api/webrtc` | LightNVR port 8080 (authorized HTTP signaling) |
| All other paths | LightNVR port 8080 |

The public media prefix is always `/go2rtc`. LightNVR translates it to the configured go2rtc API port and `api.base_path`, including custom or empty base paths. Keep LightNVR authentication enabled. A dashboard session cookie also authenticates the WebSocket handshake; the camera's live-view permission is checked before connecting upstream. Cross-origin WebSocket requests are rejected, so preserve the browser's `Host` header, including any non-default port.

## HTTPS without a reverse proxy

LightNVR can terminate TLS itself, which is useful on a LAN-only install or an embedded device without room for a gateway. It needs a build configured with `-DENABLE_SSL=ON` (see [Building](BUILD.md#https-support)) and a PEM certificate and key:

```ini
[web]
port = 8443
ssl_enabled = true
ssl_cert_path = /etc/lightnvr/tls/cert.pem
ssl_key_path = /etc/lightnvr/tls/key.pem
```

Container deployments can set `LIGHTNVR_WEB_SSL_ENABLED`, `LIGHTNVR_WEB_SSL_CERT_PATH` and `LIGHTNVR_WEB_SSL_KEY_PATH` instead.

- The port serves HTTPS only; plain `http://` requests to it are refused. There is no HTTP-to-HTTPS redirect listener.
- TLS 1.2 and 1.3 are accepted, with mbedTLS's default cipher suites. Client certificates are not requested.
- Session and trusted-device cookies get the `Secure` attribute while HTTPS is on.
- MSE WebSockets (`wss://`), WebRTC signaling, HLS, snapshots and downloads all use the same port, as with HTTP. WebRTC media still flows directly between the browser and go2rtc.
- If the certificate or key cannot be loaded, the web server does not start and the log names the file and the reason. LightNVR never falls back to HTTP.
- The certificate is read at start-up. Restart LightNVR after renewing it.
- Automatic certificates (ACME / Let's Encrypt) are not built in; use a reverse proxy for that, or renew with an external client and restart.

A self-signed certificate for testing:

```bash
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes -days 825 \
  -subj "/CN=nvr.local" -addext "subjectAltName=DNS:nvr.local,IP:192.168.1.10" \
  -keyout /etc/lightnvr/tls/key.pem -out /etc/lightnvr/tls/cert.pem
```

Make sure the user LightNVR runs as can read the key, and nobody else can (`chmod 600`).

## Network placement

The HTTPS gateway only needs network access to LightNVR's web port. It can run on the host, in a separate container on the same Docker network, or in the same network namespace. The examples use `127.0.0.1:8080`; use `lightnvr:8080` for a separate gateway container on the same Docker network.

Port 1984 stays private inside the recorder's network namespace. WebRTC media still needs reachable ICE candidates (normally port 8555 TCP/UDP).

## Trusted proxy addresses

Configure LightNVR to trust forwarded client addresses only from the proxy:

```ini
[web]
trusted_proxy_cidrs = 127.0.0.1/32,::1/128
```

Use the actual proxy address range for other network arrangements. This setting controls forwarded address handling; it does not authenticate media requests.

## nginx

Install your TLS certificate and use this configuration. Place `map` in the `http` context.

```nginx
map $http_upgrade $connection_upgrade {
    default upgrade;
    ''      close;
}

server {
    listen 80;
    server_name nvr.example.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name nvr.example.com;
    ssl_certificate     /etc/ssl/certs/nvr.crt;
    ssl_certificate_key /etc/ssl/private/nvr.key;
    ssl_protocols TLSv1.2 TLSv1.3;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection $connection_upgrade;
        proxy_set_header Host $http_host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_read_timeout 3600s;
        proxy_send_timeout 3600s;
    }
}
```

## Caddy

```caddy
nvr.example.com {
    encode zstd gzip
    reverse_proxy 127.0.0.1:8080
}
```

Caddy preserves the Host header and handles WebSocket upgrades automatically.

## Traefik

This file-provider configuration assumes a TLS `websecure` entry point and a `letsencrypt` certificate resolver are already configured.

```yaml
http:
  routers:
    nvr:
      rule: "Host(`nvr.example.com`)"
      entryPoints: [websecure]
      service: lightnvr
      tls:
        certResolver: letsencrypt
  services:
    lightnvr:
      loadBalancer:
        passHostHeader: true
        servers:
          - url: http://127.0.0.1:8080
```

## Verify after upgrading

1. From another host, confirm TCP port 1984 is unreachable. Inside the recorder's network namespace, confirm it listens on loopback only.
2. Without a LightNVR login, media requests must return `401` or `403`, including WebSocket upgrades and WebRTC POSTs.
3. With a login permitted to view the camera, confirm MSE upgrades to `101`, receives media, and works with several cameras open. Verify HLS grid playback and its subsequent segment requests as well.
4. With a camera-restricted login, confirm other cameras and their substreams are denied. Browser requests must stay on the dashboard origin even with a custom go2rtc port or base path.
5. Confirm `/go2rtc/api/config`, `/go2rtc/api/restart`, and `/go2rtc/api/exit` return `404` through LightNVR. Check that audit logs show the correct client address.

## Existing deployments

Remove any old gateway route to go2rtc, including a dedicated `/go2rtc/api/ws` route, and any port-1984 mapping. Route them through LightNVR instead. Review `override.yaml`: an explicit `api.listen` or `api.tls_listen` can override the generated loopback default. `/api/config`, `/api/restart`, and `/api/exit` are disabled without go2rtc credentials; when credentials are configured, authentication is mandatory even on loopback. Manage the subprocess through LightNVR instead of exposing administrative routes.

In v0.44.0–v0.44.2, the MSE frontend used the dashboard origin but LightNVR lacked a WebSocket relay. Those versions could return `404` on port 8080. Pointing the frontend at port 1984 bypasses LightNVR authorization and is not the fix. The relay now handles that request on the dashboard port. The same update resolves HLS grid authorization for managed `_sub` streams.

See [Docker deployment](DOCKER.md) for runtime user and volume migration, and [go2rtc overrides](GO2RTC_CONFIG_OVERRIDE.md) for configuration ownership.
