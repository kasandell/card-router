use crate::api_error::ApiError;
use crate::credit_card_type::dao::{CreditCardDao, CreditCardDaoTrait};
use crate::user::entity::User;
use crate::wallet::constant::WalletCardAttemptStatus;
use crate::wallet::dao::{WalletCardAttemptDao, WalletCardAttemtDaoTrait, WalletDao, WalletDaoTrait};
use crate::wallet::entity::{InsertableCardAttempt, NewCard, UpdateCardAttempt, Wallet, WalletCardAttempt};
use crate::wallet::request::{MatchAttemptRequest, RegisterAttemptRequest};

pub struct Engine {
    pub credit_card_dao: Box<dyn CreditCardDaoTrait>,
    pub wallet_card_attempt_dao: Box<dyn WalletCardAttemtDaoTrait>,
    pub wallet_dao: Box<dyn WalletDaoTrait>,
}


impl Engine {
    pub fn new() -> Self {
        Self {
            credit_card_dao: Box::new(CreditCardDao::new()),
            wallet_card_attempt_dao: Box::new(WalletCardAttemptDao::new()),
            wallet_dao: Box::new(WalletDao::new())
        }
    }

    #[cfg(test)]
    pub fn new_with_services(
        credit_card_dao: Box<dyn CreditCardDaoTrait>,
        wallet_card_attempt_dao: Box<dyn WalletCardAttemtDaoTrait>,
        wallet_dao: Box<dyn WalletDaoTrait>,
    ) -> Self {
        Self {
            credit_card_dao,
            wallet_card_attempt_dao,
            wallet_dao
        }
    }

    pub async fn attempt_register_new_attempt(
        &self,
        user: &User,
        request: &RegisterAttemptRequest
    ) -> Result<WalletCardAttempt, ApiError> {
        let credit_card = self.credit_card_dao.find_by_public_id(request.credit_card_type_public_id.clone())?;
        let wca = self.wallet_card_attempt_dao.insert(
            InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: credit_card.id,
                expected_reference_id: request.expected_reference_id.clone()
            }
        )?;
        Ok(wca)
    }

    pub fn attempt_match(
        &self,
        request: &MatchAttemptRequest
    ) -> Result<Wallet, ApiError> {
        let card_attempt = self.wallet_card_attempt_dao.find_by_reference_id(
            request.merchant_reference_id.clone()
        )?;
        info!("Found wallet card attempt id {}", card_attempt.id);

        let update = self.wallet_card_attempt_dao.update_card(card_attempt.id, UpdateCardAttempt {
            recurring_detail_reference: request.psp_reference.clone(),
            psp_id: request.original_psp_reference.clone(),
            status: WalletCardAttemptStatus::MATCHED.as_str()
        })?;
        info!("Updated to matched: {}", &update.status);

        let created_card = self.wallet_dao.insert_card(
            NewCard {
                user_id: card_attempt.user_id,
                payment_method_id: request.psp_reference.clone(),
                credit_card_id: card_attempt.credit_card_id,
                wallet_card_attempt_id: card_attempt.id,
            }
        )?;
        info!("Created card: {}", &created_card.public_id);

        Ok(created_card)
    }
}