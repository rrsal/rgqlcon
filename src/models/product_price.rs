use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLObject, GraphQLInputObject};

#[derive(Queryable,Insertable,GraphQLObject)]
#[table_name = "product_price"]
pub struct ProductPrice{
    pub date_from: NaiveDateTime,
    pub product_id: String,
    pub price: f64,
    pub price_id: String
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Product Price")]
pub struct NewProductPrice {
    pub product_id: String,
    pub price: f64
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Updated Product Price")]
pub struct UpdateProductPrice{
    pub product_id: String,
    pub price: f64
}

#[derive(AsChangeset)]
#[table_name = "product_price"]
pub struct UpdateProductReviewDB{
    pub date_from: NaiveDateTime,
    pub product_id: String,
    pub price: f64
}