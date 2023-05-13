mod chat;
use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::validate_request::ValidateRequestHeaderLayer;
#[tokio::main]
async fn main() {
    
    let app = Router::new()
        .route("/chat", post(|request_dody| async { chat::chat(request_dody).await }))
        .route_layer(ValidateRequestHeaderLayer::basic("test", "password01!"));
    let addr = SocketAddr::from(([0, 0, 0, 0], 19002));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
