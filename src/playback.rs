use std::error::Error;
use std::io::{Cursor, Seek, SeekFrom, Write};

use rodio::OutputStream;

pub fn play_audio(audio_bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let cursor = bytes_to_cursor(&audio_bytes)?;
    let sink = stream_handle.play_once(cursor)?;
    sink.sleep_until_end();

    Ok(())
}

fn bytes_to_cursor(audio_bytes: &[u8]) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_all(&audio_bytes)?;
    cursor.seek(SeekFrom::Start(0))?;

    Ok(cursor)
}
