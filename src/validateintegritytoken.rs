use crate::handler::{create_generic_response, print_message_on_thread};
use crate::{
    cache::get_cached_response,
    models::{PlayIntegrityRequest, PlayIntegrityResponse},
    validation,
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use deadpool_redis::Pool as RedisPool;
use gcp_auth::{AuthenticationManager, Token};
use reqwest::Client;
use serde_json::json;
use tracing::warn;

const SCOPE: &str = "https://www.googleapis.com/auth/playintegrity";
const GOOGLE_PLAY_INTEGRITY_API_URL: &str = "https://playintegrity.googleapis.com/v1/";

#[utoipa::path(
    post,
    path = "/api/{version}/ValidateToken",
    params(
        ("version" = String, Path, description = "API version")
    ),
    request_body = PlayIntegrityRequest,
    responses(
        (status = 200, description = "Token validation result", body = PlayIntegrityResponse)
    )
)]
pub async fn validate_token(
    body: web::Json<PlayIntegrityRequest>,
    redis_pool: web::Data<RedisPool>,
) -> impl Responder {
    print_message_on_thread(format!(
        "Play Integrity Request - {}",
        serde_json::to_string(&body).unwrap(),
    ));

    let integrity_token = &body.integrity_token;
    let app_id = &body.app_id;
    let session_id = body.session_id.as_deref().unwrap_or("");

    if !validation::is_valid_integritytoken(Some(integrity_token.to_string())) {
        return handle_error(
            400,
            "Invalid integrity token".to_string(),
            "ProximaIntegrityApi_InvalidIntegrityToken",
        );
    }

    if !validation::is_valid_appid(Some(app_id.to_string())) {
        return handle_error(
            400,
            "Invalid integrity token".to_string(),
            "ProximaIntegrityApi_InvalidAppId",
        );
    }

    if body.requires_auth && validation::is_valid_sessionid(Some(session_id.to_string())) {
        let cached_response = get_cached_response(&redis_pool, &session_id).await;
        if cached_response.is_error {
            return handle_error(
                400,
                "Invalid Session Id".to_string(),
                "ProximaIntegrityApi_InvalidSessionId",
            );
        }
    }

    let auth_manager = match AuthenticationManager::new().await {
        Ok(manager) => manager,
        Err(e) => {
            return handle_error(
                424,
                format!("{:?}", e),
                "ProximaIntegrityApi_AuthenticationManager_Error",
            )
        }
    };

    let token: Token = match auth_manager.get_token(&[SCOPE]).await {
        Ok(outtoken) => outtoken,
        Err(e) => return handle_error(401, format!("{:?}", e), "ProximaIntegrityApi_Token_Error"),
    };

    if token.as_str().is_empty() {
        return handle_error(
            401,
            "Received an empty access token".to_string(),
            "ProximaIntegrityApi_Empty_Access_Token_Error",
        );
    }

    let access_token = token.as_str();

    let client = Client::new();

    let url = format!(
        "{}{}:decodeIntegrityToken",
        GOOGLE_PLAY_INTEGRITY_API_URL, app_id
    );

    let payload = json!({
        "integrity_token": integrity_token
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status_code = resp.status();
            match resp.json::<PlayIntegrityResponse>().await {
                Ok(play_integrity_response) => {
                    print_message_on_thread(format!("{:?}", &play_integrity_response));
                    return HttpResponse::build(status_code).json(play_integrity_response);
                }
                Err(e) => handle_error(
                    500,
                    format!("{:?}", e),
                    "ProximaIntegrityApi_Validation_Error_BadRequst",
                ),
            }
        }
        Err(e) => handle_error(
            500,
            format!("{:?}", e),
            "ProximaIntegrityApi_Validation_Error_NetworkError",
        ),
    }
}

fn handle_error(errorcode: i32, errormessage: String, resource_key: &str) -> HttpResponse {
    warn!("ValidateToken. Error: {:?}", errormessage);
    eprintln!("ValidateToken. Error: : {:?}", errormessage);
    let response = create_generic_response(errorcode, errormessage, Some(resource_key), true);

    match StatusCode::from_u16(errorcode as u16) {
        Ok(status_code) => HttpResponse::build(status_code).json(response),
        Err(_) => HttpResponse::InternalServerError().json(response),
    }
}
