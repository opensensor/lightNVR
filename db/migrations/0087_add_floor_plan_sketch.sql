-- Vector layout sketch (rooms, walls, areas, labels) drawn on an operator
-- floor plan. Stored as canonical JSON produced by the API after validation.

-- migrate:up

ALTER TABLE operator_floor_plans ADD COLUMN sketch_json TEXT;

-- migrate:down

ALTER TABLE operator_floor_plans DROP COLUMN sketch_json;
