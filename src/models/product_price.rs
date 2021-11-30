use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, Insertable, GraphQLObject)]
#[table_name = "product_price"]
pub struct ProductPrice {
    pub date_from: NaiveDateTime,
    pub product_id: String,
    pub price: f64,
    pub price_id: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Product Price")]
pub struct NewProductPrice {
    pub product_id: String,
    pub price: f64,
}

#[derive(GraphQLInputObject, AsChangeset)]
#[graphql(description = "Updated Product Price")]
#[table_name = "product_price"]
pub struct UpdateProductPrice {
    pub product_id: Option<String>,
    pub price: f64,
}

impl ProductPrice {
    pub fn new(id: String, date_from: NaiveDateTime, new_price: NewProductPrice) -> Self {
        Self {
            price_id: id,
            date_from,
            product_id: new_price.product_id,
            price: new_price.price,
        }
    }
}
