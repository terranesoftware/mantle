CREATE TABLE destination.addresses (
    id BIGSERIAL PRIMARY KEY,
    destination BIGINT NOT NULL,

    kind ADDRESS_KIND NOT NULL,
    
    FOREIGN KEY (destination) REFERENCES destinations (id)
);