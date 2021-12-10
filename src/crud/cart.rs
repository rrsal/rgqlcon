use crate::crud::base::{get_current_date, CO};
use crate::gql::root::Ctx;
use crate::models::cart::{Cart, CartInput, CartUpdateInput};
use crate::models::cart_item::{CartItemIds};
use crate::models::product_price::ProductPrice;
use chrono::NaiveDate;
use diesel::prelude::*;
use juniper::{graphql_value, FieldResult};

impl CO for Cart {
    type All = Vec<Cart>;
    type Get = FieldResult<Cart>;
    type Update = CartUpdateInput;
    type New = CartInput;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        cart.limit(100)
            .load::<Self>(&connection)
            .expect("Error loading cart")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = cart.filter(cart_id.eq(id)).first::<Self>(&connection)?;
        Ok(result)
    }

    fn create(&self, ctx: &Ctx, input: Self::New) -> Self::Get {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let now = get_current_date();
        let mut tot_price = 0.0;
        let mut tot_qty = 0.0;

        for item in cart_items.clone(){
            let default = ProductPrice::default();
            let price = default.by_id(ctx, item.id);
            tot_price += price.unwrap().price * item.quantity;
            tot_qty += item.quantity;
        }

        let new = Self::new(id, now, now, Some(tot_price), Some(tot_qty), input);

        let result = diesel::insert_into(cart)
            .values(new)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(juniper::FieldError::new(
                format!("{}", e),
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, input: Self::Update) -> Self::Get {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(cart)
            .filter(cart_id.eq(id))
            .set(input)
            .get_result::<Self>(&connection)?;
        Ok(result)
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(cart)
            .filter(cart_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(result)
    }
}

impl Default for Cart {
    fn default() -> Self {
        Self {
            cart_id: String::default(),
            user_id: String::default(),
            session_id: Some(String::default()),
            token: Some(String::default()),
            status: Some(String::default()),
            created_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            updated_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            total_price: Some(0.0),
            total_items: Some(0.0)
        }
    }
}