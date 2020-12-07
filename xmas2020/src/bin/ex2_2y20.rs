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
        let mut chars: Vec<char> = vec![];

        for n in [self.first, self.second].iter() {
            match self.password.chars().nth((n - 1).into()) {
                Some(c) => chars.push(c),
                None => {}
            };
        }

        chars.iter().filter(|c| **c == self.letter).count() == 1
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
