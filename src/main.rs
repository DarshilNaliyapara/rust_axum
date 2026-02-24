use std::env;

use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_axum::{AppState, fallback_handler};
use rust_axum::{config, routes};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_axum=debug,tower_http=info,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing application...");

    let pool = config::connect_to_database().await;
    tracing::info!("Database is connected!");

    let state = AppState { db_pool: pool, jwt_secret: env::var("ACCESS_SECRET").unwrap()};
    let app = routes::create_router(state.clone())
        .with_state(state)
        .fallback(fallback_handler)
        .layer(CookieManagerLayer::new())         
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = std::env::var("URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    tracing::info!("Listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
