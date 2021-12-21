use crate::schema::*;
use chrono::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Debug, Queryable, GraphQLObject, Insertable)]
#[graphql(description = "Transactions")]
#[table_name = "transactions"]
pub struct Transaction {
    transaction_id: String,
    user_id: String,
    order_id: String,
    code: String,
    tran_type: Option<i32>,
    tran_mode: Option<i32>,
    status: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "New Transactions")]
pub struct NewTransaction {
    user_id: String,
    order_id: String,
    code: String,
    tran_type: Option<i32>,
    tran_mode: Option<i32>,
    status: Option<String>,
}

#[derive(Debug, GraphQLInputObject, AsChangeset)]
#[graphql(description = "Update Transactions")]
#[table_name = "transactions"]
pub struct UpdateTransaction {
    user_id: Option<String>,
    order_id: Option<String>,
    code: Option<String>,
    tran_type: Option<i32>,
    tran_mode: Option<i32>,
    status: Option<String>,
}

impl Transaction {
    pub fn new(
        id: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        input: NewTransaction,
    ) -> Self {
        Self {
            transaction_id: id,
            user_id: input.user_id,
            order_id: input.order_id,
            code: input.code,
            tran_type: input.tran_type,
            tran_mode: input.tran_mode,
            status: input.status,
            created_at,
            updated_at,
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            transaction_id: String::default(),
            user_id: String::default(),
            order_id: String::default(),
            code: String::default(),
            tran_type: None,
            tran_mode: None,
            status: None,
            created_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
            updated_at: NaiveDate::from_ymd(2015, 6, 3).and_hms(9, 10, 11),
        }
    }
}
