use crate::{cache, models::GenericResponseObject};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use deadpool_redis::Pool as RedisPool;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use tracing::{info, warn};

static VERSION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^v\d+(\.\d+)?$").unwrap_or_else(|_| Regex::new(r"^$").unwrap()));

#[utoipa::path(
    get,
    path = "/api/{version}/Health",
    params(
        ("version" = String, Path, description = "API version")
    ),
    responses(
        (status = 200, description = "Health check response")
    )
)]
pub async fn health_check(
    path: web::Path<(String,)>,
    request: HttpRequest,
    redis_pool: web::Data<RedisPool>,
) -> impl Responder {
    let version = path.into_inner().0;

    if !VERSION_REGEX.is_match(&version) {
        warn!(%version, "Invalid version format");
        return HttpResponse::BadRequest().body("Invalid version format");
    }

    if let Some(auth_header) = request.headers().get("Authorization") {
        match auth_header.to_str() {
            Ok(token) if token == "expected_token" => {}
            _ => {
                let response = create_generic_response(
                    401,
                    "Unauthorized request".to_string(),
                    Some("ProximaIntegrityApi_HealthCheck_Unauthorized_Request"),
                    true,
                );
                warn!(%version, "Unauthorized access");
                return HttpResponse::Unauthorized().json(response);
            }
        }
    } else {
        warn!(%version, "Missing Authorization header");
        return HttpResponse::Unauthorized().finish();
    }

    let cached_response = cache::get_cached_response(&redis_pool, &version).await;
    if !cached_response.is_error {
        return HttpResponse::Ok().json(cached_response);
    }

    //let response = format!("Health check for version: {}", version);
    let response = create_generic_response(
        200,
        format!("Health check for version: {}", version),
        Some("ProximaIntegrityApi_HealthCheck_Ok"),
        false,
    );
    let cloned_response = response.clone();

    tokio::spawn({
        async move {
            cache::set_cache(
                &redis_pool,
                &version,
                serde_json::to_string(&cloned_response).unwrap(),
                60,
            )
            .await;
        }
    });

    info!("Fetched fresh response - no cache: {}", to_json_string(&response));
    HttpResponse::Ok().json(&response)
}

pub async fn print_message(message: String) {
    println!("{}", "*".repeat(50));
    println!("{} - {}", message, Utc::now());
    println!("{}", "*".repeat(50));
}

pub fn print_message_on_thread(message: String) {
    println!("{}", "*".repeat(50));
    println!("{} - {}", message, Utc::now());
    println!("{}", "*".repeat(50));
}

pub fn create_generic_response(
    code: i32,
    description: String,
    resource_key: Option<&str>,
    is_error: bool,
) -> GenericResponseObject {
    GenericResponseObject {
        response_code: code,
        response_description: description.to_string(),
        resource_key: resource_key.map(|key| key.to_string()),
        is_error,
        time_stamp: Utc::now().to_rfc3339(),
    }
}

pub fn to_json_string<T: Serialize>(obj: &T) -> String {
    serde_json::to_string(obj).unwrap_or_else(|_| "{}".to_string())
}
