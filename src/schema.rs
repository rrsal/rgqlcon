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
    category (category_id) {
        category_id -> Varchar,
        title -> Nullable<Varchar>,
        meta_title -> Nullable<Varchar>,
        summary -> Nullable<Text>,
        content -> Nullable<Text>,
        parent_id -> Varchar,
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
    product_category (category_id) {
        category_id -> Varchar,
        product_id -> Varchar,
    }
}

table! {
    product_price (date_from) {
        date_from -> Timestamp,
        product_id -> Varchar,
        product_price -> Float8,
    }
}

table! {
    product_review (review_id) {
        review_id -> Varchar,
        product_id -> Varchar,
        parent_id -> Varchar,
        title -> Nullable<Varchar>,
        rating -> Nullable<Int4>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        published -> Nullable<Int4>,
        published_at -> Timestamp,
    }
}

table! {
    product_tag (tag_id) {
        tag_id -> Varchar,
        product_id -> Varchar,
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
    tag (tag_id) {
        tag_id -> Varchar,
        title -> Nullable<Varchar>,
        meta_title -> Nullable<Varchar>,
        content -> Nullable<Text>,
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
joinable!(product_category -> category (category_id));
joinable!(product_category -> products (product_id));
joinable!(product_price -> products (product_id));
joinable!(product_review -> products (product_id));
joinable!(product_tag -> products (product_id));
joinable!(product_tag -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    address,
    category,
    customer_address,
    product_category,
    product_price,
    product_review,
    product_tag,
    products,
    tag,
    users,
);
