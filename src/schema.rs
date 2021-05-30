table! {
    address (address_id) {
        address_id -> Varchar,
        line_1 -> Varchar,
        line_2 -> Varchar,
        line_3 -> Varchar,
        city -> Varchar,
        zip_code -> Int4,
        state_province -> Varchar,
        country -> Varchar,
        other_details -> Varchar,
    }
}

table! {
    customer_address (date_from) {
        user_id -> Varchar,
        address_id -> Varchar,
        date_from -> Timestamp,
        date_to -> Timestamp,
        address_type -> Varchar,
    }
}

table! {
    products (product_id) {
        product_id -> Varchar,
        title -> Varchar,
        meta_title -> Varchar,
        summary -> Text,
        sku -> Varchar,
        p_type -> Varchar,
        price -> Numeric,
        discount -> Float8,
        quantity -> Float8,
        seller_id -> Varchar,
        create_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Timestamp,
        other_details -> Varchar,
        category_id -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Varchar,
        first_name -> Varchar,
        middle_name -> Varchar,
        last_name -> Varchar,
        address_id -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        password_hash -> Varchar,
        registered_at -> Timestamp,
        last_login -> Timestamp,
        rating -> Int4,
        profile -> Text,
    }
}

joinable!(customer_address -> address (address_id));
joinable!(customer_address -> users (user_id));

allow_tables_to_appear_in_same_query!(address, customer_address, products, users,);
