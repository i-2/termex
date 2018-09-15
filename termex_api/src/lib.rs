#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate base64;
extern crate keyring;
extern crate openssl;
extern crate reqwest;

pub mod blob;
pub mod endpoint;
pub mod key;
pub mod vault;

pub const SERVICE_NAME: &'static str = "termex";
