ALTER TABLE health_data DROP COLUMN created_at;

ALTER TABLE user_session ADD COLUMN has_accepted_terms_conditions BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS user_interaction
(
    id                  uuid PRIMARY KEY,
    user_session_id     uuid NOT NULL,
    screen_id           INTEGER NOT NULL,
    started_at          TIMESTAMPTZ NOT NULL,
    ended_at            TIMESTAMPTZ NOT NULL,
    CONSTRAINT user_interaction_us_fk FOREIGN KEY (user_session_id) REFERENCES user_session(id)
);

CREATE TABLE IF NOT EXISTS retake
(
    user_interaction_id uuid NOT NULL,
    retaked_at          TIMESTAMPTZ NOT NULL,
    CONSTRAINT retake_ui_fk FOREIGN KEY (user_interaction_id) REFERENCES user_interaction(id)
);