use k256::ecdsa::{SigningKey, Error};
use rand::rngs::OsRng;
use std::ops::Deref;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use sha3::{Keccak256, Digest};
use crate::utils::raw_address_base_check;
use k256::ecdsa::recoverable;
use k256::ecdsa::signature::DigestSigner;

pub const PAD: u8 = 0x41;

#[derive(Debug, Eq, PartialEq)]
pub struct PrivateKey {
    key: SigningKey,
    address: Address,
}

impl PrivateKey {
    pub fn new(key: SigningKey) -> Self {
        let verify_key = key.verifying_key()
            .to_encoded_point(true)
            .to_untagged_bytes()
            .unwrap();

        let mut buf = [0; 21];
        let hash = Keccak256::digest(&verify_key);

        // 取H的最后20字节，在前面填充一个字节0x41得到address
        buf[0] = PAD; // 填充第一个字节为 0x41
        &buf[1..].copy_from_slice(&hash[12..32]);

        let address = Address::from(buf);

        PrivateKey {
            key,
            address
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(PrivateKey::new(SigningKey::from_bytes(bytes)?))
    }

    pub fn generate() -> Self {
        PrivateKey::new(SigningKey::random(OsRng::default()))
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn key(&self) -> &SigningKey {
        &self.key
    }

    pub fn address_string(&self) -> String {
        self.address.to_base58()
    }

    pub fn private_key_string(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// 对数据进行签名
    ///
    /// 常用于交易数据的签名，该签名符合以太坊风格的“可恢复签名”。
    pub fn sign<T: AsRef<[u8]>>(&self, data: &T) -> recoverable::Signature {
        // Signature
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);

        self.sign_digest(hasher)
    }
}

impl Deref for PrivateKey {
    type Target = SigningKey;

    fn deref(&self) -> &Self::Target {
        &self.key
    }
}

pub type RawAddress = [u8; 21];
pub type BaseCheck = [u8; 4];
pub type CheckedAddress = [u8; 25];

#[derive(Debug, Eq, PartialEq)]
pub struct Address {
    raw_address: RawAddress,
    base_check: BaseCheck,
}

impl Deref for Address {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.raw_address
    }
}

impl Into<Vec<u8>> for Address {
    fn into(self) -> Vec<u8> {
        self.raw_address.to_vec()
    }
}

impl Into<Vec<u8>> for &Address {
    fn into(self) -> Vec<u8> {
        self.raw_address.to_vec()
    }
}

impl From<[u8; 21]> for Address {
    fn from(raw_address: [u8; 21]) -> Self {
        let base_check = raw_address_base_check(&raw_address);
        Address {
            raw_address,
            base_check
        }
    }
}

impl From<[u8; 25]> for Address {
    fn from(checked_address: [u8; 25]) -> Self {
        let mut raw_address = [0; 21];
        let mut base_check = [0; 4];
        raw_address.copy_from_slice(&checked_address[0..21]);
        base_check.copy_from_slice(&checked_address[21..]);

        Address {
            raw_address,
            base_check
        }
    }
}

impl Address {
    pub fn from_base58(s: &str) -> Result<Self, bs58::decode::Error> {
        let buf = bs58::decode(s).into_vec()?;
        let mut checked_address = [0; 25];
        checked_address.copy_from_slice(&buf);

        Ok(Address::from(checked_address))
    }

    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        let mut raw_address = [0; 21];
        hex::decode_to_slice(s, &mut raw_address)?;

        Ok(Address::from(raw_address))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.raw_address)
    }

    pub fn to_base58(&self) -> String {
        let mut buf = [0; 25];
        &buf[0..21].copy_from_slice(&self.raw_address);
        &buf[21..].copy_from_slice(&self.base_check);
        bs58::encode(&buf).into_string()
    }

    pub fn raw_address(&self) -> &[u8] {
        &self.raw_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha3::{Keccak256, Digest};
    use k256::elliptic_curve::sec1::ToEncodedPoint;

    #[test]
    fn test_generate_private_key() {
        let private_key = PrivateKey::generate();
        let bytes = private_key.to_bytes();

        assert_eq!(PrivateKey::from_bytes(&bytes).unwrap(), private_key);
    }

    #[test]
    fn test_address_generate() {
        let mut buf = [0; 32];
        hex::decode_to_slice("81fb3e8cacea0567c6d76630f825cbcafc6c0e437642c469427118ad196b680e", &mut buf);
        let private_key = PrivateKey::from_bytes(&buf).unwrap();

        assert_eq!("TD19GP9scAsF5R8Y1TWXNBeSbhavMYjVyP", private_key.address_string());
    }
}