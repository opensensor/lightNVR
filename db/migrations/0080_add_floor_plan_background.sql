-- Optional uploaded background image for operator floor plans.

-- migrate:up

ALTER TABLE operator_floor_plans ADD COLUMN background_mime TEXT
    CHECK (background_mime IS NULL OR background_mime IN ('image/png', 'image/jpeg'));

-- migrate:down

ALTER TABLE operator_floor_plans DROP COLUMN background_mime;
