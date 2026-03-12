/**
 * Recordings Page UI Tests
 * 
 * Tests for recordings list, pagination, playback, download, delete, and batch operations.
 * @tags @ui @recordings
 */

import { test, expect } from '@playwright/test';
import { RecordingsPage } from '../pages/RecordingsPage';
import { USERS, login, sleep } from '../fixtures/test-fixtures';

type MockRecording = {
  id: number;
  stream: string;
  capture_method: string;
  tags: string[];
  detection_labels: Array<{ label: string; count: number }>;
  start_time_unix?: number;
  duration?: number;
  size?: string;
  protected?: boolean;
  has_detections?: boolean;
};

function includesCsvValue(rawValue: string | null, expected: string): boolean {
  return (rawValue || '').split(',').map(value => value.trim()).includes(expected);
}

function matchesRecording(url: URL, recording: MockRecording): boolean {
  const stream = url.searchParams.get('stream');
  if (stream && !includesCsvValue(stream, recording.stream)) return false;

  const detectionLabel = url.searchParams.get('detection_label');
  if (detectionLabel) {
    const labels = recording.detection_labels.map(item => item.label);
    const requestedLabels = detectionLabel.split(',').map(value => value.trim()).filter(Boolean);
    if (!requestedLabels.some(label => labels.includes(label))) return false;
  }

  const tag = url.searchParams.get('tag');
  if (tag) {
    const requestedTags = tag.split(',').map(value => value.trim()).filter(Boolean);
    if (!requestedTags.some(value => recording.tags.includes(value))) return false;
  }

  const captureMethod = url.searchParams.get('capture_method');
  if (captureMethod && !includesCsvValue(captureMethod, recording.capture_method)) return false;

  const hasDetection = url.searchParams.get('has_detection');
  if (hasDetection === '1' && recording.detection_labels.length === 0) return false;
  if (hasDetection === '-1' && recording.detection_labels.length > 0) return false;

  return true;
}

test.describe('Recordings Page @ui @recordings', () => {
  
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test.describe('Page Load', () => {
    test('should load recordings page successfully', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await expect(page.locator('body')).toBeVisible();
      expect(await recordingsPage.isOnPage()).toBeTruthy();
      
      await page.screenshot({ path: 'test-results/recordings-page-load.png' });
    });

    test('should display recordings list or empty state', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const recordingCount = await recordingsPage.getRecordingCount();
      const isEmpty = await recordingsPage.isEmpty();
      
      console.log(`Recordings count: ${recordingCount}, isEmpty: ${isEmpty}`);
      
      // Either we have recordings or we show an empty state
      if (isEmpty) {
        console.log('No recordings found - this is expected in test environment');
      } else {
        console.log(`Found ${recordingCount} recordings`);
      }
      
      await page.screenshot({ path: 'test-results/recordings-list.png' });
    });

    test('should display filter controls', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      // Check for stream filter
      const hasStreamFilter = await recordingsPage.streamFilter.isVisible();
      console.log(`Has stream filter: ${hasStreamFilter}`);
      
      // Check for date filter
      const hasDateFilter = await recordingsPage.dateFilter.isVisible();
      console.log(`Has date filter: ${hasDateFilter}`);
      
      await page.screenshot({ path: 'test-results/recordings-filters.png' });
    });
  });

  test.describe('Filtering', () => {
    test('should filter recordings by date', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      if (await recordingsPage.dateFilter.isVisible()) {
        // Get today's date in YYYY-MM-DD format
        const today = new Date().toISOString().split('T')[0];
        await recordingsPage.filterByDate(today);
        
        console.log(`Filtered by date: ${today}`);
      }
      
      await page.screenshot({ path: 'test-results/recordings-filter-date.png' });
    });

    test('should filter recordings by stream', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      if (await recordingsPage.streamFilter.isVisible()) {
        // Get available options
        const options = await recordingsPage.streamFilter.locator('option').allTextContents();
        console.log(`Stream filter options: ${options.join(', ')}`);
        
        if (options.length > 1) {
          // Select first non-empty option
          await recordingsPage.streamFilter.selectOption({ index: 1 });
          await sleep(1000);
          console.log('Selected stream filter');
        }
      }
      
      await page.screenshot({ path: 'test-results/recordings-filter-stream.png' });
    });

    test('should support multi-select recordings filters end to end', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      const recordings: MockRecording[] = [
        {
          id: 101,
          stream: 'cam1',
          capture_method: 'scheduled',
          tags: ['urgent'],
          detection_labels: [{ label: 'person', count: 1 }]
        },
        {
          id: 102,
          stream: 'cam2',
          capture_method: 'manual',
          tags: ['review'],
          detection_labels: [{ label: 'car', count: 1 }]
        },
        {
          id: 103,
          stream: 'cam3',
          capture_method: 'motion',
          tags: ['night'],
          detection_labels: [{ label: 'dog', count: 1 }]
        }
      ];
      const seenRequests: string[] = [];

      await page.addInitScript(() => {
        localStorage.setItem('recordings_view_mode', 'table');
      });

      await page.route('**/api/settings*', route => route.fulfill({ json: { generate_thumbnails: false } }));
      await page.route('**/api/streams*', route => route.fulfill({
        json: [{ name: 'cam1' }, { name: 'cam2' }, { name: 'cam3' }]
      }));
      await page.route('**/api/recordings/detection-labels*', route => route.fulfill({
        json: { labels: ['person', 'car', 'dog'] }
      }));
      await page.route('**/api/recordings/tags*', route => route.fulfill({
        json: { tags: ['urgent', 'review', 'night'] }
      }));
      await page.route('**/api/recordings?**', route => {
        const url = new URL(route.request().url());
        seenRequests.push(url.search);

        const filtered = recordings.filter(recording => matchesRecording(url, recording));
        return route.fulfill({
          json: {
            recordings: filtered.map(recording => ({
              ...recording,
              start_time_unix: 1741420800 + recording.id,
              duration: 60,
              size: '1 MB',
              protected: false,
              has_detections: recording.detection_labels.length > 0
            })),
            pagination: { total: filtered.length, pages: 1, limit: 20 }
          }
        });
      });

      await recordingsPage.goto();

      await expect(page.locator('#recordings-table')).toBeVisible();
      await expect.poll(() => seenRequests.length).toBeGreaterThan(0);
      await expect(page.locator('#recordings-table thead th').filter({ hasText: 'Capture Method' })).toHaveCount(1);

      await recordingsPage.expandDetectionObjectsSection();
      await recordingsPage.expandCaptureMethodSection();
      await recordingsPage.expandRecordingTagsSection();

      await expect(recordingsPage.detectionLabelFilter.locator('option')).toContainText(['Add detection object…', 'person', 'car', 'dog']);

      await recordingsPage.filterByStream('cam1');
      await recordingsPage.filterByStream('cam2');
      await recordingsPage.addDetectionLabel('person');
      await recordingsPage.addDetectionLabel('car');
      await recordingsPage.addCaptureMethod('scheduled');
      await recordingsPage.addCaptureMethod('manual');
      await recordingsPage.addTag('urgent');
      await recordingsPage.addTag('review');
      await recordingsPage.applyFilters();

      await expect(recordingsPage.getActiveFilter('Stream: cam1')).toBeVisible();
      await expect(recordingsPage.getActiveFilter('Stream: cam2')).toBeVisible();
      await expect(recordingsPage.getActiveFilter('Object: person')).toBeVisible();
      await expect(recordingsPage.getActiveFilter('Object: car')).toBeVisible();
      await expect(recordingsPage.getActiveFilter('Capture: Continuous')).toBeVisible();
      await expect(recordingsPage.getActiveFilter('Capture: Manual')).toBeVisible();
      await expect(recordingsPage.getActiveFilter('Tag: urgent')).toBeVisible();
      await expect(recordingsPage.getActiveFilter('Tag: review')).toBeVisible();

      await expect.poll(() => seenRequests.some(search => {
        const params = new URL(`http://localhost/${search}`).searchParams;
        return params.get('stream') === 'cam1,cam2' &&
          params.get('detection_label') === 'person,car' &&
          params.get('capture_method') === 'scheduled,manual' &&
          params.get('tag') === 'urgent,review';
      })).toBe(true);

      await expect(page.locator('#recordings-table tbody tr')).toHaveCount(2);
      await expect(page.locator('#recordings-table tbody')).toContainText('cam1');
      await expect(page.locator('#recordings-table tbody')).toContainText('cam2');
      await expect(page.locator('#recordings-table tbody')).toContainText('Continuous');
      await expect(page.locator('#recordings-table tbody')).toContainText('Manual');

      await recordingsPage.getActiveFilter('Stream: cam1').getByRole('button').click();

      await expect(recordingsPage.getActiveFilter('Stream: cam1')).toHaveCount(0);
      await expect(recordingsPage.getActiveFilter('Stream: cam2')).toBeVisible();
      await expect.poll(() => seenRequests.some(search => {
        const params = new URL(`http://localhost/${search}`).searchParams;
        return params.get('stream') === 'cam2' &&
          params.get('detection_label') === 'person,car' &&
          params.get('capture_method') === 'scheduled,manual' &&
          params.get('tag') === 'urgent,review';
      })).toBe(true);

      await expect(page.locator('#recordings-table tbody tr')).toHaveCount(1);
      await expect(page.locator('#recordings-table tbody')).toContainText('cam2');
      await expect(page.locator('#recordings-table tbody')).toContainText('Manual');
      await expect(page.locator('#recordings-table tbody')).not.toContainText('cam1');

      await recordingsPage.getActiveFilter('Object: person').getByRole('button').click();

      await expect(recordingsPage.getActiveFilter('Object: person')).toHaveCount(0);
      await expect(recordingsPage.getActiveFilter('Object: car')).toBeVisible();
      await expect.poll(() => seenRequests.some(search => {
        const params = new URL(`http://localhost/${search}`).searchParams;
        return params.get('stream') === 'cam2' &&
          params.get('detection_label') === 'car' &&
          params.get('capture_method') === 'scheduled,manual' &&
          params.get('tag') === 'urgent,review';
      })).toBe(true);
    });

    test('uses container fullscreen so recording detection overlays remain visible', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);

      await page.addInitScript(() => {
        localStorage.setItem('recordings_view_mode', 'table');
      });

      await page.route('**/api/settings*', route => route.fulfill({ json: { generate_thumbnails: false } }));
      await page.route('**/api/streams*', route => route.fulfill({ json: [{ name: 'cam1' }] }));
      await page.route('**/api/recordings/detection-labels*', route => route.fulfill({ json: { labels: ['person'] } }));
      await page.route('**/api/recordings/tags*', route => route.fulfill({ json: { tags: [] } }));
      await page.route('**/api/recordings?**', route => route.fulfill({ json: {
        recordings: [{
          id: 601,
          stream: 'cam1',
          capture_method: 'scheduled',
          tags: [],
          detection_labels: [{ label: 'person', count: 1 }],
          start_time_unix: 1741420801,
          duration: 60,
          size: '1 MB',
          protected: false,
          has_detections: true
        }],
        pagination: { total: 1, pages: 1, limit: 20 }
      } }));
      await page.route('**/api/recordings/play/601*', route => route.fulfill({ status: 204, body: '' }));
      await page.route('**/api/recordings/601', route => route.fulfill({ json: {
        id: 601,
        stream: 'cam1',
        start_time: '2026-03-08T14:00:00Z',
        end_time: '2026-03-08T14:01:00Z',
        protected: false,
        has_detection: true,
        detection_labels: [{ label: 'person', count: 1 }]
      } }));
      await page.route('**/api/detection/results/cam1?**', route => route.fulfill({ json: {
        detections: [{
          timestamp: 1741442401,
          x: 0.1,
          y: 0.1,
          width: 0.25,
          height: 0.25,
          label: 'person',
          confidence: 0.95
        }]
      } }));

      await recordingsPage.goto();

      await expect(page.locator('#recordings-table')).toBeVisible();
      await page.locator('button[title="Play"]').first().click();

      await expect(page.locator('#video-preview-modal')).toBeVisible();
      await expect(page.locator('#recording-playback-position')).toHaveText('cam1 - 00:00:00');

      await page.locator('#video-preview-modal video').evaluate(video => {
        Object.defineProperty(video, 'currentTime', {
          configurable: true,
          value: 12.6
        });
        video.dispatchEvent(new Event('timeupdate'));
      });

      await expect(page.locator('#recording-playback-position')).toHaveText('cam1 - 00:00:12');
      await expect.poll(() => page.locator('#video-preview-modal video').evaluate(video => video.controlsList?.contains('nofullscreen') ?? false)).toBe(true);
      await expect(page.locator('#detection-overlay-checkbox')).toBeEnabled();

      await page.locator('#detection-overlay-checkbox').check();
      await expect(page.locator('[data-testid="recording-video-container"] canvas')).toBeVisible();

      await page.getByRole('button', { name: 'Fullscreen' }).click();

      await expect.poll(() => page.evaluate(() => document.fullscreenElement?.getAttribute('data-testid'))).toBe('recording-video-container');
      await expect.poll(() => page.evaluate(() => {
        const fullscreenElement = document.fullscreenElement;
        return Boolean(fullscreenElement?.querySelector('video') && fullscreenElement?.querySelector('canvas'));
      })).toBe(true);

      await page.evaluate(() => document.exitFullscreen());
      await expect.poll(() => page.evaluate(() => document.fullscreenElement)).toBeNull();

      await page.locator('#video-preview-modal button.close').click();
      await expect(page.locator('#video-preview-modal')).toHaveCount(0);

      await page.locator('button[title="Play"]').first().click();
      await expect(page.locator('#video-preview-modal')).toBeVisible();
      await expect(page.locator('#recording-playback-position')).toHaveText('cam1 - 00:00:00');
    });
  });

  test.describe('Recording Actions', () => {
    test('should have action buttons on recording cards', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      if (!await recordingsPage.isEmpty()) {
        const firstCard = recordingsPage.getRecordingByIndex(0);
        
        // Check for play button
        const playButton = firstCard.locator('button, a').filter({ hasText: /play|view/i });
        const hasPlay = await playButton.count() > 0;
        console.log(`Has play button: ${hasPlay}`);
        
        // Check for download button
        const downloadButton = firstCard.locator('button, a').filter({ hasText: /download/i });
        const hasDownload = await downloadButton.count() > 0;
        console.log(`Has download button: ${hasDownload}`);
        
        // Check for delete button
        const deleteButton = firstCard.locator('button').filter({ hasText: /delete|remove/i });
        const hasDelete = await deleteButton.count() > 0;
        console.log(`Has delete button: ${hasDelete}`);
      } else {
        console.log('No recordings to check actions on');
      }
      
      await page.screenshot({ path: 'test-results/recordings-actions.png' });
    });
  });

  test.describe('Pagination', () => {
    test('should display pagination if many recordings', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(2000);
      
      const hasPagination = await recordingsPage.pagination.isVisible();
      console.log(`Has pagination: ${hasPagination}`);
      
      if (hasPagination) {
        const hasNext = await recordingsPage.nextPageButton.isVisible();
        const hasPrev = await recordingsPage.prevPageButton.isVisible();
        console.log(`Next button: ${hasNext}, Prev button: ${hasPrev}`);
      }
      
      await page.screenshot({ path: 'test-results/recordings-pagination.png' });
    });
  });

  test.describe('i18n — table column headers and toolbar labels', () => {
    /** Shared route setup for recordings table i18n tests */
    async function setupRecordingsMocks(page: any) {
      await page.addInitScript(() => {
        localStorage.setItem('recordings_view_mode', 'table');
      });
      await page.route('**/api/settings*', route => route.fulfill({ json: { generate_thumbnails: false } }));
      await page.route('**/api/streams*', route => route.fulfill({ json: [{ name: 'cam1' }] }));
      await page.route('**/api/recordings/detection-labels*', route => route.fulfill({ json: { labels: [] } }));
      await page.route('**/api/recordings/tags*', route => route.fulfill({ json: { tags: [] } }));
      await page.route('**/api/recordings?**', route => route.fulfill({
        json: {
          recordings: [{
            id: 1,
            stream: 'cam1',
            capture_method: 'scheduled',
            tags: [],
            detection_labels: [],
            start_time_unix: 1741420800,
            duration: 60,
            size: '1 MB',
            protected: false,
            has_detections: false,
          }],
          pagination: { total: 1, pages: 1, limit: 20 },
        }
      }));
    }

    test('table column headers are rendered via i18n (English)', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await setupRecordingsMocks(page);
      await recordingsPage.goto();

      const thead = page.locator('#recordings-table thead');
      await expect(thead).toBeVisible();

      // All column headers the user reported as missing from translations
      await expect(thead.locator('th').filter({ hasText: 'Stream' })).toHaveCount(1);
      await expect(thead.locator('th').filter({ hasText: 'Capture Method' })).toHaveCount(1);
      await expect(thead.locator('th').filter({ hasText: 'Start Time' })).toHaveCount(1);
      await expect(thead.locator('th').filter({ hasText: 'Duration' })).toHaveCount(1);
      await expect(thead.locator('th').filter({ hasText: 'Size' })).toHaveCount(1);
      await expect(thead.locator('th').filter({ hasText: 'Detections' })).toHaveCount(1);
      await expect(thead.locator('th').filter({ hasText: 'Tags' })).toHaveCount(1);
      await expect(thead.locator('th').filter({ hasText: 'Actions' })).toHaveCount(1);

      await page.screenshot({ path: 'test-results/recordings-i18n-headers.png' });
    });

    test('toolbar batch-action buttons carry correct i18n labels (English)', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await setupRecordingsMocks(page);
      await recordingsPage.goto();

      await expect(page.locator('#recordings-table')).toBeVisible();

      // The toolbar renders labels via t() — verify the English strings are present
      await expect(page.getByRole('button', { name: 'Delete Selected' })).toBeVisible();
      await expect(page.getByRole('button', { name: 'Delete All Filtered' })).toBeVisible();
      await expect(page.getByRole('button', { name: 'Download Selected' })).toBeVisible();
      await expect(page.getByRole('button', { name: 'Manage Tags' })).toBeVisible();

      await page.screenshot({ path: 'test-results/recordings-i18n-toolbar.png' });
    });

    test('empty toolbar shows "No recordings selected" via i18n', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await setupRecordingsMocks(page);
      await recordingsPage.goto();

      await expect(page.locator('#recordings-table')).toBeVisible();

      // With no checkboxes ticked the selected-count div should show the i18n string
      const selectedCount = page.locator('.selected-count');
      await expect(selectedCount).toHaveText('No recordings selected');
    });
  });

  test.describe('Batch Operations', () => {
    test('keeps accumulated selections across pages and supports All items per page', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      const allRecordings: MockRecording[] = Array.from({ length: 105 }, (_, index) => ({
        id: index + 1,
        stream: `cam${String(index + 1).padStart(3, '0')}`,
        capture_method: 'scheduled',
        tags: [],
        detection_labels: [],
        start_time_unix: 1741420800 + index,
        duration: 60,
        size: '1 MB',
        protected: false,
        has_detections: false
      }));
      const seenRequests: string[] = [];

      await page.addInitScript(() => {
        localStorage.setItem('recordings_view_mode', 'table');
      });

      await page.route('**/api/settings*', route => route.fulfill({ json: { generate_thumbnails: false } }));
      await page.route('**/api/streams*', route => route.fulfill({ json: [] }));
      await page.route('**/api/recordings/detection-labels*', route => route.fulfill({ json: { labels: [] } }));
      await page.route('**/api/recordings/tags*', route => route.fulfill({ json: { tags: [] } }));
      await page.route('**/api/recordings?**', route => {
        const url = new URL(route.request().url());
        seenRequests.push(url.search);

        const limitParam = url.searchParams.get('limit') || '20';
        const currentPage = parseInt(url.searchParams.get('page') || '1', 10);
        const allMode = limitParam === 'all';
        const limit = allMode ? allRecordings.length : parseInt(limitParam, 10);
        const startIndex = allMode ? 0 : (currentPage - 1) * limit;
        const pageRecordings = allRecordings.slice(startIndex, startIndex + limit);
        const totalPages = allMode ? 1 : Math.ceil(allRecordings.length / limit);

        return route.fulfill({
          json: {
            recordings: pageRecordings,
            pagination: {
              total: allRecordings.length,
              pages: totalPages,
              limit
            }
          }
        });
      });

      await recordingsPage.goto();

      const selectPageCheckbox = page.getByLabel('Select all recordings on this page');
      const selectedCount = page.locator('.selected-count');
      const clearButton = page.getByRole('button', { name: 'Clear' });

      await expect(page.locator('#recordings-table')).toBeVisible();
      await expect(page.locator('#recordings-table tbody tr')).toHaveCount(20);

      await selectPageCheckbox.check();
      await expect(selectedCount).toHaveText('20 recordings selected');

      await page.getByTitle('Next page').click();
      await expect(page.locator('#recordings-table tbody')).toContainText('cam021');
      await expect(selectedCount).toHaveText('20 recordings selected');

      await selectPageCheckbox.check();
      await expect(selectedCount).toHaveText('40 recordings selected');

      await page.getByTitle('Previous page').click();
      await expect(page.locator('#recordings-table tbody')).toContainText('cam001');
      await selectPageCheckbox.uncheck();
      await expect(selectedCount).toHaveText('20 recordings selected');

      await clearButton.click();
      await expect(selectedCount).toHaveText('No recordings selected');

      await page.getByRole('button', { name: 'Display' }).click();
      await page.locator('#page-size').selectOption('all');

      await expect.poll(() => seenRequests.some(search => {
        const params = new URL(`http://localhost/${search}`).searchParams;
        return params.get('limit') === 'all' && params.get('page') === '1';
      })).toBe(true);
      await expect(page.locator('#recordings-table tbody tr')).toHaveCount(105);
      await expect(page.getByTitle('Next page')).toHaveCount(0);

      await selectPageCheckbox.check();
      await expect(selectedCount).toHaveText('105 recordings selected');
    });

    test('should display batch delete button if available', async ({ page }) => {
      const recordingsPage = new RecordingsPage(page);
      await recordingsPage.goto({ waitForNetworkIdle: true });
      
      await sleep(1000);
      
      const hasBatchDelete = await recordingsPage.batchDeleteButton.isVisible();
      console.log(`Has batch delete button: ${hasBatchDelete}`);
      
      await page.screenshot({ path: 'test-results/recordings-batch.png' });
    });
  });
});

