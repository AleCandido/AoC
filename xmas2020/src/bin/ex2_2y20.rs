//1. Quick_sort on the numbers' list
//2. start from the beginning and loop over:
//- subtract the number from the target
//- binary search if the difference it's present
use aocinput::request;

#[derive(Debug)]
struct PasswordRecord {
    first: u8,
    second: u8,
    letter: char,
    password: String,
}

impl PasswordRecord {
    fn from_string(s: String) -> PasswordRecord {
        let elems: Vec<&str> = s.split(|c| !char::is_alphanumeric(c)).collect();

        PasswordRecord {
            first: elems[0].parse().unwrap(),
            second: elems[1].parse().unwrap(),
            letter: elems[2].chars().next().unwrap(),
            password: elems[4].to_owned(),
        }
    }

    fn is_correct(&self) -> bool {
        true
    }
}

fn main() {
    let resp = request::get_input(2020, 2);
    let passwords: Vec<PasswordRecord> = resp
        .trim()
        .split("\n")
        .map(|s| PasswordRecord::from_string(s.to_owned()))
        .collect();

    println!(
        "#correct: {}",
        passwords.iter().filter(|pwd| pwd.is_correct()).count()
    );
}
