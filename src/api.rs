use std::collections::HashMap;
use std::error::Error;

use reqwest::blocking::Client;

static API_URI: &'static str = "http://127.0.0.1:5000/audio";

pub fn get_audio_bytes(text: &str) -> Result<std::vec::Vec<u8>, Box<dyn Error>> {
    let mut json_body = HashMap::new();
    json_body.insert("text", text);

    //TODO: don't create client on each request
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()?;

    let res = client
        .post(API_URI)
        .json(&json_body)
        .send()?
        .bytes()?
        .to_vec();

    Ok(res)
}
