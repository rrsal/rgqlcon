-- Your SQL goes here
ALTER TABLE cart_items DROP COLUMN created_at;
ALTER TABLE cart_items DROP COLUMN updated_at;

ALTER TABLE cart ADD COLUMN cart_items text[];
