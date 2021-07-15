use tonic::transport::Channel;

pub use transfer::Transfer;
pub use resource::Resource;

use crate::apis::wallet_client::WalletClient;
use crate::key::PrivateKey;
use crate::Result;

mod transfer;
mod resource;

pub const DEFAULT_ENDPOINT: &'static str = "http://34.253.187.192:50051";

pub struct ServiceAgent<'s> {
    key: PrivateKey,
    client: &'s mut WalletClient<Channel>,
}

pub struct Service {
    client: WalletClient<Channel>,
}

impl Service {
    pub async fn new() -> Result<Self> {
        info!("Connect to block chain endpoint: {}", DEFAULT_ENDPOINT);

        Ok(Self {
            client: WalletClient::connect(DEFAULT_ENDPOINT).await?,
        })
    }

    pub fn agent(&mut self, key: PrivateKey) -> ServiceAgent {
        ServiceAgent {
            client: &mut self.client,
            key
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceConfig {
    /// 区块链服务节点
    pub endpoint: String,
}