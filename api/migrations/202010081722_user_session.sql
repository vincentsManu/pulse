
CREATE TABLE IF NOT EXISTS user_session
(
    id              uuid PRIMARY KEY,
    has_consented   BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);