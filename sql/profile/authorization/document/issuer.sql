CREATE TABLE issuers (
    id BIGSERIAL PRIMARY KEY,
    document BIGINT NOT NULL,
    
    name TEXT NOT NULL,
    jurisdiction LOCATION NOT NULL,

    FOREIGN KEY (document) REFERENCES documents (id)
);