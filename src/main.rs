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

fn speak(text: &str) -> () {
    let mut hasher = Sha1::new();
    hasher.input_str(text);
    let hex = hasher.result_str();
    let path_string = format!("{}/{}.wav", &STORAGE_DIR, hex);
    let file_path = Path::new(&path_string);
    let res_bytes;

    if !file_path.exists() {
        let now = SystemTime::now();
        res_bytes = api::get_audio(text);
        let duration = now.elapsed();
        println!("Time elapsed: {:?}", duration);

        fs::write(&file_path, &res_bytes).expect("Failed to write to storage");
    } else {
        res_bytes = fs::read(&file_path).expect("Failed to read file");
    }

    let cursor = playback::bytes_to_cursor(&res_bytes);
    playback::play_audio(cursor);
}

struct Handler;
impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let text = ctx.get_contents().unwrap();
        let is_japanese =
            Regex::new(r"[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff66-\uff9f]")
                .unwrap();

        if is_japanese.is_match(&text) {
            println!("{}", text);
            speak(&text);
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

pub fn main() {
    let _ = Master::new(Handler).run();
    println!("Listening for clipboard changes...")
}
