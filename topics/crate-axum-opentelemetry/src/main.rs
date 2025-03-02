mod open_telemetry;

use crate::open_telemetry::init_tracing_subscriber;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
// use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
// use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;
use opentelemetry::trace::TraceError;
use rand::Rng;
use tracing::{error, event, info, warn, Level};

#[tracing::instrument(name = "health_handler", level = "info")]
async fn health_handler() -> StatusCode {
    let number = rand::rng().random_range(1..4);
    match number {
        1 => {
            info!("Number is 1 returning Ok Response");
            StatusCode::OK
        }
        2 => {
            error!("Number is 2 returning Error Response");
            StatusCode::UNAUTHORIZED
        }
        3 => {
            warn!("Number is 3 returning Forbidden");
            StatusCode::FORBIDDEN
        }
        _ => {
            event!(Level::INFO, "reason" = "Unknown number detected");
            StatusCode::BAD_REQUEST
        }
    }
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), TraceError> {
    let _guard = init_tracing_subscriber();

    let router = Router::new().route("/health", get(health_handler));
    // .layer(OtelInResponseLayer::default())
    // .layer(OtelAxumLayer::default());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("App is running...");
    let result = axum::serve(listener, router.into_make_service()).await;

    if let Err(_) = result {
        error!("Application is dying...");
        // tracer_provider.shutdown().unwrap();
    }
    Ok(())
}
