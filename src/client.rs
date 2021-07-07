use apis::wallet_client::WalletClient;
use apis::{EmptyMessage, NumberMessage};

pub mod apis {
    tonic::include_proto!("protocol");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WalletClient::connect("http://34.253.187.192:50051").await?;

    let request = tonic::Request::new(NumberMessage {
        num: 31679800
    });

    let response = client.get_block_by_num2(request).await?;


    // println!("RESPONSE={:?}", response);
    println!("len: {}", response.get_ref().transactions.len());
    println!("{}", hex::encode(&response.get_ref().blockid));

    assert_eq!("0000000001e36538b918f1643d20d9bb741f08839a026349458a7b80ca8672e0", hex::encode(&response.get_ref().blockid));
    Ok(())
}
