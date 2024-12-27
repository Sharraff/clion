-- achievements.sql

CREATE TABLE IF NOT EXISTS achievement (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    earned_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);