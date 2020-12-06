use std::io;
use std::io::prelude::*;

pub fn user_input(input: &mut String) -> io::Result<usize> {
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
