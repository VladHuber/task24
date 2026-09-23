-- Add down migration script here
ALTER TABLE users DROP COLUMN email;
ALTER TABLE users DROP COLUMN password_hash;
ALTER TABLE users DROP COLUMN created_at;