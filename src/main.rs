mod model;
use axum::{routing::post, Router};
use std::env;
use std::net::SocketAddr;
use tower_http::validate_request::ValidateRequestHeaderLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //加载环境变量 string 转 成 &'static str
    let models: &'static str = Box::leak(env::var("CHAT_GPT_MODEL").unwrap().into_boxed_str());
    //加载环境变量 string 转 成 &'static str
    let many_chat: &'static str = Box::leak(env::var("MANY_CHAT").unwrap().into_boxed_str());

    let app = Router::new()
        .route(
            "/chat",
            post(|request_dody| async { model::chat(request_dody, models, many_chat).await }),
        )
        .route(
            "/completion",
            post(|request_dody| async { model::completion(request_dody, many_chat).await }),
        )
        .route_layer(ValidateRequestHeaderLayer::basic("test", "password01!"));
    let addr = SocketAddr::from(([0, 0, 0, 0], 19002));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
