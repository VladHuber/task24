-- Add up migration script here
CREATE TABLE IF NOT EXISTS audit_log (
    id BIGSERIAL PRIMARY KEY,
    task_id BIGINT NOT NULL REFERENCES tasks(id),
    action TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);