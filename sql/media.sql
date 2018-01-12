CREATE TYPE storage_location_type AS ENUM ('webserver_local');

CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    owner_id SERIAL REFERENCES users(id),
    file_format VARCHAR(32) NOT NULL,
    storage_location storage_location_type NOT NULL,
    reference VARCHAR(256) NOT NULL
);
