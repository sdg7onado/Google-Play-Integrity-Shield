mod api_doc;
mod cache;
mod config;
mod db;
mod handler;
mod logger;
mod middleware;
mod models;
mod routes;
mod telemetry;
mod validateintegritytoken;
mod validation;

use crate::handler::print_message;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use api_doc::ApiDoc;
use config::Config;
use tokio::signal;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    logger::init();

    let config = Config::from_env().expect("Failed to load configuration");

    telemetry::init(&config.app_insights_key);

    let db_pool = db::init_pool(&config.database_url).await;
    let redis_pool = cache::init_pool(&config.redis_url).await;

    print_message(format!("Starting server on {}", config.server_addr)).await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
            .wrap(Logger::default())
            .wrap(middleware::security_headers())
            .configure(routes::configure_routes)
            .route(
                "/swagger-ui",
                web::get().to(|| async { HttpResponse::Ok() }),
            )
            .service(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
    })
    .bind(config.server_addr)?
    .run();

    let server_handle = server.handle();

    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }
    });

    print_message(format!("Started the Play Integrity API server!")).await;

    signal::ctrl_c().await?;

    print_message("Termination signal received. Shutting down gracefully...".to_string()).await;
    print_message("Server is shutting down...".to_string()).await;

    server_handle.stop(true).await;

    Ok(())
}
