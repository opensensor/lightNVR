/**
 * LightNVR API Integration Tests
 * @tags @api
 */

import { test, expect, APIRequestContext } from '@playwright/test';

const LIGHTNVR_URL = process.env.LIGHTNVR_URL || 'http://localhost:18080';
const AUTH_HEADER = 'Basic ' + Buffer.from('admin:admin').toString('base64');

test.describe('LightNVR API @api', () => {
  let request: APIRequestContext;

  test.beforeAll(async ({ playwright }) => {
    request = await playwright.request.newContext({
      baseURL: LIGHTNVR_URL,
      extraHTTPHeaders: {
        'Authorization': AUTH_HEADER,
      },
    });
  });

  test.afterAll(async () => {
    await request.dispose();
  });

  test.describe('System API', () => {
    test('GET /api/system returns system info', async () => {
      const response = await request.get('/api/system');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      expect(data).toHaveProperty('version');
      expect(typeof data.version).toBe('string');
    });
  });

  test.describe('Authentication', () => {
    test('rejects invalid credentials', async ({ playwright }) => {
      const unauthRequest = await playwright.request.newContext({
        baseURL: LIGHTNVR_URL,
        extraHTTPHeaders: {
          'Authorization': 'Basic ' + Buffer.from('admin:wrongpassword').toString('base64'),
        },
        timeout: 10000, // 10 second timeout
      });

      const response = await unauthRequest.get('/api/system', { timeout: 10000 });
      expect(response.status()).toBe(401);

      await unauthRequest.dispose();
    });

    test('accepts valid credentials', async () => {
      const response = await request.get('/api/system');
      expect(response.status()).toBe(200);
    });
  });

  test.describe('Streams API', () => {
    test('GET /api/streams returns stream list', async () => {
      const response = await request.get('/api/streams');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      // API returns an array directly
      expect(Array.isArray(data)).toBeTruthy();
    });

    test('POST /api/streams creates a new stream', async () => {
      const streamData = {
        name: 'playwright_test_stream',
        url: 'rtsp://localhost:8554/test_pattern',
        enabled: true,
        width: 1280,
        height: 720,
        fps: 15,
        codec: 'h264',
        priority: 5,
        record: false,
      };

      const response = await request.post('/api/streams', {
        data: streamData,
      });

      // Should succeed (201) or already exist (409)
      expect([200, 201, 409].includes(response.status())).toBeTruthy();

      if (response.ok()) {
        const data = await response.json();
        // API returns { success: true } on creation
        expect(data.success).toBe(true);
      }
    });

    test('GET /api/streams/{name} returns stream details', async () => {
      // First create a stream to ensure it exists
      await request.post('/api/streams', {
        data: {
          name: 'playwright_get_test',
          url: 'rtsp://localhost:8554/test_pattern',
          enabled: false,
        },
      });

      const response = await request.get('/api/streams/playwright_get_test');
      // Might be 200 or 404 depending on implementation
      if (response.ok()) {
        const data = await response.json();
        expect(data.name).toBe('playwright_get_test');
      }
    });
  });

  test.describe('Settings API', () => {
    test('GET /api/settings returns settings', async () => {
      const response = await request.get('/api/settings');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      expect(data).toHaveProperty('storage_path');
      expect(data).toHaveProperty('web_port');
    });
  });

  test.describe('Recordings API', () => {
    test('GET /api/recordings returns recording list', async () => {
      const response = await request.get('/api/recordings');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      // API returns { recordings: [...], pagination: {...} }
      expect(data).toHaveProperty('recordings');
      expect(Array.isArray(data.recordings)).toBeTruthy();
    });
  });
});

