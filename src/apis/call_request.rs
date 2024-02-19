use crate::models::general::llm::{APIResponse, ChatCompletion, Message};

use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Box::new(e) wraps the error e in a Box. Box<dyn std::error::Error + Send> means the error is boxed into a heap-allocated type
// that implements the std::error::Error trait and can be sent across threads (Send). This is a common pattern for handling errors in
// Rust when you want to return a generic error type from a function that can represent multiple different errors.

// The -> Box<dyn std::error::Error + Send> part before the closure body explicitly specifies the return type of the closure,
// which is necessary here because we're transforming the error into a boxed error, a type that can represent any kind of error at runtime.
// This makes the error handling more flexible.
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    // get information from .env file
    dotenv().ok();

    // Extract API key
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in the .env file!");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_KEY not found in the .env file!");

    // Confirm Endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org)
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages: messages,
        temperature: 0.1,
    };

    // Extract API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send Response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test, give me a short response.".to_string(),
        };
        let messages: Vec<Message> = vec![message];
        let res = call_gpt(messages).await;
        if let Ok(res_str) = res {
            dbg!(res_str);
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
//cargo test test_call_to_openai -- --nocapture
