CREATE  TABLE iorder ( 
	order_id             varchar(64)  NOT NULL ,
	user_id              varchar(64)  NOT NULL ,
	session_id           varchar(100)   ,
	token                varchar(100)   ,
	status               char(1)   ,
	sub_total            float8   ,
	discount             float8   ,
	tax                  float8   ,
	total                float8   ,
	promo                varchar(50)   ,
	CONSTRAINT pk_tbl_order_id PRIMARY KEY ( order_id )
 );
ALTER TABLE iorder ADD CONSTRAINT fk_order_users FOREIGN KEY ( user_id ) REFERENCES users( user_id );



CREATE  TABLE order_items ( 
	item_id              varchar(64)  NOT NULL ,
	product_id           varchar(64)  NOT NULL ,
	order_id             varchar(64)  NOT NULL ,
	sku                  varchar(100)   ,
	price                float8   ,
	discount             float8   ,
	quantity             float8   ,
	measure              float8   ,
	created_at           timestamp(0)  NOT NULL  ,
	updated_at           timestamp(0)  NOT NULL  ,
	CONSTRAINT pk_cart_cart_id_1 PRIMARY KEY ( item_id )
 );
ALTER TABLE order_items ADD CONSTRAINT fk_order_items_order FOREIGN KEY ( order_id ) REFERENCES iorder( order_id );
ALTER TABLE order_items ADD CONSTRAINT fk_order_items_products FOREIGN KEY ( product_id ) REFERENCES products( product_id );



CREATE  TABLE transactions ( 
	transaction_id       varchar(64)  NOT NULL ,
	user_id              varchar(64)  NOT NULL ,
	order_id             varchar(64)  NOT NULL ,
	code                 varchar(100)  NOT NULL ,
	tran_type            INTEGER   ,
	tran_mode            INTEGER   ,
	status               char(1)   ,
	created_at           timestamp(0)  NOT NULL  ,
	updated_at           timestamp(0)  NOT NULL  ,
	CONSTRAINT pk_transactions_transaction_id PRIMARY KEY ( transaction_id )
 );
ALTER TABLE transactions ADD CONSTRAINT fk_transactions_order FOREIGN KEY ( order_id ) REFERENCES iorder( order_id );
ALTER TABLE transactions ADD CONSTRAINT fk_transactions_users FOREIGN KEY ( user_id ) REFERENCES users( user_id );




