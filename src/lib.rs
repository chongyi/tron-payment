#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

pub mod key;
pub mod apis;
pub mod utils;
pub mod services;

pub use anyhow::Result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
