[![Clippy Linting Result](https://clippy.bashy.io/github/danielrs/dobro/master/badge.svg)](https://clippy.bashy.io/github/danielrs/dobro/master/log)

# dobro
Unofficial Pandora terminal client written in Rust.

### Building

#### Required libraries

The libraries and their versions that are used to compile the project are:

* ffmpeg 2.8 (libavcodec, libavformat, libavdevice)
* libao 1.1.0
* libncurses 6.0

#### Compiling

If everything is installed, a simple `cargo run` with the nightly compiler should suffice for testing the player.

### What's going on right now?

This an app that I'm building during my free time. It will consist of the following main components (most to least important):

- API interaction (pandora-rs).
- Audio playback.
- Text-based user interface (TUI).
- User Settings.

Local crates for components can be found at [src/lib](https://github.com/DanielRS/dobro/tree/master/src/lib).

#### API Interaction (pandora-rs)
Most of the work for this module is already done. It interacts with the API in a very rusty way using [hyper][hyper]; all the requests/responses are serialized/deserialized using [serde][serde] and [serde_json][serde_json]. The pandora-rs module interacts with the API found [here](https://6xq.net/pandora-apidoc/json/).

#### Audio playback (earwax, ao-rs)
For **audio decoding** I made a small C library with Rust bindings based on [ffmpeg 2.8][ffmpeg] called Earwax. For audio playpack I'm using [libao][libao] with safe ffi bindings.

#### TUI
Simple interface made with ncurses. This would be the "main" Dobro application, and it builds on the lower-level components.

#### User settings
After everything else is done. Should load from simple configuration files (preferably in toml format).

[hyper]: https://github.com/hyperium/hyper
[serde]: https://github.com/serde-rs/serde
[serde_json]: https://github.com/serde-rs/json

[ffmpeg]: https://www.ffmpeg.org/
[libao]: https://www.xiph.org/ao/
