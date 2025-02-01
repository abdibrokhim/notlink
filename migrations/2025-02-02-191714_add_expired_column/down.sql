-- This file should undo anything in `up.sql`
ALTER TABLE short_urls
DROP COLUMN expired;
