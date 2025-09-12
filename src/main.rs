mod api;
mod domain;
mod service;

//#[cfg(any(test, feature = "test-utils"))]
pub mod test_util;

use api::routes::setup_routing;

#[tokio::main]
async fn main() {
    let router = setup_routing().await;
    let addr = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
