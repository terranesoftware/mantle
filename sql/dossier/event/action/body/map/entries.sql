CREATE TABLE entries (
    id BIGSERIAL PRIMARY KEY,
    map BIGINT NOT NULL,

    contains BIGINT NOT NULL,
    name TEXT NOT NULL,
    position BIGINT NOT NULL,

    FOREIGN KEY (map) REFERENCES maps (id)
);