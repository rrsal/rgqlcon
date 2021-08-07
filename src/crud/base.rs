use crate::gql::root::Ctx;

pub trait CO {
    type All;
    type Get;
    type Update;
    type New;
    fn all(&self, ctx: &Ctx) -> Self::All;
    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get;
    fn create(&self,ctx:& Ctx, new_data:Self::New) -> Self::Get;
    fn update(&self, ctx: &Ctx, id: String, update_data: Self::Update) -> Self::Get;
    fn delete(&self,ctx:&Ctx,id:String) -> Self::Get;
}
