use sha2::Digest;

pub fn to_raw_address<T: AsRef<[u8]>>(checked_address: &T) -> &[u8] {
    &checked_address.as_ref()[0..21]
}

pub fn raw_address_base_check(raw_address: &[u8]) -> [u8; 4] {
    let mut buf = [0; 4];
    let h1 = sha2::Sha256::digest(raw_address);
    let h2 = sha2::Sha256::digest(&h1);

    buf.copy_from_slice(&h2[0..4]);
    buf
}