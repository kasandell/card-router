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
    inner_charge_ledger (id) {
        id -> Int4,
        registered_transaction_id -> Uuid,
        user_id -> Int4,
        wallet_card_id -> Int4,
        amount_cents -> Int8,
        #[max_length = 255]
        status -> Varchar,
        is_success -> Nullable<Bool>,
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
    outer_charge_ledger (id) {
        id -> Int4,
        registered_transaction_id -> Uuid,
        user_id -> Int4,
        passthrough_card_id -> Int4,
        amount_cents -> Int8,
        #[max_length = 255]
        status -> Varchar,
        is_success -> Nullable<Bool>,
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
    registered_transactions (id) {
        id -> Int4,
        user_id -> Int4,
        transaction_id -> Uuid,
        #[max_length = 255]
        memo -> Varchar,
        amount_cents -> Int8,
        #[max_length = 255]
        mcc -> Varchar,
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
    transaction_ledger (id) {
        id -> Int4,
        transaction_id -> Uuid,
        inner_charge_ledger_id -> Int4,
        outer_charge_ledger_id -> Int4,
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
        payment_method_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        credit_card_id -> Int4,
        wallet_card_attempt_id -> Int4,
    }
}

diesel::table! {
    wallet_card_attempt (id) {
        id -> Int4,
        public_id -> Uuid,
        user_id -> Int4,
        credit_card_id -> Int4,
        #[max_length = 255]
        expected_reference_id -> Varchar,
        #[max_length = 255]
        psp_id -> Nullable<Varchar>,
        #[max_length = 255]
        status -> Varchar,
        #[max_length = 255]
        recurring_detail_reference -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(credit_card -> credit_card_issuer (credit_card_issuer_id));
diesel::joinable!(credit_card -> credit_card_type (credit_card_type_id));
diesel::joinable!(inner_charge_ledger -> users (user_id));
diesel::joinable!(inner_charge_ledger -> wallet (wallet_card_id));
diesel::joinable!(mcc_mapping -> category (category_id));
diesel::joinable!(outer_charge_ledger -> passthrough_card (passthrough_card_id));
diesel::joinable!(outer_charge_ledger -> users (user_id));
diesel::joinable!(passthrough_card -> users (user_id));
diesel::joinable!(registered_transactions -> users (user_id));
diesel::joinable!(rule -> credit_card (credit_card_id));
diesel::joinable!(transaction_ledger -> inner_charge_ledger (inner_charge_ledger_id));
diesel::joinable!(transaction_ledger -> outer_charge_ledger (outer_charge_ledger_id));
diesel::joinable!(wallet -> credit_card (credit_card_id));
diesel::joinable!(wallet -> users (user_id));
diesel::joinable!(wallet -> wallet_card_attempt (wallet_card_attempt_id));
diesel::joinable!(wallet_card_attempt -> credit_card (credit_card_id));
diesel::joinable!(wallet_card_attempt -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    credit_card,
    credit_card_issuer,
    credit_card_type,
    inner_charge_ledger,
    mcc_mapping,
    outer_charge_ledger,
    passthrough_card,
    registered_transactions,
    rule,
    transaction_ledger,
    users,
    wallet,
    wallet_card_attempt,
);
