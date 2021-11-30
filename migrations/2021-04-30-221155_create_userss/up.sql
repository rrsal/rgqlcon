CREATE  TABLE users ( 
	user_id              varchar  NOT NULL,
	first_name           varchar(100) NOT NULL,
	middle_name          varchar(100) NOT NULL,
	last_name            varchar(100) NOT NULL,
	address_id           varchar NOT NULL,
	email                varchar(100) NOT NULL,
	phone                varchar(15) NOT NULL,
	password_hash        varchar  NOT NULL,
	registered_at        timestamp(0) NOT NULL,
	last_login           timestamp(0) NOT NULL,
	rating               Integer NOT NULL,
	profile             Text NOT NULL,
	CONSTRAINT pk_users_user_id PRIMARY KEY ( user_id )
 );





CREATE  TABLE address ( 
	address_id           varchar  NOT NULL ,
	line_1               varchar  NOT NULL ,
	line_2               varchar  ,
	line_3               varchar  ,
	city                 varchar  NOT NULL ,
	zip_code             integer  NOT NULL ,
	state_province       varchar  NOT NULL ,
	country              varchar  NOT NULL ,
	other_details        varchar ,
	CONSTRAINT pk_address_address_id PRIMARY KEY ( address_id )
 );


CREATE  TABLE customer_address ( 
	user_id              varchar  NOT NULL ,
	address_id           varchar  NOT NULL ,
	date_from            timestamp(0)  NOT NULL ,
	date_to              timestamp(0)  NOT NULL ,
	address_type         varchar  NOT NULL ,
	CONSTRAINT pk_customer_address_date_from PRIMARY KEY ( date_from )
 );
ALTER TABLE customer_address ADD CONSTRAINT fk_customer_address_users FOREIGN KEY ( user_id ) REFERENCES users( user_id );
ALTER TABLE customer_address ADD CONSTRAINT fk_customer_address_address FOREIGN KEY ( address_id ) REFERENCES address( address_id );



CREATE  TABLE products ( 
	product_id           varchar(64)  NOT NULL ,
	title                varchar(75)  NOT NULL ,
	meta_title           varchar(100)  NOT NULL ,
	summary              text  NOT NULL ,
	sku                  varchar(100)  NOT NULL ,
	p_type               varchar  NOT NULL ,
	price                float8  NOT NULL ,
	discount             float8  NOT NULL ,
	quantity             float8  NOT NULL ,
	seller_id            varchar  NOT NULL ,
	created_at            timestamp(0)  NOT NULL ,
	updated_at           timestamp(0) NOT NULL ,
	published_at         timestamp(0)  NOT NULL ,
	other_details        varchar(100)  NOT NULL ,
	category_id          Integer  NOT NULL ,
	CONSTRAINT pk_products_product_id PRIMARY KEY ( product_id )
 );



INSERT INTO users(user_id, first_name, middle_name, last_name, address_id, email, phone, password_hash, registered_at, last_login, rating, profile) VALUES( '100', 'ef', 'ff', 'ff', 1, 'f', 'd', '1', '2018-07-07 17:01:18.410677', '2018-07-07 17:01:18.410677', 1, 'kk');
