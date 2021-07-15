#[macro_use]
extern crate log;

use tron_payment::apis::wallet_client::WalletClient;
use tonic::Request;
use tron_payment::key::{PrivateKey, Address};
use tron_payment::apis::TransferContract;
use tron_payment::utils::to_raw_address;
use prost::Message;
use sha2::Digest;
use k256::ecdsa::signature::Signer;
use k256::ecdsa::Signature;
use k256::ecdsa::recoverable;
use tron_payment::services::{Service, Transfer};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("load private key");
    let key = PrivateKey::from_bytes(&hex::decode("5bce54327f175cb45c6643e2d1564aeace89471f2e7de780cbaa336da0024ae1")?)?;

    let mut service = Service::new().await?;
    let mut agent = service.agent(key);
    let res = agent.transfer(&Address::from_base58("TUCecrsyFh9kzWLqTkDNNXL9zbdaMhpyyc")?, 300000).await?;
    info!("transfer result: {}", res.unwrap().result);

    Ok(())
}

