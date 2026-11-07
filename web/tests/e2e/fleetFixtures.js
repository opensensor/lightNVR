/** Complete native location responses for component browser scenarios. */
const { validateFleetLocationsResponse } = require('../../generated/api/validators.js');

function fleetLocations(locations) {
  const body = {
    locations: locations.map(location => ({
      parent_uuid: null,
      type: 'site',
      sort_order: 0,
      is_system: false,
      metadata: {},
      child_count: 0,
      camera_count: 0,
      created_at: 1,
      updated_at: 1,
      ...location,
    })),
    count: locations.length,
  };
  // Fail during fixture construction, before a missing UI element times out.
  if (!validateFleetLocationsResponse(body)) {
    throw new Error(`Invalid FleetLocationsResponse fixture: ${JSON.stringify(validateFleetLocationsResponse.errors)}`);
  }
  return body;
}

module.exports = { fleetLocations };

/** Complete workspace rows; invalid fixture data fails before browser startup. */
function floorPlan(overrides) {
  const value = {
    uuid: '10000000-0000-4000-8000-000000000001', name: 'Owned plan',
    location_uuid: null, parent_plan_uuid: null, canvas_width: 1200, canvas_height: 800,
    cameras: [], sketch: null, background_mime: null, partially_authorized: false,
    revision: 1, created_at: 1, updated_at: 1, ...overrides,
  };
  value.cameras = value.cameras.map(camera => ({ x: 0.5, y: 0.5, rotation: 0, fov: 90, ...camera }));
  const { validateFloorPlan } = require('../../generated/api/validators.js');
  if (!validateFloorPlan(value)) throw new Error(`Invalid FloorPlan fixture: ${JSON.stringify(validateFloorPlan.errors)}`);
  return value;
}
function liveLayout(overrides) {
  const value = {
    uuid: '20000000-0000-4000-8000-000000000001', name: 'Owned layout',
    is_shared: false, location_uuid: null, availability: 'live', columns: 2, rows: 1,
    camera_slots: [], owned: true, partially_authorized: false, revision: 1,
    created_at: 1, updated_at: 1, ...overrides,
  };
  const { validateLiveLayout } = require('../../generated/api/validators.js');
  if (!validateLiveLayout(value)) throw new Error(`Invalid LiveLayout fixture: ${JSON.stringify(validateLiveLayout.errors)}`);
  return value;
}
module.exports.floorPlan = floorPlan;
module.exports.liveLayout = liveLayout;
