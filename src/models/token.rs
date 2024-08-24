use serde::{Deserialize, Serialize};

use crate::models::user::UserSchema;

#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(default)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub expires_at: u64,
    pub refresh_token: String,
    pub user: Option<UserSchema>,
    pub provider_token: String,
    pub provider_refresh_token: String,
    pub weak_password: Option<WeakPasswordError>,
}

#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(default)]
pub struct WeakPasswordError {
    pub message: String,
    pub reasons: Vec<String>,
}
