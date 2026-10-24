# Reverse Proxy & HTTPS

Keep the managed go2rtc API bound to `127.0.0.1:1984`. Do not publish port 1984 or forward its entire `/go2rtc/` prefix. Browser media URLs use the dashboard's origin over both HTTP and HTTPS.

LightNVR's authenticated proxy serves HLS, snapshots and WebRTC signaling, including per-camera live-view permissions. WebRTC works directly on port 8080 or through a normal HTTPS proxy to LightNVR. MSE still requires a reverse proxy that can reach go2rtc on loopback and **authenticates requests before forwarding** the exact WebSocket path:

| Public path | Upstream |
| --- | --- |
| `/go2rtc/api/ws` | `http://127.0.0.1:1984` (WebSocket upgrade) |
| `/go2rtc/api/webrtc` | `http://127.0.0.1:8080` (authorized HTTP signaling) |
| All other paths | `http://127.0.0.1:8080` (LightNVR's authorization and route allowlist) |

The go2rtc admin UI, config, restart, and exit endpoints are not live-view routes. Do not expose them. If using an overridden `api.base_path`, change the WebSocket path in the proxy configuration to match. Keep the public WebRTC path at `/go2rtc/api/webrtc`; LightNVR translates the upstream prefix.

The examples below add a gateway login for the entire site, so the browser authenticates before opening a WebSocket. Keep LightNVR authentication enabled as well. The gateway credentials authorize access to go2rtc's WebSocket endpoint and **do not enforce LightNVR's per-camera permissions**. Only grant them to users permitted to access all streams. Use LightNVR's WebRTC/HLS/snapshot routes when per-camera authorization is required.

## Network placement

These examples require the proxy and go2rtc to share a network namespace: native services on one host, containers using `network_mode: service:lightnvr`, or containers in the same Kubernetes Pod. A separate Docker bridge container or a proxy on the Docker host cannot reach the container's loopback listener. Place the proxy in the same namespace; do not restore a public API listener to work around this.

When a proxy container shares LightNVR's network namespace, publish the proxy's HTTPS port on the LightNVR service and mount only the proxy's config, certificates, and password file into the proxy container. Port 1984 remains unpublished. Direct access to port 8080 supports WebRTC, HLS and snapshots; MSE requires the proxy. WebRTC media still needs reachable ICE candidates (normally port 8555 TCP/UDP).

## Trusted proxy addresses

Configure LightNVR to trust forwarded client addresses only from the proxy:

```ini
[web]
trusted_proxy_cidrs = 127.0.0.1/32,::1/128
```

Use the actual proxy address range for any other network arrangement. This setting controls forwarded address handling; it does not authenticate media requests.

## nginx

Create `/etc/nginx/nvr.htpasswd` with `htpasswd`, restrict access to that file, and install your TLS certificate. The WebSocket location inherits the server's authentication requirement.

```nginx
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

    auth_basic "NVR gateway";
    auth_basic_user_file /etc/nginx/nvr.htpasswd;

    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_read_timeout 3600s;
    proxy_send_timeout 3600s;

    location = /go2rtc/api/ws {
        proxy_pass http://127.0.0.1:1984;
    }
    location / {
        proxy_pass http://127.0.0.1:8080;
    }
}
```

## Caddy

Generate a password hash with `caddy hash-password`, then replace the placeholder below. See Caddy's [basic_auth documentation](https://caddyserver.com/docs/caddyfile/directives/basic_auth).

```caddy
nvr.example.com {
    encode zstd gzip
    basic_auth {
        operator REPLACE_WITH_PASSWORD_HASH
    }

    @media path /go2rtc/api/ws
    handle @media {
        reverse_proxy 127.0.0.1:1984
    }
    handle {
        reverse_proxy 127.0.0.1:8080
    }
}
```

Caddy handles WebSocket upgrades automatically. Both handlers inherit the site authentication requirement.

## Traefik

Use the file provider with Traefik in the same network namespace as LightNVR. This dynamic configuration assumes a TLS `websecure` entry point and a `letsencrypt` certificate resolver are already configured. Create the password file inside the **Traefik** container; see [BasicAuth configuration](https://doc.traefik.io/traefik/v2.11/middlewares/http/basicauth/).

```yaml
http:
  routers:
    nvr-media:
      rule: "Host(`nvr.example.com`) && Path(`/go2rtc/api/ws`)"
      priority: 100
      entryPoints: [websecure]
      middlewares: [nvr-auth]
      service: go2rtc-media
      tls:
        certResolver: letsencrypt
    nvr:
      rule: "Host(`nvr.example.com`)"
      entryPoints: [websecure]
      middlewares: [nvr-auth]
      service: lightnvr
      tls:
        certResolver: letsencrypt
  middlewares:
    nvr-auth:
      basicAuth:
        usersFile: /etc/traefik/nvr.htpasswd
  services:
    go2rtc-media:
      loadBalancer:
        servers:
          - url: http://127.0.0.1:1984
    lightnvr:
      loadBalancer:
        servers:
          - url: http://127.0.0.1:8080
```

## Verify after upgrading

1. From another host, confirm TCP port 1984 is unreachable. Inside the recorder's network namespace, confirm it listens on `127.0.0.1` only.
2. Without credentials, requests to both media routes must return `401` or `403`, including WebSocket upgrade requests and WebRTC POSTs.
3. With gateway credentials, confirm MSE upgrades to `101`. With a LightNVR login authorized for the camera, confirm WebRTC negotiates video. Browser network requests must stay on the dashboard origin.
4. With and without gateway credentials, confirm `/go2rtc/api/config`, `/go2rtc/api/restart`, and `/go2rtc/api/exit` never reach go2rtc through the public proxy. Likewise check path suffixes and traversal variants.
5. Confirm WebRTC, HLS and snapshots use LightNVR authorization, and audit logs show the correct client address.

## Existing deployments

Remove any old wildcard proxy route and port-1984 mapping. Review `override.yaml`: an explicit `api.listen` or `api.tls_listen` can override the generated loopback default. `/api/config`, `/api/restart`, and `/api/exit` are disabled without go2rtc credentials; when credentials are configured, authentication is mandatory even on loopback. Manage the subprocess through LightNVR instead of exposing administrative routes.

See [Docker deployment](DOCKER.md) for runtime user and volume migration, and [go2rtc overrides](GO2RTC_CONFIG_OVERRIDE.md) for configuration ownership.
