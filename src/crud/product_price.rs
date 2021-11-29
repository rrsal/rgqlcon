use crate::crud::base::{get_current_date, CO};
use crate::gql::root::Ctx;
use crate::models::product_price::{NewProductPrice, ProductPrice, UpdateProductPrice};
use chrono::*;
use diesel::prelude::*;
use juniper::FieldResult;

impl CO for ProductPrice{
    type All = Vec<ProductPrice>;
    type Get = FieldResult<ProductPrice>;
    type Update = UpdateProductPrice;
    type New = NewProductPrice;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::product_price::dsl::*;
        let connection = ctx.db.get().unwrap();

        product_price
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error laoding product price")
    }

    
    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::product_price::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = product_price
            .filter(price_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(result)
    }

    fn create(&self, ctx: &Ctx, new_price: Self::New) -> Self::Get {
        use crate::schema::product_price::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let now = get_current_date();
        let new = Self::new(id,now,new_price);

        let result = diesel::insert_into(product_price)
            .values(new)
            .get_result::<Self>(&connection);
        
        match result {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
        }

    }

    fn update(&self, ctx: &Ctx, update_price: Self::Update) -> Self::Get {
        use crate::schema::product_price::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::update(update_price)
            .filter(price_id.eq(id))
            .set(update_price)
            .get_result::<Self>(&connection)?;
        Ok(result)
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::product_price::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::delete(product_price)
            .filter(price_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(result)
    }

}

impl Default for ProductPrice {
    fn default() -> Self {
        Self {
            price_id: String::from(""),
            date_from: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            product_id: String::from(""),
            price: 0.0
        }
    }
}
