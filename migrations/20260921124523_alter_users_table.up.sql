-- Add up migration script here

ALTER TABLE users ADD COLUMN IF NOT EXISTS email VARCHAR(255) UNIQUE;

ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255);


UPDATE users SET email = 'placeholder_' || id || '@example.com',
    password_hash = '$2b$12$placeholderhashplaceholderhashplaceholde'
    WHERE email is null;

ALTER TABLE users ALTER COLUMN email SET NOT NULL;
ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL;

-- 4. Для created_at это делать не нужно, так как DEFAULT NOW() сам заполнит старые строки текущим временем!
ALTER TABLE users ADD COLUMN IF NOT EXISTS created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW();