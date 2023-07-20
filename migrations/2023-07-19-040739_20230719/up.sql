-- Enable the extension for generating UUIDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Start transaction
BEGIN;

-- Create new tables with UUID primary keys
CREATE TABLE new_users (LIKE users INCLUDING ALL);
ALTER TABLE new_users ADD COLUMN uuid UUID DEFAULT uuid_generate_v4();
ALTER TABLE new_users DROP COLUMN id;
ALTER TABLE new_users RENAME COLUMN uuid TO id;
ALTER TABLE new_users ADD PRIMARY KEY (id);

-- Create new auths table
CREATE TABLE new_auths (LIKE auths INCLUDING ALL);
ALTER TABLE new_auths ADD COLUMN uuid UUID DEFAULT uuid_generate_v4();
ALTER TABLE new_auths DROP COLUMN id;
ALTER TABLE new_auths RENAME COLUMN uuid TO id;
ALTER TABLE new_auths ADD PRIMARY KEY (id);
ALTER TABLE new_auths DROP COLUMN user_id;
ALTER TABLE new_auths ADD COLUMN user_id UUID;

-- Create new chat_rooms table
CREATE TABLE new_chat_rooms (LIKE chat_rooms INCLUDING ALL);
ALTER TABLE new_chat_rooms ADD COLUMN uuid UUID DEFAULT uuid_generate_v4();
ALTER TABLE new_chat_rooms DROP COLUMN id;
ALTER TABLE new_chat_rooms RENAME COLUMN uuid TO id;
ALTER TABLE new_chat_rooms ADD PRIMARY KEY (id);
ALTER TABLE new_chat_rooms DROP COLUMN last_sent_user_id;
ALTER TABLE new_chat_rooms ADD COLUMN last_sent_user_id UUID;

-- Create new user_chat_room table
CREATE TABLE new_user_chat_room (LIKE user_chat_room INCLUDING ALL);
ALTER TABLE new_user_chat_room ADD COLUMN uuid UUID DEFAULT uuid_generate_v4();
ALTER TABLE new_user_chat_room ADD PRIMARY KEY (uuid);
ALTER TABLE new_user_chat_room DROP COLUMN user_id;
ALTER TABLE new_user_chat_room DROP COLUMN chat_room_id;
ALTER TABLE new_user_chat_room ADD COLUMN user_id UUID;
ALTER TABLE new_user_chat_room ADD COLUMN chat_room_id UUID;


-- Create new messages table
CREATE TABLE new_messages (LIKE messages INCLUDING ALL);
ALTER TABLE new_messages ADD COLUMN uuid UUID DEFAULT uuid_generate_v4();
ALTER TABLE new_messages DROP COLUMN id;
ALTER TABLE new_messages RENAME COLUMN uuid TO id;
ALTER TABLE new_messages ADD PRIMARY KEY (id);
ALTER TABLE new_messages DROP COLUMN chat_room_id;
ALTER TABLE new_messages DROP COLUMN sent_by;
ALTER TABLE new_messages ADD COLUMN chat_room_id UUID;
ALTER TABLE new_messages ADD COLUMN sent_by UUID;

-- Copy data over to new_users with UUIDs
INSERT INTO new_users (username, password, email, last_login_at, user_image, created_at, updated_at, tester) 
SELECT username, password, email, last_login_at, user_image, created_at, updated_at, tester FROM users;

-- Create a mapping table for old users IDs to new UUIDs
CREATE TEMPORARY TABLE user_id_mapping AS
SELECT old.id AS old_id, new.id AS new_id
FROM users old
JOIN new_users new ON old.username = new.username AND old.email = new.email AND old.password = new.password AND old.last_login_at = new.last_login_at AND old.user_image = new.user_image AND old.created_at = new.created_at AND old.updated_at = new.updated_at AND old.tester = new.tester;

-- Copy data over to new_auths
INSERT INTO new_auths (access_token, refresh_token, user_id, created_at, is_valid, expiration, auth_provider, id) 
SELECT access_token, refresh_token, user_id_mapping.new_id, created_at, is_valid, expiration, auth_provider, uuid_generate_v4() 
FROM auths 
JOIN user_id_mapping ON auths.user_id = user_id_mapping.old_id;

-- Copy data over to new_chat_rooms
INSERT INTO new_chat_rooms (created_at, updated_at, deleted_at, name, last_message, last_sent_user_id, chat_type) 
SELECT created_at, updated_at, deleted_at, name, last_message, user_id_mapping.new_id, chat_type FROM chat_rooms 
JOIN user_id_mapping ON chat_rooms.last_sent_user_id = user_id_mapping.old_id;

-- Create a mapping table for old chat_rooms IDs to new UUIDs
CREATE TEMPORARY TABLE chat_room_id_mapping AS
SELECT old.id AS old_id, new.id AS new_id
FROM chat_rooms old
JOIN new_chat_rooms new ON old.name = new.name AND old.last_message = new.last_message AND old.chat_type = new.chat_type;

-- Copy data over to new_user_chat_room
INSERT INTO new_user_chat_room (user_id, chat_room_id) 
SELECT user_id_mapping.new_id, chat_room_id_mapping.new_id 
FROM user_chat_room 
JOIN user_id_mapping ON user_chat_room.user_id = user_id_mapping.old_id 
JOIN chat_room_id_mapping ON user_chat_room.chat_room_id = chat_room_id_mapping.old_id;

-- Copy data over to new_messages
INSERT INTO new_messages (chat_room_id, sent_by, message, sent_at, deleted_at) 
SELECT chat_room_id_mapping.new_id, user_id_mapping.new_id, message, sent_at, deleted_at 
FROM messages 
JOIN chat_room_id_mapping ON messages.chat_room_id = chat_room_id_mapping.old_id 
JOIN user_id_mapping ON messages.sent_by = user_id_mapping.old_id;

-- Rename old tables
ALTER TABLE auths RENAME TO old_auths;
ALTER TABLE users RENAME TO old_users;
ALTER TABLE chat_rooms RENAME TO old_chat_rooms;
ALTER TABLE user_chat_room RENAME TO old_user_chat_room;
ALTER TABLE messages RENAME TO old_messages;

-- Rename new tables to old names
ALTER TABLE new_users RENAME TO users;
ALTER TABLE new_auths RENAME TO auths;
ALTER TABLE new_chat_rooms RENAME TO chat_rooms;
ALTER TABLE new_user_chat_room RENAME TO user_chat_room;
ALTER TABLE new_messages RENAME TO messages;

-- End transaction
COMMIT;

BEGIN;
DROP TABLE old_auths CASCADE;
DROP TABLE old_users CASCADE;
DROP TABLE old_chat_rooms CASCADE;
DROP TABLE old_user_chat_room CASCADE;
DROP TABLE old_messages CASCADE;
COMMIT;
