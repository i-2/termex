//! Key decryption login behind termext
//! Text  |>  encryptor |> blob
use base64::{decode, encode};
use openssl::error::ErrorStack;
use openssl::pkey::Private;
use openssl::rsa::Padding;
use openssl::rsa::Rsa;

pub trait Decryptor {
    fn decrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, ErrorStack>;
}

pub trait Encryptor {
    fn encrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, ErrorStack>;
}

pub struct Key {
    key: Rsa<Private>,
}

impl Decryptor for Key {
    fn decrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, ErrorStack> {
        let mut rounded = vec![0u8; msg.len() as usize];
        let decoded = match decode(&msg) {
            Ok(res) => res,
            Err(_e) => return Err(ErrorStack::get()),
        };
        let decryption =
            self.key
                .private_decrypt(&decoded, &mut rounded, Padding::PKCS1);
        let glen = match decryption {
            Ok(dec) => dec,
            Err(_e) => return Err(ErrorStack::get()),
        };
        Ok(rounded[0..glen].to_owned())
    }
}

impl Encryptor for Key {
    fn encrypt(&self, msg: Vec<u8>) -> Result<Vec<u8>, ErrorStack> {
        let mut rounded = vec![0u8; self.key.size() as usize];
        let encryption =
            self.key.public_encrypt(&msg, &mut rounded, Padding::PKCS1);
        let glen = match encryption {
            Ok(dec) => dec,
            Err(_e) => return Err(ErrorStack::get()),
        };
        let encoded_bytes = encode(&rounded[0..glen]);
        Ok(encoded_bytes.as_bytes().to_owned())
    }
}

impl Key {
    pub fn new(key_bytes: Rsa<Private>) -> Self {
        Key { key: key_bytes }
    }

    pub fn as_string(&self) -> Result<Vec<u8>, ErrorStack> {
        self.key.private_key_to_pem()
    }

    pub fn generate(num: u32) -> Result<Self, ErrorStack> {
        match Rsa::generate(num) {
            Ok(key) => Ok(Key::new(key)),
            Err(_e) => Err(_e),
        }
    }

    pub fn to_pem_string(&self) -> Result<String, ErrorStack> {
        self.key.private_key_to_pem().and_then(|res| {
            String::from_utf8(res).map_err(|e| ErrorStack::get())
        })
    }

    pub fn from_pem_string(bytes: Vec<u8>) -> Result<Self, ErrorStack> {
        Rsa::private_key_from_pem(&bytes)
            .and_then(|key_bytes| Ok(Key::new(key_bytes)))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use openssl::pkey::*;
    use openssl::rsa::*;

    #[test]
    fn test_message_encryption() {
        // test the message encryption
        let hello = "hello world";
        let rsa: Rsa<Private> = Rsa::generate(256).unwrap();
        let keyed: Key = Key::new(rsa);
        let veced = keyed.encrypt(hello.as_bytes().to_owned()).unwrap();
        assert_eq!(veced.len(), 44);
    }

    #[test]
    fn test_message_decryption() {
        // test message decryption.
        let hello = "hello world";
        let rsa: Rsa<Private> = Rsa::generate(256).unwrap();
        let keyed: Key = Key::new(rsa);
        let string: Vec<u8> = keyed.encrypt(hello.as_bytes().to_vec()).unwrap();
        let decrypted = keyed.decrypt(string).unwrap();
        assert_eq!(decrypted, hello.as_bytes().to_vec());
    }

    #[test]
    fn test_print_pem_string() {
        let key: Key = Key::generate(256).unwrap();
        println!("{:?}", key.to_pem_string())
    }
}
