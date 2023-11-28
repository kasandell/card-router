// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Int4,
        public_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    credit_card (id) {
        id -> Int4,
        public_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        credit_card_type_id -> Int4,
        credit_card_issuer_id -> Int4,
        #[max_length = 255]
        card_image_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    credit_card_issuer (id) {
        id -> Int4,
        public_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    credit_card_type (id) {
        id -> Int4,
        public_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    mcc_mapping (id) {
        id -> Int4,
        public_id -> Uuid,
        #[max_length = 4]
        mcc_code -> Varchar,
        category_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    passthrough_card (id) {
        id -> Int4,
        public_id -> Uuid,
        #[max_length = 255]
        passthrough_card_status -> Varchar,
        is_active -> Nullable<Bool>,
        user_id -> Int4,
        #[max_length = 255]
        token -> Varchar,
        expiration -> Date,
        #[max_length = 4]
        last_four -> Varchar,
        #[max_length = 255]
        passthrough_card_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rule (id) {
        id -> Int4,
        public_id -> Uuid,
        credit_card_id -> Int4,
        #[max_length = 4]
        rule_mcc -> Nullable<Varchar>,
        #[max_length = 255]
        merchant_name -> Nullable<Varchar>,
        points_multiplier -> Nullable<Int4>,
        cashback_percentage_bips -> Nullable<Int4>,
        #[max_length = 255]
        recurring_day_of_month -> Nullable<Varchar>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        #[max_length = 255]
        rule_status -> Varchar,
    }
}

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
        credit_card_id -> Int4,
    }
}

diesel::joinable!(credit_card -> credit_card_issuer (credit_card_issuer_id));
diesel::joinable!(credit_card -> credit_card_type (credit_card_type_id));
diesel::joinable!(mcc_mapping -> category (category_id));
diesel::joinable!(passthrough_card -> users (user_id));
diesel::joinable!(rule -> credit_card (credit_card_id));
diesel::joinable!(wallet -> credit_card (credit_card_id));
diesel::joinable!(wallet -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    credit_card,
    credit_card_issuer,
    credit_card_type,
    mcc_mapping,
    passthrough_card,
    rule,
    users,
    wallet,
);
