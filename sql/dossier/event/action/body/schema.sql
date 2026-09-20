CREATE TABLE bodies (
    id BIGSERIAL PRIMARY KEY,
    action BIGINT NOT NULL,

    kind BODY_KIND NOT NULL,

    FOREIGN KEY (action) REFERENCES actions (id)
);