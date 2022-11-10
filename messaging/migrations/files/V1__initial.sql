CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

BEGIN;

CREATE TABLE IF NOT EXISTS public.sessions
(
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL,
    addr text NOT NULL,
    created_at timestamp without time zone DEFAULT (NOW() at time zone 'utc') NOT NULL,
    PRIMARY KEY (id, user_id)
);

END;