mod chat;
use axum::{routing::post, Router};
use std::env;
use std::net::SocketAddr;
use tower_http::validate_request::ValidateRequestHeaderLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // string 转 成 &'static str
    let models: &'static str = Box::leak(env::var("CHAT_GPT_MODEL").unwrap().into_boxed_str());
    let many_chat: &'static str = Box::leak(env::var("MANY_CHAT").unwrap().into_boxed_str());
    // let many:u8 = many_chat.parse::<u8>()?;
    // let many_value=many.clone();
    let app = Router::new()
        .route(
            "/chat",
            post(|request_dody| async {

                 chat::chat(request_dody, models,many_chat).await 
                }),
        )
        .route_layer(ValidateRequestHeaderLayer::basic("test", "password01!"));
    let addr = SocketAddr::from(([0, 0, 0, 0], 19002));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
