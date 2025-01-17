use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PlayIntegrityRequest {
    #[serde(rename = "IntegrityToken")]
    pub integrity_token: String,
    #[serde(rename = "AppId")]
    pub app_id: String,
    #[serde(rename = "SessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "RequiresAuth")]
    pub requires_auth: bool,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PlayIntegrityResponse {
    #[serde(rename = "tokenPayloadExternal")]
    pub token_payload_external: Option<String>,
    #[serde(rename = "error")]
    pub error: Option<PlayIntegrityError>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlayIntegrityError {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub errors: Option<Vec<ErrorDetail>>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorDetail {
    pub message: Option<String>,
    pub domain: Option<String>,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct GenericResponseObject {
    #[serde(rename = "ResponseCode")]
    pub response_code: i32,
    #[serde(rename = "ResponseDescription")]
    pub response_description: String,
    #[serde(rename = "ResourceKey")]
    pub resource_key: Option<String>,
    #[serde(rename = "IsError")]
    pub is_error: bool,
    #[serde(rename = "Timestamp")]
    pub time_stamp: String,
}

#[derive(Debug)]
pub struct AppProperties {
    pub app_id: String,
    pub db_url: String,
    pub env: String,
}
