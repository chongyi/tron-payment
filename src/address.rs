use k256::ecdsa::{SigningKey, Error};
use rand::rngs::OsRng;
use std::ops::Deref;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use sha3::{Keccak256, Digest};

pub const PAD: u8 = 0x41;

#[derive(Debug, Eq, PartialEq)]
pub struct PrivateKey(SigningKey);

impl PrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(PrivateKey(SigningKey::from_bytes(bytes)?))
    }

    pub fn generate() -> Self {
        PrivateKey(SigningKey::random(OsRng::default()))
    }

    pub fn raw_address(&self) -> RawAddress {
        let verify_key = self.verifying_key().to_encoded_point(true).to_untagged_bytes().unwrap();
        let mut buf = [0; 21];
        let hash = Keccak256::digest(&verify_key);

        // 取H的最后20字节，在前面填充一个字节0x41得到address
        buf[0] = PAD; // 填充第一个字节为 0x41
        &buf[1..].copy_from_slice(&hash[12..32]);

        buf
    }

    pub fn checked_address(&self) -> CheckedAddress {
        let raw_address = self.raw_address();

        let h1 = sha2::Sha256::digest(&raw_address);
        let h2 = sha2::Sha256::digest(&h1);

        let mut buf = [0; 25];
        &buf[0..21].copy_from_slice(&raw_address);
        &buf[21..].copy_from_slice(&h2[0..4]);
        buf
    }

    pub fn address_string(&self) -> String {
        bs58::encode(&self.checked_address())
            .into_string()
    }

    pub fn private_key_string(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn sign(&self) {

    }
}

impl Deref for PrivateKey {
    type Target = SigningKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type RawAddress = [u8; 21];
pub type CheckedAddress = [u8; 25];

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