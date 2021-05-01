use juniper::{RootNode,EmptyMutation,EmptySubscription};
use crate::db::PgPool;
use crate::schemas::queries::QueryRoot;

// Arbitrary context data.
#[derive(Clone)]
pub struct Ctx{
    pub db:PgPool,
}

impl juniper::Context for Ctx {}


pub type Schema = RootNode<'static,QueryRoot,EmptyMutation<Ctx>,EmptySubscription<Ctx>>;

pub fn create_schema()-> Schema{
    Schema::new(QueryRoot,EmptyMutation::<Ctx>::new(),EmptySubscription::<Ctx>::new())
}
