use std::error::Error;
use std::io::{Cursor, Seek, SeekFrom, Write};

use rodio::OutputStream;

pub fn play_audio(cursor: Cursor<Vec<u8>>) -> Result<(), Box<dyn Error>> {
  let (_stream, stream_handle) = OutputStream::try_default()?;

  let sink = stream_handle.play_once(cursor)?;
  sink.sleep_until_end();

  Ok(())
}

pub fn bytes_to_cursor(sound_bytes: &[u8]) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
  let mut cursor = Cursor::new(Vec::new());
  cursor.write_all(&sound_bytes)?;
  cursor.seek(SeekFrom::Start(0))?;

  Ok(cursor)
}
