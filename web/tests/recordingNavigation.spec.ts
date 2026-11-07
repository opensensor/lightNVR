import {recordingViewLinks, recordingsReturnUrl} from '../js/components/preact/recordings/recordingNavigation';
const params = (href: string) => new URL(href, 'https://nvr.invalid/').searchParams;

test('all workspaces retain a recordings return destination including sort, page and filters', () => {
  const original = 'recordings.html?stream=Gate&dateRange=custom&startDate=2026-11-07&tag=incident&page=3&sort=duration&order=asc&view=grid';
  const links = recordingViewLinks({recordingsUrl:original, streams:['Gate'],cameras:['camera-a'],start:1794013200,end:1794016800,cursor:1794013300});
  expect(params(links.table).get('view')).toBe('table');
  expect(params(links.grid).get('page')).toBe('3');
  expect(params(links.timeline).get('camera_uuid')).toBe('camera-a');
  for (const href of [links.timeline,links.investigation,links.plates]) {
    expect(params(href).get('return_to')).toBe(original);
    const back=recordingViewLinks({recordingsUrl:params(href).get('return_to')});
    expect(back.grid).toBe(original);
  }
});

test('new navigation uses local dates and preserves epoch-zero timestamps', () => {
  const date = new Date(2026,10,7,13,14,15);
  const start=date.getTime()/1000;
  const links=recordingViewLinks({streams:['Gate'],start,end:start+60,cursor:start+5});
  expect(params(links.timeline).get('date')).toBe('2026-11-07');
  expect(params(links.timeline).get('time')).toBe('13:14:20');
  expect(params(links.grid).get('startTime')).toBe('13:14:15');
  expect(params(recordingViewLinks({start:0,end:60,cursor:0}).investigation).get('cursor')).toBe('0');
});

test.each(['https://evil.invalid/recordings.html','//evil.invalid/recordings.html','javascript:alert(1)','recordings.html.evil','timeline.html'])('rejects an external or unrelated return URL: %s', value=>{
  expect(recordingsReturnUrl(value)).toBeUndefined();
  expect(recordingViewLinks({recordingsUrl:value}).table).toBe('recordings.html?view=table');
});
