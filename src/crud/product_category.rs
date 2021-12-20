use crate::crud::base::CO;
use crate::gql::root::Ctx;
use crate::models::product_category::{NewProductCategory, ProductCategory, UpdateProductCategory};
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

impl CO for ProductCategory {
    type All = Vec<ProductCategory>;
    type Get = FieldResult<Option<ProductCategory>>;
    type Update = UpdateProductCategory;
    type New = NewProductCategory;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::product_category::dsl::*;
        let connection = ctx.db.get().unwrap();
        product_category
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading product category")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::product_category::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = product_category
            .filter(product_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::product_category::dsl::*;
        let connection = ctx.db.get().unwrap();
        let new = Self::new(new_data);

        let result = diesel::insert_into(product_category)
            .values(new)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(Some(t)),
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, update_data: Self::Update) -> Self::Get {
        use crate::schema::product_category::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::update(product_category)
            .filter(product_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::product_category::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::delete(product_category)
            .filter(product_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }
}
