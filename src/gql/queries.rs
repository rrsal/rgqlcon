use crate::crud::base::CO;
use crate::crud::user;
use crate::gql::root::Ctx;
use crate::models::address::Addresses;
use crate::models::category::Categories;
use crate::models::product::Products;
use crate::models::product_price::ProductPrice;
use crate::models::product_review::ProductReview;
use crate::models::user::Users;
use juniper::FieldResult;
pub struct QueryRoot;

#[juniper::graphql_object(context = Ctx)]
impl QueryRoot {
    fn users(ctx: &Ctx) -> Vec<Users> {
        user::allusers(ctx)
    }

    fn user(ctx: &Ctx, user_id: String) -> FieldResult<Users> {
        user::user(ctx, user_id)
    }

    fn products(ctx: &Ctx) -> Vec<Products> {
        let product_default = Products::default();
        product_default.all(ctx)
    }

    fn product(ctx: &Ctx, id: String) -> FieldResult<Products> {
        let product_default = Products::default();
        product_default.by_id(ctx, id)
    }

    fn categories(ctx: &Ctx) -> Vec<Categories> {
        let category_default = Categories::default();
        category_default.all(ctx)
    }

    fn category(ctx: &Ctx, id: String) -> FieldResult<Categories> {
        let category_default = Categories::default();
        category_default.by_id(ctx, id)
    }

    fn reviews(ctx: &Ctx) -> Vec<ProductReview> {
        let review_default = ProductReview::default();
        review_default.all(ctx)
    }

    fn review(ctx: &Ctx, id: String) -> FieldResult<ProductReview> {
        let review_default = ProductReview::default();
        review_default.by_id(ctx, id)
    }

    fn prices(ctx: &Ctx) -> Vec<ProductPrice> {
        let product_price = ProductPrice::default();
        product_price.all(ctx)
    }

    fn price(ctx: &Ctx, id: String) -> FieldResult<ProductPrice> {
        let product_price = ProductPrice::default();
        product_price.by_id(ctx, id)
    }

    fn addresses(ctx: &Ctx) -> Vec<Addresses> {
        let address_default = Addresses::default();
        address_default.all(ctx)
    }

    fn address(ctx: &Ctx, id: String) -> FieldResult<Addresses> {
        let address_default = Addresses::default();
        address_default.by_id(ctx, id)
    }
}

impl Default for Addresses {
    fn default() -> Self {
        Self {
            address_id: String::from(""),
            line_1: String::from(""),
            line_2: Some(String::from("")),
            line_3: Some(String::from("")),
            city: String::from(""),
            zip_code: 0,
            state_province: String::from(""),
            country: String::from(""),
            other_details: Some(String::from("")),
        }
    }
}
