jest.mock('../js/fetch-utils.js', () => ({ fetchJSON: jest.fn() }));
const { fetchJSON } = require('../js/fetch-utils.js');
const { getSettings, getGo2rtcBaseUrl, getGo2rtcWebSocketUrl } = require('../js/utils/settings-utils.js');

beforeEach(() => {
  global.window = { location: {
    protocol: 'http:', hostname: 'camera.test', host: 'camera.test:8080',
    origin: 'http://camera.test:8080'
  } };
});

test.each(['/foo', '/nested/go2rtc', '', '/go2rtc'])('HTTP playback uses the public proxy with upstream prefix %j', async (prefix) => {
  fetchJSON.mockResolvedValue({ go2rtc_api_port: 31984, go2rtc_base_path: prefix });
  await getSettings(true);
  expect(await getGo2rtcBaseUrl()).toBe(`http://camera.test:8080/go2rtc`);
  expect(await getGo2rtcWebSocketUrl()).toBe('ws://camera.test:8080/go2rtc');
});

test('HTTPS playback uses the public HTTP and WebSocket proxy', async () => {
  window.location.protocol = 'https:';
  window.location.origin = 'https://camera.test:8080';
  fetchJSON.mockResolvedValue({ go2rtc_base_path: '/foo' });
  await getSettings(true);
  expect(await getGo2rtcBaseUrl()).toBe('https://camera.test:8080/go2rtc');
  expect(await getGo2rtcWebSocketUrl()).toBe('wss://camera.test:8080/go2rtc');
});

test('older servers retain the default prefix', async () => {
  fetchJSON.mockResolvedValue({});
  await getSettings(true);
  expect(await getGo2rtcBaseUrl()).toBe('http://camera.test:8080/go2rtc');
});
