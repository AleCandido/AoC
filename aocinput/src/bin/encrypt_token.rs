use std::fs::File;
use std::io::Write;

use aocinput;
use aocinput::crypt;

fn main() {
    println!("Insert token to be encrypted");
    let mut token = String::new();
    aocinput::user_input(&mut token).unwrap();

    println!("\nInsert an arbitrary password");
    let mut password = String::new();
    aocinput::user_input(&mut password).unwrap();

    //println!("\ntoken: {}\npassword: {}", token, password);
    let crypted = crypt::encrypt(token, password);

    let mut file = File::create("session_token.txt").unwrap();
    file.write_all(crypted.as_ref()).unwrap();
}
