use std::io;
use std::io::Write;

use aocinput::request;

fn main() {
    let mut input = String::new();
    print!(">._.< ");
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            input.retain(|c| !c.is_whitespace());
            println!("{} bytes read", n);
            println!();
        }
        Err(error) => println!("error: {}", error),
    };

    let resp = request::get_input(input.as_str());
    println!("{}", resp);
}
