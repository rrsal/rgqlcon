use crate::schema::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, GraphQLObject, Queryable, Insertable)]
#[table_name = "product_tag"]
#[graphql(description = "A Product Tag")]
pub struct ProductTag {
    pub tag_id: String,
    pub product_id: String,
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "New Product Tag")]
pub struct NewProductTag {
    pub product_id: String,
}

#[derive(Debug, GraphQLInputObject, AsChangeset)]
#[table_name = "product_tag"]
#[graphql(description = "Update Tag")]
pub struct UpdateProductTag {
    pub tag_id: Option<String>,
    pub product_id: Option<String>,
}

impl ProductTag {
    pub fn new(id: String, input: NewProductTag) -> Self {
        Self {
            tag_id: id,
            product_id: input.product_id,
        }
    }
}

impl Default for ProductTag {
    fn default() -> Self {
        Self {
            tag_id: String::default(),
            product_id: String::default(),
        }
    }
}
