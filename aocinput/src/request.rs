use reqwest::header::COOKIE;

use super::load_session;

pub fn retrieve_input(session_key: &str, year: u64, day: u8) -> String {
    let cookie = "session=".to_owned() + session_key;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(url.as_str())
        .header(COOKIE, cookie)
        .send()
        .unwrap();

    resp.text().unwrap()
}

pub fn get_input(year: u64, day: u8) -> String {
    let session_token = load_session::decrypt_session_token();

    retrieve_input(session_token.as_str(), year, day)
}
