-- Your SQL goes here
ALTER TABLE short_urls
ADD COLUMN encrypted BOOLEAN NOT NULL DEFAULT false;
