CREATE TABLE address (
    id SERIAL PRIMARY KEY,
    address_line_1 TEXT,
    address_line_2 TEXT,
    address_line_3 TEXT,
    address_line_4 TEXT,
    address_line_5 TEXT,
    address_line_6 TEXT,
    address_line_7 TEXT,
    country_name VARCHAR(80) NOT NULL REFERENCES country(name)
);

CREATE TABLE contact_info (
   id SERIAL PRIMARY KEY,
   primary_email VARCHAR(128) NOT NULL,
   backup_email VARCHAR(128),
   main_intl_phone_number VARCHAR(64), 
   secondary_intl_phone_number VARCHAR(64) 
);

CREATE TYPE photo_status_type AS ENUM ('approved', 'denied', 'unclear', 'waiting_approval');

CREATE TABLE personal_identity (
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id),

    -- identity numbers
    nz_passport_number VARCHAR(64),
    nz_drivers_licence_number VARCHAR(64),

    -- photo references
    face_with_document_photo SERIAL REFERENCES media(id),
    face_photo SERIAL REFERENCES media(id),
    document_scan_photo SERIAL REFERENCES media(id),

    -- human needs to take a look
    face_with_document_status photo_status_type, -- face should match photo on doc
    face_status photo_status_type, -- should be a clear photo
    document_scan_status photo_status_type -- should be a real-looking ID, and match the licence or passport number
);

CREATE TABLE personal_info (
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id) NOT NULL,
    given_names TEXT NOT NULL,
    family_names TEXT NOT NULL,
    date_of_birth DATE NOT NULL,
    address_id SERIAL REFERENCES address(id),
    contact_info_id INTEGER REFERENCES contact_info(id),
    personal_identity_id INTEGER REFERENCES personal_identity(id) 
);
