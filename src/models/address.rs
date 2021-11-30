use crate::schema::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable)]
#[table_name = "address"]
pub struct Addresses {
    pub address_id: String,
    pub line_1: String,
    pub line_2: Option<String>,
    pub line_3: Option<String>,
    pub city: String,
    pub zip_code: i32,
    pub state_province: String,
    pub country: String,
    pub other_details: Option<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Address for user")]
pub struct NewAddress {
    pub line_1: String,
    pub line_2: Option<String>,
    pub line_3: Option<String>,
    pub city: String,
    pub zip_code: i32,
    pub state_province: String,
    pub country: String,
    pub other_details: Option<String>,
}

#[derive(Debug, GraphQLInputObject, AsChangeset)]
#[graphql(description = "Update Address for user")]
#[table_name = "address"]
pub struct UpdateAddress {
    pub line_1: Option<String>,
    pub line_2: Option<String>,
    pub line_3: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<i32>,
    pub state_province: Option<String>,
    pub country: Option<String>,
    pub other_details: Option<String>,
}

impl Addresses {
    pub fn new(id: String, new_address: NewAddress) -> Self {
        Self {
            address_id: id,
            line_1: new_address.line_1,
            line_2: new_address.line_2,
            line_3: new_address.line_3,
            city: new_address.city,
            zip_code: new_address.zip_code,
            state_province: new_address.state_province,
            country: new_address.country,
            other_details: new_address.other_details,
        }
    }
}
