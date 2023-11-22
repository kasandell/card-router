// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        public_id -> Uuid,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    wallet (id) {
        id -> Int4,
        public_id -> Uuid,
        user_id -> Int4,
        #[max_length = 255]
        stripe_payment_method_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(wallet -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    wallet,
);
