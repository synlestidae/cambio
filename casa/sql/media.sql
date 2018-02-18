CREATE TYPE storage_location_type AS ENUM ('webserver_local');
CREATE TYPE file_format_type AS ENUM ('png', 'jpeg', 'bmp', 'gif');

CREATE TABLE media (
    id SERIAL PRIMARY KEY,
    owner_id SERIAL REFERENCES users(id),
    file_format file_format_type NOT NULL,
    storage_location storage_location_type NOT NULL,
    reference VARCHAR(256) NOT NULL,
    file_size UINT NOT NULL
);
