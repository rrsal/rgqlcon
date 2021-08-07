use crate::schema::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive{Queryable,GraphQLObject,GraphQLInputObject,Insertable,AsChangeset}]
#[table_name= "product_category"]
#[graphql(description = "Category")]
pub struct ProductCategory {
    pub product_id: String,
    pub category_id: String
}