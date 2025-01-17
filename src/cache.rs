use crate::handler::create_generic_response;
use crate::models::GenericResponseObject;
use deadpool_redis::{redis::AsyncCommands, Config, Pool as RedisPool, Runtime};
use tracing::warn;

pub async fn init_pool(redis_url: &str) -> RedisPool {
    let cfg = Config::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis connection pool")
}

pub async fn get_cached_response(redis_pool: &RedisPool, key: &str) -> GenericResponseObject {
    let mut redis_conn = match redis_pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            let error_message = format!("Redis connection error: {}", err);
            warn!("{}", error_message);
            return create_generic_response(401, error_message, Some("Redis_Error"), true);
        }
    };

    match redis_conn.get::<_, String>(key).await {
        Ok(value) => {
            return create_generic_response(200, value, Some("Redis_Found_Key"), false);
        }
        Err(err) => {
            let error_message = format!("Error fetching key {}: {}", key, err);
            warn!("{}", error_message);
            return create_generic_response(404, error_message, Some("Redis_Missing_Key"), true);
        }
    }
}

pub async fn set_cache(redis_pool: &RedisPool, key: &str, value: String, expiration: usize) {
    let mut redis_conn = match redis_pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            let error_message = format!("Redis connection error: {}", err);
            warn!("{}", error_message);
            return;
        }
    };

    if let Err(e) = redis_conn
        .set_ex::<_, String, i32>(key, value, expiration)
        .await
    {
        warn!(error = %e, "Failed to cache health check response");
    }
}
