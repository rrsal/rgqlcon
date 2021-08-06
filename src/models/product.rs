use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, GraphQLObject, Insertable)]
#[table_name = "products"]
pub struct Products {
    pub product_id: String,
    pub title: String,
    pub meta_title: String,
    pub summary: String,
    pub sku: String,
    pub p_type: String,
    pub price: f64,
    pub discount: f64,
    pub quantity: f64,
    pub seller_id: String,
    pub create_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published_at: NaiveDateTime,
    pub other_details: String,
    pub category_id: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Product Object")]
pub struct NewProduct {
    pub title: String,
    pub meta_title: String,
    pub summary: String,
    pub sku: String,
    pub p_type: String,
    pub price: f64,
    pub discount: f64,
    pub quantity: f64,
    pub seller_id: String,
    pub other_details: String,
    pub category_id: i32,
}

#[derive(GraphQLInputObject, AsChangeset)]
#[graphql(description = "Update Product Object")]
#[table_name = "products"]
pub struct UpdateProduct {
    pub title: String,
    pub meta_title: String,
    pub summary: String,
    pub sku: String,
    pub p_type: String,
    pub price: f64,
    pub discount: f64,
    pub quantity: f64,
    pub seller_id: String,
    pub other_details: String,
    pub category_id: i32,
}
