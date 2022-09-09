//! Contains the REST based API for calculating the damage statistics for the supported games.
//! This is a very light interface layer, with the logic being called from
//! [`aos-statshammer`](aos_statshammer) and `warcry-statshammer` crates.
use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

mod abilities;
mod comparisons;
mod errors;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let aos_routes = Router::new()
        .route("/abilities", get(abilities::get_abilities))
        .route("/compare/average", post(comparisons::compare_average))
        .route("/compare/simulated", post(comparisons::compare_simulated));

    let app = Router::new().nest("/aos", aos_routes).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new()),
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
