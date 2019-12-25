# https://www.WebAssemblyMan.com
MAN page for WebAssembly : Rust, Blazor & Emscripten. Samples, tutorials and examples.

For the rustwasm samples, install Rust.

https://www.rust-lang.org/tools/install

__cargo install wasm-bindgen-cli__

__rustup target add wasm32-unknown-unknown__

To build the samples, simply go to the specific folders and enter :

__wasm-pack build --target web__

To run the samples, launch a http server at the specific folder :

__python3 -m http.server__

## License
GPLv3.0
