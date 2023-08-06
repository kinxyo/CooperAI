use serde_derive::{Deserialize, Serialize};
/* CHAT */
#[derive(Deserialize, Serialize, Debug)]
pub struct Convo {
    pub content: String,
    pub role: String,
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/* RESPONSE */
#[derive(Deserialize, Debug)]
pub struct OIChoices { //this is a subset of openai-response.
    pub message: Convo,
    index: u8,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct OIUsage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct OIResponse {
    pub choices: Vec<OIChoices>,
    created: Option<u64>,
    id: Option<String>,
    model: Option<String>,
    object: Option<String>,
    usage: Option<OIUsage>,
}

/* REQUEST */
#[derive(Serialize, Debug)]
pub struct OIRequest {
    pub model: String,
    pub messages: Vec<Convo>,
}
