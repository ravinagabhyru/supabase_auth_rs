use log::error;
use reqwest::StatusCode;
use tracing::{debug, instrument, trace_span, Instrument};
use uuid::Uuid;

use crate::error::{AuthError, AuthErrorKind};
use crate::models::user::UserSchema;
use crate::util::handle_response_code;
use crate::AuthClient;

impl AuthClient {
    #[instrument(skip(self))]
    pub async fn get_user_by_token(&self, auth_token: &str) -> Result<UserSchema, AuthError> {
        if auth_token.is_empty() {
            error!("empty token");
            return Err(AuthError::InvalidParameters);
        }

        let resp = match self
            .http_client
            .get(format!("{}/auth/v1/{}", self.supabase_api_url, "user"))
            .bearer_auth(auth_token)
            .header("apiKey", &self.supabase_anon_key)
            .send()
            .instrument(trace_span!("gotrue get user"))
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                debug!("{}", e);
                return Err(AuthError::Http);
            }
        };
        let resp_code_result = handle_response_code(resp.status()).await;
        let resp_text = match resp.text().await {
            Ok(resp_text) => resp_text,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        resp_code_result?;

        let user = match serde_json::from_str::<UserSchema>(&resp_text) {
            Ok(user) => user,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };

        Ok(user)
    }

    #[instrument(skip(self))]
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<UserSchema>, AuthError> {
        let query_result = self.postgrest_client
            .from("users")
            .select("*")
            .eq("id", user_id.to_string())
            .single()
            .execute()
            .await;
        let query_response = match query_result {
            Ok(query_response) => query_response,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        if StatusCode::NOT_FOUND == query_response.status() {
            return Ok(None);
        }

        let handle_response_code_result = handle_response_code(query_response.status()).await;
        let body_text = match query_response.text().await {
            Ok(resp_text) => resp_text,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!(body = body_text);
        if let Err(e) = handle_response_code_result {
            if e.kind() == AuthErrorKind::NotFound {
                return Ok(None);
            }
            handle_response_code_result?
        }

        let user = match serde_json::from_str::<UserSchema>(&body_text) {
            Ok(user) => user,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };

        Ok(Some(user))
    }
}
