use driver::todo::TodoDriver;
use gateway::todo::TodoGateway;
use rest::axum::routing::get;
use rest::routes::Container;
use rest::tokio;
use rest::{
    axum::{Router, Server, Extension},
    routes,
};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let container = create_container();
    let app = Router::new()
        .nest("", routes::routes())
        .layer(Extension(Arc::new(container)))
        .route("/v1/systems/ping", get(|| async { "pong" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 10500));

    tracing::debug!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_container() -> Container {
    Container {
        todo_gateway: TodoGateway::new(mry::new!(TodoDriver{})),
    }
}