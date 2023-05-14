use async_openai::{
    types::{ CreateCompletionRequestArgs},
    Client,
};

use axum::{http::StatusCode, response::IntoResponse, Json};

use super::*;

pub async fn completion(Json(_mesage): Json<Message>, models: &str, many_chat: &str) -> impl IntoResponse{
    let many: u8 = many_chat.parse::<u8>().unwrap();
    let client: Client = Client::new();
    let request=CreateCompletionRequestArgs::default()
        .model(models)
        .n(many)
        .user("async-openai")
        .build();

       let request = match request {
        Ok(request) => request,
        Err(_) => panic!("error"),
    };
    let respose=client.completions().create(request).await;

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
