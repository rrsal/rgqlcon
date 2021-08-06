use crate::gql::root::Ctx;
use crate::models::product::{NewProduct,UpdateProduct,Products};
use chrono::*;
use diesel::prelude::*;
use juniper::FieldResult;


pub fn allproducts(ctx: &Ctx) -> Vec<Products> {
    use crate::schema::products::dsl::*;
    let connection = ctx.db.get().unwrap();
    products
        .limit(100)
        .load::<Products>(&connection)
        .expect("Error loading products")
}

pub fn product(ctx: &Ctx, pid: String) -> FieldResult<Products> {
    use crate::schema::products::dsl::*;
    let connection = ctx.db.get().unwrap();
    let result = products.filter(product_id.eq(pid)).first::<Products>(&connection)?;
    Ok(result)
}

pub fn create_product(ctx: &Ctx,new_product: NewProduct) -> FieldResult<Products> {
    use crate::schema::products::dsl::*;
    let connection = ctx.db.get().unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let current_date = chrono::offset::Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    let hour = current_date.hour();
    let minute = current_date.minute();
    let second = current_date.second();
    let product = Products{
        product_id: id,
        title: new_product.title,
        meta_title: new_product.meta_title,
        summary: new_product.summary,
        sku: new_product.sku,
        p_type: new_product.p_type,
        price: new_product.price,
        discount: new_product.discount,
        quantity: new_product.quantity,
        seller_id: new_product.seller_id,
        create_at:  NaiveDate::from_ymd(year,month,day).and_hms(hour,minute,second),
        updated_at: NaiveDate::from_ymd(year,month,day).and_hms(hour,minute,second),
        published_at: NaiveDate::from_ymd(year,month,day).and_hms(hour,minute,second),
        other_details: new_product.other_details,
        category_id: new_product.category_id
    };
    let res = diesel::insert_into(products)
    .values(product)
    .get_result::<Products>(&connection);

    match res{
        Ok(t)=> Ok(t),
        Err(e)=> FieldResult::Err(juniper::FieldError::from(e))
    }
}