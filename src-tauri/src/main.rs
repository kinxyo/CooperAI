// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;

/* RESPONSE */
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct OIChoices { //this is a subset of openai-response.
    text: String,
    index: u8,
    longprobs: Option<u8>,
    finish_reason: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OIChoices>,
}

/* REQUEST */
#[derive(Serialize, Debug)]
struct OIRequest {
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_response])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

#[tokio::main]
async fn completion(input: &str, token: &str) -> Result<std::string::String, std::string::String> {
  let https = HttpsConnector::new();
  let client = Client::builder().build(https);
  let uri = "https://api.openai.com/v1/engines/text-davinci-003/completions";

  let preamble = "You're a mental health professional and you're talking to a patient who needs a therapy. You want to help them feel better.";

  /* auth token */
  let oai_token: String = token.to_string();
  
  /* build auth header */
  let auth_header_val = format!("Bearer {}", oai_token);      
      let oi_request = OIRequest {
          prompt: format!("{} {}", preamble, input),
          max_tokens: 100,
          temperature: 0.7,
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
      let body = hyper::body::aggregate(res).await.expect("failed to read response body");
      
      /* deserialize response */
      let json: OIResponse = serde_json::from_reader(body.reader()).expect("failed to parse response");

      let answer: String = json.choices[0].text.to_string();
      println!("{}", answer );
      Ok(answer)

}

#[tauri::command]
fn get_response(input: &str, token: &str) -> String {
  let response = completion(input, token);
  response.unwrap()
}