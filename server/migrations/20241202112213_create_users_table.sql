CREATE TABLE users (
    id BLOB NOT NULL,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (username),
    UNIQUE (email)
);

INSERT INTO users (id, username, email, password, created_at, updated_at) VALUES
    (X'019388060f357ff2a42acc0d7e74a172', 'admin', 'admin@plaatsoft.nl',
    '$2a$12$NWCyM414g16OqWvGT04vVOFL0BpZbm8fLBsFyMpvfwF5DUQJc8a.e', time('now'), time('now'));
