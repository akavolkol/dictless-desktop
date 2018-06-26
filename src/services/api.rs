use reqwest::{Client, Result};
use std::collections::HashMap;
use models::word::Word;

const ENDPOINT: &str = "http://localhost:3000";

#[derive(Deserialize)]
pub struct TranslationResponse {
    pub text: String,
}

pub fn translate(text: &String) -> Result<TranslationResponse> {
    let client = Client::new();
    let mut map = HashMap::new();
    map.insert("text", &text);
    let res = client
        .post(&[ENDPOINT, "/api/translate"].join(""))
        .json(&map)
        .send()?
        .json();
    res
}

pub fn get_words(query: &String) -> Result<Option<Vec<Word>>> {
    let client = Client::new();
    let res = client
        .get(&[ENDPOINT, "/api/words?query=", &query].join(""))
        .send()?
        .json();

    res
}
