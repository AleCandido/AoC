use std::fs::File;
use std::io::prelude::*;
use std::str;

pub mod crypt;
pub mod request;

pub fn decrypt_session_token() -> String {
    //pub fn decrypt_session_token() {
    let f = File::open("session_token.txt").unwrap();

    let mut password = "".to_owned().into_bytes();
    password.resize_with(32, || 0);

    let key = &password;
    let iv: [u8; 16] = [0; 16];

    let encrypted_data = f.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
    let data = crypt::raw_decrypt(encrypted_data.as_ref(), &key, &iv).unwrap();

    str::from_utf8(&data).unwrap().to_owned()
}
