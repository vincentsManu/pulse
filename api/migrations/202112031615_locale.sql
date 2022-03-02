CREATE TYPE LOCALE AS ENUM (
    'en_US',
    'zh_Hans_HK',
    'zh_Hant_HK'
);

ALTER TABLE user_session
    ADD COLUMN locale LOCALE NOT NULL DEFAULT 'en_US';

