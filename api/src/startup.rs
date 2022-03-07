use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{
    create_user_session_handler, get_health_data_handler, get_stats_handler, get_stats_sse_handler,
    health_check_handler, list_user_session_handler,
};
use crate::stats;
use axum::{
    error_handling::HandleErrorLayer, extract::Extension, http::Method, http::StatusCode,
    routing::get, routing::post, BoxError, Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::TcpListener, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer, Origin},
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub async fn prepare(configuration: &Settings) -> eyre::Result<Router> {
    let connection_pool = get_connection_pool(&configuration.database);
    let connection_pool_writable = get_connection_pool(&configuration.database_writable);

    let (tx_get_stats, watch_stats) =
        stats::updates::init(&connection_pool, &connection_pool_writable).await?;

    let base_url = configuration.clone().application.base_url;

    let middleware = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Millis)),
        )
        // Set a timeout
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(1)),
        )
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:8080".parse().unwrap()))
                .allow_methods(vec![Method::GET]),
        )
        .layer(Extension(tx_get_stats))
        .layer(Extension(watch_stats))
        .layer(Extension(base_url))
        .layer(Extension(connection_pool))
        .layer(Extension(connection_pool_writable));

    // build our application with a route
    let app = Router::new()
        .route("/healthz", get(health_check_handler))
        .route("/pulstats/health_data", get(get_health_data_handler))
        .route("/pulstats/stats", get(get_stats_handler))
        .route("/pulstats/stats/listen", get(get_stats_sse_handler))
        .route("/pulse/user_session", post(create_user_session_handler))
        .route("/manu/user_session", get(list_user_session_handler))
        .layer(middleware.into_inner());

    Ok(app)
}

pub async fn run(app: Router, listener: TcpListener) -> eyre::Result<()> {
    let address = listener.local_addr().unwrap();
    tracing::debug!("listening on {}", address);

    // run it
    axum::Server::from_tcp(listener).unwrap().serve(app.into_make_service()).await?;

    Ok(())
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "Request took too long".to_string())
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", err))
    }
}
