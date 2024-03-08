use chrono::{NaiveDate, Utc};
use uuid::Uuid;
use crate::credit_card_type::entity::CreditCard;
use crate::data_error::DataError;
use crate::passthrough_card::entity::{create_test_lithic_card, PassthroughCard};
use crate::transaction::constant::ChargeStatus;
use crate::transaction::entity::{InnerChargeLedger, InsertableRegisteredTransaction, OuterChargeLedger, RegisteredTransaction, TransactionLedger, TransactionMetadata};
use crate::user::entity::{User, UserMessage};
use crate::wallet::entity::{InsertableCardAttempt, NewCard, Wallet, WalletCardAttempt};

#[cfg(test)]
pub async fn initialize_user() -> User {
    User::create(
        &UserMessage {
            email: "test@example.com",
            auth0_user_id: "1234",
        }
    ).await.expect("User should be created")
}

#[cfg(test)]
pub async fn initialize_wallet(
    user: &User,
    credit_card_id: i32
) -> (Wallet, WalletCardAttempt) {
    let ca = WalletCardAttempt::insert(
        &InsertableCardAttempt {
            user_id: user.id,
            credit_card_id: credit_card_id,
            expected_reference_id: "test"
        }
    ).await.expect("should create");
    let wallet = Wallet::insert_card(
        &NewCard {
            user_id: user.id,
            payment_method_id: "test",
            credit_card_id: credit_card_id,
            wallet_card_attempt_id: ca.id,

        }
    ).await.expect("should create");
    (wallet, ca)
}

#[cfg(test)]
pub async fn initialize_wallet_with_payment_method(
    user: &User,
    credit_card_id: i32,
    payment_method_id: String
) -> (Wallet, WalletCardAttempt) {
    let ca = WalletCardAttempt::insert(
        &InsertableCardAttempt {
            user_id: user.id,
            credit_card_id: credit_card_id,
            expected_reference_id: &Uuid::new_v4().to_string()
        }
    ).await.expect("should create");
    let wallet = Wallet::insert_card(
        &NewCard {
            user_id: user.id,
            payment_method_id: &payment_method_id,
            credit_card_id: credit_card_id,
            wallet_card_attempt_id: ca.id,

        }
    ).await.expect("should create");
    (wallet, ca)
}

#[cfg(test)]
pub async fn initialize_passthrough_card(
    user: &User
) -> PassthroughCard {
    PassthroughCard::create_from_api_card(
        &create_test_lithic_card(
            "09".to_string(),
            "2026".to_string(),
            "1234".to_string(),
            Uuid::new_v4()
        ),
        &user
    ).await.expect("should create card")
}

#[cfg(test)]
pub async fn initialize_registered_transaction_for_user(
    user: &User,
    metadata: &TransactionMetadata

) -> RegisteredTransaction {
    RegisteredTransaction::insert(
        &InsertableRegisteredTransaction {
            user_id: user.id,
            memo: &metadata.memo,
            amount_cents: metadata.amount_cents,
            mcc: &metadata.mcc
        }
    ).await.expect("should create")
}

pub fn default_transaction_metadata() -> TransactionMetadata {
    TransactionMetadata {
        amount_cents: 0,
        memo: "".to_string(),
        mcc: "7184".to_string()
    }
}


#[cfg(test)]
pub async fn create_user_in_mem(id: i32) -> User {
    User::create_test_user(id).await
}

#[cfg(test)]
pub fn create_failed_inner_charge(user_id: i32) -> InnerChargeLedger {
    InnerChargeLedger {
        id: 1,
        registered_transaction_id: 1,
        user_id: user_id,
        wallet_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Fail.as_str(),
        is_success: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

#[cfg(test)]
pub fn create_success_inner_charge(user_id: i32) -> InnerChargeLedger {
    InnerChargeLedger {
        id: 1,
        registered_transaction_id: 1,
        user_id: user_id,
        wallet_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Success.as_str(),
        is_success: Some(true),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

#[cfg(test)]
pub fn create_failed_outer_charge(user_id: i32) -> OuterChargeLedger {
    OuterChargeLedger {
        id: 1,
        registered_transaction_id: 1,
        user_id: user_id,
        passthrough_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Fail.as_str(),
        is_success: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

#[cfg(test)]
pub fn create_success_outer_charge(user_id: i32) -> OuterChargeLedger {
    OuterChargeLedger {
        id: 1,
        registered_transaction_id: 1,
        user_id: user_id,
        passthrough_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Success.as_str(),
        is_success: Some(true),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

#[cfg(test)]
pub fn create_full_transaction() -> TransactionLedger {
    TransactionLedger {
        id: 1,
        registered_transaction_id: 1,
        inner_charge_ledger_id: 1,
        outer_charge_ledger_id: 1,
    }
}

#[cfg(test)]
pub fn create_registered_transaction() -> RegisteredTransaction {
    RegisteredTransaction {
        id: 1,
        user_id: 1,
        transaction_id: Default::default(),
        memo: "".to_string(),
        amount_cents: 0,
        mcc: "".to_string(),
    }
}

#[cfg(test)]
pub fn create_passthrough_card(
    user: &User
) -> PassthroughCard {
    PassthroughCard {
        id: 0,
        public_id: Default::default(),
        passthrough_card_status: "".to_string(),
        is_active: Some(true),
        user_id: user.id,
        token: "".to_string(),
        expiration: NaiveDate::MAX,
        last_four: "1234".to_string(),
        passthrough_card_type: "VIRTUAL".to_string(),
        created_at: Default::default(),
        updated_at: Default::default(),
    }
}

#[cfg(test)]
pub fn create_credit_card(id: i32) -> CreditCard {
    CreditCard {
        id: id,
        public_id: Default::default(),
        name: "".to_string(),
        credit_card_type_id: 0,
        credit_card_issuer_id: 0,
        card_image_url: "".to_string(),
        created_at: Default::default(),
        updated_at: Default::default(),
    }

}

#[cfg(test)]
pub fn create_wallet_card_attempt(user_id: i32, credit_card_id: i32) -> WalletCardAttempt {
    WalletCardAttempt {
        id: 0,
        public_id: Default::default(),
        user_id: user_id,
        credit_card_id: credit_card_id,
        expected_reference_id: "".to_string(),
        psp_id: None,
        status: "".to_string(),
        recurring_detail_reference: None,
        created_at: Default::default(),
        updated_at: Default::default(),
    }
}

#[cfg(test)]
pub fn create_wallet_card(user_id: i32) -> Wallet {
    Wallet {
        id: 0,
        public_id: Default::default(),
        user_id: user_id,
        payment_method_id: "".to_string(),
        created_at: Default::default(),
        updated_at: Default::default(),
        credit_card_id: 0,
        wallet_card_attempt_id: 0,
    }
}
