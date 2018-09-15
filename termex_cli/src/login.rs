//! Login system
//!
use termex_api::endpoint::{TermexClient, TermexClientError};

pub fn login(
    username: String,
    password: String,
) -> Result<String, TermexClientError> {
    TermexClient::login(username, password).and_then(|t| Ok(t.token))
}
