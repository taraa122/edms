mod config;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use config::AppConfig;
use handlers::{echo::echo, health::health, hello::hello};
use std::{net::SocketAddr, time::Duration};
use tower::limit::RateLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
 
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_webserver=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

   
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/config.yaml".into());
    let cfg = AppConfig::from_file(&config_path)?;
    tracing::info!("Loaded config from {}: {:?}", config_path, cfg);

   
    let endpoint_count = 3; // /health, /hello/:name, /echo
    if endpoint_count > cfg.limits.max_endpoints {
        anyhow::bail!(
            "Configured max_endpoints={} but server defines {} endpoints",
            cfg.limits.max_endpoints,
            endpoint_count
        );
    }

    
    let app = Router::new()
        .route("/health", get(health))
        .route("/hello/:name", get(hello))
        .route("/echo", post(echo));

    
    let app = app.layer(RateLimitLayer::new(
        cfg.limits.max_requests_per_minute as u64,
        Duration::from_secs(60),
    ));

    
    let addr = SocketAddr::from((
        cfg.server
            .host
            .parse::<std::net::IpAddr>()
            .expect("invalid host in config"),
        cfg.server.port,
    ));

    tracing::info!("Starting server on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
