#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(dead_code)]

mod arch;
mod server;

use arch::*;
use server::*;

use hyper::body::Buf;
use hyper::{header, Body, Request};
use std::env;
use tauri::State;

fn main() {
    tauri::Builder::default()
        .manage(SharedStream::default())
        .invoke_handler(tauri::generate_handler![get_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tokio::main]
async fn completion(
    backend: Server,
    chat_history: Vec<Convo>,
) -> Result<std::string::String, std::string::String> {

    /* build auth header */
    let oi_request = OIRequest {
        model: String::from("gpt-3.5-turbo"),
        messages: chat_history,
    };

    /* building request */
    let body = Body::from(serde_json::to_vec(&oi_request).expect("failed to serialize request"));
    let req = Request::post(backend.uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &backend.header)
        .body(body)
        .expect("poor request or network error");

    /* now we're gonna use our hyper client to send the request to the endpoint and wait for a response */
    let res: hyper::Response<Body>;
    let res_err = backend.client.request(req).await;
    match res_err {
        Ok(v) => res = v,
        Err(_) => return Err(String::from("Please check your internet connection, CooperAI requires it.")),
    }
    let body = hyper::body::aggregate(res)
        .await
        .expect("failed to read response body");

    /* deserialize response */
    let json: OIResponse = serde_json::from_reader(body.reader()).expect("invalid json");
    let verdict: String = json.choices[0].message.content.to_string();

    Ok(verdict)
}

#[tauri::command]
fn get_response(chats: Vec<Convo>, state: State<'_, SharedStream>) -> String {
    let backend = state.0.lock().unwrap();
    let response = completion(backend.clone(), chats);
    match response {
        Ok(v) => return v,
        Err(e) => return e,
    }
}