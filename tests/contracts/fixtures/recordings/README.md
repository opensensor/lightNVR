# Recording read fixtures

`seed.ts` creates a real two-second H.264 MP4 using FFmpeg's synthetic test source,
checks it with ffprobe, decodes it, and derives a JPEG thumbnail. Every generated
file and database write stays in the owned `lightnvr-contract-*` temporary server
directory. There is no recording-creation API, so the helper inserts controlled
recording/tag/detection metadata after server startup. Authentication and policy
are exercised through the real server; no permission mocks are used.

Rows cover visible/hidden cameras, finalized and in-progress recordings,
interrupted invalid media, deleted-camera capture identity, and historical names
reused by another current camera. HTTP tests compare actual media bytes and
range headers, reconstruct and probe the downloaded file, and exercise polling
after supported sibling deletion. That last check qualifies stable recording
IDs, not the retention scheduler or disk-pressure engine.

Fixtures use synthetic pixels, labels and credentials only. Native process
failures retain their temporary fixture for diagnosis. There are no external
camera, storage or network dependencies beyond the owned loopback HTTP server.

A nonempty stored capture UUID is authoritative: if no current camera owns that
UUID, direct reads return404 even when the historical display name is reused.
Null/empty capture UUIDs retain the distinct legacy behavior of authorizing via
the current camera with the stored stream name. Historical-name regressions
exercise UUID-backed rows; they do not silently redefine legacy row authority.
