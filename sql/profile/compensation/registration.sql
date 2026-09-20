CREATE TABLE registrations (
    id BIGSERIAL PRIMARY KEY,
    compensation BIGINT NOT NULL,

    scheme TEXT NOT NULL,
    identifier TEXT NOT NULL,
    jurisdiction LOCATION NOT NULL,

    FOREIGN KEY (compensation) REFERENCES compensations (id)
);