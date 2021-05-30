use crate::crud::user;
use crate::gql::root::Ctx;
use crate::models::user::Users;
use juniper::FieldResult;
pub struct QueryRoot;

#[juniper::graphql_object(context = Ctx)]
impl QueryRoot {
    fn users(ctx: &Ctx) -> Vec<Users> {
        user::allusers(ctx)
    }

    fn user(ctx: &Ctx, user_id: String) -> FieldResult<Users> {
        user::user(ctx, user_id)
    }
}
