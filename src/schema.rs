table! {
    users (user_id) {
        user_id -> Varchar,
        first_name -> Varchar,
        middle_name -> Varchar,
        last_name -> Varchar,
        address_id -> Int4,
        email -> Varchar,
        phone -> Varchar,
        password_hash -> Varchar,
        registered_at -> Timestamp,
        last_login -> Timestamp,
        rating -> Int4,
        profile -> Text,
    }
}
