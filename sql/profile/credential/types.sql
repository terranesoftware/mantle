CREATE TYPE credential_kind AS ENUM (
    'education'
);

CREATE TYPE status AS ENUM (
    'completed',
    'enrolled',
    'dropped_out',
    'transferred',
    'withdrawn'
);