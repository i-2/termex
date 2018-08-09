
use std::collections::HashMap;
use reqwest::{Client, Error, StatusCode};
use reqwest::header::Authorization;
use key::{Key, Decryptor, Encryptor};
use blob::{Data, Blobs};

const TERMEX_URL : &'static str= "http://termex.herokuapp.com";

pub type TermexResult<T> = Result<T, TermexClientError>;

pub enum TermexClientError {
    AuthFailed,
    WithMessage(String)
}

impl ToString for TermexClientError {
    fn to_string(&self) -> String {
        match self {
            &TermexClientError::AuthFailed => "Auth Failed".to_owned(),
            &TermexClientError::WithMessage(ref e) => e.clone()
        }
    }
}


impl From<Error> for TermexClientError {
    fn from(_err: Error) -> Self {
        TermexClientError::AuthFailed
    }
}

#[derive(Deserialize)]
pub struct TermexClient {
    token: String
}

impl TermexClient {

    pub fn login(username: String, password: String) -> TermexResult<Self> {
        let login_url: String = format!("{}/{}", TERMEX_URL, "login");
        let client = Client::new();
        let mut requestmap = HashMap::new();
        requestmap.insert("username", username);
        requestmap.insert("password", password);
        let mut res = client.post(&login_url[..]).json(&requestmap).send()?;
        let selfed : TermexClient = res.json()?;
        Ok(selfed)
    }

    pub fn signup(username: String, password: String) -> TermexResult<()> {
        let signup_url : String = format!("{}/{}", TERMEX_URL, "register");
        let client = Client::new();
        let mut requestmap = HashMap::new();
        requestmap.insert("username", username);
        requestmap.insert("password", password);
        let mut res = client.post(&signup_url[..]).json(&requestmap).send()?;
        match res.status() {
            StatusCode::Ok => Ok(()),
            StatusCode::Unauthorized => Err(TermexClientError::WithMessage("Already exist".to_owned())),
            _ => Err(TermexClientError::WithMessage("Signup failed".to_owned()))
        }
    }

    pub fn blobs(&self) -> TermexResult<Blobs<Data>> {
        let blob_url: String = format!("{}/{}", TERMEX_URL, "blobs");
        let client = Client::new();
        let mut res = client.get(&blob_url[..]).header(Authorization(self.token.clone())).send()?;
        let datas: Vec<Data> = res.json()?;
        Ok(Blobs(datas))
    }


}