# Running LightNVR on Windows with Podman + WSL2

*Verified against LightNVR 0.37.x.*

LightNVR is Linux software. There is no native Windows build and none is planned — on
Windows you run the published Linux container. This guide uses **Podman** with the
**WSL2** backend, which is the lightest way to do that: no background desktop service,
no daemon running as SYSTEM, and no subscription requirement for commercial use.

If you already run Docker Desktop and are happy with it, [DOCKER.md](DOCKER.md) applies
unchanged — everything below is Podman-specific setup around the same image.

**Time to first camera: about 20 minutes**, most of it waiting on downloads.

---

## Before you start

| Requirement | Notes |
|---|---|
| Windows 11 | Podman's WSL backend requires Windows 11 or later. |
| Hardware virtualization enabled | Check Task Manager → Performance → CPU → "Virtualization: Enabled". Enable in UEFI/BIOS if not. |
| ~10 GB free on `C:` | For WSL, the image, and headroom. Recordings need much more — see [Storage](#storage-decide-this-before-you-start). |
| Cameras reachable by IP | RTSP/ONVIF over your LAN. USB webcams are **not** supported through WSL2. |

A note on expectations: this runs a 24/7 recorder on a desktop OS. Windows sleeping,
updating, or rebooting stops your recordings. [Keeping it running](#keeping-it-running)
covers what to change.

---

## 1. Install WSL2

Open PowerShell **as Administrator**:

```powershell
wsl --install
wsl --update
```

Reboot if prompted, then confirm you are on WSL 2:

```powershell
wsl --version
```

You do not need to install Ubuntu or any other distribution. Podman creates and manages
its own.

## 2. Install Podman

In a normal (non-admin) PowerShell:

```powershell
winget install RedHat.Podman
```

Close and reopen PowerShell so `podman` lands on your `PATH`, then:

```powershell
podman --version
```

If you prefer a GUI — and you probably do for the autostart setting later — install
Podman Desktop as well:

```powershell
winget install RedHat.Podman-Desktop
```

## 3. Size the WSL VM

Podman's WSL backend shares one VM with all your WSL distributions, so its CPU and memory
come from WSL's global config, **not** from `podman machine set`. By default WSL will take
up to 50% of your RAM, which is more than LightNVR needs and more than you probably want a
background recorder holding.

Create `%UserProfile%\.wslconfig`:

```ini
[wsl2]
memory=4GB
processors=4
```

4 GB is comfortable for a handful of cameras with detection enabled. Raise it if you run
many streams or see the container getting OOM-killed.

Apply it:

```powershell
wsl --shutdown
```

## 4. Create the Podman machine

```powershell
podman machine init
podman machine start
podman info
```

`podman machine init` creates a WSL distribution named `podman-machine-default`. The
machine runs **rootless** by default, which is what you want — LightNVR's ports are all
above 1023, so nothing here needs root.

---

## Storage: decide this before you start

This is the one decision that is painful to undo, so make it deliberately.

**Do not put LightNVR's database or recordings on a `C:\` path.** Windows drives are
exposed inside WSL through DrvFs (`/mnt/c`), which is slow for the small synchronous
writes SQLite makes and does not implement POSIX file locking the way SQLite expects.
That combination is how you get a corrupted database, and continuous video writes through
DrvFs will underperform badly on top of it.

Use **named volumes** instead. They live on the ext4 filesystem inside the WSL virtual
disk and behave exactly like they would on a Linux host:

```powershell
podman volume create lightnvr-config
podman volume create lightnvr-data
```

**Where does that actually live?** Inside the WSL virtual disk on `C:`, which grows on
demand up to 1 TB by default. You can browse it from Explorer at:

```
\\wsl.localhost\podman-machine-default\home\user\.local\share\containers\storage\volumes\
```

**If you want recordings on a different, bigger drive**, do not bind-mount `D:\` into the
container — move the whole WSL distribution to that drive instead, so everything stays on
ext4:

```powershell
podman machine stop
wsl --manage podman-machine-default --move D:\wsl\podman
podman machine start
```

---

## 5. Choose how you will reach the web UI

Podman on Windows publishes container ports **against localhost (127.0.0.1) only**. So
`http://localhost:8080` works on the Windows machine itself and nothing else on your
network can reach it. For an NVR you almost certainly want to view it from a phone or
another PC. Pick one of these two.

### Option A — Mirrored networking (recommended)

WSL's mirrored mode makes the VM share the host's network interfaces. This gets you LAN
access **and** working multicast, which matters because ONVIF camera discovery uses
multicast. It requires Windows 11 22H2 or higher.

Add to `%UserProfile%\.wslconfig`:

```ini
[wsl2]
memory=4GB
processors=4
networkingMode=mirrored
```

Then allow inbound connections through the Hyper-V firewall, in PowerShell **as
Administrator**:

```powershell
Set-NetFirewallHyperVVMSetting -Name '{40E0AC32-46A5-438A-A0B2-2B479E8F2E90}' -DefaultInboundAction Allow
```

Restart WSL for both to take effect:

```powershell
wsl --shutdown
podman machine start
```

### Option B — NAT with port forwarding

If you are on Windows 10 or mirrored mode causes trouble (it can interact badly with some
VPN clients), stay on the default NAT mode and forward the web port from the host:

```powershell
netsh interface portproxy add v4tov4 listenport=8080 listenaddress=0.0.0.0 connectport=8080 connectaddress=127.0.0.1
New-NetFirewallRule -DisplayName "LightNVR Web UI" -Direction Inbound -Protocol TCP -LocalPort 8080 -Action Allow
```

Repeat for any other port you need to expose (8554 RTSP, 8555 WebRTC, 1984 go2rtc).
With NAT, **ONVIF auto-discovery by multicast will not work** — see
[Adding cameras](#7-adding-cameras).

---

## 6. Run LightNVR

Replace `192.168.1.0/24` with your actual camera network and set your timezone:

```powershell
podman run -d `
  --name lightnvr `
  --restart always `
  -p 8080:8080 `
  -p 8554:8554 `
  -p 8555:8555 `
  -p 8555:8555/udp `
  -p 1984:1984 `
  -v lightnvr-config:/etc/lightnvr `
  -v lightnvr-data:/var/lib/lightnvr/data `
  -e TZ=America/New_York `
  -e LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24 `
  ghcr.io/opensensor/lightnvr:latest
```

(Those are PowerShell backticks for line continuation, not quotes. In CMD, use `^`, or
just put it all on one line.)

Watch it come up:

```powershell
podman logs -f lightnvr
```

Then open **http://localhost:8080**.

> **Change the password immediately.** The container ships with `admin` / `admin` and the
> web server binds to `0.0.0.0`. Once you expose port 8080 to your LAN, those credentials
> are all that stands in front of your cameras. Change it under **Settings → Users** on
> first login.

### Ports, and which ones you actually need

| Port | Purpose | Needed? |
|---|---|---|
| 8080 | Web UI and API | Always |
| 1984 | go2rtc API | Yes — the UI's live view uses it |
| 8554 | RTSP restream | Only if other apps pull streams from LightNVR |
| 8555 TCP+UDP | WebRTC | Only for low-latency live view from other devices |

---

## 7. Adding cameras

`LIGHTNVR_ONVIF_NETWORK` is what makes discovery work from inside a container. LightNVR
sweeps that CIDR range with unicast probes, which crosses NAT fine — this is why it is set
in the run command above. It falls back to multicast WS-Discovery, and *that* part only
works in mirrored networking mode.

So:

- **Mirrored mode** — discovery works both ways. Use **Settings → ONVIF Discovery**.
- **NAT mode** — set `LIGHTNVR_ONVIF_NETWORK` to the right subnet and discovery still
  works via the unicast sweep. If a camera does not appear, add it manually by RTSP URL.

Outbound connections to cameras work in either mode. NAT only breaks *discovery*, never
recording.

---

## Keeping it running

Four separate things can stop your recorder. Handle all four.

**1. The container.** `--restart always` is already in the run command. Make sure the
service that acts on it is enabled inside the machine:

```powershell
podman machine ssh 'systemctl --user enable --now podman-restart.service'
podman machine ssh 'sudo loginctl enable-linger $USER'
```

**2. The Podman machine.** It does not start itself when you log in. The simplest fix is
Podman Desktop → **Settings → Preferences**, and enable the login/engine autostart
options. Without a GUI, create a logon task:

```powershell
$action  = New-ScheduledTaskAction -Execute "podman" -Argument "machine start"
$trigger = New-ScheduledTaskTrigger -AtLogOn
Register-ScheduledTask -TaskName "Start Podman Machine" -Action $action -Trigger $trigger
```

Note the consequence of rootless: the machine starts **at logon**, not at boot. If the PC
reboots and nobody logs in, nothing records. If that is unacceptable, this workload wants
a Linux host or a NAS, not a Windows desktop.

**3. Windows sleeping.** A sleeping PC records nothing.

```powershell
powercfg /change standby-timeout-ac 0
powercfg /change hibernate-timeout-ac 0
powercfg /change disk-timeout-ac 0
```

**4. Windows Update reboots.** Set active hours, or accept the gaps.

---

## Running it as a service instead (Quadlet)

If you would rather manage LightNVR as a systemd unit inside the machine than as a
`podman run` invocation you have to remember, Podman's Quadlet does that. Remove the
container from step 6 first, or the two will fight over the same ports:

```powershell
podman rm -f lightnvr
```

Open a shell in the machine:

```powershell
podman machine ssh
```

and from that Linux shell, write the unit:

```bash
mkdir -p ~/.config/containers/systemd
cat > ~/.config/containers/systemd/lightnvr.container <<'EOF'
[Unit]
Description=LightNVR

[Container]
Image=ghcr.io/opensensor/lightnvr:latest
AutoUpdate=registry
PublishPort=8080:8080
PublishPort=8554:8554
PublishPort=8555:8555
PublishPort=8555:8555/udp
PublishPort=1984:1984
Volume=lightnvr-config:/etc/lightnvr
Volume=lightnvr-data:/var/lib/lightnvr/data
Environment=TZ=America/New_York
Environment=LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24

[Service]
Restart=always

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
systemctl --user start lightnvr
exit
```

`AutoUpdate=registry` lets `podman auto-update` pull new releases in place.

---

## Updating

```powershell
podman pull ghcr.io/opensensor/lightnvr:latest
podman stop lightnvr
podman rm lightnvr
# re-run the podman run command from step 6
```

Your config and recordings are in the named volumes, so they survive. Pin a version tag
instead of `latest` (for example `ghcr.io/opensensor/lightnvr:0.37.1`) if you would rather
choose when to move.

## Backing up

The database and recordings both live in `lightnvr-data`. To pull a copy out to Windows:

```powershell
podman cp lightnvr:/var/lib/lightnvr/data/database C:\lightnvr-backup\
```

Or browse the volume directly in Explorer through `\\wsl.localhost\podman-machine-default\`.
Copying *out* through DrvFs is fine — it is only running the live database on it that
causes problems.

---

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `podman machine start` fails with a WSL error | WSL not updated | `wsl --update`, then retry |
| Web UI works on the PC but not from a phone | Ports bind to 127.0.0.1 only | Mirrored mode, or `netsh portproxy` — [step 5](#5-choose-how-you-will-reach-the-web-ui) |
| ONVIF discovery finds nothing | Multicast does not cross NAT | Set `LIGHTNVR_ONVIF_NETWORK` to your camera subnet, or switch to mirrored mode |
| Recording timestamps are wrong | Timezone not set | Pass `-e TZ=Your/Zone` |
| "database is locked", corruption | Database on a `C:\` bind mount | Move to a named volume — [Storage](#storage-decide-this-before-you-start) |
| Container killed unexpectedly | WSL VM memory too low | Raise `memory=` in `.wslconfig`, `wsl --shutdown`, restart |
| Everything stops after a reboot | Machine only starts at logon | [Keeping it running](#keeping-it-running) |
| Live view black, UI otherwise fine | go2rtc port not published | Ensure `-p 1984:1984` |

Container logs are the first place to look:

```powershell
podman logs --tail 100 lightnvr
```

For problems that are not Windows-specific, [TROUBLESHOOTING.md](TROUBLESHOOTING.md) and
[TROUBLESHOOTING_WEB_INTERFACE.md](TROUBLESHOOTING_WEB_INTERFACE.md) apply here too.

---

## Uninstalling

```powershell
podman stop lightnvr
podman rm lightnvr
podman volume rm lightnvr-config lightnvr-data   # deletes recordings
podman machine stop
podman machine rm podman-machine-default
winget uninstall RedHat.Podman
```

If you added port proxies, remove them:

```powershell
netsh interface portproxy delete v4tov4 listenport=8080 listenaddress=0.0.0.0
```

---

## See also

- [DOCKER.md](DOCKER.md) — full container reference: every environment variable, volume, and compose options
- [CONFIGURATION.md](CONFIGURATION.md) — `lightnvr.ini` settings
- [REVERSE_PROXY.md](REVERSE_PROXY.md) — putting LightNVR behind HTTPS
