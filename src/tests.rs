use crate::{cache, validation};
use actix_web::{test, App};
use deadpool_redis::{Config as RedisConfig, Runtime};

#[actix_web::test]
async fn test_health_check() {
    let mut redis_cfg = RedisConfig::default();
    redis_cfg.url = Some("redis://127.0.0.1/".to_string());

    let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_pool))
            .configure(config_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/v1/health")
        .insert_header(("Authorization", "expected_token"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
