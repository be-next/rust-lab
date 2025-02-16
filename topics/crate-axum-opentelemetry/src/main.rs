mod open_telemetry;

use crate::open_telemetry::init_trace;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::trace::TraceError;
use rand::Rng;
use tracing::{error, event, info, warn, Level};
use tracing_subscriber::layer::SubscriberExt;

// OpenTelemetry Setup in RUST

#[tracing::instrument]
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
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = init_trace().unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let router = Router::new().route("/health", get(health_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("App is running..");
    info!("App is running...");
    let result = axum::serve(listener, router).await;

    if let Err(_) = result {
        error!("Application is dying...");
        global::shutdown_tracer_provider();
    }
    Ok(())
}
