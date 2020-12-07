use aocinput::request;

#[derive(Debug)]
struct PasswordRecord {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl PasswordRecord {
    fn from_string(s: String) -> PasswordRecord {
        let elems: Vec<&str> = s.split(|c| !char::is_alphanumeric(c)).collect();

        PasswordRecord {
            min: elems[0].parse().unwrap(),
            max: elems[1].parse().unwrap(),
            letter: elems[2].chars().next().unwrap(),
            password: elems[4].to_owned(),
        }
    }

    fn is_correct(&self) -> bool {
        let occ = self.password.matches(self.letter).count();

        (occ >= self.min as usize) && (occ <= self.max as usize)
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
