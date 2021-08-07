use crate::gql::root::Ctx;
use chrono::*;

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


pub fn getCurrentDate() -> NaiveDateTime {
    let current_date = chrono::offset::Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    let hour = current_date.hour();
    let minute = current_date.minute();
    let second = current_date.second();
    return NaiveDate::from_ymd(year,month,day).and_hms(hour,minute,second)
}
