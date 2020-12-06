use std::io;
use std::io::Write;

use std::fs::File;

use aocinput::crypt;

fn user_input(input: &mut String) -> io::Result<usize> {
    print!(">._.< ");
    let _ = io::stdout().flush();

    let mut tmp_in = String::new();
    let res = io::stdin().read_line(&mut tmp_in);
    match res {
        Ok(_) => {
            tmp_in.retain(|c| !c.is_whitespace());

            //input = tmp_in.to_owned();
            input.push_str(tmp_in.as_str());
        }
        Err(_) => {}
    };

    res
}

fn main() {
    println!("Insert token to be encrypted");
    let mut token = String::new();
    user_input(&mut token).unwrap();

    println!("\nInsert an arbitrary password");
    let mut password = String::new();
    user_input(&mut password).unwrap();

    //println!("\ntoken: {}\npassword: {}", token, password);
    let crypted = crypt::encrypt(token, password);

    let mut file = File::create("session_token.txt").unwrap();
    file.write_all(crypted.as_ref()).unwrap();
}
