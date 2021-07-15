use crate::key::Address;
use crate::Result;
use crate::services::{Service, ServiceAgent};
use crate::apis::{TransferContract, Return, TriggerSmartContract, Transaction};
use prost::Message;
use crate::error::Error;

#[async_trait]
pub trait Transfer {
    async fn transfer(&mut self, to: &Address, amount: i64) -> Result<Return>;
    async fn contract_transfer(&mut self, owner: &Address, contract: &Address, data: Vec<u8>) -> Result<Return>;
    async fn sign_and_broadcast(&mut self, mut transaction: Transaction) -> Result<Return>;
}

#[async_trait]
impl<'s> Transfer for ServiceAgent<'s> {
    async fn transfer(&mut self, to: &Address, amount: i64) -> Result<Return> {
        let mut trx_ext = self.client
            .create_transaction2(TransferContract {
                owner_address: self.key.address().into(),
                to_address: to.into(),
                amount,
            })
            .await?
            .into_inner();

        debug!("created transaction, id = {}", hex::encode(&trx_ext.txid));

        let transaction = trx_ext.transaction.ok_or(Error::EmptyTransaction)?;

        self.sign_and_broadcast(transaction).await
    }

    async fn contract_transfer(&mut self, owner: &Address, contract: &Address, data: Vec<u8>) -> Result<Return> {
        let trx_ext = self.client
            .trigger_contract(TriggerSmartContract {
                owner_address: owner.into(),
                contract_address: contract.into(),
                call_value: 0,
                data,
                call_token_value: 0,
                token_id: 0
            })
            .await?
            .into_inner();

        let transaction = trx_ext.transaction.ok_or(Error::EmptyTransaction)?;

        self.sign_and_broadcast(transaction).await
    }

    async fn sign_and_broadcast(&mut self, mut transaction: Transaction) -> Result<Return> {
        if let Some(raw_data) = transaction.raw_data.as_ref() {
            let mut buf = Vec::with_capacity(raw_data.encoded_len());
            raw_data.encode(&mut buf).unwrap();

            transaction.signature = vec![Vec::from(self.key.sign(&buf).as_ref())];
        }

        Ok(
            self.client
                .broadcast_transaction(transaction)
                .await?
                .into_inner()
        )
    }
}

fn sign_transaction() {

}