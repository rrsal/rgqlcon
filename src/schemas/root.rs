use juniper::{RootNode,EmptySubscription};
use crate::db::PgPool;
use crate::schemas::queries::QueryRoot;
use crate::schemas::mutations::MutationRoot;

// Arbitrary context data.
#[derive(Clone)]
pub struct Ctx{
    pub db:PgPool,
}
impl juniper::Context for Ctx {}

pub type Schema = RootNode<'static,QueryRoot,MutationRoot,EmptySubscription<Ctx>>;

pub fn create_schema()-> Schema{
    Schema::new(QueryRoot,MutationRoot,EmptySubscription::<Ctx>::new())
}
