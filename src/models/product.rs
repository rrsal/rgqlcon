use juniper::{GraphQLObject, GraphQLInputObject};
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable,GraphQLObject,Insertable)]
#[table_name = "products"]
pub struct Products{
    pub product_id: String,
    pub title: String,
    pub meta_title: String,
    pub summary: String,
    pub sku: String,
    pub p_type: String,
    pub price: i32,
    pub discount: f32 ,
    pub quantity: f32,
    pub seller_id: String,
    pub create_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published_at: NaiveDateTime,
    pub other_details: String,
    pub category_id: i32
}

#[derive(GraphQLInpuutObject)]
#[graphql(description="New Product Object")]
pub struct NewProduct(
    pub title: String,
    pub meta_title: String,
    pub summary: String,
    pub sku: String,
    pub p_type: String,
    pub price: i32,
    pub discount: f32 ,
    pub quantity: f32,
    pub seller_id: String,
    pub other_details: String,
    pub category_id: i32
)

#[derive(GraphQLInpuutObject)]
#[graphql(description="Update Product Object")]
#[table_name = "products"]
pub struct UpdateProduct(
    pub title: String,
    pub meta_title: String,
    pub summary: String,
    pub sku: String,
    pub p_type: String,
    pub price: i32,
    pub discount: f32 ,
    pub quantity: f32,
    pub seller_id: String,
    pub other_details: String,
    pub category_id: i32
)

