//! using vault to keyring.

use std::env::{var, VarError};
use openssl::rsa::Rsa;
use keyring::{Keyring, Result as KeyResult};
use SERVICE_NAME;

const TOKENSTORE: &'static str = "TERMEX_TOKEN";

pub struct Vault<'a>{
  inner: Keyring<'a>,
  token: Keyring<'a>
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

    pub fn set_token(&self, passphrase: String) -> KeyResult<()> {
        self.token.set_password(&passphrase)
    }

    pub fn get_token(&self) -> KeyResult<String> {
        self.token.get_password()
    }

    pub fn new(username: &'a str) -> Self{
        Vault {
            inner: Keyring::new(SERVICE_NAME, username),
            token: Keyring::new(TOKENSTORE, username)
        }
    }
}