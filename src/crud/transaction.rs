use crate::crud::base::{get_current_date, CO};
use crate::gql::root::Ctx;
use crate::models::transaction::{NewTransaction, Transaction, UpdateTransaction};
use diesel::prelude::*;
use juniper::{graphql_value, FieldResult};

impl CO for Transaction {
    type All = Vec<Transaction>;
    type Get = FieldResult<Option<Transaction>>;
    type Update = UpdateTransaction;
    type New = NewTransaction;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::transactions::dsl::*;
        let connection = ctx.db.get().unwrap();
        transactions
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading users")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::transactions::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = transactions
            .filter(transaction_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::transactions::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let now = get_current_date();
        let new = Self::new(id, now, now, new_data);

        let result = diesel::insert_into(transactions)
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
        use crate::schema::transactions::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(transactions)
            .filter(transaction_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::transactions::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(transactions)
            .filter(transaction_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }
}
