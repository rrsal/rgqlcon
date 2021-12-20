use crate::crud::base::CO;
use crate::gql::root::Ctx;
use crate::models::customer_address::{CustomerAddress, NewCustomerAddress, UpdateCustomerAddress};
use diesel::prelude::*;
use juniper::FieldResult;

impl CO for CustomerAddress {
    type All = Vec<CustomerAddress>;
    type Get = FieldResult<Option<CustomerAddress>>;
    type Update = UpdateCustomerAddress;
    type New = NewCustomerAddress;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::customer_address::dsl::*;
        let connection = ctx.db.get().unwrap();
        customer_address
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading customer_address")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::customer_address::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = customer_address
            .filter(user_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::customer_address::dsl::*;
        let connection = ctx.db.get().unwrap();
        let new = Self::new(new_data);

        let result = diesel::insert_into(customer_address)
            .values(new)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(Some(t)),
            Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, update_data: Self::Update) -> Self::Get {
        use crate::schema::customer_address::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::update(customer_address.filter(user_id.eq(id)))
            .set(update_data)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(Some(t)),
            Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
        }
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::customer_address::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result =
            diesel::delete(customer_address.filter(user_id.eq(id))).get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(Some(t)),
            Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
        }
    }
}
