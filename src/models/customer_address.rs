use crate::schema::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable)]
#[table_name = "customer_address"]
pub struct CustomerAddress {
    pub user_id: String,
    pub address_id: Vec<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New CustomerAddress Mapping")]
pub struct NewCustomerAddress {
    pub user_id: String,
    pub address_id: Vec<String>,
}

#[derive(GraphQLInputObject, AsChangeset)]
#[table_name = "customer_address"]
#[graphql(description = "New CustomerAddress Mapping")]
pub struct UpdateCustomerAddress {
    pub user_id: String,
    pub address_id: Vec<String>,
}

impl CustomerAddress {
    pub fn new(input: NewCustomerAddress) -> Self {
        Self {
            user_id: input.user_id,
            address_id: input.address_id,
        }
    }
}

impl Default for CustomerAddress {
    fn default() -> Self {
        Self {
            user_id: String::default(),
            address_id: Vec::default(),
        }
    }
}
