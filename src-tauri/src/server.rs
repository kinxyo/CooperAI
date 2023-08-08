use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use std::env;
use hyper_tls::HttpsConnector;
use hyper::Client;

/* SERVER CREATION */
pub struct Server {
    pub client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    pub uri: String,
    pub header: String
}

/* STATE MANAGEMENT */
#[derive(Default)]
pub struct SharedStream(pub Arc<Mutex<Server>>);

impl Default for Server {
    fn default() -> Self {
        dotenv().ok(); let secret = env::var("OAI_KEY").unwrap();
        let https: HttpsConnector<hyper::client::HttpConnector> = HttpsConnector::new();
        let connector: Client<HttpsConnector<hyper::client::HttpConnector>> =
            Client::builder().build(https);
        Self {
            client: connector,
            uri: String::from("https://api.openai.com/v1/chat/completions"),
            header: format!("Bearer {}", secret)

        }
    }
}

impl Server {
    pub fn initialize_sever() -> Self {
        Server::default()
    }
}