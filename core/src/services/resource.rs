use crate::apis::{FreezeBalanceContract, ResourceCode, Return};
use crate::Result;
use crate::services::{ServiceAgent, Transfer};

#[async_trait]
pub trait Resource {
    async fn freeze_balance(&mut self, balance: i64, duration: i64, resource: ResourceCode) -> Result<Return>;
}

#[async_trait]
impl<'s> Resource for ServiceAgent<'s> {
    async fn freeze_balance(&mut self, balance: i64, duration: i64, resource: ResourceCode) -> Result<Return> {
        let trx_ext = self.client
            .freeze_balance2(FreezeBalanceContract {
                owner_address: self.key.address().into(),
                frozen_balance: balance,
                frozen_duration: duration,
                resource: resource.into(),
                receiver_address: vec![],
            })
            .await?
            .into_inner();

        Ok(
            self.sign_and_broadcast(trx_ext).await?
        )
    }
} 