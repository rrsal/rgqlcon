use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, GraphQLObject, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub user_id: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub address_id: String,
    pub email: String,
    pub phone: String,
    pub password_hash: String,
    pub registered_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
    pub rating: i32,
    pub profile: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New User object")]
pub struct NewUser {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub address_id: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub rating: i32,
    pub profile: String,
}

impl Users {
    pub fn new(
        id: String,
        password_hash: String,
        registered_at: NaiveDateTime,
        last_login: NaiveDateTime,
        new_user: NewUser,
    ) -> Self {
        Self {
            user_id: id,
            first_name: new_user.first_name,
            middle_name: new_user.middle_name,
            last_name: new_user.last_name,
            address_id: new_user.address_id,
            email: new_user.email,
            phone: new_user.phone,
            rating: new_user.rating,
            profile: new_user.profile,
            password_hash,
            registered_at,
            last_login,
        }
    }
}

#[derive(GraphQLInputObject, AsChangeset)]
#[graphql(description = "Update User object")]
#[table_name = "users"]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub address_id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub rating: Option<i32>,
    pub profile: Option<String>,
}

#[derive(Queryable, GraphQLObject, Insertable)]
#[table_name = "address"]
pub struct Address {
    pub address_id: String,
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub city: String,
    pub zip_code: i32,
    pub state_province: String,
    pub country: String,
    pub other_details: String,
}

#[derive(Queryable, GraphQLObject)]
pub struct CustomerAddress {
    pub user_id: String,
    pub address_id: String,
    pub date_from: NaiveDateTime,
    pub date_to: NaiveDateTime,
    pub address_type: String,
    pub address: Vec<Address>,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "address"]
pub struct NewAddressInput {
    pub address_id: String,
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub city: String,
    pub zip_code: i32,
    pub state_province: String,
    pub country: String,
    pub other_details: String,
}

#[derive(GraphQLInputObject)]
pub struct NewCustomerAddressInput {
    pub user_id: String,
    pub address_id: String,
    pub address_type: String,
    pub address: NewAddressInput,
}

#[derive(Insertable)]
#[table_name = "customer_address"]
pub struct NewCustomerAddress {
    pub user_id: String,
    pub address_id: Vec<String>,
}
