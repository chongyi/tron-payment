use tron_payment::address::*;

fn main() {
    let pk = PrivateKey::generate();
    println!("pk: {}", pk.private_key_string());
    println!("address: {}", pk.address_string());

    qr2term::print_qr(pk.address_string());
}
