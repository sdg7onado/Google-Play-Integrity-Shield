use actix_web::middleware::DefaultHeaders;

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("X-Frame-Options", "DENY"))
        .add((
            "Strict-Transport-Security",
            "max-age=63072000; includeSubDomains; preload",
        ))
}
