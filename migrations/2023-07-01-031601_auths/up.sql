-- Your SQL goes here
CREATE TABLE auths (
    id SERIAL PRIMARY KEY,
    access_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_valid BOOLEAN NOT NULL,
    expiration TIMESTAMP WITH TIME ZONE,
    auth_provider TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
