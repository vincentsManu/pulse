CREATE INDEX health_data_kiosk_location_index ON health_data (kiosk_location);

-- cannot run CONCURRENTLY in the migration from sqlx
