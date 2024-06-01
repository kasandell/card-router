use std::collections::HashSet;
use adyen_checkout::models::{Amount, PaymentCancelResponse, PaymentRequest, PaymentRequestPaymentMethod, PaymentResponse};
use std::sync::Arc;
use adyen_checkout::models::payment_response::ResultCode;
use async_trait::async_trait;
use lazy_static::lazy_static;
use mockall::automock;
use uuid::Uuid;
use crate::asa::request::AsaRequest;
use crate::asa::response::AsaResponseResult;
use crate::charge::constant::{ChargeCardAttemptResult, ChargeEngineResult, ChargeStatus};
use crate::charge::dao::{ChargeDao, ChargeDaoTrait};
use crate::charge::entity::{ExpectedWalletChargeReference, InsertableExpectedWalletChargeReference, InsertablePassthroughCardCharge, InsertableRegisteredTransaction, InsertableSuccessfulEndToEndCharge, InsertableWalletCardCharge, PassthroughCardCharge, RegisteredTransaction, WalletCardCharge};
use crate::charge::error::ChargeError;
use crate::charge::model::{RegisteredTransactionModel, SuccessfulEndToEndChargeModel};
use crate::common::model::TransactionMetadata;
use crate::error::data_error::DataError;
use crate::footprint::error::FootprintError;
use crate::footprint::request::ChargeThroughProxyRequest;
use crate::footprint::service::FootprintServiceTrait;
use crate::ledger::error::LedgerError;
use crate::ledger::model::PendingPassthroughCardTransactionLedgerModel;
use crate::passthrough_card::model::PassthroughCardModel as PassthroughCard;
use crate::user::model::UserModel as User;
use crate::wallet::model::WalletModelWithRule as Wallet;
use crate::ledger::service::LedgerServiceTrait;
use crate::user::service::UserServiceTrait;
use crate::util::error::UtilityError::DateError;
use crate::util::transaction::{Transaction, transactional};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
// TODO: do not group ledger calls in with payment calls in txn. need ledger data to go in always?
pub trait ChargeServiceTrait {
    async fn charge_from_asa_request(
        self: Arc<Self>,
        request: &AsaRequest,
        wallet: &Vec<Wallet>,
        passthrough_card: &PassthroughCard,
        user: &User,
    ) -> Result<AsaResponseResult, ChargeError>;
}

pub struct ChargeService {
    user_service: Arc<dyn UserServiceTrait>,
    ledger_service: Arc<dyn LedgerServiceTrait + Send + Sync>,
    footprint_service: Arc<dyn FootprintServiceTrait>,
    dao: Arc<dyn ChargeDaoTrait + Send + Sync>
}

lazy_static! {
    static ref ACCEPTABLE_STATUSES: HashSet<ResultCode> = HashSet::from(
        [
            ResultCode::Authorised
        ]
    );
    static ref FINAL_STATE_ERROR_CODES: HashSet<ResultCode> = HashSet::from(
        [
            ResultCode::Cancelled,
            ResultCode::Error,
            ResultCode::Refused
        ]
    );
}



#[async_trait(?Send)]
impl ChargeServiceTrait for ChargeService {
    #[tracing::instrument(skip(self))]
    async fn charge_from_asa_request(
        self: Arc<Self>,
        request: &AsaRequest,
        wallet: &Vec<Wallet>,
        passthrough_card: &PassthroughCard,
        user: &User,
    ) -> Result<AsaResponseResult, ChargeError> {
        tracing::info!("Starting charge");
        let metadata = TransactionMetadata::convert(&request)
            .map_err(|e| {
                tracing::error!("Error converting to required metadata");
                ChargeError::Unexpected(e.into())
            })?;
        let card = request.card.clone().ok_or_else(|| {
            tracing::error!("No card found to charge in request");
            ChargeError::NoCardInRequest
        })?;
        let token = card.token.clone().ok_or_else(|| {
            tracing::error!("No token found for card in request");
            ChargeError::NoCardInRequest
        })?;

        tracing::info!("Registering transaction");

        tracing::info!("Placing hold on passthrough funds");
        let registered_transaction = self.clone().register_transaction_and_pending_passthrough_card_charge(
            &user,
            &metadata,
            &passthrough_card
        ).await?;

        tracing::info!("Registered transaction with public_id={}", &registered_transaction.transaction_id);

        tracing::info!("Charging wallet");
        let (charge_result, ledger) = self.clone().charge_wallet(&user, wallet, &metadata, &registered_transaction).await?;

        tracing::info!("Charged wallet with result={:?}", &charge_result);
        return match charge_result {
            ChargeEngineResult::Approved => {
                return match ledger {
                    Some(ledger) => {
                        // TODO: should verify that this is success
                        tracing::info!("Charge success, registering in ledger for transaction={}", &registered_transaction.transaction_id);
                        self.clone().register_successful_passthrough_card_charge(&registered_transaction, &ledger, &passthrough_card).await?;
                        Ok(AsaResponseResult::from(charge_result))
                    },
                    None => {
                        tracing::warn!("Outer transaction came in with no registered inner transaction ledgers");
                        tracing::warn!("Registering failed outer charge for transaction={}", &registered_transaction.transaction_id);
                        self.clone().register_failed_passthrough_card_charge(&registered_transaction, &passthrough_card).await?;
                        // TODO: this might actually just mean user has no cards
                        Err(ChargeError::Unexpected("Approved inner charge with no ledger entry, should not be possible".into()))
                    }
                }
            },
            _ => {
                tracing::warn!("Registering failed outer charge for transaction={}", &registered_transaction.transaction_id);
                self.clone().register_failed_passthrough_card_charge(&registered_transaction, &passthrough_card).await?;
                Ok(AsaResponseResult::from(charge_result))
            }
        }
    }
}

// TODO: probably need this to be a threadsafe singleton to avoid reinit everywhere
impl ChargeService {

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    pub fn new_with_services(
        user_service: Arc<dyn UserServiceTrait>,
        ledger_service: Arc<dyn LedgerServiceTrait + Send + Sync>,
        footprint_service: Arc<dyn FootprintServiceTrait>
    ) -> Self {
        Self {
            user_service,
            ledger_service,
            footprint_service,
            dao: Arc::new(ChargeDao::new()),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn charge_wallet(
        self: Arc<Self>,
        user: &User,
        wallet: &Vec<Wallet>,
        transaction_metadata: &TransactionMetadata,
        registered_transaction: &RegisteredTransactionModel
    ) -> Result<(ChargeEngineResult, Option<WalletCardCharge>), ChargeError> {
        // iterate through the users wallet, charging one and ONLY ONE card
        tracing::info!("Charging {} cards for user={}", wallet.len(), user.id);
        let idempotency_key = Uuid::new_v4();
        let mut success_charge = false;
        let mut codes : Vec<ChargeCardAttemptResult> = vec![];
        let mut ledger_res: Option<WalletCardCharge> = None;
        for card in wallet {
            if success_charge { break; }
            if let Ok((charge_attempt, ledger)) = self.clone().charge_card_with_cleanup(
                idempotency_key,
                card,
                user,
                transaction_metadata,
                registered_transaction
            ).await {
                tracing::info!("Charged card={} for user={} with result={:?}", card.id, &user.id, &charge_attempt);
                success_charge = bool::from(&charge_attempt);
                ledger_res = ledger;
                codes.push(charge_attempt)
            }
        }
        if success_charge {
            tracing::info!("Successfully charged a card for user={}", &user.id);
            Ok((ChargeEngineResult::Approved, ledger_res))
        } else {
            tracing::warn!("Unable to charge a card for user={}", &user.id);
            Ok((ChargeEngineResult::Denied, ledger_res))
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn charge_card_with_cleanup(
        self: Arc<Self>,
        idempotency_key: Uuid,
        card: &Wallet,
        user: &User,
        transaction_metadata: &TransactionMetadata,
        registered_transaction: &RegisteredTransactionModel
    ) -> Result<(ChargeCardAttemptResult, Option<WalletCardCharge>), ChargeError> {
        tracing::info!("Charging card with cleanup for user={} card={}", &user.id, card.id);

        let wallet_reserve = self.clone().register_reserve_wallet_charge(
            registered_transaction,
            card,

        ).await?;

        let resp = self.footprint_service.clone().proxy_adyen_payment_request(
            &ChargeThroughProxyRequest {
                amount_cents: transaction_metadata.amount_cents as i32, // TODO: edit model to be i32
                mcc: &transaction_metadata.mcc,
                payment_method_id: &card.payment_method_id,
                customer_public_id: &user.public_id.to_string(), // needed to proxy the data in correctly. should change arg name
                footprint_vault_id: &user.footprint_vault_id.to_string(), // needed to proxy the data in correctly. should change arg name
                idempotency_key: &idempotency_key,
                reference: &wallet_reserve.reference_id.to_string(),
                statement: &transaction_metadata.memo
            }
        ).await;
        tracing::info!("Made request through proxy to charge card");

        if let Ok(response) = resp {
            if let Some(code) = response.result_code {
                tracing::info!("Checkout returned code={:?} for card={} user={}", code, card.id, user.id);
                if ACCEPTABLE_STATUSES.contains(&code) {
                    tracing::info!("Charged card={} for user={}", card.id, user.id);
                    let wallet_charge = self.register_successful_wallet_charge(registered_transaction, card, &wallet_reserve, &response).await?;
                    tracing::info!("Registered successful inner charge in ledger for transaction={} id={}", &registered_transaction.transaction_id, &wallet_charge.id);
                    return Ok((ChargeCardAttemptResult::from(code), Some(wallet_charge)));
                    //add to ledger
                } else if FINAL_STATE_ERROR_CODES.contains(&code) {
                    tracing::warn!("Error charging card={} for user={}", card.id, user.id);
                    let wallet_charge = self.clone().register_failed_wallet_charge(registered_transaction, card, &wallet_reserve, &response).await?;
                    tracing::warn!("Registered unsuccessful inner charge in ledger for transaction={} id={}", &registered_transaction.transaction_id, &wallet_charge.id);
                    return Ok((ChargeCardAttemptResult::Denied, Some(wallet_charge)));
                    //can safely bypass this branch
                } else {
                    tracing::warn!("Intermediate state needs cleanup");
                    let wallet_charge = self.clone().register_partial_wallet_charge(registered_transaction, card, &wallet_reserve, &response).await?;
                    return Ok((ChargeCardAttemptResult::Denied, Some(wallet_charge)));
                    /*
                    tracing::warn!("Intermediate state needs cleanup for card={} for user={}", card.id, user.id);
                    if let Some(psp) = response.psp_reference {
                        // TODO: move this call to proxy
                        tracing::warn!("Cancelling transaction for user={} card={}", &user.id, card.id);
                        let cancel = self.footprint_service.clone().proxy_adyen_cancel_request(
                            &psp
                        ).await;
                        if let Ok(cancel) = cancel {
                            tracing::info!("Cancelled with status: {:?}", cancel.status);
                            let ledger_entry = self.ledger_service.clone().register_failed_inner_charge(
                                registered_transaction,
                                transaction_metadata,
                                card
                            ).await?;
                            tracing::warn!("Registered unsuccessful inner charge in ledger for transaction={} id={}", &registered_transaction.transaction_id, ledger_entry.id);
                            return Ok((ChargeCardAttemptResult::PartialCancelSucceeded, Some(ledger_entry)));
                            //cancel received. block on webhook response?
                        } else {
                            tracing::error!("Error cancelling unsuccessful payment with psp={}", psp);
                            let ledger_entry = self.ledger_service.clone().register_failed_inner_charge(
                                registered_transaction,
                                transaction_metadata,
                                card
                            ).await?;
                            tracing::error!("Registered unsuccessful inner charge in ledger for transaction={} id={}, requires further cleanup", &registered_transaction.transaction_id, ledger_entry.id);
                            return Ok((ChargeCardAttemptResult::PartialCancelFailed, Some(ledger_entry)));
                            // TODO: error cancelling. figure out what to do
                        }
                    }
                     */
                }
            } else {
                let wallet_charge = self.clone().register_failed_wallet_charge(
                    &registered_transaction, &card, &wallet_reserve, &response
                ).await?;
                return Ok((ChargeCardAttemptResult::Denied, Some(wallet_charge)))
            }
        }
        tracing::warn!("Fell through charge logic");

        let wallet_charge = self.clone().register_failed_wallet_charge_no_response_body(&registered_transaction, &card, &wallet_reserve).await?;
        tracing::warn!("Registered unsuccessful inner charge in ledger for transaction={} id={}", &registered_transaction.transaction_id, wallet_charge.id);
        Ok((ChargeCardAttemptResult::Denied, Some(wallet_charge)))
    }

    pub async fn register_transaction_and_pending_passthrough_card_charge(
        self: Arc<Self>,
        user: &User,
        metadata: &TransactionMetadata,
        passthrough_card: &PassthroughCard,
    ) -> Result<RegisteredTransactionModel, ChargeError> {
        let ledger_service = self.ledger_service.clone();
        let dao = self.dao.clone();
        let metadata = metadata.clone();
        let passthrough_card = passthrough_card.clone();
        let user = user.clone();
        transactional(move |conn| {
            Box::pin(async move {
                let rtx = dao.clone().insert_registered_transaction(
                    conn,
                    &InsertableRegisteredTransaction {
                        user_id: user.id,
                        memo: &metadata.memo,
                        amount_cents: metadata.amount_cents,
                        mcc: &metadata.mcc,
                    }
                ).await?.into();

                let reserve = ledger_service.clone().reserve_passthrough_card_amount(
                    conn, &rtx, &passthrough_card.clone(), rtx.amount_cents
                ).await.map_err(|e| DataError::Unexpected(e.into()))?;
                Ok(rtx)
            })
        }).await.map_err(|e: DataError| ChargeError::Unexpected(e.into()))
    }


    pub async fn register_successful_passthrough_card_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        wallet_card_charge: &WalletCardCharge,
        passthrough_card: &PassthroughCard,
    ) -> Result<(), ChargeError> {

        let ledger_service = self.ledger_service.clone();
        let dao = self.dao.clone();
        let registered_transaction = registered_transaction.clone();
        let wallet_card_charge = wallet_card_charge.clone();
        let passthrough_card = passthrough_card.clone();
        transactional(move |conn| {
            Box::pin(async move {
                let outer_success = dao.clone().insert_passthrough_card_charge(
                    conn,
                    &InsertablePassthroughCardCharge {
                        registered_transaction_id: registered_transaction.id,
                        user_id: registered_transaction.user_id,
                        passthrough_card_id: passthrough_card.id,
                        amount_cents: registered_transaction.amount_cents,
                        status: ChargeStatus::Success,
                        is_success: Some(true),
                    }
                ).await?; // we don't want these to unwrap and shit the ledger call?

                let full_txn = dao.clone().insert_successful_end_to_end_charge(
                    conn,
                    &InsertableSuccessfulEndToEndCharge {
                        registered_transaction_id: registered_transaction.id,
                        wallet_card_charge_id: wallet_card_charge.id,
                        passthrough_card_charge_id: outer_success.id,
                    }
                ).await?; // we don't want these to unwrap and shit the ledger call?;

                let ledger_entry = ledger_service.clone().settle_passthrough_card_amount(
                    conn,
                    &registered_transaction.clone().into(),
                    &passthrough_card,
                    registered_transaction.amount_cents
                ).await.map_err(|e| DataError::Unexpected(e.into()))?;

                Ok(())
            })
        }).await.map_err(|e: DataError| ChargeError::Unexpected(e.into()))
    }

    pub async fn register_failed_passthrough_card_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        passthrough_card: &PassthroughCard,
    ) -> Result<(), ChargeError> {
        let ledger_service = self.ledger_service.clone();
        let dao = self.dao.clone();
        let registered_transaction = registered_transaction.clone();
        let passthrough_card = passthrough_card.clone();
        transactional( move |conn| {
            Box::pin(async move {
                let outer_fail = dao.clone().insert_passthrough_card_charge(
                    conn,
                    &InsertablePassthroughCardCharge {
                        registered_transaction_id: registered_transaction.id,
                        user_id: registered_transaction.user_id,
                        passthrough_card_id: passthrough_card.id,
                        amount_cents: registered_transaction.amount_cents,
                        status: ChargeStatus::Fail,
                        is_success: None
                    }
                ).await?;
                let ledger_entry = ledger_service.clone().release_passthrough_card_amount(
                    conn,
                    &registered_transaction.clone().into(),
                    &passthrough_card.clone(),
                    registered_transaction.amount_cents
                ).await.map_err(|e| DataError::Unexpected(e.into()))?;
                Ok(())
            })
        }).await.map_err(|e: DataError| ChargeError::Unexpected(e.into()))
    }


    pub async fn register_successful_wallet_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        wallet: &Wallet,
        expected_wallet_charge_reference: &ExpectedWalletChargeReference,
        payment_response: &PaymentResponse
    ) -> Result<WalletCardCharge, ChargeError> {
        let ledger_service = self.ledger_service.clone();
        let dao = self.dao.clone();
        let wallet = wallet.clone();
        let registered_transaction = registered_transaction.clone();
        let expected_wallet_charge_reference = expected_wallet_charge_reference.clone();
        let payment_response = payment_response.clone();
        transactional(move |conn| {
            Box::pin(async move {
                let wallet_charge = dao.clone().insert_wallet_charge(
                    conn,
                    &InsertableWalletCardCharge {
                        registered_transaction_id: registered_transaction.id,
                        user_id: registered_transaction.user_id,
                        wallet_card_id: wallet.id,
                        amount_cents: registered_transaction.amount_cents,
                        rule_id: wallet.rule_id,
                        expected_wallet_charge_reference_id: expected_wallet_charge_reference.id,
                        resolved_charge_status: ChargeStatus::Success,
                        psp_reference: payment_response.psp_reference.clone(),
                        returned_reference: payment_response.merchant_reference.clone(),
                        returned_charge_status: match payment_response.result_code {
                            Some(code) => serde_json::to_string(&code).ok(),
                            None => None,
                        },
                        is_success: Some(true),
                    }
                ).await?;

                let ledger_entry = ledger_service.clone().settle_wallet_card_amount(
                    conn,
                    &registered_transaction.clone().into(),
                    wallet.id,
                    registered_transaction.amount_cents
                ).await.map_err(|e| DataError::Unexpected(e.into()))?;

                Ok(wallet_charge)
            })
        }).await.map_err(|e: DataError| ChargeError::Unexpected(e.into()))
    }

    pub async fn register_failed_wallet_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        wallet: &Wallet,
        expected_wallet_charge_reference: &ExpectedWalletChargeReference,
        payment_response: &PaymentResponse
    ) -> Result<WalletCardCharge, ChargeError> {
        let ledger_service = self.ledger_service.clone();
        let dao = self.dao.clone();
        let wallet = wallet.clone();
        let registered_transaction = registered_transaction.clone();
        let expected_wallet_charge_reference = expected_wallet_charge_reference.clone();
        let payment_response = payment_response.clone();
        transactional(move |conn| {
            Box::pin(async move {
                let wallet_charge = dao.clone().insert_wallet_charge(
                    conn,
                    &InsertableWalletCardCharge {
                        registered_transaction_id: registered_transaction.id,
                        user_id: registered_transaction.user_id,
                        wallet_card_id: wallet.id,
                        amount_cents: registered_transaction.amount_cents,
                        rule_id: wallet.rule_id,
                        expected_wallet_charge_reference_id: expected_wallet_charge_reference.id,
                        resolved_charge_status: ChargeStatus::Fail,
                        psp_reference: payment_response.psp_reference.clone(),
                        returned_reference: payment_response.merchant_reference.clone(),
                        returned_charge_status: match payment_response.result_code {
                            Some(code) => serde_json::to_string(&code).ok(),
                            None => None,
                        },
                        is_success: None,
                    }
                ).await?;

                let ledger_entry = ledger_service.clone().release_wallet_amount(
                    conn,
                    &(registered_transaction.clone().into()),
                    wallet.id,
                    registered_transaction.amount_cents
                ).await.map_err(|e| DataError::Unexpected(e.into()))?;

                Ok(wallet_charge)
            })
        }).await.map_err(|e: DataError| ChargeError::Unexpected(e.into()))
    }

    pub async fn register_partial_wallet_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        wallet: &Wallet,
        expected_wallet_charge_reference: &ExpectedWalletChargeReference,
        payment_response: &PaymentResponse
    ) -> Result<WalletCardCharge, ChargeError> {
        let ledger_service = self.ledger_service.clone();
        let dao = self.dao.clone();
        let registered_transaction = registered_transaction.clone();
        let wallet = wallet.clone();
        let expected_wallet_charge_reference = expected_wallet_charge_reference.clone();
        let payment_response = payment_response.clone();
        transactional(move |conn| {
            Box::pin(async move {
                let wallet_charge = dao.clone().insert_wallet_charge(
                    conn,
                    &InsertableWalletCardCharge {
                        registered_transaction_id: registered_transaction.id,
                        user_id: registered_transaction.user_id,
                        wallet_card_id: wallet.id,
                        amount_cents: registered_transaction.amount_cents,
                        rule_id: wallet.rule_id,
                        expected_wallet_charge_reference_id: expected_wallet_charge_reference.id,
                        resolved_charge_status: ChargeStatus::Fail,
                        psp_reference: payment_response.psp_reference.clone(),
                        returned_reference: payment_response.merchant_reference.clone(),
                        returned_charge_status: match &payment_response.result_code {
                            Some(code) => serde_json::to_string(code).ok(),
                            None => None,
                        },
                        is_success: None,
                    }
                ).await?;

                let ledger_entry = ledger_service.clone().settle_wallet_card_amount(
                    conn,
                    &registered_transaction.clone().into(),
                    wallet.id,
                    match payment_response.amount {
                        None => 0,
                        Some(amount) => amount.value as i32 //TODO: rounding / truncation
                    }
                ).await.map_err(|e| DataError::Unexpected(e.into()))?;

                Ok(wallet_charge)
            })
        }).await.map_err(|e: DataError| ChargeError::Unexpected(e.into()))
    }

    pub async fn register_failed_wallet_charge_no_response_body(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        wallet: &Wallet,
        expected_wallet_charge_reference: &ExpectedWalletChargeReference,
    ) -> Result<WalletCardCharge, ChargeError> {
        let ledger_service = self.ledger_service.clone();
        let dao = self.dao.clone();
        let registered_transaction = registered_transaction.clone();
        let wallet = wallet.clone();
        let expected_wallet_charge_reference = expected_wallet_charge_reference.clone();
        transactional(move |conn| {
            Box::pin(async move {
                let wallet_charge = dao.clone().insert_wallet_charge(
                    conn,
                    &InsertableWalletCardCharge {
                        registered_transaction_id: registered_transaction.id,
                        user_id: registered_transaction.user_id,
                        wallet_card_id: wallet.id,
                        amount_cents: registered_transaction.amount_cents,
                        rule_id: wallet.rule_id,
                        expected_wallet_charge_reference_id: expected_wallet_charge_reference.id,
                        resolved_charge_status: ChargeStatus::Fail,
                        psp_reference: None,
                        returned_reference: None,
                        returned_charge_status: None,
                        is_success: None,
                    }
                ).await?;

                let ledger_entry = ledger_service.clone().release_wallet_amount(
                    conn,
                    &registered_transaction.clone().into(),
                    wallet.id,
                    registered_transaction.amount_cents
                ).await.map_err(|e| DataError::Unexpected(e.into()))?;

                Ok(wallet_charge)
            })
        }).await.map_err(|e: DataError| ChargeError::Unexpected(e.into()))
    }

    pub async fn register_reserve_wallet_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        wallet: &Wallet,
    ) -> Result<ExpectedWalletChargeReference, ChargeError> {
        let ledger_service = self.ledger_service.clone();
        let dao = self.dao.clone();
        let registered_transaction = registered_transaction.clone();
        let wallet = wallet.clone();
        transactional(move |conn| {
            Box::pin(async move {
                let wallet_success = dao.clone().insert_expected_wallet_charge_reference(
                    conn,
                    &InsertableExpectedWalletChargeReference {
                        registered_transaction_id: registered_transaction.id,
                        user_id: wallet.user_id,
                        wallet_card_id: wallet.id,
                        amount_cents: registered_transaction.amount_cents,
                    }
                ).await?;

                let rt = registered_transaction.clone();
                let ledger_entry = ledger_service.clone().reserve_wallet_amount(
                    conn,
                    &rt.into(),
                    wallet.id,
                    registered_transaction.amount_cents
                ).await.map_err(|e| DataError::Unexpected(e.into()))?;

                Ok(wallet_success)
            })
        }).await.map_err(|e: DataError| ChargeError::Unexpected(e.into()))
    }
}