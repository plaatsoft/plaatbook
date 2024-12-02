CREATE TABLE sessions (
    id BLOB NOT NULL,
    user_id BLOB NOT NULL,
    ip_address TEXT NOT NULL,
    ip_latitude REAL NULL,
    ip_longitude REAL NULL,
    ip_country TEXT NULL,
    ip_city TEXT NULL,
    client_name TEXT NULL,
    client_version TEXT NULL,
    client_os TEXT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
