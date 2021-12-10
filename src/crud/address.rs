use crate::crud::base::CO;
use crate::gql::root::Ctx;
use crate::models::address::{Addresses, NewAddress, UpdateAddress};
use diesel::prelude::*;
use juniper::FieldResult;

impl CO for Addresses {
    type All = Vec<Addresses>;
    type Get = FieldResult<Addresses>;
    type Update = UpdateAddress;
    type New = NewAddress;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::address::dsl::*;
        let connection = ctx.db.get().unwrap();

        address
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading addresses")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::address::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = address
            .filter(address_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(result)
    }

    fn create(&self, ctx: &Ctx, new_data: Self::New) -> Self::Get {
        use crate::schema::address::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let new = Self::new(id, new_data);

        let result = diesel::insert_into(address)
            .values(new)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, update_data: Self::Update) -> Self::Get {
        use crate::schema::address::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(address)
            .filter(address_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(result)
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::address::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(address)
            .filter(address_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(result)
    }
}


impl Default for Addresses {
    fn default() -> Self {
        Self {
            address_id: String::from(""),
            line_1: String::from(""),
            line_2: Some(String::from("")),
            line_3: Some(String::from("")),
            city: String::from(""),
            zip_code: 0,
            state_province: String::from(""),
            country: String::from(""),
            other_details: Some(String::from("")),
        }
    }
}
