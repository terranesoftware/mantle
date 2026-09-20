CREATE SCHEMA destination;

CREATE TABLE destinations (
    id BIGSERIAL PRIMARY KEY,
    compensation BIGINT NOT NULL,

    kind DESTINATION_KIND NOT NULL,

    FOREIGN KEY (compensation) REFERENCES compensations (id)
);