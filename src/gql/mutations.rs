use crate::crud::base::CO;
use crate::crud::user;
use crate::gql::root::Ctx;
use crate::models::category::{Categories, NewCategory, UpdateCategory};
use crate::models::product::{NewProduct, Products, UpdateProduct};
use crate::models::product_price::{NewProductPrice, ProductPrice, UpdateProductPrice};
use crate::models::product_review::{NewProductReview, ProductReview, UpdateProductReview};
use crate::models::user::{NewUser, UpdateUser, Users};
use juniper::FieldResult;
pub struct MutationRoot;

#[juniper::graphql_object(context = Ctx)]
impl MutationRoot {
    fn create_user(new_user: NewUser, ctx: &Ctx) -> FieldResult<Vec<Users>> {
        user::create_user(new_user, ctx)
    }

    fn update(ctx: &Ctx, id: String, updated_user: UpdateUser) -> FieldResult<Users> {
        user::update(ctx, id, updated_user)
    }

    fn deleteUser(ctx: &Ctx, user_id: String) -> FieldResult<Users> {
        user::delete(ctx, user_id)
    }

    fn create_product(ctx: &Ctx, new_product: NewProduct) -> FieldResult<Products> {
        let product_default = Products::default();
        product_default.create(ctx, new_product)
    }

    fn update_product(ctx: &Ctx, id: String, new_product: UpdateProduct) -> FieldResult<Products> {
        let product_default = Products::default();
        product_default.update(ctx, id, new_product)
    }

    fn delete_product(ctx: &Ctx, id: String) -> FieldResult<Products> {
        let product_default = Products::default();
        product_default.delete(ctx, id)
    }

    fn create_category(ctx: &Ctx, new_categoty: NewCategory) -> FieldResult<Categories> {
        let category_default = Categories::default();
        category_default.create(ctx, new_categoty)
    }

    fn update_category(
        ctx: &Ctx,
        id: String,
        updated_category: UpdateCategory,
    ) -> FieldResult<Categories> {
        let category_default = Categories::default();
        category_default.update(ctx, id, updated_category)
    }

    fn delete_category(ctx: &Ctx, id: String) -> FieldResult<Categories> {
        let category_default = Categories::default();
        category_default.delete(ctx, id)
    }

    fn create_product_review(
        ctx: &Ctx,
        new_review: NewProductReview,
    ) -> FieldResult<ProductReview> {
        let review_default = ProductReview::default();
        review_default.create(ctx, new_review)
    }

    fn update_product_review(
        ctx: &Ctx,
        id: String,
        updated_review: UpdateProductReview,
    ) -> FieldResult<ProductReview> {
        let review_default = ProductReview::default();
        review_default.update(ctx, id, updated_review)
    }

    fn delete_product_review(ctx: &Ctx, id: String) -> FieldResult<ProductReview> {
        let review_default = ProductReview::default();
        review_default.delete(ctx, id)
    }

    fn create_product_price(ctx: &Ctx, new_price: NewProductPrice) -> FieldResult<ProductPrice> {
        let price_default = ProductPrice::default();
        price_default.create(ctx, new_price)
    }

    fn update_product_price(
        ctx: &Ctx,
        id: String,
        updated_price: UpdateProductPrice,
    ) -> FieldResult<ProductPrice> {
        let price_default = ProductPrice::default();
        price_default.update(ctx, id, updated_price)
    }
    fn delete_product_price(ctx: &Ctx, id: String) -> FieldResult<ProductPrice> {
        let price_default = ProductPrice::default();
        price_default.delete(ctx, id)
    }
}
