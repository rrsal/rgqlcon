
DROP TABLE product_price;

CREATE  TABLE product_price ( 
	date_from            date DEFAULT current_date NOT NULL ,
	product_id           integer  NOT NULL ,
	product_price        varchar(64)  NOT NULL ,
	price_id             varchar(64)  NOT NULL ,
	CONSTRAINT pk_product_price_price_id PRIMARY KEY ( price_id )
 );

