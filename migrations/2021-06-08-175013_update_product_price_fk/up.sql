DROP TABLE product_price;

CREATE  TABLE product_price ( 
	date_from            date DEFAULT current_date NOT NULL ,
	product_id           varchar(64)  NOT NULL ,
	product_price        float8  NOT NULL ,
	price_id             varchar(64)  NOT NULL ,
	CONSTRAINT pk_product_price_price_id PRIMARY KEY ( price_id )
 );


ALTER TABLE product_price ADD CONSTRAINT fk_product_price_products FOREIGN KEY ( product_id ) REFERENCES products( product_id );
