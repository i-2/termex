#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate keyring;
extern crate reqwest;
extern crate openssl;
extern crate base64;

pub mod endpoint;
pub mod blob;
pub mod key;
pub mod vault;

const SERVICE_NAME: & 'static str = "termex";



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}
