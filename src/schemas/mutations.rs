use crate::schemas::models::usersqms;
use crate::schemas::models::user::{Users,UpdateUser,NewUser};
use crate::schemas::root::Ctx;
use juniper::{FieldResult};
pub struct MutationRoot;

#[juniper::graphql_object(context = Ctx)]
impl MutationRoot{
    fn create_user(new_user:NewUser,ctx:&Ctx)->FieldResult<Vec<Users>>{
        usersqms::create_user(new_user,&ctx)
    }

    fn update(ctx:&Ctx,id:String,updated_user:UpdateUser)->FieldResult<Users>{
        usersqms::update(&ctx ,id, updated_user)
    }
}