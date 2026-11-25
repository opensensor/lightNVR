# ZimaOS Custom App

Import the [ZimaOS Compose file](../deploy/zimaos/docker-compose.yml) through
ZimaOS's Custom App Compose import. The file includes the `x-casaos` app entry
metadata used for the dashboard shortcut and maps the web UI to port 8080.
Ports 8554 and 8555 expose RTSP and WebRTC. Change the published ports before
installing if they conflict with other apps.

The Compose variant uses named `config` and `data` volumes. The `data` volume
contains the database, recordings, models, and swap under
`/var/lib/lightnvr/data`. Keep both volumes when updating the app. Do not
replace these mounts with empty host directories during an upgrade: that makes
the existing database and recordings disappear from the container's view.

For an existing LightNVR installation, back up and migrate its config and data
before importing this variant. See [Data paths in existing installations](DOCKER.md#data-paths-in-existing-installations).

The container runs as UID:GID 10001:10001. If you choose bind mounts instead
of named volumes, create the host directories and make them writable by that
UID:GID before starting. See the [Docker deployment guide](DOCKER.md) for
ownership migration and troubleshooting.
