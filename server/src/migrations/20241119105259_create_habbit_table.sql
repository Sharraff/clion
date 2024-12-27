-- Add migration script here
CREATE TABLE IF NOT EXISTS habit (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    frequency VARCHAR(50) NOT NULL, -- E.g., daily, weekly, monthly
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Add a unique constraint for user_id and name to avoid duplicates
CREATE UNIQUE INDEX idx_user_habit_name ON habits (user_id, name);