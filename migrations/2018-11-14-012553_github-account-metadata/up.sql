ALTER TABLE github_accounts ADD COLUMN username TEXT;
ALTER TABLE github_accounts ALTER COLUMN access_token DROP NOT NULL;
