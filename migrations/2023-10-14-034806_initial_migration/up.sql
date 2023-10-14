-- Define the 'notification_type' enum type

CREATE TYPE notification_type AS ENUM ('CHAT');

CREATE TYPE chat_type_enum AS ENUM ('PRIVATE', 'GROUP', 'ALONE');

CREATE TABLE
    users (
        id UUID PRIMARY KEY,
        username TEXT NOT NULL,
        password TEXT,
        email TEXT NOT NULL,
        last_login_at TIMESTAMPTZ,
        user_image TEXT,
        created_at TIMESTAMPTZ,
        updated_at TIMESTAMPTZ,
        tester BOOLEAN
    );

-- Create the 'auths' table with a foreign key relationship to 'users'

CREATE TABLE
    auths (
        id UUID PRIMARY KEY,
        access_token TEXT NOT NULL,
        refresh_token TEXT NOT NULL,
        created_at TIMESTAMPTZ,
        is_valid BOOLEAN NOT NULL,
        expiration TIMESTAMPTZ,
        auth_provider TEXT NOT NULL,
        user_id UUID REFERENCES users(id) -- Define the foreign key constraint
    );

-- Create the 'cats' table

CREATE TABLE
    cats (
        id INT4 PRIMARY KEY,
        name TEXT NOT NULL,
        age INT4 NOT NULL,
        breed TEXT NOT NULL,
        color TEXT NOT NULL,
        weight FLOAT8 NOT NULL,
        image TEXT,
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL
    );

-- Create the 'chat_rooms' table

CREATE TABLE
    chat_rooms (
        id UUID PRIMARY KEY,
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL,
        deleted_at TIMESTAMPTZ,
        name VARCHAR(255) NOT NULL,
        last_message VARCHAR(255),
        chat_type chat_type_enum,
        last_sent_user_id UUID REFERENCES users(id)
    );

-- Create the 'messages' table

CREATE TABLE
    messages (
        id UUID PRIMARY KEY,
        message VARCHAR(255) NOT NULL,
        sent_at TIMESTAMP NOT NULL,
        deleted_at TIMESTAMPTZ,
        chat_room_id UUID REFERENCES chat_rooms(id),
        sent_by UUID REFERENCES users(id)
    );

-- Create the 'notifications' table

CREATE TABLE
    notifications (
        id UUID PRIMARY KEY,
        user_id UUID REFERENCES users(id),
        user_to_notify UUID REFERENCES users(id),
        notification_type notification_type,
        data VARCHAR(255),
        read BOOLEAN,
        created_at TIMESTAMPTZ,
        updated_at TIMESTAMPTZ
    );

-- Create the 'user_chat_room' table

CREATE TABLE
    user_chat_room (
        id UUID PRIMARY KEY,
        user_id UUID REFERENCES users(id),
        chat_room_id UUID REFERENCES chat_rooms(id)
    );