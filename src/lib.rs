use std::fmt::{Display, Formatter};

use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use error::AuthError;
pub use models::user::UserSchema as User;
#[allow(unused)]
pub use ErrorSchema as Error;

mod error;
mod get_user;
mod logout;
pub mod models;
mod refresh_token;
mod signin_with_password;
mod signup;
mod util;
mod delete_user;

#[derive(Clone, Debug)]
pub struct AuthClient {
    http_client: reqwest::Client,
    supabase_api_url: String,
    supabase_anon_key: String,
    #[allow(unused)]
    postgrest_client: Postgrest,
}

impl AuthClient {
    pub fn new(api_url: &str, anon_key: &str) -> anyhow::Result<Self> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            supabase_api_url: api_url.to_owned(),
            supabase_anon_key: anon_key.to_owned(),
            postgrest_client: Postgrest::new(format!("{}/rest/v1/", api_url.to_owned()))
                .schema("auth")
                .insert_header("apikey", anon_key.to_owned()),
        })
    }
}

#[derive(Debug, Error, Deserialize, Serialize)]
pub struct ErrorSchema {
    pub code: Option<u8>,
    pub error: Option<String>,
    pub error_description: Option<String>,
    pub msg: Option<String>,
}

impl Display for ErrorSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref e) = self.error {
            f.write_str(e)?;
            return Ok(());
        }
        if let Some(ref msg) = self.msg {
            f.write_str(msg)?;
            return Ok(());
        }
        Err(std::fmt::Error)
    }
}

#[derive(Debug)]
pub enum IdType {
    Email(String),
    PhoneNumber(String),
}
