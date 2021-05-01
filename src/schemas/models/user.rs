use juniper::{GraphQLObject};
use chrono::NaiveDateTime;



#[derive(Queryable,GraphQLObject)]
pub struct Users{
    user_id:i32,
    first_name:String,
    middle_name:String,
    last_name:String,
    address_id:i32,
    email:String,
    phone:String,
    password_hash:String,
    registered_at:NaiveDateTime,
    last_login:NaiveDateTime,
    rating:i32,
    profile:String
}