-- Your SQL goes here
ALTER TABLE cart_items DROP CONSTRAINT fk_cart_items_cart; 
ALTER TABLE cart_items DROP COLUMN cart_id;
