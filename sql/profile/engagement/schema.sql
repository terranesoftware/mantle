CREATE TABLE engagements (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    arrangement ARRANGEMENT NOT NULL,
    employer TEXT NOT NULL,
    employment EMPLOYMENT NOT NULL,
    location LOCATION,
    period PERIOD NOT NULL,
    title TEXT NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id)
);