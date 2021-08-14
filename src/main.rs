use std::error::Error;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, io};

use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use regex::Regex;

mod api;
mod playback;

static STORAGE_DIR: &'static str = "./audio_cache";

fn speak(text: &str) -> Result<(), Box<dyn Error>> {
    let function_start = SystemTime::now();

    let mut hasher = Sha1::new();
    hasher.input_str(text);
    let hex = hasher.result_str();
    let path_string = format!("{}/{}.wav", &STORAGE_DIR, hex);
    let file_path = Path::new(&path_string);
    let audio_bytes;

    if !file_path.exists() {
        audio_bytes = api::get_audio_bytes(text)?;
        fs::write(&file_path, &audio_bytes)?
    } else {
        audio_bytes = fs::read(&file_path)?
    }
    println!("Time elapsed: {:?}", function_start.elapsed());

    playback::play_audio(&audio_bytes)?;
    Ok(())
}

struct Handler;
impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let mut ctx: ClipboardContext =
            ClipboardProvider::new().expect("Failed to obtain clipboard context");
        let text = match ctx.get_contents() {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Failed to read text from clipboard: {}", error);
                String::from("error")
            }
        };

        let is_japanese =
            Regex::new(r"[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff66-\uff9f]")
                .unwrap();

        if is_japanese.is_match(&text) {
            println!("{}", text);

            match speak(&text.trim()) {
                Err(error) => {
                    eprintln!("There was an error: {}", error)
                }
                _ => {}
            }
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

pub fn main() {
    fs::create_dir_all(&STORAGE_DIR).expect("Failed to create audio storage directory");
    println!("Listening for clipboard changes...");
    let _ = Master::new(Handler).run();
}
