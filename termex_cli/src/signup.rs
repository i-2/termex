use termex_api::endpoint::{TermexClient, TermexResult};

pub fn signup(username: String, password: String) -> TermexResult<()> {
    TermexClient::signup(username, password)
}
