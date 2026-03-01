use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

/// Sends a prompt to the Gemini API and returns the raw JSON response string.
pub async fn ask_gemini(api_key: &str, prompt: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    response.text().await
}


// AIzaSyB_dXpxFgXVhsBkrx-MaIfikWhMD2xUlT0

