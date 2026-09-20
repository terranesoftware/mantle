CREATE TABLE varves (
    id BIGSERIAL PRIMARY KEY,

    account TEXT NOT NULL,
    from TIMESTAMPTZ NOT NULL
);

-- Dossiers
-- Table
CREATE TABLE dossiers (
    id BIGSERIAL PRIMARY KEY,
    varve BIGINT NOT NULL,

    from TIMESTAMPTZ NOT NULL,
    hash BYTEA NOT NULL,
    position BIGINT NOT NULL,
    to TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (varve) REFERENCES varves (id) ON DELETE CASCADE
);

-- Events
-- Table
CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    dossier BIGINT NOT NULL,

    from TIMESTAMPTZ NOT NULL,
    hash BYTEA NOT NULL,
    position BIGINT NOT NULL,
    to TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (dossier) REFERENCES dossiers (id) ON DELETE CASCADE
);

-- Actions
-- Table
CREATE TABLE actions (
    id BIGSERIAL PRIMARY KEY,
    event BIGINT NOT NULL,

    hash BYTEA NOT NULL,
    name TEXT NOT NULL,
    position BIGINT NOT NULL,
    source TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (event) REFERENCES events (id) ON DELETE CASCADE
);

-- Bodies
-- Types
CREATE TYPE body_kind AS ENUM (
    'boolean',
    'handle',
    'integer',
    'list',
    'map',
    'text'
);

-- Table
CREATE TABLE bodies (
    id BIGSERIAL PRIMARY KEY,
    action BIGINT NOT NULL,

    kind BODY_KIND NOT NULL,

    FOREIGN KEY (action) REFERENCES actions (id) ON DELETE CASCADE
);

-- Booleans
CREATE TABLE booleans (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    boolean BOOLEAN NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id) ON DELETE CASCADE
);

-- Handles
CREATE TABLE handles (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    handle TEXT NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id) ON DELETE CASCADE
);

-- Integers
CREATE TABLE integers (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    integer BIGINT NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id) ON DELETE CASCADE
);

-- Lists
CREATE TABLE lists (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    contains BIGINT[] NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id) ON DELETE CASCADE
);

-- Maps
-- Table
CREATE TABLE maps (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id) ON DELETE CASCADE
);

-- Entries
CREATE TABLE entries (
    id BIGSERIAL PRIMARY KEY,
    map BIGINT NOT NULL,

    contains BIGINT NOT NULL,
    name TEXT NOT NULL,
    position BIGINT NOT NULL,

    FOREIGN KEY (map) REFERENCES maps (id) ON DELETE CASCADE
);

-- Texts
CREATE TABLE texts (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    text TEXT NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id) ON DELETE CASCADE
);

-- Profiles
-- Types
CREATE TYPE location AS (
    city TEXT,
    country TEXT,
    county TEXT,
    state TEXT
);

CREATE TYPE period AS (
    start DATE NOT NULL,
    end DATE
);

-- Table
CREATE TABLE profiles (
    id BIGSERIAL PRIMARY KEY,
    varve BIGINT NOT NULL,

    FOREIGN KEY (varve) REFERENCES varves (id) ON DELETE CASCADE
);

-- Authorizations
-- Table
CREATE TABLE authorizations (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id) ON DELETE CASCADE
);

-- Documents
-- Table
CREATE TABLE documents (
    id BIGSERIAL PRIMARY KEY,
    authorization BIGINT NOT NULL,

    name TEXT NOT NULL,
    number TEXT,
    validity DATERANGE,

    FOREIGN KEY (authorization) REFERENCES authorizations (id) ON DELETE CASCADE
);

-- Issuers
CREATE TABLE issuers (
    id BIGSERIAL PRIMARY KEY,
    document BIGINT NOT NULL,
    
    name TEXT NOT NULL,
    jurisdiction LOCATION NOT NULL,

    FOREIGN KEY (document) REFERENCES documents (id) ON DELETE CASCADE
);

-- Compensations
-- Table
CREATE TABLE compensations (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id) ON DELETE CASCADE
);

-- Destinations
-- Types
CREATE TYPE destination_kind AS ENUM (
    'default',
    'retirement'
);

-- Schema
CREATE SCHEMA destination;

-- Table
CREATE TABLE destinations (
    id BIGSERIAL PRIMARY KEY,
    compensation BIGINT NOT NULL,

    kind DESTINATION_KIND NOT NULL,

    FOREIGN KEY (compensation) REFERENCES compensations (id) ON DELETE CASCADE
);

-- Addresses
-- Types
CREATE TYPE address_kind AS ENUM (
    'ach',
    'iban',
    'pix',
    'upi'
);

-- Table
CREATE TABLE destination.addresses (
    id BIGSERIAL PRIMARY KEY,
    destination BIGINT NOT NULL,

    kind ADDRESS_KIND NOT NULL,
    
    FOREIGN KEY (destination) REFERENCES destinations (id) ON DELETE CASCADE
);

-- ACHs
-- Types
CREATE TYPE account_kind AS ENUM (
    'checking',
    'savings'
);

-- Table
CREATE TABLE achs (
    id BIGSERIAL PRIMARY KEY,
    address BIGINT NOT NULL,

    account TEXT NOT NULL,
    kind ACCOUNT_KIND NOT NULL,
    routing TEXT NOT NULL,

    FOREIGN KEY (address) REFERENCES addresses (id) ON DELETE CASCADE
);

-- IBANs
CREATE TABLE ibans (
    id BIGSERIAL PRIMARY KEY,
    address BIGINT NOT NULL,

    iban TEXT NOT NULL,

    FOREIGN KEY (address) REFERENCES addresses (id) ON DELETE CASCADE
);

-- PIXs
CREATE TABLE pixs (
    id BIGSERIAL PRIMARY KEY,
    address BIGINT NOT NULL,

    identifier TEXT NOT NULL,

    FOREIGN KEY (address) REFERENCES addresses (id) ON DELETE CASCADE
);

-- UPIs
CREATE TABLE upis (
    id BIGSERIAL PRIMARY KEY,
    address BIGINT NOT NULL,

    vpa TEXT NOT NULL,

    FOREIGN KEY (address) REFERENCES addresses (id) ON DELETE CASCADE
);

-- Registrations
CREATE TABLE registrations (
    id BIGSERIAL PRIMARY KEY,
    compensation BIGINT NOT NULL,

    scheme TEXT NOT NULL,
    identifier TEXT NOT NULL,
    jurisdiction LOCATION NOT NULL,

    FOREIGN KEY (compensation) REFERENCES compensations (id) ON DELETE CASCADE
);

-- Credentials
-- Types
CREATE TYPE credential_kind AS ENUM (
    'education'
);

CREATE TYPE status AS ENUM (
    'completed',
    'enrolled',
    'dropped_out',
    'transferred',
    'withdrawn'
);

-- Table
CREATE TABLE credentials (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    kind CREDENTIAL_KIND NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id) ON DELETE CASCADE
);

-- Educations
CREATE TABLE educations (
    id BIGSERIAL PRIMARY KEY,
    credential BIGINT NOT NULL,

    degree TEXT NOT NULL,
    discipline TEXT NOT NULL,
    location LOCATION NOT NULL,
    school TEXT NOT NULL,
    status STATUS NOT NULL,

    FOREIGN KEY (credential) REFERENCES credentials (id) ON DELETE CASCADE
);

-- Engagements
-- Types
CREATE TYPE arrangement AS ENUM (
    'hybrid',
    'on_site',
    'remote'
);

CREATE TYPE employment AS ENUM (
    'apprenticeship',
    'contract',
    'internship',
    'full_time',
    'part_time'
);

-- Table
CREATE TABLE engagements (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    arrangement ARRANGEMENT NOT NULL,
    employer TEXT NOT NULL,
    employment EMPLOYMENT NOT NULL,
    location LOCATION,
    period PERIOD NOT NULL,
    title TEXT NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id) ON DELETE CASCADE
);

-- Identities
-- Schema
CREATE SCHEMA identity;

-- Table
CREATE TABLE identities (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    birth DATE NOT NULL,
    emails TEXT[] NOT NULL,
    phones TEXT[] NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id) ON DELETE CASCADE
);

-- Addresses
CREATE TABLE identity.addresses (
    id BIGSERIAL PRIMARY KEY,
    identity BIGINT NOT NULL,

    lines TEXT[] NOT NULL,
    location LOCATION NOT NULL,
    postcode TEXT NOT NULL,
    
    FOREIGN KEY (identity) REFERENCES identities (id) ON DELETE CASCADE
);

-- Names
-- Types
CREATE TYPE identifiers AS (
    primary TEXT,
    secondary TEXT
);

-- Table
CREATE TABLE names (
    id BIGSERIAL PRIMARY KEY,
    identity BIGINT NOT NULL,

    latin IDENTIFIERS,
    native IDENTIFIERS,

    FOREIGN KEY (identity) REFERENCES identities (id) ON DELETE CASCADE
);

-- Usage
-- Types
CREATE TYPE usage_kind AS ENUM (
    'legal',
    'prior',
    'used'
);

-- Table
CREATE TABLE usages (
    id BIGSERIAL PRIMARY KEY,
    name BIGINT NOT NULL,

    kind USAGE_KIND NOT NULL,

    FOREIGN KEY (name) REFERENCES names (id) ON DELETE CASCADE
);

-- Priors
CREATE TABLE priors (
    id BIGSERIAL PRIMARY KEY,
    usage BIGINT NOT NULL,

    period PERIOD NOT NULL,

    FOREIGN KEY (usage) REFERENCES usages (id) ON DELETE CASCADE
);