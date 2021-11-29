use crate::crud::base::{get_current_date, CO};
use crate::gql::root::Ctx;
use crate::models::product_review::{NewProductReview, ProductReview, UpdateProductReview};
use chrono::*;
use diesel::prelude::*;
use juniper::FieldResult;

impl CO for ProductReview {
    type All = Vec<ProductReview>;
    type Get = FieldResult<ProductReview>;
    type Update = UpdateProductReview;
    type New = NewProductReview;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::product_review::dsl::*;
        let connection = ctx.db.get().unwrap();
        product_review
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading products")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::product_review::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = product_review
            .filter(review_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(result)
    }

    fn create(&self, ctx: &Ctx, new_review: Self::New) -> Self::Get {
        use crate::schema::product_review::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let now = get_current_date();
        let new = Self::new(id, now, now, new_review);

        let result = diesel::insert_into(product_review)
            .values(new)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, updated_review: Self::Update) -> Self::Get {
        use crate::schema::product_review::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::update(product_review)
            .filter(review_id.eq(id))
            .set(updated_review)
            .get_result::<Self>(&connection)?;
        Ok(result)
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::product_review::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::delete(product_review)
            .filter(review_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(result)
    }
}

impl Default for ProductReview {
    fn default() -> Self {
        Self {
            review_id: String::from(""),
            product_id: String::from(""),
            parent_id: Some(String::from("")),
            title: Some(String::from("")),
            rating: Some(0),
            description: Some(String::from("")),
            created_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            published: Some(0),
            published_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
        }
    }
}
