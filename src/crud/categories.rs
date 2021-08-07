use crate::crud::base::CO;
use crate::gql::root::Ctx;
use crate::models::category::{Categories, NewCategory, UpdateCategory};
use diesel::prelude::*;
use juniper::FieldResult;

impl CO for Categories {
    type All = Vec<Categories>;
    type Get = FieldResult<Categories>;
    type Update = UpdateCategory;
    type New = NewCategory;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::category::dsl::*;
        let connection = ctx.db.get().unwrap();
        category
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading users")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::category::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = category
            .filter(category_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(result)
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::category::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let new = Self::new(id,new_data);

        let result = diesel::insert_into(category)
            .values(new)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, update_data: Self::Update) -> Self::Get {
        use crate::schema::category::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(category)
            .filter(category_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(result)
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::category::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(category)
            .filter(category_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(result)
    }
}

impl Default for Categories{
    fn default()-> Self{
        Self{
            category_id: String::from(""),
            title: Some(String::from("")),
            meta_title:Some(String::from("")),
            content:Some(String::from("")),
            summary:Some(String::from("")),
        }
    }
}
