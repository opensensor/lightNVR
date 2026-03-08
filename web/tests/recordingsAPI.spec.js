import { recordingsAPI } from '../js/components/preact/recordings/recordingsAPI.jsx';

jest.mock('../js/components/preact/ToastContainer.jsx', () => ({
  showStatusMessage: jest.fn()
}));

jest.mock('../js/fetch-utils.js', () => ({
  fetchJSON: jest.fn(),
  enhancedFetch: jest.fn()
}));

jest.mock('../js/query-client.js', () => ({
  useQuery: jest.fn(),
  useMutation: jest.fn(),
  useQueryClient: jest.fn(),
  usePostMutation: jest.fn()
}));

const { fetchJSON, enhancedFetch } = jest.requireMock('../js/fetch-utils.js');
const { showStatusMessage } = jest.requireMock('../js/components/preact/ToastContainer.jsx');

describe('recordingsAPI', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    global.window = global.window || {};
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  test('checkRecordingHasDetections queries detection results with Unix seconds', async () => {
    const recording = {
      id: 42,
      stream: 'front-yard',
      start_time: '2026-02-18T05:00:00Z',
      end_time: '2026-02-18T05:01:00Z'
    };
    const start = recordingsAPI.parseRecordingTimestamp(recording.start_time);
    const end = recordingsAPI.parseRecordingTimestamp(recording.end_time);

    fetchJSON.mockResolvedValue({ detections: [{ label: 'person' }] });

    await expect(recordingsAPI.checkRecordingHasDetections(recording)).resolves.toBe(true);
    expect(fetchJSON).toHaveBeenCalledWith(
      `/api/detection/results/front-yard?start=${start}&end=${end}`,
      expect.objectContaining({ timeout: 10000, retries: 1, retryDelay: 500 })
    );
  });

  test('getRecordingDetections returns detections and queries with Unix seconds', async () => {
    const recording = {
      id: 77,
      stream: 'driveway',
      start_time: '2026-02-18T05:02:00Z',
      end_time: '2026-02-18T05:03:30Z'
    };
    const start = recordingsAPI.parseRecordingTimestamp(recording.start_time);
    const end = recordingsAPI.parseRecordingTimestamp(recording.end_time);
    const detections = [{ label: 'car' }];

    fetchJSON.mockResolvedValue({ detections });

    await expect(recordingsAPI.getRecordingDetections(recording)).resolves.toEqual(detections);
    expect(fetchJSON).toHaveBeenCalledWith(
      `/api/detection/results/driveway?start=${start}&end=${end}`,
      expect.objectContaining({ timeout: 15000, retries: 1, retryDelay: 1000 })
    );
  });

  test('deleteSelectedRecordingsHttp delegates response handling to shared helper', async () => {
    const response = { json: jest.fn() };
    const expected = { succeeded: 2, failed: 0 };

    enhancedFetch.mockResolvedValue(response);
    const helperSpy = jest
      .spyOn(recordingsAPI, 'handleBatchDeleteResponse')
      .mockResolvedValue(expected);

    await expect(recordingsAPI.deleteSelectedRecordingsHttp([5, 9])).resolves.toEqual(expected);

    expect(enhancedFetch).toHaveBeenCalledWith(
      '/api/recordings/batch-delete',
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({ ids: [5, 9] }),
        timeout: 60000,
        retries: 1,
        retryDelay: 2000
      })
    );
    expect(helperSpy).toHaveBeenCalledWith(response);
  });

  test('handleBatchDeleteResponse polls async batch jobs and shows status', async () => {
    const response = {
      json: jest.fn().mockResolvedValue({ job_id: 'job-123' })
    };
    const finalResult = { succeeded: 3, failed: 1 };
    const pollSpy = jest
      .spyOn(recordingsAPI, 'pollBatchDeleteProgress')
      .mockResolvedValue(finalResult);

    await expect(recordingsAPI.handleBatchDeleteResponse(response)).resolves.toEqual(finalResult);

    expect(pollSpy).toHaveBeenCalledWith('job-123');
    expect(showStatusMessage).toHaveBeenCalledWith(
      expect.stringContaining('Deleted 3 recordings')
    );
  });
});