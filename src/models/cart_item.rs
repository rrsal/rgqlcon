use crate::schema::cart_items;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable)]
#[table_name = "cart_items"]
pub struct CartItem {
    pub item_id: String,
    pub cart_id: String,
    pub product_id: String,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub discount: Option<f64>,
    pub quantity: Option<f64>,
    pub measure: Option<f64>,
    pub active: Option<i32>,
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "Input for creating a new cart item")]
pub struct NewCartItem {
    pub item_id: String,
    pub cart_id: String,
    pub product_id: String,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub discount: Option<f64>,
    pub quantity: Option<f64>,
    pub measure: Option<f64>,
    pub active: Option<i32>,
}

#[derive(Debug, GraphQLInputObject, AsChangeset)]
#[graphql(description = "Input for updating a cart item")]
#[table_name = "cart_items"]
pub struct UpdateCartItem {
    pub item_id: Option<String>,
    pub cart_id: Option<String>,
    pub product_id: Option<String>,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub discount: Option<f64>,
    pub quantity: Option<f64>,
    pub measure: Option<f64>,
    pub active: Option<i32>,
}

impl CartItem {
    pub fn new(id: String, new_cart_item: NewCartItem) -> Self {
        Self {
            item_id: id,
            cart_id: new_cart_item.cart_id,
            product_id: new_cart_item.product_id,
            sku: new_cart_item.sku,
            price: new_cart_item.price,
            discount: new_cart_item.discount,
            quantity: new_cart_item.quantity,
            measure: new_cart_item.measure,
            active: new_cart_item.active,
        }
    }
}
