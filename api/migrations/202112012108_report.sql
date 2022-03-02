ALTER TABLE health_data
    DROP COLUMN report_date;

ALTER TABLE health_data
    ADD COLUMN report_date TIMESTAMPTZ NOT NULL DEFAULT NOW();

