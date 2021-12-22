use crate::crud::base::CO;
use crate::crud::user;
use crate::gql::root::Ctx;
use crate::models::address::Addresses;
use crate::models::cart::{Cart, CartOutput};
use crate::models::cart_item::CartItem;
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

    fn user(ctx: &Ctx, user_id: String) -> FieldResult<Option<Users>> {
        user::user(ctx, user_id)
    }

    fn products(ctx: &Ctx) -> Vec<Products> {
        let default = Products::default();
        default.all(ctx)
    }

    fn product(ctx: &Ctx, id: String) -> FieldResult<Option<Products>> {
        let default = Products::default();
        default.by_id(ctx, id)
    }

    fn categories(ctx: &Ctx) -> Vec<Categories> {
        let default = Categories::default();
        default.all(ctx)
    }

    fn category(ctx: &Ctx, id: String) -> FieldResult<Option<Categories>> {
        let default = Categories::default();
        default.by_id(ctx, id)
    }

    fn reviews(ctx: &Ctx) -> Vec<ProductReview> {
        let default = ProductReview::default();
        default.all(ctx)
    }

    fn review(ctx: &Ctx, id: String) -> FieldResult<Option<ProductReview>> {
        let default = ProductReview::default();
        default.by_id(ctx, id)
    }

    fn prices(ctx: &Ctx) -> Vec<ProductPrice> {
        let price = ProductPrice::default();
        price.all(ctx)
    }

    fn price(ctx: &Ctx, id: String) -> FieldResult<Option<ProductPrice>> {
        let price = ProductPrice::default();
        price.by_id(ctx, id)
    }

    fn addresses(ctx: &Ctx) -> Vec<Addresses> {
        let default = Addresses::default();
        default.all(ctx)
    }

    fn address(ctx: &Ctx, id: String) -> FieldResult<Option<Addresses>> {
        let default = Addresses::default();
        default.by_id(ctx, id)
    }

    fn cart(ctx: &Ctx, id: String) -> FieldResult<Option<CartOutput>> {
        let cart = Cart::default().by_id(ctx, id);
        let mut items: Vec<CartItem> = Vec::new();
        let mut cart_output = None;
        if let Ok(Some(result)) = cart {
            let item_id_list = result.clone().cart_items.unwrap();
            for id in item_id_list {
                println!("{}", id);
                let cart_item = CartItem::default().by_id(ctx, id);
                if let Ok(Some(item)) = cart_item {
                    items.push(item);
                }
            }
            cart_output = Some(CartOutput::new(result, items));
        };
        Ok(cart_output)
    }

    fn carts(ctx: &Ctx) -> Vec<Cart> {
        let cart_default = Cart::default();
        cart_default.all(ctx)
    }
}
