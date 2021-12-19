use crate::schema::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, GraphQLObject, Insertable)]
#[table_name = "product_category"]
#[graphql(description = "Category")]
pub struct ProductCategory {
    pub product_id: String,
    pub category_id: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New ProductCategory")]
pub struct NewProductCategory {
    pub product_id: String,
    pub category_id: String,
}

#[derive(GraphQLInputObject, AsChangeset)]
#[graphql(description = "Update ProductCategory")]
#[table_name = "product_category"]
pub struct UpdateProductCategory {
    pub product_id: String,
    pub category_id: String,
}

impl ProductCategory {
    pub fn new(input: NewProductCategory) -> Self {
        Self {
            product_id: input.product_id,
            category_id: input.category_id,
        }
    }
}

impl Default for ProductCategory {
    fn default() -> Self {
        Self {
            product_id: String::default(),
            category_id: String::default(),
        }
    }
}
