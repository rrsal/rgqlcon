use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable)]
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published_at: NaiveDateTime,
    pub other_details: String,
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
    pub category_id: String,
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
}

impl Products {
    pub fn new(
        id: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        published_at: NaiveDateTime,
        new_product: NewProduct,
    ) -> Self {
        Self {
            product_id: id,
            title: new_product.title,
            meta_title: new_product.meta_title,
            summary: new_product.summary,
            sku: new_product.sku,
            p_type: new_product.p_type,
            price: new_product.price,
            discount: new_product.discount,
            quantity: new_product.quantity,
            seller_id: new_product.seller_id,
            created_at,
            updated_at,
            published_at,
            other_details: new_product.other_details,
        }
    }
}
