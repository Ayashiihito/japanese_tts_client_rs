use std::path::Path;
use std::{fs, io};

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use crate::settings::SETTINGS;

pub fn init() {
    fs::create_dir_all(&SETTINGS.storage_dir).expect("Failed to create audio storage directory");
}

// Calling it every time is definitely suboptimal,
// but I think it looks cleaner this way
fn get_file_path(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(key);
    let key_hash = hasher.result_str();

    format!("{}/{}.wav", &SETTINGS.storage_dir, key_hash)
}

//TODO should be generic
pub fn get(key: &str) -> io::Result<Vec<u8>> {
    let file_path = get_file_path(key);
    fs::read(Path::new(&file_path))
}

pub fn has(key: &str) -> bool {
    let file_path = get_file_path(key);
    Path::new(&file_path).exists()
}

pub fn set<'a>(&mut self, key: &str, value: &'a [u8]) -> io::Result<()> {
    let file_path = self.get_file_path(key);
    let value_size = value.len() as u64;

    while self.size + value_size > SETTINGS.max_cache_size {
        self.remove_oldest()?;
    }

    fs::write(Path::new(&file_path), value)?;

    self.size += value_size;
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + SETTINGS.cache_expiration;
    self.expiration.insert(key.to_string(), expiration);

    Ok(())
}

fn remove_oldest(&mut self) -> io::Result<()> {
    if let Some((key, _)) = self.expiration.iter().min_by_key(|(_, &v)| v) {
        let file_path = self.get_file_path(key);
        let metadata = fs::metadata(&file_path)?;
        let file_size = metadata.len();

        fs::remove_file(&file_path)?;
        self.size -= file_size;
        self.expiration.remove(key);
    }

    Ok(())
}
