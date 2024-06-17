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
    expected_wallet_charge_reference (id) {
        id -> Int4,
        registered_transaction_id -> Int4,
        reference_id -> Uuid,
        user_id -> Int4,
        wallet_card_id -> Int4,
        amount_cents -> Int4,
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
    passthrough_card_charge (id) {
        id -> Int4,
        registered_transaction_id -> Int4,
        user_id -> Int4,
        passthrough_card_id -> Int4,
        amount_cents -> Int4,
        #[max_length = 255]
        status -> Varchar,
        is_success -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pending_passthrough_card_transaction_ledger (id) {
        id -> Int4,
        registered_transaction_id -> Int4,
        user_id -> Int4,
        passthrough_card_id -> Int4,
        #[max_length = 20]
        money_movement_direction -> Varchar,
        #[max_length = 40]
        money_movement_type -> Varchar,
        amount_cents -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pending_wallet_transaction_ledger (id) {
        id -> Int4,
        registered_transaction_id -> Int4,
        user_id -> Int4,
        wallet_id -> Int4,
        #[max_length = 20]
        money_movement_direction -> Varchar,
        #[max_length = 40]
        money_movement_type -> Varchar,
        amount_cents -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    registered_transaction (id) {
        id -> Int4,
        user_id -> Int4,
        transaction_id -> Uuid,
        #[max_length = 255]
        memo -> Varchar,
        amount_cents -> Int4,
        #[max_length = 255]
        mcc -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    registered_transaction_metadata (registered_transaction_id) {
        registered_transaction_id -> Int4,
        body -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rule (id) {
        id -> Int4,
        public_id -> Uuid,
        credit_card_id -> Int4,
        rule_category_id -> Nullable<Int4>,
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
    settled_passthrough_card_transaction_ledger (id) {
        id -> Int4,
        registered_transaction_id -> Int4,
        user_id -> Int4,
        passthrough_card_id -> Int4,
        #[max_length = 20]
        money_movement_direction -> Varchar,
        #[max_length = 40]
        money_movement_type -> Varchar,
        amount_cents -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    settled_wallet_transaction_ledger (id) {
        id -> Int4,
        registered_transaction_id -> Int4,
        user_id -> Int4,
        wallet_id -> Int4,
        #[max_length = 20]
        money_movement_direction -> Varchar,
        #[max_length = 40]
        money_movement_type -> Varchar,
        amount_cents -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    successful_end_to_end_charge (id) {
        id -> Int4,
        registered_transaction_id -> Int4,
        wallet_card_charge_id -> Int4,
        passthrough_card_charge_id -> Int4,
        public_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        public_id -> Uuid,
        email -> Text,
        auth0_user_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        footprint_vault_id -> Varchar,
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
        #[max_length = 20]
        status -> Varchar,
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
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    wallet_card_charge (id) {
        id -> Int4,
        registered_transaction_id -> Int4,
        user_id -> Int4,
        wallet_card_id -> Int4,
        amount_cents -> Int4,
        rule_id -> Nullable<Int4>,
        expected_wallet_charge_reference_id -> Int4,
        #[max_length = 255]
        resolved_charge_status -> Varchar,
        #[max_length = 255]
        psp_reference -> Nullable<Varchar>,
        #[max_length = 255]
        returned_reference -> Nullable<Varchar>,
        #[max_length = 30]
        returned_charge_status -> Nullable<Varchar>,
        is_success -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        public_id -> Uuid,
    }
}

diesel::table! {
    wallet_status_history (id) {
        id -> Int4,
        public_id -> Uuid,
        wallet_id -> Int4,
        #[max_length = 20]
        prior_status -> Varchar,
        #[max_length = 20]
        current_status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(credit_card -> credit_card_issuer (credit_card_issuer_id));
diesel::joinable!(credit_card -> credit_card_type (credit_card_type_id));
diesel::joinable!(expected_wallet_charge_reference -> registered_transaction (registered_transaction_id));
diesel::joinable!(expected_wallet_charge_reference -> users (user_id));
diesel::joinable!(expected_wallet_charge_reference -> wallet (wallet_card_id));
diesel::joinable!(mcc_mapping -> category (category_id));
diesel::joinable!(passthrough_card -> users (user_id));
diesel::joinable!(passthrough_card_charge -> passthrough_card (passthrough_card_id));
diesel::joinable!(passthrough_card_charge -> registered_transaction (registered_transaction_id));
diesel::joinable!(passthrough_card_charge -> users (user_id));
diesel::joinable!(pending_passthrough_card_transaction_ledger -> passthrough_card (passthrough_card_id));
diesel::joinable!(pending_passthrough_card_transaction_ledger -> registered_transaction (registered_transaction_id));
diesel::joinable!(pending_passthrough_card_transaction_ledger -> users (user_id));
diesel::joinable!(pending_wallet_transaction_ledger -> registered_transaction (registered_transaction_id));
diesel::joinable!(pending_wallet_transaction_ledger -> users (user_id));
diesel::joinable!(pending_wallet_transaction_ledger -> wallet (wallet_id));
diesel::joinable!(registered_transaction -> users (user_id));
diesel::joinable!(registered_transaction_metadata -> registered_transaction (registered_transaction_id));
diesel::joinable!(rule -> category (rule_category_id));
diesel::joinable!(rule -> credit_card (credit_card_id));
diesel::joinable!(settled_passthrough_card_transaction_ledger -> passthrough_card (passthrough_card_id));
diesel::joinable!(settled_passthrough_card_transaction_ledger -> registered_transaction (registered_transaction_id));
diesel::joinable!(settled_passthrough_card_transaction_ledger -> users (user_id));
diesel::joinable!(settled_wallet_transaction_ledger -> registered_transaction (registered_transaction_id));
diesel::joinable!(settled_wallet_transaction_ledger -> users (user_id));
diesel::joinable!(settled_wallet_transaction_ledger -> wallet (wallet_id));
diesel::joinable!(successful_end_to_end_charge -> passthrough_card_charge (passthrough_card_charge_id));
diesel::joinable!(successful_end_to_end_charge -> registered_transaction (registered_transaction_id));
diesel::joinable!(successful_end_to_end_charge -> wallet_card_charge (wallet_card_charge_id));
diesel::joinable!(wallet -> credit_card (credit_card_id));
diesel::joinable!(wallet -> users (user_id));
diesel::joinable!(wallet -> wallet_card_attempt (wallet_card_attempt_id));
diesel::joinable!(wallet_card_attempt -> credit_card (credit_card_id));
diesel::joinable!(wallet_card_attempt -> users (user_id));
diesel::joinable!(wallet_card_charge -> expected_wallet_charge_reference (expected_wallet_charge_reference_id));
diesel::joinable!(wallet_card_charge -> registered_transaction (registered_transaction_id));
diesel::joinable!(wallet_card_charge -> rule (rule_id));
diesel::joinable!(wallet_card_charge -> users (user_id));
diesel::joinable!(wallet_card_charge -> wallet (wallet_card_id));
diesel::joinable!(wallet_status_history -> wallet (wallet_id));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    credit_card,
    credit_card_issuer,
    credit_card_type,
    expected_wallet_charge_reference,
    mcc_mapping,
    passthrough_card,
    passthrough_card_charge,
    pending_passthrough_card_transaction_ledger,
    pending_wallet_transaction_ledger,
    registered_transaction,
    registered_transaction_metadata,
    rule,
    settled_passthrough_card_transaction_ledger,
    settled_wallet_transaction_ledger,
    successful_end_to_end_charge,
    users,
    wallet,
    wallet_card_attempt,
    wallet_card_charge,
    wallet_status_history,
);
