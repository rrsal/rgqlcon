use crate::crud::base::CO;
use crate::gql::root::Ctx;
use crate::models::order::{NewOrder, Order, UpdateOrder};
use diesel::prelude::*;
use juniper::{graphql_value, FieldResult};

impl CO for Order {
    type All = Vec<Order>;
    type Get = FieldResult<Option<Order>>;
    type Update = UpdateOrder;
    type New = NewOrder;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::iorder::dsl::*;
        let connection = ctx.db.get().unwrap();
        iorder
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading users")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::iorder::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = iorder.filter(order_id.eq(id)).first::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::iorder::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let new = Self::new(id, new_data);

        let result = diesel::insert_into(iorder)
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
        use crate::schema::iorder::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(iorder)
            .filter(order_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::iorder::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(iorder)
            .filter(order_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }
}
