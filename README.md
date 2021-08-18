# jttsr client

is a program that uses machine learning text-to-speech to say aloud any Japanese text that you copy into the clipboard.  
You need an instance of [jtts server](https://github.com/Ayashiihito/japanese_tts_server) running for it to work.

All generated audio is permanently stored inside of `audio_cache` folder as `.wav` files with `SHA1` hash of the text as a title.

### Planned features:
- [x] Audio caching
- [ ] Audio metadata (with copied text as the track's title)  
- [ ] Separate configuration file
- - [ ] Server address and port
- - [ ] Max cache size

### Supported platforms:
- **Windows, Linux, MacOS**
