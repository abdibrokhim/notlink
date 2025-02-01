-- Your SQL goes here
ALTER TABLE short_urls
ADD COLUMN expired BOOLEAN NOT NULL DEFAULT false;
