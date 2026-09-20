CREATE TABLE bools (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    bool BOOLEAN NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id)
);