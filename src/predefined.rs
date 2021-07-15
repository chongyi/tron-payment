use ethabi::{Function, Param, ParamType, StateMutability, Token};
use crate::key::Address;

pub fn tether_transfer(address: &Address, amount: i64) -> Vec<u8> {
    let mut buf = [0; 20];
    buf.copy_from_slice(&address[1..]);
    Function {
        name: "transfer".into(),
        inputs: vec![
            Param {
                name: "_to".into(),
                kind: ParamType::Address,
            },
            Param {
                name: "_value".into(),
                kind: ParamType::Uint(256),
            }
        ],
        outputs: vec![
            Param {
                name: String::new(),
                kind: ParamType::Bool
            }
        ],
        state_mutability: StateMutability::NonPayable,
        constant: false
    }.encode_input(&[
        Token::Address(buf.into()),
        Token::Uint(amount.into())
    ]).unwrap()
}