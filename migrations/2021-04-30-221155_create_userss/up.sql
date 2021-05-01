CREATE  TABLE users ( 
	user_id              Integer  NOT NULL,
	first_name           varchar(100) NOT NULL,
	middle_name          varchar(100) NOT NULL,
	last_name            varchar(100) NOT NULL,
	address_id           Integer NOT NULL,
	email                varchar(100) NOT NULL,
	phone                varchar(15) NOT NULL,
	password_hash        varchar  NOT NULL,
	registered_at        timestamp NOT NULL,
	last_login           timestamp NOT NULL,
	rating               Integer NOT NULL,
	profiles             Text NOT NULL,
	CONSTRAINT pk_users_user_id PRIMARY KEY ( user_id )
 );

INSERT INTO users(user_id, first_name, middle_name, last_name, address_id, email, phone, password_hash, registered_at, last_login, rating, profiles) VALUES( 1, 'ef', 'ff', 'ff', 1, 'f', 'd', '1', '2018-07-07 17:01:18.410677', '2018-07-07 17:01:18.410677', 1, 'kk');
