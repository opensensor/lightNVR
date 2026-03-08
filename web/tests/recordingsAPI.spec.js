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

const { fetchJSON, enhancedFetch } = require('../js/fetch-utils.js');
const { showStatusMessage } = require('../js/components/preact/ToastContainer.jsx');

describe('recordingsAPI', () => {
  beforeEach(() => {
    jest.clearAllMocks();
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

  test('checkRecordingHasDetections returns false when detections array is empty', async () => {
    const recording = {
      id: 43,
      stream: 'front-yard',
      start_time: '2026-02-18T06:00:00Z',
      end_time: '2026-02-18T06:01:00Z'
    };

    fetchJSON.mockResolvedValue({ detections: [] });

    await expect(recordingsAPI.checkRecordingHasDetections(recording)).resolves.toBe(false);
  });

  test('checkRecordingHasDetections returns false when recording is missing required fields', async () => {
    const incompleteRecording = {
      // id is missing
      stream: 'front-yard',
      start_time: '2026-02-18T07:00:00Z',
      end_time: '2026-02-18T07:01:00Z'
    };

    await expect(
      recordingsAPI.checkRecordingHasDetections(incompleteRecording)
    ).resolves.toBe(false);
  });

  test('checkRecordingHasDetections returns false when timestamp parsing fails', async () => {
    const recording = {
      id: 44,
      stream: 'front-yard',
      start_time: 'invalid-timestamp',
      end_time: 'invalid-timestamp'
    };

    const parseSpy = jest
      .spyOn(recordingsAPI, 'parseRecordingTimestamp')
      .mockImplementation(() => {
        throw new Error('parse error');
      });

    await expect(recordingsAPI.checkRecordingHasDetections(recording)).resolves.toBe(false);

    parseSpy.mockRestore();
  });

  test('checkRecordingHasDetections returns false when fetchJSON throws an error', async () => {
    const recording = {
      id: 45,
      stream: 'front-yard',
      start_time: '2026-02-18T08:00:00Z',
      end_time: '2026-02-18T08:01:00Z'
    };

    fetchJSON.mockRejectedValue(new Error('network error'));

    await expect(recordingsAPI.checkRecordingHasDetections(recording)).resolves.toBe(false);
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

  test('getRecordingDetections returns empty array when required recording fields are missing', async () => {
    const recording = {
      // stream is intentionally missing
      id: 88,
      start_time: '2026-02-18T05:10:00Z',
      end_time: '2026-02-18T05:11:00Z'
    };

    const result = await recordingsAPI.getRecordingDetections(recording);

    expect(result).toEqual([]);
    expect(fetchJSON).not.toHaveBeenCalled();
  });

  test('getRecordingDetections returns empty array when timestamp parsing fails', async () => {
    const recording = {
      id: 89,
      stream: 'backyard',
      start_time: 'invalid-start-timestamp',
      end_time: 'invalid-end-timestamp'
    };

    const parseSpy = jest
      .spyOn(recordingsAPI, 'parseRecordingTimestamp')
      .mockImplementation(() => {
        throw new Error('parse error');
      });

    const result = await recordingsAPI.getRecordingDetections(recording);

    expect(result).toEqual([]);
    expect(fetchJSON).not.toHaveBeenCalled();

    parseSpy.mockRestore();
  });

  test('getRecordingDetections returns empty array when fetchJSON rejects', async () => {
    const recording = {
      id: 90,
      stream: 'side-yard',
      start_time: '2026-02-18T05:20:00Z',
      end_time: '2026-02-18T05:21:00Z'
    };

    fetchJSON.mockRejectedValue(new Error('network error'));

    const result = await recordingsAPI.getRecordingDetections(recording);

    expect(result).toEqual([]);
  });

  test('getRecordingDetections returns empty array when response is missing detections property', async () => {
    const recording = {
      id: 91,
      stream: 'garage',
      start_time: '2026-02-18T05:30:00Z',
      end_time: '2026-02-18T05:31:00Z'
    };

    fetchJSON.mockResolvedValue({});

    const result = await recordingsAPI.getRecordingDetections(recording);

    expect(result).toEqual([]);
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
    const finalResult = { succeeded: 3, failed: 0 };
    const pollSpy = jest
      .spyOn(recordingsAPI, 'pollBatchDeleteProgress')
      .mockResolvedValue(finalResult);

    await expect(recordingsAPI.handleBatchDeleteResponse(response)).resolves.toEqual(finalResult);

    expect(pollSpy).toHaveBeenCalledWith('job-123');
    expect(showStatusMessage).toHaveBeenCalledWith(
      expect.stringContaining('Successfully deleted 3 recordings')
    );
  });

  test('handleBatchDeleteResponse returns direct result without polling when no job_id', async () => {
    const finalResult = { succeeded: 2, failed: 0 };
    const response = {
      json: jest.fn().mockResolvedValue(finalResult)
    };
    const pollSpy = jest.spyOn(recordingsAPI, 'pollBatchDeleteProgress');

    await expect(recordingsAPI.handleBatchDeleteResponse(response)).resolves.toEqual(finalResult);

    expect(pollSpy).not.toHaveBeenCalled();
    expect(showStatusMessage).toHaveBeenCalledWith(
      expect.stringContaining('Successfully deleted 2 recordings')
    );
  });
});