use std::path::Path;
use std::{fs, io};

use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub static STORAGE_DIR: &'static str = "./audio_cache";

pub fn init() {
  fs::create_dir_all(&STORAGE_DIR).expect("Failed to create audio storage directory");
}

// Calling it every time is definitely suboptimal,
// but I think it looks cleaner this way
fn get_file_path(key: &str) -> String {
  let mut hasher = Sha1::new();
  hasher.input_str(key);
  let key_hash = hasher.result_str();
  format!("{}/{}.wav", &STORAGE_DIR, key_hash)
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
