use crate::api_doc;
use crate::{handler, validateintegritytoken};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/{version}")
            .route(
                "/ValidateToken",
                web::post().to(validateintegritytoken::validate_token),
            )
            .route(
                "/DecodeIntegrityToken",
                web::post().to(validateintegritytoken::validate_token),
            )
            .route("/Health", web::get().to(handler::health_check)),
    );
}
