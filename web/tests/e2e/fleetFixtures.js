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
