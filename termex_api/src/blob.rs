//! Blob object

use key::Decryptor;
use key::Encryptor;
use key::Key;
use std::slice::Iter;

pub enum MsgError {
    ConversionError,
    DecodeError(String),
    EncodeError(String),
}

pub trait Totext<D: Decryptor> {
    fn decrypt(&self, &D) -> Result<String, MsgError>;
}

pub trait Toblob<E: Encryptor> {
    fn encrypt(&self, &E) -> Result<String, MsgError>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub blob_type: String,
    pub blob_text: String,
}

impl Data {
    pub fn new_history(text: String) -> Self {
        Data {
            blob_type: "history".to_owned(),
            blob_text: text,
        }
    }
}

impl<D: Decryptor> Totext<D> for Data {
    fn decrypt(&self, d: &D) -> Result<String, MsgError> {
        if self.blob_text.is_empty() {
            return Ok(String::new());
        }
        let bytes: Vec<u8> = d
            .decrypt(self.blob_text.clone().as_bytes().to_vec())
            .map_err(|_e| MsgError::DecodeError(self.blob_text.clone()))?;
        return String::from_utf8(bytes).map_err(|_e| MsgError::ConversionError);
    }
}

impl<E: Encryptor> Toblob<E> for Data {
    fn encrypt(&self, enc: &E) -> Result<String, MsgError> {
        let bytes = enc
            .encrypt(self.blob_text.clone().as_bytes().to_vec())
            .map_err(|_e| MsgError::EncodeError(self.blob_text.clone()))?;
        return String::from_utf8(bytes).map_err(|_e| MsgError::ConversionError);
    }
}
#[derive(Debug)]
pub struct Blobs<T: Toblob<Key> + Totext<Key>>(pub Vec<T>);

impl<'a, T> Blobs<T>
where
    T: Toblob<Key> + Totext<Key>,
{
    pub fn iter(&'a self, key: &'a Key) -> BlobIterator<T> {
        return BlobIterator {
            key: key,
            pos: 0,
            blob: self,
        };
    }
}

pub struct BlobIterator<'a, T: 'a + Toblob<Key> + Totext<Key>> {
    key: &'a Key,
    blob: &'a Blobs<T>,
    pos: usize,
}

impl<'a, T: 'a + Toblob<Key> + Totext<Key>> Iterator for BlobIterator<'a, T> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.pos >= self.blob.0.len() {
            None
        } else {
            self.pos += 1;
            match self.blob.0.get(self.pos - 1) {
                None => None,
                Some(val) => match val.decrypt(self.key) {
                    Ok(s) => Some(s),
                    Err(_) => None,
                },
            }
        }
    }
}
