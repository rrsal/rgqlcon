use crate::crud::user;
use crate::models::user::{Users,UpdateUser,NewUser};
use crate::gql::root::Ctx;
use juniper::{FieldResult};
pub struct MutationRoot;

#[juniper::graphql_object(context = Ctx)]
impl MutationRoot{
    fn create_user(new_user:NewUser,ctx:&Ctx)->FieldResult<Vec<Users>>{
        user::create_user(new_user,&ctx)
    }

    fn update(ctx:&Ctx,id:String,updated_user:UpdateUser)->FieldResult<Users>{
        user::update(&ctx ,id, updated_user)
    }

    fn deleteUser(ctx: &Ctx, user_id: String) -> FieldResult<Users> {
        user::delete(&ctx, user_id)
    }
}