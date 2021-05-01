use juniper::FieldResult;
use crate::schemas::models::user::{NewUser,Users};
use crate::schemas::root::Ctx;
use diesel::prelude::*;


use chrono::{NaiveDate};

pub struct MutationRoot;

#[juniper::graphql_object(context = Ctx)]
impl MutationRoot{
    fn create_user(new_user:NewUser,ctx:&Ctx)->FieldResult<Vec<Users>>{
        use crate::schema::users::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let user = Users{
            user_id:id,
            first_name:new_user.first_name,
            middle_name:new_user.middle_name,
            last_name:new_user.last_name,
            address_id:new_user.address_id,
            email:new_user.email.to_lowercase(),
            phone:new_user.phone,
            password_hash:new_user.password_hash,
            registered_at:NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            last_login:NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            rating:new_user.rating,
            profile:new_user.profile,
        };
        let res = diesel::insert_into(users)
        .values(user)
        .get_results::<Users>(&connection);
        match res{
            Ok(t)=> Ok(t),
            Err(e)=> FieldResult::Err(juniper::FieldError::from(e)),
        }
    }

    // fn update_user(){

    // }
}