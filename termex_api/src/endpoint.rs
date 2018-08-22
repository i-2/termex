
use std::collections::HashMap;
use reqwest::{Client, Error, StatusCode};
use reqwest::header::Authorization;
use key::{Key, Decryptor, Encryptor};
use blob::{Data, Blobs};

const TERMEX_URL : &'static str= "http://termex.herokuapp.com";

pub type TermexResult<T> = Result<T, TermexClientError>;

#[derive(Debug)]
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
    pub token: String
}

impl TermexClient {

    pub fn new(token: String) -> Self {
        TermexClient {
            token
        }
    }

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
            StatusCode::Ok | StatusCode::Created => Ok(()),
            StatusCode::Unauthorized => Err(TermexClientError::WithMessage("Already exist".to_owned())),
            k => {
                println!("{:?}", k);
                Err(TermexClientError::WithMessage("Signup failed".to_owned()))
            }
        }
    }

    pub fn blobs(&self, page: u32, limit: u32) -> TermexResult<Blobs<Data>> {
        let blob_url: String = format!("{}/{}", TERMEX_URL, "blobs");
        let client = Client::new();
        let mut req_map = HashMap::new();
        req_map.insert("page", page);
        req_map.insert("limit", limit);
        let mut res = client.get(&blob_url[..])
                            .query(&req_map)
                            .header(Authorization(self.token.clone()))
                            .send()?;
        let datas: Vec<Data> = res.json()?;
        Ok(Blobs(datas))
    }

    pub fn new_blob(&self, text: String) -> TermexResult<()> {
        let blob_url : String = format!("{}/{}", TERMEX_URL, "blobs");
        let client = Client::new();
        let mut request_map = HashMap::new();
        request_map.insert("blob_type", "history");
        request_map.insert("blob_text", text.as_str());
        let mut res = client.post(&blob_url)
        .json(&request_map)
        .header(Authorization(self.token.clone()))
        .send()?;
        Ok(())
    }

}