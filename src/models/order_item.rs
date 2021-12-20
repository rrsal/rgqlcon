use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable)]
#[table_name = "order_items"]
#[graphql(description = "An order item")]
pub struct OrderItem {
    pub item_id: String,
    pub product_id: String,
    pub order_id: String,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub discount: Option<f64>,
    pub quantity: Option<f64>,
    pub measure: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "New order item")]
pub struct NewOrderItem {
    pub product_id: String,
    pub order_id: String,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub discount: Option<f64>,
    pub quantity: Option<f64>,
    pub measure: Option<f64>,
}

#[derive(Debug, GraphQLInputObject, AsChangeset)]
#[graphql(description = "Order item update")]
#[table_name = "order_items"]
pub struct UpdateOrderItem {
    pub product_id: Option<String>,
    pub order_id: Option<String>,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub discount: Option<f64>,
    pub quantity: Option<f64>,
    pub measure: Option<f64>,
}

impl OrderItem {
    pub fn new(
        item_id: String,
        input: NewOrderItem,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            item_id,
            product_id: input.product_id,
            order_id: input.order_id,
            sku: input.sku,
            price: input.price,
            discount: input.discount,
            quantity: input.quantity,
            measure: input.measure,
            created_at,
            updated_at,
        }
    }
}

impl Default for OrderItem {
    fn default() -> Self {
        Self {
            item_id: String::default(),
            product_id: String::default(),
            order_id: String::default(),
            sku: None,
            price: None,
            discount: None,
            quantity: None,
            measure: None,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}
