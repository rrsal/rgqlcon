se crate::crud::base::CO;
use crate::gql::root::Ctx;
use crate::models::cart_item::{CartItem, NewCartItem, UpdateCartItem};
use diesel::prelude::*;
use juniper::{graphql_value, FieldResult};

impl CO for CartItem {
    type All = Vec<CartItem>;
    type Get = FieldResult<Option<CartItem>>;
    type Update = UpdateCartItem;
    type New = NewCartItem;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::cart_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        cart_items
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error in loading cart items")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::cart_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = cart_items.filter(item_id.eq(id)).first::<Self>(&connection);
        match result {
            Ok(t) => {
                let x = Ok(Some(t));
                x
            }
            Err(e) => FieldResult::Err(juniper::FieldError::new(
                format!("{}", e),
                graphql_value!({"code":"INTERNAL_SERVER_ERROR"}),
            )),
        }
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::cart_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let new = Self::new(id, new_data);

        let result = diesel::insert_into(cart_items)
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
        use crate::schema::cart_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(cart_items)
            .filter(item_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::cart_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(cart_items)
            .filter(item_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }
}

impl Default for CartItem {
    fn default() -> Self {
        Self {
            item_id: String::default(),
            product_id: String::default(),
            sku: Some(String::default()),
            price: Some(0.0),
            discount: Some(0.0),
            quantity: Some(0.0),
            measure: Some(0.0),
            active: Some(0),
        }
    }
}
