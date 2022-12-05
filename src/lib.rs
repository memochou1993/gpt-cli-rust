use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

pub async fn fetch() -> Result<Response, Box<dyn Error>> {
    let api_url = env::var("OPENAI_API_URL").unwrap();
    let api_key = env::var("OPENAI_API_KEY").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", api_key).parse().unwrap(),
    );

    let body = Request{
        model: String::from("text-davinci-003"),
        prompt: String::from("\n\nHuman: Hello, who are you?\nAI: I am an AI created by OpenAI. How can I help you today?\nHuman: Hello, who are you?"),
        temperature: 0.9,
        max_tokens: 150,
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.6,
        stop: vec![String::from(" Human:"), String::from(" AI:")],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/completions", api_url))
        .headers(headers)
        .json(&body)
        .send()
        .await?
        .json::<Response>()
        .await?;

    return Ok(res);
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<usize>,
    pub model: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub text: Option<String>,
    pub index: Option<usize>,
    pub logprobs: Option<usize>,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: Option<usize>,
    pub completion_tokens: Option<usize>,
    pub total_tokens: Option<usize>,
}

#[derive(Serialize)]
struct Request {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: usize,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
    stop: Vec<String>,
}
