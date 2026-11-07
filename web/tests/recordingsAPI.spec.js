import {ContractValidationError} from '../js/api/contractErrors.ts';
import { recordingsAPI } from '../js/components/preact/recordings/recordingsAPI.tsx';

jest.mock('../js/components/preact/ToastContainer.tsx', () => ({
  showStatusMessage: jest.fn()
}));

jest.mock('../js/i18n.ts', () => {
  const english = require('../public/locales/en.json');
  return {
    t: (key, params = {}) => (english[key] || key).replace(/\{([^}]+)\}/g, (_, name) => String(params[name] ?? '')),
  };
});

jest.mock('../js/fetch-utils.ts', () => ({
  fetchJSON: jest.fn(),
  enhancedFetch: jest.fn()
}));

jest.mock('../js/query-client.ts', () => ({
  useQuery: jest.fn(),
  useMutation: jest.fn(),
  useQueryClient: jest.fn(),
  usePostMutation: jest.fn()
}));

const { fetchJSON, enhancedFetch } = require('../js/fetch-utils.ts');
const { showStatusMessage } = require('../js/components/preact/ToastContainer.tsx');
const detection = (label) => ({label,confidence:0.9,x:0,y:0,width:1,height:1,timestamp:1771390800,end_timestamp:1771390800});
const detectionResponse = (detections) => new Response(JSON.stringify({detections,timestamp:'2026-02-18 05:00:00',stream_status:'Running'}),{status:200,headers:{'Content-Type':'application/json'}});

describe('recordingsAPI', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    fetchJSON.mockReset();
    enhancedFetch.mockReset();
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  test('tag requests preserve an exact large timeline recording ID', async () => {
    const id = '9007199254740993';
    fetchJSON.mockResolvedValue({tags: ['event']});
    await expect(recordingsAPI.getRecordingTags(id)).resolves.toEqual(['event']);
    expect(fetchJSON).toHaveBeenLastCalledWith(`/api/recordings/${id}/tags`, expect.any(Object));
    await recordingsAPI.setRecordingTags(id, ['event']);
    expect(fetchJSON).toHaveBeenLastCalledWith(`/api/recordings/${id}/tags`, expect.objectContaining({
      method: 'PUT', body: JSON.stringify({tags: ['event']}),
    }));
  });

  test.each(['invalid_media', 'no_video', 'decode_error'])('explains %s without opening a broken player', playback_issue => {
    const modal = jest.fn();
    recordingsAPI.playRecording({id:42, playback_issue}, modal);
    expect(modal).not.toHaveBeenCalled();
    expect(showStatusMessage).toHaveBeenCalledWith(expect.stringContaining('Download the original'));
  });

  test.each(['probe_limit', 'decoder_unavailable', 'probe_error', 'future_issue', ''])('allows a playback attempt for %s', playback_issue => {
    const modal = jest.fn();
    recordingsAPI.playRecording({id:42, stream:'Entry', start_time:'2026-11-01T10:00:00Z', playback_issue}, modal);
    expect(modal).toHaveBeenCalledWith('/api/recordings/play/42', expect.any(String), '/api/recordings/download/42');
    expect(showStatusMessage).not.toHaveBeenCalled();
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

    enhancedFetch.mockImplementation(async()=>detectionResponse([detection('person')]));

    await expect(recordingsAPI.checkRecordingHasDetections(recording)).resolves.toBe(true);
    expect(enhancedFetch).toHaveBeenCalledWith(
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

    enhancedFetch.mockImplementation(async()=>detectionResponse([]));

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

  test('checkRecordingHasDetections returns false when timestamps are invalid', async () => {
    const recording = {
      id: 44,
      stream: 'front-yard',
      start_time: 'invalid-timestamp',
      end_time: 'invalid-timestamp'
    };

    await expect(recordingsAPI.checkRecordingHasDetections(recording)).resolves.toBe(false);
  });

  test('checkRecordingHasDetections returns false when enhancedFetch throws an error', async () => {
    const recording = {
      id: 45,
      stream: 'front-yard',
      start_time: '2026-02-18T08:00:00Z',
      end_time: '2026-02-18T08:01:00Z'
    };

    enhancedFetch.mockRejectedValue(new Error('network error'));

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
    const detections = [detection('car')];

    enhancedFetch.mockImplementation(async()=>detectionResponse(detections));

    await expect(recordingsAPI.getRecordingDetections(recording)).resolves.toEqual(detections);
    expect(enhancedFetch).toHaveBeenCalledWith(
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
    expect(enhancedFetch).not.toHaveBeenCalled();
  });

  test('getRecordingDetections returns empty array when timestamp parsing fails', async () => {
    const recording = {
      id: 89,
      stream: 'backyard',
      start_time: 'not-a-timestamp',
      end_time: '2026-02-18T05:30:00Z'
    };

    const result = await recordingsAPI.getRecordingDetections(recording);

    expect(result).toEqual([]);
    expect(enhancedFetch).not.toHaveBeenCalled();
  });

  test('getRecordingDetections returns empty array when enhancedFetch rejects', async () => {
    const recording = {
      id: 90,
      stream: 'side-yard',
      start_time: '2026-02-18T05:20:00Z',
      end_time: '2026-02-18T05:21:00Z'
    };

    enhancedFetch.mockRejectedValue(new Error('network error'));

    const result = await recordingsAPI.getRecordingDetections(recording);

    expect(result).toEqual([]);
  });

  test('getRecordingDetections reports a contract error before its tolerant empty fallback', async () => {
    const recording = {
      id: 91,
      stream: 'garage',
      start_time: '2026-02-18T05:30:00Z',
      end_time: '2026-02-18T05:31:00Z'
    };

    const errorLog=jest.spyOn(console,'error').mockImplementation(()=>{});
    enhancedFetch.mockImplementation(async()=>new Response('{}',{status:200,headers:{'Content-Type':'application/json'}}));

    const result = await recordingsAPI.getRecordingDetections(recording);

    expect(result).toEqual([]);
    expect(errorLog).toHaveBeenCalledWith('Error getting detections:',expect.any(ContractValidationError));
    expect(errorLog.mock.calls[0][1]).toMatchObject({phase:'response'});
  });

  test('filter option loaders retain their tolerant default behavior', async () => {
    fetchJSON.mockRejectedValue(new Error('metadata unavailable'));

    await expect(recordingsAPI.getAllDetectionLabels()).resolves.toEqual([]);
    await expect(recordingsAPI.getAllRecordingTags()).resolves.toEqual([]);
  });

  test('filter option loaders can surface failures to lazy-loading UI', async () => {
    const error = new Error('metadata unavailable');
    fetchJSON.mockRejectedValue(error);

    await expect(recordingsAPI.getAllDetectionLabels({ throwOnError: true })).rejects.toBe(error);
    await expect(recordingsAPI.getAllRecordingTags({ throwOnError: true })).rejects.toBe(error);
  });

  test('getAllDetectionLabels gives the label list one generous attempt without retries', async () => {
    fetchJSON.mockResolvedValue({ labels: ['car', 'person'] });

    await expect(recordingsAPI.getAllDetectionLabels()).resolves.toEqual(['car', 'person']);
    expect(fetchJSON).toHaveBeenCalledWith('/api/recordings/detection-labels', {
      timeout: 15000,
      retries: 0
    });
  });

  test.each([true, false])('selected deletion preserves exact keys with modal=%s', async useModal => {
    const priorWindow = global.window;
    const expected = {succeeded: 2, failed: 0};
    const modal = jest.fn().mockResolvedValue(expected);
    global.window = useModal ? {batchDeleteRecordingsByHttpRequest: modal} : {};
    const direct = jest.spyOn(recordingsAPI, 'deleteSelectedRecordingsHttp').mockResolvedValue(expected);
    try {
      await expect(recordingsAPI.deleteSelectedRecordings({
        '42': true, '9007199254740993': true, '9007199254740992': false,
      })).resolves.toEqual(expected);
      if (useModal) {
        expect(modal).toHaveBeenCalledWith({ids: ['42', '9007199254740993']});
        expect(direct).not.toHaveBeenCalled();
      } else expect(direct).toHaveBeenCalledWith(['42', '9007199254740993']);
    } finally {global.window = priorWindow;}
  });

  test('direct deletion sends large IDs as unchanged strings', async () => {
    const response = {json: jest.fn()};
    enhancedFetch.mockResolvedValue(response);
    jest.spyOn(recordingsAPI, 'handleBatchDeleteResponse').mockResolvedValue({succeeded: 1, failed: 0});
    await recordingsAPI.deleteSelectedRecordingsHttp(['9007199254740993']);
    expect(enhancedFetch).toHaveBeenCalledWith('/api/recordings/batch-delete', expect.objectContaining({
      body: JSON.stringify({ids: ['9007199254740993']}),
    }));
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

describe('archive batch deletion outcomes', () => {
  const { useMutation, useQueryClient } = require('../js/query-client.ts');
  beforeEach(() => {
    jest.clearAllMocks();
    fetchJSON.mockReset();
    global.window = {};
  });
  afterEach(() => { jest.restoreAllMocks(); delete global.window; });

  test('mutation waits for the durable job outcome before resolving and refreshing', async () => {
    const invalidateQueries = jest.fn();
    useQueryClient.mockReturnValue({ invalidateQueries });
    useMutation.mockImplementation(options => options);
    fetchJSON.mockResolvedValueOnce({ job_id: 'archive-job', status: 'processing' });
    let finish;
    const poll = jest.spyOn(recordingsAPI, 'pollBatchDeleteProgress').mockReturnValue(new Promise(resolve => { finish = resolve; }));
    const mutation = recordingsAPI.hooks.useBatchDeleteRecordings();
    const outcome = { succeeded: 2, pending_deletions: 1, failed: 1 };
    let settled = false;
    const pending = mutation.mutationFn({ ids: [1, 2, 3] }).then(result => { settled = true; return result; });
    await Promise.resolve();
    expect(poll).toHaveBeenCalledWith('archive-job');
    expect(settled).toBe(false);
    expect(invalidateQueries).not.toHaveBeenCalled();
    expect(showStatusMessage).not.toHaveBeenCalled();
    finish(outcome);
    await expect(pending).resolves.toEqual(outcome);
    mutation.onSuccess(outcome);
    expect(invalidateQueries).toHaveBeenCalledWith({ queryKey: ['recordings'] });
    expect(showStatusMessage).toHaveBeenCalledWith('Accepted 2 deletions; 1 await storage cleanup; failed to delete 1');
  });

  test('HTTP fallback retains failures alongside pending cleanup', async () => {
    const outcome = { succeeded: 4, failed: 2, pending_deletions: 3 };
    jest.spyOn(recordingsAPI, 'pollBatchDeleteProgress').mockResolvedValue(outcome);
    await expect(recordingsAPI.handleBatchDeleteResponse({ json: async () => ({ job_id: 'mixed' }) })).resolves.toEqual(outcome);
    expect(showStatusMessage).toHaveBeenCalledWith('Accepted 4 deletions; 3 await storage cleanup; failed to delete 2');
  });

  test('job polling preserves pending cleanup and failure counts', async () => {
    fetchJSON.mockResolvedValueOnce({ complete: true, succeeded: 3, pending_deletions: 2, failed: 1 });
    await expect(recordingsAPI.pollBatchDeleteProgress('finished')).resolves.toEqual({ succeeded: 3, pending_deletions: 2, failed: 1 });
  });
});

describe('batch deletion polling deadline', () => {
  let priorWindow;
  beforeEach(() => {
    jest.useFakeTimers();
    fetchJSON.mockReset();
    priorWindow = global.window;
    global.window = {updateBatchDeleteProgress: jest.fn()};
    for (const method of ['log', 'debug', 'warn', 'error']) {
      jest.spyOn(console, method).mockImplementation(() => {});
    }
  });
  afterEach(() => {
    jest.clearAllTimers();
    jest.useRealTimers();
    jest.restoreAllMocks();
    global.window = priorWindow;
  });

  async function expiresAtDeadline() {
    let settled = false;
    const result = recordingsAPI.pollBatchDeleteProgress('slow-job').catch(error => error)
      .then(value => { settled = true; return value; });
    await jest.advanceTimersByTimeAsync(119999);
    expect(settled).toBe(false);
    await jest.advanceTimersByTimeAsync(1);
    expect(await result).toEqual(new Error('Batch delete operation timed out'));
    const calls = fetchJSON.mock.calls.length;
    await jest.advanceTimersByTimeAsync(60000);
    expect(fetchJSON).toHaveBeenCalledTimes(calls);
    expect(jest.getTimerCount()).toBe(0);
  }

  test('pending responses and exponential backoff share a two-minute deadline', async () => {
    fetchJSON.mockResolvedValue({complete: false});
    await expiresAtDeadline();
    expect(fetchJSON.mock.calls.length).toBeGreaterThan(1);
  });

  test('a stalled response is aborted and cannot publish a late completion', async () => {
    let finish;
    let signal;
    fetchJSON.mockImplementation((_url, options) => {
      signal = options.signal;
      return new Promise(resolve => {finish = resolve;});
    });
    await expiresAtDeadline();
    expect(signal.aborted).toBe(true);
    finish({complete: true, succeeded: 1});
    await jest.advanceTimersByTimeAsync(0);
    expect(window.updateBatchDeleteProgress).not.toHaveBeenCalled();
    expect(fetchJSON).toHaveBeenCalledTimes(1);
  });

  test('real fetch timeouts and internal retries cannot extend the deadline', async () => {
    const actual = jest.requireActual('../js/fetch-utils.ts');
    fetchJSON.mockImplementation(actual.fetchJSON);
    const signals = [];
    const fetch = jest.spyOn(global, 'fetch').mockImplementation((_url, {signal}) => {
      signals.push(signal);
      return new Promise((_, reject) => {
        signal.addEventListener('abort', () => reject(new DOMException('Aborted', 'AbortError')), {once: true});
      });
    });
    await expiresAtDeadline();
    expect(fetch.mock.calls.length).toBeGreaterThan(fetchJSON.mock.calls.length);
    expect(signals.every(signal => signal.aborted)).toBe(true);
  });

  test('a completed job after a transient failure clears all deadline timers', async () => {
    fetchJSON.mockRejectedValueOnce(new Error('temporary failure'))
      .mockResolvedValueOnce({complete: true, succeeded: 3, pending_deletions: 2, failed: 1});
    const result = recordingsAPI.pollBatchDeleteProgress('recovered');
    await jest.advanceTimersByTimeAsync(1000);
    await expect(result).resolves.toEqual({succeeded: 3, pending_deletions: 2, failed: 1});
    expect(jest.getTimerCount()).toBe(0);
    await jest.advanceTimersByTimeAsync(120000);
    expect(fetchJSON).toHaveBeenCalledTimes(2);
  });
});

describe('recording detection URL encoding', () => {
  afterEach(() => jest.restoreAllMocks());
  test.each(['checkRecordingHasDetections', 'getRecordingDetections'])('%s encodes stream path characters', async method => {
    enhancedFetch.mockReset();
    enhancedFetch.mockImplementation(async () => detectionResponse([detection('person')]));
    const stream = 'Front #1?zone=2%/east';
    const recording = {id: 42, stream, start_time: '2026-02-18T05:00:00Z', end_time: '2026-02-18T05:01:00Z'};
    const result = await recordingsAPI[method](recording);
    expect(result).toEqual(method === 'checkRecordingHasDetections' ? true : [detection('person')]);
    const url = enhancedFetch.mock.calls[0][0];
    expect(url.split('?')[0]).toBe(`/api/detection/results/${encodeURIComponent(stream)}`);
    expect(new URL(url, 'http://localhost').searchParams.has('zone')).toBe(false);
  });
});
