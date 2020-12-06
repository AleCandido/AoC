use std::fs::File;
use std::io::prelude::*;
use std::str;

use super::crypt;
use super::user;

pub fn decrypt_session_token() -> String {
    let f = File::open("session_token.txt").unwrap();

    let mut password = String::new();
    match File::open("password.txt") {
        Ok(ref mut pwd_f) => {
            pwd_f.read_to_string(&mut password).expect("Unable to read");
            password.retain(|c| !c.is_whitespace());
        }
        Err(_) => {
            println!("Please type your password:");
            user::user_input(&mut password).expect("Unable to read");
        }
    }
    let mut password_b = password.into_bytes();
    password_b.resize_with(32, || 0);

    let key = &password_b;
    let iv: [u8; 16] = [0; 16];

    let encrypted_data = f.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
    let data = crypt::raw_decrypt(encrypted_data.as_ref(), &key, &iv).unwrap();

    str::from_utf8(&data).unwrap().to_owned()
}
