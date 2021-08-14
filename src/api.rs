use std::collections::HashMap;
use std::error::Error;

use reqwest::blocking::Client;

static API_URI: &'static str = "http://127.0.0.1:5000/audio";

pub fn get_audio_bytes(text: &str) -> Result<std::vec::Vec<u8>, Box<dyn Error>> {
  let mut json_body = HashMap::new();
  json_body.insert("text", text);
  let res = Client::new()
    .post(API_URI)
    .json(&json_body)
    .send()?
    .bytes()?
    .to_vec();

  Ok(res)
}
