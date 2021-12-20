-- Your SQL goes here
DROP TABLE customer_address;
CREATE  TABLE customer_address ( 
	user_id              varchar  NOT NULL ,
	address_id           text[]  NOT NULL ,
	CONSTRAINT pk_customer_address_user_id PRIMARY KEY ( user_id )
 );

ALTER TABLE customer_address ADD CONSTRAINT fk_customer_address_users FOREIGN KEY ( user_id ) REFERENCES users( user_id );

