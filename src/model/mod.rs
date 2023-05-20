pub mod chat;

pub use chat::*;

pub mod completions;

pub use completions::*;

use async_openai::types::{ChatChoice, Choice};

use serde::{Deserialize, Serialize};

use std::marker::PhantomData ;

use tower_http::validate_request::ValidateRequest;


use hyper::{Request, Response};

use log::info;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Message {
    pub msg: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_max_token")]
    pub max_token: u16,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResult {
    pub code: u8,
    pub data: Vec<ChatChoice>,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResult {
    pub code: u8,
    pub data: Vec<Choice>,
    pub msg: String,
}

fn default_model() -> String {
    "text-davinci-003".to_string()
}

fn default_max_token() -> u16 {
    7
}

fn default_temperature() -> f32 {
    0.0
}

pub struct Authorization<ResBody> {
   pub _ty: PhantomData<fn() -> ResBody>,
}

impl<ResBody> Clone for Authorization<ResBody> {
    fn clone(&self) -> Self {
        Self { _ty: PhantomData }
    }
}

impl<B, ResBody> ValidateRequest<B> for Authorization<ResBody>
where
    ResBody: http_body::Body + Default,
{
    type ResponseBody = ResBody;
    fn validate(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {

        let auth=request.headers().get(http::header::AUTHORIZATION);
        match auth {
            Some(a)=>{
        
                info!("authorization {:?}",a);
                Ok(())
            },
             _ => {
                 let mut res = Response::new(ResBody::default());
                *res.status_mut() = axum::http::StatusCode::UNAUTHORIZED;
                Err(res)
             },
        }
       
    }
}