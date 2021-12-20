use crate::schema::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, GraphQLObject, Insertable)]
#[table_name = "category"]
pub struct Categories {
    pub category_id: String,
    pub title: Option<String>,
    pub meta_title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New Category for product")]
pub struct NewCategory {
    pub title: Option<String>,
    pub meta_title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
}

impl Categories {
    pub fn new(id: String, new_category: NewCategory) -> Self {
        Self {
            category_id: id,
            title: new_category.title,
            meta_title: new_category.meta_title,
            summary: new_category.summary,
            content: new_category.content,
        }
    }
}

#[derive(GraphQLInputObject, AsChangeset)]
#[graphql(description = "Update Category for product")]
#[table_name = "category"]
pub struct UpdateCategory {
    pub title: Option<String>,
    pub meta_title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
}
