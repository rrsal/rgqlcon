use crate::schema::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable, Clone)]
#[graphql(description = "Order")]
#[table_name = "iorder"]
pub struct Order {
    pub order_id: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub token: Option<String>,
    pub status: Option<String>,
    pub sub_total: Option<f64>,
    pub discount: Option<f64>,
    pub tax: Option<f64>,
    pub total: Option<f64>,
    pub promo: Option<String>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "New Order")]
pub struct NewOrder {
    pub user_id: String,
    pub session_id: Option<String>,
    pub token: Option<String>,
    pub status: Option<String>,
    pub sub_total: Option<f64>,
    pub discount: Option<f64>,
    pub tax: Option<f64>,
    pub total: Option<f64>,
    pub promo: Option<String>,
}

#[derive(GraphQLInputObject, Debug, Clone, AsChangeset)]
#[graphql(description = "Order Update")]
#[table_name = "iorder"]
pub struct OrderUpdate {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub token: Option<String>,
    pub status: Option<String>,
    pub sub_total: Option<f64>,
    pub discount: Option<f64>,
    pub tax: Option<f64>,
    pub total: Option<f64>,
    pub promo: Option<String>,
}

impl Order {
    pub fn new(order_id: String, user_id: String, input: NewOrder) -> Self {
        Self {
            order_id,
            user_id,
            session_id: input.session_id,
            token: input.token,
            status: input.status,
            sub_total: input.sub_total,
            discount: input.discount,
            tax: input.tax,
            total: input.total,
            promo: input.promo,
        }
    }
}

impl Default for Order {
    fn default() -> Self {
        Self {
            order_id: String::default(),
            user_id: String::default(),
            session_id: None,
            token: None,
            status: None,
            sub_total: None,
            discount: None,
            tax: None,
            total: None,
            promo: None,
        }
    }
}
