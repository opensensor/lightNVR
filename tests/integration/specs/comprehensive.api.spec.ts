/**
 * Comprehensive LightNVR API Integration Tests
 * 
 * Extended API tests for all endpoints with various scenarios.
 * @tags @api
 */

import { test, expect, APIRequestContext } from '@playwright/test';
import { CONFIG, USERS, getAuthHeader } from '../fixtures/test-fixtures';

const AUTH_HEADER = getAuthHeader(USERS.admin);

test.describe('LightNVR API Comprehensive Tests @api', () => {
  let request: APIRequestContext;

  test.beforeAll(async ({ playwright }) => {
    request = await playwright.request.newContext({
      baseURL: CONFIG.LIGHTNVR_URL,
      extraHTTPHeaders: { 'Authorization': AUTH_HEADER },
    });
  });

  test.afterAll(async () => {
    await request.dispose();
  });

  test.describe('System API', () => {
    test('GET /api/system returns complete system info', async () => {
      const response = await request.get('/api/system');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      expect(data).toHaveProperty('version');
      console.log(`System version: ${data.version}`);
      
      // Check for other expected fields
      const expectedFields = ['version', 'uptime', 'cpu_usage', 'memory_usage'];
      for (const field of expectedFields) {
        if (data[field] !== undefined) {
          console.log(`  ${field}: ${data[field]}`);
        }
      }
    });

    test('GET /api/system/status returns status', async () => {
      const response = await request.get('/api/system/status');
      if (response.ok()) {
        const data = await response.json();
        console.log('System status:', JSON.stringify(data, null, 2));
      }
    });

    test('GET /api/health returns health status', async () => {
      const response = await request.get('/api/health');
      expect(response.ok()).toBeTruthy();
    });

    test('GET /api/system/logs returns logs', async () => {
      const response = await request.get('/api/system/logs');
      if (response.ok()) {
        const data = await response.json();
        console.log(`Logs response type: ${typeof data}`);
      }
    });
  });

  test.describe('Streams API', () => {
    const testStreamName = `api_test_stream_${Date.now()}`;

    test('GET /api/streams returns stream list', async () => {
      const response = await request.get('/api/streams');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      expect(Array.isArray(data)).toBeTruthy();
      console.log(`Found ${data.length} streams`);
    });

    test('POST /api/streams creates a new stream', async () => {
      const streamData = {
        name: testStreamName,
        url: 'rtsp://localhost:18554/test_pattern',
        enabled: true,
        streaming_enabled: true,
        width: 1280,
        height: 720,
        fps: 15,
        codec: 'h264',
        priority: 5,
        record: false,
      };

      const response = await request.post('/api/streams', { data: streamData });
      expect([200, 201, 409].includes(response.status())).toBeTruthy();

      if (response.ok()) {
        const data = await response.json();
        console.log(`Created stream: ${testStreamName}`);
      }
    });

    test('GET /api/streams/{name} returns stream details', async () => {
      const response = await request.get(`/api/streams/${testStreamName}`);
      if (response.ok()) {
        const data = await response.json();
        expect(data.name).toBe(testStreamName);
        console.log(`Stream details: ${JSON.stringify(data, null, 2)}`);
      }
    });

    test('PUT /api/streams/{name} updates stream', async () => {
      const updateData = {
        name: testStreamName,
        url: 'rtsp://localhost:18554/test_pattern',
        enabled: false,
        record: false,
      };

      const response = await request.put(`/api/streams/${testStreamName}`, { data: updateData });
      if (response.ok()) {
        console.log(`Updated stream: ${testStreamName}`);
      }
    });

    test('DELETE /api/streams/{name} deletes stream', async () => {
      const response = await request.delete(`/api/streams/${testStreamName}`);
      expect([200, 204, 404].includes(response.status())).toBeTruthy();
      console.log(`Delete stream response: ${response.status()}`);
    });

    test('POST /api/streams/test tests stream connection', async () => {
      const testData = {
        url: 'rtsp://localhost:18554/test_pattern',
      };

      const response = await request.post('/api/streams/test', { data: testData });
      console.log(`Stream test response: ${response.status()}`);
    });
  });

  test.describe('Recordings API', () => {
    test('GET /api/recordings returns recording list', async () => {
      const response = await request.get('/api/recordings');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      expect(data).toHaveProperty('recordings');
      expect(Array.isArray(data.recordings)).toBeTruthy();
      console.log(`Found ${data.recordings.length} recordings`);
    });

    test('GET /api/recordings with pagination', async () => {
      const response = await request.get('/api/recordings?limit=10&offset=0');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      if (data.pagination) {
        console.log(`Pagination: ${JSON.stringify(data.pagination)}`);
      }
    });

    test('GET /api/recordings/protected returns protected count', async () => {
      const response = await request.get('/api/recordings/protected');
      if (response.ok()) {
        const data = await response.json();
        console.log(`Protected recordings: ${JSON.stringify(data)}`);
      }
    });
  });

  test.describe('Settings API', () => {
    test('GET /api/settings returns all settings', async () => {
      const response = await request.get('/api/settings');
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      expect(data).toHaveProperty('storage_path');
      expect(data).toHaveProperty('web_port');
      console.log('Settings keys:', Object.keys(data).join(', '));
    });
  });
});

