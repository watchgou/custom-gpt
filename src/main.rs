mod model;
use axum::{routing::post, Router};
use std::env;
use std::{marker::PhantomData, net::SocketAddr};

use tower_http::validate_request::ValidateRequestHeaderLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    //加载环境变量 string 转 成 &'static str
    let models: &'static str = Box::leak(env::var("CHAT_GPT_MODEL").unwrap().into_boxed_str());
    //加载环境变量 string 转 成 &'static str
    let many_chat: &'static str = Box::leak(env::var("MANY_CHAT").unwrap().into_boxed_str());

    let static_file =
        Router::new().nest_service("/gpt", tower_http::services::ServeDir::new("./static/"));

    let app = Router::new()
        .route(
            "/chat",
            post(|request_dody| async { model::chat(request_dody, models, many_chat).await }),
        )
        .route(
            "/completion",
            post(|request_dody| async { model::completion(request_dody, many_chat).await }),
        )
        .route_layer(ValidateRequestHeaderLayer::custom(model::Authorization {
            _ty: PhantomData,
        }));
    let all = Router::new().merge(static_file).merge(app);
    let addr = SocketAddr::from(([0, 0, 0, 0], 19002));
    axum::Server::bind(&addr)
        .serve(all.into_make_service())
        .await
        .unwrap();
    Ok(())
}
