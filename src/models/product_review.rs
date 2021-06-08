use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLObject,GraphQLInputObject};

#[derive(Queryable, Insertable, GraphQLObject)]
#[table_name = "product_review"]
pub struct ProductReview{
    pub review_id: String,
    pub product_id: String,
    pub parent_id: String,
    pub title: String,
    pub rating: i32,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub published: i32,
    pub published_at: NaiveDateTime
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Product Review")]
pub struct NewProducrReview {
    pub product_id: String,
    pub parent_id: String,
    pub title: String,
    pub rating: i32,
    pub description: String,
    pub published: i32
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Updated Product Review")]
pub struct UpdateProductReview{
    pub product_id: String,
    pub parent_id: String,
    pub title: String,
    pub rating: i32,
    pub description: String,
    pub published: i32
}


#[derive(AsChangeset)]
#[table_name = "product_review"]
pub struct UpdateProductReviewDB{
    pub product_id: String,
    pub parent_id: String,
    pub title: String,
    pub rating: i32,
    pub description: String,
    pub published: i32,
    pub published_at: NaiveDateTime
}