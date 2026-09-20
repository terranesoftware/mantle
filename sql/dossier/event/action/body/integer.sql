CREATE TABLE integers (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    integer BIGINT NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id)
);