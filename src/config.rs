use crate::models::AppProperties;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;

static APP_PROPERTIES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut properties = HashMap::new();

    // Application Properties
    properties.insert("app_name", "Proxima Integrity API");
    properties.insert("app_id", "ProximaIntegrityApi");
    properties.insert("version", "0.1.0");
    properties.insert("author", "Okechukwu Opel Nnamdi Agufuobi");
    properties.insert("email", "Okechukwu.Agufuobi@hotmail.com");
    properties.insert("timezone", "GMT");
    properties.insert("license", "MIT");

    // App Hosting
    properties.insert("base_url", "http://proximaintegrity/");
    properties.insert("port", "8098");

    // Database Properties
    properties.insert("db_connection_timeout", "5000");
    properties.insert("db_pool_size", "10");

    // Security Configuration
    properties.insert("token_expiry", "10");

    if cfg!(feature = "dev") {
        properties.insert("enable_https", "5000");
        properties.insert("allowed_origins", "10");

        properties.insert("api_timeout", "10");
        properties.insert("max_api_retries", "10");
        properties.insert("rate_limit", "10");
    } else if cfg!(feature = "staging") {
        properties.insert("enable_https", "5000");
        properties.insert("allowed_origins", "10");

        properties.insert("api_timeout", "10");
        properties.insert("max_api_retries", "10");
        properties.insert("rate_limit", "10");
    } else if cfg!(feature = "release") {
        properties.insert("enable_https", "5000");
        properties.insert("allowed_origins", "10");

        properties.insert("api_timeout", "10");
        properties.insert("max_api_retries", "10");
        properties.insert("rate_limit", "10");
    }

    properties
});

pub struct Config {
    pub server_addr: String,
    pub database_url: String,
    pub redis_url: String,
    pub app_insights_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8098".to_string()),
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")?,
            app_insights_key: env::var("APP_INSIGHTS_KEY")?,
        })
    }
}

impl AppProperties {
    fn from_hashmap(map: &HashMap<&str, &str>) -> Result<Self, &'static str> {
        Ok(AppProperties {
            app_id: map.get("APP_ID").ok_or("APP_ID not found")?.to_string(),
            db_url: map.get("DB_URL").ok_or("DB_URL not found")?.to_string(),
            env: map.get("ENV").ok_or("ENV not found")?.to_string(),
        })
    }
}
