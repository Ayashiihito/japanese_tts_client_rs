# jtts client

jtts is a program that uses machine learning text-to-speech to pronounce any Japanese text that you copy into the clipboard.  
You need an instance of [jtts server](https://github.com/Ayashiihito/japanese_tts_server) running for it to work.

All generated audio is stored inside of `audio_cache` directory as `.wav` files with `SHA256` hash of the text as a title.

### Planned features:
- [x] Audio caching
- [ ] Audio metadata (with copied text as the track's title)  
- [-] Separate configuration file
- - [x] Server address and port
- - [x] Max cache size 

### Supported platforms:
- **Windows, Linux, MacOS (not tested, but should work)**
