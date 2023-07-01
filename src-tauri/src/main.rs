// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;

/* CHAT */
#[derive(Deserialize, Debug)]
struct Convo {
    content: String,
    role: String,
}

#[derive(Serialize, Debug)]
struct Message {
    role: String,
    content: String,
}

/* RESPONSE */
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct OIChoices { //this is a subset of openai-response.
    message: Convo,
    index: u8,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct OIUsage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OIResponse {
    choices: Vec<OIChoices>,
    created: Option<u64>,
    id: Option<String>,
    model: Option<String>,
    object: Option<String>,
    usage: Option<OIUsage>,
}

/* REQUEST */
#[derive(Serialize, Debug)]
struct OIRequest {
    model: String,
    messages: Vec<Message>,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tokio::main]
async fn completion(
    token: &str,
    chat_history: Vec<Convo>,
) -> Result<std::string::String, std::string::String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/chat/completions";

    /* auth token */
    let oai_token: String = token.to_string();

    /* serialization-error CHEAT */
    let mut chats: Vec<Message> = Vec::new();
    for i in chat_history {
        println!("{}: {}", i.role, i.content);
        let message = Message {
            role: i.role,
            content: i.content,
        };
        chats.push(message);
    }

    /* build auth header */
    let auth_header_val = format!("Bearer {}", oai_token);
    let oi_request = OIRequest {
        model: String::from("gpt-3.5-turbo"),
        messages: chats,
    };

    /* building request */
    let body = Body::from(serde_json::to_vec(&oi_request).expect("failed to serialize request"));
    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)
        .expect("poor request or network error");

    /* now we're gonna use our hyper client to send the request to the endpoint and wait for a response */
    let res = client.request(req).await.expect("failed to send request");
    let body = hyper::body::aggregate(res)
        .await
        .expect("failed to read response body");

    /* deserialize response */
    let json: OIResponse = serde_json::from_reader(body.reader()).expect("invalid json");

    println!("response: {:?}", json);

    let answer: String = json.choices[0].message.content.to_string();

    Ok(answer)
}

#[tauri::command]
fn get_response(chats: Vec<Convo>, token: &str) -> String {
    /* extracting chats to sendable prompts */
    let response = completion(token, chats);
    response.unwrap()
}