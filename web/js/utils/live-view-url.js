export function buildLiveViewHref(path, currentSearch = '', mode = '') {
  const params = new URLSearchParams(currentSearch);
  if (mode) params.set('mode', mode);
  else params.delete('mode');
  const query = params.toString();
  return query ? `${path}?${query}` : path;
}
