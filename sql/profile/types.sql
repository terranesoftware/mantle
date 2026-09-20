CREATE TYPE location AS (
    city TEXT,
    country TEXT,
    county TEXT,
    state TEXT
);

CREATE TYPE period AS (
    start DATE NOT NULL,
    end DATE
);