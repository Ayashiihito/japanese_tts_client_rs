use std::error::Error;
use std::io;

use std::time::SystemTime;

use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler, Master};

#[macro_use]
extern crate lazy_static;

mod api;
mod fs_cache;
mod playback;
mod settings;
mod util;

fn play(text: &str) -> Result<(), Box<dyn Error>> {
    println!("{}", text);
    let function_start = SystemTime::now();

    let audio_bytes = if fs_cache::has(&text) {
        fs_cache::get(&text)
    } else {
        fs_cache::set(&text, &api::get_audio_bytes(text)?)
    }?;

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

        if util::is_japanese(&text) {
            match play(&text.trim()) {
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
    fs_cache::init();
    println!("Listening for clipboard changes...");
    let _ = Master::new(Handler).run();
}
fs_cache::remove_expired_items();
