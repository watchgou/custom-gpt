use async_openai::{
    types::{ChatChoice, ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, Role},
    Client,
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

//use super::CHAT_GPT_MODEL;

#[derive(Deserialize)]
pub struct Message {
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResult {
    pub code: u8,
    pub data: Vec<ChatChoice>,
    pub msg: String,
}

pub async fn chat(Json(_mesage): Json<Message>,models:&str,many_chat:& str) ->impl IntoResponse {
    let many:u8 = many_chat.parse::<u8>().unwrap();
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let  client: Client = Client::new();
    let mut message = ChatCompletionRequestMessage::default();
    message.role = Role::User;
    message.content = _mesage.msg;
    let msg_vec = vec![message];

    let request = CreateChatCompletionRequestArgs::default()
        .messages(msg_vec)
        .model(models)
        .n(many)
        .user("async-openai")
        .build();

    let request = match request {
        Ok(request) => request,
        Err(_) => panic!("error"),
    };

    let respons = client.chat().create(request).await;

    let result = match respons {
        Ok(choices) => ChatResult {
            code: 0,
            data: choices.choices,
            msg: String::from("SUCCESSED"),
        },
        Err(e) => ChatResult {
            code: 0,
            data: vec![],
            msg: e.to_string(),
        },
    };

   (StatusCode::OK, Json(result))
}
