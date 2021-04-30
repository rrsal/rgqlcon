use juniper::{ 
    RootNode,EmptyMutation,EmptySubscription,graphql_object
};

pub struct QueryRoot;



pub struct Users{
    user_id:i32,
    first_name:String,
    middle_name:String,
    last_name:String,
    email:String,
    phone:String,
    address_id:i32,
    password_hash:String,
    registered_at:String,
    last_login:String,
    rating:i32,
    profile:String
}


// Arbitrary context data.
pub struct Ctx;

impl juniper::Context for Ctx {}

#[graphql_object(context = Ctx)]
impl Users{
    pub fn user_id(&self)->i32{
        self.user_id
    }
    pub fn first_name(&self)->String{
        self.first_name.to_string()
    }
    pub fn middle_name(&self)->String{
        self.middle_name.to_string()
    }
    pub fn last_name(&self)->String{
        self.last_name.to_string()
    }
    pub fn email(&self)->String{
        self.email.to_string()
    }
    pub fn phone(&self)->String{
        self.phone.to_string()
    }
    pub fn address_id(&self)->i32{
        self.address_id
    }
    pub fn password_hash(&self)->String{
        self.password_hash.to_string()
    }
    pub fn registered_at(&self)->String{
        self.registered_at.to_string()
    }
    pub fn last_login(&self)->String{
        self.last_login.to_string()
    }
    pub fn rating(&self)->i32{
        self.rating
    }
    pub fn profile(&self)->String{
        self.profile.to_string()
    }
}

#[juniper::graphql_object(context = Ctx)]
impl QueryRoot{
    fn users() -> Vec<Users>{
        vec![
            Users{
                user_id:1,
                first_name:"user one".to_string(),
                middle_name:"user one".to_string(),
                last_name:"user one".to_string(),
                email:"user one".to_string(),
                phone:"user one".to_string(),
                address_id:1,
                password_hash:"user one".to_string(),
                registered_at:"user one".to_string(),
                last_login:"user one".to_string(),
                rating:1,
                profile:"user one".to_string(),
            },
            Users{
                user_id:2,
                first_name:"user two".to_string(),
                middle_name:"user two".to_string(),
                last_name:"user two".to_string(),
                email:"user two".to_string(),
                phone:"user two".to_string(),
                address_id:2,
                password_hash:"user two".to_string(),
                registered_at:"user two".to_string(),
                last_login:"user two".to_string(),
                rating:2,
                profile:"user two".to_string(),
            },
        ]
    }
}





pub type Schema = RootNode<'static,QueryRoot,EmptyMutation<Ctx>,EmptySubscription<Ctx>>;

pub fn create_schema()-> Schema{
    Schema::new(QueryRoot,EmptyMutation::<Ctx>::new(),EmptySubscription::<Ctx>::new())
}

