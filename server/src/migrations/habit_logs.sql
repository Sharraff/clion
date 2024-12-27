-- habit_logs.sql
CREATE TABLE IF NOT EXISTS habit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    habit_id UUID NOT NULL REFERENCES habit(id) ON DELETE CASCADE,
    log_date DATE NOT NULL,
    status BOOLEAN NOT NULL DEFAULT FALSE, -- True for completed, false otherwise
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (habit_id, log_date) -- Ensure no duplicate logs for the same date
);