/* couldn't think of a suitable name so named it `core` | it's supposed to be opposite of 'interface' */

#![allow(dead_code)]

use dotenv::dotenv;
use hyper::body::Buf;
use hyper::Client;
use hyper::{header, Body, Request, Response};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};

/* ----------------------------------------------------------- */

// The initial version of these structs was inspired by `codetothemoon`'s youtube video.

// CHAT
#[derive(Deserialize, Serialize, Debug)]
pub struct Conversation {
    pub content: String,
    pub role: String,
}

// REQUEST
#[derive(Serialize, Debug)]
pub struct Payload {
    pub model: String,
    pub messages: Vec<Conversation>,
}

// RESPONSE
#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub choices: Vec<Choices>,
    created: Option<u64>,
    id: Option<String>,
    model: Option<String>,
    object: Option<String>,
    usage: Option<Usage>,
}

// subset of `ChatResponse`
#[derive(Deserialize, Debug)]
pub struct Choices {
    pub message: Conversation,
    index: u8,
    finish_reason: String,
}

// subset of `ChatResponse`
#[derive(Deserialize, Debug)]
pub struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}

/* ----------------------------------------------------------- */

// CLIENT
#[derive(Debug)]
pub struct APIClient {
    /* these parts remain constant so building them at the start of the application */
    pub client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    pub uri: String,
    pub header: String, //token, in some cases, may change though.
}

impl Default for APIClient {
    fn default() -> Self {
        
        dotenv().ok();
        
        let secret = match env::var("OAI_KEY") {
            Ok(val) => val,
            Err(e) => {
                println!("Unable to create `APIClient`:\n {e}\nPlease open an issue @github/kinxyo/CooperAI");
                String::new()
            }
        };

        let https = HttpsConnector::new();
        let connector = Client::builder().build(https);

        println!("✅ APIClient");

        Self {
            client: connector,
            uri: String::from("https://api.openai.com/v1/chat/completions"),
            header: format!("Bearer {}", secret),
        }
    }   

}

/* ----------------------------------------------------------- */

// STATE MANAGEMENT
#[derive(Default)]
pub struct SharedStream(pub Arc<Mutex<APIClient>>);

/* ----------------------------------------------------------- */

// HELPER FUNCTIONS

pub fn save_token(token: &String) -> Result<(), std::io::Error> {
    
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(".env")?;

    std::env::set_var("OAI_KEY", token);
    
    write!(file, "OAI_KEY={}", token)?;
    Ok(())
}

pub fn generate_request(payload: Payload, client: &APIClient ) -> Result<Request<Body>, Box<dyn std::error::Error>> {
    
    let body = Body::from(serde_json::to_vec(&payload)?);
    
    /* creating a http request for POST method */
    let request = Request::post(&client.uri) 
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &client.header)
        .body(body)?;

    Ok(request)
}

pub async fn generate_answer(response: Response<Body> ) -> Result<String, Box<dyn std::error::Error>> {
    
    let body = hyper::body::aggregate(response)
        .await?;

    /* any error occuring here 👇 is likey due to invalid API key */
    let deserialized_body: ChatResponse = serde_json::from_reader(body.reader())?; 

    let answer: String = deserialized_body.choices[0].message.content.to_string();

    Ok(answer)
}

pub async fn completion(client: &mut APIClient, chat_history: Vec<Conversation> ) -> Result<String, Box<dyn std::error::Error>> {

    let history = &chat_history.len();

    /* generating context */
    let payload = Payload {
        model: "gpt-3.5-turbo".to_string(),
        messages: chat_history,
    };

    /* creating request */
    let request = match generate_request(payload, &client) {
        Ok(v) => v,
        Err(e) => {
            println!("🆘 Failure experienced in building request!\n{e}");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "ERROR: Failure experienced in creating request | Please open an issue @github/kinxyo/CooperAI")));
        }
    };

    /* fetching response */
    let response = match client.client.request(request).await {
        Ok(v) => v,
        Err(e) => {
            println!("🆘 Failure experienced in sending the request over network\n{e}");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "ERROR: Please check your internet connection, Cooper requires it.",
            )));
        }
    };

    /* deriving answer from the response */
    match generate_answer(response).await {
        Ok(v) => Ok(v),
        Err(e) => {
            println!(
                "🆘 Failure experienced in fetching answer-- aggregating or deserializing\n{e}"
            );

            // println!("{:?}", client); //for debugging purpose

            if history > &5 {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Topic Closed. Press `Ctrl + r` to restart.")));
            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "ERROR: Please make sure your provided API Key is valid; otherwise open an issue @github/kinxyo/CooperAI.")));
            }
        }
    }
}

pub fn env_var_exists() -> bool {

    match env::var("OAI_KEY") {
        Ok(val) => !val.is_empty(),
        Err(_e) => false,
    }

}