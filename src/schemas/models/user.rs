use juniper::{GraphQLObject,GraphQLInputObject};
use chrono::NaiveDateTime;
use crate::schema::users;


#[derive(Queryable,GraphQLObject,Insertable)]
#[table_name = "users"]
pub struct Users{
    pub user_id:String,
    pub first_name:String,
    pub middle_name:String,
    pub last_name:String,
    pub address_id:i32,
    pub email:String,
    pub phone:String,
    pub password_hash:String,
    pub registered_at:NaiveDateTime,
    pub last_login:NaiveDateTime,
    pub rating:i32,
    pub profile:String
}



#[derive(GraphQLInputObject)]
#[graphql(description="New User object")]
pub struct NewUser{
    pub first_name:String,
    pub middle_name:String,
    pub last_name:String,
    pub address_id:i32,
    pub email:String,
    pub phone:String,
    pub password_hash:String,
    pub rating:i32,
    pub profile:String
}