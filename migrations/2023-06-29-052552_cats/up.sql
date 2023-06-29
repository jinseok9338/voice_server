-- Your SQL goes here
CREATE TABLE cats (
  id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    age INT NOT NULL,
    breed TEXT NOT NULL,
    color TEXT NOT NULL,
    weight FLOAT NOT NULL,
    image TEXT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)



