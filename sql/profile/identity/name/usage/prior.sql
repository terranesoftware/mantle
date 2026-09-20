CREATE TABLE priors (
    id BIGSERIAL PRIMARY KEY,
    usage BIGINT NOT NULL,

    period PERIOD NOT NULL,

    FOREIGN KEY (usage) REFERENCES usages (id)
);