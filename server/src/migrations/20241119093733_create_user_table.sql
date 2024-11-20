-- Add migration script here
CREATE TABLE user (
    id SERIAL PRIMARY KEY,    -- Auto-incrementing unique ID
    name VARCHAR(100) NOT NULL,    -- User's full name
    email VARCHAR(255) UNIQUE NOT NULL,  -- User's email (unique)
    password TEXT NOT NULL,    -- Hashed password
    created_at TIMESTAMP DEFAULT NOW(),  -- Timestamp of account creation
    updated_at TIMESTAMP DEFAULT NOW()  -- Timestamp of last update
    );

-- Add an index on email for faster lookups
CREATE INDEX idx_user_email ON user (email);