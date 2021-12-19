use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable)]
#[table_name = "cart"]
pub struct Cart {
    pub cart_id: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub token: Option<String>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub total_price: Option<f64>,
    pub total_items: Option<f64>,
    pub cart_items: Option<Vec<String>>,
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "Cart Input")]
pub struct CartInput {
    pub user_id: String,
    pub session_id: Option<String>,
    pub token: Option<String>,
    pub status: Option<String>,
    pub total_price: Option<f64>,
    pub total_items: Option<f64>,
    pub cart_items: Option<Vec<String>>,
}

#[derive(Debug, GraphQLInputObject, AsChangeset)]
#[graphql(description = "Cart Update Input")]
#[table_name = "cart"]
pub struct CartUpdateInput {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub token: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub total_price: Option<f64>,
    pub total_items: Option<f64>,
    pub cart_items: Option<Vec<String>>,
}

impl Cart {
    pub fn new(
        id: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        total_price: Option<f64>,
        total_items: Option<f64>,
        new_cart: CartInput,
    ) -> Self {
        Cart {
            cart_id: id,
            user_id: new_cart.user_id,
            session_id: new_cart.session_id,
            token: new_cart.token,
            status: new_cart.status,
            created_at,
            updated_at,
            total_price,
            total_items,
            cart_items: new_cart.cart_items,
        }
    }
}
