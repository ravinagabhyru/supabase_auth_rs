use crate::util::handle_response_code;
use crate::AuthClient;
use crate::AuthError;
use log::error;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct DeleteBody {
    should_soft_delete: bool,
}

impl AuthClient {
    #[instrument(skip_all)]
    pub async fn soft_delete_user(&self, auth_token: &str) -> Result<(), AuthError> {
        if auth_token.is_empty() {
            error!("empty token");
            return Err(AuthError::InvalidParameters);
        }

        let resp = match self
            .http_client
            .delete(format!("{}/auth/v1/admin/users/", self.supabase_api_url))
            .json(&DeleteBody {
                should_soft_delete: true
            })
            .bearer_auth(&self.supabase_anon_key)
            .header("apiKey", &self.supabase_anon_key)
            .send()
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
                log::error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        Ok(resp_code_result?)
    }


    #[instrument(skip_all)]
    pub async fn hard_delete_user(&self, user_id: Uuid) -> Result<(), AuthError> {
        let resp = match self
            .http_client
            .delete(format!("{}/auth/v1/admin/users/{}", self.supabase_api_url, user_id.to_string()))
            .json(&DeleteBody {
                should_soft_delete: true
            })
            .bearer_auth(&self.supabase_anon_key)
            .header("apiKey", &self.supabase_anon_key)
            .send()
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
                log::error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        Ok(resp_code_result?)
    }
}