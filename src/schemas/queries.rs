use crate::schemas::root::Ctx;
use crate::schemas::models::user::Users;
use diesel::prelude::*;

pub struct QueryRoot;

#[juniper::graphql_object(context = Ctx)]
impl QueryRoot{
    fn users(ctx:&Ctx) -> Vec<Users>{
        use crate::schema::users::dsl::{users};
        let connection = ctx.db.get().unwrap();
        users.limit(100).load::<Users>(&connection).expect("Error loading users")

    }
}