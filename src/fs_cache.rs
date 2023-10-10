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

fn sort_files_by_creation_date(paths: Vec<fs::DirEntry>) -> io::Result<Vec<fs::DirEntry>> {
    let mut sorted_paths = paths;
    sorted_paths.sort_by_cached_key(|dir| dir.path().metadata()?.created()?);
    Ok(sorted_paths)
}

fn delete_files_until_size(paths: Vec<fs::DirEntry>, mut current_cache_size: u64, max_cache_size: u64) -> io::Result<u64> {
    for file_path in paths {
        if file_path
            .file_name()
            .into_string()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to convert filename to string"))?
            .contains(".gitkeep")
        {
            continue;
        }

        fs::remove_file(&file_path.path())?;

        let size_after_delete = current_cache_size - file_path.metadata()?.len();

        if size_after_delete < max_cache_size {
            return Ok(size_after_delete);
        } else {
            current_cache_size = size_after_delete
        }
    }

    Ok(current_cache_size)
}

pub fn remove_oldest() -> io::Result<()> {
    let max_cache_size = ByteSize::mb(SETTINGS.max_cache_size_mb).as_u64();

    let store_dir = Path::new(&SETTINGS.storage_dir);
    let mut current_cache_size = get_size(store_dir)?;

    if current_cache_size < max_cache_size {
        return Ok(());
    }

    let paths: Vec<fs::DirEntry> = fs::read_dir(store_dir)?
        .map(|dir_result| dir_result.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
        .collect();

    let sorted_paths = sort_files_by_creation_date(paths)?;
    let new_cache_size = delete_files_until_size(sorted_paths, current_cache_size, max_cache_size)?;

    Ok(())
}
