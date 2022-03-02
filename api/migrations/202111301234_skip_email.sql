ALTER TABLE user_interaction
    ADD COLUMN has_been_skipped BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE user_session
    ADD COLUMN has_been_printed BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE user_session
    ADD COLUMN has_been_emailed BOOLEAN NOT NULL DEFAULT FALSE;

