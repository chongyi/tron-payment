use crate::key::Address;
use crate::Result;
use crate::services::{Service, ServiceAgent};
use tonic::Request;
use crate::apis::{TransferContract, Return};
use prost::Message;

#[async_trait]
pub trait Transfer {
    async fn transfer(&mut self, to: &Address, amount: i64) -> Result<Option<Return>>;
}

#[async_trait]
impl<'s> Transfer for ServiceAgent<'s> {
    async fn transfer(&mut self, to: &Address, amount: i64) -> Result<Option<Return>> {
        let mut trx_ext = self.client
            .create_transaction2(Request::new(TransferContract {
                owner_address: self.key.address().into(),
                to_address: to.into(),
                amount,
            }))
            .await?
            .into_inner();

        debug!("created transaction, id = {}", hex::encode(&trx_ext.txid));

        let transaction = trx_ext.transaction
            .map(|mut transaction| {
                if let Some(raw_data) = transaction.raw_data.as_ref() {
                    let mut buf = Vec::with_capacity(raw_data.encoded_len());
                    raw_data.encode(&mut buf).unwrap();

                    transaction.signature = vec![Vec::from(self.key.sign(&buf).as_ref())];
                }

                transaction
            });

        match transaction {
            Some(transaction) => {
                let res = self.client
                    .broadcast_transaction(Request::new(transaction))
                    .await?
                    .into_inner();

                Ok(Some(res))
            },
            None => Ok(None),
        }
    }


}