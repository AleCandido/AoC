use aocinput::request;

const FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let resp = request::get_input(2020, 4);
    let passports: Vec<String> = resp.trim().split("\n\n").map(|s| s.to_owned()).collect();

    let mut fields = FIELDS.to_vec();
    fields.sort();
    let mut fields_complete = fields.clone();
    fields_complete.push("cid");
    fields_complete.sort();

    let mut valid_pps = 0;

    for passport in passports.iter() {
        let mut pp_fields: Vec<&str> = passport
            .trim()
            .split_whitespace()
            .map(|s| s.split(":").next().unwrap())
            .collect();

        pp_fields.sort();

        if pp_fields == fields || pp_fields == fields_complete {
            valid_pps += 1
        }
    }

    println!("#valid passports: {}", valid_pps);
}
