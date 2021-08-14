use std::io::{Cursor, Seek, SeekFrom, Write};

use rodio::OutputStream;

pub fn play_audio(cursor: Cursor<Vec<u8>>) {
  let (_stream, stream_handle) =
    OutputStream::try_default().expect("Failed to create output stream");
  let sink = stream_handle
    .play_once(cursor)
    .expect("Failed to play audio");
  sink.sleep_until_end();
}

pub fn bytes_to_cursor(sound_bytes: &[u8]) -> Cursor<Vec<u8>> {
  let mut c = Cursor::new(Vec::new());
  c.write_all(&sound_bytes)
    .expect("Failed to read sound file data");
  c.seek(SeekFrom::Start(0))
    .expect("Failed to seek to start of buffer");
  c
}
