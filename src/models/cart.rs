use crate::models::cart_item::CartItem;
use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable, Clone)]
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

#[derive(Debug, GraphQLObject)]
pub struct CartOutput {
    pub cart_id: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub token: Option<String>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub total_price: Option<f64>,
    pub total_items: Option<f64>,
    pub cart_items: Vec<CartItem>,
}

#[derive(Debug, GraphQLInputObject, Clone)]
#[graphql(description = "Product-Price Input")]
pub struct Items {
    pub id: String,
    pub quantity: f64,
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
    pub cart_items: Option<Vec<Items>>,
}

#[derive(Debug, GraphQLInputObject, AsChangeset, Clone)]
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

impl CartOutput {
    pub fn new(cart: Cart, items: Vec<CartItem>) -> Self {
        Self {
            cart_id: cart.cart_id,
            user_id: cart.user_id,
            session_id: cart.session_id,
            token: cart.token,
            status: cart.status,
            created_at: cart.created_at,
            updated_at: cart.updated_at,
            total_price: cart.total_price,
            total_items: cart.total_items,
            cart_items: items,
        }
    }
}

impl Cart {
    pub fn new(
        id: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        total_price: Option<f64>,
        total_items: Option<f64>,
        cart_items: Option<Vec<String>>,
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
            cart_items,
        }
    }
}
