CREATE  TABLE cart ( 
	cart_id              varchar(64)  NOT NULL ,
	user_id              varchar(64)  NOT NULL ,
	session_id           varchar(100)   ,
	token                varchar(100)   ,
	status               char(1)   ,
	CONSTRAINT pk_cart_cart_id_0 PRIMARY KEY ( cart_id )
 );
ALTER TABLE cart ADD CONSTRAINT fk_cart_users FOREIGN KEY ( user_id ) REFERENCES users( user_id );



CREATE  TABLE cart_items ( 
	item_id              varchar(64)  NOT NULL ,
	product_id           varchar(64)  NOT NULL ,
	cart_id              varchar(64)  NOT NULL ,
	sku                  varchar(100)   ,
	price                float8   ,
	discount             float8   ,
	quantity             float8   ,
	measure              float8   ,
	active               integer   ,
	created_at           timestamp(0)  NOT NULL  ,
	updated_at           timestamp(0)  NOT NULL  ,
	CONSTRAINT pk_cart_cart_id PRIMARY KEY ( item_id )
 );
ALTER TABLE cart_items ADD CONSTRAINT fk_cart_items_cart FOREIGN KEY ( cart_id ) REFERENCES cart( cart_id );
ALTER TABLE cart_items ADD CONSTRAINT fk_cart_items_products FOREIGN KEY ( product_id ) REFERENCES products( product_id );


