use crate::schema::*;
use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, Insertable, GraphQLObject)]
#[table_name = "product_review"]
pub struct ProductReview {
    pub review_id: String,
    pub product_id: String,
    pub parent_id: Option<String>,
    pub title: Option<String>,
    pub rating: Option<i32>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub published: Option<i32>,
    pub published_at: NaiveDateTime,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Product Review")]
pub struct NewProductReview {
    pub product_id: String,
    pub parent_id: Option<String>,
    pub title: Option<String>,
    pub rating: Option<i32>,
    pub description: Option<String>,
    pub published: Option<i32>,
}

#[derive(GraphQLInputObject, AsChangeset)]
#[graphql(description = "Updated Product Review")]
#[table_name = "product_review"]
pub struct UpdateProductReview {
    pub product_id: Option<String>,
    pub parent_id: Option<String>,
    pub title: Option<String>,
    pub rating: Option<i32>,
    pub description: Option<String>,
    pub published: Option<i32>,
    pub published_at: Option<NaiveDateTime>,
}

impl ProductReview {
    pub fn new(
        id: String,
        published_at: NaiveDateTime,
        created_at: NaiveDateTime,
        new_review: NewProductReview,
    ) -> Self {
        Self {
            review_id: id,
            product_id: new_review.product_id,
            parent_id: new_review.parent_id,
            title: new_review.title,
            rating: new_review.rating,
            description: new_review.description,
            created_at,
            published: new_review.published,
            published_at,
        }
    }
}

// #[derive(AsChangeset)]
// #[table_name = "product_review"]
// pub struct UpdateProductReviewDB{
//     pub product_id: String,
//     pub parent_id: String,
//     pub title: String,
//     pub rating: i32,
//     pub description: String,
//     pub published: i32,
//     pub published_at: NaiveDateTime
// }
