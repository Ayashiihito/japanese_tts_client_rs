use std::path::Path;
use std::{fs, io};

use data_encoding::BASE64;
use ring::digest::{digest, SHA256};

use crate::settings::SETTINGS;

pub fn init() {
    fs::create_dir_all(&SETTINGS.storage_dir).expect("Failed to create audio storage directory");
}

// Calling it every time is definitely suboptimal,
// but I think it looks cleaner this way
fn get_file_path(key: &str) -> String {
    let key_digest = digest(&SHA256, key.as_bytes());

    let key_hash = BASE64.encode(key_digest.as_ref());
    let dir = &SETTINGS.storage_dir;

    format!("{dir}/{key_hash}.wav")
}

pub fn get(key: &str) -> io::Result<Vec<u8>> {
    let file_path = get_file_path(key);
    fs::read(Path::new(&file_path))
}

pub fn has(key: &str) -> bool {
    let file_path = get_file_path(key);
    Path::new(&file_path).exists()
}

pub fn set<'a>(key: &str, value: &'a [u8]) -> io::Result<Vec<u8>> {
    let file_path = get_file_path(key);
    fs::write(Path::new(&file_path), value)?;

    Ok(value.to_vec())
}
