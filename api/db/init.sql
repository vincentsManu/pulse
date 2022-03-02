-- kubectl port-forward svc/mypostgres 5432:5432 -n postgres
-- psql -h localhost -p 5432 -U postgres -f db/.init.sql
-- user creation
CREATE USER manupulse_app WITH PASSWORD 'DB_PASSWORD';

CREATE USER pulstats_app WITH PASSWORD 'DB_PASSWORD';

CREATE DATABASE manupulseDB_SUFFIX;

GRANT ALL PRIVILEGES ON DATABASE manupulseDB_SUFFIX TO manupulse_app;

GRANT CONNECT ON DATABASE manupulseDB_SUFFIX TO pulstats_app;

GRANT SELECT ON ALL TABLES IN SCHEMA public TO pulstats_app;

