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
    cart (cart_id) {
        cart_id -> Varchar,
        user_id -> Varchar,
        session_id -> Nullable<Varchar>,
        token -> Nullable<Varchar>,
        status -> Nullable<Bpchar>,
    }
}

table! {
    cart_items (item_id) {
        item_id -> Varchar,
        product_id -> Varchar,
        cart_id -> Varchar,
        sku -> Nullable<Varchar>,
        price -> Nullable<Float8>,
        discount -> Nullable<Float8>,
        quantity -> Nullable<Float8>,
        measure -> Nullable<Float8>,
        active -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    iorder (order_id) {
        order_id -> Varchar,
        user_id -> Varchar,
        session_id -> Nullable<Varchar>,
        token -> Nullable<Varchar>,
        status -> Nullable<Bpchar>,
        sub_total -> Nullable<Float8>,
        discount -> Nullable<Float8>,
        tax -> Nullable<Float8>,
        total -> Nullable<Float8>,
        promo -> Nullable<Varchar>,
    }
}

table! {
    order_items (item_id) {
        item_id -> Varchar,
        product_id -> Varchar,
        order_id -> Varchar,
        sku -> Nullable<Varchar>,
        price -> Nullable<Float8>,
        discount -> Nullable<Float8>,
        quantity -> Nullable<Float8>,
        measure -> Nullable<Float8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    product_category (category_id) {
        category_id -> Varchar,
        product_id -> Varchar,
    }
}

table! {
    product_price (price_id) {
        date_from -> Timestamp,
        product_id -> Varchar,
        price -> Float8,
        price_id -> Varchar,
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
        price -> Float8,
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
    transactions (transaction_id) {
        transaction_id -> Varchar,
        user_id -> Varchar,
        order_id -> Varchar,
        code -> Varchar,
        tran_type -> Nullable<Int4>,
        tran_mode -> Nullable<Int4>,
        status -> Nullable<Bpchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

joinable!(cart -> users (user_id));
joinable!(cart_items -> cart (cart_id));
joinable!(cart_items -> products (product_id));
joinable!(customer_address -> address (address_id));
joinable!(customer_address -> users (user_id));
joinable!(iorder -> users (user_id));
joinable!(order_items -> iorder (order_id));
joinable!(order_items -> products (product_id));
joinable!(product_category -> category (category_id));
joinable!(product_category -> products (product_id));
joinable!(product_price -> products (product_id));
joinable!(product_review -> products (product_id));
joinable!(product_tag -> products (product_id));
joinable!(product_tag -> tag (tag_id));
joinable!(transactions -> iorder (order_id));
joinable!(transactions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    address,
    cart,
    cart_items,
    category,
    customer_address,
    iorder,
    order_items,
    product_category,
    product_price,
    product_review,
    product_tag,
    products,
    tag,
    transactions,
    users,
);
