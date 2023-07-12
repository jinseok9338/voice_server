-- Create chat_rooms table
-- Create chat_rooms table with chat_type enum
CREATE TABLE chat_rooms (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  deleted_at TIMESTAMP,
  name VARCHAR(255) NOT NULL,
  last_message VARCHAR(255),
  last_sent_user_id INT,
  chat_type chat_type_enum NOT NULL, -- Add chat_type column as enum
  FOREIGN KEY (last_sent_user_id) REFERENCES users(id)
);

-- Define chat_type_enum enum type
CREATE TYPE chat_type_enum AS ENUM ('PRIVATE', 'GROUP', 'ALONE');
CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  chat_room_id INT NOT NULL,
  sent_by INT NOT NULL,
  message VARCHAR(255) NOT NULL,
  sent_at TIMESTAMP NOT NULL,
  deleted_at TIMESTAMP,
  FOREIGN KEY (chat_room_id) REFERENCES chat_rooms(id),
  FOREIGN KEY (sent_by) REFERENCES users(id)
);

-- Create user_chat_room table to establish many-to-many relationship between users and chat rooms
CREATE TABLE user_chat_room (
  user_id INT NOT NULL,
  chat_room_id INT NOT NULL,
  PRIMARY KEY (user_id, chat_room_id),
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (chat_room_id) REFERENCES chat_rooms(id)
);
