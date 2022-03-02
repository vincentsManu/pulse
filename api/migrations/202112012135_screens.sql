CREATE TYPE SCREEN AS ENUM (
    'welcome',
    'termsandconditions',
    'privacy',
    'entername'
);

ALTER TABLE user_interaction
    DROP COLUMN screen_id;

ALTER TABLE user_interaction
    ADD COLUMN screen_id SCREEN NOT NULL DEFAULT 'welcome';

