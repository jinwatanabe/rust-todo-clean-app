use std::net::SocketAddr;

use rest::axum::routing::get;
use rest::tokio;
use rest::{
    axum::{Router, Server},
    routes,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("", routes::routes())
        .route("/v1/systems/ping", get(|| async { "pong" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 10500));

    tracing::debug!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
