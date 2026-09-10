import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import type { Route } from '@playwright/test';

// Ten minutes of blue, 64x64 H.264 at 1 fps. Long enough for timeline seeks,
// small enough to serve inline, and decodable without cameras or local FFmpeg.
// Generated with: ffmpeg -f lavfi -i color=c=blue:size=64x64:rate=1:duration=600
//   -c:v libx264 -threads 1 -pix_fmt yuv420p -movflags +faststart recording.mp4
export const PLAYABLE_RECORDING_MP4 = readFileSync(join(__dirname, 'recording.mp4'));

// Match the production file server's range contract so Chromium exposes a
// seekable duration; an empty media response cannot test timeline seeking.
export function serveRecordingMedia(route: Route) {
  const range = route.request().headers().range?.match(/^bytes=(\d+)-(\d*)$/);
  const headers: Record<string, string> = { 'Accept-Ranges': 'bytes' };
  let body = PLAYABLE_RECORDING_MP4;
  if (range) {
    const start = Number(range[1]);
    const end = Math.min(range[2] ? Number(range[2]) : body.length - 1, body.length - 1);
    headers['Content-Range'] = `bytes ${start}-${end}/${body.length}`;
    body = body.subarray(start, end + 1);
  }
  return route.fulfill({ status: range ? 206 : 200, contentType: 'video/mp4', headers, body });
}
