CREATE TABLE identity.addresses (
    id BIGSERIAL PRIMARY KEY,
    identity BIGINT NOT NULL,

    lines TEXT[] NOT NULL,
    location LOCATION NOT NULL,
    postcode TEXT NOT NULL,
    
    FOREIGN KEY (identity) REFERENCES identities (id)
);