//! using vault to keyring.

use std::env::{var, VarError};
use openssl::rsa::Rsa;
use keyring::{Keyring, Result as KeyResult};
use SERVICE_NAME;


pub struct Vault<'a>{
  inner: Keyring<'a>
}

impl<'a> Vault<'a> {
    pub fn exists(&self) -> bool {
        self.inner.get_password().is_ok()
    }

    pub fn get(&self) -> KeyResult<String> {
        self.inner.get_password()
    }

    pub fn set(&self, passphrase: String) -> KeyResult<()> {
        self.inner.set_password(&passphrase)
    }

    pub fn new(value: &'a str) -> Self{
        Vault {
            inner: Keyring::new(SERVICE_NAME, value)
        }
    }
}