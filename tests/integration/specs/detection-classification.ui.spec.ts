import { test, expect, Page } from '@playwright/test';
import { login, USERS } from '../fixtures/test-fixtures';
import { TimelinePage } from '../pages/TimelinePage';

// INT_01 L15: detections linked to an observation show the classification
// beside the detector's own label, with an owner badge and a link back.
type Segment = { id: number; stream: string; start_timestamp: number; end_timestamp: number };

const OWNER_ID = '44444444-4444-4444-8444-444444444444';
const AUTO_ID = '66666666-6666-4666-8666-666666666666';

function localTimestamp(date: string, time: string): number {
  const [y, m, d] = date.split('-').map(Number);
  const [hh, mm, ss] = time.split(':').map(Number);
  return Math.floor(new Date(y, m - 1, d, hh, mm, ss).getTime() / 1000);
}

function annotation(kind: 'automated' | 'owner_confirmed', label: string, confidence: number) {
  return {
    key: 'yawamf/species', writer_namespace: 'yawamf', classifier: 'yawamf', model_version: null,
    label, taxonomy_id: null, confidence, classified_at: '2026-03-08T12:01:00.000Z',
    input_asset_id: '55555555-5555-4555-8555-555555555555', input_revision: 2, kind,
    annotation_revision: 1, created_at: '2026-03-08T12:01:00.000Z', updated_at: '2026-03-08T12:01:00.000Z',
  };
}

async function mockTimeline(page: Page, stream: string, segment: Segment) {
  await page.route(/\/api\/streams(?:\?.*)?$/, route => route.fulfill({
    json: { streams: [{ name: stream }], total_pages: 1 },
  }));
  await page.route('**/api/timeline/segments?**', route => route.fulfill({ json: { segments: [segment] } }));
  await page.route('**/api/timeline/segments-by-ids?**', route => route.fulfill({ json: {
    segments: [segment], multi_stream: false,
    start_time: new Date(segment.start_timestamp * 1000).toISOString(),
    end_time: new Date(segment.end_timestamp * 1000).toISOString(),
  } }));
  const at = segment.start_timestamp + 30;
  const box = { x: 0.1, y: 0.2, width: 0.3, height: 0.4 };
  await page.route('**/api/detection/results/**', route => route.fulfill({ json: {
    timestamp: '2026-03-08 12:00:30', stream_status: 'Running',
    detections: [
      { label: 'bird', confidence: 0.82, ...box, timestamp: at, end_timestamp: at,
        observation: { id: OWNER_ID, owner_confirmed: true,
          classification: annotation('owner_confirmed', 'Carolina Chickadee', 1) } },
      { label: 'bird', confidence: 0.71, ...box, timestamp: at + 5, end_timestamp: at + 5,
        observation: { id: AUTO_ID, owner_confirmed: false,
          classification: annotation('automated', 'House Finch', 0.88) } },
      { label: 'person', confidence: 0.9, ...box, timestamp: at + 9, end_timestamp: at + 9 },
    ],
  } }));
  await page.route('**/api/recordings/**', async route => {
    const url = new URL(route.request().url());
    if (/\/api\/recordings\/play\/\d+$/.test(url.pathname)) {
      return route.fulfill(url.searchParams.get('prepare') === '1'
        ? { json: { status: 'ready' } } : { status: 204, body: '' });
    }
    if (/\/api\/recordings\/\d+\/tags$/.test(url.pathname)) return route.fulfill({ json: { tags: [] } });
    if (/\/api\/recordings\/\d+$/.test(url.pathname)) {
      return route.fulfill({ json: {
        id: segment.id, stream, protected: false,
        start_time: new Date(segment.start_timestamp * 1000).toISOString(),
        end_time: new Date(segment.end_timestamp * 1000).toISOString(),
      } });
    }
    return route.continue();
  });
}

test.describe('Detection classifications @ui @timeline', () => {
  test.beforeEach(async ({ page }) => {
    await login(page, USERS.admin);
  });

  test('timeline shows the detector label next to the linked classification', async ({ page }) => {
    const stream = 'feeder';
    const date = '2026-03-08';
    const segment: Segment = { id: 901, stream, start_timestamp: localTimestamp(date, '12:00:00'),
      end_timestamp: localTimestamp(date, '12:05:00') };
    await mockTimeline(page, stream, segment);
    await page.goto(`/timeline.html?stream=${stream}&date=${date}&time=12:00:30`, { waitUntil: 'domcontentloaded' });
    await expect(new TimelinePage(page).timelineContainer).toBeVisible();

    const rows = page.getByTestId('detection-classification');
    await expect(rows).toHaveCount(2);   // the unlinked "person" detection has no row
    const owner = rows.nth(0);
    await expect(owner).toContainText('Detector: bird (82%)');
    await expect(owner).toContainText('Carolina Chickadee (100%)');
    await expect(owner).toContainText('via yawamf');
    await expect(owner.getByTestId('detection-classification-owner')).toHaveText('Owner confirmed');
    await expect(owner.getByRole('link', { name: 'Observation' }))
      .toHaveAttribute('href', `/api/observations/${OWNER_ID}`);

    const automated = rows.nth(1);
    await expect(automated).toContainText('Detector: bird (71%)');
    await expect(automated).toContainText('House Finch (88%)');
    await expect(automated.getByTestId('detection-classification-owner')).toHaveCount(0);
    await expect(automated.getByRole('link', { name: 'Observation' }))
      .toHaveAttribute('href', `/api/observations/${AUTO_ID}`);
  });
});
