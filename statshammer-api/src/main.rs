use axum::{
    routing::{get, post},
    Router,
};
mod abilities;
mod units;

#[tokio::main]
async fn main() {
    let aos_routes = Router::new()
        .route("/abilities", get(abilities::get_abilities))
        .route("/compare/average", post(units::compare_average));

    let app = Router::new().nest("/aos", aos_routes);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
