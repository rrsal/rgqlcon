use crate::crud::base::CO;
use crate::crud::user;
use crate::gql::root::Ctx;
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
}
