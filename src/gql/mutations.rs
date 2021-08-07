use crate::crud::{user,product};
use crate::models::user::{Users,UpdateUser,NewUser};
use crate::models::product::{Products,NewProduct,UpdateProduct};
use crate::models::category::{Categories,NewCategory,UpdateCategory};
use crate::crud::base::CO;
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

    fn create_product(ctx: &Ctx, new_product: NewProduct) -> FieldResult<Products> {
        product::create_product(&ctx, new_product)
    }

    fn create_category(ctx:&Ctx, new_categoty:NewCategory) -> FieldResult<Categories> {
        let category_default = Categories::default();
        category_default.create(ctx,new_categoty)
    }

    fn update_category(ctx:&Ctx, id:String, updated_category:UpdateCategory) -> FieldResult<Categories>{
        let category_default = Categories::default();
        category_default.update(ctx,id, updated_category)
    }

    fn delete_category(ctx:&Ctx, id:String,) -> FieldResult<Categories>{
        let category_default = Categories::default();
        category_default.delete(ctx,id)
    }
}