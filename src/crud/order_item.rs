use crate::crud::base::{get_current_date, CO};
use crate::gql::root::Ctx;
use crate::models::order_item::{NewOrderItem, OrderItem, UpdateOrderItem};
use diesel::prelude::*;
use juniper::{graphql_value, FieldResult};

impl CO for OrderItem {
    type All = Vec<OrderItem>;
    type Get = FieldResult<Option<OrderItem>>;
    type Update = UpdateOrderItem;
    type New = NewOrderItem;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::order_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        order_items
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading users")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::order_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = order_items
            .filter(item_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::order_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let now = get_current_date();
        let new = Self::new(id, now, now, new_data);

        let result = diesel::insert_into(order_items)
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
        use crate::schema::order_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(order_items)
            .filter(item_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::order_items::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(order_items)
            .filter(item_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }
}
