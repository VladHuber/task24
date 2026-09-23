-- Add down migration script here
ALTER TABLE tasks DROP CONSTRAINT fk_tasks_user;
ALTER TABLE tasks DROP COLUMN user_id;