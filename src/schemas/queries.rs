use crate::schemas::models::usersqms;
use crate::schemas::models::user::{Users};
use crate::schemas::root::Ctx;
use juniper::{FieldResult};
pub struct QueryRoot;

#[juniper::graphql_object(context = Ctx)]
impl QueryRoot{
    fn users(ctx:&Ctx)->Vec<Users>{
        usersqms::allusers(ctx)
    }

    fn user(ctx:&Ctx,user_id:String) -> FieldResult<Users>{
        usersqms::user(ctx, user_id)
    }
}