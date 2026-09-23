-- Add up migration script here
DELETE FROM audit_log;
DELETE FROM tasks;
ALTER TABLE tasks ADD COLUMN user_id BIGINT;
UPDATE tasks SET user_id = 1 WHERE user_id IS NULL;

ALTER TABLE tasks ALTER COLUMN  user_id SET NOT NULL, ADD CONSTRAINT fk_tasks_user 
    FOREIGN KEY (user_id) 
    REFERENCES users(id) 
    ON DELETE CASCADE;