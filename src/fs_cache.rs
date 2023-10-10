use std::path::Path;
use std::{fs, io};

use bytesize::ByteSize;
use data_encoding::HEXLOWER;
use fs_extra::dir::get_size;
use ring::digest::{digest, SHA256};

use crate::settings::SETTINGS;

pub fn init() {
    fs::create_dir_all(&SETTINGS.storage_dir).expect("Failed to create audio storage directory");
}

// Calling it every time is definitely suboptimal,
// but I think it looks cleaner this way
fn get_file_path(key: &str) -> String {
    let key_digest = digest(&SHA256, key.as_bytes());

    let key_hash = HEXLOWER.encode(key_digest.as_ref());
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

pub fn remove_oldest() -> io::Result<()> {
    let max_cache_size = ByteSize::mb(SETTINGS.max_cache_size_mb).as_u64();

    let store_dir = Path::new(&SETTINGS.storage_dir);
    let mut current_cache_size = get_size(store_dir).unwrap();

    if current_cache_size < max_cache_size {
        return Ok(());
    }

    let mut paths: Vec<fs::DirEntry> = fs::read_dir(store_dir)
        .unwrap()
        .map(|dir_result| (dir_result.unwrap()))
        .collect();

    paths.sort_by_cached_key(|dir| dir.path().metadata().unwrap().created().unwrap());

    for file_path in paths {
        if file_path
            .file_name()
            .into_string()
            .unwrap()
            .contains(".gitkeep")
        {
            continue;
        }

        fs::remove_file(&file_path.path())?;

        let size_after_delete = current_cache_size - file_path.metadata().unwrap().len();

        if size_after_delete < max_cache_size {
            break;
        } else {
            current_cache_size = size_after_delete
        }
    }

    Ok(())
}
