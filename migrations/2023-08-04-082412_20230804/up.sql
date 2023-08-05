-- Your SQL goes here
CREATE TYPE notification_type AS ENUM ('CHAT');

CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id UUID,
    user_to_notify UUID,
    type notification_type,
    data VARCHAR(255),
    read INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (user_to_notify) REFERENCES users (id)
);