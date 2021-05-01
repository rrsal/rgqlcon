use juniper::{RootNode,EmptyMutation,EmptySubscription,GraphQLObject};

use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::PgPool;

pub struct QueryRoot;



#[derive(Queryable,GraphQLObject)]
pub struct Users{
    user_id:i32,
    first_name:String,
    middle_name:String,
    last_name:String,
    address_id:i32,
    email:String,
    phone:String,
    password_hash:String,
    registered_at:NaiveDateTime,
    last_login:NaiveDateTime,
    rating:i32,
    profile:String
}

// Arbitrary context data.
#[derive(Clone)]
pub struct Ctx{
    pub db:PgPool,
}

impl juniper::Context for Ctx {}

// when we need to return modified 
// data from database, we do not use `GraphQLObject` with the struct as in line 11
// and the code below can be used to fill the requirements.

// #[graphql_object(context = Ctx)]
// impl Users{
//     pub fn user_id(&self)->i32{
//         self.user_id
//     }
//     pub fn first_name(&self)->String{
//         self.first_name.to_string()
//     }
//     pub fn middle_name(&self)->String{
//         self.middle_name.to_string()
//     }
//     pub fn last_name(&self)->String{
//         self.last_name.to_string()
//     }
//     pub fn address_id(&self)->i32{
//         self.address_id
//     }
//     pub fn email(&self)->String{
//         self.email.to_string()
//     }
//     pub fn phone(&self)->String{
//         self.phone.to_string()
//     }
  
//     pub fn password_hash(&self)->String{
//         self.password_hash.to_string()
//     }
//     pub fn registered_at(&self)->String{
//         self.registered_at.to_string()
//     }
//     pub fn last_login(&self)->String{
//         self.last_login.to_string()
//     }
//     pub fn rating(&self)->i32{
//         self.rating
//     }
//     pub fn profile(&self)->String{
//         self.profile.to_string()
//     }
// }

#[juniper::graphql_object(context = Ctx)]
impl QueryRoot{
    fn users(ctx:&Ctx) -> Vec<Users>{
        use crate::schema::users::dsl::{users};
        let connection = ctx.db.get().unwrap();
        users.limit(100).load::<Users>(&connection).expect("Error loading users")

    }
}


pub type Schema = RootNode<'static,QueryRoot,EmptyMutation<Ctx>,EmptySubscription<Ctx>>;

pub fn create_schema()-> Schema{
    Schema::new(QueryRoot,EmptyMutation::<Ctx>::new(),EmptySubscription::<Ctx>::new())
}

