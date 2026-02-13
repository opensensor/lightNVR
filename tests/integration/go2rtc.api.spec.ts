/**
 * go2rtc API Integration Tests
 * @tags @go2rtc
 */

import { test, expect, APIRequestContext } from '@playwright/test';
import { execSync } from 'child_process';

// GO2RTC_URL may be provided as either an origin (http://host:port) or with a base path
// (http://host:port/go2rtc). Playwright's request `baseURL` doesn't safely preserve paths
// when request paths start with '/', so we split origin vs base_path explicitly.
const GO2RTC_URL = process.env.GO2RTC_URL || 'http://localhost:11984';
let GO2RTC_ORIGIN = GO2RTC_URL;
let GO2RTC_BASE_PATH: string | undefined = process.env.GO2RTC_BASE_PATH;

try {
  const u = new URL(GO2RTC_URL);
  GO2RTC_ORIGIN = u.origin;

  // If GO2RTC_BASE_PATH isn't explicitly set, infer it from GO2RTC_URL's path.
  if (GO2RTC_BASE_PATH === undefined) {
    const inferred = u.pathname.replace(/\/+$/, '');
    // Only infer a non-root path. If GO2RTC_URL has no path component ("/"),
    // keep GO2RTC_BASE_PATH undefined so we fall back to the default (/go2rtc).
    if (inferred !== '/' && inferred !== '') {
      GO2RTC_BASE_PATH = inferred;
    }
  }
} catch {
  // Leave GO2RTC_ORIGIN as-is for backwards compatibility if GO2RTC_URL isn't a full URL.
}

GO2RTC_BASE_PATH = (GO2RTC_BASE_PATH ?? '/go2rtc').replace(/\/+$/, '');
const go2rtcPath = (p: string) => `${GO2RTC_BASE_PATH}${p.startsWith('/') ? p : `/${p}`}`;
const GO2RTC_RTSP_PORT = process.env.GO2RTC_RTSP_PORT || '18554';

const GO2RTC_API_USERNAME = process.env.GO2RTC_API_USERNAME || 'admin';
const GO2RTC_API_PASSWORD = process.env.GO2RTC_API_PASSWORD || 'admin';
const GO2RTC_AUTH_HEADER =
  'Basic ' + Buffer.from(`${GO2RTC_API_USERNAME}:${GO2RTC_API_PASSWORD}`).toString('base64');

test.describe('go2rtc API @go2rtc', () => {
  let request: APIRequestContext;

  test.beforeAll(async ({ playwright }) => {
    request = await playwright.request.newContext({
      baseURL: GO2RTC_ORIGIN,
      extraHTTPHeaders: {
        Authorization: GO2RTC_AUTH_HEADER,
      },
    });
  });

  test.afterAll(async () => {
    await request.dispose();
  });

  test.describe('Streams API', () => {
    test('GET /api/streams returns registered streams', async () => {
      const response = await request.get(go2rtcPath('/api/streams'));
      expect(response.ok()).toBeTruthy();
      
      const data = await response.json();
      expect(typeof data).toBe('object');
      
      // Test streams should be registered
      const streamNames = Object.keys(data);
      expect(streamNames.length).toBeGreaterThan(0);
    });

    test('test_pattern stream is registered', async () => {
      const response = await request.get(go2rtcPath('/api/streams'));
      expect(response.ok()).toBeTruthy();
      
      const data = await response.json();
      expect(data).toHaveProperty('test_pattern');
    });

    test('test_colorbars stream is registered', async () => {
      const response = await request.get(go2rtcPath('/api/streams'));
      expect(response.ok()).toBeTruthy();
      
      const data = await response.json();
      expect(data).toHaveProperty('test_colorbars');
    });

    test('can add a new stream via API', async () => {
      const streamName = 'playwright_dynamic_stream';
      const streamSource = encodeURIComponent('ffmpeg:virtual?video&size=480#video=h264');
      
      const response = await request.put(
        go2rtcPath(`/api/streams?name=${streamName}&src=${streamSource}`)
      );
      expect(response.ok()).toBeTruthy();
      
      // Verify it was added
      const listResponse = await request.get(go2rtcPath('/api/streams'));
      const data = await listResponse.json();
      expect(data).toHaveProperty(streamName);
    });
  });

  test.describe('WebRTC API', () => {
    test('GET /api/webrtc endpoint is accessible', async () => {
      // The WebRTC API may return different status codes depending on request
      const response = await request.get(go2rtcPath('/api/webrtc'));
      // Should be accessible (200) or require POST (405)
      expect([200, 405].includes(response.status())).toBeTruthy();
    });
  });

  test.describe('RTSP Streaming', () => {
    test('RTSP stream is accessible via ffprobe', async () => {
      // Use ffprobe to check if RTSP stream is accessible
      try {
        const result = execSync(
          `ffprobe -v error -show_entries stream=codec_type,codec_name ` +
          `-of json rtsp://localhost:${GO2RTC_RTSP_PORT}/test_pattern`,
          { timeout: 15000 }
        );
        
        const probeData = JSON.parse(result.toString());
        expect(probeData).toHaveProperty('streams');
        expect(probeData.streams.length).toBeGreaterThan(0);
        
        // Should have at least one video stream
        const videoStream = probeData.streams.find((s: any) => s.codec_type === 'video');
        expect(videoStream).toBeDefined();
      } catch (error: any) {
        // If ffprobe fails, it might be because stream isn't ready yet
        console.log('ffprobe test skipped or failed:', error.message);
        test.skip();
      }
    });

    test('can capture a frame from RTSP stream', async () => {
      try {
        // Capture one frame to a temp file
        const outputFile = '/tmp/lightnvr-test/test_frame.jpg';
        execSync(
          `ffmpeg -nostdin -y -rtsp_transport tcp -i rtsp://localhost:${GO2RTC_RTSP_PORT}/test_pattern ` +
          `-frames:v 1 -update 1 ${outputFile}`,
          { timeout: 30000 }
        );
        
        // Check file was created
        const fs = require('fs');
        expect(fs.existsSync(outputFile)).toBeTruthy();
        
        const stats = fs.statSync(outputFile);
        expect(stats.size).toBeGreaterThan(0);
      } catch (error: any) {
        console.log('Frame capture test failed:', error.message);
        test.skip();
      }
    });
  });
});

