use reqwest::header::COOKIE;

pub fn get_input(session_key: &str) -> String {
    let cookie = "session=".to_owned() + session_key;
    let client = reqwest::blocking::Client::new();

    let resp = client
        .get("https://adventofcode.com/2020/day/1/input")
        .header(COOKIE, cookie)
        .send()
        .unwrap();

    resp.text().unwrap()
}
