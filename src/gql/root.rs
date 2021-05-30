use juniper::{RootNode,EmptySubscription};
use crate::db::PgPool;
use crate::gql::queries::QueryRoot;
use crate::gql::mutations::MutationRoot;

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
