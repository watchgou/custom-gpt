use async_openai::{
    types::{CreateCompletionRequestArgs, Prompt},
    Client,
};

use axum::{http::StatusCode, response::IntoResponse, Json};

use super::*;

// 补全
pub async fn completion(Json(_mesage): Json<Message>, many_chat: &str) -> impl IntoResponse {
    let many: u8 = many_chat.parse::<u8>().unwrap();
    let client: Client = Client::new();
    let request = CreateCompletionRequestArgs::default()
        .prompt(Prompt::String(_mesage.msg))
        .model(_mesage.model)
        .n(many)
        .user("async-openai")
        .max_tokens(_mesage.max_token)
        .temperature(_mesage.temperature)
        .build();

    let request = match request {
        Ok(request) => request,
        Err(_) => panic!("error"),
    };
    let respose = client.completions().create(request).await;

    let result = match respose {
        Ok(choices) => CompletionResult {
            code: 0,
            data: choices.choices,
            msg: String::from("SUCCESSED"),
        },
        Err(e) => CompletionResult {
            code: 0,
            data: vec![],
            msg: e.to_string(),
        },
    };

    (StatusCode::OK, Json(result))
}
