CREATE  TABLE product_review ( 
	review_id            varchar(64)  NOT NULL ,
	product_id           varchar(64)  NOT NULL ,
	parent_id            varchar(64),
	title                varchar(100)   ,
	rating               Integer   ,
	description          text   ,
	created_at           timestamp(0)  NOT NULL  ,
	published            Integer   ,
	published_at         timestamp(0)  NOT NULL  ,
	CONSTRAINT pk_product_review_review_id PRIMARY KEY ( review_id )
 );
ALTER TABLE product_review ADD CONSTRAINT fk_product_review_product_review FOREIGN KEY ( parent_id ) REFERENCES product_review( review_id );
ALTER TABLE product_review ADD CONSTRAINT fk_product_review_products FOREIGN KEY ( product_id ) REFERENCES products( product_id );




CREATE  TABLE product_price ( 
	date_from            timestamp(0)  NOT NULL,
	product_id           varchar(64)  NOT NULL ,
	price        float8  NOT NULL ,
	CONSTRAINT pk_price_date_from PRIMARY KEY ( date_from )
 );
ALTER TABLE product_price ADD CONSTRAINT fk_product_price_products FOREIGN KEY ( product_id ) REFERENCES products( product_id );



CREATE  TABLE category ( 
	category_id          varchar(64)  NOT NULL ,
	title                varchar(75)   ,
	meta_title           varchar(100)   ,
	summary              text   ,
	content              text   ,
	CONSTRAINT pk_category_category_id PRIMARY KEY ( category_id )
 );


CREATE  TABLE product_category ( 
	category_id          varchar(64)  NOT NULL ,
	product_id           varchar(64)  NOT NULL ,
	CONSTRAINT pk_product_category_category_id PRIMARY KEY ( category_id )
 );
ALTER TABLE product_category ADD CONSTRAINT fk_product_category_category FOREIGN KEY ( category_id ) REFERENCES category( category_id );
ALTER TABLE product_category ADD CONSTRAINT fk_product_category_products FOREIGN KEY ( product_id ) REFERENCES products( product_id );


CREATE  TABLE tag ( 
	tag_id               varchar(64)  NOT NULL ,
	title                varchar(75)   ,
	meta_title           varchar(100)   ,
	content              text   ,
	CONSTRAINT pk_tag_tag_id PRIMARY KEY ( tag_id )
 );



CREATE  TABLE product_tag ( 
	tag_id                varchar(64)  NOT NULL ,
	product_id            varchar(64)  NOT NULL ,
	CONSTRAINT pk_product_tag_tag_id PRIMARY KEY ( tag_id )
 );

ALTER TABLE product_tag ADD CONSTRAINT fk_product_tag_tag FOREIGN KEY ( tag_id ) REFERENCES tag( tag_id );
ALTER TABLE product_tag ADD CONSTRAINT fk_product_tag_products FOREIGN KEY ( product_id ) REFERENCES products( product_id );



