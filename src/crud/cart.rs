use crate::crud::base::{get_current_date, CO};
use crate::gql::root::Ctx;
use crate::models::cart::{Cart, CartInput, CartUpdateInput};
use crate::models::cart_item::{CartItem, NewCartItem};
use crate::models::product::Products;
use chrono::NaiveDate;
use diesel::prelude::*;
use juniper::{graphql_value, FieldResult};

impl CO for Cart {
    type All = Vec<Cart>;
    type Get = FieldResult<Option<Cart>>;
    type Update = CartUpdateInput;
    type New = CartInput;

    fn all(&self, ctx: &Ctx) -> Self::All {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        cart.limit(100)
            .load::<Self>(&connection)
            .expect("Error loading cart")
    }

    fn by_id(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = cart.filter(cart_id.eq(id)).first::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn create(&self, ctx: &Ctx, input: Self::New) -> Self::Get {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let now = get_current_date();
        let mut tot_price = 0.0;
        let mut tot_qty = 0.0;
        let cis = input.cart_items.clone().unwrap();
        let mut products = Vec::new();
        for item in cis {
            let product = Products::default().by_id(ctx, item.id.clone());
            if let Ok(Some(product_data)) = product {
                let new_item = NewCartItem {
                    product_id: item.id.clone(),
                    sku: Some(product_data.sku),
                    price: Some(product_data.price * item.quantity),
                    discount: Some(product_data.discount),
                    quantity: Some(item.quantity),
                    measure: None,
                    active: Some(1),
                };
                if let Ok(Some(item_id)) = CartItem::default().create(ctx, new_item) {
                    tot_price += item_id.price.unwrap();
                    tot_qty += item_id.quantity.unwrap();
                    products.push(item_id.item_id);
                }
            } else {
                print!("Error occured");
            }
        }

        let new = Self::new(
            id,
            now,
            now,
            Some(tot_price),
            Some(tot_qty),
            Some(products),
            input,
        );

        let result = diesel::insert_into(cart)
            .values(new)
            .get_result::<Self>(&connection);

        match result {
            Ok(t) => Ok(Some(t)),
            Err(e) => FieldResult::Err(juniper::FieldError::new(
                format!("{}", e),
                graphql_value!({ "code": "INTERNAL_SERVER_ERROR" }),
            )),
        }
    }

    fn update(&self, ctx: &Ctx, id: String, input: Self::Update) -> Self::Get {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::update(cart)
            .filter(cart_id.eq(id))
            .set(input)
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }

    fn delete(&self, ctx: &Ctx, id: String) -> Self::Get {
        use crate::schema::cart::dsl::*;
        let connection = ctx.db.get().unwrap();
        let result = diesel::delete(cart)
            .filter(cart_id.eq(id))
            .get_result::<Self>(&connection)?;
        Ok(Some(result))
    }
}

impl Default for Cart {
    fn default() -> Self {
        Self {
            cart_id: String::default(),
            user_id: String::default(),
            session_id: Some(String::default()),
            token: Some(String::default()),
            status: Some(String::default()),
            created_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            updated_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            total_price: Some(0.0),
            total_items: Some(0.0),
            cart_items: Some(Vec::default()),
        }
    }
}
