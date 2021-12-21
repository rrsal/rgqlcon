use crate::crud::base::CO;
use crate::gql::root::Ctx;
use crate::models::tag::{NewTag, Tag, UpdateTag};
use diesel::prelude::*;
use juniper::{graphql_value, FieldResult};

impl CO for Tag {
    type All = Vec<Tag>;
    type Get = FieldResult<Option<Tag>>;
    type Update = UpdateTag;
    type New = NewTag;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::tag::dsl::*;
        let connection = ctx.db.get().unwrap();
        tag.limit(100)
            .load::<Self>(&connection)
            .expect("Error loading users")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::tag::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = tag.filter(tag_id.eq(id)).first::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::tag::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let new = Self::new(id, new_data);

        let result = diesel::insert_into(tag)
            .values(new)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(Some(t)),
            Err(e) => FieldResult::Err(juniper::FieldError::new(
                format!("{}", e),
                graphql_value!({"code":"INTERNAL_SERVER_ERROR"}),
            )),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, update_data: Self::Update) -> Self::Get {
        use crate::schema::tag::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(tag)
            .filter(tag_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn delete(&self, ctx: &Ctx, id: String) -> FieldResult<Option<Tag>> {
        use crate::schema::tag::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(tag)
            .filter(tag_id.eq(id))
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(Some(t)),
            Err(e) => FieldResult::Err(juniper::FieldError::new(
                format!("{}", e),
                graphql_value!({"code":"INTERNAL_SERVER_ERROR"}),
            )),
        }
    }
}
