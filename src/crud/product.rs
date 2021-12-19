use crate::crud::base::{get_current_date, CO};
use crate::gql::root::Ctx;
use crate::models::product::{NewProduct, Products, UpdateProduct};
use crate::models::product_category::{NewProductCategory, ProductCategory};
use chrono::*;
use diesel::prelude::*;
use juniper::FieldResult;

impl CO for Products {
    type All = Vec<Products>;
    type Get = FieldResult<Products>;
    type Update = UpdateProduct;
    type New = NewProduct;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::products::dsl::*;
        let connection = ctx.db.get().unwrap();
        products
            .limit(100)
            .load::<Self>(&connection)
            .expect("Error loading products")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::products::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = products
            .filter(product_id.eq(id))
            .first::<Self>(&connection)?;
        Ok(result)
    }

    fn create(&self, ctx: &Ctx, new_product: Self::New) -> Self::Get {
        use crate::schema::products::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = String::from(uuid::Uuid::new_v4().to_string());
        let category_id = new_product.category_id.clone();
        let now = get_current_date();
        let new = Self::new(id.clone(), now, now, now, new_product);

        let res = diesel::insert_into(products)
            .values(new)
            .get_result::<Self>(&connection);

        match res {
            Ok(t) => {
                let new_pc = NewProductCategory {
                    product_id: id.clone(),
                    category_id,
                };
                let default = ProductCategory::default();
                let pc_result = default.create(ctx, new_pc);
                match pc_result {
                    Ok(_) => Ok(t),
                    Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
                }
            }
            Err(e) => FieldResult::Err(juniper::FieldError::from(e)),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, update_data: Self::Update) -> Self::Get {
        use crate::schema::products::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::update(products)
            .filter(product_id.eq(id))
            .set(update_data)
            .get_result::<Self>(&connection)?;
        Ok(result)
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::products::dsl::*;
        let connection = ctx.db.get().unwrap();

        let result = diesel::delete(products)
            .filter(product_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(result)
    }
}

impl Default for Products {
    fn default() -> Self {
        Self {
            product_id: String::from(""),
            title: String::from(""),
            meta_title: String::from(""),
            summary: String::from(""),
            sku: String::from(""),
            p_type: String::from(""),
            price: f64::INFINITY,
            discount: 0.0,
            quantity: 0.0,
            seller_id: String::from(""),
            created_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            updated_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            published_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            other_details: String::from(""),
        }
    }
}
