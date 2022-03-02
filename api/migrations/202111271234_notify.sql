CREATE OR REPLACE FUNCTION notify_user_session_insert ()
    RETURNS TRIGGER
    AS $$
DECLARE
    ROW RECORD;
    output text;
BEGIN
    PERFORM
        pg_notify('user_session_inserted', CAST(NEW.id AS text));
    RETURN NULL;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER trigger_user_session_insert
    AFTER INSERT OR UPDATE OR DELETE ON user_session
    FOR EACH ROW
    EXECUTE PROCEDURE notify_user_session_insert ();

