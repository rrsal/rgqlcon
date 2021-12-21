use crate::schema::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, GraphQLObject, Queryable, Insertable)]
#[graphql(description = "Tag")]
#[table_name = "tag"]
pub struct Tag {
    tag_id: String,
    title: Option<String>,
    meta_title: Option<String>,
    content: Option<String>,
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "New Tag")]
pub struct NewTag {
    title: Option<String>,
    meta_title: Option<String>,
    content: Option<String>,
}

#[derive(Debug, GraphQLInputObject, AsChangeset)]
#[graphql(description = "Update Tag")]
#[table_name = "tag"]
pub struct UpdateTag {
    title: Option<String>,
    meta_title: Option<String>,
    content: Option<String>,
}

impl Tag {
    pub fn new(id: String, input: NewTag) -> Self {
        Self {
            tag_id: id,
            title: input.title,
            meta_title: input.meta_title,
            content: input.content,
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            tag_id: String::default(),
            title: Some(String::default()),
            meta_title: Some(String::default()),
            content: Some(String::default()),
        }
    }
}
