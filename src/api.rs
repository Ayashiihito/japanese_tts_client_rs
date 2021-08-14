use std::collections::HashMap;

use reqwest::blocking::Client;

static API_URI: &'static str = "http://127.0.0.1:5000/audio";

pub fn get_audio_bytes(text: &str) -> std::vec::Vec<u8> {
  let mut json_body = HashMap::new();
  json_body.insert("text", text);
  Client::new()
    .post(API_URI)
    .json(&json_body)
    .send()
    .expect("Failed to send audio request")
    .bytes()
    .expect("Failed to read bytes")
    .to_vec()
}
